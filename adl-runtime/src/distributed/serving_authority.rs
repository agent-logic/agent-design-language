//! Durable, reconcile-before-publish serving-authority foundation.
//!
//! This module is a replica of sealed authority, never an authority issuer.

use std::{collections::BTreeMap, path::Path, sync::Arc};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use super::{
    authority_protocol::PublishedAuthorityResult,
    authority_store_adapters::PublishedStoreAuthorityReceiptView,
    polis_runtime::{
        CheckpointMetadata, CheckpointMetadataSource, CheckpointedJson,
        ConsensusCheckpointAuthority, DurableEnvelope, PolisRuntimeError,
    },
};

const OBSERVATORY_BINDING_DOMAIN: &str = "adl.observatory-serving-authority-binding.v1";
const I_JSON_MAX_INTEGER: u64 = 9_007_199_254_740_991;

const BINDING_SCHEMA: &str = "adl.serving-authority-foundation.binding.v1";
const PROJECTION_SCHEMA: &str = "adl.serving-authority-foundation.projection.v1";
const DOMAIN: &[u8] = b"ADL-SERVING-AUTHORITY-FOUNDATION-BINDING-V1\0";
const MAX_PREIMAGE_BYTES: usize = 4096;
const MAX_IDENTIFIER_BYTES: usize = 128;
const MAX_CAPACITY: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServingAuthorityError {
    InvalidIdentity,
    InvalidBinding,
    ReceiptMismatch,
    PriorStateMismatch,
    RetryConflict,
    CapacityExceeded,
    Serialization,
    Storage,
    StateRegression,
}

pub type ServingAuthorityResult<T> = Result<T, ServingAuthorityError>;

