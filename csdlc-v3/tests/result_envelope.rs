//! PVF: deterministic, small CPU-only contract regression proof for #868/SIM-07.
//! These owner-result fixtures test classification invariants; installed journeys
//! separately prove that production commands produce the claimed owner results.
use csdlc_v3::commands::result::{envelope, Invocation};
use serde_json::{json, Value};

fn invocation(command: &str) -> Invocation {
    Invocation {
        command: command.into(),
        request: json!({"issue":868,"expected_lifecycle_digest":"before"}),
        execute: true,
        correlation: "fixture-correlation".into(),
        started_unix_ms: 100,
        elapsed_ms: 7,
    }
}

#[test]
fn blocked_owner_results_are_not_completed_or_reclassified_by_exit_status() {
    let guard = envelope(
        json!([{"status":"blocked","code":"authority_stale","message":"stale"}]),
        &invocation("bind"),
        true,
    );
    assert_eq!(guard["envelope"]["status"], "blocked");
    assert_eq!(guard["envelope"]["process_status"], "failed");
    for result in [
        json!({"routing":{"state":"blocked","eligible_operations":[]},"findings":[]}),
        json!({"findings":[{"status":"blocked","code":"cards_missing"}]}),
    ] {
        let output = envelope(
            json!({"read_only":true,"operational_authority":true,"result":result}),
            &invocation("doctor"),
            false,
        );
        assert_eq!(output["envelope"]["status"], "blocked");
        assert_eq!(output["envelope"]["effects"]["outcome"], "none");
    }
}

#[test]
fn pure_review_and_publication_admission_do_not_claim_persisted_effects() {
    for command in ["review", "publish"] {
        for status in ["ready", "blocked"] {
            let owner = json!({"schema":"csdlc.v3.operational_remote.v1",
                "read_only":true,"operational_authority":true,
                "result":{"outcome":{"kind":command,"result":{"status":status,"findings":[]}}}});
            let output = envelope(owner.clone(), &invocation(command), false);
            assert_eq!(output["envelope"]["status"], status);
            assert_eq!(output["envelope"]["effects"]["outcome"], "none");
            assert_eq!(output["result"], owner["result"]);
            assert_eq!(output["schema"], owner["schema"]);
        }
    }
}

#[test]
fn failed_and_uncertain_operations_preserve_effect_evidence() {
    let partial = envelope(
        json!({"performed_mutation":true,"status":"failed","findings":[{"code":"after_write_failure"}]}),
        &invocation("edit"),
        true,
    );
    assert_eq!(partial["envelope"]["status"], "failed");
    assert_eq!(partial["envelope"]["effects"]["outcome"], "performed");
    let uncertain = envelope(
        json!({"findings":[{"code":"remote_reconciliation_required"}]}),
        &invocation("github-pr"),
        true,
    );
    assert_eq!(uncertain["envelope"]["status"], "recovery_required");
    assert_eq!(uncertain["envelope"]["effects"]["outcome"], "unknown");
    assert_eq!(uncertain["envelope"]["wait_owner"], "operator");
}

#[test]
fn expected_noop_and_missing_identity_remain_explicit() {
    let mut request = invocation("clean");
    request.request = Value::Null;
    let output = envelope(
        json!({"performed_mutation":false,"result":{"cleanup":{"decision":"already_removed"}}}),
        &request,
        false,
    );
    assert_eq!(output["envelope"]["status"], "expected_noop");
    assert_eq!(output["envelope"]["effects"]["outcome"], "none");
    assert_eq!(output["envelope"]["issue"]["status"], "unavailable");
    assert!(output["envelope"]["issue_version"]["after"]["digest"].is_null());
    assert_eq!(output["envelope"]["correlation_id"], "fixture-correlation");
    assert_eq!(output["envelope"]["timing"]["monotonic_duration_ms"], 7);
}

#[test]
fn unvalidated_request_fields_do_not_become_asserted_identity_or_version() {
    for issue in [json!(0), json!(-1), json!("868"), json!({"number":868})] {
        for digest in [json!(42), json!({"digest":"fake"}), json!(["fake"])] {
            let mut call = invocation("edit");
            call.request = json!({"issue":issue,"expected_lifecycle_digest":digest});
            let output = envelope(
                json!({"status":"failed","performed_mutation":false,
                    "findings":[{"code":"typed_contract_invalid_json"}]}),
                &call,
                true,
            );
            assert_eq!(output["envelope"]["issue"]["status"], "unavailable");
            assert!(output["envelope"]["issue"].get("number").is_none());
            assert!(output["envelope"]["issue_version"]["before"]["digest"].is_null());
            assert_eq!(output["envelope"]["status"], "failed");
            assert_eq!(output["envelope"]["effects"]["outcome"], "none");
        }
    }
}

#[test]
fn an_observation_descriptor_cannot_hide_an_owner_reported_mutation() {
    let output = envelope(
        json!({"performed_mutation":true,"status":"ready","findings":[]}),
        &invocation("doctor"),
        false,
    );
    assert_eq!(output["envelope"]["status"], "failed");
    assert_eq!(output["envelope"]["effects"]["outcome"], "performed");
    assert_eq!(
        output["envelope"]["effects"]["class"],
        "undeclared_mutation"
    );
    assert_eq!(
        output["envelope"]["reason_code"],
        "effect_contract_violation"
    );
    assert!(output["envelope"]["findings"]
        .as_array()
        .unwrap()
        .iter()
        .any(|finding| finding["code"] == "effect_contract_violation"));
}
