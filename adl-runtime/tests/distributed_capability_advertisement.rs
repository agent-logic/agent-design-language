#[path = "../src/distributed/capability_advertisement.rs"]
mod capability_advertisement;
// PVF: lane=exact-child-tests; proof=signed capability advertisement positive and negative behavior;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;

use std::{path::PathBuf, sync::Arc};

use capability_advertisement::{
    AdvertisementError, CapabilityAdvertisementBody, CapabilityAdvertisementPolicy,
    CapabilityAdvertisementVerifier, CapabilityEvidence, SignedCapabilityAdvertisement,
};
use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, RevocationReason,
    TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::SigningKey;

const DOMAIN: &str = "polis.example";
const NOW: u64 = 20_000;

struct Fixture {
    _directory: tempfile::TempDir,
    replay_path: PathBuf,
    store: Arc<DistributedCertificateStore>,
    policy: CapabilityAdvertisementPolicy,
    root: SigningKey,
    signer: SigningKey,
    certificate: AuthorityCertificate,
}

impl Fixture {
    fn new(holder: &str, purpose: CertificatePurpose, generation: u64) -> Self {
        let root = SigningKey::from_bytes(&[41; 32]);
        let signer = SigningKey::from_bytes(&[42; 32]);
        let certificate_policy = CertificatePolicy::new(DOMAIN, [root.verifying_key()])
            .unwrap()
            .with_bounds(3600, 60, 60, 64, 64)
            .unwrap();
        let directory = tempfile::tempdir().unwrap();
        let canonical_directory = directory.path().canonicalize().unwrap();
        let path = canonical_directory.join("certificates.redb");
        let replay_path = canonical_directory.join("capability-replay.redb");
        let store = Arc::new(
            DistributedCertificateStore::open(
                &TEST_CERTIFICATE_STORE_ACCESS,
                path,
                certificate_policy,
            )
            .unwrap(),
        );
        let body = CertificateBody::new(
            DOMAIN,
            holder,
            purpose,
            generation,
            CertificateValidity {
                issued_at_unix_secs: NOW - 100,
                expires_at_unix_secs: NOW + 600,
            },
            signer.verifying_key(),
            &root.verifying_key(),
        );
        let certificate = AuthorityCertificate::issue(body, &root).unwrap();
        store
            .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW - 10)
            .unwrap();
        Self {
            _directory: directory,
            replay_path,
            store,
            policy: policy(DOMAIN),
            root,
            signer,
            certificate,
        }
    }

    fn verifier(&self) -> CapabilityAdvertisementVerifier {
        CapabilityAdvertisementVerifier::open_for_test(
            self.store.clone(),
            self.policy.clone(),
            &self.replay_path,
        )
        .unwrap()
    }

    fn advertisement(
        &self,
        sequence: u64,
        capabilities: impl IntoIterator<Item = CapabilityEvidence>,
    ) -> SignedCapabilityAdvertisement {
        let body = CapabilityAdvertisementBody::new(
            DOMAIN,
            &self.certificate.body.holder_id,
            self.certificate.body.generation,
            sequence,
            NOW - 1,
            NOW + 60,
            capabilities,
            &self.policy,
        )
        .unwrap();
        SignedCapabilityAdvertisement::issue(
            body,
            self.certificate.clone(),
            &self.signer,
            &self.policy,
        )
        .unwrap()
    }

    fn timed_advertisement(
        &self,
        sequence: u64,
        measured_at_unix_secs: u64,
        expires_at_unix_secs: u64,
    ) -> SignedCapabilityAdvertisement {
        let body = CapabilityAdvertisementBody::new(
            DOMAIN,
            &self.certificate.body.holder_id,
            self.certificate.body.generation,
            sequence,
            measured_at_unix_secs,
            expires_at_unix_secs,
            [evidence("tool:rust", 1)],
            &self.policy,
        )
        .unwrap();
        SignedCapabilityAdvertisement::issue(
            body,
            self.certificate.clone(),
            &self.signer,
            &self.policy,
        )
        .unwrap()
    }

    fn additional_advertisement(
        &self,
        holder: &str,
        signer_seed: u8,
        sequence: u64,
    ) -> SignedCapabilityAdvertisement {
        let signer = SigningKey::from_bytes(&[signer_seed; 32]);
        let certificate = AuthorityCertificate::issue(
            CertificateBody::new(
                DOMAIN,
                holder,
                CertificatePurpose::AdvertisementSigning,
                1,
                CertificateValidity {
                    issued_at_unix_secs: NOW - 100,
                    expires_at_unix_secs: NOW + 600,
                },
                signer.verifying_key(),
                &self.root.verifying_key(),
            ),
            &self.root,
        )
        .unwrap();
        self.store
            .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW - 10)
            .unwrap();
        let body = CapabilityAdvertisementBody::new(
            DOMAIN,
            holder,
            1,
            sequence,
            NOW - 1,
            NOW + 60,
            [evidence("tool:rust", 1)],
            &self.policy,
        )
        .unwrap();
        SignedCapabilityAdvertisement::issue(body, certificate, &signer, &self.policy).unwrap()
    }
}

