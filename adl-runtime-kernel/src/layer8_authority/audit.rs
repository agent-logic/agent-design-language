use std::collections::BTreeSet;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use fs2::FileExt;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    AuthorityDecision, AuthorityRequest, AuthorityScope, AuthorizationGrant, Layer8Action,
    Layer8Capability, Layer8Policy, Layer8Principal, PublicRefusal, RefusalReason,
    LAYER8_AUDIT_SCHEMA,
};

const GENESIS_HASH: &str = "layer8-authority-genesis-v1";

#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AuditRecord {
    schema: String,
    sequence: u64,
    previous_hash: String,
    record_hash: String,
    timestamp_epoch_secs: u64,
    principal_hash: String,
    polis_hash: String,
    action: Layer8Action,
    conversation_hash: Option<String>,
    recipient_set_hash: String,
    attachment_hash: Option<String>,
    replay_hash: String,
    capability_hash: String,
    policy_hash: String,
    authorized: bool,
    refusal: Option<RefusalReason>,
    correlation_hash: String,
}

#[derive(Debug, Serialize)]
struct AuditHashInput<'a> {
    schema: &'a str,
    sequence: u64,
    previous_hash: &'a str,
    timestamp_epoch_secs: u64,
    principal_hash: &'a str,
    polis_hash: &'a str,
    action: &'a Layer8Action,
    conversation_hash: &'a Option<String>,
    recipient_set_hash: &'a str,
    attachment_hash: &'a Option<String>,
    replay_hash: &'a str,
    capability_hash: &'a str,
    policy_hash: &'a str,
    authorized: bool,
    refusal: &'a Option<RefusalReason>,
    correlation_hash: &'a str,
}

#[derive(Debug)]
struct StoreState {
    sequence: u64,
    head_hash: String,
    replay_hashes: BTreeSet<String>,
}

fn load_store_state(file: &File) -> Result<StoreState, RefusalReason> {
    let mut state = StoreState {
        sequence: 0,
        head_hash: GENESIS_HASH.to_string(),
        replay_hashes: BTreeSet::new(),
    };
    for line in BufReader::new(file).lines() {
        let line = line.map_err(|_| RefusalReason::AuditUnavailable)?;
        if line.trim().is_empty() {
            return Err(RefusalReason::AuditUnavailable);
        }
        let record: AuditRecord =
            serde_json::from_str(&line).map_err(|_| RefusalReason::AuditUnavailable)?;
        if record.schema != LAYER8_AUDIT_SCHEMA
            || record.sequence != state.sequence + 1
            || record.previous_hash != state.head_hash
            || record.record_hash != calculate_record_hash(&record)?
        {
            return Err(RefusalReason::AuditUnavailable);
        }
        state.replay_hashes.insert(record.replay_hash.clone());
        state.sequence = record.sequence;
        state.head_hash = record.record_hash;
    }
    Ok(state)
}

#[derive(Debug)]
pub struct Layer8AuthorityStore {
    path: PathBuf,
    state: Mutex<StoreState>,
}

