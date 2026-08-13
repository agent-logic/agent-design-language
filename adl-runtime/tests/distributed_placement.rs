#[allow(dead_code)]
#[path = "../src/distributed/capability_advertisement.rs"]
mod capability_advertisement;
#[allow(dead_code)]
#[path = "../src/distributed/certificates.rs"]
mod certificates;
#[allow(dead_code)]
#[path = "../src/distributed/fencing.rs"]
mod fencing;
#[allow(dead_code)]
#[path = "../src/distributed/lease.rs"]
mod lease;
#[allow(dead_code)]
#[path = "../src/distributed/membership.rs"]
mod membership;
#[path = "../src/distributed/placement.rs"]
mod placement;
#[allow(dead_code)]
#[path = "../src/distributed/resource_weather.rs"]
mod resource_weather;

use std::sync::Arc;

use capability_advertisement::{
    CapabilityAdvertisementBody, CapabilityAdvertisementPolicy, CapabilityAdvertisementVerifier,
    CapabilityEvidence, SignedCapabilityAdvertisement, VerifiedCapabilityAdvertisement,
};
use certificates::{
    AuthorityCertificate, CertificateBody, CertificatePolicy, CertificatePurpose,
    CertificateValidity, DistributedCertificateStore, TEST_CERTIFICATE_STORE_ACCESS,
};
use ed25519_dalek::SigningKey;
use fencing::{
    FenceReceipt, FencingCheckpoint, FencingCheckpointAuthority, FencingError, FencingPolicy,
    FencingStore, TEST_FENCING_STORE_ACCESS,
};
use lease::{
    AuthorityCertificateBodyV1, AuthorityCertificateV1, AuthorityLedger, LeasePolicy, LeaseState,
    OperationClass, TEST_LEASE_STORE_ACCESS,
};
use membership::{
    CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
    MembershipState,
};
use placement::{
    CapabilityRequirement, PlacementCapabilitySnapshot, PlacementClock, PlacementError,
    PlacementFencingSnapshot, PlacementInputs, PlacementPolicy, PlacementRequest, PlacementService,
    PlacementWeatherSnapshot,
};
use prost::Message;
use resource_weather::{
    PlacementWeather, ResourceWeatherPolicy, ResourceWeatherStore, WeatherAvailability,
};

const TRUST: &str = "polis.test";
const NOW: u64 = 1_787_000_100;

#[derive(Clone, Copy, Debug)]
struct FixedClock(u64);

impl PlacementClock for FixedClock {
    fn now_unix_secs(&self) -> Result<u64, PlacementError> {
        Ok(self.0)
    }
}

struct AdvertisementCertificates {
    _directory: tempfile::TempDir,
    store: Arc<DistributedCertificateStore>,
}

impl AdvertisementCertificates {
    fn new() -> Self {
        let root = SigningKey::from_bytes(&[91; 32]);
        let directory = tempfile::tempdir().unwrap();
        let path = directory
            .path()
            .canonicalize()
            .unwrap()
            .join("certificates.redb");
        let policy = CertificatePolicy::new(TRUST, [root.verifying_key()])
            .unwrap()
            .with_bounds(3_600, 60, 60, 64, 64)
            .unwrap();
        let store = Arc::new(
            DistributedCertificateStore::open(&TEST_CERTIFICATE_STORE_ACCESS, path, policy)
                .unwrap(),
        );
        for holder in [
            "guardian-1",
            "guardian-2",
            "guardian-3",
            "node-1",
            "intruder",
        ] {
            let certificate = advertisement_certificate(holder, 3);
            store
                .activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW - 10)
                .unwrap();
        }
        Self {
            _directory: directory,
            store,
        }
    }
}

fn advertisement_certificates() -> &'static AdvertisementCertificates {
    static CERTIFICATES: std::sync::OnceLock<AdvertisementCertificates> =
        std::sync::OnceLock::new();
    CERTIFICATES.get_or_init(AdvertisementCertificates::new)
}

