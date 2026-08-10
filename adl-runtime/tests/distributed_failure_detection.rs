#[path = "../src/distributed/failure_detection.rs"]
mod failure_detection;

use std::collections::{BTreeMap, BTreeSet};

use ed25519_dalek::{SigningKey, VerifyingKey};
use failure_detection::{
    FailureClass, FailureDetector, FailureError, FailurePolicy, FailureProbeClaims,
    FailureThresholds, ProbeAuthority, ProbeResult, SignedFailureProbe, FAILURE_PROBE_SCHEMA,
};

#[derive(Default, Clone)]
struct Authority {
    keys: BTreeMap<(String, u64), VerifyingKey>,
    members: BTreeSet<(String, u64)>,
}

impl Authority {
    fn enroll(&mut self, node: &str, generation: u64, key: &SigningKey, epoch: u64) {
        self.keys
            .insert((node.to_owned(), generation), key.verifying_key());
        self.members.insert((node.to_owned(), epoch));
    }
}

impl ProbeAuthority for Authority {
    fn observer_key(&self, node: &str, generation: u64) -> Option<VerifyingKey> {
        self.keys.get(&(node.to_owned(), generation)).copied()
    }

    fn is_member(&self, node: &str, epoch: u64) -> bool {
        self.members.contains(&(node.to_owned(), epoch))
    }
}

fn signer(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn policy(max_nodes: usize, max_observers: usize, flap_limit: usize) -> FailurePolicy {
    policy_for_domain("polis.test", max_nodes, max_observers, flap_limit)
}

fn policy_for_domain(
    trust_domain: &str,
    max_nodes: usize,
    max_observers: usize,
    flap_limit: usize,
) -> FailurePolicy {
    FailurePolicy::new(
        trust_domain,
        "node_local",
        7,
        FailureThresholds {
            suspect_after_secs: 5,
            unavailable_after_secs: 10,
            evidence_window_secs: 5,
            flap_window_secs: 30,
        },
        2,
    )
    .unwrap()
    .with_bounds(2, flap_limit, max_nodes, max_observers, 8)
    .unwrap()
}

fn authority() -> (Authority, BTreeMap<&'static str, SigningKey>) {
    let keys = BTreeMap::from([
        ("node_local", signer(1)),
        ("node_a", signer(2)),
        ("node_b", signer(3)),
        ("node_c", signer(4)),
    ]);
    let mut authority = Authority::default();
    for (node, key) in &keys {
        authority.enroll(node, 1, key, 7);
    }
    (authority, keys)
}

fn probe(
    signer: &SigningKey,
    observer: &str,
    subject: &str,
    sequence: u64,
    observed_at: u64,
    result: ProbeResult,
) -> SignedFailureProbe {
    SignedFailureProbe::sign(
        FailureProbeClaims {
            schema: FAILURE_PROBE_SCHEMA.into(),
            trust_domain: "polis.test".into(),
            membership_epoch: 7,
            observer_node_id: observer.into(),
            observer_identity_generation: 1,
            subject_node_id: subject.into(),
            sequence,
            observed_at_unix_secs: observed_at,
            expires_at_unix_secs: observed_at + 20,
            result,
        },
        signer,
    )
    .unwrap()
}

#[test]
fn silence_becomes_suspect_without_granting_authority() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Reachable,
            ),
            100,
        )
        .unwrap();
    let event = detector.evaluate("node_a", 106).unwrap().unwrap();
    assert_eq!(event.projection.class, FailureClass::Suspect);
    assert!(event.projection.advisory_only);
    assert!(!event.projection.authority_granted);
}

#[test]
fn correlated_unreachable_quorum_marks_unavailable() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Unreachable,
            ),
            100,
        )
        .unwrap();
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_b"],
                "node_b",
                "node_a",
                1,
                110,
                ProbeResult::Unreachable,
            ),
            110,
        )
        .unwrap();
    let event = detector
        .observe(
            &authority,
            &probe(
                &keys["node_c"],
                "node_c",
                "node_a",
                1,
                110,
                ProbeResult::Unreachable,
            ),
            110,
        )
        .unwrap()
        .unwrap();
    assert_eq!(event.projection.class, FailureClass::Unavailable);
    assert_eq!(event.projection.supporting_observers, 2);
}

