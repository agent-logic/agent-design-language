use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const BIRTHDAY_CANDIDATE_SCHEMA: &str = "adl.birthday.candidate.v1";
pub const BIRTHDAY_DECISION_SCHEMA: &str = "adl.birthday.decision.v1";
pub const SUPPORTED_COGNITIVE_PROFILE: &str = "bounded_acp_v1";

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LifecycleEvent {
    BirthCandidate,
    ProcessStartup,
    OrdinaryTaskExecution,
    SnapshotCreation,
    WakeOrResume,
    RestoreFromCheckpoint,
    TestEnvironmentAdmission,
    CopiedState,
    NamedTestFixture,
    DormantRehydration,
    SimulationRun,
    InTransitMigration,
    ForcedSuspension,
    ShutdownAndRestart,
    ProvisionalCitizenRecord,
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceKind {
    StableName,
    IdentityRoot,
    ContinuityHead,
    MemoryGrounding,
    CapabilityEnvelope,
    CognitiveProfile,
    InheritedMoralContext,
    WitnessSet,
    Receipt,
    ReviewerValidation,
}

impl EvidenceKind {
    pub const REQUIRED: [Self; 10] = [
        Self::StableName,
        Self::IdentityRoot,
        Self::ContinuityHead,
        Self::MemoryGrounding,
        Self::CapabilityEnvelope,
        Self::CognitiveProfile,
        Self::InheritedMoralContext,
        Self::WitnessSet,
        Self::Receipt,
        Self::ReviewerValidation,
    ];
}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceVisibility {
    ReviewerVisible,
    RawPrivate,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EvidenceReference {
    pub kind: EvidenceKind,
    pub path: String,
    pub sha256: String,
    pub visibility: EvidenceVisibility,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContinuityCycle {
    pub cycle: u64,
    pub identity_root: String,
    pub continuity_head: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BirthdayCandidate {
    pub schema: String,
    pub candidate_id: String,
    pub lifecycle_event: LifecycleEvent,
    pub stable_name: String,
    pub identity_root: String,
    pub continuity_head: String,
    pub bounded_cycles: Vec<ContinuityCycle>,
    pub evidence: Vec<EvidenceReference>,
    pub cognitive_profile: String,
    #[serde(default)]
    pub public_claims: Vec<String>,
    pub packet_sha256: String,
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum BirthdayRejection {
    UnsupportedSchema,
    InvalidCandidateId,
    EmptyStableName,
    LifecycleLookalike { event: LifecycleEvent },
    MissingEvidence { kind: EvidenceKind },
    DuplicateEvidence { kind: EvidenceKind },
    UnsafeEvidencePath { kind: EvidenceKind },
    PrivateEvidence { kind: EvidenceKind },
    MalformedEvidenceDigest { kind: EvidenceKind },
    InsufficientBoundedCycles,
    DuplicateCycle { cycle: u64 },
    NonMonotonicCycles,
    MalformedIdentityRoot,
    MalformedContinuityHead,
    IdentityRootMismatch,
    ContinuityHeadMismatch,
    UnsupportedCognitiveProfile,
    ProhibitedPublicClaim { claim: String },
    PacketDigestMismatch,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BirthdayDecision {
    pub schema: String,
    pub candidate_id: String,
    pub accepted: bool,
    pub rejections: Vec<BirthdayRejection>,
}

#[derive(Serialize)]
struct DigestMaterial<'a> {
    schema: &'a str,
    candidate_id: &'a str,
    lifecycle_event: LifecycleEvent,
    stable_name: &'a str,
    identity_root: &'a str,
    continuity_head: &'a str,
    bounded_cycles: &'a [ContinuityCycle],
    evidence: &'a [EvidenceReference],
    cognitive_profile: &'a str,
    public_claims: &'a [String],
}

pub fn candidate_digest(candidate: &BirthdayCandidate) -> Result<String, serde_json::Error> {
    let mut evidence = candidate.evidence.clone();
    evidence.sort_by(|left, right| {
        left.kind
            .cmp(&right.kind)
            .then_with(|| left.path.cmp(&right.path))
            .then_with(|| left.sha256.cmp(&right.sha256))
            .then_with(|| left.visibility.cmp(&right.visibility))
    });
    let mut public_claims = candidate.public_claims.clone();
    public_claims.sort();
    let material = DigestMaterial {
        schema: &candidate.schema,
        candidate_id: &candidate.candidate_id,
        lifecycle_event: candidate.lifecycle_event,
        stable_name: &candidate.stable_name,
        identity_root: &candidate.identity_root,
        continuity_head: &candidate.continuity_head,
        bounded_cycles: &candidate.bounded_cycles,
        evidence: &evidence,
        cognitive_profile: &candidate.cognitive_profile,
        public_claims: &public_claims,
    };
    let bytes = serde_jcs::to_vec(&material)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

pub fn decide_birthday(candidate: &BirthdayCandidate) -> BirthdayDecision {
    let mut rejections = BTreeSet::new();

    if candidate.schema != BIRTHDAY_CANDIDATE_SCHEMA {
        rejections.insert(BirthdayRejection::UnsupportedSchema);
    }
    if !valid_candidate_id(&candidate.candidate_id) {
        rejections.insert(BirthdayRejection::InvalidCandidateId);
    }
    if candidate.stable_name.trim().is_empty() {
        rejections.insert(BirthdayRejection::EmptyStableName);
    }
    if candidate.lifecycle_event != LifecycleEvent::BirthCandidate {
        rejections.insert(BirthdayRejection::LifecycleLookalike {
            event: candidate.lifecycle_event,
        });
    }

    let mut evidence_by_kind: BTreeMap<EvidenceKind, &EvidenceReference> = BTreeMap::new();
    for evidence in &candidate.evidence {
        if evidence_by_kind.insert(evidence.kind, evidence).is_some() {
            rejections.insert(BirthdayRejection::DuplicateEvidence {
                kind: evidence.kind,
            });
        }
        if !safe_repository_path(&evidence.path) {
            rejections.insert(BirthdayRejection::UnsafeEvidencePath {
                kind: evidence.kind,
            });
        }
        if evidence.visibility == EvidenceVisibility::RawPrivate {
            rejections.insert(BirthdayRejection::PrivateEvidence {
                kind: evidence.kind,
            });
        }
        if !is_sha256(&evidence.sha256) {
            rejections.insert(BirthdayRejection::MalformedEvidenceDigest {
                kind: evidence.kind,
            });
        }
    }
    for kind in EvidenceKind::REQUIRED {
        if !evidence_by_kind.contains_key(&kind) {
            rejections.insert(BirthdayRejection::MissingEvidence { kind });
        }
    }

    if candidate.bounded_cycles.len() < 2 {
        rejections.insert(BirthdayRejection::InsufficientBoundedCycles);
    }
    let mut cycle_numbers = BTreeSet::new();
    for cycle in &candidate.bounded_cycles {
        if !cycle_numbers.insert(cycle.cycle) {
            rejections.insert(BirthdayRejection::DuplicateCycle { cycle: cycle.cycle });
        }
        if cycle.identity_root != candidate.identity_root {
            rejections.insert(BirthdayRejection::IdentityRootMismatch);
        }
    }
    if candidate
        .bounded_cycles
        .windows(2)
        .any(|pair| pair[0].cycle >= pair[1].cycle)
    {
        rejections.insert(BirthdayRejection::NonMonotonicCycles);
    }
    if candidate
        .bounded_cycles
        .last()
        .is_some_and(|cycle| cycle.continuity_head != candidate.continuity_head)
    {
        rejections.insert(BirthdayRejection::ContinuityHeadMismatch);
    }
    if !is_sha256(&candidate.identity_root) {
        rejections.insert(BirthdayRejection::MalformedIdentityRoot);
    }
    if !is_sha256(&candidate.continuity_head) {
        rejections.insert(BirthdayRejection::MalformedContinuityHead);
    }

    if candidate.cognitive_profile != SUPPORTED_COGNITIVE_PROFILE {
        rejections.insert(BirthdayRejection::UnsupportedCognitiveProfile);
    }
    for claim in &candidate.public_claims {
        rejections.insert(BirthdayRejection::ProhibitedPublicClaim {
            claim: claim.clone(),
        });
    }

    if candidate_digest(candidate).ok().as_deref() != Some(candidate.packet_sha256.as_str()) {
        rejections.insert(BirthdayRejection::PacketDigestMismatch);
    }

    BirthdayDecision {
        schema: BIRTHDAY_DECISION_SCHEMA.to_owned(),
        candidate_id: candidate.candidate_id.clone(),
        accepted: rejections.is_empty(),
        rejections: rejections.into_iter().collect(),
    }
}

fn valid_candidate_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn safe_repository_path(value: &str) -> bool {
    if value.is_empty()
        || value.starts_with('/')
        || value.starts_with('\\')
        || value.contains('\\')
        || value.as_bytes().get(1).copied() == Some(b':')
    {
        return false;
    }
    value
        .split('/')
        .all(|segment| !segment.is_empty() && segment != "." && segment != "..")
}
