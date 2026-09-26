//! PVF: deterministic offline bootstrap-evidence contract, bounded local files/CPU.
//! Required RD12 gate. Five captured positive repositories and three adverse case
//! families; this is not a live API probe, product CI, or installed-owner proof.
use serde_json::{json, Value};

const PACKET: &str = include_str!(
    "../../docs/milestones/v0.93.1/repository-decomposition/rd12-1183/bootstrap-readbacks.json"
);
const NAMES: [&str; 5] = [
    "agent-logic/cognitive-sdlc",
    "agent-logic/agent-logic-runtime",
    "agent-logic/codefriend",
    "agent-logic/agent-logic-infrastructure",
    "agent-logic/agent-logic-enterprise-security",
];

fn check(packet: &Value) -> Result<(), &'static str> {
    if packet["authenticated_actor"] != "danielbaustin"
        || packet["organization_default_repository_permission"] != "read"
    {
        return Err("access baseline");
    }
    let rows = packet["repositories"].as_array().ok_or("repositories")?;
    if rows.len() != NAMES.len() {
        return Err("denominator");
    }
    for name in NAMES {
        let matches: Vec<_> = rows.iter().filter(|r| r["full_name"] == name).collect();
        if matches.len() != 1 {
            return Err("unapproved or missing identity");
        }
        let row = matches[0];
        if row["id"].as_u64().is_none_or(|id| id == 0) {
            return Err("missing repository id");
        }
        if row["private"] != true || row["archived"] != false {
            return Err("visibility");
        }
        if row["owner"]["login"] != "danielbaustin" || row["owner"]["permission"] != "admin" {
            return Err("owner access");
        }
        let sha = row["main"]["sha"].as_str().ok_or("main sha")?;
        if sha.len() != 40 || !sha.bytes().all(|c| c.is_ascii_hexdigit()) {
            return Err("main sha");
        }
        if row["default_branch"] != "main"
            || row["main"]["name"] != "main"
            || row["main"]["protected"] != true
            || row["has_issues"] != true
            || row["allow_merge_commit"] != true
            || row["allow_auto_merge"] != false
            || row["delete_branch_on_merge"] != false
        {
            return Err("branch or repository settings");
        }
        let ruleset = &row["ruleset"];
        if ruleset["target"] != "branch"
            || ruleset["enforcement"] != "active"
            || ruleset["bypass_actors"] != json!([])
            || ruleset["conditions"]["ref_name"]
                != json!({"include":["~DEFAULT_BRANCH"],"exclude":[]})
        {
            return Err("ruleset settings");
        }
        for rules in [&ruleset["rules"], &row["effective_rules"]] {
            let rules = rules.as_array().ok_or("rules")?;
            for kind in ["deletion", "non_fast_forward", "pull_request"] {
                if !rules.iter().any(|r| r["type"] == kind) {
                    return Err("missing protection");
                }
            }
            let pr = rules.iter().find(|r| r["type"] == "pull_request").unwrap();
            if pr["parameters"]["required_approving_review_count"] != 0 {
                return Err("approval baseline");
            }
            if rules.iter().any(|r| r["type"] == "required_status_checks") {
                return Err("inert required checks");
            }
        }
        if row["workflow_permissions"]["default_workflow_permissions"] != "read"
            || row["workflow_permissions"]["can_approve_pull_request_reviews"] != false
            || row["workflow_count"] != 0
            || row["native_csdlc_authority"] != "not_installed"
        {
            return Err("bootstrap qualification boundary");
        }
    }
    Ok(())
}

#[test]
fn five_captured_repository_bootstraps_match_approved_policy() {
    let packet: Value = serde_json::from_str(PACKET).unwrap();
    assert_eq!(check(&packet), Ok(()));
}

#[test]
fn wrong_visibility_is_rejected_for_each_destination() {
    for i in 0..5 {
        let mut packet: Value = serde_json::from_str(PACKET).unwrap();
        packet["repositories"][i]["private"] = json!(false);
        assert_eq!(check(&packet), Err("visibility"));
    }
}

#[test]
fn unapproved_identity_is_rejected_for_each_destination() {
    for i in 0..5 {
        let mut packet: Value = serde_json::from_str(PACKET).unwrap();
        packet["repositories"][i]["full_name"] = json!("agent-logic/unapproved");
        assert_eq!(check(&packet), Err("unapproved or missing identity"));
    }
}

#[test]
fn missing_owner_access_is_rejected_for_each_destination() {
    for i in 0..5 {
        let mut packet: Value = serde_json::from_str(PACKET).unwrap();
        packet["repositories"][i]["owner"]["permission"] = json!("read");
        assert_eq!(check(&packet), Err("owner access"));
    }
}