impl Layer8AuthorityStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, RefusalReason> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(|_| RefusalReason::AuditUnavailable)?;
        }
        let state = match File::open(&path) {
            Ok(file) => load_store_state(&file)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => StoreState {
                sequence: 0,
                head_hash: GENESIS_HASH.to_string(),
                replay_hashes: BTreeSet::new(),
            },
            Err(_) => return Err(RefusalReason::AuditUnavailable),
        };
        Ok(Self {
            path,
            state: Mutex::new(state),
        })
    }

    pub(super) fn authorize(
        &self,
        request: AuthorityRequest,
        capability: &Layer8Capability,
        agent_policy: &Layer8Policy,
        polis_policy: &Layer8Policy,
    ) -> AuthorityDecision {
        let correlation_id = bounded_correlation(&request.correlation_id);
        let mut state = match self.state.lock() {
            Ok(state) => state,
            Err(_) => return refused(RefusalReason::AuditUnavailable, correlation_id),
        };
        let mut file = match OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.path)
        {
            Ok(file) => file,
            Err(_) => return refused(RefusalReason::AuditUnavailable, correlation_id),
        };
        if file.lock_exclusive().is_err() {
            return refused(RefusalReason::AuditUnavailable, correlation_id);
        }
        let current = match load_store_state(&file) {
            Ok(current) => current,
            Err(reason) => return refused(reason, correlation_id),
        };
        *state = current;
        let replay_hash = hash_text(&request.replay_id);
        let principal = request.evidence.derive_principal(request.now_epoch_secs);
        let reason = request.prechecked_refusal.or_else(|| {
            validate(
                &request,
                principal.as_ref().ok(),
                capability,
                agent_policy,
                polis_policy,
                &state.replay_hashes,
                &replay_hash,
            )
        });
        let authorized = reason.is_none();
        let principal_hash = principal
            .as_ref()
            .map(|p| hash_text(&p.principal_id))
            .unwrap_or_else(|_| hash_text("unknown"));
        let record = build_record(
            &state,
            &request,
            capability,
            agent_policy,
            polis_policy,
            principal_hash,
            replay_hash.clone(),
            authorized,
            reason,
        );
        let record = match record.and_then(|record| Self::append_locked(&mut file, record)) {
            Ok(record) => record,
            Err(reason) => return refused(reason, correlation_id),
        };
        state.sequence = record.sequence;
        state.head_hash = record.record_hash.clone();
        state.replay_hashes.insert(replay_hash);
        match reason {
            Some(reason) => refused(reason, correlation_id),
            None => AuthorityDecision::Authorized(AuthorizationGrant {
                principal: principal.expect("validated principal"),
                action: request.action,
                conversation_id: request.conversation_id,
                recipients: request.recipients,
                correlation_id,
                audit_hash: record.record_hash,
            }),
        }
    }

    fn append_locked(file: &mut File, record: AuditRecord) -> Result<AuditRecord, RefusalReason> {
        let bytes = serde_json::to_vec(&record).map_err(|_| RefusalReason::AuditUnavailable)?;
        file.write_all(&bytes)
            .and_then(|_| file.write_all(b"\n"))
            .and_then(|_| file.sync_data())
            .map_err(|_| RefusalReason::AuditUnavailable)?;
        Ok(record)
    }
}

fn validate(
    request: &AuthorityRequest,
    principal: Option<&Layer8Principal>,
    capability: &Layer8Capability,
    agent_policy: &Layer8Policy,
    polis_policy: &Layer8Policy,
    replay_hashes: &BTreeSet<String>,
    replay_hash: &str,
) -> Option<RefusalReason> {
    let principal = match principal {
        Some(value) => value,
        None => {
            return request
                .evidence
                .derive_principal(request.now_epoch_secs)
                .err()
        }
    };
    if request.replay_id.trim().is_empty() || request.correlation_id.trim().is_empty() {
        return Some(RefusalReason::InvalidRequest);
    }
    if replay_hashes.contains(replay_hash) {
        return Some(RefusalReason::ReplayRefused);
    }
    if capability.revoked {
        return Some(RefusalReason::CapabilityRevoked);
    }
    if request.now_epoch_secs >= capability.expires_at_epoch_secs {
        return Some(RefusalReason::CapabilityExpired);
    }
    if capability.epoch == 0
        || capability.epoch != agent_policy.epoch
        || capability.epoch != polis_policy.epoch
    {
        return Some(RefusalReason::StaleCapability);
    }
    if !agent_policy.available || !polis_policy.available {
        return Some(RefusalReason::PolicyUnavailable);
    }
    if capability.principal_id != principal.principal_id {
        return Some(RefusalReason::CapabilityDenied);
    }
    if !scope_matches(request, principal, &capability.scope)
        || !scope_matches(request, principal, &agent_policy.scope)
        || !scope_matches(request, principal, &polis_policy.scope)
    {
        return Some(RefusalReason::ScopeDenied);
    }
    None
}

