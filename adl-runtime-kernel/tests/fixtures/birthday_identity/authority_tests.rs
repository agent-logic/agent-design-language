//! PVF: deterministic-core, release-gating identity-contract proof with a small
//! resource profile. The positive case is the sole native semantic-output writer.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

use crate::{
    build_birthday_identity, derive_identity_root, record_digest,
    validate_birthday_identity_record, verify_birthday_evidence, AliasBinding,
    BirthdayAuthorityPolicy, BirthdayEvidenceError, BirthdayEvidenceRequirements,
    BirthdayIdentityCandidate, IdentityAuthority, IdentityBasis, IdentityBinding,
    IdentityReference, IdentityRejection, MemoryCheckpoint, MemoryClass, MemoryLedger,
    PrivateStateAuthority, PrivateStateLineage, PrivateStateRecord, PrivateStateSealRequest,
    ProjectionRequest, SanctuaryPolicy, VerifiedBirthdayEvidence,
    BIRTHDAY_IDENTITY_CANDIDATE_SCHEMA,
};
use ed25519_dalek::VerifyingKey;

const GENESIS: &str = "0000000000000000000000000000000000000000000000000000000000000000";

#[derive(Clone)]
struct AuthorityMaterial {
    binding: IdentityBinding,
    checkpoint: MemoryCheckpoint,
    identity_keys: BTreeMap<String, VerifyingKey>,
    private_record: PrivateStateRecord,
    private_keys: BTreeMap<String, VerifyingKey>,
    projection: BTreeMap<String, String>,
    policy: SanctuaryPolicy,
    request: ProjectionRequest,
    requirements: BirthdayEvidenceRequirements,
    authority_policy: BirthdayAuthorityPolicy,
}

impl AuthorityMaterial {
    fn verify(&self) -> Result<VerifiedBirthdayEvidence, BirthdayEvidenceError> {
        verify_birthday_evidence(
            &self.authority_policy,
            &self.binding,
            &self.checkpoint,
            &self.private_record,
            &mut PrivateStateLineage::default(),
            &self.projection,
        )
    }

    fn reestablish_policy(&mut self) {
        self.authority_policy = BirthdayAuthorityPolicy::establish(
            self.identity_keys.clone(),
            self.private_keys.clone(),
            self.requirements.clone(),
            self.policy.clone(),
            self.request.clone(),
        )
        .expect("trusted runtime authority policy");
    }
}

fn authoritative_material() -> AuthorityMaterial {
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
    let projection = BTreeMap::from([
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
            projection: projection.clone(),
            sanctuary_level: 1,
        })
        .expect("signed private-state record");
    let policy = SanctuaryPolicy {
        allowed_principals: BTreeSet::from(["birthday-reviewer".to_owned()]),
        max_sanctuary_level: 1,
        allow_raw_export: false,
    };
    let request = ProjectionRequest {
        principal: "birthday-reviewer".to_owned(),
        requested_fields: BTreeSet::from(["identity_summary".to_owned()]),
        raw_export: false,
    };
    let requirements = BirthdayEvidenceRequirements {
        identity_signing_key_id: "identity-birthday-key".to_owned(),
        private_state_signing_key_id: "private-birthday-key".to_owned(),
        identity_generation: 7,
        continuity_generation: 1,
        projection_generation: 1,
    };
    let authority_policy = BirthdayAuthorityPolicy::establish(
        identity_keys.clone(),
        private_keys.clone(),
        requirements.clone(),
        policy.clone(),
        request.clone(),
    )
    .expect("trusted runtime authority policy");
    AuthorityMaterial {
        binding,
        checkpoint,
        identity_keys,
        private_record,
        private_keys,
        projection,
        policy,
        request,
        requirements,
        authority_policy,
    }
}

fn authoritative_candidate(evidence: &VerifiedBirthdayEvidence) -> BirthdayIdentityCandidate {
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
            reference: reference("origin-binding", evidence.binding_sha256()),
        },
        continuity: crate::ContinuityBinding {
            identity_root: "0".repeat(64),
            head_sha256: evidence.checkpoint_head().to_owned(),
            reference: reference("continuity-checkpoint", evidence.checkpoint_sha256()),
        },
        provenance: vec![
            reference("origin-prov", evidence.binding_sha256()),
            reference("alias-one", evidence.checkpoint_sha256()),
            reference("alias-north", evidence.checkpoint_sha256()),
        ],
        witnesses: vec![
            reference("private-record", evidence.private_record_sha256()),
            reference(
                "governed-projection-witness",
                &evidence.projection_receipt().projection_sha256,
            ),
        ],
        governed_projection: reference(
            "governed-projection",
            &evidence.projection_receipt().projection_sha256,
        ),
    };
    candidate.identity_root = derive_identity_root(&candidate, evidence).expect("identity root");
    candidate.continuity.identity_root = candidate.identity_root.clone();
    candidate
}

fn reference(id: &str, sha256: &str) -> IdentityReference {
    IdentityReference {
        id: id.to_owned(),
        path: format!("evidence/identity/{id}.json"),
        sha256: sha256.to_owned(),
    }
}