fn policy(domain: &str) -> CapabilityAdvertisementPolicy {
    CapabilityAdvertisementPolicy::new(domain)
        .unwrap()
        .with_bounds(4, 100, 200, 120, 30, 2, 4096, 2)
        .unwrap()
}

fn evidence(name: &str, units: u32) -> CapabilityEvidence {
    CapabilityEvidence::new(name, units)
}

#[test]
fn canonical_signed_advertisement_projects_deterministic_evidence_only() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 3);
    let advertisement = fixture.advertisement(
        7,
        [
            evidence("tool:rust", 4),
            evidence("model:reasoning", 2),
            evidence("tool:rust", 4),
        ],
    );
    assert_eq!(
        advertisement
            .body
            .capabilities
            .iter()
            .map(|entry| entry.capability.as_str())
            .collect::<Vec<_>>(),
        ["model:reasoning", "tool:rust"]
    );

    let bytes = advertisement.encode(&fixture.policy).unwrap();
    let verified = fixture.verifier().decode_and_verify(&bytes, NOW).unwrap();
    assert_eq!(verified.issuer_id, "guardian-a");
    assert_eq!(verified.certificate_generation, 3);
    assert_eq!(verified.sequence, 7);
    assert!(verified.verification_deadline_unix_secs <= verified.expires_at_unix_secs);
    assert_eq!(verified.evidence(), advertisement.body.capabilities);
    let wire = String::from_utf8(bytes).unwrap();
    assert!(!wire.contains("lease"));
    assert!(!wire.contains("fencing"));
    assert!(!wire.contains("placement"));
    assert!(!wire.contains("command"));
}

#[test]
fn wrong_signer_is_rejected_before_publication() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let body = CapabilityAdvertisementBody::new(
        DOMAIN,
        "guardian-a",
        1,
        1,
        NOW,
        NOW + 60,
        [evidence("tool:rust", 1)],
        &fixture.policy,
    )
    .unwrap();
    let unrelated = SigningKey::from_bytes(&[99; 32]);
    assert_eq!(
        SignedCapabilityAdvertisement::issue(
            body,
            fixture.certificate,
            &unrelated,
            &fixture.policy
        )
        .unwrap_err(),
        AdvertisementError::WrongSigner
    );
}

#[test]
fn wrong_purpose_domain_holder_generation_and_certificate_fail_closed() {
    let wrong_purpose = Fixture::new("guardian-a", CertificatePurpose::GuardianControl, 1);
    let body = CapabilityAdvertisementBody::new(
        DOMAIN,
        "guardian-a",
        1,
        1,
        NOW,
        NOW + 60,
        [evidence("tool:rust", 1)],
        &wrong_purpose.policy,
    )
    .unwrap();
    assert_eq!(
        SignedCapabilityAdvertisement::issue(
            body,
            wrong_purpose.certificate,
            &wrong_purpose.signer,
            &wrong_purpose.policy
        )
        .unwrap_err(),
        AdvertisementError::CertificateAuthorization
    );

    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    for (domain, holder, generation) in [
        ("other.example", "guardian-a", 1),
        (DOMAIN, "guardian-b", 1),
        (DOMAIN, "guardian-a", 2),
    ] {
        let mismatched_policy = policy(domain);
        let body = CapabilityAdvertisementBody::new(
            domain,
            holder,
            generation,
            1,
            NOW,
            NOW + 60,
            [evidence("tool:rust", 1)],
            &mismatched_policy,
        )
        .unwrap();
        assert_eq!(
            SignedCapabilityAdvertisement::issue(
                body,
                fixture.certificate.clone(),
                &fixture.signer,
                &mismatched_policy
            )
            .unwrap_err(),
            AdvertisementError::CertificateAuthorization
        );
    }

    let beyond_certificate_body = CapabilityAdvertisementBody::new(
        DOMAIN,
        "guardian-a",
        1,
        4,
        NOW + 550,
        NOW + 650,
        [evidence("tool:rust", 1)],
        &fixture.policy,
    )
    .unwrap();
    assert_eq!(
        SignedCapabilityAdvertisement::issue(
            beyond_certificate_body,
            fixture.certificate.clone(),
            &fixture.signer,
            &fixture.policy
        )
        .unwrap_err(),
        AdvertisementError::CertificateAuthorization
    );

    let mut advertisement = fixture.advertisement(1, [evidence("tool:rust", 1)]);
    advertisement.authority_certificate.issuer_signature[0] ^= 1;
    assert_eq!(
        fixture.verifier().verify(&advertisement, NOW).unwrap_err(),
        AdvertisementError::CertificateAuthorization
    );
}

