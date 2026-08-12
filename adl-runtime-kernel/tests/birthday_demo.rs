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
        (BirthdayDemoCase::Startup, LifecycleEvent::ProcessStartup),
        (BirthdayDemoCase::Wake, LifecycleEvent::WakeOrResume),
        (
            BirthdayDemoCase::Restore,
            LifecycleEvent::RestoreFromCheckpoint,
        ),
        (BirthdayDemoCase::Snapshot, LifecycleEvent::SnapshotCreation),
        (
            BirthdayDemoCase::Admission,
            LifecycleEvent::TestEnvironmentAdmission,
        ),
        (BirthdayDemoCase::CopiedState, LifecycleEvent::CopiedState),
        (BirthdayDemoCase::Simulation, LifecycleEvent::SimulationRun),
        (
            BirthdayDemoCase::NamedFixture,
            LifecycleEvent::NamedTestFixture,
        ),
    ];
    for (case, event) in cases {
        let packet = run_first_birthday_demo(case).await.unwrap();
        assert_eq!(packet.status, BirthdayDemoStatus::Rejected);
        assert!(!packet.decision.accepted);
        assert_eq!(
            packet.rejections,
            vec![BirthdayDemoRejection::Birthday {
                rejection: BirthdayRejection::LifecycleLookalike { event }
            }]
        );
        assert!(
            packet.capability.is_none()
                && packet.cognitive_profile.is_none()
                && packet.witness_packet.is_none()
        );
    }

    for kind in [
        EvidenceKind::IdentityRoot,
        EvidenceKind::ContinuityHead,
        EvidenceKind::MemoryGrounding,
        EvidenceKind::CapabilityEnvelope,
        EvidenceKind::CognitiveProfile,
        EvidenceKind::WitnessSet,
        EvidenceKind::Receipt,
        EvidenceKind::ReviewerValidation,
    ] {
        let packet = run_first_birthday_demo(BirthdayDemoCase::MissingEvidence(kind))
            .await
            .unwrap();
        assert_eq!(packet.status, BirthdayDemoStatus::Rejected);
        assert_eq!(
            packet.rejections,
            vec![BirthdayDemoRejection::Birthday {
                rejection: BirthdayRejection::MissingEvidence { kind }
            }]
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