fn semantic_output_path(value: &str) -> Result<PathBuf, &'static str> {
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
    let repository_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .ok_or("manifest directory must have a repository parent")?;
    let output = repository_root.join(relative);
    if !output.starts_with(repository_root.join(".csdlc/evidence/5826")) {
        return Err("semantic output must remain below .csdlc/evidence/5826");
    }
    Ok(output)
}

#[test]
fn builds_from_signed_lineage_and_governed_projection() {
    let evidence = authoritative_material()
        .verify()
        .expect("verified evidence");
    let candidate = authoritative_candidate(&evidence);
    let record = build_birthday_identity(&candidate, &evidence).expect("identity record");
    validate_birthday_identity_record(&record, &evidence).expect("canonical record");
    assert_eq!(record.record_sha256, record_digest(&record).unwrap());
    assert_eq!(record.aliases[0].name, "Aster One");
    assert_eq!(record.projection_receipt.visible_fields.len(), 1);
    assert!(record.projection_receipt.redacted_fields.is_empty());
    assert!(!serde_json::to_string(&record)
        .unwrap()
        .contains("raw private birthday state"));

    if let Ok(output) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = semantic_output_path(&output).expect("safe semantic output path");
        fs::create_dir_all(path.parent().unwrap()).expect("semantic output directory");
        fs::write(
            path,
            serde_jcs::to_vec(&record).expect("canonical record bytes"),
        )
        .expect("semantic output");
    }
}

#[test]
fn rejects_forged_mismatched_and_stale_authorities() {
    let material = authoritative_material();

    let mut forged_binding = material.clone();
    forged_binding.binding.citizen_id = "invented-citizen".to_owned();
    assert_eq!(
        forged_binding.verify(),
        Err(BirthdayEvidenceError::IdentitySignature)
    );

    let mut forged_private = material.clone();
    forged_private.private_record.subject_id = "invented-citizen".to_owned();
    assert_eq!(
        forged_private.verify(),
        Err(BirthdayEvidenceError::PrivateSignature)
    );

    let other_private = PrivateStateAuthority::from_bytes("private-birthday-key", &[8_u8; 32]);
    let mut mismatched_subject = material.clone();
    mismatched_subject.private_keys = BTreeMap::from([(
        "private-birthday-key".to_owned(),
        other_private.verifying_key(),
    )]);
    mismatched_subject.private_record = other_private
        .issue_record(PrivateStateSealRequest {
            subject_id: "other-citizen".to_owned(),
            lineage_id: "continuity-aster".to_owned(),
            sequence: 1,
            predecessor_hash: GENESIS.to_owned(),
            private_payload: b"private".to_vec(),
            projection: material.projection.clone(),
            sanctuary_level: 1,
        })
        .unwrap();
    mismatched_subject.reestablish_policy();
    assert_eq!(
        mismatched_subject.verify(),
        Err(BirthdayEvidenceError::AuthoritySubjectMismatch)
    );

    let mut missing_identity_root = material.clone();
    missing_identity_root.requirements.identity_signing_key_id = "stale-identity-key".to_owned();
    assert!(matches!(
        BirthdayAuthorityPolicy::establish(
            missing_identity_root.identity_keys,
            missing_identity_root.private_keys,
            missing_identity_root.requirements,
            missing_identity_root.policy,
            missing_identity_root.request,
        ),
        Err(BirthdayEvidenceError::PolicyAuthorityMissing)
    ));

    let mut missing_private_root = material.clone();
    missing_private_root
        .requirements
        .private_state_signing_key_id = "stale-private-key".to_owned();
    assert!(matches!(
        BirthdayAuthorityPolicy::establish(
            missing_private_root.identity_keys,
            missing_private_root.private_keys,
            missing_private_root.requirements,
            missing_private_root.policy,
            missing_private_root.request,
        ),
        Err(BirthdayEvidenceError::PolicyAuthorityMissing)
    ));

    for expected in [
        BirthdayEvidenceError::IdentityGenerationMismatch,
        BirthdayEvidenceError::ContinuityGenerationMismatch,
        BirthdayEvidenceError::ProjectionGenerationMismatch,
    ] {
        let mut stale = material.clone();
        match expected {
            BirthdayEvidenceError::IdentityGenerationMismatch => {
                stale.requirements.identity_generation += 1
            }
            BirthdayEvidenceError::ContinuityGenerationMismatch => {
                stale.requirements.continuity_generation += 1
            }
            BirthdayEvidenceError::ProjectionGenerationMismatch => {
                stale.requirements.projection_generation += 1
            }
            _ => unreachable!(),
        }
        stale.reestablish_policy();
        assert_eq!(stale.verify(), Err(expected));
    }
}

