//! PVF: deterministic-core, release-gating contract proof with small resource profile.
//! The positive case emits the sole native semantic artifact; all other cases are
//! fail-closed table-driven negatives over repository fixtures.

use std::{fs, path::Path};

use adl_runtime_kernel::{
    candidate_digest, decide_birthday, BirthdayCandidate, BirthdayRejection, EvidenceKind,
    EvidenceVisibility, LifecycleEvent,
};

fn fixture(name: &str) -> String {
    format!(
        "{}/tests/fixtures/birthday/{name}",
        env!("CARGO_MANIFEST_DIR")
    )
}

fn valid_candidate() -> BirthdayCandidate {
    serde_json::from_str(&fs::read_to_string(fixture("valid.json")).expect("read valid fixture"))
        .expect("parse valid fixture")
}

fn valid_candidate_value() -> serde_json::Value {
    serde_json::from_str(&fs::read_to_string(fixture("valid.json")).expect("read valid fixture"))
        .expect("parse valid fixture value")
}

fn refresh_digest(candidate: &mut BirthdayCandidate) {
    candidate.packet_sha256 = candidate_digest(candidate).expect("canonical candidate digest");
}

#[test]
fn accepts_complete_candidate_and_emits_canonical_semantics() {
    let candidate = valid_candidate();
    assert_eq!(
        candidate.packet_sha256,
        candidate_digest(&candidate).expect("candidate digest")
    );
    let decision = decide_birthday(&candidate);
    assert!(
        decision.accepted,
        "unexpected rejections: {:?}",
        decision.rejections
    );
    assert!(decision.rejections.is_empty());

    if let Ok(output) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = Path::new(&output);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).expect("create semantic output directory");
        }
        let bytes = serde_jcs::to_vec(&decision).expect("canonical decision");
        fs::write(path, bytes).expect("write semantic output");
    }
}

#[test]
fn rejects_every_declared_lifecycle_lookalike() {
    let events: Vec<LifecycleEvent> = serde_json::from_str(
        &fs::read_to_string(fixture("lifecycle_lookalikes.json")).expect("read lifecycle matrix"),
    )
    .expect("parse lifecycle matrix");
    assert_eq!(
        events.len(),
        14,
        "negative lifecycle matrix changed unexpectedly"
    );

    for event in events {
        let mut candidate = valid_candidate();
        candidate.lifecycle_event = event;
        refresh_digest(&mut candidate);
        let decision = decide_birthday(&candidate);
        assert!(!decision.accepted, "accepted lifecycle lookalike {event:?}");
        assert_eq!(
            decision.rejections,
            vec![BirthdayRejection::LifecycleLookalike { event }]
        );
    }
}

#[test]
fn rejects_each_missing_evidence_surface() {
    let kinds: Vec<EvidenceKind> = serde_json::from_str(
        &fs::read_to_string(fixture("missing_evidence.json")).expect("read omission matrix"),
    )
    .expect("parse omission matrix");
    assert_eq!(kinds, EvidenceKind::REQUIRED);

    for kind in kinds {
        let mut candidate = valid_candidate();
        candidate.evidence.retain(|entry| entry.kind != kind);
        refresh_digest(&mut candidate);
        let decision = decide_birthday(&candidate);
        assert_eq!(
            decision.rejections,
            vec![BirthdayRejection::MissingEvidence { kind }]
        );
    }
}

#[test]
fn rejects_integrity_privacy_path_and_claim_boundary_failures() {
    let cases: Vec<String> = serde_json::from_str(
        &fs::read_to_string(fixture("integrity_and_claim_boundaries.json"))
            .expect("read integrity matrix"),
    )
    .expect("parse integrity matrix");
    assert_eq!(cases.len(), 26, "integrity matrix changed unexpectedly");

    for case in cases {
        let mut candidate = valid_candidate();
        let expected = mutate_case(&case, &mut candidate);
        if case != "packet_digest_mismatch" {
            refresh_digest(&mut candidate);
        }
        let decision = decide_birthday(&candidate);
        assert!(!decision.accepted, "accepted integrity case {case}");
        assert!(
            decision.rejections.contains(&expected),
            "case {case} expected {expected:?}, got {:?}",
            decision.rejections
        );
    }
}

#[test]
fn decisions_are_stable_across_evidence_order_and_round_trip() {
    let candidate = valid_candidate();
    let first = decide_birthday(&candidate);
    let mut reordered = candidate.clone();
    reordered.evidence.reverse();
    refresh_digest(&mut reordered);
    let second = decide_birthday(&reordered);
    assert_eq!(candidate.packet_sha256, reordered.packet_sha256);
    assert_eq!(first, second);

    let encoded = serde_jcs::to_vec(&second).expect("canonical decision");
    let decoded = serde_json::from_slice(&encoded).expect("round-trip decision");
    assert_eq!(second, decoded);
}

#[test]
fn rejects_unknown_top_level_and_nested_fields_during_parse() {
    let mut top_level = valid_candidate_value();
    top_level
        .as_object_mut()
        .expect("candidate object")
        .insert("undeclared_authority".to_owned(), serde_json::json!(true));
    assert!(
        serde_json::from_value::<BirthdayCandidate>(top_level).is_err(),
        "unknown top-level field must fail closed"
    );

    let mut evidence = valid_candidate_value();
    evidence["evidence"][0]
        .as_object_mut()
        .expect("evidence object")
        .insert("raw_payload".to_owned(), serde_json::json!("private"));
    assert!(
        serde_json::from_value::<BirthdayCandidate>(evidence).is_err(),
        "unknown evidence field must fail closed"
    );

    let mut cycle = valid_candidate_value();
    cycle["bounded_cycles"][0]
        .as_object_mut()
        .expect("cycle object")
        .insert("unreviewed_state".to_owned(), serde_json::json!(true));
    assert!(
        serde_json::from_value::<BirthdayCandidate>(cycle).is_err(),
        "unknown continuity-cycle field must fail closed"
    );
}

