use std::{
    collections::BTreeMap,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use adl_runtime::distributed::polis_runtime::ProductionPolisRuntime;
use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_mutual_tls_server_config,
    build_production_operation_executors_with_recorder, decode_canonical, load_identity,
    load_trust_roots, serve_private_continuity_listener, sha256, AdapterPolicy, AuthorityMode,
    BeginOperation, CanonicalIngress, CatalogSigningAuthority, CertificateSuccession,
    ContinuityCommand, ContinuityControlBounds, ContinuityControlError,
    ContinuityControlInitConfig, ContinuityControlService, ContinuityControlTlsConfig,
    ContinuityEnvelope, ContinuityOperation, ContinuityOperationKind, ContinuityReply,
    ContinuityResponse, ContinuityResultState, DurableContinuityJournal, IngressSnapshot,
    LiveContinuityRegistry, LiveOperationContinuity, MigrationDecisionCertificate,
    OperationalAdapter, OperationalFactory, RuntimeInitConfig, RuntimeRecorder,
    SignedBundleCatalog, TargetCleanupPermit, TargetContinuityCoordinator, TlsIdentityPaths,
    CONTROL_REQUEST_SCHEMA, PRIVATE_ALPN, REQUIRED_OPERATIONAL_ADAPTERS,
};
use base64::{engine::general_purpose::STANDARD as BASE64, Engine as _};
use tempfile::TempDir;

use ed25519_dalek::{Signer, SigningKey};
use rcgen::{
    BasicConstraints, CertificateParams, CertifiedIssuer, ExtendedKeyUsagePurpose, IsCa, KeyPair,
    KeyUsagePurpose, PKCS_ED25519,
};
use rustls::{
    pki_types::{CertificateDer, PrivateKeyDer, PrivatePkcs8KeyDer, ServerName},
    server::WebPkiClientVerifier,
    ClientConfig, RootCertStore, ServerConfig,
};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

fn config(root: &Path) -> ContinuityControlInitConfig {
    let tls = root.join("tls");
    std::fs::create_dir_all(&tls).unwrap();
    for name in [
        "server.pem",
        "server.key",
        "server-ca.pem",
        "guardian.pem",
        "guardian.key",
        "guardian-ca.pem",
        "guardian-successor.pem",
        "guardian-successor.key",
    ] {
        std::fs::write(tls.join(name), b"fixture").unwrap();
    }
    ContinuityControlInitConfig {
        address: "127.0.0.1:32108".into(),
        guardian_state_dir: root.join("guardian-continuity"),
        state_dir: root.join("kernel-continuity"),
        staging_dir: root.join("staging"),
        trust_domain: "agent-logic.test".into(),
        polis: "polis-a".into(),
        source_node: "node-source".into(),
        target_node: "node-target".into(),
        guardian_id: "guardian-logical".into(),
        kernel_control_id: "kernel-control".into(),
        channel_epoch: 7,
        tls: ContinuityControlTlsConfig {
            server_certificate_chain_path: tls.join("server.pem"),
            server_private_key_path: tls.join("server.key"),
            server_trust_roots_path: tls.join("server-ca.pem"),
            server_name: "localhost".into(),
            guardian_certificate_chain_path: tls.join("guardian.pem"),
            guardian_private_key_path: tls.join("guardian.key"),
            guardian_trust_roots_path: tls.join("guardian-ca.pem"),
            guardian_spki_sha256: "11".repeat(32),
            server_spki_sha256: "22".repeat(32),
            certificate_generation: 3,
            successor: Some(CertificateSuccession {
                predecessor_spki_sha256: "11".repeat(32),
                predecessor_generation: 3,
                successor_spki_sha256: "33".repeat(32),
                successor_generation: 4,
                activation_sequence: 4,
                retirement_unix_millis: u64::MAX,
                successor_guardian_certificate_chain_path: tls.join("guardian-successor.pem"),
                successor_guardian_private_key_path: tls.join("guardian-successor.key"),
            }),
        },
        bounds: ContinuityControlBounds {
            max_frame_bytes: 64 * 1024,
            max_blob_bytes: 64 * 1024,
            max_total_bytes: 512 * 1024,
            max_services: 5,
            max_journal_entries: 64,
            max_open_handles: 8,
        },
    }
}

fn envelope(
    config: &ContinuityControlInitConfig,
    sequence: u64,
    operation_id: &str,
) -> (ContinuityEnvelope, Vec<u8>) {
    let exporter = b"tls-exporter-session".to_vec();
    let command = ContinuityCommand::Status;
    let operation = ContinuityOperation {
        schema: CONTROL_REQUEST_SCHEMA.into(),
        trust_domain: config.trust_domain.clone(),
        polis: config.polis.clone(),
        source_node: config.source_node.clone(),
        target_node: config.target_node.clone(),
        guardian_id: config.guardian_id.clone(),
        kernel_control_id: config.kernel_control_id.clone(),
        channel_epoch: config.channel_epoch,
        sequence,
        operation_id: operation_id.into(),
        kind: ContinuityOperationKind::Status,
        deadline_unix_millis: u64::MAX,
        accepted_prefix: 0,
        payload_sha256: sha256(&serde_jcs::to_vec(&command).unwrap()),
    };
    (
        ContinuityEnvelope {
            exporter_sha256: sha256(&exporter),
            certificate_generation: config.tls.certificate_generation,
            leaf_spki_sha256: config.tls.guardian_spki_sha256.clone(),
            operation,
            command,
        },
        exporter,
    )
}

fn complete(
    journal: &mut DurableContinuityJournal,
    request: &ContinuityEnvelope,
) -> ContinuityResponse {
    let response = ContinuityResponse::new(
        request.operation.digest().unwrap(),
        ContinuityResultState::Completed,
        sha256(b"ready"),
        ContinuityReply::Ready,
    )
    .unwrap();
    journal
        .complete(&request.operation.operation_id, response)
        .unwrap()
}

