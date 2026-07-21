use std::{
    collections::{BTreeMap, BTreeSet},
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Mutex},
    time::Duration,
};

use adl_runtime_kernel::{
    ActuationShell, AdapterKind, AdapterPolicy, Aee, AuthorityGrant, AuthorityMode,
    CanonicalIngress, Commitment, ComponentRegistry, DomainWork, ExecutorError, FailureClass,
    FreedomGate, GovernanceKeys, GovernedActionRequest, Kernel, MediationDecision,
    OperationExecutor, OperationRequest, OperationalAdapter, OperationalFactory, RefusalReason,
    RuntimeRecorder, TrustedGovernanceTime, DOMAIN_WORK_SCHEMA, OPERATION_REQUEST_SCHEMA,
};
use ed25519_dalek::{SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};

const STATE_SCHEMA: &str = "adl.runtime.parity_c.state.v2";
const OUTPUT_SCHEMA: &str = "adl.runtime.parity_c.outcome.v2";

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct GovernedCommand {
    pub request_id: String,
    pub idempotency_key: String,
    pub citizen_id: String,
    pub agent_id: String,
    pub action: String,
    pub resource: String,
    pub units: u64,
    pub payload: String,
    pub commitment: Commitment,
    pub authority_chain: Vec<AuthorityGrant>,
    #[serde(default)]
    pub read_citizen_id: Option<String>,
    #[serde(default)]
    pub cancelled: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct GovernedOutcome {
    pub schema: String,
    pub request_id: String,
    pub citizen_id: String,
    pub status: String,
    pub classification: String,
    pub result_hash: Option<String>,
    pub checkpoint_generation: u64,
    pub actuation_count: u64,
    pub adapters: Vec<String>,
    pub gate_before_actuation: bool,
    pub lifelog_authoritative: bool,
    pub private_payload_retained: bool,
}

#[derive(Clone, Debug)]
pub struct RuntimeConfig {
    state_dir: PathBuf,
    tool_root: PathBuf,
    policy_hash: String,
    policy_key: VerifyingKey,
    authority_key: VerifyingKey,
    authority_principal: String,
    permit_key: [u8; 32],
    checkpoint_key: [u8; 32],
    trusted_time_millis: u64,
    provider_program: PathBuf,
    provider_condition: String,
    revoked_commitments: BTreeSet<String>,
}

impl RuntimeConfig {
    pub fn from_env() -> Result<Self, String> {
        let state_dir = PathBuf::from(required_env("ADL_PARITY_C_STATE_DIR")?);
        let tool_root = PathBuf::from(required_env("ADL_PARITY_C_TOOL_ROOT")?)
            .canonicalize()
            .map_err(|_| "tool_root_unavailable".to_owned())?;
        let provider_program = PathBuf::from(required_env("ADL_PARITY_C_PROVIDER_PROGRAM")?);
        if !provider_program.is_absolute() {
            return Err("provider_program_must_be_absolute".to_owned());
        }
        Ok(Self {
            state_dir,
            tool_root,
            policy_hash: required_env("ADL_PARITY_C_POLICY_HASH")?,
            policy_key: public_env("ADL_PARITY_C_POLICY_PUBLIC_KEY_HEX")?,
            authority_key: public_env("ADL_PARITY_C_AUTHORITY_PUBLIC_KEY_HEX")?,
            authority_principal: required_env("ADL_PARITY_C_AUTHORITY_PRINCIPAL")?,
            permit_key: secret_env("ADL_PARITY_C_PERMIT_KEY_HEX")?,
            checkpoint_key: secret_env("ADL_PARITY_C_CHECKPOINT_KEY_HEX")?,
            trusted_time_millis: required_env("ADL_PARITY_C_TRUSTED_TIME_MILLIS")?
                .parse()
                .map_err(|_| "invalid_trusted_time".to_owned())?,
            provider_program,
            provider_condition: std::env::var("ADL_PARITY_C_PROVIDER_CONDITION")
                .unwrap_or_else(|_| "healthy".to_owned()),
            revoked_commitments: std::env::var("ADL_PARITY_C_REVOKED_COMMITMENTS")
                .unwrap_or_default()
                .split(',')
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .collect(),
        })
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct RuntimeState {
    schema: String,
    generation: u64,
    last_time: u64,
    actuation_count: u64,
    shutdown: bool,
    completed: BTreeMap<String, PersistedOutcome>,
    request_ids: BTreeSet<String>,
    pending_requests: BTreeSet<String>,
    private_state: BTreeMap<String, String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
struct PersistedOutcome {
    request_id: String,
    citizen_id: String,
    result_hash: String,
    generation: u64,
    actuation_count: u64,
}

#[derive(Deserialize, Serialize)]
struct SignedState {
    state: RuntimeState,
    integrity: String,
}

struct QualifiedTime(u64);
impl TrustedGovernanceTime for QualifiedTime {
    fn now_unix_millis(&self) -> u64 {
        self.0
    }
}

struct StateLock(PathBuf);
impl StateLock {
    fn acquire(config: &RuntimeConfig) -> Result<Self, String> {
        std::fs::create_dir_all(&config.state_dir)
            .map_err(|_| "checkpoint_unavailable".to_owned())?;
        let path = config.state_dir.join("checkpoint.lock");
        std::fs::create_dir(&path).map_err(|_| "checkpoint_busy".to_owned())?;
        Ok(Self(path))
    }
}
impl Drop for StateLock {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir(&self.0);
    }
}

struct Passthrough;
#[async_trait::async_trait]
impl OperationExecutor for Passthrough {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        Ok(request.payload.clone())
    }
}

struct ProviderPort {
    program: PathBuf,
    condition: String,
}
#[async_trait::async_trait]
impl OperationExecutor for ProviderPort {
    async fn execute(&self, request: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        if self.condition != "healthy" {
            return Err(ExecutorError {
                class: if matches!(self.condition.as_str(), "timeout" | "unavailable") {
                    FailureClass::Retryable
                } else {
                    FailureClass::Fatal
                },
                message: format!("provider_{}", self.condition),
            });
        }
        let mut child = Command::new(&self.program)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|_| executor_error("provider_unavailable"))?;
        child
            .stdin
            .take()
            .ok_or_else(|| executor_error("provider_unavailable"))?
            .write_all(&request.payload)
            .map_err(|_| executor_error("provider_unavailable"))?;
        let output = child
            .wait_with_output()
            .map_err(|_| executor_error("provider_unavailable"))?;
        if !output.status.success() || output.stdout.len() > 1_048_576 {
            return Err(executor_error("provider_malformed_output"));
        }
        Ok(output.stdout)
    }
}

#[derive(Clone)]
struct ToolPort(PathBuf);
impl ToolPort {
    fn execute(&self, payload: &str) -> Result<Vec<u8>, String> {
        let requested = Path::new(payload);
        if requested.is_absolute() {
            return Err("tool_path_not_allowlisted".to_owned());
        }
        let resolved = self
            .0
            .join(requested)
            .canonicalize()
            .map_err(|_| "tool_unavailable".to_owned())?;
        if !resolved.starts_with(&self.0) {
            return Err("tool_path_not_allowlisted".to_owned());
        }
        let metadata = std::fs::metadata(resolved).map_err(|_| "tool_unavailable".to_owned())?;
        Ok(format!("bytes={}", metadata.len()).into_bytes())
    }
}

struct GovernedExecutor {
    command: GovernedCommand,
    permit: adl_runtime_kernel::ExecutionPermit,
    permit_key: VerifyingKey,
    provider: Arc<OperationalAdapter>,
    scheduler: Arc<OperationalAdapter>,
    shepherd: Arc<OperationalAdapter>,
    tool: ToolPort,
    failure: Arc<Mutex<Option<String>>>,
}

#[async_trait::async_trait]
impl OperationExecutor for GovernedExecutor {
    async fn execute(&self, _: &OperationRequest) -> Result<Vec<u8>, ExecutorError> {
        let shell = Arc::new(ProductionShell {
            command: self.command.clone(),
            provider: self.provider.clone(),
            scheduler: self.scheduler.clone(),
            shepherd: self.shepherd.clone(),
            tool: self.tool.clone(),
        });
        let aee = Aee::new(
            BTreeMap::from([("permit".to_owned(), self.permit_key)]),
            shell,
        );
        let recorded = aee
            .actuate(&self.permit)
            .await
            .map_err(|_| executor_error("actuation_rejected"))?;
        if recorded.success {
            Ok(recorded.result_bytes)
        } else {
            let classification = std::str::from_utf8(&recorded.result_bytes)
                .unwrap_or("actuation_quarantined")
                .to_owned();
            *self.failure.lock().expect("failure mutex poisoned") = Some(classification.clone());
            Err(executor_error(&classification))
        }
    }
}

struct ProductionShell {
    command: GovernedCommand,
    provider: Arc<OperationalAdapter>,
    scheduler: Arc<OperationalAdapter>,
    shepherd: Arc<OperationalAdapter>,
    tool: ToolPort,
}
#[async_trait::async_trait]
impl ActuationShell for ProductionShell {
    async fn execute(
        &self,
        permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        let request = |kind: &str, payload: Vec<u8>, governed: bool| OperationRequest {
            schema: OPERATION_REQUEST_SCHEMA.to_owned(),
            request_id: if governed {
                self.command.request_id.clone()
            } else {
                format!("{}-{kind}", self.command.request_id)
            },
            idempotency_key: if governed {
                self.command.idempotency_key.clone()
            } else {
                format!("{}-{kind}", self.command.idempotency_key)
            },
            principal: self.command.citizen_id.clone(),
            payload,
            permit: governed.then(|| permit.clone()),
        };
        if self.command.cancelled {
            return Err("scheduler_cancelled".to_owned());
        }
        self.shepherd
            .invoke(request(
                "shepherd",
                self.command.agent_id.as_bytes().to_vec(),
                false,
            ))
            .await
            .map_err(classify_operation)?;
        self.scheduler
            .invoke(request("scheduler", Vec::new(), false))
            .await
            .map_err(classify_operation)?;
        match self.command.action.as_str() {
            "provider.invoke" => self
                .provider
                .invoke(request(
                    "provider",
                    self.command.payload.as_bytes().to_vec(),
                    true,
                ))
                .await
                .map(|result| result.payload)
                .map_err(classify_operation),
            "tool.file_metadata" => self.tool.execute(&self.command.payload),
            "system.shutdown" => Ok(b"shutdown_checkpointed".to_vec()),
            _ => Err("unsupported_governed_action".to_owned()),
        }
    }
}

pub async fn execute(config: RuntimeConfig, command: GovernedCommand) -> GovernedOutcome {
    let request_id = command.request_id.clone();
    let citizen_id = command.citizen_id.clone();
    match execute_inner(&config, &command).await {
        Ok(outcome) => outcome,
        Err((classification, state)) => GovernedOutcome {
            schema: OUTPUT_SCHEMA.to_owned(),
            request_id,
            citizen_id,
            status: "refused".to_owned(),
            classification,
            result_hash: None,
            checkpoint_generation: state.generation,
            actuation_count: state.actuation_count,
            adapters: adapter_inventory(),
            gate_before_actuation: true,
            lifelog_authoritative: false,
            private_payload_retained: false,
        },
    }
}

async fn execute_inner(
    config: &RuntimeConfig,
    command: &GovernedCommand,
) -> Result<GovernedOutcome, (String, RuntimeState)> {
    let _lock = StateLock::acquire(config).map_err(|error| (error, RuntimeState::default()))?;
    let mut state = load_state(config).map_err(|error| (error, RuntimeState::default()))?;
    let refuse = |reason: &str, state: &RuntimeState| Err((reason.to_owned(), state.clone()));
    if state.shutdown {
        return refuse("admission_closed", &state);
    }
    if !safe_id(&command.request_id)
        || !safe_id(&command.idempotency_key)
        || !safe_id(&command.citizen_id)
        || !safe_id(&command.agent_id)
        || command.units == 0
    {
        return refuse("invalid_request", &state);
    }
    if config.trusted_time_millis <= state.last_time {
        return refuse("unqualified_or_regressing_time", &state);
    }
    if command
        .read_citizen_id
        .as_deref()
        .is_some_and(|subject| subject != command.citizen_id)
    {
        return refuse("cross_identity_denied", &state);
    }
    if config
        .revoked_commitments
        .contains(&command.commitment.commitment_id)
    {
        return refuse("revoked", &state);
    }
    if state.pending_requests.contains(&command.request_id) {
        return refuse("incomplete_recovery_quarantined", &state);
    }
    if let Some(cached) = state.completed.get(&command.idempotency_key) {
        if cached.request_id != command.request_id || cached.citizen_id != command.citizen_id {
            return refuse("idempotency_conflict", &state);
        }
        return Ok(success_outcome(cached, true));
    }
    if state.request_ids.contains(&command.request_id) {
        return refuse("request_replay", &state);
    }

    let permit_signer = SigningKey::from_bytes(&config.permit_key);
    let keys = GovernanceKeys {
        policy: BTreeMap::from([("policy".to_owned(), config.policy_key)]),
        authority: BTreeMap::from([("authority".to_owned(), config.authority_key)]),
        authority_principals: BTreeMap::from([(
            "authority".to_owned(),
            config.authority_principal.clone(),
        )]),
        root_authority_keys: BTreeSet::from(["authority".to_owned()]),
        operator: BTreeMap::new(),
    };
    let gate = FreedomGate::new(
        config.policy_hash.clone(),
        keys,
        "permit",
        permit_signer.clone(),
        Arc::new(QualifiedTime(config.trusted_time_millis)),
        BTreeMap::from([(command.resource.clone(), 8)]),
    )
    .map_err(|_| ("gate_configuration".to_owned(), state.clone()))?;
    let request = GovernedActionRequest {
        request_id: command.request_id.clone(),
        principal: command.citizen_id.clone(),
        action: command.action.clone(),
        resource: command.resource.clone(),
        units: command.units,
        payload_hash: blake3::hash(command.payload.as_bytes())
            .to_hex()
            .to_string(),
        policy_hash: config.policy_hash.clone(),
        commitment: command.commitment.clone(),
        authority_chain: command.authority_chain.clone(),
    };
    let permit = match gate.mediate(&request) {
        MediationDecision::Allowed(permit) => permit,
        MediationDecision::Refused(evidence) => {
            return refuse(refusal_classification(evidence.reason), &state)
        }
    };

    state.generation += 1;
    state.last_time = config.trusted_time_millis;
    state.pending_requests.insert(command.request_id.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;

    let policy = |authority| AdapterPolicy {
        capacity: 2,
        max_in_flight: 1,
        timeout_millis: 2_000,
        max_attempts: 1,
        idempotency_entries: 64,
        authority,
    };
    let scheduler = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Scheduler,
            policy(AuthorityMode::Internal),
            Arc::new(Passthrough),
        )
        .map_err(|_| ("scheduler_configuration".to_owned(), state.clone()))?,
    );
    let shepherd = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Shepherd,
            policy(AuthorityMode::Internal),
            Arc::new(Passthrough),
        )
        .map_err(|_| ("shepherd_configuration".to_owned(), state.clone()))?,
    );
    let provider = Arc::new(
        OperationalAdapter::with_permit_keys(
            AdapterKind::Provider,
            policy(AuthorityMode::Governed),
            Arc::new(ProviderPort {
                program: config.provider_program.clone(),
                condition: config.provider_condition.clone(),
            }),
            BTreeMap::from([("permit".to_owned(), permit_signer.verifying_key())]),
        )
        .map_err(|_| ("provider_configuration".to_owned(), state.clone()))?,
    );
    let failure = Arc::new(Mutex::new(None));
    let executor = Arc::new(GovernedExecutor {
        command: command.clone(),
        permit,
        permit_key: permit_signer.verifying_key(),
        provider: provider.clone(),
        scheduler: scheduler.clone(),
        shepherd: shepherd.clone(),
        tool: ToolPort(config.tool_root.clone()),
        failure: failure.clone(),
    });
    let agent = Arc::new(
        OperationalAdapter::new(
            AdapterKind::Agent,
            policy(AuthorityMode::Internal),
            executor,
        )
        .map_err(|_| ("agent_configuration".to_owned(), state.clone()))?,
    );
    let agent_factory = OperationalFactory::new(agent, vec![]);
    let ingress = CanonicalIngress::new(
        2,
        RuntimeRecorder::new(64),
        BTreeMap::from([("governed".to_owned(), agent_factory.clone())]),
    );
    let mut components = ComponentRegistry::new();
    components.register(agent_factory);
    components.register(OperationalFactory::new(scheduler, vec![]));
    components.register(OperationalFactory::new(shepherd, vec![]));
    components.register(OperationalFactory::new(provider, vec![]));
    components.register(ingress.clone());
    let topology = components
        .validate()
        .map_err(|_| ("topology_invalid".to_owned(), state.clone()))?;
    let kernel = Kernel::new(topology, RuntimeRecorder::new(64))
        .start()
        .await
        .map_err(|_| ("kernel_start_failed".to_owned(), state.clone()))?;
    let result = ingress
        .submit(
            DomainWork {
                schema: DOMAIN_WORK_SCHEMA.to_owned(),
                work_id: command.request_id.clone(),
                kind: "governed".to_owned(),
                payload: serde_json::to_vec(command).unwrap_or_default(),
            },
            command.request_id.clone(),
        )
        .await;
    ingress.close();
    let _ = kernel.shutdown(Duration::from_secs(2)).await;
    let result_hash = match result {
        Ok(result) => result.result_hash,
        Err(_) => {
            state.pending_requests.remove(&command.request_id);
            state.generation += 1;
            persist_state(config, &state).map_err(|error| (error, state.clone()))?;
            let classification = failure
                .lock()
                .expect("failure mutex poisoned")
                .clone()
                .filter(|value| {
                    value != "actuation_rejected"
                        || (config.provider_condition == "healthy" && !command.cancelled)
                })
                .unwrap_or_else(|| classify_configured_failure(config, command).to_owned());
            return refuse(&classification, &state);
        }
    };

    state.actuation_count += 1;
    state.generation += 1;
    state.pending_requests.remove(&command.request_id);
    state.request_ids.insert(command.request_id.clone());
    let scope = format!(
        "{}|{}|{}|{}",
        command.citizen_id, command.action, command.resource, command.commitment.commitment_id
    );
    state.private_state.insert(
        scope,
        blake3::keyed_hash(&config.checkpoint_key, command.payload.as_bytes())
            .to_hex()
            .to_string(),
    );
    if command.action == "system.shutdown" {
        state.shutdown = true;
    }
    let persisted = PersistedOutcome {
        request_id: command.request_id.clone(),
        citizen_id: command.citizen_id.clone(),
        result_hash,
        generation: state.generation,
        actuation_count: state.actuation_count,
    };
    state
        .completed
        .insert(command.idempotency_key.clone(), persisted.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;
    let _ = append_lifelog(config, command, &persisted);
    Ok(success_outcome(&persisted, false))
}

