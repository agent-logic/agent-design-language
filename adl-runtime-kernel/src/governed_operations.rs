use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
    sync::Arc,
};

use adl_runtime_kernel::{
    ActuationShell, Aee, AuthorityGrant, Commitment, FreedomGate, GovernanceKeys,
    GovernedActionRequest, MediationDecision, RefusalReason, TrustedGovernanceTime,
    AUTHORITY_GRANT_SCHEMA, COMMITMENT_SCHEMA,
};
use ed25519_dalek::SigningKey;
use serde::{Deserialize, Serialize};

const ZERO_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const STATE_SCHEMA: &str = "adl.runtime.parity_c.state.v1";
const OUTPUT_SCHEMA: &str = "adl.runtime.parity_c.outcome.v1";

#[derive(Clone, Debug, Deserialize)]
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
    pub qualified_unix_millis: u64,
    #[serde(default)]
    pub read_citizen_id: Option<String>,
    #[serde(default)]
    pub delegate_units: Option<u64>,
    #[serde(default)]
    pub revoke_before_dispatch: bool,
    #[serde(default)]
    pub provider_condition: ProviderCondition,
    #[serde(default)]
    pub lifelog_failure: bool,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProviderCondition {
    #[default]
    Healthy,
    Timeout,
    Auth,
    Quota,
    Malformed,
    Unavailable,
    Cancelled,
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

#[derive(Clone, Debug, Deserialize)]
pub struct RuntimeConfig {
    pub state_dir: PathBuf,
    pub policy_key: [u8; 32],
    pub authority_key: [u8; 32],
    pub permit_key: [u8; 32],
    pub checkpoint_key: [u8; 32],
}

impl RuntimeConfig {
    pub fn from_env() -> Result<Self, String> {
        let config = Self {
            state_dir: PathBuf::from(required_env("ADL_PARITY_C_STATE_DIR")?),
            policy_key: secret_env("ADL_PARITY_C_POLICY_KEY_HEX")?,
            authority_key: secret_env("ADL_PARITY_C_AUTHORITY_KEY_HEX")?,
            permit_key: secret_env("ADL_PARITY_C_PERMIT_KEY_HEX")?,
            checkpoint_key: secret_env("ADL_PARITY_C_CHECKPOINT_KEY_HEX")?,
        };
        let distinct = BTreeSet::from([
            config.policy_key,
            config.authority_key,
            config.permit_key,
            config.checkpoint_key,
        ]);
        if distinct.len() != 4 {
            return Err("parity_c_keys_must_be_distinct".to_owned());
        }
        Ok(config)
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
    #[serde(default)]
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

struct ProductionShell {
    command: GovernedCommand,
}

#[async_trait::async_trait]
impl ActuationShell for ProductionShell {
    async fn execute(
        &self,
        _permit: &adl_runtime_kernel::ExecutionPermit,
    ) -> Result<Vec<u8>, String> {
        match self.command.provider_condition {
            ProviderCondition::Healthy => {}
            ProviderCondition::Timeout => return Err("provider_timeout".to_owned()),
            ProviderCondition::Auth => return Err("provider_auth".to_owned()),
            ProviderCondition::Quota => return Err("provider_quota".to_owned()),
            ProviderCondition::Malformed => return Err("provider_malformed_output".to_owned()),
            ProviderCondition::Unavailable => return Err("provider_unavailable".to_owned()),
            ProviderCondition::Cancelled => return Err("scheduler_cancelled".to_owned()),
        }
        match self.command.action.as_str() {
            "provider.digest" => Ok(blake3::hash(self.command.payload.as_bytes())
                .to_hex()
                .to_string()
                .into_bytes()),
            "tool.file_metadata" => {
                let path = Path::new(&self.command.payload);
                if path.is_absolute()
                    || path
                        .components()
                        .any(|part| !matches!(part, std::path::Component::Normal(_)))
                {
                    return Err("tool_path_not_allowlisted".to_owned());
                }
                let metadata =
                    std::fs::metadata(path).map_err(|_| "tool_unavailable".to_owned())?;
                Ok(format!("bytes={}", metadata.len()).into_bytes())
            }
            "system.shutdown" => Ok(b"shutdown_checkpointed".to_vec()),
            _ => Err("unsupported_governed_action".to_owned()),
        }
    }
}

pub async fn execute(config: RuntimeConfig, command: GovernedCommand) -> GovernedOutcome {
    match execute_inner(&config, &command).await {
        Ok(outcome) => outcome,
        Err((classification, state)) => GovernedOutcome {
            schema: OUTPUT_SCHEMA.to_owned(),
            request_id: command.request_id,
            citizen_id: command.citizen_id,
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
    if command.qualified_unix_millis <= state.last_time {
        return refuse("unqualified_or_regressing_time", &state);
    }
    if command
        .read_citizen_id
        .as_deref()
        .is_some_and(|subject| subject != command.citizen_id)
    {
        return refuse("cross_identity_denied", &state);
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

    let policy_key = SigningKey::from_bytes(&config.policy_key);
    let authority_key = SigningKey::from_bytes(&config.authority_key);
    let permit_key = SigningKey::from_bytes(&config.permit_key);
    let policy_hash = blake3::hash(b"runtime-v3-parity-c-policy-v1")
        .to_hex()
        .to_string();
    let payload_hash = blake3::hash(command.payload.as_bytes())
        .to_hex()
        .to_string();
    let commitment_id = format!("commit-{}", command.request_id);
    let commitment = Commitment {
        schema: COMMITMENT_SCHEMA.to_owned(),
        commitment_id: commitment_id.clone(),
        principal: command.citizen_id.clone(),
        action: command.action.clone(),
        resource: command.resource.clone(),
        max_units: 8,
        policy_hash: policy_hash.clone(),
        expires_unix_millis: command.qualified_unix_millis.saturating_add(60_000),
        signing_key_id: "policy".to_owned(),
        signature: String::new(),
    }
    .sign(&policy_key)
    .map_err(|_| ("commitment_signing".to_owned(), state.clone()))?;
    let root = AuthorityGrant {
        schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
        grant_id: format!("grant-{}", command.request_id),
        principal: command.citizen_id.clone(),
        action: command.action.clone(),
        resource: command.resource.clone(),
        max_units: 8,
        max_delegation_depth: 2,
        parent_grant_hash: None,
        policy_hash: policy_hash.clone(),
        expires_unix_millis: command.qualified_unix_millis.saturating_add(60_000),
        signing_key_id: "authority".to_owned(),
        signature: String::new(),
    }
    .sign(&authority_key)
    .map_err(|_| ("authority_signing".to_owned(), state.clone()))?;
    let authority_chain = if let Some(delegate_units) = command.delegate_units {
        let child = AuthorityGrant {
            schema: AUTHORITY_GRANT_SCHEMA.to_owned(),
            grant_id: format!("delegate-{}", command.request_id),
            principal: command.citizen_id.clone(),
            action: command.action.clone(),
            resource: command.resource.clone(),
            max_units: delegate_units,
            max_delegation_depth: 1,
            parent_grant_hash: Some(
                root.hash()
                    .map_err(|_| ("authority_hash".to_owned(), state.clone()))?,
            ),
            policy_hash: policy_hash.clone(),
            expires_unix_millis: command.qualified_unix_millis.saturating_add(30_000),
            signing_key_id: "authority".to_owned(),
            signature: String::new(),
        }
        .sign(&authority_key)
        .map_err(|_| ("delegation_signing".to_owned(), state.clone()))?;
        vec![root, child]
    } else {
        vec![root]
    };
    let keys = GovernanceKeys {
        policy: BTreeMap::from([("policy".to_owned(), policy_key.verifying_key())]),
        authority: BTreeMap::from([("authority".to_owned(), authority_key.verifying_key())]),
        authority_principals: BTreeMap::from([(
            "authority".to_owned(),
            command.citizen_id.clone(),
        )]),
        root_authority_keys: BTreeSet::from(["authority".to_owned()]),
        operator: BTreeMap::new(),
    };
    let gate = FreedomGate::new(
        policy_hash.clone(),
        keys,
        "permit",
        permit_key.clone(),
        Arc::new(QualifiedTime(command.qualified_unix_millis)),
        BTreeMap::from([(command.resource.clone(), 8)]),
    )
    .map_err(|_| ("gate_configuration".to_owned(), state.clone()))?;
    if command.revoke_before_dispatch {
        gate.revoke_commitment(commitment_id)
            .map_err(|_| ("revocation_failure".to_owned(), state.clone()))?;
    }
    let request = GovernedActionRequest {
        request_id: command.request_id.clone(),
        principal: command.citizen_id.clone(),
        action: command.action.clone(),
        resource: command.resource.clone(),
        units: command.units,
        payload_hash,
        policy_hash,
        commitment,
        authority_chain,
    };
    let permit = match gate.mediate(&request) {
        MediationDecision::Allowed(permit) => permit,
        MediationDecision::Refused(evidence) => {
            return refuse(refusal_classification(evidence.reason), &state)
        }
    };

    state.generation = state.generation.saturating_add(1);
    state.last_time = command.qualified_unix_millis;
    state.pending_requests.insert(command.request_id.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;

    let shell = Arc::new(ProductionShell {
        command: command.clone(),
    });
    let aee = Aee::new(
        BTreeMap::from([("permit".to_owned(), permit_key.verifying_key())]),
        shell,
    );
    let recorded = aee
        .actuate(&permit)
        .await
        .map_err(|_| ("actuation_rejected".to_owned(), state.clone()))?;
    if !recorded.success {
        return refuse(
            std::str::from_utf8(&recorded.result_bytes).unwrap_or("actuation_quarantined"),
            &state,
        );
    }
    state.actuation_count = state.actuation_count.saturating_add(1);
    state.generation = state.generation.saturating_add(1);
    state.pending_requests.remove(&command.request_id);
    state.request_ids.insert(command.request_id.clone());
    state.private_state.insert(
        command.citizen_id.clone(),
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
        result_hash: recorded.result_hash,
        generation: state.generation,
        actuation_count: state.actuation_count,
    };
    state
        .completed
        .insert(command.idempotency_key.clone(), persisted.clone());
    persist_state(config, &state).map_err(|error| (error, state.clone()))?;
    if !command.lifelog_failure {
        let _ = append_lifelog(config, command, &persisted);
    }
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
        "freedom_gate_ed25519",
        "aee",
        "bounded_scheduler",
        "local_digest_provider",
        "allowlisted_file_metadata_tool",
        "identity_scoped_checkpoint_store",
        "redacted_append_only_lifelog",
        "qualified_monotonic_time",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
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

fn load_state(config: &RuntimeConfig) -> Result<RuntimeState, String> {
    let path = config.state_dir.join("checkpoint.json");
    if !path.exists() {
        return Ok(RuntimeState {
            schema: STATE_SCHEMA.to_owned(),
            ..RuntimeState::default()
        });
    }
    let bytes = std::fs::read(&path).map_err(|_| "checkpoint_unavailable".to_owned())?;
    let signed: SignedState =
        serde_json::from_slice(&bytes).map_err(|_| "checkpoint_corrupt".to_owned())?;
    if signed.state.schema != STATE_SCHEMA
        || state_integrity(&signed.state, &config.checkpoint_key)? != signed.integrity
    {
        return Err("checkpoint_authentication_failed".to_owned());
    }
    Ok(signed.state)
}

fn persist_state(config: &RuntimeConfig, state: &RuntimeState) -> Result<(), String> {
    std::fs::create_dir_all(&config.state_dir).map_err(|_| "checkpoint_unavailable".to_owned())?;
    let signed = SignedState {
        state: state.clone(),
        integrity: state_integrity(state, &config.checkpoint_key)?,
    };
    let bytes = serde_json::to_vec_pretty(&signed).map_err(|_| "checkpoint_encoding".to_owned())?;
    let tmp = config.state_dir.join("checkpoint.json.tmp");
    std::fs::write(&tmp, bytes).map_err(|_| "checkpoint_unavailable".to_owned())?;
    std::fs::rename(tmp, config.state_dir.join("checkpoint.json"))
        .map_err(|_| "checkpoint_unavailable".to_owned())
}

fn append_lifelog(
    config: &RuntimeConfig,
    command: &GovernedCommand,
    outcome: &PersistedOutcome,
) -> Result<(), String> {
    use std::io::Write;
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
    writeln!(file, "{}", entry).map_err(|_| "lifelog_unavailable".to_owned())
}

fn state_integrity(state: &RuntimeState, key: &[u8; 32]) -> Result<String, String> {
    let bytes = serde_json::to_vec(state).map_err(|_| "checkpoint_encoding".to_owned())?;
    Ok(blake3::keyed_hash(key, &bytes).to_hex().to_string())
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
    let value = required_env(name)?;
    let bytes = hex::decode(value).map_err(|_| format!("invalid_{name}"))?;
    bytes.try_into().map_err(|_| format!("invalid_{name}"))
}

#[allow(dead_code)]
fn _zero_hash() -> &'static str {
    ZERO_HASH
}