impl From<PolisRuntimeError> for ServingAuthorityError {
    fn from(value: PolisRuntimeError) -> Self {
        match value {
            PolisRuntimeError::StateRegression => Self::StateRegression,
            PolisRuntimeError::Serialization | PolisRuntimeError::FrameTooLarge => {
                Self::Serialization
            }
            _ => Self::Storage,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ServingAuthorityIdentity {
    pub trust_domain: String,
    pub polis_id: String,
    pub node_id: String,
    pub guardian_id: String,
    pub boot_generation: u64,
}

impl ServingAuthorityIdentity {
    fn validate(&self) -> ServingAuthorityResult<()> {
        for value in [
            &self.trust_domain,
            &self.polis_id,
            &self.node_id,
            &self.guardian_id,
        ] {
            validate_identifier(value)?;
        }
        if self.boot_generation == 0 {
            return Err(ServingAuthorityError::InvalidIdentity);
        }
        Ok(())
    }

    fn object(&self) -> String {
        let digest = Sha256::digest(format!(
            "{}\0{}\0{}\0{}\0{}",
            self.trust_domain, self.polis_id, self.node_id, self.guardian_id, self.boot_generation
        ));
        format!("serving-authority:{}", hex::encode(digest))
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ServingAuthorityBinding {
    schema: String,
    trust_domain: String,
    polis_id: String,
    lineage_id: String,
    operation_id: String,
    adapter_kind: String,
    adapter_version: u32,
    action_class: String,
    published_generation: u64,
    owner_commit_id: String,
    fencing_generation: u64,
    lease_id: String,
    prior_state_sha256: String,
    candidate_state_sha256: String,
    receipt_digest: String,
}

impl ServingAuthorityBinding {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        trust_domain: String,
        polis_id: String,
        lineage_id: String,
        operation_id: String,
        adapter_kind: String,
        adapter_version: u32,
        action_class: String,
        published_generation: u64,
        owner_commit_id: String,
        fencing_generation: u64,
        lease_id: String,
        prior_state_sha256: String,
        candidate_state_sha256: String,
        receipt_digest: String,
    ) -> Self {
        Self {
            schema: BINDING_SCHEMA.into(),
            trust_domain,
            polis_id,
            lineage_id,
            operation_id,
            adapter_kind,
            adapter_version,
            action_class,
            published_generation,
            owner_commit_id,
            fencing_generation,
            lease_id,
            prior_state_sha256,
            candidate_state_sha256,
            receipt_digest,
        }
    }

    pub fn canonical_preimage(&self) -> ServingAuthorityResult<Vec<u8>> {
        self.validate()?;
        let jcs = serde_jcs::to_vec(self).map_err(|_| ServingAuthorityError::Serialization)?;
        let len: u32 = jcs
            .len()
            .try_into()
            .map_err(|_| ServingAuthorityError::InvalidBinding)?;
        let mut framed = Vec::with_capacity(DOMAIN.len() + 4 + jcs.len());
        framed.extend_from_slice(DOMAIN);
        framed.extend_from_slice(&len.to_be_bytes());
        framed.extend_from_slice(&jcs);
        if framed.len() > MAX_PREIMAGE_BYTES {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        Ok(framed)
    }

    #[cfg(any(test, debug_assertions, feature = "internal-test-fixtures"))]
    pub fn from_canonical_preimage_fixture(bytes: &[u8]) -> ServingAuthorityResult<Self> {
        if bytes.len() > MAX_PREIMAGE_BYTES
            || !bytes.starts_with(DOMAIN)
            || bytes.len() < DOMAIN.len() + 4
        {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        let length = u32::from_be_bytes(
            bytes[DOMAIN.len()..DOMAIN.len() + 4]
                .try_into()
                .map_err(|_| ServingAuthorityError::InvalidBinding)?,
        ) as usize;
        let jcs = &bytes[DOMAIN.len() + 4..];
        if length != jcs.len() {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        let value: Self =
            serde_json::from_slice(jcs).map_err(|_| ServingAuthorityError::InvalidBinding)?;
        if value.canonical_preimage()?.as_slice() != bytes {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        Ok(value)
    }

    fn validate(&self) -> ServingAuthorityResult<()> {
        if self.schema != BINDING_SCHEMA
            || self.adapter_version == 0
            || self.published_generation == 0
            || self.fencing_generation == 0
        {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        for value in [
            &self.trust_domain,
            &self.polis_id,
            &self.lineage_id,
            &self.operation_id,
            &self.adapter_kind,
            &self.action_class,
            &self.owner_commit_id,
            &self.lease_id,
        ] {
            validate_identifier(value).map_err(|_| ServingAuthorityError::InvalidBinding)?;
        }
        for digest in [
            &self.prior_state_sha256,
            &self.candidate_state_sha256,
            &self.receipt_digest,
        ] {
            validate_digest(digest)?;
        }
        if self.prior_state_sha256 == self.candidate_state_sha256 {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        Ok(())
    }
}

trait SealedReceipt {
    fn lineage_id(&self) -> &str;
    fn operation_id(&self) -> &str;
    fn action_class(&self) -> &str;
    fn adapter_kind(&self) -> &str;
    fn adapter_version(&self) -> u32;
    fn generation(&self) -> u64;
    fn receipt_sha256(&self) -> [u8; 32];
    fn result_sha256(&self) -> [u8; 32];
}

impl SealedReceipt for PublishedStoreAuthorityReceiptView {
    fn lineage_id(&self) -> &str {
        self.lineage_id()
    }
    fn operation_id(&self) -> &str {
        self.operation_id()
    }
    fn action_class(&self) -> &str {
        self.action_class()
    }
    fn adapter_kind(&self) -> &str {
        self.adapter_kind()
    }
    fn adapter_version(&self) -> u32 {
        self.adapter_version()
    }
    fn generation(&self) -> u64 {
        self.generation()
    }
    fn receipt_sha256(&self) -> [u8; 32] {
        self.receipt_sha256()
    }
    fn result_sha256(&self) -> [u8; 32] {
        self.result_sha256()
    }
}

#[cfg(any(test, debug_assertions))]
#[derive(Clone, Debug)]
pub struct ServingAuthorityReceiptFixture {
    pub lineage_id: String,
    pub operation_id: String,
    pub action_class: String,
    pub adapter_kind: String,
    pub adapter_version: u32,
    pub generation: u64,
    pub receipt_sha256: [u8; 32],
    pub result_sha256: [u8; 32],
}

#[cfg(any(test, debug_assertions))]
impl SealedReceipt for ServingAuthorityReceiptFixture {
    fn lineage_id(&self) -> &str {
        &self.lineage_id
    }
    fn operation_id(&self) -> &str {
        &self.operation_id
    }
    fn action_class(&self) -> &str {
        &self.action_class
    }
    fn adapter_kind(&self) -> &str {
        &self.adapter_kind
    }
    fn adapter_version(&self) -> u32 {
        self.adapter_version
    }
    fn generation(&self) -> u64 {
        self.generation
    }
    fn receipt_sha256(&self) -> [u8; 32] {
        self.receipt_sha256
    }
    fn result_sha256(&self) -> [u8; 32] {
        self.result_sha256
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Phase {
    Pending,
    Reconciled,
    Published,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Operation {
    phase: Phase,
    generation: u64,
    binding_sha256: String,
    prior_state_sha256: String,
    candidate_state_sha256: String,
    receipt_digest: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct State {
    revision: u64,
    operations: BTreeMap<String, Operation>,
    published_operation: Option<String>,
}

impl CheckpointMetadataSource for State {
    fn checkpoint_metadata(&self) -> Result<CheckpointMetadata, PolisRuntimeError> {
        let bytes = serde_jcs::to_vec(self).map_err(|_| PolisRuntimeError::Serialization)?;
        Ok(CheckpointMetadata {
            committed_log_index: Some(self.revision),
            state_sha256: Some(hex::encode(Sha256::digest(bytes))),
            snapshot_log_index: None,
            snapshot_sha256: None,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ServingAuthorityProjection {
    pub schema: String,
    pub polis_ref: String,
    pub lineage_ref: String,
    pub generation: u64,
    pub state_sha256: String,
    pub result_sha256: String,
    pub receipt_generation: u64,
    pub receipt_digest: String,
    pub readiness: String,
}

/// An authenticated, durably published foundation cut.
///
/// The private fields and absence of a public constructor prevent callers from
/// upgrading a redacted projection or naked digests into serving authority.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedServingAuthorityCut {
    lineage_id: String,
    generation: u64,
    owner_commit_id: String,
    fencing_generation: u64,
    lease_id: String,
    state_sha256: String,
    result_sha256: String,
    receipt_digest: String,
}

impl VerifiedServingAuthorityCut {
    #[cfg(feature = "internal-test-fixtures")]
    #[allow(clippy::too_many_arguments)]
    pub fn fixture(
        lineage_id: String,
        generation: u64,
        owner_commit_id: String,
        fencing_generation: u64,
        lease_id: String,
        state_sha256: String,
        result_sha256: String,
        receipt_digest: String,
    ) -> Self {
        Self {
            lineage_id,
            generation,
            owner_commit_id,
            fencing_generation,
            lease_id,
            state_sha256,
            result_sha256,
            receipt_digest,
        }
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn lineage_id(&self) -> &str {
        &self.lineage_id
    }
    pub fn lineage_ref(&self) -> String {
        keyed_ref("lineage", &self.lineage_id)
    }
    pub fn owner_commit_id(&self) -> &str {
        &self.owner_commit_id
    }
    pub fn fencing_generation(&self) -> u64 {
        self.fencing_generation
    }
    pub fn lease_id(&self) -> &str {
        &self.lease_id
    }
    pub fn state_sha256(&self) -> &str {
        &self.state_sha256
    }
    pub fn result_sha256(&self) -> &str {
        &self.result_sha256
    }
    pub fn receipt_digest(&self) -> &str {
        &self.receipt_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObservatoryAuthorityBinding {
    domain: String,
    trust_domain: String,
    polis_id: String,
    lineage_id: String,
    operation_id: String,
    transition_action: ObservatoryTransitionAction,
    predecessor_operation_ref: Option<String>,
    committed_log_index: u64,
    foundation_generation: u64,
    owner_commit_id: String,
    fencing_generation: u64,
    lease_id: String,
    foundation_state_sha256: String,
    foundation_result_sha256: String,
    foundation_receipt_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ObservatoryTransitionAction {
    Acquire,
    Renew,
    Transfer,
    Revoke,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct VerifiedObservatoryAuthorityProjection {
    schema: &'static str,
    trust_domain_ref: String,
    polis_ref: String,
    lineage_ref: String,
    operation_ref: String,
    transition_action: ObservatoryTransitionAction,
    predecessor_operation_ref: Option<String>,
    committed_log_index: u64,
    foundation_generation: u64,
    fencing_generation: u64,
    authority_result_sha256: String,
    signer_set_sha256: String,
    signer_count: usize,
    inclusive_deadline_unix_seconds: i64,
    inclusive_deadline_nanos: u32,
    inclusive_deadline_uncertainty_millis: u64,
    finalization_unix_seconds: i64,
    finalization_nanos: u32,
    finalization_uncertainty_millis: u64,
}

impl VerifiedObservatoryAuthorityProjection {
    pub fn is_expired_at(&self, unix_seconds: i64, nanos: u32) -> ServingAuthorityResult<bool> {
        if unix_seconds <= 0 || nanos >= 1_000_000_000 {
            return Err(ServingAuthorityError::InvalidBinding);
        }
        Ok((unix_seconds, nanos)
            > (
                self.inclusive_deadline_unix_seconds,
                self.inclusive_deadline_nanos,
            ))
    }
    pub fn trust_domain_ref(&self) -> &str {
        &self.trust_domain_ref
    }

    pub fn polis_ref(&self) -> &str {
        &self.polis_ref
    }

    pub fn lineage_ref(&self) -> &str {
        &self.lineage_ref
    }

    pub fn operation_ref(&self) -> &str {
        &self.operation_ref
    }
    pub fn transition_action(&self) -> ObservatoryTransitionAction {
        self.transition_action
    }
    pub fn predecessor_operation_ref(&self) -> Option<&str> {
        self.predecessor_operation_ref.as_deref()
    }

    pub fn committed_log_index(&self) -> u64 {
        self.committed_log_index
    }

    pub fn foundation_generation(&self) -> u64 {
        self.foundation_generation
    }

    pub fn fencing_generation(&self) -> u64 {
        self.fencing_generation
    }

    pub fn authority_result_sha256(&self) -> &str {
        &self.authority_result_sha256
    }

    pub fn signer_set_sha256(&self) -> &str {
        &self.signer_set_sha256
    }

    pub fn signer_count(&self) -> usize {
        self.signer_count
    }

    pub fn inclusive_deadline_unix_seconds(&self) -> i64 {
        self.inclusive_deadline_unix_seconds
    }
    pub fn inclusive_deadline_nanos(&self) -> u32 {
        self.inclusive_deadline_nanos
    }
    pub fn inclusive_deadline_uncertainty_millis(&self) -> u64 {
        self.inclusive_deadline_uncertainty_millis
    }

    pub fn finalization_unix_seconds(&self) -> i64 {
        self.finalization_unix_seconds
    }
    pub fn finalization_nanos(&self) -> u32 {
        self.finalization_nanos
    }
    pub fn finalization_uncertainty_millis(&self) -> u64 {
        self.finalization_uncertainty_millis
    }
}

pub fn verify_observatory_authority_projection(
    authority: &PublishedAuthorityResult,
    cut: &VerifiedServingAuthorityCut,
) -> ServingAuthorityResult<VerifiedObservatoryAuthorityProjection> {
    let source = authority
        .observatory_projection()
        .map_err(|_| ServingAuthorityError::InvalidBinding)?;
    let binding: ObservatoryAuthorityBinding = serde_json::from_slice(source.artifact_bytes)
        .map_err(|_| ServingAuthorityError::InvalidBinding)?;
    let canonical =
        serde_jcs::to_vec(&binding).map_err(|_| ServingAuthorityError::Serialization)?;
    if canonical != source.artifact_bytes
        || <[u8; 32]>::from(Sha256::digest(&canonical)) != source.artifact_sha256
        || binding.domain != OBSERVATORY_BINDING_DOMAIN
        || binding.trust_domain != source.trust_domain
        || binding.polis_id != source.polis_id
        || binding.operation_id != source.operation_id
        || binding.lineage_id != cut.lineage_id
        || binding.committed_log_index != source.committed_log_index
        || binding.foundation_generation != cut.generation
        || binding.owner_commit_id != cut.owner_commit_id
        || binding.fencing_generation != cut.fencing_generation
        || binding.lease_id != cut.lease_id
        || binding.foundation_state_sha256 != cut.state_sha256
        || binding.foundation_result_sha256 != cut.result_sha256
        || binding.foundation_receipt_sha256 != cut.receipt_digest
    {
        return Err(ServingAuthorityError::InvalidBinding);
    }
    for value in [
        &binding.trust_domain,
        &binding.polis_id,
        &binding.lineage_id,
        &binding.operation_id,
        &binding.owner_commit_id,
        &binding.lease_id,
    ] {
        validate_observatory_identifier(value)?;
    }
    for digest in [
        &binding.foundation_state_sha256,
        &binding.foundation_result_sha256,
        &binding.foundation_receipt_sha256,
    ] {
        validate_digest(digest)?;
    }
    match (
        binding.transition_action,
        binding.predecessor_operation_ref.as_deref(),
    ) {
        (ObservatoryTransitionAction::Acquire, None) => {}
        (
            ObservatoryTransitionAction::Renew
            | ObservatoryTransitionAction::Transfer
            | ObservatoryTransitionAction::Revoke,
            Some(predecessor),
        ) if predecessor != binding.operation_id => validate_observatory_identifier(predecessor)?,
        _ => return Err(ServingAuthorityError::InvalidBinding),
    }
    if binding.committed_log_index == 0
        || binding.committed_log_index > I_JSON_MAX_INTEGER
        || binding.foundation_generation == 0
        || binding.foundation_generation > I_JSON_MAX_INTEGER
        || binding.fencing_generation == 0
        || binding.fencing_generation > I_JSON_MAX_INTEGER
        || source.signer_count == 0
    {
        return Err(ServingAuthorityError::InvalidBinding);
    }
    Ok(VerifiedObservatoryAuthorityProjection {
        schema: "adl.observatory-authority-projection.v1",
        trust_domain_ref: keyed_ref("trust-domain", &binding.trust_domain),
        polis_ref: keyed_ref("polis", &binding.polis_id),
        lineage_ref: keyed_ref("lineage", &binding.lineage_id),
        operation_ref: keyed_ref("operation", &binding.operation_id),
        transition_action: binding.transition_action,
        predecessor_operation_ref: binding
            .predecessor_operation_ref
            .as_deref()
            .map(|value| keyed_ref("operation", value)),
        committed_log_index: binding.committed_log_index,
        foundation_generation: binding.foundation_generation,
        fencing_generation: binding.fencing_generation,
        authority_result_sha256: hex::encode(source.result_sha256),
        signer_set_sha256: hex::encode(source.signer_set_sha256),
        signer_count: source.signer_count,
        inclusive_deadline_unix_seconds: source.inclusive_deadline.unix_seconds,
        inclusive_deadline_nanos: source.inclusive_deadline.nanos,
        inclusive_deadline_uncertainty_millis: source.inclusive_deadline.uncertainty_millis,
        finalization_unix_seconds: source.finalization_time.unix_seconds,
        finalization_nanos: source.finalization_time.nanos,
        finalization_uncertainty_millis: source.finalization_time.uncertainty_millis,
    })
}

fn validate_observatory_identifier(value: &str) -> ServingAuthorityResult<()> {
    if value.is_empty()
        || value.len() > MAX_IDENTIFIER_BYTES
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-' | b'.' | b':'))
    {
        return Err(ServingAuthorityError::InvalidIdentity);
    }
    Ok(())
}

#[cfg(feature = "internal-test-fixtures")]
pub struct ObservatoryBindingFixture(ObservatoryAuthorityBinding);

#[cfg(feature = "internal-test-fixtures")]
#[derive(Clone, Copy, Debug)]
pub enum ObservatoryBindingMutation {
    Domain,
    TrustDomain,
    PolisId,
    LineageId,
    OperationId,
    CommittedLogIndex,
    FoundationGeneration,
    OwnerCommitId,
    FencingGeneration,
    LeaseId,
    StateDigest,
    ResultDigest,
    ReceiptDigest,
}

#[cfg(feature = "internal-test-fixtures")]
#[derive(Clone, Copy, Debug)]
pub enum ObservatoryIdentifierField {
    TrustDomain,
    PolisId,
    LineageId,
    OperationId,
    OwnerCommitId,
    LeaseId,
}

#[cfg(feature = "internal-test-fixtures")]
#[derive(Clone, Copy, Debug)]
pub enum ObservatoryDigestField {
    State,
    Result,
    Receipt,
}

#[cfg(feature = "internal-test-fixtures")]
impl ObservatoryBindingFixture {
    pub fn new(suffix: &str) -> Self {
        let digest = hex::encode(Sha256::digest(suffix.as_bytes()));
        Self(ObservatoryAuthorityBinding {
            domain: OBSERVATORY_BINDING_DOMAIN.into(),
            trust_domain: "trust-domain".into(),
            polis_id: "polis".into(),
            lineage_id: format!("lineage-{suffix}"),
            operation_id: "operation".into(),
            transition_action: ObservatoryTransitionAction::Acquire,
            predecessor_operation_ref: None,
            committed_log_index: 2,
            foundation_generation: 1,
            owner_commit_id: format!("owner-{suffix}"),
            fencing_generation: 1,
            lease_id: format!("lease-{suffix}"),
            foundation_state_sha256: digest.clone(),
            foundation_result_sha256: digest.clone(),
            foundation_receipt_sha256: digest,
        })
    }
    pub fn artifact_bytes(&self) -> Vec<u8> {
        serde_jcs::to_vec(&self.0).expect("fixture JCS")
    }
    pub fn set_transition(
        &mut self,
        action: ObservatoryTransitionAction,
        predecessor: Option<&str>,
    ) {
        self.0.transition_action = action;
        self.0.predecessor_operation_ref = predecessor.map(str::to_owned);
    }
    pub fn set_operation(&mut self, operation_id: &str, committed_log_index: u64) {
        self.0.operation_id = operation_id.to_owned();
        self.0.committed_log_index = committed_log_index;
    }

    pub fn mutate(&mut self, mutation: ObservatoryBindingMutation) {
        match mutation {
            ObservatoryBindingMutation::Domain => self.0.domain.push_str(".wrong"),
            ObservatoryBindingMutation::TrustDomain => self.0.trust_domain.push('x'),
            ObservatoryBindingMutation::PolisId => self.0.polis_id.push('x'),
            ObservatoryBindingMutation::LineageId => self.0.lineage_id.push('x'),
            ObservatoryBindingMutation::OperationId => self.0.operation_id.push('x'),
            ObservatoryBindingMutation::CommittedLogIndex => self.0.committed_log_index += 1,
            ObservatoryBindingMutation::FoundationGeneration => self.0.foundation_generation += 1,
            ObservatoryBindingMutation::OwnerCommitId => self.0.owner_commit_id.push('x'),
            ObservatoryBindingMutation::FencingGeneration => self.0.fencing_generation += 1,
            ObservatoryBindingMutation::LeaseId => self.0.lease_id.push('x'),
            ObservatoryBindingMutation::StateDigest => {
                self.0.foundation_state_sha256.replace_range(0..1, "f")
            }
            ObservatoryBindingMutation::ResultDigest => {
                self.0.foundation_result_sha256.replace_range(0..1, "f")
            }
            ObservatoryBindingMutation::ReceiptDigest => {
                self.0.foundation_receipt_sha256.replace_range(0..1, "f")
            }
        }
    }

    pub fn set_integers(&mut self, committed: u64, generation: u64, fencing: u64) {
        self.0.committed_log_index = committed;
        self.0.foundation_generation = generation;
        self.0.fencing_generation = fencing;
    }

    pub fn set_invalid_identifier(&mut self, field: ObservatoryIdentifierField, value: &str) {
        *match field {
            ObservatoryIdentifierField::TrustDomain => &mut self.0.trust_domain,
            ObservatoryIdentifierField::PolisId => &mut self.0.polis_id,
            ObservatoryIdentifierField::LineageId => &mut self.0.lineage_id,
            ObservatoryIdentifierField::OperationId => &mut self.0.operation_id,
            ObservatoryIdentifierField::OwnerCommitId => &mut self.0.owner_commit_id,
            ObservatoryIdentifierField::LeaseId => &mut self.0.lease_id,
        } = value.to_owned();
    }

    pub fn set_digest_encoding(&mut self, field: ObservatoryDigestField, value: &str) {
        *match field {
            ObservatoryDigestField::State => &mut self.0.foundation_state_sha256,
            ObservatoryDigestField::Result => &mut self.0.foundation_result_sha256,
            ObservatoryDigestField::Receipt => &mut self.0.foundation_receipt_sha256,
        } = value.to_owned();
    }
}

#[cfg(feature = "internal-test-fixtures")]
impl VerifiedServingAuthorityCut {
    pub fn fixture_from_observatory(fixture: &ObservatoryBindingFixture) -> Self {
        let binding = &fixture.0;
        Self::fixture(
            binding.lineage_id.clone(),
            binding.foundation_generation,
            binding.owner_commit_id.clone(),
            binding.fencing_generation,
            binding.lease_id.clone(),
            binding.foundation_state_sha256.clone(),
            binding.foundation_result_sha256.clone(),
            binding.foundation_receipt_sha256.clone(),
        )
    }
}

pub struct ServingAuthorityStore {
    store: CheckpointedJson<State>,
    envelope: DurableEnvelope<State>,
    identity: ServingAuthorityIdentity,
    capacity: usize,
}

#[cfg(any(test, debug_assertions))]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ServingAuthorityTestBoundary {
    Pending,
    Reconciled,
}

impl ServingAuthorityStore {
    pub fn open(
        root: &Path,
        identity: ServingAuthorityIdentity,
        authority: Arc<dyn ConsensusCheckpointAuthority>,
        capacity: usize,
    ) -> ServingAuthorityResult<Self> {
        identity.validate()?;
        if capacity == 0 || capacity > MAX_CAPACITY {
            return Err(ServingAuthorityError::CapacityExceeded);
        }
        let object = identity.object();
        let (store, envelope) = CheckpointedJson::open(
            root,
            &object,
            "serving-authority.json",
            State::default(),
            authority,
        )?;
        if envelope.payload().operations.len() > capacity {
            return Err(ServingAuthorityError::CapacityExceeded);
        }
        Ok(Self {
            store,
            envelope,
            identity,
            capacity,
        })
    }

    pub fn reconcile_and_publish(
        &mut self,
        receipt: &PublishedStoreAuthorityReceiptView,
        binding: &ServingAuthorityBinding,
    ) -> ServingAuthorityResult<ServingAuthorityProjection> {
        self.apply(receipt, binding)
    }

    /// Reconciles the sealed receipt and binding through the existing durable
    /// publication path, then returns the non-policy authenticated cut.
    pub fn reconcile_and_verify_cut(
        &mut self,
        receipt: &PublishedStoreAuthorityReceiptView,
        binding: &ServingAuthorityBinding,
    ) -> ServingAuthorityResult<(ServingAuthorityProjection, VerifiedServingAuthorityCut)> {
        let projection = self.apply(receipt, binding)?;
        let cut = VerifiedServingAuthorityCut {
            lineage_id: binding.lineage_id.clone(),
            generation: binding.published_generation,
            owner_commit_id: binding.owner_commit_id.clone(),
            fencing_generation: binding.fencing_generation,
            lease_id: binding.lease_id.clone(),
            state_sha256: binding.candidate_state_sha256.clone(),
            result_sha256: hex::encode(receipt.result_sha256()),
            receipt_digest: binding.receipt_digest.clone(),
        };
        Ok((projection, cut))
    }

    #[cfg(any(test, debug_assertions))]
    pub fn reconcile_and_publish_fixture(
        &mut self,
        receipt: &ServingAuthorityReceiptFixture,
        binding: &ServingAuthorityBinding,
    ) -> ServingAuthorityResult<ServingAuthorityProjection> {
        self.apply(receipt, binding)
    }

    #[cfg(any(test, debug_assertions))]
    pub fn persist_fixture_boundary(
        &mut self,
        receipt: &ServingAuthorityReceiptFixture,
        binding: &ServingAuthorityBinding,
        boundary: ServingAuthorityTestBoundary,
    ) -> ServingAuthorityResult<()> {
        self.apply_until(receipt, binding, Some(boundary))
            .map(|_| ())
    }

    #[cfg(any(test, debug_assertions))]
    pub fn visible_projection_fixture(&self) -> Option<String> {
        self.envelope.payload().published_operation.clone()
    }

    fn apply(
        &mut self,
        receipt: &dyn SealedReceipt,
        binding: &ServingAuthorityBinding,
    ) -> ServingAuthorityResult<ServingAuthorityProjection> {
        self.apply_until(receipt, binding, None)?
            .ok_or(ServingAuthorityError::StateRegression)
    }

    fn apply_until(
        &mut self,
        receipt: &dyn SealedReceipt,
        binding: &ServingAuthorityBinding,
        #[cfg(any(test, debug_assertions))] stop: Option<ServingAuthorityTestBoundary>,
        #[cfg(not(any(test, debug_assertions)))] _stop: Option<()>,
    ) -> ServingAuthorityResult<Option<ServingAuthorityProjection>> {
        verify(&self.identity, receipt, binding)?;
        let binding_sha256 = hex::encode(Sha256::digest(binding.canonical_preimage()?));
        if let Some(existing) = self
            .envelope
            .payload()
            .operations
            .get(&binding.operation_id)
        {
            if existing.phase == Phase::Published && existing.binding_sha256 == binding_sha256 {
                return projection(&self.identity, binding, receipt, existing).map(Some);
            }
            if existing.binding_sha256 != binding_sha256
                || existing.prior_state_sha256 != binding.prior_state_sha256
                || existing.candidate_state_sha256 != binding.candidate_state_sha256
                || existing.receipt_digest != binding.receipt_digest
            {
                return Err(ServingAuthorityError::RetryConflict);
            }
        } else {
            if self.envelope.payload().operations.len() >= self.capacity {
                return Err(ServingAuthorityError::CapacityExceeded);
            }
            let prior = current_state_sha256(self.envelope.payload())?;
            if binding.prior_state_sha256 != prior {
                return Err(ServingAuthorityError::PriorStateMismatch);
            }
            let mut next = self.envelope.payload().clone();
            next.revision += 1;
            next.operations.insert(
                binding.operation_id.clone(),
                Operation {
                    phase: Phase::Pending,
                    generation: binding.published_generation,
                    binding_sha256: binding_sha256.clone(),
                    prior_state_sha256: binding.prior_state_sha256.clone(),
                    candidate_state_sha256: binding.candidate_state_sha256.clone(),
                    receipt_digest: binding.receipt_digest.clone(),
                },
            );
            self.envelope = self.store.commit(&self.envelope, next)?;
        }
        #[cfg(any(test, debug_assertions))]
        if stop == Some(ServingAuthorityTestBoundary::Pending) {
            return Ok(None);
        }
        if self
            .envelope
            .payload()
            .operations
            .get(&binding.operation_id)
            .ok_or(ServingAuthorityError::StateRegression)?
            .phase
            == Phase::Pending
        {
            self.advance(&binding.operation_id, Phase::Reconciled)?;
        }
        #[cfg(any(test, debug_assertions))]
        if stop == Some(ServingAuthorityTestBoundary::Reconciled) {
            return Ok(None);
        }
        self.advance(&binding.operation_id, Phase::Published)?;
        let operation = self
            .envelope
            .payload()
            .operations
            .get(&binding.operation_id)
            .ok_or(ServingAuthorityError::StateRegression)?;
        projection(&self.identity, binding, receipt, operation).map(Some)
    }

    fn advance(&mut self, operation_id: &str, phase: Phase) -> ServingAuthorityResult<()> {
        let mut next = self.envelope.payload().clone();
        next.revision += 1;
        let operation = next
            .operations
            .get_mut(operation_id)
            .ok_or(ServingAuthorityError::StateRegression)?;
        if operation.phase == phase {
            return Ok(());
        }
        operation.phase = phase;
        if phase == Phase::Published {
            next.published_operation = Some(operation_id.to_owned());
        }
        self.envelope = self.store.commit(&self.envelope, next)?;
        Ok(())
    }
}

fn verify(
    identity: &ServingAuthorityIdentity,
    receipt: &dyn SealedReceipt,
    binding: &ServingAuthorityBinding,
) -> ServingAuthorityResult<()> {
    binding.validate()?;
    if binding.trust_domain != identity.trust_domain
        || binding.polis_id != identity.polis_id
        || binding.lineage_id != receipt.lineage_id()
        || binding.operation_id != receipt.operation_id()
        || binding.action_class != receipt.action_class()
        || binding.adapter_kind != receipt.adapter_kind()
        || binding.adapter_version != receipt.adapter_version()
        || binding.published_generation != receipt.generation()
        || binding.receipt_digest != hex::encode(receipt.receipt_sha256())
        || Sha256::digest(binding.canonical_preimage()?).as_slice() != receipt.result_sha256()
    {
        return Err(ServingAuthorityError::ReceiptMismatch);
    }
    Ok(())
}

fn projection(
    identity: &ServingAuthorityIdentity,
    binding: &ServingAuthorityBinding,
    receipt: &dyn SealedReceipt,
    operation: &Operation,
) -> ServingAuthorityResult<ServingAuthorityProjection> {
    Ok(ServingAuthorityProjection {
        schema: PROJECTION_SCHEMA.into(),
        polis_ref: keyed_ref("polis", &identity.polis_id),
        lineage_ref: keyed_ref("lineage", &binding.lineage_id),
        generation: operation.generation,
        state_sha256: operation.candidate_state_sha256.clone(),
        result_sha256: hex::encode(receipt.result_sha256()),
        receipt_generation: receipt.generation(),
        receipt_digest: operation.receipt_digest.clone(),
        readiness: if operation.phase == Phase::Published {
            "published"
        } else {
            "pending"
        }
        .into(),
    })
}

fn current_state_sha256(state: &State) -> ServingAuthorityResult<String> {
    serde_jcs::to_vec(state)
        .map(|b| hex::encode(Sha256::digest(b)))
        .map_err(|_| ServingAuthorityError::Serialization)
}
fn keyed_ref(domain: &str, value: &str) -> String {
    hex::encode(Sha256::digest(format!(
        "ADL-SERVING-REF-V1\0{domain}\0{value}"
    )))
}
fn validate_identifier(value: &str) -> ServingAuthorityResult<()> {
    if value.is_empty() || value.len() > MAX_IDENTIFIER_BYTES {
        Err(ServingAuthorityError::InvalidIdentity)
    } else {
        Ok(())
    }
}
fn validate_digest(value: &str) -> ServingAuthorityResult<()> {
    if value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        Ok(())
    } else {
        Err(ServingAuthorityError::InvalidBinding)
    }
}

pub fn empty_state_sha256() -> String {
    current_state_sha256(&State::default()).expect("default state is serializable")
}
