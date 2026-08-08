// PVF: lane=exact-child-tests; proof=certificate purpose/lifecycle positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[path = "../src/distributed/certificates.rs"]
mod certificates;

use certificates::{
    ActivationOutcome, AuthorityCertificate, CertificateBody, CertificateError, CertificatePolicy,
    CertificatePurpose, CertificateValidity, DistributedCertificateStore, RevocationReason,
};
use ed25519_dalek::SigningKey;
use tempfile::TempDir;

const DOMAIN: &str = "polis.example";
const NOW: u64 = 1_000_000;

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn policy(root: &SigningKey) -> CertificatePolicy {
    CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3_600, 30, 10, 64, 64)
        .unwrap()
}

fn open_store(temp: &TempDir, root: &SigningKey) -> DistributedCertificateStore {
    DistributedCertificateStore::open(
        temp.path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        policy(root),
    )
    .unwrap()
}

fn certificate(
    root: &SigningKey,
    holder: &str,
    purpose: CertificatePurpose,
    generation: u64,
    subject: &SigningKey,
) -> AuthorityCertificate {
    AuthorityCertificate::issue(
        CertificateBody::new(
            DOMAIN,
            holder,
            purpose,
            generation,
            CertificateValidity {
                issued_at_unix_secs: NOW,
                expires_at_unix_secs: NOW + 300,
            },
            subject.verifying_key(),
            &root.verifying_key(),
        ),
        root,
    )
    .unwrap()
}

#[test]
fn all_certificate_purposes_are_distinct_and_keys_are_not_interchangeable() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(1);
    let store = open_store(&temp, &root);
    let purposes = [
        CertificatePurpose::NodeIdentity,
        CertificatePurpose::GuardianControl,
        CertificatePurpose::Transport,
        CertificatePurpose::AdvertisementSigning,
        CertificatePurpose::SnapshotSigning,
    ];
    for (index, purpose) in purposes.into_iter().enumerate() {
        let cert = certificate(&root, "guardian-a", purpose, 1, &key(index as u8 + 10));
        assert!(matches!(
            store.activate(&cert, NOW).unwrap(),
            ActivationOutcome::Activated(_)
        ));
        assert_eq!(
            store
                .authorize("guardian-a", purpose, 1, NOW + 1)
                .unwrap()
                .purpose,
            purpose
        );
    }

    let reused_transport_key = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::AdvertisementSigning,
        2,
        &key(12),
    );
    assert_eq!(
        store.activate(&reused_transport_key, NOW + 1).unwrap_err(),
        CertificateError::KeyPurposeConflict
    );
    assert_eq!(
        store
            .authorize(
                "guardian-a",
                CertificatePurpose::SnapshotSigning,
                2,
                NOW + 1,
            )
            .unwrap_err(),
        CertificateError::CertificateNotFound
    );
}

#[test]
fn issuer_chain_is_root_bound_and_tampering_fails_closed() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(2);
    let unknown_root = key(3);
    let store = open_store(&temp, &root);
    let unknown = certificate(
        &unknown_root,
        "node-a",
        CertificatePurpose::Transport,
        1,
        &key(20),
    );
    assert_eq!(
        store.activate(&unknown, NOW).unwrap_err(),
        CertificateError::IssuerNotApproved
    );

    let mut tampered = certificate(&root, "node-a", CertificatePurpose::Transport, 1, &key(21));
    tampered.body.holder_id = "node-b".to_owned();
    assert_eq!(
        store.activate(&tampered, NOW).unwrap_err(),
        CertificateError::InvalidIssuerSignature
    );
}

#[test]
fn rotation_overlap_is_bounded_and_one_identity_remains_one_quorum_vote() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(4);
    let store = open_store(&temp, &root);
    let first = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::GuardianControl,
        1,
        &key(30),
    );
    let second = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::GuardianControl,
        2,
        &key(31),
    );
    store.activate(&first, NOW).unwrap();
    store.activate(&second, NOW + 5).unwrap();

    assert_eq!(
        store.activate(&first, NOW + 6).unwrap_err(),
        CertificateError::GenerationNotMonotonic
    );

    assert_eq!(store.quorum_voter_count(NOW + 6).unwrap(), 1);
    store
        .authorize(
            "guardian-a",
            CertificatePurpose::GuardianControl,
            1,
            NOW + 6,
        )
        .unwrap();
    assert_eq!(
        store
            .authorize(
                "guardian-a",
                CertificatePurpose::GuardianControl,
                1,
                NOW + 35,
            )
            .unwrap_err(),
        CertificateError::RotationOverlapExpired
    );
    store
        .authorize(
            "guardian-a",
            CertificatePurpose::GuardianControl,
            2,
            NOW + 35,
        )
        .unwrap();
    assert_eq!(store.quorum_voter_count(NOW + 35).unwrap(), 1);
}

#[test]
fn one_guardian_control_key_cannot_represent_two_voters() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(5);
    let store = open_store(&temp, &root);
    let shared = key(40);
    store
        .activate(
            &certificate(
                &root,
                "guardian-a",
                CertificatePurpose::GuardianControl,
                1,
                &shared,
            ),
            NOW,
        )
        .unwrap();
    assert_eq!(
        store
            .activate(
                &certificate(
                    &root,
                    "guardian-b",
                    CertificatePurpose::GuardianControl,
                    1,
                    &shared,
                ),
                NOW + 1,
            )
            .unwrap_err(),
        CertificateError::DuplicateGuardianControlKey
    );
    assert_eq!(store.quorum_voter_count(NOW + 1).unwrap(), 1);
}

