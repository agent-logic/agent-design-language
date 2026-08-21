use std::collections::{BTreeMap, BTreeSet};

use crate::{
    birthday_continuity::BirthdayContinuityAuthorityPolicy,
    birthday_identity::{
        validate_birthday_identity_record, BirthdayAuthorityPolicy, BirthdayEvidenceError,
    },
    verify_birthday_continuity_record, verify_birthday_cycles, verify_birthday_evidence,
    BirthdayContinuityRecord, BirthdayCycleEvidence, BirthdayIdentityRecord, CheckpointManifest,
    ContinuityRejection, IdentityBinding, IdentityRejection, MemoryCheckpoint, PrivateStateLineage,
    PrivateStateRecord, ProjectionRequest, SanctuaryPolicy, VerifiedBirthdayContinuity,
    VerifiedBirthdayEvidence,
};

#[derive(Clone)]
pub struct BirthdayAuthorityBootstrap {
    pub identity_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    pub private_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    pub identity_requirements: crate::BirthdayEvidenceRequirements,
    pub sanctuary_policy: SanctuaryPolicy,
    pub projection_request: ProjectionRequest,
    pub continuity_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    pub continuity_signing_key_id: String,
    pub continuity_service_schema: String,
    pub first_continuity_generation: u64,
    pub first_previous_integrity: Option<String>,
}

