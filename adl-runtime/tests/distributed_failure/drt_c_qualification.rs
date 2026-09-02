use adl_runtime::qualification::{DistributedQualificationContract, DrtCQualificationDecision};

const RUNTIME_REVISION: &str = "d022d6c198669bcbc10cd98bee4d7c8520f9c4d4";

#[test]
fn drt_c_final_qualification_decision_is_exact() {
    let decision = DistributedQualificationContract::deterministic_drt_a()
        .deterministic_drt_c(RUNTIME_REVISION)
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

    let retained: DrtCQualificationDecision = serde_json::from_str(include_str!(
        "../../../docs/milestones/v0.92.1/evidence/runtime/drt-c/qualification.json"
    ))
    .expect("retained DRT-C qualification json");
    assert_eq!(
        retained, decision,
        "retained DRT-C evidence must match the deterministic Runtime decision"
    );
}
