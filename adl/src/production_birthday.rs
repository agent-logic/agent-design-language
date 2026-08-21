use std::path::Path;

use adl_runtime_kernel::{
    BirthdayCandidate, BirthdayDecision, ProductionBirthdayError, ProductionBirthdayInput,
    ProductionBirthdayReceipt, ProductionBirthdayStore, ResidentAdaptiveLearningReceipt,
    VerifiedBirthWitnessBinding, VerifiedMemoryPalaceAuthority, VerifiedResidentCycle,
    VerifiedToolAuthorityBinding,
};
use serde::Serialize;
use sha2::{Digest, Sha256};

#[allow(clippy::too_many_arguments)]
pub fn activate_verified_resident_birthday(
    durable_root: &Path,
    transaction_id: &str,
    implementation_revision_sha256: &str,
    memory_palace: &VerifiedMemoryPalaceAuthority,
    resident_cycle: &VerifiedResidentCycle,
    adaptive_learning: &ResidentAdaptiveLearningReceipt,
    tool_authority: VerifiedToolAuthorityBinding,
    birth_witness: VerifiedBirthWitnessBinding,
    trusted_time: &dyn adl_runtime_kernel::TrustedTime,
    candidate: BirthdayCandidate,
    decision: BirthdayDecision,
) -> Result<ProductionBirthdayReceipt, ProductionBirthdayError> {
    let memory_palace_authority_sha256 = canonical_sha256(&(
        memory_palace.identity().record_sha256.as_str(),
        memory_palace.continuity().record().record_sha256.as_str(),
    ))?;
    let adaptive_learning_receipt_sha256 = canonical_sha256(adaptive_learning)?;
    let input = ProductionBirthdayInput {
        resident_id: resident_cycle.resident_id.clone(),
        cycle_id: resident_cycle.cycle_id.clone(),
        transaction_id: transaction_id.to_string(),
        implementation_revision_sha256: implementation_revision_sha256.to_string(),
        identity_root_sha256: memory_palace.identity().identity_root.clone(),
        continuity_head_sha256: memory_palace.continuity().continuity_head().to_string(),
        memory_palace_authority_sha256,
        capability_envelope_sha256: resident_cycle.capability.envelope_sha256().to_string(),
        cognitive_profile_sha256: resident_cycle
            .cognitive_profile
            .profile_sha256()
            .to_string(),
        adaptive_learning_receipt_sha256,
        birth_witness,
        trusted_time_unix_millis: trusted_time.now_unix_millis(),
        candidate,
        decision,
        tool_authority,
    };
    ProductionBirthdayStore::open(durable_root)?.activate(&input)
}

fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, ProductionBirthdayError> {
    serde_jcs::to_vec(value)
        .map(|bytes| hex::encode(Sha256::digest(bytes)))
        .map_err(|_| ProductionBirthdayError::InvalidInput)
}