fn success_outcome(persisted: &PersistedOutcome, replay: bool) -> GovernedOutcome {
    GovernedOutcome {
        schema: OUTPUT_SCHEMA.to_owned(),
        request_id: persisted.request_id.clone(),
        citizen_id: persisted.citizen_id.clone(),
        status: "completed".to_owned(),
        classification: if replay {
            "idempotent_replay"
        } else {
            "success"
        }
        .to_owned(),
        result_hash: Some(persisted.result_hash.clone()),
        checkpoint_generation: persisted.generation,
        actuation_count: persisted.actuation_count,
        adapters: adapter_inventory(),
        gate_before_actuation: true,
        lifelog_authoritative: false,
        private_payload_retained: false,
    }
}

fn adapter_inventory() -> Vec<String> {
    [
        "canonical_ingress",
        "freedom_gate_ed25519",
        "aee",
        "resident_agent",
        "resident_shepherd",
        "bounded_scheduler",
        "external_process_provider",
        "canonical_allowlisted_file_metadata_tool",
        "capability_scoped_authenticated_checkpoint",
        "redacted_append_only_lifelog",
        "trusted_monotonic_time",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn load_state(config: &RuntimeConfig) -> Result<RuntimeState, String> {
    let path = config.state_dir.join("checkpoint.json");
    if !path.exists() {
        return Ok(RuntimeState {
            schema: STATE_SCHEMA.to_owned(),
            ..RuntimeState::default()
        });
    }
    let signed: SignedState = serde_json::from_slice(
        &std::fs::read(path).map_err(|_| "checkpoint_unavailable".to_owned())?,
    )
    .map_err(|_| "checkpoint_corrupt".to_owned())?;
    if signed.state.schema != STATE_SCHEMA
        || state_integrity(&signed.state, &config.checkpoint_key)? != signed.integrity
    {
        return Err("checkpoint_authentication_failed".to_owned());
    }
    Ok(signed.state)
}

fn persist_state(config: &RuntimeConfig, state: &RuntimeState) -> Result<(), String> {
    let signed = SignedState {
        state: state.clone(),
        integrity: state_integrity(state, &config.checkpoint_key)?,
    };
    let tmp = config
        .state_dir
        .join(format!("checkpoint.{}.tmp", std::process::id()));
    let mut file = std::fs::File::create(&tmp).map_err(|_| "checkpoint_unavailable".to_owned())?;
    file.write_all(&serde_json::to_vec(&signed).map_err(|_| "checkpoint_encoding".to_owned())?)
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    file.sync_all()
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    std::fs::rename(tmp, config.state_dir.join("checkpoint.json"))
        .map_err(|_| "checkpoint_unavailable".to_owned())?;
    std::fs::File::open(&config.state_dir)
        .and_then(|directory| directory.sync_all())
        .map_err(|_| "checkpoint_unavailable".to_owned())
}

fn append_lifelog(
    config: &RuntimeConfig,
    command: &GovernedCommand,
    outcome: &PersistedOutcome,
) -> Result<(), String> {
    let entry = serde_json::json!({
        "schema": "adl.runtime.parity_c.lifelog.v1",
        "request_id": command.request_id,
        "citizen_id": command.citizen_id,
        "action": command.action,
        "result_hash": outcome.result_hash,
        "checkpoint_generation": outcome.generation,
        "redacted_fields": ["payload", "keys"]
    });
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(config.state_dir.join("lifelog.jsonl"))
        .map_err(|_| "lifelog_unavailable".to_owned())?;
    writeln!(file, "{entry}").map_err(|_| "lifelog_unavailable".to_owned())?;
    file.sync_data()
        .map_err(|_| "lifelog_unavailable".to_owned())
}

fn state_integrity(state: &RuntimeState, key: &[u8; 32]) -> Result<String, String> {
    let bytes = serde_json::to_vec(state).map_err(|_| "checkpoint_encoding".to_owned())?;
    Ok(blake3::keyed_hash(key, &bytes).to_hex().to_string())
}

fn classify_operation(error: adl_runtime_kernel::OperationError) -> String {
    let message = error.to_string();
    [
        "provider_timeout",
        "provider_auth",
        "provider_quota",
        "provider_malformed_output",
        "provider_unavailable",
    ]
    .into_iter()
    .find(|code| message.contains(code))
    .unwrap_or("actuation_rejected")
    .to_owned()
}

fn classify_configured_failure(config: &RuntimeConfig, command: &GovernedCommand) -> &'static str {
    if command.cancelled {
        "scheduler_cancelled"
    } else {
        match config.provider_condition.as_str() {
            "timeout" => "provider_timeout",
            "auth" => "provider_auth",
            "quota" => "provider_quota",
            "malformed" => "provider_malformed_output",
            "unavailable" => "provider_unavailable",
            _ => "actuation_rejected",
        }
    }
}

fn refusal_classification(reason: RefusalReason) -> &'static str {
    match reason {
        RefusalReason::InvalidRequest => "invalid_request",
        RefusalReason::InvalidCommitment => "invalid_commitment",
        RefusalReason::MissingAuthority => "missing_authority",
        RefusalReason::InvalidDelegation => "invalid_delegation",
        RefusalReason::Revoked => "revoked",
        RefusalReason::StalePolicy => "stale_policy",
        RefusalReason::ResourceExhausted => "resource_exhausted",
        RefusalReason::Replay => "request_replay",
        RefusalReason::OperatorDenied => "operator_denied",
    }
}

fn executor_error(message: &str) -> ExecutorError {
    ExecutorError {
        class: FailureClass::Fatal,
        message: message.to_owned(),
    }
}

fn safe_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn required_env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|_| format!("missing_{name}"))
}

fn secret_env(name: &str) -> Result<[u8; 32], String> {
    let bytes = hex::decode(required_env(name)?).map_err(|_| format!("invalid_{name}"))?;
    bytes.try_into().map_err(|_| format!("invalid_{name}"))
}

fn public_env(name: &str) -> Result<VerifyingKey, String> {
    VerifyingKey::from_bytes(&secret_env(name)?).map_err(|_| format!("invalid_{name}"))
}