#[test]
fn expiry_staleness_and_future_measurement_are_distinct_failures() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let verifier = fixture.verifier();
    let expired = fixture.advertisement(1, [evidence("tool:rust", 1)]);
    assert_eq!(
        verifier.verify(&expired, NOW + 60).unwrap_err(),
        AdvertisementError::Expired
    );

    let stale_body = CapabilityAdvertisementBody::new(
        DOMAIN,
        "guardian-a",
        1,
        2,
        NOW - 40,
        NOW + 20,
        [evidence("tool:rust", 1)],
        &fixture.policy,
    )
    .unwrap();
    let stale = SignedCapabilityAdvertisement::issue(
        stale_body,
        fixture.certificate.clone(),
        &fixture.signer,
        &fixture.policy,
    )
    .unwrap();
    assert_eq!(
        verifier.verify(&stale, NOW).unwrap_err(),
        AdvertisementError::Stale
    );

    let future_body = CapabilityAdvertisementBody::new(
        DOMAIN,
        "guardian-a",
        1,
        3,
        NOW + 3,
        NOW + 60,
        [evidence("tool:rust", 1)],
        &fixture.policy,
    )
    .unwrap();
    let future = SignedCapabilityAdvertisement::issue(
        future_body,
        fixture.certificate,
        &fixture.signer,
        &fixture.policy,
    )
    .unwrap();
    assert_eq!(
        verifier.verify(&future, NOW).unwrap_err(),
        AdvertisementError::NotYetValid
    );
}

#[test]
fn replay_and_out_of_order_sequences_fail_closed() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let first = fixture.advertisement(10, [evidence("tool:rust", 1)]);
    let replay = first.clone();
    let older = fixture.advertisement(9, [evidence("tool:rust", 1)]);
    let newer = fixture.advertisement(11, [evidence("tool:rust", 1)]);
    let verifier = fixture.verifier();
    verifier.verify(&first, NOW).unwrap();
    assert_eq!(
        verifier.verify(&replay, NOW).unwrap_err(),
        AdvertisementError::Replay
    );
    assert_eq!(
        verifier.verify(&older, NOW).unwrap_err(),
        AdvertisementError::Replay
    );
    verifier.verify(&newer, NOW).unwrap();
}

#[test]
fn replay_high_water_survives_short_newer_expiry_and_restart() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let short_newer = fixture.timed_advertisement(10, NOW - 1, NOW + 2);
    let long_older = fixture.timed_advertisement(9, NOW - 1, NOW + 60);

    let verifier = fixture.verifier();
    verifier.verify(&short_newer, NOW).unwrap();
    drop(verifier);

    let reopened = fixture.verifier();
    assert_eq!(
        reopened.verify(&short_newer, NOW + 1).unwrap_err(),
        AdvertisementError::Replay
    );
    assert_eq!(
        reopened.verify(&long_older, NOW + 3).unwrap_err(),
        AdvertisementError::Replay
    );
}

#[test]
fn configured_bounds_cannot_exceed_absolute_resource_maxima() {
    let base = CapabilityAdvertisementPolicy::new(DOMAIN).unwrap();
    let invalid = [
        base.clone()
            .with_bounds(4097, 100, 200, 120, 30, 2, 4096, 2),
        base.clone()
            .with_bounds(4, 1_000_000_001, 1_000_000_001, 120, 30, 2, 4096, 2),
        base.clone()
            .with_bounds(4, 100, 1_000_000_000_001, 120, 30, 2, 4096, 2),
        base.clone()
            .with_bounds(4, 100, 200, 86_401, 30, 2, 4096, 2),
        base.clone()
            .with_bounds(4, 100, 200, 120, 86_401, 2, 4096, 2),
        base.clone().with_bounds(4, 100, 200, 120, 30, 301, 4096, 2),
        base.clone()
            .with_bounds(4, 100, 200, 120, 30, 2, 1_048_577, 2),
        base.with_bounds(4, 100, 200, 120, 30, 2, 4096, 65_537),
    ];
    for result in invalid {
        assert_eq!(result.unwrap_err(), AdvertisementError::InvalidPolicy);
    }
}