fn advertisement_certificate(holder: &str, generation: u64) -> AuthorityCertificate {
    let root = SigningKey::from_bytes(&[91; 32]);
    let seed = holder
        .bytes()
        .fold(17_u8, |acc, byte| acc.wrapping_add(byte));
    let signer = SigningKey::from_bytes(&[seed; 32]);
    AuthorityCertificate::issue(
        CertificateBody::new(
            TRUST,
            holder,
            CertificatePurpose::AdvertisementSigning,
            generation,
            CertificateValidity {
                issued_at_unix_secs: NOW - 100,
                expires_at_unix_secs: NOW + 600,
            },
            signer.verifying_key(),
            &root.verifying_key(),
        ),
        &root,
    )
    .unwrap()
}

#[derive(Debug, Default)]
struct CheckpointAuthority(std::sync::Mutex<Option<FencingCheckpoint>>);

impl FencingCheckpointAuthority for CheckpointAuthority {
    fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
        Ok(*self.0.lock().unwrap())
    }

    fn compare_and_swap(
        &self,
        expected: Option<FencingCheckpoint>,
        next: FencingCheckpoint,
    ) -> Result<(), FencingError> {
        let mut current = self.0.lock().unwrap();
        if *current != expected {
            return Err(FencingError::Rollback);
        }
        *current = Some(next);
        Ok(())
    }
}

fn marker(case: &str, result: &str) {
    println!(
        "ADL_ISSUE_5873_NEGATIVE_CASE_V1 {}",
        serde_json::json!({"case": case, "result": result})
    );
}

fn member(index: u8) -> Member {
    Member {
        node_id: format!("node-{index}"),
        guardian_id: format!("guardian-{index}"),
        identity_generation: 3,
        guardian_control_public_key: [index; 32],
        role: MemberRole::NonVoting,
    }
}

fn membership(voters: &[u8], non_voters: &[u8]) -> MembershipState {
    membership_in_domain(TRUST, voters, non_voters)
}

fn membership_in_domain(domain: &str, voters: &[u8], non_voters: &[u8]) -> MembershipState {
    let mut state = MembershipState::new(MembershipPolicy::new(domain, 32, 64).unwrap());
    let mut epoch = 0_u64;
    for index in voters.iter().chain(non_voters) {
        epoch += 1;
        state
            .apply(&CommittedMembershipEvent::new(
                domain,
                [*index; 32],
                epoch,
                epoch * 10,
                MembershipOperation::Join {
                    member: member(*index),
                },
            ))
            .unwrap();
    }
    for index in voters {
        epoch += 1;
        state
            .apply(&CommittedMembershipEvent::new(
                domain,
                [index.saturating_add(100); 32],
                epoch,
                epoch * 10,
                MembershipOperation::Promote {
                    node_id: format!("node-{index}"),
                },
            ))
            .unwrap();
    }
    state
}

fn capability(
    holder: &str,
    generation: u64,
    sequence: u64,
    expires: u64,
) -> VerifiedCapabilityAdvertisement {
    VerifiedCapabilityAdvertisement {
        trust_domain: TRUST.to_owned(),
        issuer_id: holder.to_owned(),
        certificate_id: advertisement_certificate(holder, generation)
            .certificate_id()
            .unwrap(),
        certificate_generation: generation,
        sequence,
        measured_at_unix_secs: NOW - 5,
        expires_at_unix_secs: expires,
        verification_deadline_unix_secs: expires,
        capabilities: vec![
            CapabilityEvidence::new("cpu", 16),
            CapabilityEvidence::new("memory", 64),
        ],
    }
}

fn weather(
    holder: &str,
    generation: u64,
    sequence: u64,
    pressure: u16,
    slots: u16,
    expires: u64,
) -> PlacementWeather {
    PlacementWeather {
        holder_id: holder.to_owned(),
        certificate_generation: generation,
        sequence,
        sampled_at_unix_secs: NOW - 3,
        expires_at_unix_secs: expires,
        availability: WeatherAvailability::Available,
        pressure_permille: Some(pressure),
        available_slots: Some(slots),
        advisory_only: true,
    }
}

