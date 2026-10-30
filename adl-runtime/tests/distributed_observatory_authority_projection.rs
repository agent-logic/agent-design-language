#![cfg(feature = "internal-test-fixtures")]

use adl_runtime::distributed::{
    authority_protocol::{
        test_observatory_authority_mutation_rejected,
        test_observatory_durable_restore_mutation_rejected, test_observatory_durable_round_trip,
        test_observatory_legacy_durable_state_rejected, test_observatory_published_authority,
        test_observatory_published_authority_for_operation, ObservatoryAuthorityMutation,
    },
    serving_authority::{
        verify_observatory_authority_projection, ObservatoryBindingFixture,
        ObservatoryBindingMutation, ObservatoryDigestField, ObservatoryIdentifierField,
        ObservatoryTransitionAction, VerifiedServingAuthorityCut,
    },
};

fn distinct_pair(
    suffix: &str,
    operation: &str,
    index: u64,
    fence: u64,
    action: ObservatoryTransitionAction,
    predecessor: Option<&str>,
) -> (
    adl_runtime::distributed::authority_protocol::PublishedAuthorityResult,
    VerifiedServingAuthorityCut,
) {
    let mut fixture = ObservatoryBindingFixture::new(suffix);
    fixture.set_operation(operation, index);
    fixture.set_integers(index, 1, fence);
    fixture.set_transition(action, predecessor);
    let authority = test_observatory_published_authority_for_operation(
        fixture.artifact_bytes(),
        operation,
        index,
    );
    let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
    (authority, cut)
}

