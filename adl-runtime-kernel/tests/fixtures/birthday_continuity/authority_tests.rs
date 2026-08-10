//! PVF: deterministic-core, release-gating Birthday continuity proof with a
//! small resource profile. The positive case is the native semantic writer.

use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use super::{
    build_birthday_continuity, validate_birthday_continuity_record, verify_birthday_cycles,
    BirthdayContinuityAuthorityPolicy, BirthdayCycleEvidence, ContinuityGrade, ContinuityRejection,
    VerifiedBirthdayCycle,
};
use crate::{
    build_birthday_identity, derive_identity_root, verify_birthday_evidence, AliasBinding,
    BirthdayAuthorityPolicy as IdentityAuthorityPolicy, BirthdayEvidenceRequirements,
    BirthdayIdentityCandidate, BirthdayIdentityRecord, CheckpointAuthority, CheckpointManifest,
    IdentityAuthority, IdentityBasis, IdentityReference, MemoryClass, MemoryLedger,
    MigrationPolicy, PrivateStateAuthority, PrivateStateLineage, PrivateStateSealRequest,
    ProjectionRequest, SanctuaryPolicy, SnapshotEntry, BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA,
    CHECKPOINT_SCHEMA, LIVE_KERNEL_CHECKPOINT_SCHEMA,
};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";

fn verified_identity_fixture() -> (BirthdayIdentityRecord, crate::VerifiedBirthdayEvidence) {
    let identity_authority = IdentityAuthority::from_bytes("identity-birthday-key", &[11_u8; 32]);
    let identity_keys = BTreeMap::from([(
        "identity-birthday-key".to_owned(),
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
        .expect("signed identity binding");
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
                (
                    "birthday.alias.alias-north".to_owned(),
                    "North Star".to_owned(),
                ),
            ]),
            None,
        )
        .expect("accepted identity event");
    let checkpoint = ledger
        .checkpoint(&binding, &identity_keys, &identity_authority)
        .expect("signed memory checkpoint");

    let private_authority = PrivateStateAuthority::from_bytes("private-birthday-key", &[7_u8; 32]);
    let private_keys = BTreeMap::from([(
        "private-birthday-key".to_owned(),
        private_authority.verifying_key(),
    )]);
    let available_projection = BTreeMap::from([
        (
            "identity_summary".to_owned(),
            "Aster continuity accepted".to_owned(),
        ),
        ("witness_status".to_owned(), "governed".to_owned()),
    ]);
    let private_record = private_authority
        .issue_record(PrivateStateSealRequest {
            subject_id: binding.citizen_id.clone(),
            lineage_id: binding.continuity_id.clone(),
            sequence: 1,
            predecessor_hash: GENESIS.to_owned(),
            private_payload: b"raw private birthday state that must never be projected".to_vec(),
            projection: available_projection.clone(),
            sanctuary_level: 1,
        })
        .expect("signed private-state record");
    let sanctuary_policy = SanctuaryPolicy {
        allowed_principals: BTreeSet::from(["birthday-reviewer".to_owned()]),
        max_sanctuary_level: 1,
        allow_raw_export: false,
    };
    let projection_request = ProjectionRequest {
        principal: "birthday-reviewer".to_owned(),
        requested_fields: BTreeSet::from(["identity_summary".to_owned()]),
        raw_export: false,
    };
    let identity_policy = IdentityAuthorityPolicy::establish(
        identity_keys,
        private_keys,
        BirthdayEvidenceRequirements {
            identity_signing_key_id: "identity-birthday-key".to_owned(),
            private_state_signing_key_id: "private-birthday-key".to_owned(),
            identity_generation: 7,
            continuity_generation: 1,
            projection_generation: 1,
        },
        sanctuary_policy,
        projection_request,
    )
    .expect("trusted runtime identity policy");
    let evidence = verify_birthday_evidence(
        &identity_policy,
        &binding,
        &checkpoint,
        &private_record,
        &mut PrivateStateLineage::default(),
        &available_projection,
    )
    .expect("verified Birthday authority evidence");
    let mut candidate = BirthdayIdentityCandidate {
        schema: BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA.to_owned(),
        basis: IdentityBasis::OriginEvidence,
        stable_name: "Aster".to_owned(),
        identity_root: "0".repeat(64),
        aliases: vec![
            AliasBinding {
                name: "North Star".to_owned(),
                provenance_id: "alias-north".to_owned(),
            },
            AliasBinding {
                name: "Aster One".to_owned(),
                provenance_id: "alias-one".to_owned(),
            },
        ],
        origin: crate::OriginBinding {
            event_id: "origin-event-001".to_owned(),
            provenance_id: "origin-prov".to_owned(),
            reference: reference_with_digest("origin-binding", evidence.binding_sha256()),
        },
        continuity: crate::ContinuityBinding {
            identity_root: "0".repeat(64),
            head_sha256: evidence.checkpoint_head().to_owned(),
            reference: reference_with_digest("continuity-checkpoint", evidence.checkpoint_sha256()),
        },
        provenance: vec![
            reference_with_digest("origin-prov", evidence.binding_sha256()),
            reference_with_digest("alias-one", evidence.checkpoint_sha256()),
            reference_with_digest("alias-north", evidence.checkpoint_sha256()),
        ],
        witnesses: vec![
            reference_with_digest("private-record", evidence.private_record_sha256()),
            reference_with_digest(
                "governed-projection-witness",
                &evidence.projection_receipt().projection_sha256,
            ),
        ],
        governed_projection: reference_with_digest(
            "governed-projection",
            &evidence.projection_receipt().projection_sha256,
        ),
    };
    candidate.identity_root = derive_identity_root(&candidate, &evidence).expect("identity root");
    candidate.continuity.identity_root = candidate.identity_root.clone();
    let identity =
        build_birthday_identity(&candidate, &evidence).expect("accepted Birthday identity");
    (identity, evidence)
}

