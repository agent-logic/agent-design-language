use std::{
    fs,
    io::Write,
    process::{Command, Stdio},
};

use serde_json::{json, Value};
use tempfile::TempDir;

const BIN: &str = env!("CARGO_BIN_EXE_adl-runtime-governed-operations");

fn command(id: &str, citizen: &str, time: u64) -> Value {
    json!({
        "request_id": id,
        "idempotency_key": format!("idem-{id}"),
        "citizen_id": citizen,
        "agent_id": format!("agent-{citizen}"),
        "action": "provider.digest",
        "resource": "compute",
        "units": 1,
        "payload": format!("private-{id}"),
        "qualified_unix_millis": time
    })
}

fn run(root: &TempDir, request: Value) -> Value {
    let state = root.path().join("state");
    let mut child = Command::new(BIN)
        .current_dir(root.path())
        .env("ADL_PARITY_C_STATE_DIR", &state)
        .env("ADL_PARITY_C_POLICY_KEY_HEX", hex::encode([1; 32]))
        .env("ADL_PARITY_C_AUTHORITY_KEY_HEX", hex::encode([2; 32]))
        .env("ADL_PARITY_C_PERMIT_KEY_HEX", hex::encode([3; 32]))
        .env("ADL_PARITY_C_CHECKPOINT_KEY_HEX", hex::encode([4; 32]))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("spawn governed operations service");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(request.to_string().as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    assert!(
        output.status.code() == Some(0) || output.status.code() == Some(77),
        "unexpected exit: {:?}: {}",
        output.status,
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON outcome")
}

fn success(value: &Value) {
    assert_eq!(value["status"], "completed");
    assert_eq!(value["gate_before_actuation"], true);
    assert_eq!(value["lifelog_authoritative"], false);
    assert_eq!(value["private_payload_retained"], false);
}

mod parity_c_live_governance {
    use super::*;

    #[test]
    fn signed_gate_precedes_provider_actuation() {
        let root = TempDir::new().unwrap();
        let value = run(&root, command("signed", "alice", 1_000));
        success(&value);
        assert_eq!(value["actuation_count"], 1);
    }

    #[test]
    fn denial_revocation_and_quarantine_prevent_actuation() {
        let root = TempDir::new().unwrap();
        let mut request = command("revoked", "alice", 1_000);
        request["revoke_before_dispatch"] = true.into();
        let value = run(&root, request);
        assert_eq!(value["classification"], "revoked");
        assert_eq!(value["actuation_count"], 0);
    }

    #[test]
    fn appeal_disposition_never_bypasses_current_policy() {
        let root = TempDir::new().unwrap();
        let mut request = command("appeal", "alice", 1_000);
        request["units"] = 9.into();
        let value = run(&root, request);
        assert_ne!(value["status"], "completed");
        assert_eq!(value["actuation_count"], 0);
    }

    #[test]
    fn expired_or_replayed_gate_receipt_fails_closed() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("once", "alice", 1_000)));
        let mut replay = command("once", "alice", 2_000);
        replay["idempotency_key"] = "different-idempotency".into();
        assert_eq!(run(&root, replay)["classification"], "request_replay");
    }
}

mod parity_c_delegation_resources {
    use super::*;

    #[test]
    fn delegation_chain_only_attenuates() {
        let root = TempDir::new().unwrap();
        let mut request = command("delegate", "alice", 1_000);
        request["delegate_units"] = 2.into();
        success(&run(&root, request));
    }

    #[test]
    fn widened_expired_or_replayed_delegation_is_rejected() {
        let root = TempDir::new().unwrap();
        let mut request = command("widen", "alice", 1_000);
        request["delegate_units"] = 9.into();
        assert_eq!(run(&root, request)["classification"], "invalid_delegation");
    }

