use super::*;

#[test]
fn runtime_v2_constructability_anchor_validator_contract_is_stable() {
    let packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet
        .validate()
        .expect("valid constructability anchor validator packet");

    assert_eq!(
        packet.schema_version,
        RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_SCHEMA
    );
    assert_eq!(packet.milestone, "v0.91.7");
    assert_eq!(packet.wp, "WP-10");
    assert_eq!(packet.construction_events.len(), 2);
    assert_eq!(packet.admissible_anchors.len(), 2);
    assert!(packet.shared_reality_boundary.promotion_requires_anchor);
    assert!(packet
        .validation_commands
        .iter()
        .any(|command| command.contains(RUNTIME_V2_CONSTRUCTABILITY_ANCHOR_VALIDATOR_TEST_MARKER)));
}

#[test]
fn runtime_v2_constructability_anchor_validator_canonical_json_is_deterministic() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    let expected = packet
        .pretty_json_bytes()
        .expect("baseline canonical constructability json");
    packet.construction_events.reverse();
    for event in &mut packet.construction_events {
        event.anchor_refs.reverse();
        event.validator_refs.reverse();
    }
    packet.admissible_anchors.reverse();
    packet.decisions.reverse();
    for decision in &mut packet.decisions {
        decision.blocking_reasons.reverse();
        decision.evidence_refs.reverse();
    }
    packet.validation_commands.reverse();

    let json = String::from_utf8(packet.pretty_json_bytes().expect("constructability json"))
        .expect("utf8 constructability json");
    assert_eq!(json.as_bytes(), expected);
    let reparsed: RuntimeV2ConstructabilityAnchorValidatorPacket =
        serde_json::from_str(&json).expect("reparse constructability json");

    assert_eq!(
        reparsed.construction_events[0].event_id,
        "event-curiosity-proposal-admission"
    );
    assert_eq!(
        reparsed.decisions[0].decision_id,
        "decision-curiosity-proposal-admitted"
    );
    reparsed
        .validate()
        .expect("canonical constructability packet remains valid");
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_unanchored_promotion_pass() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    let decision = packet
        .decisions
        .iter_mut()
        .find(|decision| decision.event_id == "event-unanchored-promotion-attempt")
        .expect("unanchored decision");
    decision.outcome = RuntimeV2ConstructabilityOutcome::Pass;
    decision.blocking_reasons.clear();
    decision
        .evidence_refs
        .push("anchor-curiosity-engine-packet".to_string());

    assert!(packet
        .validate()
        .expect_err("unanchored shared-reality promotion must fail closed")
        .to_string()
        .contains("must fail closed"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_non_admissible_evidence() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.admissible_anchors[0].admissibility = RuntimeV2ConstructabilityAdmissibility::Rejected;

    assert!(packet
        .validate()
        .expect_err("pass decision must cite admissible anchors")
        .to_string()
        .contains("non-admissible anchor"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_disabled_boundary() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.shared_reality_boundary.promotion_requires_anchor = false;

    assert!(packet
        .validate()
        .expect_err("boundary must require anchors")
        .to_string()
        .contains("admissible anchor"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_missing_event_anchor() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.construction_events[0]
        .anchor_refs
        .push("anchor-does-not-exist".to_string());

    assert!(packet
        .validate()
        .expect_err("event anchor refs must resolve")
        .to_string()
        .contains("cites missing anchor"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_duplicate_event_decision() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    let mut duplicate = packet.decisions[0].clone();
    duplicate.decision_id = "decision-duplicate-for-event".to_string();
    packet.decisions.push(duplicate);

    assert!(packet
        .validate()
        .expect_err("one event must have exactly one decision")
        .to_string()
        .contains("multiple validator decisions"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_unretained_failure_reason() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.failure_modes[0].expected_error = "different failure reason".to_string();

    assert!(packet
        .validate()
        .expect_err("failure reason must be retained in the decision")
        .to_string()
        .contains("expected error is not retained"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_unknown_validator_ref() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.construction_events[0].validator_refs = vec!["validator-unknown".to_string()];

    assert!(packet
        .validate()
        .expect_err("events must cite the canonical validator")
        .to_string()
        .contains("must cite exactly canonical validator"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_additional_validator_ref() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.construction_events[0]
        .validator_refs
        .push("validator-unknown".to_string());

    assert!(packet
        .validate()
        .expect_err("events must not cite additional validators")
        .to_string()
        .contains("must cite exactly canonical validator"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_partial_pass_evidence() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.decisions[0].evidence_refs.pop();

    assert!(packet
        .validate()
        .expect_err("pass evidence must retain all declared anchors")
        .to_string()
        .contains("must retain every declared event anchor"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_duplicate_pass_evidence() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    let duplicate = packet.decisions[0].evidence_refs[0].clone();
    packet.decisions[0].evidence_refs.push(duplicate);

    assert!(packet
        .validate()
        .expect_err("pass evidence refs must be unique")
        .to_string()
        .contains("repeats evidence ref"));
}

#[test]
fn runtime_v2_constructability_anchor_validator_rejects_shared_reality_without_operator_approval() {
    let mut packet = runtime_v2_constructability_anchor_validator_contract()
        .expect("constructability anchor validator packet");
    packet.construction_events[0].requested_publication =
        RuntimeV2ConstructabilityPublicationScope::SharedReality;
    packet.construction_events[0]
        .anchor_refs
        .retain(|anchor_ref| anchor_ref != "anchor-operator-review-boundary");
    packet.decisions[0]
        .evidence_refs
        .retain(|anchor_ref| anchor_ref != "anchor-operator-review-boundary");

    assert!(packet
        .validate()
        .expect_err("shared-reality pass requires operator approval")
        .to_string()
        .contains("requires an admissible operator-approval anchor"));
}