fn reference_with_digest(id: &str, sha256: &str) -> IdentityReference {
    IdentityReference {
        id: id.to_owned(),
        path: format!("evidence/identity/{id}.json"),
        sha256: sha256.to_owned(),
    }
}

fn signed_manifest(
    authority: &CheckpointAuthority,
    identity: &BirthdayIdentityRecord,
    generation: u64,
    previous: &str,
) -> CheckpointManifest {
    let mut manifest = CheckpointManifest {
        schema: CHECKPOINT_SCHEMA.to_owned(),
        generation,
        previous_integrity: Some(previous.to_owned()),
        accepted_through: generation,
        provenance: format!("birthday-identity:{}", identity.record_sha256),
        topology_hash: H.to_owned(),
        config_hash: "b".repeat(64),
        migration: MigrationPolicy::Exact,
        snapshots: vec![SnapshotEntry {
            service: "live_kernel".to_owned(),
            service_schema: LIVE_KERNEL_CHECKPOINT_SCHEMA.to_owned(),
            file: format!("evidence/cycle-{generation}.bin"),
            bytes: 4,
            checksum: "c".repeat(64),
        }],
        integrity: String::new(),
        signing_algorithm: String::new(),
        signing_key_id: String::new(),
        signature: String::new(),
    };
    authority.sign_manifest(&mut manifest).unwrap();
    manifest
}

fn material() -> (
    BirthdayIdentityRecord,
    BirthdayContinuityAuthorityPolicy,
    Vec<CheckpointManifest>,
) {
    let (identity, identity_evidence) = verified_identity_fixture();
    let authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let policy = BirthdayContinuityAuthorityPolicy::establish(
        BTreeMap::from([("runtime-continuity".to_owned(), authority.verifying_key())]),
        "runtime-continuity",
        &identity,
        &identity_evidence,
        H,
        "b".repeat(64),
        LIVE_KERNEL_CHECKPOINT_SCHEMA,
        1,
    )
    .unwrap();
    let first = signed_manifest(&authority, &identity, 1, &identity.continuity.head_sha256);
    let second = signed_manifest(&authority, &identity, 2, &first.integrity);
    (identity, policy, vec![first, second])
}

fn verify(
    policy: &BirthdayContinuityAuthorityPolicy,
    identity: &BirthdayIdentityRecord,
    manifests: &[CheckpointManifest],
) -> Result<Vec<VerifiedBirthdayCycle>, Vec<ContinuityRejection>> {
    let evidence = manifests
        .iter()
        .map(|manifest| BirthdayCycleEvidence { manifest })
        .collect::<Vec<_>>();
    verify_birthday_cycles(policy, identity, &evidence)
}

fn semantic_output_path(value: &str) -> Result<std::path::PathBuf, &'static str> {
    let relative = Path::new(value);
    if value.is_empty()
        || relative.is_absolute()
        || value.starts_with('\\')
        || value.contains('\\')
        || value.as_bytes().get(1).copied() == Some(b':')
        || relative
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err("semantic output must be a normalized repository-relative path");
    }
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("manifest directory must have a repository parent")?;
    let output = root.join(relative);
    if !output.starts_with(root.join(".csdlc/evidence/5827")) {
        return Err("semantic output must remain below .csdlc/evidence/5827");
    }
    Ok(output)
}