#[test]
fn rejects_projection_tamper_and_raw_private_mislabelling() {
    let mut tampered = authoritative_material();
    tampered
        .projection
        .insert("identity_summary".to_owned(), "tampered".to_owned());
    assert_eq!(
        tampered.verify(),
        Err(BirthdayEvidenceError::PrivateProjection)
    );

    let private_authority = PrivateStateAuthority::from_bytes("private-birthday-key", &[7_u8; 32]);
    let mut raw = authoritative_material();
    raw.projection
        .insert("raw_private_state".to_owned(), "secret".to_owned());
    raw.private_record = private_authority
        .issue_record(PrivateStateSealRequest {
            subject_id: raw.binding.citizen_id.clone(),
            lineage_id: raw.binding.continuity_id.clone(),
            sequence: 1,
            predecessor_hash: GENESIS.to_owned(),
            private_payload: b"private".to_vec(),
            projection: raw.projection.clone(),
            sanctuary_level: 1,
        })
        .unwrap();
    raw.request.requested_fields = BTreeSet::from(["raw_private_state".to_owned()]);
    assert!(matches!(
        BirthdayAuthorityPolicy::establish(
            raw.identity_keys,
            raw.private_keys,
            raw.requirements,
            raw.policy,
            raw.request,
        ),
        Err(BirthdayEvidenceError::PolicyProjectionUnsafe)
    ));
}

#[test]
fn rejects_invented_provenance_wrong_continuity_and_projection_substitution() {
    let evidence = authoritative_material().verify().unwrap();
    let candidate = authoritative_candidate(&evidence);

    let mut invented = candidate.clone();
    invented.provenance[0].sha256 = "a".repeat(64);
    assert!(build_birthday_identity(&invented, &evidence)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, IdentityRejection::UnverifiedProvenance { .. })));

    let mut wrong_head = candidate.clone();
    wrong_head.continuity.head_sha256 = "b".repeat(64);
    assert!(build_birthday_identity(&wrong_head, &evidence)
        .unwrap_err()
        .contains(&IdentityRejection::ContinuityHeadMismatch));

    let mut missing_witness = candidate.clone();
    missing_witness.witnesses.clear();
    let errors = build_birthday_identity(&missing_witness, &evidence).unwrap_err();
    assert!(errors.contains(&IdentityRejection::MissingWitnesses));
    assert!(errors.contains(&IdentityRejection::MissingGovernedWitness));

    let mut projection_substitution = candidate;
    projection_substitution.governed_projection.sha256 = "c".repeat(64);
    assert!(build_birthday_identity(&projection_substitution, &evidence)
        .unwrap_err()
        .contains(&IdentityRejection::ProjectionAuthorityMismatch));
}

#[test]
fn replay_order_is_deterministic_and_record_tampering_fails() {
    let evidence = authoritative_material().verify().unwrap();
    let candidate = authoritative_candidate(&evidence);
    let first = build_birthday_identity(&candidate, &evidence).unwrap();
    let mut reordered = candidate;
    reordered.aliases.reverse();
    reordered.provenance.reverse();
    reordered.witnesses.reverse();
    let second = build_birthday_identity(&reordered, &evidence).unwrap();
    assert_eq!(first, second);

    let mut tampered = first;
    tampered.stable_name = "Tampered".to_owned();
    assert!(validate_birthday_identity_record(&tampered, &evidence)
        .unwrap_err()
        .contains(&IdentityRejection::RecordDigestMismatch));
}

#[test]
fn rejects_non_origin_bases_unknown_fields_and_unsafe_paths() {
    let evidence = authoritative_material().verify().unwrap();
    for basis in [
        IdentityBasis::DisplayName,
        IdentityBasis::BootAdmission,
        IdentityBasis::WakeState,
        IdentityBasis::Snapshot,
        IdentityBasis::CopiedState,
    ] {
        let mut candidate = authoritative_candidate(&evidence);
        candidate.basis = basis;
        assert_eq!(
            build_birthday_identity(&candidate, &evidence),
            Err(vec![IdentityRejection::UnsupportedBasis { basis }])
        );
    }

    let candidate = authoritative_candidate(&evidence);
    let mut value = serde_json::to_value(&candidate).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("reviewer_visible".to_owned(), serde_json::Value::Bool(true));
    assert!(serde_json::from_value::<BirthdayIdentityCandidate>(value).is_err());

    let mut unsafe_path = candidate;
    unsafe_path.origin.reference.path = "/tmp/private.json".to_owned();
    assert!(build_birthday_identity(&unsafe_path, &evidence)
        .unwrap_err()
        .iter()
        .any(|error| matches!(error, IdentityRejection::UnsafeReferencePath { .. })));
}

#[test]
fn semantic_output_rejects_host_paths_and_traversal() {
    for unsafe_path in [
        "",
        "/tmp/identity.json",
        "../identity.json",
        ".csdlc/evidence/5826/../identity.json",
        "C:\\tmp\\identity.json",
        "adl-runtime-kernel/identity.json",
    ] {
        assert!(semantic_output_path(unsafe_path).is_err());
    }
    assert!(
        semantic_output_path(".csdlc/evidence/5826/native-platform/linux-semantic.json").is_ok()
    );
}
