// PVF: lane=exact-child-tests; proof=production module registration plus bounded signed authority;
// deterministic=true; resource_profile=medium; release_gate=true; nonzero selection required.
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;

use std::collections::BTreeSet;

use adl_runtime::distributed::{
    capability_advertisement, certificates, discovery, failure_detection, fencing, identity,
    membership, migration, placement, projection, recovery, resource_weather, snapshot_catalog,
    transport,
};
use ed25519_dalek::SigningKey;
use lease::{
    activation_signature, encode_certificate, endorse, AuthorityApplication,
    AuthorityCertificateBodyV1, AuthorityCertificateV1, AuthorityError, AuthorityLedger,
    AuthorityMembership, ControlCertificatePurpose, LeasePolicy, OperationClass, VoterAuthority,
    AUTHORITY_CERTIFICATE_SCHEMA_VERSION, SIGNING_ALGORITHM_ED25519, TEST_LEASE_STORE_ACCESS,
};
use sha2::{Digest, Sha256};
use transport::{decode_frame, encode_frame, TransportEnvelope, TransportError, TransportLimits};

const TRUST: &[u8] = b"polis.test";
const LINEAGE: &[u8] = b"lineage-a";
const NODE: &[u8] = b"node-a";
const GUARDIAN: &[u8] = b"guardian-a";
const NOW: i64 = 1_787_000_100;

fn marker(case: &str, result: &str) {
    println!("ADL_ISSUE_5878_NEGATIVE_CASE_V1 {case} {result}");
}

fn key(seed: u8) -> SigningKey {
    SigningKey::from_bytes(&[seed; 32])
}

fn lease_policy() -> LeasePolicy {
    LeasePolicy {
        max_lease_duration_millis: 1_000,
        max_clock_uncertainty_millis: 10,
        message_delay_margin_millis: 5,
        max_lineages: 8,
        max_snapshot_bytes: 1024 * 1024,
    }
}

struct QuorumFixture {
    voter_ids: Vec<Vec<u8>>,
    voter_keys: Vec<SigningKey>,
    membership: AuthorityMembership,
}

impl QuorumFixture {
    fn new() -> Self {
        let voter_ids = (0..3)
            .map(|index| format!("voter-{index}").into_bytes())
            .collect::<Vec<_>>();
        let voter_keys = (1..=3).map(key).collect::<Vec<_>>();
        let voters = voter_ids
            .iter()
            .zip(&voter_keys)
            .map(|(guardian_id, signing_key)| VoterAuthority {
                guardian_id: guardian_id.clone(),
                trust_domain_id: TRUST.to_vec(),
                certificate_generation: 7,
                purpose: ControlCertificatePurpose::AuthorityEndorsement,
                not_before_unix_seconds: NOW - 100,
                not_after_unix_seconds: NOW + 100,
                revoked: false,
                control_public_key: signing_key.verifying_key().to_bytes(),
            })
            .collect::<Vec<_>>();
        let configuration = voter_ids.iter().cloned().collect::<BTreeSet<_>>();
        let membership =
            AuthorityMembership::new(TRUST.to_vec(), 7, 10, vec![configuration], voters).unwrap();
        Self {
            voter_ids,
            voter_keys,
            membership,
        }
    }

    fn certificate(&self, body: AuthorityCertificateBodyV1) -> Vec<u8> {
        let mut endorsements = [0, 1]
            .into_iter()
            .map(|index| {
                endorse(
                    &body,
                    self.voter_ids[index].clone(),
                    7,
                    &self.voter_keys[index],
                )
            })
            .collect::<Vec<_>>();
        endorsements.sort_by(|left, right| left.signer_guardian_id.cmp(&right.signer_guardian_id));
        encode_certificate(&AuthorityCertificateV1 {
            body: Some(body),
            endorsements,
        })
        .unwrap()
    }
}

fn grant_body(trust_domain_id: &[u8], activation: &SigningKey) -> AuthorityCertificateBodyV1 {
    AuthorityCertificateBodyV1 {
        schema_version: AUTHORITY_CERTIFICATE_SCHEMA_VERSION,
        trust_domain_id: trust_domain_id.to_vec(),
        lineage_id: LINEAGE.to_vec(),
        voter_set_generation: 7,
        raft_term: 1,
        committed_log_index: 10,
        epoch: 1,
        holder_node_id: NODE.to_vec(),
        holder_guardian_id: GUARDIAN.to_vec(),
        activation_key_sha256: Sha256::digest(activation.verifying_key().to_bytes()).to_vec(),
        operation_class: OperationClass::LeaseGrant as u32,
        issued_unix_seconds: NOW,
        issued_nanos: 0,
        lease_duration_millis: 1_000,
        policy_sha256: lease_policy().sha256().unwrap().to_vec(),
        signing_algorithm: SIGNING_ALGORITHM_ED25519,
    }
}