#[test]
fn revocation_is_immediate_and_authorization_is_refresh_bounded() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(6);
    let store = open_store(&temp, &root);
    let cert = certificate(&root, "node-a", CertificatePurpose::Transport, 1, &key(50));
    store.activate(&cert, NOW).unwrap();
    let verified = store
        .authorize("node-a", CertificatePurpose::Transport, 1, NOW + 1)
        .unwrap();
    assert_eq!(verified.authorization_deadline_unix_secs, NOW + 11);
    store
        .revoke(
            &verified.certificate_id,
            NOW + 2,
            RevocationReason::OperatorRevoked,
        )
        .unwrap();
    assert_eq!(
        store
            .authorize("node-a", CertificatePurpose::Transport, 1, NOW + 2)
            .unwrap_err(),
        CertificateError::Revoked
    );
}

#[test]
fn expiry_and_not_yet_valid_are_enforced_before_authority() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(7);
    let store = open_store(&temp, &root);
    let future = AuthorityCertificate::issue(
        CertificateBody::new(
            DOMAIN,
            "node-a",
            CertificatePurpose::Transport,
            1,
            CertificateValidity {
                issued_at_unix_secs: NOW + 10,
                expires_at_unix_secs: NOW + 20,
            },
            key(60).verifying_key(),
            &root.verifying_key(),
        ),
        &root,
    )
    .unwrap();
    assert_eq!(
        store.activate(&future, NOW).unwrap_err(),
        CertificateError::NotYetValid
    );
    store.activate(&future, NOW + 10).unwrap();
    assert_eq!(
        store
            .authorize("node-a", CertificatePurpose::Transport, 1, NOW + 20)
            .unwrap_err(),
        CertificateError::Expired
    );
}

#[test]
fn compromised_certificate_fences_all_identity_purposes_and_reenrollment() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(8);
    let store = open_store(&temp, &root);
    let transport = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::Transport,
        1,
        &key(70),
    );
    let advertisement = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::AdvertisementSigning,
        1,
        &key(71),
    );
    store.activate(&transport, NOW).unwrap();
    store.activate(&advertisement, NOW).unwrap();
    store
        .mark_compromised(&transport.certificate_id().unwrap(), NOW + 1)
        .unwrap();
    for purpose in [
        CertificatePurpose::Transport,
        CertificatePurpose::AdvertisementSigning,
    ] {
        assert_eq!(
            store
                .authorize("guardian-a", purpose, 1, NOW + 1)
                .unwrap_err(),
            CertificateError::IdentityFenced
        );
    }
    let replacement = certificate(
        &root,
        "guardian-a",
        CertificatePurpose::Transport,
        2,
        &key(72),
    );
    assert_eq!(
        store.activate(&replacement, NOW + 2).unwrap_err(),
        CertificateError::IdentityFenced
    );
}

#[test]
fn rotation_revocation_and_compromise_survive_restart() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(9);
    let transport = certificate(&root, "node-a", CertificatePurpose::Transport, 1, &key(80));
    let snapshot = certificate(
        &root,
        "node-b",
        CertificatePurpose::SnapshotSigning,
        1,
        &key(81),
    );
    {
        let store = open_store(&temp, &root);
        store.activate(&transport, NOW).unwrap();
        store.activate(&snapshot, NOW).unwrap();
        store
            .revoke(
                &transport.certificate_id().unwrap(),
                NOW + 1,
                RevocationReason::OperatorRevoked,
            )
            .unwrap();
        store
            .mark_compromised(&snapshot.certificate_id().unwrap(), NOW + 1)
            .unwrap();
    }

    let restarted = open_store(&temp, &root);
    assert_eq!(
        restarted
            .authorize("node-a", CertificatePurpose::Transport, 1, NOW + 2)
            .unwrap_err(),
        CertificateError::Revoked
    );
    assert_eq!(
        restarted
            .authorize("node-b", CertificatePurpose::SnapshotSigning, 1, NOW + 2,)
            .unwrap_err(),
        CertificateError::IdentityFenced
    );
    assert_eq!(
        restarted.database_path(),
        temp.path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb")
    );
}

#[test]
fn corrupt_durable_certificate_state_fails_closed_on_restart() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(10);
    {
        let store = open_store(&temp, &root);
        let cert = certificate(&root, "node-a", CertificatePurpose::Transport, 1, &key(82));
        store.activate(&cert, NOW).unwrap();
        store
            .corrupt_certificate_for_test("node-a", CertificatePurpose::Transport, 1)
            .unwrap();
    }

    let error = DistributedCertificateStore::open(
        temp.path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        policy(&root),
    )
    .err()
    .unwrap();
    assert_eq!(error, CertificateError::DurableStateCorrupt);
}

#[test]
fn certificate_count_is_bounded_before_persisting_more_authority() {
    let temp = tempfile::tempdir().unwrap();
    let root = key(11);
    let bounded = CertificatePolicy::new(DOMAIN, [root.verifying_key()])
        .unwrap()
        .with_bounds(3_600, 30, 10, 1, 1)
        .unwrap();
    let store = DistributedCertificateStore::open(
        temp.path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb"),
        bounded,
    )
    .unwrap();
    store
        .activate(
            &certificate(&root, "node-a", CertificatePurpose::Transport, 1, &key(83)),
            NOW,
        )
        .unwrap();
    assert_eq!(
        store
            .activate(
                &certificate(&root, "node-b", CertificatePurpose::Transport, 1, &key(84),),
                NOW,
            )
            .unwrap_err(),
        CertificateError::ResourceExhausted
    );
}