#[test]
fn remote_reachable_quorum_distinguishes_local_partition() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Unreachable,
            ),
            100,
        )
        .unwrap();
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_b"],
                "node_b",
                "node_a",
                1,
                110,
                ProbeResult::Reachable,
            ),
            110,
        )
        .unwrap();
    let event = detector
        .observe(
            &authority,
            &probe(
                &keys["node_c"],
                "node_c",
                "node_a",
                1,
                110,
                ProbeResult::Reachable,
            ),
            110,
        )
        .unwrap()
        .unwrap();
    assert_eq!(event.projection.class, FailureClass::Partitioned);
}

#[test]
fn recovery_requires_two_authenticated_direct_confirmations() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Unreachable,
            ),
            100,
        )
        .unwrap();
    detector.evaluate("node_a", 106).unwrap();
    let first = detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                2,
                107,
                ProbeResult::Reachable,
            ),
            107,
        )
        .unwrap();
    assert_eq!(first, None);
    let recovered = detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                3,
                114,
                ProbeResult::Reachable,
            ),
            114,
        )
        .unwrap()
        .unwrap();
    assert_eq!(recovered.projection.class, FailureClass::Recovered);
}

#[test]
fn repeated_state_changes_are_bounded_as_flapping() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 3));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Reachable,
            ),
            100,
        )
        .unwrap();
    detector.evaluate("node_a", 106).unwrap();
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                2,
                107,
                ProbeResult::Reachable,
            ),
            107,
        )
        .unwrap();
    let event = detector.evaluate("node_a", 113).unwrap().unwrap();
    assert_eq!(event.projection.class, FailureClass::Flapping);
    assert!(detector.events().count() <= 8);
}

#[test]
fn replay_wrong_domain_epoch_and_stale_probes_fail_closed() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    let valid = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        1,
        100,
        ProbeResult::Reachable,
    );
    detector.observe(&authority, &valid, 100).unwrap();
    assert_eq!(
        detector.observe(&authority, &valid, 100),
        Err(FailureError::Replay)
    );

    let mut wrong_domain = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        2,
        101,
        ProbeResult::Reachable,
    );
    wrong_domain.claims.trust_domain = "other.test".into();
    assert_eq!(
        detector.observe(&authority, &wrong_domain, 101),
        Err(FailureError::WrongTrustDomain)
    );

    let mut wrong_epoch = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        2,
        101,
        ProbeResult::Reachable,
    );
    wrong_epoch.claims.membership_epoch = 8;
    assert_eq!(
        detector.observe(&authority, &wrong_epoch, 101),
        Err(FailureError::WrongMembershipEpoch)
    );

    let stale = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        2,
        101,
        ProbeResult::Reachable,
    );
    assert_eq!(
        detector.observe(&authority, &stale, 200),
        Err(FailureError::StaleProbe)
    );
}

#[test]
fn forged_unenrolled_and_nonmember_reports_fail_closed() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    let mut forged = probe(
        &keys["node_b"],
        "node_b",
        "node_a",
        1,
        100,
        ProbeResult::Unreachable,
    );
    forged.signature[0] ^= 1;
    assert_eq!(
        detector.observe(&authority, &forged, 100),
        Err(FailureError::InvalidSignature)
    );

    let mut unenrolled_authority = authority.clone();
    unenrolled_authority
        .members
        .insert(("node_outsider".to_owned(), 7));
    let outsider = probe(
        &signer(9),
        "node_outsider",
        "node_a",
        1,
        100,
        ProbeResult::Unreachable,
    );
    assert_eq!(
        detector.observe(&unenrolled_authority, &outsider, 100),
        Err(FailureError::ObserverNotEnrolled)
    );

    let nonmember = probe(
        &keys["node_b"],
        "node_b",
        "node_missing",
        1,
        100,
        ProbeResult::Unreachable,
    );
    assert_eq!(
        detector.observe(&authority, &nonmember, 100),
        Err(FailureError::SubjectNotMember)
    );

    let mut nonmember_observer_authority = authority.clone();
    nonmember_observer_authority
        .members
        .remove(&("node_b".to_owned(), 7));
    let nonmember_observer = probe(
        &keys["node_b"],
        "node_b",
        "node_a",
        2,
        101,
        ProbeResult::Unreachable,
    );
    assert_eq!(
        detector.observe(&nonmember_observer_authority, &nonmember_observer, 101),
        Err(FailureError::ObserverNotMember)
    );
}

