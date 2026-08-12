//! PVF: deterministic-core, release-gating WP-18 Runtime orchestration proof.

use adl_runtime_kernel::*;
use sha2::Digest;

#[tokio::test]
async fn runtime_owned_positive_is_complete_replay_stable_and_redacted() {
    let first = run_first_birthday_demo(BirthdayDemoCase::Positive)
        .await
        .unwrap();
    let second = run_first_birthday_demo(BirthdayDemoCase::Positive)
        .await
        .unwrap();
    assert_eq!(first, second);
    assert_eq!(first.status, BirthdayDemoStatus::Complete);
    assert!(first.decision.accepted);
    assert!(first.capability.is_some());
    assert!(first.cognitive_profile.is_some());
    assert!(first.witness_packet.is_some());
    let mut digest_input = first.clone();
    digest_input.packet_sha256.clear();
    assert_eq!(
        first.packet_sha256,
        hex::encode(sha2::Sha256::digest(
            serde_jcs::to_vec(&digest_input).unwrap()
        ))
    );
    let encoded = serde_json::to_string(&first).unwrap().to_ascii_lowercase();
    for forbidden in [
        "runtime-private-state-not-exported",
        "/users/",
        "/home/",
        "/private/",
        "github_pat_",
        "bearer ",
    ] {
        assert!(!encoded.contains(forbidden), "leaked {forbidden}");
    }
}

#[tokio::test]
async fn every_declared_negative_has_a_typed_reason() {
    let cases = [
        BirthdayDemoCase::Startup,
        BirthdayDemoCase::Wake,
        BirthdayDemoCase::Restore,
        BirthdayDemoCase::Snapshot,
        BirthdayDemoCase::Admission,
        BirthdayDemoCase::CopiedState,
        BirthdayDemoCase::Simulation,
        BirthdayDemoCase::NamedFixture,
        BirthdayDemoCase::MissingEvidence(EvidenceKind::IdentityRoot),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::ContinuityHead),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::MemoryGrounding),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::CapabilityEnvelope),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::CognitiveProfile),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::WitnessSet),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::Receipt),
        BirthdayDemoCase::MissingEvidence(EvidenceKind::ReviewerValidation),
    ];
    for case in cases {
        let packet = run_first_birthday_demo(case).await.unwrap();
        assert_eq!(packet.status, BirthdayDemoStatus::Rejected);
        assert!(!packet.decision.accepted);
        assert!(!packet.rejections.is_empty());
        assert!(
            packet.capability.is_none()
                && packet.cognitive_profile.is_none()
                && packet.witness_packet.is_none()
        );
    }
}

#[tokio::test]
async fn interruption_is_retained_as_incomplete_not_birth() {
    let packet = run_first_birthday_demo(BirthdayDemoCase::Interrupted)
        .await
        .unwrap();
    assert_eq!(packet.status, BirthdayDemoStatus::Incomplete);
    assert!(
        packet.capability.is_none()
            && packet.cognitive_profile.is_none()
            && packet.witness_packet.is_none()
    );
    assert_eq!(
        packet.rejections,
        vec![BirthdayDemoRejection::InterruptedBeforeReceipt]
    );
}

#[test]
fn public_api_exposes_no_authority_inputs() {
    let _entrypoint: fn(BirthdayDemoCase) -> _ = run_first_birthday_demo;
}