pub(super) fn scope_matches(
    request: &AuthorityRequest,
    principal: &Layer8Principal,
    scope: &AuthorityScope,
) -> bool {
    !request.recipients.is_empty()
        && scope.polis_id == principal.polis_id
        && scope.action == request.action
        && scope
            .conversation_id
            .as_ref()
            .is_none_or(|allowed| request.conversation_id.as_ref() == Some(allowed))
        && if request.action == Layer8Action::AddressRecipients {
            request.recipients == scope.recipients
        } else {
            request.recipients.is_subset(&scope.recipients)
        }
        && scope.attachment_id == request.attachment_id
        && !(request.action == Layer8Action::Attach && request.attachment_id.is_none())
}

#[allow(clippy::too_many_arguments)]
fn build_record(
    state: &StoreState,
    request: &AuthorityRequest,
    capability: &Layer8Capability,
    agent_policy: &Layer8Policy,
    polis_policy: &Layer8Policy,
    principal_hash: String,
    replay_hash: String,
    authorized: bool,
    refusal: Option<RefusalReason>,
) -> Result<AuditRecord, RefusalReason> {
    let mut record = AuditRecord {
        schema: LAYER8_AUDIT_SCHEMA.to_string(),
        sequence: state
            .sequence
            .checked_add(1)
            .ok_or(RefusalReason::AuditUnavailable)?,
        previous_hash: state.head_hash.clone(),
        record_hash: String::new(),
        timestamp_epoch_secs: request.now_epoch_secs,
        principal_hash,
        polis_hash: hash_text(&request.evidence.polis_id),
        action: request.action.clone(),
        conversation_hash: request.conversation_id.as_deref().map(hash_text),
        recipient_set_hash: hash_recipients(&request.recipients),
        attachment_hash: request.attachment_id.as_deref().map(hash_text),
        replay_hash,
        capability_hash: hash_text(&capability.capability_id),
        policy_hash: hash_text(&format!(
            "{}:{}",
            agent_policy.policy_id, polis_policy.policy_id
        )),
        authorized,
        refusal,
        correlation_hash: hash_text(&request.correlation_id),
    };
    record.record_hash = calculate_record_hash(&record)?;
    Ok(record)
}

fn calculate_record_hash(record: &AuditRecord) -> Result<String, RefusalReason> {
    let input = AuditHashInput {
        schema: &record.schema,
        sequence: record.sequence,
        previous_hash: &record.previous_hash,
        timestamp_epoch_secs: record.timestamp_epoch_secs,
        principal_hash: &record.principal_hash,
        polis_hash: &record.polis_hash,
        action: &record.action,
        conversation_hash: &record.conversation_hash,
        recipient_set_hash: &record.recipient_set_hash,
        attachment_hash: &record.attachment_hash,
        replay_hash: &record.replay_hash,
        capability_hash: &record.capability_hash,
        policy_hash: &record.policy_hash,
        authorized: record.authorized,
        refusal: &record.refusal,
        correlation_hash: &record.correlation_hash,
    };
    let bytes = serde_jcs::to_vec(&input).map_err(|_| RefusalReason::AuditUnavailable)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn hash_text(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

fn hash_recipients(recipients: &BTreeSet<String>) -> String {
    hash_text(&recipients.iter().cloned().collect::<Vec<_>>().join("\n"))
}

fn bounded_correlation(value: &str) -> String {
    hash_text(value)
}

fn refused(reason: RefusalReason, correlation_id: String) -> AuthorityDecision {
    AuthorityDecision::Refused(PublicRefusal {
        authorized: false,
        reason,
        retryable: matches!(
            reason,
            RefusalReason::PolicyUnavailable | RefusalReason::AuditUnavailable
        ),
        correlation_id,
    })
}