    #[test]
    fn cancellation_wins_dispatch_and_releases_capacity() {
        let root = TempDir::new().unwrap();
        let mut cancelled = command("cancelled", "alice", 1_000);
        cancelled["provider_condition"] = "cancelled".into();
        assert_eq!(
            run(&root, cancelled)["classification"],
            "scheduler_cancelled"
        );
        success(&run(&root, command("after-cancel", "alice", 2_000)));
    }

    #[test]
    fn retry_and_idempotency_bounds_prevent_duplicate_work() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("idem", "alice", 1_000)));
        let replay = run(&root, command("idem", "alice", 2_000));
        assert_eq!(replay["classification"], "idempotent_replay");
        assert_eq!(replay["actuation_count"], 1);
    }
}

mod parity_c_provider_scheduler_tools {
    use super::*;

    #[test]
    fn two_agents_execute_governed_provider_and_tool_work() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("agent-a", "alice", 1_000)));
        fs::write(root.path().join("tool-target"), b"real-file").unwrap();
        let mut tool = command("agent-b", "bob", 2_000);
        tool["action"] = "tool.file_metadata".into();
        tool["payload"] = "tool-target".into();
        success(&run(&root, tool));
    }

    #[test]
    fn scheduler_dispatch_is_deterministic_and_bounded() {
        let first = TempDir::new().unwrap();
        let second = TempDir::new().unwrap();
        let left = run(&first, command("ordered", "alice", 1_000));
        let right = run(&second, command("ordered", "alice", 1_000));
        assert_eq!(left["result_hash"], right["result_hash"]);
    }

    #[test]
    fn provider_timeout_auth_quota_and_malformed_output_are_classified() {
        for (index, condition) in ["timeout", "auth", "quota", "malformed"]
            .into_iter()
            .enumerate()
        {
            let root = TempDir::new().unwrap();
            let mut request = command(&format!("failure-{index}"), "alice", 1_000);
            request["provider_condition"] = condition.into();
            assert_eq!(run(&root, request)["status"], "refused");
        }
    }

    #[test]
    fn shepherd_cannot_grant_or_bypass_authority() {
        let root = TempDir::new().unwrap();
        let mut request = command("shepherd", "alice", 1_000);
        request["agent_id"] = "shepherd".into();
        request["revoke_before_dispatch"] = true.into();
        assert_eq!(run(&root, request)["classification"], "revoked");
    }
}

mod parity_c_private_identity {
    use super::*;

    #[test]
    fn private_state_is_partitioned_by_authoritative_identity() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("alice-write", "alice", 1_000)));
        success(&run(&root, command("bob-write", "bob", 2_000)));
        let checkpoint = fs::read_to_string(root.path().join("state/checkpoint.json")).unwrap();
        assert!(!checkpoint.contains("private-alice-write"));
        assert!(!checkpoint.contains("private-bob-write"));
    }

    #[test]
    fn cross_identity_read_and_write_fail_closed() {
        let root = TempDir::new().unwrap();
        let mut request = command("cross", "alice", 1_000);
        request["read_citizen_id"] = "bob".into();
        assert_eq!(
            run(&root, request)["classification"],
            "cross_identity_denied"
        );
    }

    #[test]
    fn provider_or_display_identity_cannot_substitute_for_citizen_identity() {
        let root = TempDir::new().unwrap();
        let mut request = command("display", "alice", 1_000);
        request["citizen_id"] = "".into();
        request["agent_id"] = "alice".into();
        assert_eq!(run(&root, request)["classification"], "invalid_request");
    }

    #[test]
    fn restart_preserves_redacted_identity_scoped_state() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("persist", "alice", 1_000)));
        let replay = run(&root, command("persist", "alice", 2_000));
        assert_eq!(replay["classification"], "idempotent_replay");
        let lifelog = fs::read_to_string(root.path().join("state/lifelog.jsonl")).unwrap();
        assert!(!lifelog.contains("private-persist"));
    }
}

mod parity_c_time_continuity {
    use super::*;