#[test]
fn authentic_distinct_transition_builder_cross_binds_real_verifier() {
    for (operation, index, fence, action, predecessor) in [
        ("acquire", 2, 1, ObservatoryTransitionAction::Acquire, None),
        (
            "renew",
            3,
            2,
            ObservatoryTransitionAction::Renew,
            Some("acquire"),
        ),
        (
            "transfer",
            4,
            3,
            ObservatoryTransitionAction::Transfer,
            Some("renew"),
        ),
        (
            "revoke",
            5,
            4,
            ObservatoryTransitionAction::Revoke,
            Some("transfer"),
        ),
    ] {
        let (authority, cut) =
            distinct_pair("distinct", operation, index, fence, action, predecessor);
        let projection = verify_observatory_authority_projection(&authority, &cut).unwrap();
        assert_eq!(projection.transition_action(), action);
        assert_eq!(projection.committed_log_index(), index);
        assert_eq!(projection.fencing_generation(), fence);
        assert_eq!(
            projection.predecessor_operation_ref().is_some(),
            predecessor.is_some()
        );
    }
    let (authority_a, cut_a) = distinct_pair(
        "a",
        "operation-a",
        2,
        1,
        ObservatoryTransitionAction::Acquire,
        None,
    );
    let (authority_b, cut_b) = distinct_pair(
        "b",
        "operation-b",
        3,
        2,
        ObservatoryTransitionAction::Renew,
        Some("operation-a"),
    );
    assert!(verify_observatory_authority_projection(&authority_a, &cut_b).is_err());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_a).is_err());
}

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
fn sealed_transition_intent_and_full_canonical_time_are_authenticated() {
    for (action, predecessor) in [
        (ObservatoryTransitionAction::Acquire, None),
        (ObservatoryTransitionAction::Renew, Some("prior-renew")),
        (
            ObservatoryTransitionAction::Transfer,
            Some("prior-transfer"),
        ),
        (ObservatoryTransitionAction::Revoke, Some("prior-revoke")),
    ] {
        let mut fixture = ObservatoryBindingFixture::new("transition");
        fixture.set_transition(action, predecessor);
        let authority = test_observatory_published_authority(fixture.artifact_bytes());
        let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
        let projection = verify_observatory_authority_projection(&authority, &cut).unwrap();
        assert_eq!(projection.transition_action(), action);
        assert_eq!(
            projection.predecessor_operation_ref().is_some(),
            predecessor.is_some()
        );
        assert!(projection.inclusive_deadline_nanos() < 1_000_000_000);
        assert!(projection.finalization_nanos() < 1_000_000_000);
        assert!(projection.inclusive_deadline_uncertainty_millis() > 0);
        assert!(projection.finalization_uncertainty_millis() > 0);
        let seconds = projection.inclusive_deadline_unix_seconds();
        let nanos = projection.inclusive_deadline_nanos();
        assert!(!projection
            .is_expired_at(seconds, nanos.saturating_sub(1))
            .unwrap());
        assert!(!projection.is_expired_at(seconds, nanos).unwrap());
        if nanos < 999_999_999 {
            assert!(projection.is_expired_at(seconds, nanos + 1).unwrap());
        } else {
            assert!(projection.is_expired_at(seconds + 1, 0).unwrap());
        }

        let restored_authority = test_observatory_durable_round_trip(fixture.artifact_bytes());
        let restored = verify_observatory_authority_projection(&restored_authority, &cut).unwrap();
        assert_eq!(restored.transition_action(), action);
        assert_eq!(
            restored.predecessor_operation_ref(),
            projection.predecessor_operation_ref()
        );
        assert_eq!(
            restored.inclusive_deadline_unix_seconds(),
            projection.inclusive_deadline_unix_seconds()
        );
        assert_eq!(
            restored.inclusive_deadline_nanos(),
            projection.inclusive_deadline_nanos()
        );
        assert_eq!(
            restored.finalization_unix_seconds(),
            projection.finalization_unix_seconds()
        );
        assert_eq!(
            restored.finalization_nanos(),
            projection.finalization_nanos()
        );
        assert_eq!(
            restored.finalization_uncertainty_millis(),
            projection.finalization_uncertainty_millis()
        );
        assert_eq!(
            restored.inclusive_deadline_uncertainty_millis(),
            projection.inclusive_deadline_uncertainty_millis()
        );
    }

    for (action, predecessor) in [
        (ObservatoryTransitionAction::Acquire, Some("unexpected")),
        (ObservatoryTransitionAction::Renew, None),
        (ObservatoryTransitionAction::Transfer, Some("operation")),
        (ObservatoryTransitionAction::Revoke, None),
    ] {
        let mut fixture = ObservatoryBindingFixture::new("invalid-transition");
        fixture.set_transition(action, predecessor);
        let authority = test_observatory_published_authority(fixture.artifact_bytes());
        let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
        assert!(verify_observatory_authority_projection(&authority, &cut).is_err());
    }
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
    for field in [
        ObservatoryIdentifierField::TrustDomain,
        ObservatoryIdentifierField::PolisId,
        ObservatoryIdentifierField::LineageId,
        ObservatoryIdentifierField::OperationId,
        ObservatoryIdentifierField::OwnerCommitId,
        ObservatoryIdentifierField::LeaseId,
    ] {
        for value in ["bad space", "bad☃", "bad/char", "bad\nline", "bad@char"] {
            let mut fixture = ObservatoryBindingFixture::new("identifier");
            fixture.set_invalid_identifier(field, value);
            let authority = test_observatory_published_authority(fixture.artifact_bytes());
            let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
            assert!(
                verify_observatory_authority_projection(&authority, &cut).is_err(),
                "{field:?} {value:?}"
            );
        }
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
    let text = String::from_utf8(canonical.clone()).unwrap();
    for noncanonical in [
        text.replace("\"committed_log_index\":2", "\"committed_log_index\":2.0"),
        text.replace("\"committed_log_index\":2", "\"committed_log_index\":2e0"),
        format!(
            "{{{}}}",
            text[1..text.len() - 1]
                .split(',')
                .rev()
                .collect::<Vec<_>>()
                .join(",")
        ),
    ] {
        assert!(verify_observatory_authority_projection(
            &test_observatory_published_authority(noncanonical.into_bytes()),
            &cut
        )
        .is_err());
    }

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

    for field in [
        ObservatoryDigestField::State,
        ObservatoryDigestField::Result,
        ObservatoryDigestField::Receipt,
    ] {
        for digest in ["ABCDEF", "sha256:00", "AA", "00", "Zm9v"] {
            let mut fixture = ObservatoryBindingFixture::new("digest");
            fixture.set_digest_encoding(field, digest);
            let authority = test_observatory_published_authority(fixture.artifact_bytes());
            let cut = VerifiedServingAuthorityCut::fixture_from_observatory(&fixture);
            assert!(
                verify_observatory_authority_projection(&authority, &cut).is_err(),
                "{field:?} {digest}"
            );
        }
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
        ObservatoryAuthorityMutation::ExtraSigner,
        ObservatoryAuthorityMutation::ResultDigest,
        ObservatoryAuthorityMutation::RetryDigest,
    ] {
        if !matches!(
            mutation,
            ObservatoryAuthorityMutation::ResultDigest | ObservatoryAuthorityMutation::RetryDigest
        ) {
            assert!(
                test_observatory_authority_mutation_rejected(fixture.artifact_bytes(), mutation),
                "{mutation:?}"
            );
        }
        if !matches!(mutation, ObservatoryAuthorityMutation::MissingQuorumBasis) {
            assert!(
                test_observatory_durable_restore_mutation_rejected(
                    fixture.artifact_bytes(),
                    mutation
                ),
                "durable {mutation:?}"
            );
        }
    }
    assert!(test_observatory_legacy_durable_state_rejected(
        fixture.artifact_bytes()
    ));
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

#[test]
fn sealed_projection_accessors_match_verified_redacted_values() {
    let (_, authority_a, cut_a) = pair("accessor-a");
    let (_, authority_b, cut_b) = pair("accessor-b");
    assert!(verify_observatory_authority_projection(&authority_a, &cut_b).is_err());
    assert!(verify_observatory_authority_projection(&authority_b, &cut_a).is_err());

    let projection = verify_observatory_authority_projection(&authority_a, &cut_a).unwrap();
    let value = serde_json::to_value(&projection).unwrap();
    assert_eq!(projection.trust_domain_ref(), value["trust_domain_ref"]);
    assert_eq!(projection.polis_ref(), value["polis_ref"]);
    assert_eq!(projection.lineage_ref(), value["lineage_ref"]);
    assert_eq!(projection.operation_ref(), value["operation_ref"]);
    assert_eq!(projection.committed_log_index(), 2);
    assert_eq!(
        projection.committed_log_index(),
        value["committed_log_index"]
    );
    assert_eq!(projection.foundation_generation(), 1);
    assert_eq!(
        projection.foundation_generation(),
        value["foundation_generation"]
    );
    assert_eq!(projection.fencing_generation(), 1);
    assert_eq!(projection.fencing_generation(), value["fencing_generation"]);
    assert_eq!(
        projection.authority_result_sha256(),
        value["authority_result_sha256"]
    );
    assert_eq!(projection.signer_set_sha256(), value["signer_set_sha256"]);
    assert_eq!(projection.signer_count(), value["signer_count"]);
    assert_eq!(
        projection.inclusive_deadline_unix_seconds(),
        value["inclusive_deadline_unix_seconds"]
    );
    assert_eq!(
        projection.finalization_unix_seconds(),
        value["finalization_unix_seconds"]
    );
    assert!(projection.inclusive_deadline_unix_seconds() >= projection.finalization_unix_seconds());

    let debug = format!("{projection:?}");
    let json = serde_json::to_string(&projection).unwrap();
    for forbidden in [
        "owner-accessor-a",
        "lease-accessor-a",
        "lineage-accessor-a",
        "guardian-a",
    ] {
        assert!(!debug.contains(forbidden));
        assert!(!json.contains(forbidden));
    }
}