pub fn birthday_authority_bootstrap_from_runtime_keys(
    identity_key_id: impl Into<String>,
    identity_key: ed25519_dalek::VerifyingKey,
    private_state_key_id: impl Into<String>,
    private_state_key: ed25519_dalek::VerifyingKey,
    continuity_key_id: impl Into<String>,
    continuity_key: ed25519_dalek::VerifyingKey,
    identity_generation: u64,
    continuity_generation: u64,
    projection_generation: u64,
) -> BirthdayAuthorityBootstrap {
    let identity_key_id = identity_key_id.into();
    let private_state_key_id = private_state_key_id.into();
    let continuity_key_id = continuity_key_id.into();
    BirthdayAuthorityBootstrap {
        identity_keys: BTreeMap::from([(identity_key_id.clone(), identity_key)]),
        private_keys: BTreeMap::from([(private_state_key_id.clone(), private_state_key)]),
        identity_requirements: crate::BirthdayEvidenceRequirements {
            identity_signing_key_id: identity_key_id,
            private_state_signing_key_id: private_state_key_id,
            identity_generation,
            continuity_generation,
            projection_generation,
        },
        sanctuary_policy: SanctuaryPolicy {
            allowed_principals: BTreeSet::from(["memory-palace-runtime".to_owned()]),
            max_sanctuary_level: 1,
            allow_raw_export: false,
        },
        projection_request: ProjectionRequest {
            principal: "memory-palace-runtime".to_owned(),
            requested_fields: BTreeSet::from(["identity_summary".to_owned()]),
            raw_export: false,
        },
        continuity_keys: BTreeMap::from([(continuity_key_id.clone(), continuity_key)]),
        continuity_signing_key_id: continuity_key_id,
        continuity_service_schema: crate::LIVE_KERNEL_CHECKPOINT_SCHEMA.to_owned(),
        first_continuity_generation: continuity_generation,
        first_previous_integrity: None,
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedMemoryPalaceAuthority {
    identity: BirthdayIdentityRecord,
    identity_evidence: VerifiedBirthdayEvidence,
    continuity: VerifiedBirthdayContinuity,
}

impl VerifiedMemoryPalaceAuthority {
    pub fn identity(&self) -> &BirthdayIdentityRecord {
        &self.identity
    }

    pub fn identity_evidence(&self) -> &VerifiedBirthdayEvidence {
        &self.identity_evidence
    }

    pub fn continuity(&self) -> &VerifiedBirthdayContinuity {
        &self.continuity
    }
}

#[derive(Clone)]
pub struct RuntimeMemoryPalaceProvisioner {
    identity_policy: BirthdayAuthorityPolicy,
    continuity_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
    continuity_signing_key_id: String,
    topology_hash: String,
    config_hash: String,
    service_schema: String,
    first_generation: u64,
    first_previous_integrity: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryPalaceAuthorityError {
    IdentityEvidence(BirthdayEvidenceError),
    IdentityRecord(Vec<IdentityRejection>),
    ContinuityPolicy(ContinuityRejection),
    ContinuityCycles(Vec<ContinuityRejection>),
    ContinuityRecord(Vec<ContinuityRejection>),
    IdentityLineageMismatch,
}

pub struct MemoryPalaceAuthorityEvidence<'a> {
    pub identity_record: &'a BirthdayIdentityRecord,
    pub identity_binding: &'a IdentityBinding,
    pub identity_checkpoint: &'a MemoryCheckpoint,
    pub private_record: &'a PrivateStateRecord,
    pub private_lineage: &'a mut PrivateStateLineage,
    pub available_projection: &'a BTreeMap<String, String>,
    pub continuity_record: &'a BirthdayContinuityRecord,
    pub continuity_manifests: &'a [CheckpointManifest],
}

impl RuntimeMemoryPalaceProvisioner {
    pub(crate) fn from_bootstrap(
        bootstrap: BirthdayAuthorityBootstrap,
        topology_hash: impl Into<String>,
        config_hash: impl Into<String>,
    ) -> Result<Self, MemoryPalaceAuthorityBootstrapError> {
        let identity_policy = BirthdayAuthorityPolicy::establish(
            bootstrap.identity_keys,
            bootstrap.private_keys,
            bootstrap.identity_requirements,
            bootstrap.sanctuary_policy,
            bootstrap.projection_request,
        )
        .map_err(MemoryPalaceAuthorityBootstrapError::IdentityPolicy)?;
        Self::establish(
            identity_policy,
            bootstrap.continuity_keys,
            bootstrap.continuity_signing_key_id,
            topology_hash,
            config_hash,
            bootstrap.continuity_service_schema,
            bootstrap.first_continuity_generation,
            bootstrap.first_previous_integrity,
        )
        .map_err(MemoryPalaceAuthorityBootstrapError::ContinuityPolicy)
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn establish(
        identity_policy: BirthdayAuthorityPolicy,
        continuity_keys: BTreeMap<String, ed25519_dalek::VerifyingKey>,
        continuity_signing_key_id: impl Into<String>,
        topology_hash: impl Into<String>,
        config_hash: impl Into<String>,
        service_schema: impl Into<String>,
        first_generation: u64,
        first_previous_integrity: Option<String>,
    ) -> Result<Self, ContinuityRejection> {
        let continuity_signing_key_id = continuity_signing_key_id.into();
        let topology_hash = topology_hash.into();
        let config_hash = config_hash.into();
        let service_schema = service_schema.into();
        if !continuity_keys.contains_key(&continuity_signing_key_id)
            || !is_sha256(&topology_hash)
            || !is_sha256(&config_hash)
            || service_schema.trim().is_empty()
            || first_generation == 0
            || (first_generation == 1 && first_previous_integrity.is_some())
            || (first_generation > 1 && !first_previous_integrity.as_deref().is_some_and(is_sha256))
        {
            return Err(ContinuityRejection::PolicyInvalid);
        }
        Ok(Self {
            identity_policy,
            continuity_keys,
            continuity_signing_key_id,
            topology_hash,
            config_hash,
            service_schema,
            first_generation,
            first_previous_integrity,
        })
    }

    pub fn provision(
        &self,
        evidence: MemoryPalaceAuthorityEvidence<'_>,
    ) -> Result<VerifiedMemoryPalaceAuthority, MemoryPalaceAuthorityError> {
        let identity_evidence = verify_birthday_evidence(
            &self.identity_policy,
            evidence.identity_binding,
            evidence.identity_checkpoint,
            evidence.private_record,
            evidence.private_lineage,
            evidence.available_projection,
        )
        .map_err(MemoryPalaceAuthorityError::IdentityEvidence)?;
        validate_birthday_identity_record(evidence.identity_record, &identity_evidence)
            .map_err(MemoryPalaceAuthorityError::IdentityRecord)?;
        let continuity_policy = BirthdayContinuityAuthorityPolicy::establish(
            self.continuity_keys.clone(),
            self.continuity_signing_key_id.clone(),
            evidence.identity_record,
            &identity_evidence,
            self.topology_hash.clone(),
            self.config_hash.clone(),
            self.service_schema.clone(),
            self.first_generation,
            self.first_previous_integrity.clone(),
        )
        .map_err(MemoryPalaceAuthorityError::ContinuityPolicy)?;
        let cycle_evidence = evidence
            .continuity_manifests
            .iter()
            .map(|manifest| BirthdayCycleEvidence { manifest })
            .collect::<Vec<_>>();
        let verified_cycles = verify_birthday_cycles(
            &continuity_policy,
            evidence.identity_record,
            &cycle_evidence,
        )
        .map_err(MemoryPalaceAuthorityError::ContinuityCycles)?;
        let continuity = verify_birthday_continuity_record(
            evidence.continuity_record,
            evidence.identity_record,
            &verified_cycles,
        )
        .map_err(MemoryPalaceAuthorityError::ContinuityRecord)?;
        if continuity.record().identity_root != evidence.identity_record.identity_root
            || continuity.record().identity_record_sha256 != evidence.identity_record.record_sha256
        {
            return Err(MemoryPalaceAuthorityError::IdentityLineageMismatch);
        }
        Ok(VerifiedMemoryPalaceAuthority {
            identity: evidence.identity_record.clone(),
            identity_evidence,
            continuity,
        })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum MemoryPalaceAuthorityBootstrapError {
    IdentityPolicy(BirthdayEvidenceError),
    ContinuityPolicy(ContinuityRejection),
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}