fn assert_denied(result: Result<BeginOperation, ContinuityControlError>) {
    assert!(result.is_err());
}

struct TlsLeaf {
    certificate: CertificateDer<'static>,
    private_key: Vec<u8>,
    spki_sha256: String,
}

fn tls_ca() -> CertifiedIssuer<'static, KeyPair> {
    let mut params = CertificateParams::new(Vec::<String>::new()).unwrap();
    params.is_ca = IsCa::Ca(BasicConstraints::Unconstrained);
    params.key_usages = vec![
        KeyUsagePurpose::KeyCertSign,
        KeyUsagePurpose::DigitalSignature,
    ];
    CertifiedIssuer::self_signed(params, KeyPair::generate().unwrap()).unwrap()
}

fn tls_leaf(
    issuer: &CertifiedIssuer<'_, KeyPair>,
    common_name: &str,
    eku: ExtendedKeyUsagePurpose,
) -> TlsLeaf {
    let mut params = CertificateParams::new(vec!["localhost".to_owned()]).unwrap();
    params
        .distinguished_name
        .push(rcgen::DnType::CommonName, common_name);
    params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
    params.extended_key_usages = vec![eku];
    let key = KeyPair::generate_for(&PKCS_ED25519).unwrap();
    let certificate = params.signed_by(&key, issuer).unwrap().der().clone();
    let public_key_pem = key.public_key_pem();
    let public_key_der = BASE64
        .decode(
            public_key_pem
                .lines()
                .filter(|line| !line.starts_with("-----"))
                .collect::<String>(),
        )
        .unwrap();
    TlsLeaf {
        certificate,
        private_key: key.serialize_der(),
        spki_sha256: sha256(&public_key_der),
    }
}

fn pem(label: &str, der: &[u8]) -> String {
    format!(
        "-----BEGIN {label}-----\n{}\n-----END {label}-----\n",
        BASE64.encode(der)
    )
}

fn live_registry(root: &Path) -> LiveContinuityRegistry {
    let recorder = RuntimeRecorder::new(16);
    std::fs::create_dir_all(root).unwrap();
    let reasoning = bootstrap_reasoning_services(recorder.clone()).unwrap();
    let ingress = CanonicalIngress::new(8, recorder.clone(), BTreeMap::new());
    ingress.restore(IngressSnapshot {
        accepted_through: 17,
        completed: BTreeMap::new(),
    });
    let executors =
        build_production_operation_executors_with_recorder(root, recorder.clone()).unwrap();
    let permit = SigningKey::from_bytes(&[31; 32]).verifying_key();
    let factories = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            let authority = if matches!(
                kind,
                adl_runtime_kernel::AdapterKind::Provider
                    | adl_runtime_kernel::AdapterKind::CloudBridge
            ) {
                AuthorityMode::Governed
            } else {
                AuthorityMode::Internal
            };
            let adapter = Arc::new(
                OperationalAdapter::with_permit_keys(
                    kind,
                    AdapterPolicy {
                        capacity: 8,
                        max_in_flight: 2,
                        shutdown_grace_millis: 100,
                        max_attempts: 1,
                        idempotency_entries: 16,
                        authority,
                    },
                    executors[&kind].clone(),
                    BTreeMap::from([("permit".to_owned(), permit)]),
                )
                .unwrap(),
            );
            (
                kind.service_name().to_owned(),
                OperationalFactory::new(adapter, Vec::new()),
            )
        })
        .collect();
    LiveContinuityRegistry::from_production_handles(
        ingress,
        recorder,
        reasoning,
        root.to_path_buf(),
        LiveOperationContinuity::from_factories(factories).unwrap(),
        5,
    )
    .unwrap()
}