fn application<'a>(activation: &SigningKey, proof: &'a [u8]) -> AuthorityApplication<'a> {
    AuthorityApplication {
        now_unix_seconds: NOW,
        now_unix_nanos: 0,
        now_elapsed_millis: 10,
        clock_uncertainty_millis: 1,
        activation_public_key: activation.verifying_key().to_bytes(),
        activation_proof: proof,
    }
}

#[test]
fn all_fifteen_distributed_contracts_are_registered_and_protobuf_is_bounded() {
    let schemas = [
        identity::IDENTITY_SCHEMA,
        certificates::CERTIFICATE_SCHEMA,
        transport::TRANSPORT_SCHEMA,
        discovery::JOIN_REQUEST_SCHEMA,
        membership::MEMBERSHIP_EVENT_SCHEMA,
        failure_detection::FAILURE_PROBE_SCHEMA,
        lease::AUTHORITY_SNAPSHOT_SCHEMA,
        fencing::FENCING_STATE_SCHEMA,
        capability_advertisement::CAPABILITY_ADVERTISEMENT_SCHEMA,
        resource_weather::RESOURCE_WEATHER_SCHEMA,
        snapshot_catalog::SNAPSHOT_CATALOG_SCHEMA,
        migration::MIGRATION_STATE_SCHEMA,
        recovery::RECOVERY_STATE_SCHEMA,
        projection::PROJECTION_SCHEMA_V1,
    ];
    let placement_registered = placement::PlacementPolicy::new("polis.test").is_ok();
    assert_eq!(schemas.len() + usize::from(placement_registered), 15);
    assert!(schemas
        .iter()
        .all(|schema| schema.starts_with("adl.distributed.")));

    let limits = TransportLimits::bounded(
        256,
        4,
        std::time::Duration::from_secs(1),
        std::time::Duration::from_secs(1),
    )
    .unwrap();
    let envelope = TransportEnvelope {
        schema: transport::TRANSPORT_SCHEMA.into(),
        trust_domain: "polis.test".into(),
        node_id: "node-a".into(),
        guardian_id: "guardian-a".into(),
        protocol_version: 1,
        certificate_generation: 1,
        sequence: 1,
        payload: b"bounded-protobuf".to_vec(),
    };
    let encoded = encode_frame(envelope.clone(), &limits).unwrap();
    assert_eq!(decode_frame(&encoded, &limits).unwrap(), envelope);

    let mut oversized = envelope;
    oversized.payload = vec![0; 257];
    assert_eq!(
        encode_frame(oversized, &limits),
        Err(TransportError::FrameTooLarge)
    );
    marker("oversized_protobuf_frame", "rejected");
}

#[test]
fn signed_quorum_authority_applies_once_and_rejects_replay_and_wrong_domain() {
    let fixture = QuorumFixture::new();
    let activation = key(90);
    let body = grant_body(TRUST, &activation);
    let proof = activation_signature(&body, &activation);
    let certificate = fixture.certificate(body);
    let mut ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();

    let lease = ledger
        .apply(
            &TEST_LEASE_STORE_ACCESS,
            &certificate,
            &fixture.membership,
            application(&activation, &proof),
        )
        .unwrap();
    assert_eq!(lease.lineage_id, LINEAGE);
    assert_eq!(lease.holder_node_id, NODE);
    assert!(!lease.revoked);
    assert!(!ledger.snapshot().unwrap().is_empty());

    assert_eq!(
        ledger.apply(
            &TEST_LEASE_STORE_ACCESS,
            &certificate,
            &fixture.membership,
            application(&activation, &proof),
        ),
        Err(AuthorityError::Replay)
    );
    marker("authority_replay", "rejected");

    let wrong_body = grant_body(b"other.test", &activation);
    let wrong_proof = activation_signature(&wrong_body, &activation);
    let wrong_certificate = fixture.certificate(wrong_body);
    let mut wrong_ledger = AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap();
    assert_eq!(
        wrong_ledger.apply(
            &TEST_LEASE_STORE_ACCESS,
            &wrong_certificate,
            &fixture.membership,
            application(&activation, &wrong_proof),
        ),
        Err(AuthorityError::WrongTrustDomain)
    );
    marker("wrong_authority_domain", "rejected");
}