#[test]
fn continuity_record_replays_identically_across_two_signed_cycles() {
    let (identity, policy, manifests) = material();
    let verified = verify(&policy, &identity, &manifests).unwrap();
    let first = build_birthday_continuity(&identity, &verified).unwrap();
    let second = build_birthday_continuity(&identity, &verified).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.cycles.len(), 2);
    assert_eq!(first.grade, ContinuityGrade::EvidenceBacked);
    validate_birthday_continuity_record(&first, &identity, &verified).unwrap();
    if let Ok(path) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = semantic_output_path(&path).expect("safe semantic output path");
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, serde_jcs::to_vec(&first).unwrap()).unwrap();
    }
}

#[test]
fn missing_or_reordered_cycle_fails_closed() {
    let (identity, policy, manifests) = material();
    assert_eq!(
        verify(&policy, &identity, &manifests[..1]).unwrap_err(),
        vec![ContinuityRejection::InsufficientCycles]
    );
    let reversed = vec![manifests[1].clone(), manifests[0].clone()];
    let errors = verify(&policy, &identity, &reversed).unwrap_err();
    assert!(errors
        .iter()
        .any(|error| matches!(error, ContinuityRejection::WrongGeneration { .. })));
    assert!(errors
        .iter()
        .any(|error| matches!(error, ContinuityRejection::DiscontinuousPredecessor { .. })));
}

#[test]
fn verified_tokens_cannot_be_reordered_duplicated_or_relabelled() {
    let (identity, policy, manifests) = material();
    let verified = verify(&policy, &identity, &manifests).unwrap();

    let reordered = vec![verified[1].clone(), verified[0].clone()];
    assert!(build_birthday_continuity(&identity, &reordered)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, ContinuityRejection::DiscontinuousPredecessor { .. })));

    let duplicated = vec![verified[0].clone(), verified[0].clone()];
    assert!(build_birthday_continuity(&identity, &duplicated)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, ContinuityRejection::DuplicateCycle { .. })));

    let mut relabelled_identity = identity.clone();
    relabelled_identity.stable_name.push_str(" Copy");
    relabelled_identity.record_sha256 =
        crate::birthday_identity::record_digest(&relabelled_identity)
            .expect("self-consistent substituted record");
    assert!(build_birthday_continuity(&relabelled_identity, &verified)
        .unwrap_err()
        .contains(&ContinuityRejection::IdentityRecordMismatch));
}

#[test]
fn terminal_generation_overflow_fails_closed() {
    let (identity, identity_evidence) = verified_identity_fixture();
    let authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let policy = BirthdayContinuityAuthorityPolicy::establish(
        BTreeMap::from([("runtime-continuity".to_owned(), authority.verifying_key())]),
        "runtime-continuity",
        &identity,
        &identity_evidence,
        H,
        "b".repeat(64),
        LIVE_KERNEL_CHECKPOINT_SCHEMA,
        u64::MAX - 1,
    )
    .unwrap();
    let first = signed_manifest(
        &authority,
        &identity,
        u64::MAX - 1,
        &identity.continuity.head_sha256,
    );
    let second = signed_manifest(&authority, &identity, u64::MAX, &first.integrity);
    let third = signed_manifest(&authority, &identity, u64::MAX, &second.integrity);
    assert!(verify(&policy, &identity, &[first, second, third])
        .unwrap_err()
        .contains(&ContinuityRejection::GenerationOverflow));
}

#[test]
fn forged_witness_and_identity_substitution_fail_closed() {
    let (identity, policy, mut manifests) = material();
    manifests[1].accepted_through = 99;
    assert!(verify(&policy, &identity, &manifests)
        .unwrap_err()
        .contains(&ContinuityRejection::InvalidSignature { generation: 2 }));
    let mut substituted = identity.clone();
    substituted.identity_root = "d".repeat(64);
    substituted.record_sha256 = crate::birthday_identity::record_digest(&substituted).unwrap();
    assert!(verify(&policy, &substituted, &material().2)
        .unwrap_err()
        .contains(&ContinuityRejection::IdentityRecordMismatch));
}