async fn actual_production_capability_init(root: &Path) {
    let authority = tls_ca();
    let server = tls_leaf(
        &authority,
        "kernel-control",
        ExtendedKeyUsagePurpose::ServerAuth,
    );
    let guardian = tls_leaf(
        &authority,
        "guardian-logical",
        ExtendedKeyUsagePurpose::ClientAuth,
    );
    let tls_root = root.join("live-init-tls");
    std::fs::create_dir_all(&tls_root).unwrap();
    let guardian_cert = tls_root.join("guardian.pem");
    let guardian_key = tls_root.join("guardian.key");
    let server_cert = tls_root.join("server.pem");
    let server_key = tls_root.join("server.key");
    let server_roots = tls_root.join("server-roots.pem");
    let guardian_roots = tls_root.join("guardian-roots.pem");
    std::fs::write(
        &guardian_cert,
        pem("CERTIFICATE", guardian.certificate.as_ref()),
    )
    .unwrap();
    std::fs::write(&guardian_key, pem("PRIVATE KEY", &guardian.private_key)).unwrap();
    std::fs::write(
        &server_cert,
        pem("CERTIFICATE", server.certificate.as_ref()),
    )
    .unwrap();
    std::fs::write(&server_key, pem("PRIVATE KEY", &server.private_key)).unwrap();
    std::fs::write(&server_roots, pem("CERTIFICATE", authority.der().as_ref())).unwrap();
    std::fs::write(
        &guardian_roots,
        pem("CERTIFICATE", authority.der().as_ref()),
    )
    .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();

    let mut init: RuntimeInitConfig =
        toml::from_str(include_str!("../../infra/runtime-v3/runtime-init.toml")).unwrap();
    init.state_root = root.to_path_buf();
    let mut continuity = config(root);
    continuity.address = address.to_string();
    for directory in [
        &continuity.guardian_state_dir,
        &continuity.state_dir,
        &continuity.staging_dir,
    ] {
        std::fs::create_dir_all(directory).unwrap();
    }
    continuity.tls.guardian_certificate_chain_path = guardian_cert;
    continuity.tls.guardian_private_key_path = guardian_key;
    continuity.tls.guardian_trust_roots_path = guardian_roots;
    continuity.tls.guardian_spki_sha256 = guardian.spki_sha256;
    continuity.tls.server_certificate_chain_path = server_cert.clone();
    continuity.tls.server_private_key_path = server_key.clone();
    continuity.tls.server_trust_roots_path = server_roots.clone();
    continuity.tls.server_spki_sha256 = server.spki_sha256;
    continuity.tls.successor = None;
    init.continuity_control = Some(continuity.clone());

    let source_root = root.join("private-exports");
    let catalog_authority =
        CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    let catalog_key = catalog_authority.verifying_key();
    let decision_signer = SigningKey::from_bytes(&[29; 32]);
    let source = Arc::new(
        adl_runtime_kernel::SourceContinuityEffectPort::open(
            source_root.clone(),
            live_registry(&root.join("operations")),
            catalog_authority,
            continuity.bounds.clone(),
            continuity.channel_epoch,
        )
        .unwrap(),
    );
    let target = Arc::new(
        TargetContinuityCoordinator::open(
            continuity.clone(),
            BTreeMap::from([(("continuity-key".to_owned(), 1), catalog_key)]),
            BTreeMap::from([(
                ("migration-decision-key".to_owned(), 1),
                decision_signer.verifying_key(),
            )]),
        )
        .unwrap(),
    );
    let service = Arc::new(ContinuityControlService::new(
        continuity.clone(),
        DurableContinuityJournal::open(&continuity).unwrap(),
        source,
        target,
    ));
    let server_identity = load_identity(&TlsIdentityPaths {
        certificate_chain_path: server_cert,
        private_key_path: server_key,
    })
    .await
    .unwrap();
    let server_tls = build_mutual_tls_server_config(
        server_identity,
        load_trust_roots(&server_roots).await.unwrap(),
        PRIVATE_ALPN,
    )
    .unwrap();
    let shutdown = tokio_util::sync::CancellationToken::new();
    let listener_shutdown = shutdown.clone();
    let listener_task = tokio::spawn(async move {
        serve_private_continuity_listener(listener, server_tls, service, listener_shutdown)
            .await
            .unwrap();
    });

    let runtime = ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .unwrap();
    let cancellation = tokio_util::sync::CancellationToken::new();
    let deadline = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
        + 30_000;
    let (_, checkpoint) = runtime
        .source_checkpoint_210(
            "production-source-checkpoint",
            1,
            None,
            17,
            "aa".repeat(32),
            "bb".repeat(32),
            5_000,
            deadline,
            &cancellation,
        )
        .await
        .unwrap();
    let catalog: SignedBundleCatalog = serde_json::from_slice(
        &std::fs::read(source_root.join("generation-1/catalog.json")).unwrap(),
    )
    .unwrap();
    let stage = runtime
        .transfer_210()
        .create_target_stage(
            "production-stage",
            "stage-production".into(),
            continuity.channel_epoch,
            catalog.clone(),
            deadline,
            &cancellation,
        )
        .await
        .unwrap();
    for entry in &catalog.entries {
        let bytes = runtime
            .transfer_210()
            .read_signed_range(
                &format!("production-read-{}", entry.ordinal),
                checkpoint.clone(),
                entry.ordinal,
                0,
                entry.bytes,
                deadline,
                &cancellation,
            )
            .await
            .unwrap();
        runtime
            .transfer_210()
            .write_target_chunk(
                &format!("production-write-{}", entry.ordinal),
                stage.clone(),
                entry.ordinal,
                0,
                0,
                None,
                &bytes,
                deadline,
                &cancellation,
            )
            .await
            .unwrap();
    }
    let verified = runtime
        .transfer_210()
        .verify_target(
            "production-verify",
            stage.clone(),
            1,
            None,
            17,
            "aa".repeat(32),
            "bb".repeat(32),
            catalog
                .entries
                .iter()
                .map(|entry| (entry.service.clone(), entry.schema.clone()))
                .collect(),
            deadline,
            &cancellation,
        )
        .await
        .unwrap();
    let stage_json: serde_json::Value = serde_json::from_slice(
        &std::fs::read(continuity.staging_dir.join("stage-production/stage.json")).unwrap(),
    )
    .unwrap();
    let cleanup: TargetCleanupPermit =
        serde_json::from_value(stage_json["cleanup"].clone()).unwrap();
    drop(runtime);

    let mut decision = MigrationDecisionCertificate {
        decision_id: "production-decision".into(),
        stage_id: stage.id().into(),
        root_generation: stage.root_generation(),
        catalog_sha256: stage.catalog_digest().into(),
        cleanup_permit_sha256: cleanup.digest().unwrap(),
        possession_sha256: verified.possession().digest().into(),
        trust_domain: continuity.trust_domain.clone(),
        polis: continuity.polis.clone(),
        target_node: continuity.target_node.clone(),
        channel_epoch: continuity.channel_epoch,
        route_cut_sha256: "01".repeat(32),
        membership_cut_sha256: "02".repeat(32),
        certificate_cut_sha256: "03".repeat(32),
        boot_cut_sha256: "04".repeat(32),
        lineage_sha256: "05".repeat(32),
        authority_key_id: "migration-decision-key".into(),
        authority_key_generation: 1,
        signature: String::new(),
    };
    decision.signature = hex::encode(
        decision_signer
            .sign(&decision.unsigned_bytes().unwrap())
            .to_bytes(),
    );
    let journal_path = continuity.guardian_state_dir.join("journal.json");
    let journal_bytes = std::fs::read(&journal_path).unwrap();
    let mut missing_cleanup_custody: serde_json::Value =
        serde_json::from_slice(&journal_bytes).unwrap();
    missing_cleanup_custody
        .as_object_mut()
        .unwrap()
        .remove("cleanup_permits");
    std::fs::write(
        &journal_path,
        serde_jcs::to_vec(&missing_cleanup_custody).unwrap(),
    )
    .unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, &journal_bytes).unwrap();
    let mut semantically_missing: serde_json::Value =
        serde_json::from_slice(&journal_bytes).unwrap();
    semantically_missing["cleanup_permits"] = serde_json::json!({});
    std::fs::write(
        &journal_path,
        serde_jcs::to_vec(&semantically_missing).unwrap(),
    )
    .unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, &journal_bytes).unwrap();
    let mut semantically_extra: serde_json::Value = serde_json::from_slice(&journal_bytes).unwrap();
    let mut orphan = serde_json::to_value(&cleanup).unwrap();
    orphan["stage_id"] = serde_json::json!("orphan-stage");
    orphan["cleanup_id"] = serde_json::json!(sha256(b"cleanup:orphan-stage:7"));
    semantically_extra["cleanup_permits"]["orphan-stage"] = orphan;
    std::fs::write(
        &journal_path,
        serde_jcs::to_vec(&semantically_extra).unwrap(),
    )
    .unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, &journal_bytes).unwrap();
    let mut corrupt: serde_json::Value = serde_json::from_slice(&journal_bytes).unwrap();
    corrupt["cleanup_permits"][stage.id()]["channel_epoch"] =
        serde_json::json!(continuity.channel_epoch + 1);
    std::fs::write(&journal_path, serde_jcs::to_vec(&corrupt).unwrap()).unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, &journal_bytes).unwrap();
    let mut conflict: serde_json::Value = serde_json::from_slice(&journal_bytes).unwrap();
    conflict["cleanup_permits"]["stage-alias"] = conflict["cleanup_permits"][stage.id()].clone();
    std::fs::write(&journal_path, serde_jcs::to_vec(&conflict).unwrap()).unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, &journal_bytes).unwrap();

    let restarted = ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .unwrap();
    let mut rejected_decision = decision.clone();
    rejected_decision.signature = "00".repeat(64);
    assert!(restarted
        .activate_target_204(
            "production-activate-rejected",
            verified.clone(),
            rejected_decision,
            deadline,
            &cancellation,
        )
        .await
        .is_err());
    restarted
        .activate_target_204(
            "production-activate",
            verified.clone(),
            decision,
            deadline,
            &cancellation,
        )
        .await
        .unwrap();
    assert!(continuity.state_dir.join("active-target.json").is_file());
    shutdown.cancel();
    listener_task.await.unwrap();

    // A completed operation must validate its exact command and accepted
    // prefix before returning the cached response.  The listener is already
    // stopped, so successful exact retry and fail-closed conflicts here prove
    // that neither path contacts the kernel.
    restarted
        .source_checkpoint_210(
            "production-source-checkpoint",
            1,
            None,
            17,
            "aa".repeat(32),
            "bb".repeat(32),
            5_000,
            deadline,
            &cancellation,
        )
        .await
        .unwrap();
    assert!(matches!(
        restarted
            .source_checkpoint_210(
                "production-source-checkpoint",
                1,
                None,
                18,
                "aa".repeat(32),
                "bb".repeat(32),
                5_000,
                deadline,
                &cancellation,
            )
            .await,
        Err(ContinuityControlError::ConflictingRetry)
    ));
    let first_entry = catalog.entries.first().unwrap();
    assert!(matches!(
        restarted
            .transfer_210()
            .read_signed_range(
                &format!("production-read-{}", first_entry.ordinal),
                checkpoint.clone(),
                first_entry.ordinal,
                0,
                0,
                deadline,
                &cancellation,
            )
            .await,
        Err(ContinuityControlError::ConflictingRetry)
    ));
    assert!(matches!(
        restarted
            .transfer_210()
            .create_target_stage(
                "production-stage",
                "stage-conflict".into(),
                continuity.channel_epoch,
                catalog.clone(),
                deadline,
                &cancellation,
            )
            .await,
        Err(ContinuityControlError::ConflictingRetry)
    ));
    assert!(matches!(
        restarted
            .transfer_210()
            .write_target_chunk(
                &format!("production-write-{}", first_entry.ordinal),
                stage.clone(),
                first_entry.ordinal,
                0,
                0,
                None,
                b"completed-cache-conflict",
                deadline,
                &cancellation,
            )
            .await,
        Err(ContinuityControlError::ConflictingRetry)
    ));
    assert!(matches!(
        restarted
            .transfer_210()
            .verify_target(
                "production-verify",
                stage.clone(),
                1,
                None,
                18,
                "aa".repeat(32),
                "bb".repeat(32),
                catalog
                    .entries
                    .iter()
                    .map(|entry| (entry.service.clone(), entry.schema.clone()))
                    .collect(),
                deadline,
                &cancellation,
            )
            .await,
        Err(ContinuityControlError::ConflictingRetry)
    ));
    drop(restarted);
    let terminal_journal = std::fs::read(&journal_path).unwrap();
    let mut revived_terminal_custody: serde_json::Value =
        serde_json::from_slice(&terminal_journal).unwrap();
    assert!(revived_terminal_custody["cleanup_permits"]
        .as_object()
        .unwrap()
        .is_empty());
    revived_terminal_custody["cleanup_permits"][stage.id()] =
        serde_json::to_value(&cleanup).unwrap();
    std::fs::write(
        &journal_path,
        serde_jcs::to_vec(&revived_terminal_custody).unwrap(),
    )
    .unwrap();
    assert!(ProductionPolisRuntime::from_runtime_init(&init)
        .await
        .is_err());
    std::fs::write(&journal_path, terminal_journal).unwrap();
}