fn mutate_case(case: &str, candidate: &mut BirthdayCandidate) -> BirthdayRejection {
    match case {
        "duplicate_evidence" => {
            let duplicate = candidate.evidence[0].clone();
            candidate.evidence.push(duplicate);
            BirthdayRejection::DuplicateEvidence {
                kind: EvidenceKind::StableName,
            }
        }
        "insufficient_cycles" => {
            candidate.bounded_cycles.truncate(1);
            candidate.continuity_head = candidate.bounded_cycles[0].continuity_head.clone();
            BirthdayRejection::InsufficientBoundedCycles
        }
        "duplicate_cycle" => {
            candidate.bounded_cycles[1].cycle = candidate.bounded_cycles[0].cycle;
            BirthdayRejection::DuplicateCycle { cycle: 1 }
        }
        "non_monotonic_cycles" => {
            candidate.bounded_cycles.swap(0, 1);
            BirthdayRejection::NonMonotonicCycles
        }
        "identity_mismatch" => {
            candidate.bounded_cycles[1].identity_root = "d".repeat(64);
            BirthdayRejection::IdentityRootMismatch
        }
        "continuity_mismatch" => {
            candidate.bounded_cycles[1].continuity_head = "d".repeat(64);
            BirthdayRejection::ContinuityHeadMismatch
        }
        "absolute_path" => {
            candidate.evidence[0].path = "/private/evidence.json".to_owned();
            BirthdayRejection::UnsafeEvidencePath {
                kind: EvidenceKind::StableName,
            }
        }
        "windows_path" => {
            candidate.evidence[0].path = "C:\\private\\evidence.json".to_owned();
            BirthdayRejection::UnsafeEvidencePath {
                kind: EvidenceKind::StableName,
            }
        }
        "parent_path" => {
            candidate.evidence[0].path = "evidence/../private.json".to_owned();
            BirthdayRejection::UnsafeEvidencePath {
                kind: EvidenceKind::StableName,
            }
        }
        "raw_private_evidence" => {
            candidate.evidence[0].visibility = EvidenceVisibility::RawPrivate;
            BirthdayRejection::PrivateEvidence {
                kind: EvidenceKind::StableName,
            }
        }
        "malformed_evidence_digest" => {
            candidate.evidence[0].sha256 = "not-a-digest".to_owned();
            BirthdayRejection::MalformedEvidenceDigest {
                kind: EvidenceKind::StableName,
            }
        }
        "malformed_identity_root" => {
            candidate.identity_root = "short".to_owned();
            for cycle in &mut candidate.bounded_cycles {
                cycle.identity_root = candidate.identity_root.clone();
            }
            BirthdayRejection::MalformedIdentityRoot
        }
        "malformed_continuity_head" => {
            candidate.continuity_head = "short".to_owned();
            candidate.bounded_cycles[1].continuity_head = candidate.continuity_head.clone();
            BirthdayRejection::MalformedContinuityHead
        }
        "malformed_non_terminal_continuity_head" => {
            candidate.bounded_cycles[0].continuity_head = "short".to_owned();
            BirthdayRejection::MalformedCycleContinuityHead { cycle: 1 }
        }
        "unsupported_cognitive_profile" => {
            candidate.cognitive_profile = "personality_or_reputation_label".to_owned();
            BirthdayRejection::UnsupportedCognitiveProfile
        }
        "legal_personhood_claim" => prohibited(candidate, "legal_personhood"),
        "consciousness_claim" => prohibited(candidate, "consciousness_proof"),
        "production_citizenship_claim" => prohibited(candidate, "production_citizenship"),
        "constitutional_governance_claim" => {
            prohibited(candidate, "completed_constitutional_governance")
        }
        "production_migration_claim" => prohibited(candidate, "production_cross_polis_migration"),
        "public_launch_ready_claim" => prohibited(candidate, "public_launch_ready"),
        "transport_security_completion_claim" => {
            prohibited(candidate, "production_transport_security_complete")
        }
        "invalid_candidate_id" => {
            candidate.candidate_id = "../../candidate".to_owned();
            BirthdayRejection::InvalidCandidateId
        }
        "empty_stable_name" => {
            candidate.stable_name = "  ".to_owned();
            BirthdayRejection::EmptyStableName
        }
        "unsupported_schema" => {
            candidate.schema = "adl.birthday.candidate.v2".to_owned();
            BirthdayRejection::UnsupportedSchema
        }
        "packet_digest_mismatch" => {
            candidate.stable_name = "Tampered".to_owned();
            BirthdayRejection::PacketDigestMismatch
        }
        other => panic!("unknown integrity case: {other}"),
    }
}

fn prohibited(candidate: &mut BirthdayCandidate, claim: &str) -> BirthdayRejection {
    candidate.public_claims.push(claim.to_owned());
    BirthdayRejection::ProhibitedPublicClaim {
        claim: claim.to_owned(),
    }
}