#[test]
fn entry_unit_total_and_tracker_capacity_are_bounded() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    assert_eq!(
        CapabilityAdvertisementBody::new(
            DOMAIN,
            "guardian-a",
            1,
            1,
            NOW,
            NOW + 60,
            [evidence("tool:rust", 101)],
            &fixture.policy
        )
        .unwrap_err(),
        AdvertisementError::CapacityExceeded
    );
    assert_eq!(
        CapabilityAdvertisementBody::new(
            DOMAIN,
            "guardian-a",
            1,
            1,
            NOW,
            NOW + 60,
            [evidence("a", 100), evidence("b", 100), evidence("c", 1)],
            &fixture.policy
        )
        .unwrap_err(),
        AdvertisementError::CapacityExceeded
    );

    let first = fixture.advertisement(1, [evidence("a", 1)]);
    let second = fixture.additional_advertisement("guardian-b", 51, 1);
    let third = fixture.additional_advertisement("guardian-c", 52, 1);
    let verifier = fixture.verifier();
    verifier.verify(&first, NOW).unwrap();
    verifier.verify(&second, NOW).unwrap();
    assert_eq!(
        verifier.verify(&third, NOW).unwrap_err(),
        AdvertisementError::CapacityExceeded
    );
}

#[test]
fn conflicting_duplicate_capability_is_not_silently_merged() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    assert_eq!(
        CapabilityAdvertisementBody::new(
            DOMAIN,
            "guardian-a",
            1,
            1,
            NOW,
            NOW + 60,
            [evidence("tool:rust", 1), evidence("tool:rust", 2)],
            &fixture.policy
        )
        .unwrap_err(),
        AdvertisementError::NonCanonical
    );
}

#[test]
fn revocation_withdraws_previously_valid_advertisement() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let advertisement = fixture.advertisement(1, [evidence("tool:rust", 1)]);
    let certificate_id = fixture.certificate.certificate_id().unwrap();
    fixture
        .store
        .revoke(
            &TEST_CERTIFICATE_STORE_ACCESS,
            &certificate_id,
            NOW,
            RevocationReason::OperatorRevoked,
        )
        .unwrap();
    assert_eq!(
        fixture.verifier().verify(&advertisement, NOW).unwrap_err(),
        AdvertisementError::CertificateAuthorization
    );
}

#[test]
fn malformed_noncanonical_and_oversized_wire_inputs_fail_closed() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let advertisement = fixture.advertisement(1, [evidence("tool:rust", 1)]);
    let verifier = fixture.verifier();
    assert_eq!(
        verifier.decode_and_verify(b"not-json", NOW).unwrap_err(),
        AdvertisementError::Malformed
    );
    assert_eq!(
        verifier
            .decode_and_verify(&vec![b'x'; 4097], NOW)
            .unwrap_err(),
        AdvertisementError::Oversized
    );
    let pretty = serde_json::to_vec_pretty(&advertisement).unwrap();
    assert_eq!(
        verifier.decode_and_verify(&pretty, NOW).unwrap_err(),
        AdvertisementError::NonCanonical
    );
}

#[test]
fn tampering_body_or_signature_is_rejected() {
    let fixture = Fixture::new("guardian-a", CertificatePurpose::AdvertisementSigning, 1);
    let mut body_tamper = fixture.advertisement(1, [evidence("tool:rust", 1)]);
    let mut signature_tamper = fixture.advertisement(2, [evidence("tool:rust", 1)]);
    let verifier = fixture.verifier();
    body_tamper.body.capabilities[0].observed_units = 2;
    assert_eq!(
        verifier.verify(&body_tamper, NOW).unwrap_err(),
        AdvertisementError::WrongSigner
    );
    signature_tamper.signature[0] ^= 1;
    assert_eq!(
        verifier.verify(&signature_tamper, NOW).unwrap_err(),
        AdvertisementError::WrongSigner
    );
}