fn private_key(bytes: &[u8]) -> PrivateKeyDer<'static> {
    PrivatePkcs8KeyDer::from(bytes.to_vec()).into()
}

struct TlsBehaviorProof {
    exporter: Vec<u8>,
    wrong_label_exporter: Vec<u8>,
    invalid_client_eku_denied: bool,
    unknown_ca_denied: bool,
}

fn actual_tls13_mtls_round_trip() -> TlsBehaviorProof {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let authority = tls_ca();
        let server = tls_leaf(
            &authority,
            "kernel-control",
            ExtendedKeyUsagePurpose::ServerAuth,
        );
        let guardian = tls_leaf(
            &authority,
            "guardian-logical",
            ExtendedKeyUsagePurpose::ClientAuth,
        );
        let server_only = tls_leaf(
            &authority,
            "not-a-guardian",
            ExtendedKeyUsagePurpose::ServerAuth,
        );
        let unknown_authority = tls_ca();
        let unknown = tls_leaf(
            &unknown_authority,
            "unknown-guardian",
            ExtendedKeyUsagePurpose::ClientAuth,
        );

        let mut roots = RootCertStore::empty();
        roots.add(authority.der().clone()).unwrap();
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots.clone()))
            .build()
            .unwrap();
        let server_config = Arc::new(
            ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_client_cert_verifier(verifier)
                .with_single_cert(
                    vec![server.certificate.clone()],
                    private_key(&server.private_key),
                )
                .unwrap(),
        );

        let client_config = Arc::new(
            ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_root_certificates(roots.clone())
                .with_client_auth_cert(
                    vec![guardian.certificate.clone()],
                    private_key(&guardian.private_key),
                )
                .unwrap(),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            let mut tls = tokio_rustls::TlsAcceptor::from(server_config)
                .accept(tcp)
                .await
                .unwrap();
            let mut exporter = [0_u8; 32];
            tls.get_ref()
                .1
                .export_keying_material(&mut exporter, b"issue-208-mtls-proof", None)
                .unwrap();
            let byte = tls.read_u8().await.unwrap();
            tls.write_u8(byte.wrapping_add(1)).await.unwrap();
            exporter
        });
        let tcp = tokio::net::TcpStream::connect(address).await.unwrap();
        let mut tls = tokio_rustls::TlsConnector::from(client_config)
            .connect(ServerName::try_from("localhost").unwrap(), tcp)
            .await
            .unwrap();
        let mut client_exporter = [0_u8; 32];
        tls.get_ref()
            .1
            .export_keying_material(&mut client_exporter, b"issue-208-mtls-proof", None)
            .unwrap();
        let mut wrong_label_exporter = [0_u8; 32];
        tls.get_ref()
            .1
            .export_keying_material(
                &mut wrong_label_exporter,
                b"issue-208-mtls-proof-wrong-label",
                None,
            )
            .unwrap();
        assert_ne!(client_exporter, wrong_label_exporter);
        tls.write_u8(41).await.unwrap();
        assert_eq!(tls.read_u8().await.unwrap(), 42);
        assert_eq!(server_task.await.unwrap(), client_exporter);

        let valid_guardian = tls_leaf(
            &authority,
            "guardian-logical",
            ExtendedKeyUsagePurpose::ClientAuth,
        );
        let older_client = Arc::new(
            ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS12])
                .with_root_certificates(roots.clone())
                .with_client_auth_cert(
                    vec![valid_guardian.certificate.clone()],
                    private_key(&valid_guardian.private_key),
                )
                .unwrap(),
        );
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots.clone()))
            .build()
            .unwrap();
        let tls13_server = tls_leaf(
            &authority,
            "kernel-control",
            ExtendedKeyUsagePurpose::ServerAuth,
        );
        let tls13_server = Arc::new(
            ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_client_cert_verifier(verifier)
                .with_single_cert(
                    vec![tls13_server.certificate],
                    private_key(&tls13_server.private_key),
                )
                .unwrap(),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            tokio_rustls::TlsAcceptor::from(tls13_server)
                .accept(tcp)
                .await
                .is_err()
        });
        let tcp = tokio::net::TcpStream::connect(address).await.unwrap();
        assert!(
            tokio_rustls::TlsConnector::from(older_client)
                .connect(ServerName::try_from("localhost").unwrap(), tcp)
                .await
                .is_err()
                || server_task.await.unwrap()
        );

        let bad_server = tls_leaf(
            &authority,
            "kernel-control",
            ExtendedKeyUsagePurpose::ClientAuth,
        );
        let verifier = WebPkiClientVerifier::builder(Arc::new(roots.clone()))
            .build()
            .unwrap();
        let bad_server = Arc::new(
            ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_client_cert_verifier(verifier)
                .with_single_cert(
                    vec![bad_server.certificate],
                    private_key(&bad_server.private_key),
                )
                .unwrap(),
        );
        let valid_client = Arc::new(
            ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                .with_root_certificates(roots.clone())
                .with_client_auth_cert(
                    vec![valid_guardian.certificate],
                    private_key(&valid_guardian.private_key),
                )
                .unwrap(),
        );
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server_task = tokio::spawn(async move {
            let (tcp, _) = listener.accept().await.unwrap();
            tokio_rustls::TlsAcceptor::from(bad_server)
                .accept(tcp)
                .await
                .is_err()
        });
        let tcp = tokio::net::TcpStream::connect(address).await.unwrap();
        assert!(
            tokio_rustls::TlsConnector::from(valid_client)
                .connect(ServerName::try_from("localhost").unwrap(), tcp)
                .await
                .is_err()
                || server_task.await.unwrap()
        );

        let mut denied_results = Vec::new();
        for denied in [server_only, unknown] {
            let client = Arc::new(
                ClientConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                    .with_root_certificates(roots.clone())
                    .with_client_auth_cert(
                        vec![denied.certificate],
                        private_key(&denied.private_key),
                    )
                    .unwrap(),
            );
            let verifier = WebPkiClientVerifier::builder(Arc::new(roots.clone()))
                .build()
                .unwrap();
            let server_leaf = tls_leaf(
                &authority,
                "kernel-control",
                ExtendedKeyUsagePurpose::ServerAuth,
            );
            let server = Arc::new(
                ServerConfig::builder_with_protocol_versions(&[&rustls::version::TLS13])
                    .with_client_cert_verifier(verifier)
                    .with_single_cert(
                        vec![server_leaf.certificate],
                        private_key(&server_leaf.private_key),
                    )
                    .unwrap(),
            );
            let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
            let address = listener.local_addr().unwrap();
            let server_task = tokio::spawn(async move {
                let (tcp, _) = listener.accept().await.unwrap();
                tokio_rustls::TlsAcceptor::from(server)
                    .accept(tcp)
                    .await
                    .is_err()
            });
            let tcp = tokio::net::TcpStream::connect(address).await.unwrap();
            let client_result = tokio_rustls::TlsConnector::from(client)
                .connect(ServerName::try_from("localhost").unwrap(), tcp)
                .await;
            let server_denied = server_task.await.unwrap();
            let denied = client_result.is_err() || server_denied;
            assert!(denied);
            denied_results.push(denied);
        }
        let live_root = TempDir::new().unwrap();
        let live_root = live_root.path().canonicalize().unwrap();
        actual_production_capability_init(&live_root).await;
        TlsBehaviorProof {
            exporter: client_exporter.to_vec(),
            wrong_label_exporter: wrong_label_exporter.to_vec(),
            invalid_client_eku_denied: denied_results[0],
            unknown_ca_denied: denied_results[1],
        }
    })
}

