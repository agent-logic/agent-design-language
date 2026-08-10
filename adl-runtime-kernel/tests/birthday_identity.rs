//! PVF: deterministic-core, release-gating identity-contract proof with a small
//! resource profile. The positive case is the sole native semantic-output writer.

use std::{
    fs,
    path::{Component, Path, PathBuf},
};

use adl_runtime_kernel::{
    build_birthday_identity, derive_identity_root, record_digest,
    validate_birthday_identity_record, AliasBinding, BirthdayIdentityCandidate, IdentityBasis,
    IdentityReference, IdentityReferenceVisibility, IdentityRejection,
};

fn fixture(name: &str) -> String {
    format!(
        "{}/tests/fixtures/birthday_identity/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn valid_candidate() -> BirthdayIdentityCandidate {
    serde_json::from_str(&fs::read_to_string(fixture("valid.json")).expect("read valid fixture"))
        .expect("parse valid fixture")
}

fn valid_value() -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(fixture("valid.json")).expect("read valid fixture"))
        .expect("parse valid fixture value")
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
fn builds_canonical_identity_and_emits_semantic_output() {
    let candidate = valid_candidate();
    assert_eq!(
        candidate.identity_root,
        derive_identity_root(&candidate).expect("derive identity root")
    );
    let record = build_birthday_identity(&candidate).expect("valid identity record");
    assert_eq!(record.identity_root, candidate.identity_root);
    assert_eq!(
        record.record_sha256,
        record_digest(&record).expect("record digest")
    );
    validate_birthday_identity_record(&record).expect("validate canonical identity record");
    assert_eq!(
        record
            .aliases
            .iter()
            .map(|alias| alias.name.as_str())
            .collect::<Vec<_>>(),
        vec!["Aster One", "North Star"]
    );
    assert_eq!(record.provenance[0].id, "alias-north");
    assert_eq!(record.witnesses[0].id, "witness-a");
    assert_eq!(
        record.redaction_policy.redacted_fields,
        vec!["private_state", "sealed_payload"]
    );

    if let Ok(output) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = semantic_output_path(&output).expect("safe semantic output path");
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create semantic output directory");
        }
        fs::write(&path, serde_jcs::to_vec(&record).expect("canonical record"))
            .expect("write semantic output");
    }
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
        assert!(
            semantic_output_path(unsafe_path).is_err(),
            "unsafe semantic output path accepted: {unsafe_path}"
        );
    }
    let safe = ".csdlc/evidence/5826/native-platform/linux-semantic.json";
    assert_eq!(
        semantic_output_path(safe).expect("safe evidence path"),
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("repository root")
            .join(safe)
    );
}

#[test]
fn record_validator_rejects_digest_tampering_and_noncanonical_order() {
    let record = build_birthday_identity(&valid_candidate()).expect("valid identity record");

    let mut tampered = record.clone();
    tampered.stable_name = "Tampered".to_owned();
    let tampered_rejections =
        validate_birthday_identity_record(&tampered).expect_err("tampered record must reject");
    assert!(tampered_rejections.contains(&IdentityRejection::RecordDigestMismatch));

    let mut reordered = record;
    reordered.aliases.reverse();
    reordered.record_sha256 = record_digest(&reordered).expect("digest reordered record");
    assert_eq!(
        validate_birthday_identity_record(&reordered),
        Err(vec![IdentityRejection::NonCanonicalRecord])
    );
}