#[test]
fn node_and_observer_bounds_reject_without_partial_mutation() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(1, 2, 8));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Reachable,
            ),
            100,
        )
        .unwrap();
    assert_eq!(
        detector.observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_b",
                1,
                101,
                ProbeResult::Reachable
            ),
            101
        ),
        Err(FailureError::ResourceExhausted)
    );

    let mut bounded = FailureDetector::new(policy(8, 2, 8));
    bounded
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Unreachable,
            ),
            100,
        )
        .unwrap();
    bounded
        .observe(
            &authority,
            &probe(
                &keys["node_b"],
                "node_b",
                "node_a",
                1,
                101,
                ProbeResult::Unreachable,
            ),
            101,
        )
        .unwrap();
    assert_eq!(
        bounded.observe(
            &authority,
            &probe(
                &keys["node_c"],
                "node_c",
                "node_a",
                1,
                102,
                ProbeResult::Unreachable
            ),
            102
        ),
        Err(FailureError::ResourceExhausted)
    );
}

#[test]
fn policy_and_probe_shape_are_hard_bounded() {
    assert_eq!(
        FailurePolicy::new(
            "polis.test",
            "node_local",
            7,
            FailureThresholds {
                suspect_after_secs: 10,
                unavailable_after_secs: 5,
                evidence_window_secs: 5,
                flap_window_secs: 30,
            },
            2,
        )
        .unwrap_err(),
        FailureError::InvalidPolicy
    );
    let invalid = FailureProbeClaims {
        schema: FAILURE_PROBE_SCHEMA.into(),
        trust_domain: "polis.test".into(),
        membership_epoch: 7,
        observer_node_id: "node_a".into(),
        observer_identity_generation: 1,
        subject_node_id: "node_a".into(),
        sequence: 1,
        observed_at_unix_secs: 100,
        expires_at_unix_secs: 101,
        result: ProbeResult::Reachable,
    };
    assert_eq!(
        SignedFailureProbe::sign(invalid, &signer(1)),
        Err(FailureError::InvalidProbe)
    );
}

#[test]
fn deterministic_event_projection_is_stable_and_redacted() {
    let (authority, keys) = authority();
    let mut left = FailureDetector::new(policy(8, 4, 8));
    let mut right = FailureDetector::new(policy(8, 4, 8));
    let signed = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        1,
        100,
        ProbeResult::Reachable,
    );
    left.observe(&authority, &signed, 100).unwrap();
    right.observe(&authority, &signed, 100).unwrap();
    let left_event = left.evaluate("node_a", 106).unwrap().unwrap();
    let right_event = right.evaluate("node_a", 106).unwrap().unwrap();
    assert_eq!(left_event, right_event);
    let encoded = serde_json::to_string(&left_event).unwrap();
    assert!(!encoded.contains("signature"));
    assert!(!encoded.contains("public_key"));
}

#[test]
fn replay_is_scoped_to_enrolled_identity_generation() {
    let (mut authority, keys) = authority();
    let rotated = signer(8);
    authority.enroll("node_local", 2, &rotated, 7);
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    let generation_one = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        1,
        100,
        ProbeResult::Reachable,
    );
    detector.observe(&authority, &generation_one, 100).unwrap();
    assert_eq!(
        detector.observe(&authority, &generation_one, 100),
        Err(FailureError::Replay)
    );

    let generation_two = SignedFailureProbe::sign(
        FailureProbeClaims {
            observer_identity_generation: 2,
            ..generation_one.claims.clone()
        },
        &rotated,
    )
    .unwrap();
    detector.observe(&authority, &generation_two, 100).unwrap();
    assert_eq!(
        detector.observe(&authority, &generation_two, 100),
        Err(FailureError::Replay)
    );
    assert_eq!(
        detector.observe(&authority, &generation_one, 100),
        Err(FailureError::Replay)
    );
    assert_eq!(detector.replay_record_count(), 1);
}

