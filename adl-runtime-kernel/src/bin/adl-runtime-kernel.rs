use std::{
    collections::{BTreeMap, BTreeSet},
    future::Future,
    path::{Path, PathBuf},
    pin::Pin,
    process::ExitCode,
    sync::Arc,
};

#[path = "../observability.rs"]
mod observability;

use adl_runtime_kernel::layer8_authority::{
    ConversationAuthorityProfile, ConversationSigningProfile, Layer8AuthorityStore,
    Layer8ConversationAuthority, Layer8SignedExchange,
};
use adl_runtime_kernel::{
    birthday_authority_bootstrap_from_runtime_keys, bootstrap_reasoning_services,
    build_live_assembly, build_live_continuity_registry, build_mutual_tls_server_config,
    build_production_operation_executors_with_recorder, load_control_tls, load_identity,
    load_or_create_runtime_instance_id, load_trust_roots, monitor_until_stop,
    serve_control_listener_until_ready, serve_private_continuity_listener,
    validate_production_operation_executors, verifying_key_from_hex, AdapterKind,
    AgentPopulationFeed, CatalogSigningAuthority, CheckpointShutdownRequest, CheckpointingControl,
    ContinuityControlService, ControlApiPolicy, ControlAuthority, ControlCapability,
    ControlService, DurableContinuityJournal, Kernel, KernelExit, LiveBindings, LiveContinuity,
    LiveKernelSnapshot, ObservabilityDegradation, ObservabilityHealth, OperationRequest,
    RecorderTrustedTime, RsntpTimeSampleSource, RunningState, RuntimeInitConfig, RuntimeRecorder,
    SysinfoWeatherObserver, TargetContinuityCoordinator, TimeQualificationBounds, TimeSampleSource,
    TlsIdentityPaths, TrustedControlKey, TrustedTime, AGENT_ADMISSION_HEARTBEAT_TTL_MILLIS,
    OPERATION_REQUEST_SCHEMA, PRIVATE_ALPN,
};
use observability::{RuntimeVectorConfig, RuntimeVectorPipeline};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

const GUARDIAN_LEASE_ADDRESS_ENV: &str = "ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS";
const GUARDIAN_LEASE_TOKEN_ENV: &str = "ADL_RUNTIME_GUARDIAN_LEASE_TOKEN";