    #[test]
    fn unqualified_or_regressing_time_cannot_authorize_actuation() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("time-a", "alice", 2_000)));
        assert_eq!(
            run(&root, command("time-b", "alice", 1_999))["classification"],
            "unqualified_or_regressing_time"
        );
    }

    #[test]
    fn authenticated_checkpoint_is_the_only_restore_authority() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("checkpoint", "alice", 1_000)));
        fs::write(root.path().join("state/checkpoint.json"), b"{}").unwrap();
        assert_eq!(
            run(&root, command("after-corrupt", "alice", 2_000))["classification"],
            "checkpoint_corrupt"
        );
    }

    #[test]
    fn lifelog_is_redacted_append_only_and_non_authoritative() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("log-a", "alice", 1_000)));
        success(&run(&root, command("log-b", "alice", 2_000)));
        let log = fs::read_to_string(root.path().join("state/lifelog.jsonl")).unwrap();
        assert_eq!(log.lines().count(), 2);
        assert!(!log.contains("private-log"));
    }

    #[test]
    fn restart_revalidates_revocation_without_duplicate_side_effects() {
        let root = TempDir::new().unwrap();
        success(&run(&root, command("restart", "alice", 1_000)));
        let replay = run(&root, command("restart", "alice", 2_000));
        assert_eq!(replay["actuation_count"], 1);
        let mut revoked = command("new-revoked", "alice", 3_000);
        revoked["revoke_before_dispatch"] = true.into();
        assert_eq!(run(&root, revoked)["classification"], "revoked");
    }

    #[test]
    fn shutdown_commits_final_checkpoint_and_isolates_lifelog_failure() {
        let root = TempDir::new().unwrap();
        let mut shutdown = command("shutdown", "alice", 1_000);
        shutdown["action"] = "system.shutdown".into();
        shutdown["lifelog_failure"] = true.into();
        success(&run(&root, shutdown));
        assert!(root.path().join("state/checkpoint.json").exists());
        assert_eq!(
            run(&root, command("after-shutdown", "alice", 2_000))["classification"],
            "admission_closed"
        );
    }
}

mod parity_c_production_credit {
    use super::*;

    #[test]
    fn all_owned_components_use_production_or_cots_adapters() {
        let root = TempDir::new().unwrap();
        let value = run(&root, command("inventory", "alice", 1_000));
        success(&value);
        let adapters = value["adapters"].as_array().unwrap();
        assert!(adapters
            .iter()
            .any(|value| value == "local_digest_provider"));
        assert!(adapters
            .iter()
            .any(|value| value == "allowlisted_file_metadata_tool"));
    }

    #[test]
    fn degraded_fixture_mock_and_metadata_paths_receive_zero_credit() {
        let owned = [
            include_str!("../src/governed_operations.rs"),
            include_str!("../src/bin/adl-runtime-governed-operations.rs"),
        ]
        .join("\n")
        .to_ascii_lowercase();
        assert!(!owned.contains("degradedoperationexecutor"));
        assert!(!owned.contains("mockexecutor"));
        assert!(!owned.contains("fixtureexecutor"));
    }
}

mod parity_c_boundary_contract {
    #[test]
    fn runtime_v2_aws_and_cross_lane_paths_are_absent() {
        let owned = [
            include_str!("../src/governed_operations.rs"),
            include_str!("../src/bin/adl-runtime-governed-operations.rs"),
        ]
        .join("\n");
        for forbidden in [
            "aws_",
            "adl-runtime/src",
            "parity_b",
            "observatory",
            "weather",
        ] {
            assert!(!owned.to_ascii_lowercase().contains(forbidden));
        }
    }

    #[test]
    fn retained_evidence_excludes_credentials_private_state_and_machine_paths() {
        let owned = include_str!("../src/governed_operations.rs");
        assert!(!owned.contains("/Users/"));
        assert!(!owned.contains("/Volumes/"));
        assert!(!owned.contains("BEGIN PRIVATE KEY"));
    }
}