fn fence(identity: &str, log_index: u64) -> FenceReceipt {
    FenceReceipt {
        request_id: format!("fence-{identity}").into_bytes(),
        request_sha256: [1; 32],
        trust_domain_id: TRUST.as_bytes().to_vec(),
        lineage_id: identity.as_bytes().to_vec(),
        epoch: 7,
        committed_log_index: log_index,
        voter_set_generation: 4,
        operation_class: OperationClass::Fence as u32,
        certificate_sha256: [2; 32],
        safety_deadline_unix_millis: NOW * 1_000,
    }
}

fn same_epoch_successor(
    receipt: &FenceReceipt,
    operation: OperationClass,
    log_offset: u64,
) -> LeaseState {
    let committed_log_index = receipt.committed_log_index + log_offset;
    let body = AuthorityCertificateBodyV1 {
        schema_version: 1,
        trust_domain_id: TRUST.as_bytes().to_vec(),
        lineage_id: receipt.lineage_id.clone(),
        voter_set_generation: receipt.voter_set_generation,
        raft_term: 8,
        committed_log_index,
        epoch: receipt.epoch,
        holder_node_id: b"node-1".to_vec(),
        holder_guardian_id: b"guardian-1".to_vec(),
        activation_key_sha256: vec![3; 32],
        operation_class: operation as u32,
        issued_unix_seconds: NOW as i64,
        issued_nanos: 0,
        lease_duration_millis: 2_000,
        policy_sha256: vec![4; 32],
        signing_algorithm: 1,
    };
    let certificate_bytes = AuthorityCertificateV1 {
        body: Some(body),
        endorsements: Vec::new(),
    }
    .encode_to_vec();
    LeaseState {
        lineage_id: receipt.lineage_id.clone(),
        holder_node_id: b"node-1".to_vec(),
        holder_guardian_id: b"guardian-1".to_vec(),
        activation_public_key: [5; 32],
        raft_term: 8,
        committed_log_index,
        epoch: receipt.epoch,
        certificate_generation: receipt.voter_set_generation,
        activated_elapsed_millis: 100,
        deadline_elapsed_millis: 2_100,
        deadline_unix_millis: (NOW + 2) * 1_000,
        certificate_bytes,
        revoked: false,
        last_mutation_sequence: 0,
    }
}

fn request(state: &MembershipState) -> PlacementRequest {
    PlacementRequest {
        lineage_id: "lineage-a".to_owned(),
        minimum_membership_epoch: state.epoch(),
        minimum_committed_log_index: state.committed_log_index(),
        required_slots: 2,
        requirements: vec![
            CapabilityRequirement::new("cpu", 4),
            CapabilityRequirement::new("memory", 8),
        ],
    }
}

