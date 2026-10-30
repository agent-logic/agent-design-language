use adl_runtime::qualification::{DistributedQualificationContract, DrtCQualificationDecision};

#[test]
fn drt_c_final_qualification_decision_is_exact() {
    let retained: DrtCQualificationDecision = serde_json::from_str(include_str!(
        "../../../docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json"
    ))
    .expect("retained DRT-C qualification json");

    let decision = DistributedQualificationContract::deterministic_drt_a()
        .deterministic_drt_c(&retained.runtime_revision)
        .expect("DRT-C decision");
    decision.validate().expect("DRT-C validates");

    assert_eq!(decision.issue, 508);
    assert_eq!(decision.requirements, ["#185", "#186", "#187"]);
    assert_eq!(
        decision.fail_closed_cases,
        ["identity", "provider", "transport"]
    );
    assert!(decision.observatory.runtime_emitted);
    assert!(decision.observatory.redacted);
    assert!(decision.soak.bounded);
    assert_eq!(decision.soak.required_windows.len(), 2);
    assert_eq!(decision.soak.total_duration_seconds, 1_800);
    assert!(decision
        .soak
        .attempts
        .iter()
        .all(
            |attempt| attempt.source_revision == decision.runtime_revision
                && attempt.independent_replay
                && attempt.cleanup_readback == "absent"
        ));
    assert_eq!(
        decision
            .cleanup
            .values()
            .filter(|value| *value == "absent")
            .count(),
        4
    );

    println!(
        "DRT_C_QUALIFICATION_JSON={}",
        serde_json::to_string_pretty(&decision).expect("DRT-C JSON")
    );

    assert_eq!(
        retained, decision,
        "retained DRT-C evidence must match the deterministic Runtime decision"
    );
}