#[test]
fn copied_state_and_host_paths_fail_closed() {
    let (identity, policy, manifests) = material();
    let copied = vec![manifests[0].clone(), manifests[0].clone()];
    assert!(verify(&policy, &identity, &copied)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, ContinuityRejection::DuplicateCycle { .. })));
    let (identity, identity_evidence) = verified_identity_fixture();
    let authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let policy = BirthdayContinuityAuthorityPolicy::establish(
        BTreeMap::from([("runtime-continuity".to_owned(), authority.verifying_key())]),
        "runtime-continuity",
        &identity,
        &identity_evidence,
        H,
        "b".repeat(64),
        LIVE_KERNEL_CHECKPOINT_SCHEMA,
        1,
    )
    .unwrap();
    let mut private = signed_manifest(&authority, &identity, 1, &identity.continuity.head_sha256);
    private.snapshots[0].file = "evidence/private/raw-state.bin".to_owned();
    authority.sign_manifest(&mut private).unwrap();
    let second = signed_manifest(&authority, &identity, 2, &private.integrity);
    let errors = verify_birthday_cycles(
        &policy,
        &identity,
        &[
            BirthdayCycleEvidence { manifest: &private },
            BirthdayCycleEvidence { manifest: &second },
        ],
    )
    .unwrap_err();
    assert!(errors
        .iter()
        .any(|error| matches!(error, ContinuityRejection::UnsafeWitnessPath { .. })));

    for path in [
        "evidence/continuity/private_state.bin",
        "evidence/continuity/raw-state.bin",
        "evidence/continuity/sealed_payload.json",
        "evidence/continuity/privateState.bin",
        "evidence/continuity/rawState.bin",
        "evidence/continuity/sealedPayload.json",
    ] {
        let mut unsafe_manifest =
            signed_manifest(&authority, &identity, 1, &identity.continuity.head_sha256);
        unsafe_manifest.snapshots[0].file = path.to_owned();
        authority.sign_manifest(&mut unsafe_manifest).unwrap();
        let next = signed_manifest(&authority, &identity, 2, &unsafe_manifest.integrity);
        let errors = verify_birthday_cycles(
            &policy,
            &identity,
            &[
                BirthdayCycleEvidence {
                    manifest: &unsafe_manifest,
                },
                BirthdayCycleEvidence { manifest: &next },
            ],
        )
        .unwrap_err();
        assert!(
            errors
                .iter()
                .any(|error| matches!(error, ContinuityRejection::UnsafeWitnessPath { .. })),
            "unsafe filename variant was accepted: {path}"
        );
    }
}

#[test]
fn wrong_signer_and_generation_fail_closed() {
    let (identity, policy, mut manifests) = material();
    let attacker = CheckpointAuthority::from_bytes("attacker", &[23; 32]);
    manifests[0] = signed_manifest(&attacker, &identity, 1, &identity.continuity.head_sha256);
    let errors = verify(&policy, &identity, &manifests).unwrap_err();
    assert!(errors
        .iter()
        .any(|error| matches!(error, ContinuityRejection::WrongSigner { generation: 1 })));
    manifests = material().2;
    manifests[1].generation = 8;
    let errors = verify(&policy, &identity, &manifests).unwrap_err();
    assert!(errors.iter().any(|error| matches!(
        error,
        ContinuityRejection::WrongGeneration {
            expected: 2,
            actual: 8
        }
    )));
}

#[test]
fn missing_runtime_witness_and_wrong_provenance_fail_closed() {
    let (identity, policy, mut manifests) = material();
    manifests[0].snapshots.clear();
    manifests[0].provenance = "restart-only".to_owned();
    let errors = verify(&policy, &identity, &manifests).unwrap_err();
    assert!(errors.iter().any(|error| matches!(
        error,
        ContinuityRejection::MissingRuntimeWitness { generation: 1 }
    )));
    assert!(errors.iter().any(|error| matches!(
        error,
        ContinuityRejection::IdentityProvenanceMismatch { generation: 1 }
    )));
}

#[test]
fn record_tamper_and_unknown_fields_fail_closed() {
    let (identity, policy, manifests) = material();
    let verified = verify(&policy, &identity, &manifests).unwrap();
    let mut record = build_birthday_continuity(&identity, &verified).unwrap();
    record.cycles[1].accepted_through += 1;
    assert!(
        validate_birthday_continuity_record(&record, &identity, &verified)
            .unwrap_err()
            .contains(&ContinuityRejection::RecordDigestMismatch)
    );
    let mut value =
        serde_json::to_value(build_birthday_continuity(&identity, &verified).unwrap()).unwrap();
    value.as_object_mut().unwrap().insert(
        "narrative_continuity".to_owned(),
        serde_json::Value::Bool(true),
    );
    assert!(serde_json::from_value::<super::BirthdayContinuityRecord>(value).is_err());
}