fn decide_with(
    state: &MembershipState,
    capabilities: &[VerifiedCapabilityAdvertisement],
    weather: &[PlacementWeather],
    fencing: &[FenceReceipt],
) -> Result<placement::PlacementDecision, PlacementError> {
    let policy = PlacementPolicy::new(TRUST).unwrap();
    let fencing = PlacementFencingSnapshot::from_receipts_for_test(&policy, state, fencing)?;
    let weather = weather
        .iter()
        .map(|row| {
            (
                row.clone(),
                advertisement_certificate(&row.holder_id, row.certificate_generation)
                    .certificate_id()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let weather = PlacementWeatherSnapshot::from_rows_for_test(NOW, &weather);
    let capabilities = PlacementCapabilitySnapshot::from_rows_for_test(NOW, capabilities);
    PlacementService::new(policy, FixedClock(NOW)).decide(
        &request(state),
        PlacementInputs {
            membership: state,
            capabilities: &capabilities,
            weather: &weather,
            fencing: &fencing,
        },
    )
}

fn decide_request(
    policy: PlacementPolicy,
    state: &MembershipState,
    request: &PlacementRequest,
    capabilities: &[VerifiedCapabilityAdvertisement],
    weather: &[PlacementWeather],
    fencing: &PlacementFencingSnapshot,
) -> Result<placement::PlacementDecision, PlacementError> {
    let weather = weather
        .iter()
        .map(|row| {
            (
                row.clone(),
                advertisement_certificate(&row.holder_id, row.certificate_generation)
                    .certificate_id()
                    .unwrap(),
            )
        })
        .collect::<Vec<_>>();
    let weather = PlacementWeatherSnapshot::from_rows_for_test(NOW, &weather);
    let capabilities = PlacementCapabilitySnapshot::from_rows_for_test(NOW, capabilities);
    PlacementService::new(policy, FixedClock(NOW)).decide(
        request,
        PlacementInputs {
            membership: state,
            capabilities: &capabilities,
            weather: &weather,
            fencing,
        },
    )
}

#[test]
fn deterministic_ranking_is_independent_of_input_order() {
    let _production_clock = placement::SystemPlacementClock;
    let state = membership(&[1, 2, 3], &[]);
    let capabilities = vec![
        capability("guardian-1", 3, 11, NOW + 60),
        capability("guardian-2", 3, 12, NOW + 60),
        capability("guardian-3", 3, 13, NOW + 60),
    ];
    let weather_rows = vec![
        weather("guardian-1", 3, 21, 500, 8, NOW + 60),
        weather("guardian-2", 3, 22, 200, 4, NOW + 60),
        weather("guardian-3", 3, 23, 200, 9, NOW + 60),
    ];
    let left = decide_with(&state, &capabilities, &weather_rows, &[]).unwrap();
    let mut reversed_capabilities = capabilities.clone();
    let mut reversed_weather = weather_rows.clone();
    reversed_capabilities.reverse();
    reversed_weather.reverse();
    let right = decide_with(&state, &reversed_capabilities, &reversed_weather, &[]).unwrap();
    assert_eq!(left, right);
    assert_eq!(left.node_id, "node-3");
    assert_eq!(left.remaining_slots, 7);
}

#[test]
fn signed_capability_bytes_are_verified_before_placement() {
    let certificates = advertisement_certificates();
    let policy = CapabilityAdvertisementPolicy::new(TRUST).unwrap();
    let replay = certificates
        ._directory
        .path()
        .canonicalize()
        .unwrap()
        .join("placement-capability-replay.redb");
    let verifier = CapabilityAdvertisementVerifier::open_for_test(
        certificates.store.clone(),
        policy.clone(),
        replay,
    )
    .unwrap();
    let certificate = advertisement_certificate("guardian-1", 3);
    let seed = "guardian-1"
        .bytes()
        .fold(17_u8, |acc, byte| acc.wrapping_add(byte));
    let signer = SigningKey::from_bytes(&[seed; 32]);
    let body = CapabilityAdvertisementBody::new(
        TRUST,
        "guardian-1",
        3,
        1,
        NOW - 1,
        NOW + 60,
        [
            CapabilityEvidence::new("cpu", 16),
            CapabilityEvidence::new("memory", 64),
        ],
        &policy,
    )
    .unwrap();
    let signed = SignedCapabilityAdvertisement::issue(body, certificate, &signer, &policy).unwrap();
    let encoded = signed.encode(&policy).unwrap();
    assert!(
        PlacementCapabilitySnapshot::capture(&verifier, std::slice::from_ref(&encoded), NOW)
            .is_ok()
    );
    let mut tampered = encoded;
    let last = tampered.len() - 1;
    tampered[last] ^= 1;
    assert!(matches!(
        PlacementCapabilitySnapshot::capture(&verifier, &[tampered], NOW),
        Err(PlacementError::InconsistentEvidence)
    ));
}

#[test]
fn authoritative_lineage_fencing_cannot_be_omitted() {
    let state = membership(&[1], &[]);
    let directory = tempfile::tempdir_in(std::env::current_dir().unwrap()).unwrap();
    let store = FencingStore::create(
        &TEST_FENCING_STORE_ACCESS,
        directory.path(),
        FencingPolicy {
            max_lineages: 32,
            max_receipts: 64,
            max_state_bytes: 64 * 1024,
            max_clock_uncertainty_millis: 100,
            message_delay_margin_millis: 100,
        },
        std::sync::Arc::new(CheckpointAuthority::default()),
    )
    .unwrap();
    let policy = PlacementPolicy::new(TRUST).unwrap();
    let certificates = advertisement_certificates();
    let weather_store = ResourceWeatherStore::open(
        directory.path().join("weather.redb"),
        ResourceWeatherPolicy::new(TRUST).unwrap(),
    )
    .unwrap();
    let _weather =
        PlacementWeatherSnapshot::capture(&weather_store, &certificates.store, NOW).unwrap();
    let ledger = AuthorityLedger::new(
        &TEST_LEASE_STORE_ACCESS,
        LeasePolicy {
            max_lease_duration_millis: 2_000,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
            max_lineages: 32,
            max_snapshot_bytes: 64 * 1024,
        },
    )
    .unwrap();
    assert!(matches!(
        PlacementFencingSnapshot::capture(&policy, &state, &ledger, &store),
        Err(PlacementError::StaleMembership)
    ));
    assert!(matches!(
        PlacementFencingSnapshot::missing_floor_for_test(
            &policy,
            &state,
            "lineage-1",
            "node-1",
            "guardian-1",
        ),
        Err(PlacementError::InconsistentEvidence)
    ));
    marker("caller_selected_fencing_slice_unavailable", "denied");
}

#[test]
fn stable_node_identifier_breaks_an_exact_score_tie() {
    let state = membership(&[1, 2], &[]);
    let capabilities = vec![
        capability("guardian-2", 3, 12, NOW + 60),
        capability("guardian-1", 3, 11, NOW + 60),
    ];
    let weather_rows = vec![
        weather("guardian-2", 3, 22, 200, 8, NOW + 60),
        weather("guardian-1", 3, 21, 200, 8, NOW + 60),
    ];
    assert_eq!(
        decide_with(&state, &capabilities, &weather_rows, &[])
            .unwrap()
            .node_id,
        "node-1"
    );
}

#[test]
fn fenced_candidate_is_excluded_from_selection() {
    let state = membership(&[1, 2], &[]);
    let capabilities = vec![
        capability("guardian-1", 3, 11, NOW + 60),
        capability("guardian-2", 3, 12, NOW + 60),
    ];
    let weather_rows = vec![
        weather("guardian-1", 3, 21, 100, 8, NOW + 60),
        weather("guardian-2", 3, 22, 400, 8, NOW + 60),
    ];
    let result = decide_with(
        &state,
        &capabilities,
        &weather_rows,
        &[fence("lineage-1", state.committed_log_index())],
    )
    .unwrap();
    assert_eq!(result.node_id, "node-2");
    let policy = PlacementPolicy::new(TRUST).unwrap();
    let floor = fence("lineage-1", state.committed_log_index());
    let historical = PlacementFencingSnapshot::active_successor_for_test(
        &policy,
        &state,
        same_epoch_successor(&floor, OperationClass::Activate, 1),
        floor,
    )
    .unwrap();
    let successor = decide_request(
        policy,
        &state,
        &request(&state),
        &[capability("guardian-1", 3, 31, NOW + 60)],
        &[weather("guardian-1", 3, 41, 100, 8, NOW + 60)],
        &historical,
    )
    .unwrap();
    assert_eq!(successor.node_id, "node-1");
    let renewal_floor = fence("lineage-1", state.committed_log_index());
    let renewed = PlacementFencingSnapshot::active_successor_for_test(
        &PlacementPolicy::new(TRUST).unwrap(),
        &state,
        same_epoch_successor(&renewal_floor, OperationClass::LeaseRenewal, 2),
        renewal_floor,
    )
    .unwrap();
    let renewed_successor = decide_request(
        PlacementPolicy::new(TRUST).unwrap(),
        &state,
        &request(&state),
        &[capability("guardian-1", 3, 51, NOW + 60)],
        &[weather("guardian-1", 3, 61, 100, 8, NOW + 60)],
        &renewed,
    )
    .unwrap();
    assert_eq!(renewed_successor.node_id, "node-1");
    marker("fenced_node_excluded", "fenced");
}

#[test]
fn stale_capability_and_weather_never_place() {
    let state = membership(&[1], &[]);
    let stale_capability = [capability("guardian-1", 3, 11, NOW)];
    let fresh_weather = [weather("guardian-1", 3, 21, 100, 8, NOW + 60)];
    assert_eq!(
        decide_with(&state, &stale_capability, &fresh_weather, &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    let fresh_capability = [capability("guardian-1", 3, 11, NOW + 60)];
    let stale_weather = [weather("guardian-1", 3, 21, 100, 8, NOW)];
    assert_eq!(
        decide_with(&state, &fresh_capability, &stale_weather, &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    marker("stale_advertisement_denied", "denied");
}

#[test]
fn wrong_trust_domain_fails_the_entire_decision() {
    let state = membership(&[1], &[]);
    let mut capability = capability("guardian-1", 3, 11, NOW + 60);
    capability.trust_domain = "other.test".to_owned();
    assert_eq!(
        decide_with(
            &state,
            &[capability],
            &[weather("guardian-1", 3, 21, 100, 8, NOW + 60)],
            &[]
        ),
        Err(PlacementError::WrongTrustDomain)
    );
    marker("wrong_trust_domain_denied", "denied");
}

#[test]
fn membership_domain_is_bound_by_the_authoritative_snapshot() {
    let state = membership_in_domain("other.test", &[1], &[]);
    let policy = PlacementPolicy::new(TRUST).unwrap();
    assert!(matches!(
        PlacementFencingSnapshot::from_receipts_for_test(&policy, &state, &[]),
        Err(PlacementError::WrongTrustDomain)
    ));
    marker("membership_domain_mismatch_denied", "denied");
}

#[test]
fn fencing_snapshot_cannot_be_reused_after_membership_advances() {
    let mut state = membership(&[1], &[]);
    let policy = PlacementPolicy::new(TRUST).unwrap();
    let fencing = PlacementFencingSnapshot::from_receipts_for_test(&policy, &state, &[]).unwrap();
    state
        .apply(&CommittedMembershipEvent::new(
            TRUST,
            [2; 32],
            state.epoch() + 1,
            state.committed_log_index() + 10,
            MembershipOperation::Join { member: member(2) },
        ))
        .unwrap();
    assert_eq!(
        decide_request(policy, &state, &request(&state), &[], &[], &fencing,),
        Err(PlacementError::StaleMembership)
    );
    marker("incomplete_fencing_view_denied", "denied");
}

#[test]
fn stale_membership_floor_fails_closed() {
    let state = membership(&[1], &[]);
    let policy = PlacementPolicy::new(TRUST).unwrap();
    let fencing = PlacementFencingSnapshot::from_receipts_for_test(&policy, &state, &[]).unwrap();
    let mut request = request(&state);
    request.minimum_committed_log_index += 1;
    assert_eq!(
        decide_request(policy, &state, &request, &[], &[], &fencing,),
        Err(PlacementError::StaleMembership)
    );
    marker("stale_membership_denied", "denied");
}

#[test]
fn fencing_ahead_of_membership_is_not_treated_as_authority() {
    let state = membership(&[1], &[]);
    assert_eq!(
        decide_with(
            &state,
            &[capability("guardian-1", 3, 11, NOW + 60)],
            &[weather("guardian-1", 3, 21, 100, 8, NOW + 60)],
            &[fence("lineage-1", state.committed_log_index() + 1)],
        ),
        Err(PlacementError::FencingAheadOfMembership)
    );
    marker("future_fence_denied", "denied");
}

#[test]
fn capacity_capability_pressure_and_generation_constraints_are_fail_closed() {
    let state = membership(&[1], &[]);
    let cases = [
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("guardian-1", 3, 21, 100, 1, NOW + 60),
        ),
        (
            {
                let mut value = capability("guardian-1", 3, 11, NOW + 60);
                value.capabilities[0].observed_units = 1;
                value
            },
            weather("guardian-1", 3, 21, 100, 8, NOW + 60),
        ),
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("guardian-1", 3, 21, 901, 8, NOW + 60),
        ),
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("guardian-1", 4, 21, 100, 8, NOW + 60),
        ),
        (
            {
                let mut value = capability("guardian-1", 3, 11, NOW + 60);
                value.certificate_id = "invented-certificate".to_owned();
                value
            },
            weather("guardian-1", 3, 21, 100, 8, NOW + 60),
        ),
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("node-1", 3, 21, 100, 8, NOW + 60),
        ),
    ];
    for (capability, weather) in cases {
        assert_eq!(
            decide_with(&state, &[capability], &[weather], &[]),
            Err(PlacementError::NoEligibleTarget)
        );
    }
    marker("candidate_constraints_denied", "denied");
}

#[test]
fn missing_unavailable_or_non_advisory_weather_never_places() {
    let state = membership(&[1], &[]);
    let capabilities = [capability("guardian-1", 3, 11, NOW + 60)];
    assert_eq!(
        decide_with(&state, &capabilities, &[], &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    let mut unavailable = weather("guardian-1", 3, 21, 100, 8, NOW + 60);
    unavailable.availability = WeatherAvailability::Unavailable;
    assert_eq!(
        decide_with(&state, &capabilities, &[unavailable], &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    let mut commanding = weather("guardian-1", 3, 21, 100, 8, NOW + 60);
    commanding.advisory_only = false;
    assert_eq!(
        decide_with(&state, &capabilities, &[commanding], &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    marker("unavailable_weather_denied", "denied");
}

#[test]
fn nonvoter_and_uncommitted_evidence_cannot_be_selected() {
    let state = membership(&[], &[1]);
    assert_eq!(
        decide_with(
            &state,
            &[capability("guardian-1", 3, 11, NOW + 60)],
            &[weather("guardian-1", 3, 21, 100, 8, NOW + 60)],
            &[]
        ),
        Err(PlacementError::NoEligibleTarget)
    );
    assert_eq!(
        decide_with(&state, &[capability("intruder", 3, 11, NOW + 60)], &[], &[]),
        Err(PlacementError::InconsistentEvidence)
    );
    marker("unauthorized_candidate_denied", "denied");
}

#[test]
fn duplicate_evidence_and_alias_collisions_fail_closed() {
    let state = membership(&[1], &[]);
    let capability = capability("guardian-1", 3, 11, NOW + 60);
    assert_eq!(
        decide_with(&state, &[capability.clone(), capability], &[], &[]),
        Err(PlacementError::InconsistentEvidence)
    );
    let weather = weather("guardian-1", 3, 21, 100, 8, NOW + 60);
    assert_eq!(
        decide_with(&state, &[], &[weather.clone(), weather], &[]),
        Err(PlacementError::InconsistentEvidence)
    );
    marker("duplicate_evidence_denied", "denied");
}

#[test]
fn request_and_policy_bounds_are_enforced_before_ranking() {
    assert!(matches!(
        PlacementPolicy::with_bounds(TRUST, 0, 1, 1, 1, 1, 1_000, 0),
        Err(PlacementError::InvalidPolicy)
    ));
    let state = membership(&[1], &[]);
    let default_policy = PlacementPolicy::new(TRUST).unwrap();
    let default_fencing =
        PlacementFencingSnapshot::from_receipts_for_test(&default_policy, &state, &[]).unwrap();
    let mut noncanonical = request(&state);
    noncanonical.requirements.reverse();
    assert_eq!(
        decide_request(
            default_policy,
            &state,
            &noncanonical,
            &[],
            &[],
            &default_fencing,
        ),
        Err(PlacementError::NonCanonicalRequirements)
    );
    let bounded = PlacementPolicy::with_bounds(TRUST, 1, 4, 10, 20, 4, 1_000, 0).unwrap();
    let bounded_fencing =
        PlacementFencingSnapshot::from_receipts_for_test(&bounded, &state, &[]).unwrap();
    assert_eq!(
        decide_request(
            bounded,
            &state,
            &request(&state),
            &[
                capability("guardian-1", 3, 1, NOW + 60),
                capability("node-1", 3, 2, NOW + 60),
            ],
            &[],
            &bounded_fencing,
        ),
        Err(PlacementError::ResourceExhausted)
    );
    marker("policy_bounds_enforced", "fail_closed");
}
