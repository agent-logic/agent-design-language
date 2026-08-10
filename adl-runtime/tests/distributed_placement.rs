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

use capability_advertisement::{CapabilityEvidence, VerifiedCapabilityAdvertisement};
use fencing::FenceReceipt;
use lease::OperationClass;
use membership::{
    CommittedMembershipEvent, Member, MemberRole, MembershipOperation, MembershipPolicy,
    MembershipState,
};
use placement::{
    decide, CapabilityRequirement, PlacementError, PlacementInputs, PlacementPolicy,
    PlacementRequest,
};
use resource_weather::{PlacementWeather, WeatherAvailability};

const TRUST: &str = "polis.test";
const NOW: u64 = 1_787_000_100;

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
    let mut state = MembershipState::new(MembershipPolicy::new(TRUST, 32, 64).unwrap());
    let mut epoch = 0_u64;
    for index in voters.iter().chain(non_voters) {
        epoch += 1;
        state
            .apply(&CommittedMembershipEvent::new(
                TRUST,
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
                TRUST,
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
        certificate_id: format!("cert-{holder}-{generation}"),
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

fn request(state: &MembershipState) -> PlacementRequest {
    PlacementRequest {
        lineage_id: "lineage-a".to_owned(),
        minimum_membership_epoch: state.epoch(),
        minimum_committed_log_index: state.committed_log_index(),
        required_slots: 2,
        now_unix_secs: NOW,
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
    decide(
        &PlacementPolicy::new(TRUST).unwrap(),
        &request(state),
        PlacementInputs {
            membership: state,
            capabilities,
            weather,
            fencing,
        },
    )
}

#[test]
fn deterministic_ranking_is_independent_of_input_order() {
    let state = membership(&[1, 2, 3], &[]);
    let capabilities = vec![
        capability("guardian-1", 3, 11, NOW + 60),
        capability("guardian-2", 3, 12, NOW + 60),
        capability("guardian-3", 3, 13, NOW + 60),
    ];
    let weather_rows = vec![
        weather("node-1", 3, 21, 500, 8, NOW + 60),
        weather("node-2", 3, 22, 200, 4, NOW + 60),
        weather("node-3", 3, 23, 200, 9, NOW + 60),
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
fn stable_node_identifier_breaks_an_exact_score_tie() {
    let state = membership(&[1, 2], &[]);
    let capabilities = vec![
        capability("guardian-2", 3, 12, NOW + 60),
        capability("guardian-1", 3, 11, NOW + 60),
    ];
    let weather_rows = vec![
        weather("node-2", 3, 22, 200, 8, NOW + 60),
        weather("node-1", 3, 21, 200, 8, NOW + 60),
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
        weather("node-1", 3, 21, 100, 8, NOW + 60),
        weather("node-2", 3, 22, 400, 8, NOW + 60),
    ];
    let result = decide_with(
        &state,
        &capabilities,
        &weather_rows,
        &[fence("node-1", state.committed_log_index())],
    )
    .unwrap();
    assert_eq!(result.node_id, "node-2");
    marker("fenced_node_excluded", "fenced");
}

#[test]
fn stale_capability_and_weather_never_place() {
    let state = membership(&[1], &[]);
    let stale_capability = [capability("guardian-1", 3, 11, NOW)];
    let fresh_weather = [weather("node-1", 3, 21, 100, 8, NOW + 60)];
    assert_eq!(
        decide_with(&state, &stale_capability, &fresh_weather, &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    let fresh_capability = [capability("guardian-1", 3, 11, NOW + 60)];
    let stale_weather = [weather("node-1", 3, 21, 100, 8, NOW)];
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
            &[weather("node-1", 3, 21, 100, 8, NOW + 60)],
            &[]
        ),
        Err(PlacementError::WrongTrustDomain)
    );
    marker("wrong_trust_domain_denied", "denied");
}

#[test]
fn stale_membership_floor_fails_closed() {
    let state = membership(&[1], &[]);
    let mut request = request(&state);
    request.minimum_committed_log_index += 1;
    assert_eq!(
        decide(
            &PlacementPolicy::new(TRUST).unwrap(),
            &request,
            PlacementInputs {
                membership: &state,
                capabilities: &[],
                weather: &[],
                fencing: &[],
            },
        ),
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
            &[weather("node-1", 3, 21, 100, 8, NOW + 60)],
            &[fence("node-1", state.committed_log_index() + 1)],
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
            weather("node-1", 3, 21, 100, 1, NOW + 60),
        ),
        (
            {
                let mut value = capability("guardian-1", 3, 11, NOW + 60);
                value.capabilities[0].observed_units = 1;
                value
            },
            weather("node-1", 3, 21, 100, 8, NOW + 60),
        ),
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("node-1", 3, 21, 901, 8, NOW + 60),
        ),
        (
            capability("guardian-1", 3, 11, NOW + 60),
            weather("node-1", 4, 21, 100, 8, NOW + 60),
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
    let mut unavailable = weather("node-1", 3, 21, 100, 8, NOW + 60);
    unavailable.availability = WeatherAvailability::Unavailable;
    assert_eq!(
        decide_with(&state, &capabilities, &[unavailable], &[]),
        Err(PlacementError::NoEligibleTarget)
    );
    let mut commanding = weather("node-1", 3, 21, 100, 8, NOW + 60);
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
            &[weather("node-1", 3, 21, 100, 8, NOW + 60)],
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
    let weather = weather("node-1", 3, 21, 100, 8, NOW + 60);
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
    let mut noncanonical = request(&state);
    noncanonical.requirements.reverse();
    assert_eq!(
        decide(
            &PlacementPolicy::new(TRUST).unwrap(),
            &noncanonical,
            PlacementInputs {
                membership: &state,
                capabilities: &[],
                weather: &[],
                fencing: &[],
            }
        ),
        Err(PlacementError::NonCanonicalRequirements)
    );
    let bounded = PlacementPolicy::with_bounds(TRUST, 1, 4, 10, 20, 4, 1_000, 0).unwrap();
    assert_eq!(
        decide(
            &bounded,
            &request(&state),
            PlacementInputs {
                membership: &state,
                capabilities: &[
                    capability("guardian-1", 3, 1, NOW + 60),
                    capability("node-1", 3, 2, NOW + 60),
                ],
                weather: &[],
                fencing: &[],
            }
        ),
        Err(PlacementError::ResourceExhausted)
    );
    marker("policy_bounds_enforced", "fail_closed");
}
