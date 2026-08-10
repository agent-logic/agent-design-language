//! PVF: deterministic-core, release-gating Birthday continuity proof with a
//! small resource profile. The positive case is the native semantic writer.

pub use adl_runtime_kernel::{
    birthday_identity, BirthdayIdentityRecord, CheckpointAuthority, CheckpointManifest,
    IdentityReference, MigrationPolicy, SnapshotEntry, BIRTHDAY_IDENTITY_RECORD_SCHEMA,
    CHECKPOINT_SCHEMA, LIVE_KERNEL_CHECKPOINT_SCHEMA,
};

#[path = "../src/birthday_continuity.rs"]
mod under_test;

use std::{
    collections::BTreeMap,
    path::{Component, Path},
};

use sha2::Digest;
use under_test::{
    build_birthday_continuity, validate_birthday_continuity_record, verify_birthday_cycles,
    BirthdayContinuityAuthorityPolicy, BirthdayCycleEvidence, ContinuityGrade, ContinuityRejection,
    VerifiedBirthdayCycle,
};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn identity() -> BirthdayIdentityRecord {
    let mut value: BirthdayIdentityRecord = serde_json::from_str(include_str!(
        "fixtures/birthday_continuity/identity_record.json"
    ))
    .unwrap();
    value.record_sha256 = birthday_identity::record_digest(&value).unwrap();
    value
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

fn reference(generation: u64, manifest: &CheckpointManifest) -> IdentityReference {
    IdentityReference {
        id: format!("cycle-{generation}"),
        path: format!("evidence/continuity/cycle-{generation}.json"),
        sha256: hex::encode(sha2::Sha256::digest(serde_jcs::to_vec(manifest).unwrap())),
    }
}

fn material() -> (
    BirthdayIdentityRecord,
    BirthdayContinuityAuthorityPolicy,
    Vec<CheckpointManifest>,
) {
    let identity = identity();
    let authority = CheckpointAuthority::from_bytes("runtime-continuity", &[19; 32]);
    let policy = BirthdayContinuityAuthorityPolicy::establish(
        BTreeMap::from([("runtime-continuity".to_owned(), authority.verifying_key())]),
        "runtime-continuity",
        &identity,
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
        .map(|manifest| BirthdayCycleEvidence {
            manifest,
            reference: reference(manifest.generation, manifest),
        })
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
fn forged_witness_and_identity_substitution_fail_closed() {
    let (identity, policy, mut manifests) = material();
    manifests[1].accepted_through = 99;
    assert!(verify(&policy, &identity, &manifests)
        .unwrap_err()
        .contains(&ContinuityRejection::InvalidSignature { generation: 2 }));
    let mut substituted = identity.clone();
    substituted.identity_root = "d".repeat(64);
    substituted.record_sha256 = birthday_identity::record_digest(&substituted).unwrap();
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
    let evidence = manifests
        .iter()
        .map(|manifest| BirthdayCycleEvidence {
            manifest,
            reference: IdentityReference {
                id: format!("cycle-{}", manifest.generation),
                path: "/private/raw-state.json".to_owned(),
                sha256: reference(manifest.generation, manifest).sha256,
            },
        })
        .collect::<Vec<_>>();
    assert!(verify_birthday_cycles(&policy, &identity, &evidence)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, ContinuityRejection::UnsafeWitnessPath { .. })));
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
    assert!(serde_json::from_value::<under_test::BirthdayContinuityRecord>(value).is_err());
}