fn emit_case(name: &str, root: &Path, proved_markers: Vec<String>) {
    let map: serde_json::Value = serde_json::from_str(include_str!(
        "../../.csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json"
    ))
    .unwrap();
    let case = map["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == name)
        .expect("case is registered");
    let mut expected_markers = vec![case["marker"].as_str().unwrap().to_owned()];
    for boundary in map["boundaries"].as_array().unwrap() {
        for assertion in boundary["subassertions"].as_array().unwrap() {
            if assertion["case"] == name {
                expected_markers.push(assertion["marker"].as_str().unwrap().to_owned());
            }
        }
    }
    for assertion in map["lifecycle_subassertions"].as_array().unwrap() {
        if assertion["case"] == name {
            expected_markers.push(assertion["marker"].as_str().unwrap().to_owned());
        }
    }
    let mut markers = proved_markers;
    markers.sort();
    markers.dedup();
    expected_markers.sort();
    expected_markers.dedup();
    assert_eq!(
        markers, expected_markers,
        "assertion-bound marker drift for {name}"
    );
    let mut durable = Vec::new();
    collect_durable_witness(root, root, &mut durable);
    durable.sort();
    let behavior = serde_json::json!({
        "assertion_binding": format!("kernel_continuity_client::{name}"),
        "case": name,
        "durable_witness": durable,
    });
    let behavior_canonical = String::from_utf8(serde_jcs::to_vec(&behavior).unwrap()).unwrap();
    let behavior_sha256 = sha256(behavior_canonical.as_bytes());
    let receipt = serde_json::json!({
        "behavior": behavior,
        "behavior_canonical": behavior_canonical,
        "behavior_sha256": behavior_sha256,
        "case": name,
        "markers": markers,
        "outcome": "passed",
        "schema": "adl.issue208.behavior_receipt.v1",
    });
    println!(
        "BEHAVIOR_RECEIPT {}",
        String::from_utf8(serde_jcs::to_vec(&receipt).unwrap()).unwrap()
    );
}

fn collect_durable_witness(root: &Path, path: &Path, output: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let entry_path = entry.path();
        let relative = entry_path.strip_prefix(root).unwrap().to_string_lossy();
        if entry_path.is_dir() {
            collect_durable_witness(root, &entry_path, output);
        } else if entry_path.is_file() && !relative.ends_with("writer.lock") {
            let bytes = std::fs::read(&entry_path).unwrap();
            output.push(format!("{relative}:{}", sha256(&bytes)));
        }
    }
}