#[tokio::main]
async fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(command) = args.next() else {
        eprintln!("{}", usage());
        return ExitCode::from(64);
    };

    match command.as_str() {
        "serve" => {
            let serve_args = match ServeArgs::parse(args) {
                Ok(args) => args,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(64);
                }
            };
            let init_path = match canonical_init_path(&serve_args.init_path) {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("runtime init path invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let init = match RuntimeInitConfig::load(Some(init_path)) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("runtime init invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let _birth_witness_owner = match init.birth_witness_owner() {
                Ok(owner) => owner,
                Err(error) => {
                    eprintln!("runtime birth-witness trust invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_control_config = match init.continuity_control.clone() {
                Some(config) => config,
                None => {
                    eprintln!("runtime private continuity configuration is required");
                    return ExitCode::from(78);
                }
            };
            let operation_state_identity = match canonical_state_root(init.state_root()) {
                Ok(path) => path,
                Err(error) => {
                    eprintln!("runtime state root invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_root = match canonical_configured_child_dir(
                &operation_state_identity,
                &init.continuity_root(),
                "runtime continuity root",
            ) {
                Ok(root) => root,
                Err(error) => {
                    eprintln!("runtime continuity root is invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let kernel_shutdown_grace =
                std::time::Duration::from_millis(init.shutdown.kernel_grace_millis);
            let api_drain_timeout =
                std::time::Duration::from_millis(init.shutdown.api_drain_millis);
            let standard_checkpoint_deadline =
                std::time::Duration::from_millis(init.shutdown.checkpoint_deadline_millis);
            let observability_poll =
                std::time::Duration::from_millis(init.kernel.observability_poll_millis);
            let weather_retry_delay = std::time::Duration::from_millis(init.weather.sample_millis);
            let guardian_lease_connect_timeout =
                std::time::Duration::from_millis(init.kernel.guardian_lease_connect_millis);
            let guardian_lease_auth_timeout =
                std::time::Duration::from_millis(init.kernel.guardian_lease_auth_millis);
            let tls = match load_control_tls(&init.api.tls).await {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let socket_addrs = match init.socket_addrs() {
                Ok(addrs) => addrs,
                Err(error) => {
                    eprintln!("runtime init invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let private_listener = match tokio::net::TcpListener::bind(
                continuity_control_config
                    .socket_addr()
                    .expect("validated continuity address"),
            )
            .await
            {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("runtime private continuity bind failed: {error}");
                    return ExitCode::from(78);
                }
            };
            let private_identity = match load_identity(&TlsIdentityPaths {
                certificate_chain_path: continuity_control_config
                    .tls
                    .server_certificate_chain_path
                    .clone(),
                private_key_path: continuity_control_config
                    .tls
                    .server_private_key_path
                    .clone(),
            })
            .await
            {
                Ok(identity) => identity,
                Err(error) => {
                    eprintln!("runtime private continuity TLS identity invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let private_client_roots = match load_trust_roots(
                &continuity_control_config.tls.server_trust_roots_path,
            )
            .await
            {
                Ok(roots) => roots,
                Err(error) => {
                    eprintln!("runtime private continuity client roots invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let private_tls = match build_mutual_tls_server_config(
                private_identity,
                private_client_roots,
                PRIVATE_ALPN,
            ) {
                Ok(config) => config,
                Err(error) => {
                    eprintln!("runtime private continuity TLS invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_secret_text = match read_trimmed_config_file(
                &init.credentials.continuity_signing_key_path,
                "runtime continuity signing key",
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_secret =
                match LiveContinuity::signing_key_from_hex(&continuity_secret_text) {
                    Ok(secret) => secret,
                    Err(_) => {
                        eprintln!("runtime continuity signing key is missing or invalid");
                        return ExitCode::from(78);
                    }
                };
            let continuity_key_id = init.credentials.continuity_key_id.clone();
            let mut guardian_lease = match connect_guardian_lease(
                guardian_lease_connect_timeout,
                guardian_lease_auth_timeout,
            )
            .await
            {
                Ok(lease) => lease,
                Err(error) => {
                    eprintln!("runtime Guardian lease invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let instance_id = match load_or_create_runtime_instance_id(&operation_state_identity) {
                Ok(instance_id) => instance_id,
                Err(error) => {
                    eprintln!("runtime instance identity invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let recorder = RuntimeRecorder::new(init.kernel.recorder_capacity);
            let roster_trusted_time = RecorderTrustedTime::new(recorder.clone());
            let reasoning = match bootstrap_reasoning_services(recorder.clone()) {
                Ok(reasoning) => reasoning,
                Err(error) => {
                    eprintln!("runtime reasoning bootstrap invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_reasoning = reasoning.clone();
            let operation_executors = match build_production_operation_executors_with_recorder(
                operation_state_identity.clone(),
                recorder.clone(),
            ) {
                Ok(executors) => executors,
                Err(error) => {
                    eprintln!("runtime local adapter state root is invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            if let Err(error) = validate_production_operation_executors(&operation_executors) {
                eprintln!("runtime live operation adapters unavailable: {error}");
                return ExitCode::from(78);
            }
            let resident_shepherd_executor = operation_executors
                .get(&AdapterKind::Shepherd)
                .cloned()
                .expect("validated production assembly contains Shepherd adapter");
            let operation_key_text = match read_trimmed_config_file(
                &init.credentials.operation_public_key_path,
                "runtime operation permit key",
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let operation_key = match verifying_key_from_hex(&operation_key_text) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("runtime operation permit key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            if ed25519_dalek::SigningKey::from_bytes(&continuity_secret).verifying_key()
                == operation_key
            {
                eprintln!("runtime continuity and operation keys must be distinct");
                return ExitCode::from(78);
            }
            let operation_key_id = init.credentials.operation_key_id.clone();
            let migration_decision_key_text = match read_trimmed_config_file(
                &init.credentials.migration_decision_public_key_path,
                "runtime migration decision key",
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let migration_decision_key = match verifying_key_from_hex(&migration_decision_key_text)
            {
                Ok(key) if key != operation_key => key,
                _ => {
                    eprintln!(
                            "runtime migration decision key is invalid or aliases the operation permit key"
                        );
                    return ExitCode::from(78);
                }
            };
            let migration_decision_key_id = init.credentials.migration_decision_key_id.clone();
            let migration_decision_key_generation =
                init.credentials.migration_decision_key_generation;
            let continuity_public_key =
                ed25519_dalek::SigningKey::from_bytes(&continuity_secret).verifying_key();
            let time_source_identity = format!("sntp:{}", init.credentials.sntp_server);
            let time_source: Arc<dyn TimeSampleSource> = Arc::new(RsntpTimeSampleSource::new(
                init.credentials.sntp_server.clone(),
            ));
            let assembly = match build_live_assembly(LiveBindings {
                recorder: recorder.clone(),
                canonical_ingress_capacity: init.kernel.canonical_ingress_capacity,
                operation_executors,
                permit_keys: BTreeMap::from([(operation_key_id.clone(), operation_key)]),
                birthday_authority: birthday_authority_bootstrap_from_runtime_keys(
                    operation_key_id.clone(),
                    operation_key,
                    migration_decision_key_id.clone(),
                    migration_decision_key,
                    init.credentials.continuity_key_id.clone(),
                    continuity_public_key,
                    migration_decision_key_generation,
                    init.credentials.continuity_min_generation.max(1),
                    1,
                ),
                reasoning,
                time_source,
                time_bounds: TimeQualificationBounds {
                    timeout: std::time::Duration::from_millis(
                        init.kernel.trusted_time_sample_timeout_millis,
                    ),
                    max_offset: std::time::Duration::from_millis(
                        init.kernel.trusted_time_max_offset_millis,
                    ),
                    max_round_trip: std::time::Duration::from_millis(
                        init.kernel.trusted_time_max_round_trip_millis,
                    ),
                    retry_delay: std::time::Duration::from_millis(
                        init.kernel.trusted_time_retry_millis,
                    ),
                    refresh_interval: std::time::Duration::from_millis(
                        init.kernel.trusted_time_refresh_millis,
                    ),
                },
            }) {
                Ok(assembly) => assembly,
                Err(error) => {
                    eprintln!("runtime live topology invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_registry = match build_live_continuity_registry(
                &assembly,
                recorder.clone(),
                continuity_reasoning.clone(),
                &operation_state_identity,
                continuity_control_config.bounds.max_services,
            ) {
                Ok(registry) => registry,
                Err(error) => {
                    eprintln!("runtime live continuity registry invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let catalog_authority = match CatalogSigningAuthority::from_secret(
                init.credentials.continuity_key_id.clone(),
                1,
                &continuity_secret,
            ) {
                Ok(authority) => authority,
                Err(error) => {
                    eprintln!("runtime private continuity authority invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let catalog_verifying_key = catalog_authority.verifying_key();
            let source_effects = match adl_runtime_kernel::SourceContinuityEffectPort::open(
                continuity_root.join("private-exports"),
                continuity_registry,
                catalog_authority,
                continuity_control_config.bounds.clone(),
                continuity_control_config.channel_epoch,
            ) {
                Ok(port) => Arc::new(port),
                Err(error) => {
                    eprintln!("runtime source continuity effects invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let target_effects = match TargetContinuityCoordinator::open(
                continuity_control_config.clone(),
                BTreeMap::from([(
                    (init.credentials.continuity_key_id.clone(), 1),
                    catalog_verifying_key,
                )]),
                BTreeMap::from([(
                    (
                        migration_decision_key_id.clone(),
                        migration_decision_key_generation,
                    ),
                    migration_decision_key,
                )]),
            ) {
                Ok(port) => Arc::new(port),
                Err(error) => {
                    eprintln!("runtime target continuity effects invalid: {error}");
                    return ExitCode::from(78);
                }
            };
            let continuity_journal =
                match DurableContinuityJournal::open(&continuity_control_config) {
                    Ok(journal) => journal,
                    Err(error) => {
                        eprintln!("runtime private continuity journal invalid: {error}");
                        return ExitCode::from(78);
                    }
                };
            let private_service = Arc::new(ContinuityControlService::new(
                continuity_control_config,
                continuity_journal,
                source_effects,
                target_effects,
            ));
            let minimum_generation = init.credentials.continuity_min_generation;
            let public_key_text = match read_trimmed_config_file(
                &init.credentials.control_public_key_path,
                "runtime control key",
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            let public_key = match verifying_key_from_hex(&public_key_text) {
                Ok(key) => key,
                Err(_) => {
                    eprintln!("runtime control key is missing or invalid");
                    return ExitCode::from(78);
                }
            };
            if public_key == operation_key
                || public_key == continuity_public_key
                || public_key == migration_decision_key
                || migration_decision_key == continuity_public_key
            {
                eprintln!(
                    "runtime control, operation, migration decision, and continuity keys must be distinct"
                );
                return ExitCode::from(78);
            }
            let key_id = init.credentials.control_key_id.clone();
            let principal = init.credentials.control_principal.clone();
            let service_schemas = assembly
                .contracts
                .contracts()
                .map(|contract| (contract.service.clone(), contract.config_schema.clone()))
                .collect::<BTreeMap<_, _>>();
            let tls_certificate_hash = match file_hash(&init.api.tls.certificate_chain_path).await {
                Ok(hash) => hash,
                Err(error) => {
                    eprintln!("runtime TLS certificate identity could not be hashed: {error}");
                    return ExitCode::from(78);
                }
            };
            let runtime_init_identity = match init.continuity_identity_projection() {
                Ok(identity) => identity,
                Err(error) => {
                    eprintln!("runtime init identity could not be encoded: {error}");
                    return ExitCode::from(70);
                }
            };
            let binding_projection = serde_json::json!({
                "assembly_config_hash": assembly.config_hash,
                "runtime_init": runtime_init_identity,
                "time_source": &time_source_identity,
                "operation_key_id": &operation_key_id,
                "operation_key": hex::encode(operation_key.as_bytes()),
                "migration_decision_key_id": &migration_decision_key_id,
                "migration_decision_key_generation": migration_decision_key_generation,
                "migration_decision_key": hex::encode(migration_decision_key.as_bytes()),
                "control_key_id": &key_id,
                "control_principal": &principal,
                "control_key": hex::encode(public_key.as_bytes()),
                "continuity_key_id": &continuity_key_id,
                "continuity_key": hex::encode(continuity_public_key.as_bytes()),
                "operation_state_root": operation_state_identity,
                "tls_certificate_hash": tls_certificate_hash,
            });
            let config_hash = blake3::hash(
                &serde_json::to_vec(&binding_projection)
                    .expect("runtime binding JSON is encodable"),
            )
            .to_hex()
            .to_string();
            let snapshot = LiveKernelSnapshot::new(
                assembly.topology_hash.clone(),
                config_hash,
                service_schemas,
            );
            let mut continuity = LiveContinuity::new(
                &continuity_root,
                continuity_key_id,
                &continuity_secret,
                snapshot,
                minimum_generation,
            )
            .with_canonical_ingress(assembly.canonical_ingress.clone());
            if let Err(error) = continuity.restore_latest(&recorder).await {
                eprintln!("runtime continuity restore refused: {error}");
                return ExitCode::from(78);
            }
            let authority = ControlAuthority::new(BTreeMap::from([(
                key_id,
                TrustedControlKey {
                    principal,
                    verifying_key: public_key,
                    capabilities: BTreeSet::from([
                        ControlCapability::Read,
                        ControlCapability::Execute,
                        ControlCapability::Stop,
                    ]),
                },
            )]));
            let (lifecycle, mut shutdown_requests) =
                CheckpointingControl::channel(init.kernel.checkpoint_channel_capacity);
            let layer8 = match (
                std::env::var_os("ADL_LAYER8_AUTHORITY_PROFILE"),
                std::env::var_os("ADL_LAYER8_SIGNING_PROFILE"),
            ) {
                (None, None) => None,
                (Some(profile_path), Some(signing_profile_path)) => {
                    let profile = std::fs::read(profile_path).ok().and_then(|bytes| {
                        serde_json::from_slice::<ConversationAuthorityProfile>(&bytes).ok()
                    });
                    let signing_profile =
                        std::fs::read(signing_profile_path).ok().and_then(|bytes| {
                            serde_json::from_slice::<ConversationSigningProfile>(&bytes).ok()
                        });
                    let loaded = profile.zip(signing_profile).and_then(
                        |(authority_profile, signing_profile)| {
                            let sender = &signing_profile.sender;
                            if authority_profile.evidence.polis_id != instance_id
                                || sender.principal_id != authority_profile.evidence.principal_id
                                || sender.polis_id != authority_profile.evidence.polis_id
                                || sender.signing_key_id
                                    != authority_profile.evidence.signing_key_id
                                || std::fs::read_to_string(&sender.private_key_file)
                                    .ok()
                                    .and_then(|encoded| hex::decode(encoded.trim()).ok())
                                    .and_then(|secret| <[u8; 32]>::try_from(secret).ok())
                                    .map(|secret| {
                                        hex::encode(
                                            ed25519_dalek::SigningKey::from_bytes(&secret)
                                                .verifying_key()
                                                .to_bytes(),
                                        )
                                    })
                                    .as_deref()
                                    != Some(&authority_profile.evidence.verifying_key_hex)
                                || signing_profile
                                    .recipients
                                    .iter()
                                    .any(|recipient| recipient.polis_id != instance_id)
                            {
                                return None;
                            }
                            let store = Layer8AuthorityStore::open(
                                operation_state_identity
                                    .join("authority/layer8-conversation-audit.jsonl"),
                            )
                            .ok()?;
                            Some((
                                Layer8ConversationAuthority::new(store, authority_profile).ok()?,
                                Layer8SignedExchange::load(signing_profile).ok()?,
                            ))
                        },
                    );
                    match loaded {
                        Some(value) => Some(value),
                        None => {
                            eprintln!("runtime Layer 8 authority configuration invalid");
                            return ExitCode::from(78);
                        }
                    }
                }
                _ => {
                    eprintln!("runtime Layer 8 authority configuration incomplete");
                    return ExitCode::from(78);
                }
            };
            let mut service = ControlService::new_with_observatory_config_and_agents(
                instance_id.clone(),
                recorder.clone(),
                lifecycle,
                authority,
                init.kernel.control_history_capacity,
                init.observatory_allowed_origins(),
                AgentPopulationFeed::resident_shepherd(),
            )
            .with_canonical_ingress(assembly.canonical_ingress.clone());
            if let Some((authority, exchange)) = layer8 {
                service = service
                    .with_layer8_authority(authority)
                    .with_layer8_signed_exchange(exchange);
            }
            let service = Arc::new(service);
            service.set_agent_roster_token_key(blake3::derive_key(
                "adl.runtime_v3.agent_roster.page_token.continuity.v1",
                &continuity_secret,
            ));
            let api_policy = ControlApiPolicy::new(
                api_drain_timeout,
                std::time::Duration::from_millis(init.api.websocket_auth_timeout_millis),
                std::time::Duration::from_millis(init.api.websocket_refresh_millis),
                init.api.websocket_max_frame_bytes,
            )
            .expect("validated runtime init must produce a valid control API policy");
            let observatory_token = match read_trimmed_config_file(
                &init.credentials.observatory_token_path,
                "runtime Observatory read token",
            )
            .await
            {
                Ok(token) => token,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            if service
                .set_observatory_bearer_token(&observatory_token)
                .is_err()
            {
                eprintln!("runtime Observatory read token is invalid");
                return ExitCode::from(78);
            }
            let acip_write_token = match read_trimmed_config_file(
                &init.credentials.acip_write_token_path,
                "runtime ACIP write token",
            )
            .await
            {
                Ok(token) => token,
                Err(error) => {
                    eprintln!("{error}");
                    return ExitCode::from(78);
                }
            };
            if service
                .set_acip_write_bearer_token(&acip_write_token)
                .is_err()
            {
                eprintln!("runtime ACIP write token is invalid");
                return ExitCode::from(78);
            }
            if service
                .set_public_base_url(&init.api.public_base_url)
                .is_err()
            {
                eprintln!("runtime public HTTPS base is invalid");
                return ExitCode::from(78);
            }
            service.set_weather_stale_after(std::time::Duration::from_millis(
                init.kernel.weather_stale_after_millis,
            ));
            let pressure_checkpoint_deadline =
                std::time::Duration::from_millis(init.weather.checkpoint_deadline_millis);
            let api_shutdown = tokio_util::sync::CancellationToken::new();
            let vector_config = RuntimeVectorConfig::from_init_config(
                init.runtime_observability(),
                init.paths.observability_root(&operation_state_identity),
                instance_id.clone(),
            );
            let mut observability = match vector_config.and_then(RuntimeVectorPipeline::start) {
                Ok(pipeline) => Some(pipeline),
                Err(error) => {
                    eprintln!("runtime observability degraded; Runtime remains available: {error}");
                    recorder.initialize_observability(ObservabilityHealth::Degraded {
                        reason: ObservabilityDegradation::ExporterUnavailable,
                    });
                    None
                }
            };
            if let Some(observability) = observability.as_ref() {
                recorder.set_observability_pipeline(observability.snapshot());
            }
            let mut shutdown_signal = match ShutdownSignalReceiver::register() {
                Ok(signal) => signal,
                Err(error) => {
                    eprintln!("runtime signal handler registration failed: {error}");
                    if let Some(observability) = observability.as_mut() {
                        let _ = observability.shutdown().await;
                    }
                    return ExitCode::from(78);
                }
            };
            let listener = match bind_control_listener(
                &socket_addrs,
                init.api.bind_attempts,
                std::time::Duration::from_millis(init.api.bind_retry_millis),
            )
            .await
            {
                Ok(listener) => listener,
                Err(error) => {
                    eprintln!("runtime control API bind failed: {error}");
                    if let Some(observability) = observability.as_mut() {
                        let _ = observability.shutdown().await;
                    }
                    return ExitCode::from(70);
                }
            };
            let mut handle = match Kernel::new(assembly.topology, recorder.clone())
                .with_readiness_timeout(std::time::Duration::from_millis(
                    init.kernel.component_readiness_timeout_millis,
                ))
                .start()
                .await
            {
                Ok(handle) => handle,
                Err(error) => {
                    eprintln!("runtime kernel failed to start: {error}");
                    if let Some(observability) = observability.as_mut() {
                        let _ = observability.shutdown().await;
                    }
                    return ExitCode::from(70);
                }
            };
            let shepherd_admission = OperationRequest {
                schema: OPERATION_REQUEST_SCHEMA.to_owned(),
                request_id: format!("{instance_id}:resident-shepherd-admission"),
                idempotency_key: "resident-shepherd-admission".to_owned(),
                principal: "runtime-bootstrap".to_owned(),
                payload: br#"{"schema":"adl.runtime.local_shepherd_admission.v1","admit":true}"#
                    .to_vec(),
                permit: None,
            };
            if let Err(error) = resident_shepherd_executor
                .execute(&shepherd_admission)
                .await
            {
                eprintln!("runtime resident Shepherd admission failed: {error}");
                let _ = handle.shutdown(kernel_shutdown_grace).await;
                if let Some(observability) = observability.as_mut() {
                    let _ = observability.shutdown().await;
                }
                return ExitCode::from(70);
            }
            let mut private_api = tokio::spawn(serve_private_continuity_listener(
                private_listener,
                private_tls,
                private_service,
                api_shutdown.child_token(),
            ));
            let (ready_sender, ready_receiver) = tokio::sync::oneshot::channel();
            let mut api = tokio::spawn(serve_control_listener_until_ready(
                service.clone(),
                listener,
                tls,
                api_policy,
                ready_sender,
                api_shutdown.clone().cancelled_owned(),
            ));
            let bound_address = match ready_receiver.await {
                Ok(address) => address,
                Err(_) => {
                    eprintln!("runtime control API failed before readiness");
                    let _ = handle.shutdown(kernel_shutdown_grace).await;
                    drain_control_api(&mut api, api_drain_timeout).await;
                    drain_private_api(&mut private_api, api_drain_timeout).await;
                    if let Some(observability) = observability.as_mut() {
                        let _ = observability.shutdown().await;
                    }
                    return ExitCode::from(70);
                }
            };
            eprintln!(
                "{}",
                adl_runtime_kernel::control_ready_event(
                    &instance_id,
                    bound_address,
                    &init.api.public_base_url,
                )
            );
            let mut pressure_retry_at = None;
            let mut observability_tick = tokio::time::interval(observability_poll);
            observability_tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let mut shepherd_heartbeat =
                tokio::time::interval(std::time::Duration::from_millis(1_000));
            shepherd_heartbeat.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
            let serve_result = 'serve: loop {
                if let Some(observability) = observability.as_mut() {
                    if let Err(error) = observability.poll_health() {
                        recorder.set_observability_pipeline(observability.snapshot());
                        eprintln!("runtime observability pipeline failed: {error}");
                    }
                    recorder.set_observability_pipeline(observability.snapshot());
                }
                let weather_service = service.clone();
                let pressure_delay = pressure_retry_at.take();
                let pressure_monitor = async {
                    if let Some(deadline) = pressure_delay {
                        tokio::time::sleep_until(deadline).await;
                    }
                    monitor_until_stop(
                        init.weather.clone(),
                        SysinfoWeatherObserver::for_path(&continuity_root),
                        move |report| weather_service.set_weather_report(report),
                    )
                    .await
                };
                tokio::pin!(pressure_monitor);
                let trigger = 'wait: loop {
                    tokio::select! {
                    pressure = &mut pressure_monitor => {
                        eprintln!("event=resource_pressure_stop state={:?} decision={:?}",
                            pressure.resource_state, pressure.shutdown_decision);
                        break 'wait TerminalTrigger::Pressure;
                    },
                    _ = observability_tick.tick() => {
                        if let Some(observability) = observability.as_mut() {
                            if let Err(error) = observability.poll_health() {
                                recorder.set_observability_pipeline(observability.snapshot());
                                eprintln!("runtime observability pipeline failed: {error}");
                            }
                            recorder.set_observability_pipeline(observability.snapshot());
                        }
                    },
                    _ = shepherd_heartbeat.tick() => {
                        let snapshot = recorder.snapshot();
                        if snapshot.components.get(&adl_runtime_kernel::ComponentId::new("shepherd"))
                            == Some(&RunningState::Running)
                        {
                            let observed_at = roster_trusted_time.now_unix_millis();
                            if let Some(deadline) = observed_at
                                .checked_add(AGENT_ADMISSION_HEARTBEAT_TTL_MILLIS)
                                .filter(|_| observed_at > 0)
                            {
                                let _ = recorder.record_agent_heartbeat(
                                    "shepherd",
                                    observed_at,
                                    deadline,
                                );
                            }
                        }
                    },
                    signal = shutdown_signal.recv() => {
                        if let Err(error) = signal {
                            eprintln!("runtime signal handler failed: {error}");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(kernel_shutdown_grace).await;
                            drain_control_api(&mut api, api_drain_timeout).await;
                            drain_private_api(&mut private_api, api_drain_timeout).await;
                            break 'serve ExitCode::from(70);
                        }
                        break 'wait TerminalTrigger::Signal;
                    },
                    request = shutdown_requests.recv() => {
                        let Some(request) = request else {
                            eprintln!("runtime checkpoint shutdown channel closed");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(kernel_shutdown_grace).await;
                            drain_control_api(&mut api, api_drain_timeout).await;
                            drain_private_api(&mut private_api, api_drain_timeout).await;
                            break 'serve ExitCode::from(70);
                        };
                        break 'wait TerminalTrigger::Signed(request);
                    },
                    _ = guardian_lease_lost(&mut guardian_lease) => {
                        eprintln!("event=guardian_lease_lost action=checkpoint_shutdown");
                        break 'wait TerminalTrigger::GuardianLost;
                    },
                    exit = handle.wait_for_exit() => match exit {
                        Ok(exit) => {
                            api_shutdown.cancel();
                            drain_control_api(&mut api, api_drain_timeout).await;
                            drain_private_api(&mut private_api, api_drain_timeout).await;
                            break 'serve process_exit(exit);
                        },
                        Err(error) => {
                            eprintln!("runtime kernel task failed: {error}");
                            api_shutdown.cancel();
                            let _ = handle.shutdown(kernel_shutdown_grace).await;
                            drain_control_api(&mut api, api_drain_timeout).await;
                            drain_private_api(&mut private_api, api_drain_timeout).await;
                            break 'serve ExitCode::from(70);
                        }
                    },
                    result = &mut api => {
                        match result {
                            Ok(Ok(())) => eprintln!("runtime control API stopped unexpectedly"),
                            Ok(Err(error)) => eprintln!("runtime control API failed: {error}"),
                            Err(error) => eprintln!("runtime control API task failed: {error}"),
                        }
                        api_shutdown.cancel();
                        let _ = handle.shutdown(kernel_shutdown_grace).await;
                        drain_private_api(&mut private_api, api_drain_timeout).await;
                        break 'serve ExitCode::from(70);
                    },
                    result = &mut private_api => {
                        match result {
                            Ok(Ok(())) => eprintln!("runtime private continuity API stopped unexpectedly"),
                            Ok(Err(error)) => eprintln!("runtime private continuity API failed: {error}"),
                            Err(error) => eprintln!("runtime private continuity API task failed: {error}"),
                        }
                        api_shutdown.cancel();
                        let _ = handle.shutdown(kernel_shutdown_grace).await;
                        drain_control_api(&mut api, api_drain_timeout).await;
                        break 'serve ExitCode::from(70);
                    },
                    };
                };

                let (label, deadline, grace, retry_pressure, mut request) = match trigger {
                    TerminalTrigger::Pressure => (
                        "pressure",
                        pressure_checkpoint_deadline,
                        kernel_shutdown_grace,
                        true,
                        None,
                    ),
                    TerminalTrigger::Signal => (
                        "signal",
                        standard_checkpoint_deadline,
                        kernel_shutdown_grace,
                        false,
                        None,
                    ),
                    TerminalTrigger::GuardianLost => (
                        "guardian",
                        standard_checkpoint_deadline,
                        kernel_shutdown_grace,
                        false,
                        None,
                    ),
                    TerminalTrigger::Signed(request) => (
                        "signed",
                        standard_checkpoint_deadline,
                        request.grace,
                        false,
                        Some(request),
                    ),
                };
                let terminal_result = service
                    .serialize_terminal_checkpoint(&mut continuity, deadline)
                    .await;
                if let Err(error) = terminal_result {
                    if retry_pressure {
                        eprintln!("runtime pressure continuity checkpoint failed: {error}");
                        if !service.reopen_admission_if_no_terminal() {
                            eprintln!(
                                "event=resource_pressure_wait reason=terminal_request_pending"
                            );
                        }
                        pressure_retry_at = Some(tokio::time::Instant::now() + weather_retry_delay);
                        continue 'serve;
                    }
                    eprintln!("runtime {label} terminal serialization failed: {error}");
                    if let Some(request) = request.take() {
                        request.respond(Err(()));
                    }
                    api_shutdown.cancel();
                    let _ = handle.shutdown(kernel_shutdown_grace).await;
                    drain_control_api(&mut api, api_drain_timeout).await;
                    drain_private_api(&mut private_api, api_drain_timeout).await;
                    break 'serve ExitCode::from(74);
                }

                api_shutdown.cancel();
                let shutdown = handle.shutdown(grace).await;
                let restart_requested = request.as_ref().is_some_and(|request| request.restart);
                let terminal = match shutdown {
                    Ok(exit) => {
                        if let Some(request) = request.take() {
                            request.respond(Ok(exit.clone()));
                        }
                        if restart_requested {
                            ExitCode::from(75)
                        } else {
                            process_exit(exit)
                        }
                    }
                    Err(error) => {
                        eprintln!("runtime {label} shutdown failed: {error}");
                        if let Some(request) = request.take() {
                            request.respond(Err(()));
                        }
                        ExitCode::from(70)
                    }
                };
                drain_control_api(&mut api, api_drain_timeout).await;
                drain_private_api(&mut private_api, api_drain_timeout).await;
                break 'serve terminal;
            };
            let mut observability_shutdown_error = None;
            if let Some(observability) = observability.as_mut() {
                recorder.set_observability_pipeline(observability.snapshot());
                if let Err(error) = observability.shutdown().await {
                    observability_shutdown_error = Some(error.to_string());
                    recorder.set_observability_pipeline(observability.snapshot());
                }
            }
            preserve_runtime_result_after_observability(
                serve_result,
                observability_shutdown_error.as_deref(),
            )
        }
        _ => {
            eprintln!("{}", usage());
            ExitCode::from(64)
        }
    }
}

async fn bind_control_listener(
    socket_addrs: &[std::net::SocketAddr],
    attempts: u32,
    retry_delay: std::time::Duration,
) -> std::io::Result<tokio::net::TcpListener> {
    let mut last_error = None;
    for attempt in 1..=attempts {
        match bind_control_listener_once(socket_addrs) {
            Ok(listener) => return Ok(listener),
            Err(error) if error.kind() == std::io::ErrorKind::AddrInUse && attempt < attempts => {
                last_error = Some(error);
                tokio::time::sleep(retry_delay).await;
            }
            Err(error) => return Err(error),
        }
    }
    Err(last_error.unwrap_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "control API bind attempts must be nonzero",
        )
    }))
}

fn bind_control_listener_once(
    socket_addrs: &[std::net::SocketAddr],
) -> std::io::Result<tokio::net::TcpListener> {
    let mut last_error = None;
    for address in socket_addrs {
        let domain = socket2::Domain::for_address(*address);
        let socket =
            match socket2::Socket::new(domain, socket2::Type::STREAM, Some(socket2::Protocol::TCP))
            {
                Ok(socket) => socket,
                Err(error) => {
                    last_error = Some(error);
                    continue;
                }
            };
        if let Err(error) = socket.set_reuse_address(true) {
            last_error = Some(error);
            continue;
        }
        if let Err(error) = socket.set_nonblocking(true) {
            last_error = Some(error);
            continue;
        }
        if let Err(error) = socket.bind(&socket2::SockAddr::from(*address)) {
            last_error = Some(error);
            continue;
        }
        if let Err(error) = socket.listen(i32::MAX) {
            last_error = Some(error);
            continue;
        }
        let listener = std::net::TcpListener::from(socket);
        return tokio::net::TcpListener::from_std(listener);
    }
    Err(last_error.unwrap_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "no configured control API addresses",
        )
    }))
}

enum TerminalTrigger {
    Pressure,
    Signal,
    GuardianLost,
    Signed(CheckpointShutdownRequest),
}

struct ServeArgs {
    init_path: PathBuf,
}

impl ServeArgs {
    fn parse(args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut init_path = None;
        let mut args = args.peekable();
        while let Some(arg) = args.next() {
            match arg.as_str() {
                "--init" => {
                    if init_path.is_some() {
                        return Err("serve accepts exactly one --init file".to_owned());
                    }
                    let Some(path) = args.next() else {
                        return Err("--init requires a runtime init file path".to_owned());
                    };
                    if path.trim().is_empty() || path.starts_with('-') {
                        return Err("--init requires a runtime init file path".to_owned());
                    }
                    init_path = Some(PathBuf::from(path));
                }
                "--state-root" => {
                    return Err("runtime state_root must be declared inside --init".to_owned());
                }
                "--continuity-root" | "--capsule" => {
                    return Err("runtime continuity is derived below init state_root".to_owned());
                }
                other if other.starts_with('-') => {
                    return Err(format!("unknown serve option: {other}"));
                }
                other => {
                    return Err(format!(
                        "serve does not accept positional argument: {other}"
                    ));
                }
            }
        }
        Ok(Self {
            init_path: init_path.ok_or_else(|| "serve requires --init <file>".to_owned())?,
        })
    }
}

fn usage() -> &'static str {
    "usage: adl-runtime-kernel serve --init <absolute-runtime-init.toml>"
}

fn canonical_init_path(path: &Path) -> std::io::Result<PathBuf> {
    if !path.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "runtime init path must be absolute",
        ));
    }
    let canonical = path.canonicalize()?;
    if canonical.is_file() {
        Ok(canonical)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "runtime init path must name a file",
        ))
    }
}

fn canonical_state_root(path: &Path) -> std::io::Result<PathBuf> {
    if !path.is_absolute() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "runtime state root must be absolute",
        ));
    }
    std::fs::create_dir_all(path)?;
    let canonical = path.canonicalize()?;
    if canonical.is_dir() {
        Ok(canonical)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "runtime state root must name a directory",
        ))
    }
}

fn canonical_configured_child_dir(
    state_root: &Path,
    configured: &Path,
    label: &'static str,
) -> std::io::Result<PathBuf> {
    std::fs::create_dir_all(configured)?;
    let canonical = configured.canonicalize()?;
    if canonical.starts_with(state_root) && canonical.is_dir() {
        Ok(canonical)
    } else {
        Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("{label} escaped runtime state root"),
        ))
    }
}

struct ShutdownSignalReceiver {
    ctrl_c: Pin<Box<dyn Future<Output = std::io::Result<()>>>>,
    #[cfg(unix)]
    terminate: tokio::signal::unix::Signal,
    #[cfg(windows)]
    ctrl_break: tokio::signal::windows::CtrlBreak,
}

impl ShutdownSignalReceiver {
    fn register() -> std::io::Result<Self> {
        Ok(Self {
            ctrl_c: Box::pin(tokio::signal::ctrl_c()),
            #[cfg(unix)]
            terminate: tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())?,
            #[cfg(windows)]
            ctrl_break: tokio::signal::windows::ctrl_break()?,
        })
    }

    async fn recv(&mut self) -> std::io::Result<()> {
        #[cfg(unix)]
        {
            tokio::select! {
                result = &mut self.ctrl_c => result,
                _ = self.terminate.recv() => Ok(()),
            }
        }
        #[cfg(windows)]
        {
            tokio::select! {
                result = &mut self.ctrl_c => result,
                _ = self.ctrl_break.recv() => Ok(()),
            }
        }
        #[cfg(not(any(unix, windows)))]
        {
            (&mut self.ctrl_c).await
        }
    }
}

async fn connect_guardian_lease(
    connect_timeout: std::time::Duration,
    auth_timeout: std::time::Duration,
) -> Result<TcpStream, String> {
    let address = std::env::var(GUARDIAN_LEASE_ADDRESS_ENV).ok();
    let token = std::env::var(GUARDIAN_LEASE_TOKEN_ENV).ok();
    let (address, token) = match (address, token) {
        (Some(address), Some(token)) if !address.is_empty() && !token.is_empty() => {
            (address, token)
        }
        _ => return Err("required lease address and token are missing".to_owned()),
    };
    let parsed = address
        .parse::<std::net::SocketAddr>()
        .map_err(|_| "lease address is invalid".to_owned())?;
    if !parsed.ip().is_loopback() {
        return Err("lease address must be loopback".to_owned());
    }
    let mut stream = tokio::time::timeout(connect_timeout, TcpStream::connect(parsed))
        .await
        .map_err(|_| "lease connection timed out".to_owned())?
        .map_err(|error| format!("lease connection failed: {error}"))?;
    stream
        .write_all(token.as_bytes())
        .await
        .map_err(|error| format!("lease authentication failed: {error}"))?;
    let mut acknowledgement = [0_u8; 2];
    tokio::time::timeout(auth_timeout, stream.read_exact(&mut acknowledgement))
        .await
        .map_err(|_| "lease authentication timed out".to_owned())?
        .map_err(|error| format!("lease authentication failed: {error}"))?;
    if acknowledgement != *b"ok" {
        return Err("lease authentication was refused".to_owned());
    }
    Ok(stream)
}

async fn guardian_lease_lost(lease: &mut TcpStream) {
    let mut unexpected = [0_u8; 1];
    let _ = lease.read(&mut unexpected).await;
}

async fn file_hash(path: &std::path::Path) -> std::io::Result<String> {
    Ok(blake3::hash(&tokio::fs::read(path).await?)
        .to_hex()
        .to_string())
}

async fn read_trimmed_config_file(path: &Path, description: &str) -> Result<String, String> {
    let value = tokio::fs::read_to_string(path)
        .await
        .map_err(|error| format!("{description} could not be read: {error}"))?;
    let value = value.trim().to_owned();
    if value.is_empty() {
        Err(format!("{description} is empty"))
    } else {
        Ok(value)
    }
}

fn process_exit(exit: KernelExit) -> ExitCode {
    match exit {
        KernelExit::Clean => ExitCode::SUCCESS,
        KernelExit::Fatal { component } => {
            eprintln!("classified_fatal_exit:{component}");
            ExitCode::from(70)
        }
        KernelExit::ShutdownFailed { components } => {
            eprintln!("classified_shutdown_failure:{components:?}");
            ExitCode::from(74)
        }
        KernelExit::ShutdownDeadlineExceeded { aborted } => {
            eprintln!("classified_shutdown_deadline:{aborted:?}");
            ExitCode::from(70)
        }
    }
}

fn preserve_runtime_result_after_observability<T>(
    runtime_result: T,
    observability_shutdown_error: Option<&str>,
) -> T {
    if let Some(error) = observability_shutdown_error {
        eprintln!("runtime observability shutdown failed; Runtime result preserved: {error}");
    }
    runtime_result
}

async fn drain_control_api(
    api: &mut tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ControlApiError>>,
    timeout: std::time::Duration,
) {
    if tokio::time::timeout(timeout, &mut *api).await.is_err() {
        api.abort();
    }
}

async fn drain_private_api(
    api: &mut tokio::task::JoinHandle<Result<(), adl_runtime_kernel::ContinuityControlError>>,
    timeout: std::time::Duration,
) {
    if tokio::time::timeout(timeout, &mut *api).await.is_err() {
        api.abort();
    }
}

#[cfg(test)]
mod tests {
    use super::{bind_control_listener, preserve_runtime_result_after_observability};

    #[test]
    fn observability_shutdown_failure_preserves_runtime_result() {
        assert_eq!(
            preserve_runtime_result_after_observability(
                "runtime-clean-exit",
                Some("master_log_drain_incomplete")
            ),
            "runtime-clean-exit"
        );
    }

    #[tokio::test]
    async fn control_listener_rebinds_immediately_after_a_connection_closes() {
        let first = bind_control_listener(
            &["127.0.0.1:0".parse().expect("loopback address")],
            1,
            std::time::Duration::ZERO,
        )
        .await
        .expect("first listener");
        let address = first.local_addr().expect("bound address");
        let client = tokio::net::TcpStream::connect(address);
        let (client, accepted) = tokio::join!(client, first.accept());
        drop(client.expect("client connection"));
        drop(accepted.expect("accepted connection").0);
        drop(first);

        let rebound = bind_control_listener(&[address], 1, std::time::Duration::ZERO)
            .await
            .expect("immediate listener restart");
        assert_eq!(rebound.local_addr().expect("rebound address"), address);
    }

    #[tokio::test]
    async fn control_listener_retries_until_a_live_owner_releases_the_port() {
        let held = std::net::TcpListener::bind("127.0.0.1:0").expect("held listener");
        let address = held.local_addr().expect("held address");
        let release = tokio::spawn(async move {
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
            drop(held);
        });

        let rebound = bind_control_listener(&[address], 10, std::time::Duration::from_millis(5))
            .await
            .expect("listener retry");
        assert_eq!(rebound.local_addr().expect("rebound address"), address);
        release.await.expect("release task");
    }

    #[cfg(windows)]
    #[test]
    fn shutdown_signal_receiver_registers_ctrl_break() {
        use super::ShutdownSignalReceiver;

        let _receiver = ShutdownSignalReceiver::register()
            .expect("Windows Runtime v3 must register CTRL_C and CTRL_BREAK handlers");
    }
}
