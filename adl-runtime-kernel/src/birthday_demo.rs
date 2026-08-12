use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
    sync::Arc,
    time::Duration,
};

use ed25519_dalek::{Signer, SigningKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use thiserror::Error;

use crate::*;

pub const FIRST_BIRTHDAY_DEMO_SCHEMA: &str = "adl.first_birthday.demo_packet.v1";
const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";
const RETAINED_PACKET_PATH: &str = "demos/v0.92/first-birthday/positive.json";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthdayDemoCase {
    Positive,
    Startup,
    Wake,
    Restore,
    Snapshot,
    Admission,
    CopiedState,
    Simulation,
    NamedFixture,
    MissingEvidence(EvidenceKind),
    Interrupted,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BirthdayDemoStatus {
    Complete,
    Rejected,
    Incomplete,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "code", rename_all = "snake_case")]
pub enum BirthdayDemoRejection {
    Birthday { rejection: BirthdayRejection },
    InterruptedBeforeReceipt,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BirthdayDemoPacket {
    pub schema: String,
    pub status: BirthdayDemoStatus,
    pub case: BirthdayDemoCase,
    pub runtime_entrypoint: String,
    pub identity: BirthdayIdentityRecord,
    pub continuity: BirthdayContinuityRecord,
    pub capability: Option<CapabilityEnvelope>,
    pub cognitive_profile: Option<CognitiveProfile>,
    pub candidate: BirthdayCandidate,
    pub decision: BirthdayDecision,
    pub witness_packet: Option<BirthWitnessPacket>,
    pub rejections: Vec<BirthdayDemoRejection>,
    pub allowed_nondeterminism: Vec<String>,
    pub non_claims: Vec<String>,
    pub packet_sha256: String,
}

#[derive(Debug, Error)]
pub enum BirthdayDemoError {
    #[error("runtime birthday orchestration failed at {0}")]
    Stage(&'static str),
    #[error("runtime birthday orchestration rejected at {stage}: {detail}")]
    Rejected { stage: &'static str, detail: String },
    #[error("runtime birthday packet encoding failed")]
    Encoding,
}

/// Runs the trusted Runtime-owned Birthday composition path.
///
/// The API deliberately accepts only a case selector. Authority keys, trust
/// policies, evidence roots, and signing generations are established inside
/// the Runtime crate and cannot be nominated by the caller.
pub async fn run_first_birthday_demo(
    case: BirthdayDemoCase,
) -> Result<BirthdayDemoPacket, BirthdayDemoError> {
    let (identity, identity_evidence) = build_runtime_identity()?;
    let (continuity, verified_continuity) =
        build_runtime_continuity(&identity, &identity_evidence).await?;
    let witness_keys = (41_u8..=44)
        .map(|seed| SigningKey::from_bytes(&[seed; 32]))
        .collect::<Vec<_>>();
    let trusted = trusted_witnesses(&witness_keys);
    let roster_sha256 = birth_witness_roster_digest(&trusted)
        .map_err(|_| BirthdayDemoError::Stage("witness_roster"))?;

    let mut candidate = BirthdayCandidate {
        schema: BIRTHDAY_CANDIDATE_SCHEMA.to_owned(),
        candidate_id: "godel-agent-birthday-v092-001".to_owned(),
        lifecycle_event: lifecycle_for_case(&case),
        stable_name: identity.stable_name.clone(),
        identity_root: identity.identity_root.clone(),
        continuity_head: identity.continuity.head_sha256.clone(),
        bounded_cycles: continuity
            .cycles
            .iter()
            .enumerate()
            .map(|(index, _cycle)| ContinuityCycle {
                cycle: (index + 1) as u64,
                identity_root: identity.identity_root.clone(),
                // The Birthday contract's continuity head is the identity-bound
                // checkpoint. The signed Runtime cycles remain separately present
                // in `continuity` and prove forward operational continuity.
                continuity_head: identity.continuity.head_sha256.clone(),
            })
            .collect(),
        evidence: runtime_evidence(
            &identity,
            &continuity,
            identity_evidence.checkpoint_sha256(),
            &roster_sha256,
        ),
        cognitive_profile: SUPPORTED_COGNITIVE_PROFILE.to_owned(),
        public_claims: Vec::new(),
        packet_sha256: String::new(),
    };
    if let BirthdayDemoCase::MissingEvidence(kind) = case {
        candidate.evidence.retain(|entry| entry.kind != kind);
    }
    candidate.packet_sha256 =
        candidate_digest(&candidate).map_err(|_| BirthdayDemoError::Encoding)?;
    let decision = decide_birthday(&candidate);

    if matches!(case, BirthdayDemoCase::Interrupted) {
        return finish_packet(
            case,
            identity,
            continuity,
            candidate,
            decision,
            None,
            None,
            None,
            BirthdayDemoStatus::Incomplete,
            vec![BirthdayDemoRejection::InterruptedBeforeReceipt],
        );
    }
    if !decision.accepted {
        let rejections = decision
            .rejections
            .iter()
            .cloned()
            .map(|rejection| BirthdayDemoRejection::Birthday { rejection })
            .collect();
        return finish_packet(
            case,
            identity,
            continuity,
            candidate,
            decision,
            None,
            None,
            None,
            BirthdayDemoStatus::Rejected,
            rejections,
        );
    }

    let assembly = build_runtime_assembly()?;
    let (capability, capability_policy, capability_authority) =
        build_runtime_capability(&assembly, &candidate, &identity, &verified_continuity)?;
    let cognitive_profile = build_runtime_cognitive(
        &candidate,
        &identity,
        &verified_continuity,
        &capability,
        &capability_policy,
        &capability_authority,
    )?;
    let witness_packet = build_runtime_witnesses(&candidate, &decision, trusted, &witness_keys)?;
    finish_packet(
        case,
        identity,
        continuity,
        candidate,
        decision,
        Some(capability),
        Some(cognitive_profile),
        Some(witness_packet),
        BirthdayDemoStatus::Complete,
        Vec::new(),
    )
}

fn build_runtime_identity(
) -> Result<(BirthdayIdentityRecord, VerifiedBirthdayEvidence), BirthdayDemoError> {
    let identity_authority = IdentityAuthority::from_bytes("runtime-birthday-identity", &[11; 32]);
    let identity_keys = BTreeMap::from([(
        "runtime-birthday-identity".to_owned(),
        identity_authority.verifying_key(),
    )]);
    let binding = identity_authority
        .bind(
            "citizen-aster",
            "runtime-v3",
            "continuity-aster",
            7,
            BTreeSet::from(["birthday.identity".to_owned()]),
        )
        .map_err(|_| BirthdayDemoError::Stage("identity_binding"))?;
    let mut ledger = MemoryLedger::default();
    ledger
        .append(
            &binding,
            &identity_keys,
            MemoryClass::Identity,
            BTreeMap::from([
                (
                    "birthday.origin_event".to_owned(),
                    "origin-event-001".to_owned(),
                ),
                ("birthday.stable_name".to_owned(), "Aster".to_owned()),
                (
                    "birthday.alias.alias-one".to_owned(),
                    "Aster One".to_owned(),
                ),
            ]),
            None,
        )
        .map_err(|_| BirthdayDemoError::Stage("memory_grounding"))?;
    let checkpoint = ledger
        .checkpoint(&binding, &identity_keys, &identity_authority)
        .map_err(|_| BirthdayDemoError::Stage("memory_checkpoint"))?;

    let private_authority =
        PrivateStateAuthority::from_bytes("runtime-birthday-private", &[17; 32]);
    let private_keys = BTreeMap::from([(
        "runtime-birthday-private".to_owned(),
        private_authority.verifying_key(),
    )]);
    let projection = BTreeMap::from([(
        "identity_summary".to_owned(),
        "Aster continuity accepted".to_owned(),
    )]);
    let private_record = private_authority
        .issue_record(PrivateStateSealRequest {
            subject_id: binding.citizen_id.clone(),
            lineage_id: binding.continuity_id.clone(),
            sequence: 1,
            predecessor_hash: GENESIS.to_owned(),
            private_payload: b"runtime-private-state-not-exported".to_vec(),
            projection: projection.clone(),
            sanctuary_level: 1,
        })
        .map_err(|_| BirthdayDemoError::Stage("private_state"))?;
    let policy = BirthdayAuthorityPolicy::establish(
        identity_keys,
        private_keys,
        BirthdayEvidenceRequirements {
            identity_signing_key_id: "runtime-birthday-identity".to_owned(),
            private_state_signing_key_id: "runtime-birthday-private".to_owned(),
            identity_generation: 7,
            continuity_generation: 1,
            projection_generation: 1,
        },
        SanctuaryPolicy {
            allowed_principals: BTreeSet::from(["birthday-reviewer".to_owned()]),
            max_sanctuary_level: 1,
            allow_raw_export: false,
        },
        ProjectionRequest {
            principal: "birthday-reviewer".to_owned(),
            requested_fields: BTreeSet::from(["identity_summary".to_owned()]),
            raw_export: false,
        },
    )
    .map_err(|_| BirthdayDemoError::Stage("identity_policy"))?;
    let evidence = verify_birthday_evidence(
        &policy,
        &binding,
        &checkpoint,
        &private_record,
        &mut PrivateStateLineage::default(),
        &projection,
    )
    .map_err(|_| BirthdayDemoError::Stage("identity_evidence"))?;
    let mut candidate = BirthdayIdentityCandidate {
        schema: BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA.to_owned(),
        basis: IdentityBasis::OriginEvidence,
        stable_name: "Aster".to_owned(),
        identity_root: runtime_digest("identity-root-seed"),
        aliases: vec![AliasBinding {
            name: "Aster One".to_owned(),
            provenance_id: "alias-one".to_owned(),
        }],
        origin: OriginBinding {
            event_id: "origin-event-001".to_owned(),
            provenance_id: "origin-prov".to_owned(),
            reference: identity_ref("origin-binding", evidence.binding_sha256()),
        },
        continuity: ContinuityBinding {
            identity_root: runtime_digest("identity-root-seed"),
            head_sha256: evidence.checkpoint_head().to_owned(),
            reference: identity_ref("continuity-checkpoint", evidence.checkpoint_sha256()),
        },
        provenance: vec![
            identity_ref("origin-prov", evidence.binding_sha256()),
            identity_ref("alias-one", evidence.checkpoint_sha256()),
        ],
        witnesses: vec![
            identity_ref("private-record", evidence.private_record_sha256()),
            identity_ref(
                "governed-projection-witness",
                &evidence.projection_receipt().projection_sha256,
            ),
        ],
        governed_projection: identity_ref(
            "governed-projection",
            &evidence.projection_receipt().projection_sha256,
        ),
    };
    candidate.identity_root = derive_identity_root(&candidate, &evidence)
        .map_err(|_| BirthdayDemoError::Stage("identity_root"))?;
    candidate.continuity.identity_root = candidate.identity_root.clone();
    let record = build_birthday_identity(&candidate, &evidence)
        .map_err(|_| BirthdayDemoError::Stage("identity_record"))?;
    validate_birthday_identity_record(&record, &evidence)
        .map_err(|_| BirthdayDemoError::Stage("identity_validation"))?;
    Ok((record, evidence))
}

async fn build_runtime_continuity(
    identity: &BirthdayIdentityRecord,
    identity_evidence: &VerifiedBirthdayEvidence,
) -> Result<(BirthdayContinuityRecord, VerifiedBirthdayContinuity), BirthdayDemoError> {
    let authority = CheckpointAuthority::from_bytes("runtime-birthday-continuity", &[19; 32]);
    let root = transient_root();
    std::fs::create_dir_all(&root).map_err(|_| BirthdayDemoError::Stage("continuity_root"))?;
    let recorder = RuntimeRecorder::new(16);
    recorder.set_lifecycle(LifecycleState::Running);
    let state_sha256 = runtime_digest("live-kernel-state");
    let runtime_revision_sha256 = runtime_digest("runtime-v3-birthday-revision");
    let snapshot = LiveKernelSnapshot::new(
        state_sha256.clone(),
        runtime_revision_sha256.clone(),
        BTreeMap::new(),
    );
    let mut live =
        LiveContinuity::new(&root, "runtime-birthday-continuity", &[19; 32], snapshot, 0);
    let first = live
        .checkpoint(&recorder, Duration::from_secs(2))
        .await
        .map_err(|_| BirthdayDemoError::Stage("continuity_cycle_1"))?;
    recorder.set_lifecycle(LifecycleState::Stopping);
    let second = live
        .checkpoint(&recorder, Duration::from_secs(2))
        .await
        .map_err(|_| BirthdayDemoError::Stage("continuity_cycle_2"))?;
    let policy = BirthdayContinuityAuthorityPolicy::establish(
        BTreeMap::from([(
            "runtime-birthday-continuity".to_owned(),
            authority.verifying_key(),
        )]),
        "runtime-birthday-continuity",
        identity,
        identity_evidence,
        &state_sha256,
        &runtime_revision_sha256,
        LIVE_KERNEL_CHECKPOINT_SCHEMA,
        1,
        None,
    )
    .map_err(|_| BirthdayDemoError::Stage("continuity_policy"))?;
    let evidence = [
        BirthdayCycleEvidence { manifest: &first },
        BirthdayCycleEvidence { manifest: &second },
    ];
    let verified = verify_birthday_cycles(&policy, identity, &evidence)
        .map_err(|_| BirthdayDemoError::Stage("continuity_verification"))?;
    let record = build_birthday_continuity(identity, &verified)
        .map_err(|_| BirthdayDemoError::Stage("continuity_record"))?;
    validate_birthday_continuity_record(&record, identity, &verified)
        .map_err(|_| BirthdayDemoError::Stage("continuity_validation"))?;
    let verified_record = verify_birthday_continuity_record(&record, identity, &verified)
        .map_err(|_| BirthdayDemoError::Stage("continuity_verification"))?;
    let _ = std::fs::remove_dir_all(root);
    Ok((record, verified_record))
}

fn build_runtime_assembly() -> Result<LiveAssembly, BirthdayDemoError> {
    let root = transient_root();
    std::fs::create_dir_all(&root).map_err(|_| BirthdayDemoError::Stage("assembly_root"))?;
    let recorder = RuntimeRecorder::new(16);
    let permit_key = SigningKey::from_bytes(&[31; 32]);
    let executors = build_production_operation_executors_with_recorder(
        root.join("production"),
        recorder.clone(),
    )
    .map_err(|_| BirthdayDemoError::Stage("assembly_executors"))?;
    let reasoning = bootstrap_reasoning_services(recorder.clone())
        .map_err(|_| BirthdayDemoError::Stage("assembly_reasoning"))?;
    build_live_assembly(LiveBindings {
        recorder,
        canonical_ingress_capacity: 64,
        operation_executors: executors,
        permit_keys: BTreeMap::from([(
            "runtime-birthday-operator".to_owned(),
            permit_key.verifying_key(),
        )]),
        reasoning,
        time_source: Arc::new(SystemTimeSampleSource),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::from_millis(100),
            max_round_trip: Duration::from_millis(100),
            retry_delay: Duration::from_millis(10),
            refresh_interval: Duration::from_secs(60),
        },
    })
    .map_err(|_| BirthdayDemoError::Stage("assembly"))
}

fn runtime_evidence(
    identity: &BirthdayIdentityRecord,
    continuity: &BirthdayContinuityRecord,
    memory_sha: &str,
    roster_sha: &str,
) -> Vec<EvidenceReference> {
    let values = [
        (
            EvidenceKind::StableName,
            digest_bytes(identity.stable_name.as_bytes()),
        ),
        (EvidenceKind::IdentityRoot, identity.record_sha256.clone()),
        (
            EvidenceKind::ContinuityHead,
            continuity.record_sha256.clone(),
        ),
        (EvidenceKind::MemoryGrounding, memory_sha.to_owned()),
        (
            EvidenceKind::CapabilityEnvelope,
            digest_bytes(b"runtime-owned-capability-policy-v1"),
        ),
        (
            EvidenceKind::CognitiveProfile,
            digest_bytes(b"runtime-owned-cognitive-authority-v1"),
        ),
        (
            EvidenceKind::InheritedMoralContext,
            digest_bytes(b"v092-bounded-moral-context"),
        ),
        (EvidenceKind::WitnessSet, roster_sha.to_owned()),
        (
            EvidenceKind::Receipt,
            digest_bytes(b"runtime-owned-receipt-policy-v1"),
        ),
        (
            EvidenceKind::ReviewerValidation,
            digest_bytes(b"wp16-reviewed-terminal-ancestral"),
        ),
    ];
    values
        .into_iter()
        .map(|(kind, sha256)| EvidenceReference {
            kind,
            path: RETAINED_PACKET_PATH.to_owned(),
            sha256,
            visibility: EvidenceVisibility::ReviewerVisible,
        })
        .collect()
}

fn build_runtime_capability(
    assembly: &LiveAssembly,
    birthday: &BirthdayCandidate,
    identity: &BirthdayIdentityRecord,
    continuity: &VerifiedBirthdayContinuity,
) -> Result<
    (
        CapabilityEnvelope,
        CapabilityEnvelopePolicy,
        CapabilityAuthorityPolicy,
    ),
    BirthdayDemoError,
> {
    let evidence = vec![
        cap_ref(
            "birthday",
            CapabilityEvidenceKind::Birthday,
            &birthday.packet_sha256,
        ),
        cap_ref(
            "identity",
            CapabilityEvidenceKind::Identity,
            &identity.record_sha256,
        ),
        cap_ref(
            "retained",
            CapabilityEvidenceKind::RetainedCapability,
            &runtime_digest("retained-capability"),
        ),
        cap_ref(
            "provider",
            CapabilityEvidenceKind::Provider,
            &runtime_digest("runtime-demo-provider"),
        ),
        cap_ref(
            "model",
            CapabilityEvidenceKind::Model,
            &runtime_digest("runtime-demo-bounded-model"),
        ),
        cap_ref(
            "authority",
            CapabilityEvidenceKind::Authority,
            &runtime_digest("runtime-capability-authority"),
        ),
    ];
    let limits = CapabilityResourceLimits {
        max_prompt_tokens: 4096,
        max_output_tokens: 1024,
        max_tool_calls: 4,
        max_skill_invocations: 2,
        timeout_ms: 30_000,
        max_recurrence_depth: 1,
    };
    let policy = CapabilityEnvelopePolicy {
        schema: CAPABILITY_ENVELOPE_POLICY_SCHEMA.to_owned(),
        evidence: evidence.clone(),
        provider_models: vec![ProviderModelSelection {
            provider_id: "runtime-demo".to_owned(),
            model_id: "bounded".to_owned(),
            provenance_ids: vec!["model".to_owned(), "provider".to_owned()],
        }],
        allowed_tools: vec!["tool:runtime-status".to_owned()],
        allowed_skills: vec!["skill:review".to_owned()],
        allowed_grants: vec!["grant:invoke".to_owned()],
        required_denials: vec!["deny:raw-private-state".to_owned()],
        maximum_limits: limits.clone(),
        required_unsupported_claims: vec!["unsupported:unlimited-capacity".to_owned()],
    };
    let input = CapabilityEnvelopeInput {
        schema: CAPABILITY_ENVELOPE_INPUT_SCHEMA.to_owned(),
        birthday_candidate_sha256: birthday.packet_sha256.clone(),
        identity_record_sha256: identity.record_sha256.clone(),
        evidence,
        provider_model: policy.provider_models[0].clone(),
        tools: vec![cap_decl("tool:runtime-status", "retained")],
        skills: vec![cap_decl("skill:review", "retained")],
        grants: vec![cap_decl("grant:invoke", "authority")],
        denials: vec![cap_decl("deny:raw-private-state", "authority")],
        resource_limits: limits,
        unsupported_claims: vec![cap_decl("unsupported:unlimited-capacity", "retained")],
    };
    let authority = assembly
        .provision_capability_authority(&policy, continuity)
        .map_err(|_| BirthdayDemoError::Stage("capability_authority"))?;
    let envelope = build_capability_envelope_with_continuity(
        birthday, identity, continuity, &authority, &input, &policy,
    )
    .map_err(|_| BirthdayDemoError::Stage("capability_envelope"))?;
    validate_capability_envelope_with_continuity(
        &envelope, birthday, identity, continuity, &authority, &policy,
    )
    .map_err(|_| BirthdayDemoError::Stage("capability_validation"))?;
    Ok((envelope, policy, authority))
}

fn build_runtime_cognitive(
    b: &BirthdayCandidate,
    i: &BirthdayIdentityRecord,
    c: &VerifiedBirthdayContinuity,
    cap: &CapabilityEnvelope,
    cap_policy: &CapabilityEnvelopePolicy,
    capability_authority: &CapabilityAuthorityPolicy,
) -> Result<CognitiveProfile, BirthdayDemoError> {
    let evidence = vec![
        cog_ref(
            "identity",
            CognitiveEvidenceCategory::Identity,
            &i.record_sha256,
        ),
        cog_ref(
            "continuity",
            CognitiveEvidenceCategory::Continuity,
            &c.record().record_sha256,
        ),
        cog_ref(
            "memory",
            CognitiveEvidenceCategory::Memory,
            &runtime_digest("runtime-memory-grounding"),
        ),
        cog_ref(
            "capability",
            CognitiveEvidenceCategory::Capability,
            &cap.envelope_sha256,
        ),
        cog_ref(
            "tom",
            CognitiveEvidenceCategory::TheoryOfMind,
            &runtime_digest("bounded-theory-of-mind"),
        ),
        cog_ref(
            "intelligence",
            CognitiveEvidenceCategory::Intelligence,
            &runtime_digest("bounded-intelligence-profile"),
        ),
        cog_ref(
            "learning",
            CognitiveEvidenceCategory::GovernedLearning,
            &runtime_digest("governed-learning-profile"),
        ),
    ];
    let policy = CognitiveProfilePolicy {
        schema: COGNITIVE_PROFILE_POLICY_SCHEMA.to_owned(),
        evidence: evidence.clone(),
        allowed_fields: vec![
            AllowedCognitiveField {
                key: "learning_mode".to_owned(),
                allowed_values: vec!["governed".to_owned()],
                public: true,
            },
            AllowedCognitiveField {
                key: "private_reflection".to_owned(),
                allowed_values: vec!["retained".to_owned()],
                public: false,
            },
        ],
        required_nonclaims: vec![
            "no_reputation_inference".to_owned(),
            "no_personhood_inference".to_owned(),
            "no_rights_inference".to_owned(),
        ],
        redaction_policy_sha256: runtime_digest("birthday-redaction-policy-v1"),
        capability_policy: cap_policy.clone(),
    };
    let input = CognitiveProfileInput {
        schema: COGNITIVE_PROFILE_INPUT_SCHEMA.to_owned(),
        profile_id: "profile-aster".to_owned(),
        revision: 1,
        previous_profile_sha256: None,
        birthday_candidate_sha256: b.packet_sha256.clone(),
        identity_record_sha256: i.record_sha256.clone(),
        continuity_record_sha256: c.record().record_sha256.clone(),
        capability_envelope_sha256: cap.envelope_sha256.clone(),
        update_actor: "runtime-v3".to_owned(),
        update_reason: "first birthday integrated proof".to_owned(),
        added_fields: vec!["learning_mode".to_owned(), "private_reflection".to_owned()],
        removed_fields: Vec::new(),
        evidence,
        fields: vec![
            CognitiveProfileField {
                key: "learning_mode".to_owned(),
                value: "governed".to_owned(),
                evidence_ids: vec!["learning".to_owned()],
            },
            CognitiveProfileField {
                key: "private_reflection".to_owned(),
                value: "retained".to_owned(),
                evidence_ids: vec!["tom".to_owned()],
            },
        ],
        nonclaims: policy.required_nonclaims.clone(),
        redaction_policy_sha256: runtime_digest("birthday-redaction-policy-v1"),
    };
    let key = SigningKey::from_bytes(&[29; 32]);
    let policy_sha = canonical_cognitive_policy_digest(&policy)?;
    let evidence_sha = canonical_cognitive_evidence_digest(&input)?;
    let authority = CognitiveAuthorityPolicy::establish(
        "runtime-cognitive-board".to_owned(),
        "runtime-cognitive-key".to_owned(),
        1,
        key.verifying_key(),
        policy_sha.clone(),
        evidence_sha.clone(),
    )
    .map_err(|_| BirthdayDemoError::Stage("cognitive_policy"))?;
    let mut context = CognitiveAuthorityContext {
        authority_id: "runtime-cognitive-board".to_owned(),
        key_id: "runtime-cognitive-key".to_owned(),
        epoch: 1,
        context_sha256: String::new(),
        verifying_key_hex: hex::encode(key.verifying_key().as_bytes()),
    };
    context.context_sha256 = authority_context_payload_digest(&context)
        .map_err(|_| BirthdayDemoError::Stage("cognitive_context"))?;
    let canonical_input_sha256 = canonical_cognitive_input_digest(&input)?;
    let mut statement = CognitiveAuthorityStatement {
        schema: COGNITIVE_AUTHORITY_STATEMENT_SCHEMA.to_owned(),
        authority_context_sha256: digest_jcs(&context)?,
        profile_id: input.profile_id.clone(),
        revision: 1,
        previous_profile_sha256: None,
        canonical_input_sha256,
        policy_sha256: policy_sha,
        evidence_sha256: evidence_sha,
        signature: String::new(),
    };
    statement = statement
        .sign(&key)
        .map_err(|_| BirthdayDemoError::Stage("cognitive_statement"))?;
    let proof = CognitiveAuthorityProof {
        context,
        statement,
        rotation: None,
    };
    let profile = build_governed_cognitive_profile_with_continuity(
        b,
        i,
        c,
        capability_authority,
        cap,
        &input,
        &policy,
        &[],
        &authority,
        &proof,
    )
    .map_err(|rejections| BirthdayDemoError::Rejected {
        stage: "cognitive_profile",
        detail: format!("{rejections:?}"),
    })?;
    validate_governed_cognitive_profile_with_continuity(
        &profile,
        b,
        i,
        c,
        capability_authority,
        cap,
        &policy,
        &[],
        &authority,
    )
    .map_err(|rejections| BirthdayDemoError::Rejected {
        stage: "cognitive_validation",
        detail: format!("{rejections:?}"),
    })?;
    Ok(profile)
}

fn build_runtime_witnesses(
    candidate: &BirthdayCandidate,
    decision: &BirthdayDecision,
    trusted: Vec<TrustedBirthWitness>,
    keys: &[SigningKey],
) -> Result<BirthWitnessPacket, BirthdayDemoError> {
    let policy = BirthWitnessPolicy::establish(
        "runtime-v3-birthday-demo",
        candidate.packet_sha256.clone(),
        1,
        trusted,
    )
    .map_err(|_| BirthdayDemoError::Stage("witness_policy"))?;
    let evidence_sha = reviewed_evidence_set_digest(candidate)
        .map_err(|_| BirthdayDemoError::Stage("witness_evidence"))?;
    let attestations = BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(index, role)| {
            let mut a = BirthWitnessAttestation {
                schema: BIRTH_WITNESS_ATTESTATION_SCHEMA.to_owned(),
                witness_id: format!("runtime-witness-{}", index + 1),
                role,
                candidate_sha256: candidate.packet_sha256.clone(),
                evidence_set_sha256: evidence_sha.clone(),
                observed_generation: 1,
                decision: WitnessDecision::Accept,
                signing_key_id: format!("runtime-witness-key-{}", index + 1),
                signature: String::new(),
            };
            a.signature = hex::encode(
                keys[index]
                    .sign(&witness_signing_bytes(&a).expect("canonical witness bytes"))
                    .to_bytes(),
            );
            a
        })
        .collect::<Vec<_>>();
    let packet = build_birth_witness_packet(candidate, decision, &policy, &attestations)
        .map_err(|_| BirthdayDemoError::Stage("witness_packet"))?;
    validate_birth_witness_packet(&packet, candidate, decision, &policy, &attestations)
        .map_err(|_| BirthdayDemoError::Stage("witness_validation"))?;
    Ok(packet)
}

#[allow(clippy::too_many_arguments)]
fn finish_packet(
    case: BirthdayDemoCase,
    identity: BirthdayIdentityRecord,
    continuity: BirthdayContinuityRecord,
    candidate: BirthdayCandidate,
    decision: BirthdayDecision,
    capability: Option<CapabilityEnvelope>,
    cognitive_profile: Option<CognitiveProfile>,
    witness_packet: Option<BirthWitnessPacket>,
    status: BirthdayDemoStatus,
    rejections: Vec<BirthdayDemoRejection>,
) -> Result<BirthdayDemoPacket, BirthdayDemoError> {
    let mut packet = BirthdayDemoPacket {
        schema: FIRST_BIRTHDAY_DEMO_SCHEMA.to_owned(),
        status,
        case,
        runtime_entrypoint: "adl_runtime_kernel::run_first_birthday_demo".to_owned(),
        identity,
        continuity,
        capability,
        cognitive_profile,
        candidate,
        decision,
        witness_packet,
        rejections,
        allowed_nondeterminism: vec![
            "transient_runtime_directory".to_owned(),
            "native_host_class_recorded_outside_semantic_packet".to_owned(),
        ],
        non_claims: vec![
            "no_legal_personhood_claim".to_owned(),
            "no_consciousness_claim".to_owned(),
            "no_production_citizenship_claim".to_owned(),
            "no_completed_governance_claim".to_owned(),
            "no_publication_authorization".to_owned(),
        ],
        packet_sha256: String::new(),
    };
    packet.packet_sha256 = digest_jcs(&packet)?;
    Ok(packet)
}

fn lifecycle_for_case(case: &BirthdayDemoCase) -> LifecycleEvent {
    match case {
        BirthdayDemoCase::Positive
        | BirthdayDemoCase::MissingEvidence(_)
        | BirthdayDemoCase::Interrupted => LifecycleEvent::BirthCandidate,
        BirthdayDemoCase::Startup => LifecycleEvent::ProcessStartup,
        BirthdayDemoCase::Wake => LifecycleEvent::WakeOrResume,
        BirthdayDemoCase::Restore => LifecycleEvent::RestoreFromCheckpoint,
        BirthdayDemoCase::Snapshot => LifecycleEvent::SnapshotCreation,
        BirthdayDemoCase::Admission => LifecycleEvent::TestEnvironmentAdmission,
        BirthdayDemoCase::CopiedState => LifecycleEvent::CopiedState,
        BirthdayDemoCase::Simulation => LifecycleEvent::SimulationRun,
        BirthdayDemoCase::NamedFixture => LifecycleEvent::NamedTestFixture,
    }
}
fn identity_ref(id: &str, sha256: &str) -> IdentityReference {
    IdentityReference {
        id: id.to_owned(),
        path: RETAINED_PACKET_PATH.to_owned(),
        sha256: sha256.to_owned(),
    }
}
fn trusted_witnesses(keys: &[SigningKey]) -> Vec<TrustedBirthWitness> {
    BirthWitnessRole::REQUIRED
        .into_iter()
        .enumerate()
        .map(|(i, role)| TrustedBirthWitness {
            witness_id: format!("runtime-witness-{}", i + 1),
            role,
            signing_key_id: format!("runtime-witness-key-{}", i + 1),
            verifying_key: keys[i].verifying_key(),
        })
        .collect()
}
fn cap_ref(id: &str, kind: CapabilityEvidenceKind, sha: &str) -> CapabilityEvidenceReference {
    let issue = match kind {
        CapabilityEvidenceKind::Birthday => 5825,
        CapabilityEvidenceKind::Identity => 5826,
        CapabilityEvidenceKind::RetainedCapability
        | CapabilityEvidenceKind::Tool
        | CapabilityEvidenceKind::Skill
        | CapabilityEvidenceKind::Authority
        | CapabilityEvidenceKind::Limit => 4761,
        CapabilityEvidenceKind::Provider | CapabilityEvidenceKind::Model => 5665,
    };
    CapabilityEvidenceReference {
        id: id.to_owned(),
        kind,
        issue,
        path: RETAINED_PACKET_PATH.to_owned(),
        sha256: sha.to_owned(),
        revision_sha256: runtime_digest("runtime-capability-contract-v1"),
    }
}
fn cap_decl(id: &str, provenance: &str) -> CapabilityDeclaration {
    CapabilityDeclaration {
        id: id.to_owned(),
        provenance_ids: vec![provenance.to_owned()],
    }
}
fn cog_ref(id: &str, category: CognitiveEvidenceCategory, sha: &str) -> CognitiveEvidenceReference {
    let visibility = if category == CognitiveEvidenceCategory::GovernedLearning {
        CognitiveEvidenceVisibility::Public
    } else {
        CognitiveEvidenceVisibility::InternalRedacted
    };
    CognitiveEvidenceReference {
        id: id.to_owned(),
        category,
        path: RETAINED_PACKET_PATH.to_owned(),
        sha256: sha.to_owned(),
        revision_sha256: runtime_digest("runtime-cognitive-contract-v1"),
        visibility,
    }
}
fn transient_root() -> PathBuf {
    std::env::temp_dir().join(format!("adl-wp18-birthday-{}", uuid::Uuid::new_v4()))
}
fn digest_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}
fn runtime_digest(label: &str) -> String {
    digest_bytes(format!("adl.first_birthday.runtime_evidence.v1:{label}").as_bytes())
}
fn digest_jcs<T: Serialize>(value: &T) -> Result<String, BirthdayDemoError> {
    serde_jcs::to_vec(value)
        .map(|bytes| digest_bytes(&bytes))
        .map_err(|_| BirthdayDemoError::Encoding)
}
fn canonical_cognitive_input_digest(
    input: &CognitiveProfileInput,
) -> Result<String, BirthdayDemoError> {
    let mut value = input.clone();
    value.evidence.sort();
    value.fields.iter_mut().for_each(|field| {
        field.evidence_ids.sort();
        field.evidence_ids.dedup();
    });
    value.fields.sort();
    value.fields.dedup();
    value.added_fields.sort();
    value.added_fields.dedup();
    value.removed_fields.sort();
    value.removed_fields.dedup();
    value.nonclaims.sort();
    value.nonclaims.dedup();
    digest_jcs(&value)
}
fn canonical_cognitive_policy_digest(
    policy: &CognitiveProfilePolicy,
) -> Result<String, BirthdayDemoError> {
    let mut value = policy.clone();
    value.evidence.sort();
    for field in &mut value.allowed_fields {
        field.allowed_values.sort();
        field.allowed_values.dedup();
    }
    value.allowed_fields.sort();
    value.required_nonclaims.sort();
    value.required_nonclaims.dedup();
    digest_jcs(&value)
}
fn canonical_cognitive_evidence_digest(
    input: &CognitiveProfileInput,
) -> Result<String, BirthdayDemoError> {
    let mut evidence = input.evidence.clone();
    evidence.sort();
    digest_jcs(&evidence)
}