#[test]
fn identity_rotation_replaces_bounded_replay_state() {
    let (mut authority, _) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 8));
    for generation in 1..=300 {
        let key = signer(u8::try_from((generation % 250) + 1).unwrap());
        authority.enroll("node_b", generation, &key, 7);
        let signed = SignedFailureProbe::sign(
            FailureProbeClaims {
                schema: FAILURE_PROBE_SCHEMA.into(),
                trust_domain: "polis.test".into(),
                membership_epoch: 7,
                observer_node_id: "node_b".into(),
                observer_identity_generation: generation,
                subject_node_id: "node_a".into(),
                sequence: 1,
                observed_at_unix_secs: 100,
                expires_at_unix_secs: 120,
                result: ProbeResult::Reachable,
            },
            &key,
        )
        .unwrap();
        detector.observe(&authority, &signed, 100).unwrap();
    }
    assert_eq!(detector.replay_record_count(), 1);

    let old_key = signer(2);
    let stale_generation = SignedFailureProbe::sign(
        FailureProbeClaims {
            schema: FAILURE_PROBE_SCHEMA.into(),
            trust_domain: "polis.test".into(),
            membership_epoch: 7,
            observer_node_id: "node_b".into(),
            observer_identity_generation: 1,
            subject_node_id: "node_a".into(),
            sequence: 2,
            observed_at_unix_secs: 101,
            expires_at_unix_secs: 121,
            result: ProbeResult::Reachable,
        },
        &old_key,
    )
    .unwrap();
    assert_eq!(
        detector.observe(&authority, &stale_generation, 101),
        Err(FailureError::Replay)
    );
}

#[test]
fn event_identity_is_separated_across_trust_domains() {
    let (authority, keys) = authority();
    let mut polis = FailureDetector::new(policy_for_domain("polis.test", 8, 4, 8));
    let mut other = FailureDetector::new(policy_for_domain("other.test", 8, 4, 8));
    let polis_probe = probe(
        &keys["node_local"],
        "node_local",
        "node_a",
        1,
        100,
        ProbeResult::Reachable,
    );
    let other_probe = SignedFailureProbe::sign(
        FailureProbeClaims {
            trust_domain: "other.test".into(),
            ..polis_probe.claims.clone()
        },
        &keys["node_local"],
    )
    .unwrap();
    polis.observe(&authority, &polis_probe, 100).unwrap();
    other.observe(&authority, &other_probe, 100).unwrap();
    let polis_event = polis.evaluate("node_a", 106).unwrap().unwrap();
    let other_event = other.evaluate("node_a", 106).unwrap().unwrap();
    assert_eq!(polis_event.projection.trust_domain, "polis.test");
    assert_eq!(other_event.projection.trust_domain, "other.test");
    assert_ne!(polis_event.event_id, other_event.event_id);
}

#[test]
fn active_flapping_is_preserved_in_read_only_projection() {
    let (authority, keys) = authority();
    let mut detector = FailureDetector::new(policy(8, 4, 3));
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                1,
                100,
                ProbeResult::Reachable,
            ),
            100,
        )
        .unwrap();
    detector.evaluate("node_a", 106).unwrap();
    detector
        .observe(
            &authority,
            &probe(
                &keys["node_local"],
                "node_local",
                "node_a",
                2,
                107,
                ProbeResult::Reachable,
            ),
            107,
        )
        .unwrap();
    assert_eq!(
        detector
            .evaluate("node_a", 113)
            .unwrap()
            .unwrap()
            .projection
            .class,
        FailureClass::Flapping
    );
    assert_eq!(
        detector.projection("node_a", 114).unwrap().class,
        FailureClass::Flapping
    );
}
