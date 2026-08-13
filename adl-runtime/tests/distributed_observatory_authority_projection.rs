#![cfg(feature = "internal-test-fixtures")]

use adl_runtime::distributed::{
    authority_protocol::{
        test_observatory_authority_mutation_rejected, test_observatory_published_authority,
        ObservatoryAuthorityMutation,
    },
    serving_authority::{
        verify_observatory_authority_projection, ObservatoryBindingFixture,
        ObservatoryBindingMutation, VerifiedServingAuthorityCut,
    },
};

fn pair(
    suffix: &str,
) -> (
    ObservatoryBindingFixture,
    adl_runtime::distributed::authority_protocol::PublishedAuthorityResult,
    VerifiedServingAuthorityCut,
) {
    let fixture = ObservatoryBindingFixture::new(suffix);
    let authority = test_observatory_published_authority(fixture.artifact_bytes());
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
    (fixture, authority, cut)
}

#[test]
fn exact_pair_matrix_only_matching_pairs_pass() {
    let (_, authority_a, cut_a) = pair("a");
    let (_, authority_b, cut_b) = pair("b");
    assert!(verify_observatory_authority_projection(&authority_a, &cut_a).is_ok());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_b).is_ok());
    assert!(verify_observatory_authority_projection(&authority_a, &cut_b).is_err());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_a).is_err());
}

#[test]
fn every_binding_field_mismatch_fails_closed() {
    let mutations = [
        ObservatoryBindingMutation::Domain,
        ObservatoryBindingMutation::TrustDomain,
        ObservatoryBindingMutation::PolisId,
        ObservatoryBindingMutation::LineageId,
        ObservatoryBindingMutation::OperationId,
        ObservatoryBindingMutation::CommittedLogIndex,
        ObservatoryBindingMutation::FoundationGeneration,
        ObservatoryBindingMutation::OwnerCommitId,
        ObservatoryBindingMutation::FencingGeneration,
        ObservatoryBindingMutation::LeaseId,
        ObservatoryBindingMutation::StateDigest,
        ObservatoryBindingMutation::ResultDigest,
        ObservatoryBindingMutation::ReceiptDigest,
    ];
    for mutation in mutations {
        let (mut artifact_fixture, _, cut) = pair("mismatch");
        artifact_fixture.mutate(mutation);
        let authority = test_observatory_published_authority(artifact_fixture.artifact_bytes());
        assert!(
            verify_observatory_authority_projection(&authority, &cut).is_err(),
            "{mutation:?}"
        );
    }
}

#[test]
fn identifier_grammar_rejects_unicode_whitespace_control_and_punctuation() {
    for suffix in ["bad space", "bad☃", "bad/char", "bad\nline", "bad@char"] {
        let (fixture, authority, cut) = pair(suffix);
        assert!(
            verify_observatory_authority_projection(&authority, &cut).is_err(),
            "{}",
            fixture.artifact_bytes().len()
        );
    }
}

#[test]
fn noncanonical_unknown_missing_and_digest_encodings_fail_closed() {
    let (fixture, _, cut) = pair("encoding");
    let canonical = fixture.artifact_bytes();
    let mut whitespace = canonical.clone();
    whitespace.insert(1, b' ');
    assert!(verify_observatory_authority_projection(
        &test_observatory_published_authority(whitespace),
        &cut
    )
    .is_err());

    let mut value: serde_json::Value = serde_json::from_slice(&canonical).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("unknown".into(), true.into());
    assert!(verify_observatory_authority_projection(
        &test_observatory_published_authority(serde_jcs::to_vec(&value).unwrap()),
        &cut
    )
    .is_err());
    value.as_object_mut().unwrap().remove("unknown");
    value.as_object_mut().unwrap().remove("lease_id");
    assert!(verify_observatory_authority_projection(
        &test_observatory_published_authority(serde_jcs::to_vec(&value).unwrap()),
        &cut
    )
    .is_err());

    for digest in ["ABCDEF", "sha256:00", "AA", "00"] {
        let mut value: serde_json::Value = serde_json::from_slice(&canonical).unwrap();
        value["foundation_state_sha256"] = digest.into();
        assert!(verify_observatory_authority_projection(
            &test_observatory_published_authority(serde_jcs::to_vec(&value).unwrap()),
            &cut
        )
        .is_err());
    }
}

#[test]
fn i_json_integer_bounds_accept_max_and_reject_zero_or_above_max() {
    const MAX: u64 = 9_007_199_254_740_991;
    let mut maximum = ObservatoryBindingFixture::new("max");
    maximum.set_integers(2, MAX, MAX);
    let authority = test_observatory_published_authority(maximum.artifact_bytes());
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&maximum);
    assert!(verify_observatory_authority_projection(&authority, &cut).is_ok());
    for values in [
        (0, 1, 1),
        (1, 0, 1),
        (1, 1, 0),
        (MAX + 1, 1, 1),
        (1, MAX + 1, 1),
        (1, 1, MAX + 1),
    ] {
        let mut fixture = ObservatoryBindingFixture::new("bounds");
        fixture.set_integers(values.0, values.1, values.2);
        let authority = test_observatory_published_authority(fixture.artifact_bytes());
        let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
        assert!(
            verify_observatory_authority_projection(&authority, &cut).is_err(),
            "{values:?}"
        );
    }
}

#[test]
fn sealed_quorum_restore_mutations_fail_closed() {
    let fixture = ObservatoryBindingFixture::new("quorum");
    for mutation in [
        ObservatoryAuthorityMutation::MissingQuorumBasis,
        ObservatoryAuthorityMutation::NonMajorityThreshold,
        ObservatoryAuthorityMutation::OversizedGuardianId,
        ObservatoryAuthorityMutation::EmptySigners,
        ObservatoryAuthorityMutation::SignerGeneration,
        ObservatoryAuthorityMutation::DeadlineBeforeFinalization,
    ] {
        assert!(
            test_observatory_authority_mutation_rejected(fixture.artifact_bytes(), mutation),
            "{mutation:?}"
        );
    }
}

#[test]
fn projection_is_redacted_and_deadline_is_committed() {
    let (_, authority, cut) = pair("redacted");
    let projection = verify_observatory_authority_projection(&authority, &cut).unwrap();
    let json = serde_json::to_string(&projection).unwrap();
    assert!(json.contains("observatory-authority-projection"));
    for forbidden in [
        "guardian",
        "owner-redacted",
        "lease-redacted",
        "lineage-redacted",
    ] {
        assert!(!json.contains(forbidden));
    }
    assert!(projection.inclusive_deadline_unix_seconds() > 0);
}