#[test]
fn replay_and_input_order_produce_identical_records() {
    let candidate = valid_candidate();
    let first = build_birthday_identity(&candidate).expect("first record");
    let mut reordered = candidate.clone();
    reordered.aliases.reverse();
    reordered.provenance.reverse();
    reordered.witnesses.reverse();
    reordered.redaction_policy.redacted_fields.reverse();
    let second = build_birthday_identity(&reordered).expect("reordered record");
    assert_eq!(first, second);
    assert_eq!(
        serde_jcs::to_vec(&first).expect("first canonical record"),
        serde_jcs::to_vec(&second).expect("second canonical record")
    );

    let mut alias_added = candidate.clone();
    alias_added.aliases.push(AliasBinding {
        name: "Aster Two".to_owned(),
        provenance_id: "alias-two".to_owned(),
    });
    alias_added.provenance.push(IdentityReference {
        id: "alias-two".to_owned(),
        path: "evidence/identity/provenance-alias-two.json".to_owned(),
        sha256: "8".repeat(64),
        visibility: IdentityReferenceVisibility::ReviewerVisible,
    });
    assert_eq!(
        derive_identity_root(&candidate).expect("original root"),
        derive_identity_root(&alias_added).expect("root with new alias"),
        "an alias addition must not rotate root authority"
    );
}

#[test]
fn rejects_display_wake_snapshot_admission_and_copied_state_as_root_authority() {
    let bases: Vec<IdentityBasis> = serde_json::from_str(
        &fs::read_to_string(fixture("substituted_bases.json")).expect("read basis matrix"),
    )
    .expect("parse basis matrix");
    assert_eq!(bases.len(), 5);
    for basis in bases {
        let mut candidate = valid_candidate();
        candidate.basis = basis;
        assert_eq!(
            build_birthday_identity(&candidate),
            Err(vec![IdentityRejection::UnsupportedBasis { basis }])
        );
    }
}

#[test]
fn rejects_collision_provenance_substitution_privacy_and_path_matrix() {
    let cases: Vec<String> = serde_json::from_str(
        &fs::read_to_string(fixture("negative_cases.json")).expect("read negative matrix"),
    )
    .expect("parse negative matrix");
    assert_eq!(cases.len(), 24);
    for case in cases {
        let mut candidate = valid_candidate();
        let expected = mutate_case(&case, &mut candidate);
        let rejections = build_birthday_identity(&candidate).expect_err("negative must reject");
        assert!(
            rejections.contains(&expected),
            "case {case} expected {expected:?}, got {rejections:?}"
        );
    }
}

#[test]
fn rejects_unknown_top_level_and_nested_fields_during_parse() {
    let mut top = valid_value();
    top.as_object_mut()
        .expect("candidate object")
        .insert("display_authority".to_owned(), serde_json::json!(true));
    assert!(serde_json::from_value::<BirthdayIdentityCandidate>(top).is_err());

    let mut alias = valid_value();
    alias["aliases"][0]
        .as_object_mut()
        .expect("alias object")
        .insert("replace_root".to_owned(), serde_json::json!(true));
    assert!(serde_json::from_value::<BirthdayIdentityCandidate>(alias).is_err());

    let mut reference = valid_value();
    reference["origin"]["reference"]
        .as_object_mut()
        .expect("reference object")
        .insert("raw_payload".to_owned(), serde_json::json!("private"));
    assert!(serde_json::from_value::<BirthdayIdentityCandidate>(reference).is_err());

    let mut policy = valid_value();
    policy["redaction_policy"]
        .as_object_mut()
        .expect("policy object")
        .insert("allow_export".to_owned(), serde_json::json!(true));
    assert!(serde_json::from_value::<BirthdayIdentityCandidate>(policy).is_err());
}

