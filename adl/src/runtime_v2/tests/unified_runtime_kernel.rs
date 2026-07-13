use super::*;

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use crate::runtime_v2::tests::common::unique_temp_path;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use std::fs;

#[test]
fn runtime_v2_unified_runtime_kernel_contract_is_stable() {
    let artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");
    artifacts
        .validate()
        .expect("valid unified runtime kernel artifacts");

    assert_eq!(
        artifacts.summary.schema_version,
        RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SCHEMA
    );
    assert_eq!(artifacts.summary.issue, 5097);
    assert_eq!(artifacts.summary.milestone, "v0.91.7");
    assert!(artifacts
        .summary
        .participants
        .iter()
        .any(|participant| participant.participant_id == "scheduler_provider"));
    assert!(artifacts
        .summary
        .participants
        .iter()
        .any(|participant| participant.participant_id == "memory_obsmem"));
    assert!(artifacts
        .summary
        .participants
        .iter()
        .any(|participant| participant.participant_id == "external_signals"));
    assert!(artifacts
        .summary
        .retained_evidence_refs
        .iter()
        .any(|reference| reference == RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS));
    assert!(artifacts
        .negative_cases
        .iter()
        .any(|case| case.case_id == "provider_scheduler_mismatch"));
    assert!(artifacts.negative_cases.iter().any(|case| {
        case.case_id == "failed_tick_recoverable_cycle"
            && case.retained_evidence_ref == "runtime_v2/recovery/safe_resume_decision.json"
    }));
}

#[test]
fn runtime_v2_unified_runtime_kernel_rejects_missing_participant_or_negative_case() {
    let mut artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");
    artifacts
        .summary
        .participants
        .retain(|participant| participant.participant_id != "acip_boundary");
    assert!(artifacts
        .validate()
        .expect_err("missing ACIP boundary participant should fail")
        .to_string()
        .contains("acip_boundary"));

    let mut artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");
    artifacts
        .negative_cases
        .retain(|case| case.case_id != "stop_request");
    assert!(artifacts
        .validate()
        .expect_err("missing stop request negative case should fail")
        .to_string()
        .contains("stop_request"));
}

#[test]
fn runtime_v2_unified_runtime_kernel_events_are_correlated() {
    let artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");

    for participant in &artifacts.summary.participants {
        let event = artifacts
            .events
            .iter()
            .find(|event| event.participant_id == participant.participant_id)
            .expect("participant event");
        assert_eq!(event.correlation_id, participant.correlation_id);
        assert_eq!(event.artifact_ref, participant.artifact_refs[0]);
    }
}

#[test]
fn runtime_v2_unified_runtime_kernel_rejects_duplicate_event_participant() {
    let mut artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");
    let daemon_event = artifacts
        .events
        .iter()
        .find(|event| event.participant_id == "daemon_tick")
        .expect("daemon event")
        .clone();
    let external_signal_index = artifacts
        .events
        .iter()
        .position(|event| event.participant_id == "external_signals")
        .expect("external signal event");
    artifacts.events[external_signal_index] = RuntimeV2UnifiedKernelEvent {
        sequence: artifacts.events[external_signal_index].sequence,
        ..daemon_event
    };

    assert!(artifacts
        .validate()
        .expect_err("duplicate daemon event should fail")
        .to_string()
        .contains("duplicate participant"));
}

#[test]
fn runtime_v2_unified_runtime_kernel_rejects_unretained_negative_evidence() {
    let mut artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");
    let case = artifacts
        .negative_cases
        .iter_mut()
        .find(|case| case.case_id == "failed_tick_recoverable_cycle")
        .expect("failed tick case");
    case.retained_evidence_ref = "runtime_v2/recovery/not_retained.json".to_string();

    assert!(artifacts
        .validate()
        .expect_err("unretained negative evidence should fail")
        .to_string()
        .contains("not retained"));
}

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
#[test]
fn runtime_v2_unified_runtime_kernel_writes_retained_evidence() {
    let temp_root = unique_temp_path("unified-runtime-kernel");
    let artifacts =
        runtime_v2_unified_runtime_kernel_contract().expect("unified runtime kernel artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write unified runtime kernel artifacts");

    assert!(temp_root
        .join(RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY)
        .is_file());
    assert!(temp_root
        .join(RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_EVENTS)
        .is_file());
    assert!(temp_root
        .join(RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_NEGATIVE_CASES)
        .is_file());
    assert!(temp_root
        .join("issue_5097/current_runtime/run_status.json")
        .is_file());
    assert!(temp_root
        .join(RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_ACK)
        .is_file());

    let summary_text =
        fs::read_to_string(temp_root.join(RUNTIME_V2_UNIFIED_RUNTIME_KERNEL_SUMMARY))
            .expect("summary text");
    assert!(!summary_text.contains(temp_root.to_string_lossy().as_ref()));
    assert!(summary_text.contains("runtime_v2.unified_runtime_kernel.v1"));

    fs::remove_dir_all(temp_root).ok();
}
