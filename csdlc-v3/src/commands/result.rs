//! Additive public envelope: legacy owner payloads remain intact for consumers.
//! Missing outcome evidence is unknown, never a claim that no effect occurred.
use super::contract::{descriptor, RESULT_SCHEMA};
use serde_json::{json, Value};

pub struct Invocation {
    pub command: String,
    pub request: Value,
    pub execute: bool,
    pub correlation: String,
    pub started_unix_ms: u128,
    pub elapsed_ms: u128,
}

fn select<'a>(value: &'a Value, paths: &[&str]) -> Option<&'a Value> {
    paths
        .iter()
        .find_map(|path| value.pointer(path).filter(|v| !v.is_null()))
}

/// Attach one common versioned contract without renaming any owner payload field.
/// Owner status and process exit remain separate: a legacy ready/blocked report
/// may intentionally use exit zero, and a mutation error can leave partial effects.
pub fn envelope(mut payload: Value, invocation: &Invocation, failed: bool) -> Value {
    if !payload.is_object() {
        payload = json!({"findings": payload});
    }
    let command = invocation.command.as_str();
    let row = descriptor(command);
    let mut effect_class = row
        .and_then(|r| r["effect_class"].as_str())
        .unwrap_or("observation");
    let mut findings = select(
        &payload,
        &[
            "/findings",
            "/result/findings",
            "/result/outcome/result/findings",
        ],
    )
    .cloned()
    .unwrap_or_else(|| json!([]));
    if payload.get("code").is_some() {
        findings = json!([payload.clone()]);
    }
    let mut code = findings
        .as_array()
        .and_then(|f| f.iter().find_map(|f| f["code"].as_str()))
        .unwrap_or(if failed {
            "command_failed"
        } else {
            "command_completed"
        })
        .to_owned();
    let raw_status = select(
        &payload,
        &[
            "/status",
            "/result/status",
            "/result/routing/state",
            "/route_status/status",
            "/result/outcome/result/status",
        ],
    )
    .and_then(Value::as_str)
    .or_else(|| {
        findings.as_array().and_then(|rows| {
            if rows.iter().any(|row| row["status"] == "failed") {
                Some("failed")
            } else if rows.iter().any(|row| row["status"] == "blocked") {
                Some("blocked")
            } else {
                None
            }
        })
    });
    let recovery = findings.as_array().is_some_and(|f| {
        f.iter().any(|f| {
            f["code"].as_str().is_some_and(|c| {
                c.contains("recovery_required")
                    || c.contains("reconciliation_required")
                    || c.contains("reconciliation_pending")
            })
        })
    });
    let noop = select(&payload, &["/result/cleanup/decision"])
        .and_then(Value::as_str)
        .is_some_and(|decision| matches!(decision, "already_removed" | "absent"));
    let mut status = if recovery {
        "recovery_required"
    } else if raw_status == Some("failed") {
        "failed"
    } else if raw_status == Some("blocked") || failed && payload.get("code").is_some() {
        "blocked"
    } else if failed {
        "failed"
    } else if noop {
        "expected_noop"
    } else {
        match raw_status.unwrap_or("completed") {
            "operator_required" | "repair_required" => "blocked",
            "waiting" => "deferred",
            "retryable" => "failed",
            status => status,
        }
    };
    let mutation = select(
        &payload,
        &["/performed_mutation", "/writes_v3_state", "/result/mutated"],
    )
    .and_then(Value::as_bool);
    let remote_receipt = payload.pointer("/result/outcome/result/receipt").is_some();
    let effects = if mutation == Some(true) || remote_receipt {
        "performed"
    } else if mutation == Some(false) || effect_class == "observation" {
        "none"
    } else if failed {
        "unknown"
    } else if payload["read_only"] == true {
        "none"
    } else {
        "unknown"
    };
    if effects == "performed" && effect_class == "observation" {
        effect_class = "undeclared_mutation";
        status = "failed";
        code = "effect_contract_violation".into();
        findings.as_array_mut().expect("typed findings array").push(json!({
            "code":"effect_contract_violation", "message":"owner reported a mutation for an observational descriptor"
        }));
    }
    let issue = select(
        &payload,
        &[
            "/issue",
            "/request_issue",
            "/result/issue",
            "/result/outcome/result/issue",
            "/result/outcome/result/receipt/issue",
        ],
    )
    .or_else(|| select(&invocation.request, &["/issue", "/operation/request/issue"]));
    let issue_identity = match issue.and_then(Value::as_u64).filter(|issue| *issue > 0) {
        Some(number) => json!({"status":"identified", "number":number}),
        None if matches!(
            command,
            "foundation" | "remote" | "release-preflight" | "sprint"
        ) =>
        {
            json!({"status":"not_applicable"})
        }
        None => json!({"status":"unavailable"}),
    };
    let after = select(
        &payload,
        &["/result/digest", "/result/lifecycle_state/digest"],
    );
    let before = if row.is_some_and(|r| r["family"] == "local") {
        payload
            .get("request_expected_lifecycle_digest")
            .filter(|value| value.is_string())
            .or_else(|| {
                invocation
                    .request
                    .get("expected_lifecycle_digest")
                    .filter(|value| value.is_string())
            })
    } else {
        None
    };
    let next = if let Some(next) = payload
        .pointer("/result/next_route")
        .and_then(Value::as_str)
    {
        json!([next])
    } else {
        select(
            &payload,
            &[
                "/result/routing/eligible_operations",
                "/result/eligible_operations",
            ],
        )
        .cloned()
        .unwrap_or_else(|| json!([]))
    };
    let authority = if payload["operational_authority"] == true {
        "verified"
    } else if failed {
        "not_established"
    } else {
        "non_operational"
    };
    payload["envelope"] = json!({
        "schema": RESULT_SCHEMA, "command": command, "issue": issue_identity,
        "status":status, "process_status":if failed {"failed"} else {"succeeded"},
        "authority_status":authority,
        "issue_version":{"before":{"basis":"request_expectation", "digest":before}, "after":{"basis":"owner_result", "digest":after}},
        "effects":{"class":effect_class,"outcome":effects},
        "findings":findings,"reason_code":code,"allowed_next_operations":next,
        "correlation_id":invocation.correlation,
        "intent_identity":select(&payload,&["/result/outcome/result/receipt/operation_digest"]),
        "evidence_invalidation": {"status":"not_reported"},
        "timing":{"started_unix_ms":invocation.started_unix_ms,"monotonic_duration_ms":invocation.elapsed_ms},
        "wait_owner":if matches!(status,"blocked"|"recovery_required"|"failed"|"deferred") {"operator"} else {"none"}
    });
    payload
}