fn mutate_case(case: &str, candidate: &mut BirthdayIdentityCandidate) -> IdentityRejection {
    match case {
        "empty_identity_root" => {
            candidate.identity_root.clear();
            IdentityRejection::EmptyIdentityRoot
        }
        "malformed_identity_root" => {
            candidate.identity_root = "short".to_owned();
            IdentityRejection::MalformedIdentityRoot
        }
        "identity_root_mismatch" => {
            candidate.identity_root = "a".repeat(64);
            candidate.continuity.identity_root = candidate.identity_root.clone();
            IdentityRejection::IdentityRootMismatch
        }
        "invalid_stable_name" => {
            candidate.stable_name = " Aster ".to_owned();
            IdentityRejection::InvalidStableName
        }
        "alias_collision" => {
            candidate.aliases[1].name = candidate.aliases[0].name.to_ascii_lowercase();
            IdentityRejection::AliasCollision {
                name: candidate.aliases[1].name.clone(),
            }
        }
        "alias_stable_name_collision" => {
            candidate.aliases[0].name = candidate.stable_name.to_ascii_lowercase();
            IdentityRejection::AliasCollision {
                name: candidate.aliases[0].name.clone(),
            }
        }
        "alias_missing_provenance" => {
            candidate.aliases[0].provenance_id = "absent-alias-proof".to_owned();
            IdentityRejection::MissingProvenance {
                id: "absent-alias-proof".to_owned(),
            }
        }
        "origin_missing_provenance" => {
            candidate.origin.provenance_id = "absent-origin-proof".to_owned();
            IdentityRejection::MissingProvenance {
                id: "absent-origin-proof".to_owned(),
            }
        }
        "origin_provenance_mismatch" => {
            let origin_provenance = candidate
                .provenance
                .iter_mut()
                .find(|reference| reference.id == candidate.origin.provenance_id)
                .expect("origin provenance");
            origin_provenance.sha256 = "a".repeat(64);
            IdentityRejection::OriginProvenanceMismatch
        }
        "duplicate_provenance" => {
            candidate.provenance.push(candidate.provenance[0].clone());
            IdentityRejection::DuplicateProvenance {
                id: candidate.provenance[0].id.clone(),
            }
        }
        "duplicate_witness" => {
            candidate.witnesses.push(candidate.witnesses[0].clone());
            IdentityRejection::DuplicateWitness {
                id: candidate.witnesses[0].id.clone(),
            }
        }
        "provenance_digest_mismatch" => {
            candidate.provenance[0].sha256 = "not-a-digest".to_owned();
            IdentityRejection::MalformedReferenceDigest {
                id: candidate.provenance[0].id.clone(),
            }
        }
        "substituted_continuity_identity" => {
            candidate.continuity.identity_root = "a".repeat(64);
            IdentityRejection::ContinuityIdentitySubstitution
        }
        "malformed_continuity_head" => {
            candidate.continuity.head_sha256 = "short".to_owned();
            IdentityRejection::MalformedContinuityHead
        }
        "continuity_reference_mismatch" => {
            candidate.continuity.reference.sha256 = "a".repeat(64);
            IdentityRejection::ContinuityReferenceMismatch
        }
        "absolute_reference_path" => {
            candidate.origin.reference.path = "/private/identity.json".to_owned();
            IdentityRejection::UnsafeReferencePath {
                id: candidate.origin.reference.id.clone(),
            }
        }
        "windows_reference_path" => {
            candidate.origin.reference.path = "C:\\private\\identity.json".to_owned();
            IdentityRejection::UnsafeReferencePath {
                id: candidate.origin.reference.id.clone(),
            }
        }
        "parent_reference_path" => {
            candidate.origin.reference.path = "evidence/../private.json".to_owned();
            IdentityRejection::UnsafeReferencePath {
                id: candidate.origin.reference.id.clone(),
            }
        }
        "raw_private_reference" => {
            candidate.origin.reference.visibility = IdentityReferenceVisibility::RawPrivate;
            IdentityRejection::PrivateReference {
                id: candidate.origin.reference.id.clone(),
            }
        }
        "raw_private_policy" => {
            candidate.redaction_policy.raw_private_state = true;
            IdentityRejection::UnsafeRedactionPolicy
        }
        "non_projection_policy" => {
            candidate.redaction_policy.projection_only = false;
            IdentityRejection::UnsafeRedactionPolicy
        }
        "missing_witnesses" => {
            candidate.witnesses.clear();
            IdentityRejection::MissingWitnesses
        }
        "duplicate_redacted_field" => {
            candidate
                .redaction_policy
                .redacted_fields
                .push("private_state".to_owned());
            IdentityRejection::DuplicateRedactedField {
                field: "private_state".to_owned(),
            }
        }
        "unsupported_schema" => {
            candidate.schema = "adl.birthday.identity_candidate.v2".to_owned();
            IdentityRejection::UnsupportedSchema
        }
        other => panic!("unknown negative case: {other}"),
    }
}