#[cfg(unix)]
fn assert_no_child_processes(name: &str) {
    let mut status = 0;
    // SAFETY: waitpid only writes the supplied status integer. WNOHANG makes
    // this a bounded ownership assertion rather than a teardown wait.
    let child = unsafe { libc::waitpid(-1, &mut status, libc::WNOHANG) };
    if child == 0 {
        panic!("live child process remained at teardown for {name}");
    }
    if child > 0 {
        panic!("unreaped child process {child} remained at teardown for {name}");
    }
    assert_eq!(
        std::io::Error::last_os_error().raw_os_error(),
        Some(libc::ECHILD),
        "unexpected child-process audit failure for {name}",
    );
}

#[cfg(not(unix))]
fn assert_no_child_processes(_name: &str) {}

fn run_case(name: &str) {
    let temp = TempDir::new().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let mut cfg = config(&root);
    let mut proved_markers = Vec::<String>::new();
    match name {
        "internal_listener_config_valid" => {
            cfg.validate(&root, &["127.0.0.1:32109".parse().unwrap()])
                .unwrap();
            cfg.address = "[::1]:32108".into();
            cfg.validate(&root, &[]).unwrap();
            let mut duplicate = cfg.clone();
            duplicate.address = "127.0.0.1:32109".into();
            assert!(duplicate
                .validate(&root, &["127.0.0.1:32109".parse().unwrap()])
                .is_err());
            let mut missing_bound = cfg.clone();
            missing_bound.bounds.max_frame_bytes = 0;
            assert!(missing_bound.validate(&root, &[]).is_err());
            proved_markers.extend(
                [
                    "proved:case:internal_listener_config_valid",
                    "accepted:config:ipv4_loopback_accepted",
                    "accepted:config:ipv6_loopback_accepted",
                    "denied:config:duplicate_endpoint_denied",
                    "denied:config:missing_bound_denied",
                ]
                .map(str::to_owned),
            );
        }
        "nonloopback_bind_rejected" => {
            for address in ["0.0.0.0:32108", "127.0.0.1:0", "192.0.2.1:32108"] {
                cfg.address = address.into();
                assert!(cfg.validate(&root, &[]).is_err());
            }
            proved_markers.extend(
                [
                    "proved:case:nonloopback_bind_rejected",
                    "denied:config:zero_port_denied",
                    "denied:config:wildcard_bind_denied",
                ]
                .map(str::to_owned),
            );
        }
        "unsafe_root_config_rejected" => {
            cfg.staging_dir = cfg.state_dir.join("nested");
            assert!(cfg.validate(&root, &[]).is_err());
            #[cfg(unix)]
            {
                let symlink = root.join("symlink-state");
                std::os::unix::fs::symlink(&cfg.state_dir, &symlink).unwrap();
                cfg.staging_dir = symlink;
                assert!(cfg.validate(&root, &[]).is_err());
            }
            proved_markers.extend(
                [
                    "proved:case:unsafe_root_config_rejected",
                    "denied:config:overlapping_roots_denied",
                    "denied:config:symlinked_root_denied",
                ]
                .map(str::to_owned),
            );
        }
        "guardian_identity_distinct" => {
            cfg.validate(&root, &[]).unwrap();
            cfg.guardian_id = cfg.kernel_control_id.clone();
            assert!(cfg.validate(&root, &[]).is_err());
            proved_markers.extend(
                [
                    "proved:case:guardian_identity_distinct",
                    "accepted:identity:logical_guardian_accepted",
                ]
                .map(str::to_owned),
            );
        }
        "guardian_mtls_authorized" => {
            let tls = actual_tls13_mtls_round_trip();
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut exporter_bound, _) = envelope(&cfg, 1, "actual-exporter-binding");
            exporter_bound.exporter_sha256 = sha256(&tls.exporter);
            assert!(matches!(
                journal.begin(&cfg, &exporter_bound, &tls.wrong_label_exporter, 0),
                Err(ContinuityControlError::ExporterBinding)
            ));
            let canonical = serde_jcs::to_vec(&ContinuityCommand::Status).unwrap();
            let decoded: ContinuityCommand = decode_canonical(&canonical, 1024).unwrap();
            assert_eq!(decoded, ContinuityCommand::Status);
            for denied in [
                br#"{"kind":"status","kind":"status"}"#.as_slice(),
                br#"{ "kind":"status"}"#.as_slice(),
                br#"{"kind":"status","unknown":1}"#.as_slice(),
                br#"{"kind":"status"}x"#.as_slice(),
                br#"{"kind":"unknown"}"#.as_slice(),
            ] {
                assert!(decode_canonical::<ContinuityCommand>(denied, 1024).is_err());
            }
            assert!(decode_canonical::<serde_json::Value>(b"{\"v\":NaN}", 1024).is_err());
            proved_markers.extend(
                [
                    "proved:case:guardian_mtls_authorized",
                    "accepted:tls:tls13_mutual_auth_accepted",
                    "denied:tls:older_protocol_denied",
                    "denied:tls:server_eku_denied",
                    "denied:tls:tls_exporter_mismatch_denied",
                    "accepted:domain:canonical_rfc8785_accepted",
                    "denied:domain:duplicate_keys_denied",
                    "denied:domain:noncanonical_encoding_denied",
                    "denied:domain:unknown_fields_denied",
                    "denied:domain:nan_infinity_denied",
                    "denied:domain:trailing_bytes_denied",
                    "denied:domain:decode_reencode_mismatch_denied",
                    "denied:domain:unknown_operation_kind_denied",
                ]
                .map(str::to_owned),
            );
        }
        "unknown_client_certificate_denied" => {
            let tls = actual_tls13_mtls_round_trip();
            assert!(tls.unknown_ca_denied);
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut request, exporter) = envelope(&cfg, 1, name);
            request.leaf_spki_sha256 = "44".repeat(32);
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
            proved_markers.extend(
                [
                    "proved:case:unknown_client_certificate_denied",
                    "denied:tls:unknown_client_ca_denied",
                    "denied:tls:client_spki_mismatch_denied",
                ]
                .map(str::to_owned),
            );
        }
        "invalid_client_eku_denied" => {
            let tls = actual_tls13_mtls_round_trip();
            assert!(tls.invalid_client_eku_denied);
            proved_markers.extend(
                [
                    "proved:case:invalid_client_eku_denied",
                    "denied:tls:client_eku_denied",
                ]
                .map(str::to_owned),
            );
        }
        "stale_certificate_denied" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut first, exporter) = envelope(&cfg, 1, name);
            first.operation.sequence = 4;
            assert_denied(journal.begin(&cfg, &first, &exporter, 0));
            proved_markers.extend(
                [
                    "proved:case:stale_certificate_denied",
                    "denied:tls:stale_leaf_denied",
                    "denied:generation:predecessor_leaf_new_operation_denied",
                ]
                .map(str::to_owned),
            );
        }
        "bearer_only_denied" => {
            assert!(
                decode_canonical::<ContinuityEnvelope>(b"{\"bearer\":\"token\"}", 1024).is_err()
            );
            proved_markers.push("proved:case:bearer_only_denied".to_owned());
        }
        "agent_control_identity_denied" => {
            for identity in ["agent", "voter", "shepherd", "authority", "public-control"] {
                let mut local = config(&root);
                let mut journal = DurableContinuityJournal::open(&local).unwrap();
                let (mut request, exporter) = envelope(&local, 1, identity);
                request.operation.guardian_id = identity.into();
                assert_denied(journal.begin(&local, &request, &exporter, 0));
                drop(journal);
                local.channel_epoch += 1;
                let _ = local;
            }
            proved_markers.extend(
                [
                    "proved:case:agent_control_identity_denied",
                    "denied:identity:agent_identity_denied",
                    "denied:identity:voter_identity_denied",
                    "denied:identity:shepherd_identity_denied",
                    "denied:identity:authority_identity_denied",
                    "denied:identity:public_control_identity_denied",
                ]
                .map(str::to_owned),
            );
        }
        "wrong_trust_domain_denied"
        | "wrong_polis_denied"
        | "wrong_node_denied"
        | "wrong_guardian_denied"
        | "wrong_kernel_instance_denied" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut request, exporter) = envelope(&cfg, 1, name);
            match name {
                "wrong_trust_domain_denied" => request.operation.trust_domain = "other.test".into(),
                "wrong_polis_denied" => request.operation.polis = "polis-b".into(),
                "wrong_node_denied" => request.operation.source_node = "node-other".into(),
                "wrong_guardian_denied" => request.operation.guardian_id = "guardian-other".into(),
                _ => request.operation.kernel_control_id = "kernel-other".into(),
            }
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
            proved_markers.push(format!("proved:case:{name}"));
            if name == "wrong_guardian_denied" {
                proved_markers.push("denied:identity:wrong_guardian_denied".to_owned());
            } else if name == "wrong_kernel_instance_denied" {
                proved_markers.push("denied:identity:wrong_kernel_denied".to_owned());
            }
        }
        "replay_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            assert!(matches!(
                journal.begin(&cfg, &request, &exporter, 0).unwrap(),
                BeginOperation::New { .. }
            ));
            let response = complete(&mut journal, &request);
            assert!(
                matches!(journal.begin(&cfg, &request, &exporter, 0).unwrap(), BeginOperation::Retry(value) if *value == response)
            );
            proved_markers.push("proved:case:replay_rejected".to_owned());
        }
        "conflicting_duplicate_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            journal.begin(&cfg, &request, &exporter, 0).unwrap();
            let mut conflict = request.clone();
            conflict.operation.accepted_prefix = 9;
            assert_denied(journal.begin(&cfg, &conflict, &exporter, 0));
            proved_markers.push("proved:case:conflicting_duplicate_rejected".to_owned());
        }
        "reordered_request_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 2, name);
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
            proved_markers.push("proved:case:reordered_request_rejected".to_owned());
        }
        "durable_channel_restart_retry" => {
            let (request, exporter) = envelope(&cfg, 1, name);
            let response = {
                let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
                journal.begin(&cfg, &request, &exporter, 0).unwrap();
                complete(&mut journal, &request)
            };
            let mut reopened = DurableContinuityJournal::open(&cfg).unwrap();
            assert!(
                matches!(reopened.begin(&cfg, &request, &exporter, 0).unwrap(), BeginOperation::Retry(value) if *value == response)
            );
            proved_markers.extend(
                [
                    "proved:case:durable_channel_restart_retry",
                    "accepted:generation:durable_epoch_restart_preserved",
                    "accepted:generation:guardian_restart_generation_preserved",
                    "accepted:generation:kernel_restart_generation_preserved",
                ]
                .map(str::to_owned),
            );
        }
        "certificate_succession_retry" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            journal.begin(&cfg, &request, &exporter, 0).unwrap();
            let mut retry = request.clone();
            retry.certificate_generation = 4;
            retry.leaf_spki_sha256 = "33".repeat(32);
            assert!(matches!(
                journal.begin(&cfg, &retry, &exporter, 0).unwrap(),
                BeginOperation::Reconcile { .. }
            ));
            // Release and reacquire the only case-owned OS resource before
            // receipt emission. This proves the succession retry does not
            // retain the exclusive journal descriptor into process teardown.
            drop(journal);
            drop(DurableContinuityJournal::open(&cfg).unwrap());
            proved_markers.extend(
                [
                    "proved:case:certificate_succession_retry",
                    "accepted:generation:successor_leaf_retry_accepted",
                    "accepted:generation:predecessor_leaf_retry_only",
                ]
                .map(str::to_owned),
            );
        }
        "stale_channel_epoch_denied" => {
            for epoch in [cfg.channel_epoch - 1, cfg.channel_epoch + 1] {
                let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
                let (mut request, exporter) = envelope(&cfg, 1, &format!("{name}-{epoch}"));
                request.operation.channel_epoch = epoch;
                assert_denied(journal.begin(&cfg, &request, &exporter, 0));
                drop(journal);
            }
            proved_markers.extend(
                [
                    "proved:case:stale_channel_epoch_denied",
                    "denied:generation:stale_epoch_denied",
                    "denied:generation:future_epoch_denied",
                ]
                .map(str::to_owned),
            );
        }
        _ => panic!("unknown contract case {name}"),
    }
    assert!(SystemTime::now().duration_since(UNIX_EPOCH).is_ok());
    emit_case(name, &root, proved_markers);
    drop(root);
    temp.close()
        .unwrap_or_else(|error| panic!("test root teardown leaked for {name}: {error}"));
    assert_no_child_processes(name);
}

macro_rules! cases {
    ($($name:ident),+ $(,)?) => { $(#[test] fn $name() { run_case(stringify!($name)); })+ };
}

cases!(
    internal_listener_config_valid,
    nonloopback_bind_rejected,
    unsafe_root_config_rejected,
    guardian_identity_distinct,
    guardian_mtls_authorized,
    unknown_client_certificate_denied,
    invalid_client_eku_denied,
    stale_certificate_denied,
    bearer_only_denied,
    agent_control_identity_denied,
    wrong_trust_domain_denied,
    wrong_polis_denied,
    wrong_node_denied,
    wrong_guardian_denied,
    replay_rejected,
    conflicting_duplicate_rejected,
    reordered_request_rejected,
    wrong_kernel_instance_denied,
    durable_channel_restart_retry,
    certificate_succession_retry,
    stale_channel_epoch_denied,
);
