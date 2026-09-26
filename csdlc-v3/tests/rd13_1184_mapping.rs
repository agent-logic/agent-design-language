//! PVF: planning_contract, deterministic offline evidence consistency, bounded CPU/files.
//! Required RD13 gate: six authenticated captured mappings and three negative families.
//! This does not perform transfers or claim current GitHub or destination install proof.
use serde_json::{json, Value};
use std::collections::BTreeSet;

const PACKET: &str =
    include_str!("../../docs/milestones/v0.93.1/repository-decomposition/rd13-1184/issue-map.json");
const EXPECTED: [(u64, &str, u64, &[u64]); 6] = [
    (1186, "cognitive-sdlc", 1, &[1184]),
    (1187, "agent-logic-runtime", 1, &[1185, 1186]),
    (1188, "agent-logic-infrastructure", 1, &[1187]),
    (1189, "agent-logic-enterprise-security", 1, &[1187]),
    (1190, "codefriend", 1, &[1187]),
    (1191, "agent-logic-runtime", 2, &[1188, 1189, 1190]),
];
fn old_url(n: u64) -> String {
    format!("https://github.com/agent-logic/agent-design-language/issues/{n}")
}
fn resolved(n: u64) -> String {
    EXPECTED.iter().find(|x| x.0 == n).map_or_else(
        || old_url(n),
        |x| format!("https://github.com/agent-logic/{}/issues/{}", x.1, x.2),
    )
}
fn check(v: &Value) -> Result<(), &'static str> {
    if v["schema"] != "rd13.transferred_issue_map.v1"
        || v["rd12_merge"] != "a59d3048538f4a4408b4182e0cd6c0a804a6e46b"
    {
        return Err("predecessor");
    }
    let rows = v["issues"].as_array().ok_or("issues")?;
    if rows.len() != EXPECTED.len() {
        return Err("denominator");
    }
    let mut ids = BTreeSet::new();
    let mut urls = BTreeSet::new();
    for (n, repo, number, deps) in EXPECTED {
        let matched: Vec<_> = rows.iter().filter(|x| x["source"]["number"] == n).collect();
        if matched.len() != 1 {
            return Err("duplicate source");
        }
        let row = matched[0];
        let source = &row["source"];
        let dest = &row["destination"];
        let url = dest["html_url"].as_str().ok_or("destination url")?;
        let id = dest["node_id"].as_str().ok_or("destination node")?;
        if id.is_empty() || !ids.insert(id) || !urls.insert(url) {
            return Err("duplicate destination");
        }
        if row["destination_repository"] != format!("agent-logic/{repo}")
            || dest["number"] != number
            || url != resolved(n)
            || source["html_url"] != old_url(n)
            || row["source_redirect_url"] != url
        {
            return Err("wrong repository or redirect");
        }
        if source["title"] != dest["title"]
            || source["created_at"] != dest["created_at"]
            || source["state"] != "open"
            || dest["state"] != "open"
            || source["labels"] != dest["labels"]
            || dest["labels"] != json!(["version:v0.93.1"])
            || dest["milestone_title"] != "v0.93.1"
            || source["assignees"] != dest["assignees"]
            || row["source_comment_ids"] != row["destination_comment_ids"]
            || row["transfer_event"]["event"] != "transferred"
            || row["source_journal_unchanged"] != true
            || row["source_journal"]["issue"] != n
        {
            return Err("history or metadata");
        }
        let body = dest["body"].as_str().ok_or("body")?;
        if !body.contains("Worker #1.") || !body.contains(&old_url(1180)) {
            return Err("session or parent reference");
        }
        let links = row["dependencies"].as_array().ok_or("dependencies")?;
        if links.len() != deps.len() {
            return Err("missing dependency");
        }
        for dep in deps {
            let matches: Vec<_> = links
                .iter()
                .filter(|x| x["source_url"] == old_url(*dep))
                .collect();
            if matches.len() != 1
                || matches[0]["resolved_url"] != resolved(*dep)
                || !(body.contains(&old_url(*dep)) || body.contains(&resolved(*dep)))
            {
                return Err("missing dependency");
            }
        }
        if row["parent"]["resolved_url"] != old_url(1180)
            || row["destination_native_lifecycle"]
                != "not_initialized; portable installation belongs to RD04"
        {
            return Err("scope boundary");
        }
    }
    let incoming = v["incoming_dependencies"].as_array().ok_or("incoming")?;
    if incoming.len() != 1
        || incoming[0]["consumer"] != old_url(1192)
        || incoming[0]["source_dependency"] != old_url(1191)
        || incoming[0]["resolved_dependency"] != resolved(1191)
    {
        return Err("incoming dependency");
    }
    Ok(())
}
#[test]
fn six_transferred_identities_preserve_history_and_dependencies() {
    check(&serde_json::from_str(PACKET).unwrap()).unwrap();
}
#[test]
fn duplicate_identity_is_rejected() {
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["issues"][1]["destination"]["node_id"] = v["issues"][0]["destination"]["node_id"].clone();
    assert_eq!(check(&v), Err("duplicate destination"));
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["issues"][1]["source"]["number"] = json!(1186);
    assert_eq!(check(&v), Err("duplicate source"));
}
#[test]
fn missing_or_wrong_dependency_is_rejected() {
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["issues"][1]["dependencies"] = json!([]);
    assert_eq!(check(&v), Err("missing dependency"));
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["incoming_dependencies"][0]["resolved_dependency"] = json!(old_url(1191));
    assert_eq!(check(&v), Err("incoming dependency"));
}
#[test]
fn wrong_repository_and_lost_metadata_are_rejected() {
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["issues"][0]["destination_repository"] = json!("agent-logic/agent-design-language");
    assert_eq!(check(&v), Err("wrong repository or redirect"));
    let mut v: Value = serde_json::from_str(PACKET).unwrap();
    v["issues"][0]["destination"]["milestone_title"] = Value::Null;
    assert_eq!(check(&v), Err("history or metadata"));
}
