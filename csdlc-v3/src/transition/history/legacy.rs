//! Read-only admission of retained v2 terminal evidence; never a v2 lifecycle route.
use super::*;

pub(super) struct LegacyTerminal {
    pub(super) pull_request: u64,
    pub(super) head_sha: String,
    pub(super) receipt_digest: String,
    /// Paths relative to `.csdlc/issues/ISSUE`, present and byte-matched here.
    pub(super) authored_paths: BTreeSet<String>,
}

// Retained v2 receipts were serialized from ordered structs with their own
// digest cleared. preserve_order deliberately retains that historical order.
// Reordered or unknown encodings fail closed rather than gaining new authority.
fn verify_digest(value: &Value) -> Result<()> {
    let digest = value["digest"].as_str().ok_or("legacy digest missing")?;
    let mut clear = value.clone();
    clear["digest"] = Value::String(String::new());
    if digest
        != blake3::hash(&serde_json::to_vec(&clear).map_err(err)?)
            .to_hex()
            .as_str()
    {
        return Err("retained legacy digest mismatch".into());
    }
    Ok(())
}

fn normalized_terminal(value: &Value) -> Result<Value> {
    let mut normalized = value.clone();
    let object = normalized
        .as_object_mut()
        .ok_or("legacy terminal object required")?;
    for (old, current) in [
        ("released_branch", "branch"),
        ("released_worktree", "worktree"),
    ] {
        if object.contains_key(old) {
            if object.contains_key(current) {
                return Err("ambiguous legacy terminal topology".into());
            }
            let v = object.remove(old).ok_or("legacy topology missing")?;
            object.insert(current.into(), v);
        }
    }
    if let Some(paths) = object.remove("released_protected_paths") {
        if !paths.as_array().is_some_and(|rows| {
            rows.iter().all(|v| {
                v.as_str().is_some_and(|s| {
                    !s.is_empty()
                        && Path::new(s)
                            .components()
                            .all(|c| matches!(c, std::path::Component::Normal(_)))
                })
            })
        }) {
            return Err("invalid historical released paths".into());
        }
    }
    if object.keys().any(|k| {
        ![
            "pull_request",
            "disposition",
            "observed_sha",
            "observed_state",
            "receipt_path",
            "branch",
            "worktree",
        ]
        .contains(&k.as_str())
    }) {
        return Err("unknown legacy terminal field".into());
    }
    Ok(normalized)
}

fn evidence(issue: u64, repository: &str, index: &Value, receipt: &Value) -> Result<(u64, String)> {
    verify_digest(receipt)?;
    let record = &receipt["record"];
    verify_digest(record)?;
    let reference = format!("csdlc-v2/closeout/{issue}.json");
    if issue == 0
        || receipt["schema"] != "csdlc.terminal_receipt.v1"
        || record["schema"] != "csdlc.issue.index.v1"
        || index["schema"] != "csdlc.issue.index.v1"
        || [receipt, record, index].iter().any(|v| {
            v["issue"] != issue
                || v["repository"] != repository
                || v["initialization_digest"] != receipt["initialization_digest"]
        })
        || receipt["initialization_digest"]
            .as_str()
            .is_none_or(str::is_empty)
        || receipt["receipt_ref"] != reference
        || record["phase"] != "closed_out"
        || !record["branch"].is_null()
        || !record["worktree"].is_null()
        || !matches!(
            index["phase"].as_str(),
            Some(
                "initialized"
                    | "ready"
                    | "bound"
                    | "implemented"
                    | "reviewed"
                    | "published"
                    | "merge_ready"
                    | "merged"
                    | "closed_out"
            )
        )
        || index["generation"].as_u64().is_none()
        || record["generation"].as_u64() < index["generation"].as_u64()
    {
        return Err("exact legacy terminal identity required".into());
    }
    let publication = &record["publication"];
    let terminal_value = normalized_terminal(&record["terminal"])?;
    let terminal = &terminal_value;
    let pr = terminal["pull_request"]
        .as_u64()
        .filter(|n| *n > 0)
        .ok_or("legacy terminal PR required")?;
    let sha = terminal["observed_sha"]
        .as_str()
        .filter(|s| s.len() == 40 && s.bytes().all(|b| b.is_ascii_hexdigit()))
        .ok_or("legacy terminal head required")?;
    let revision = publication["revision"].as_str().unwrap_or("");
    if terminal["disposition"] != "merged"
        || terminal["observed_state"] != "merged"
        || terminal["receipt_path"] != reference
        || publication["repository"] != repository
        || publication["issue"] != issue
        || publication["pull_request"] != pr
        || publication["observed_state"] != "merged"
        || publication["draft"] != false
        || publication["url"] != format!("https://github.com/{repository}/pull/{pr}")
        || !revision.starts_with(&format!("git-blake3:{sha}:"))
        || publication["head"] != terminal["branch"]
        || terminal["branch"].as_str().is_none_or(str::is_empty)
        || terminal["worktree"].as_str().is_none_or(str::is_empty)
    {
        return Err("legacy publication and terminal delivery disagree".into());
    }
    if !index["publication"].is_null()
        && [
            "repository",
            "issue",
            "pull_request",
            "url",
            "base",
            "head",
            "revision",
        ]
        .iter()
        .any(|key| index["publication"][*key] != publication[*key])
    {
        return Err("historical publication differs from terminated delivery".into());
    }
    if !index["terminal"].is_null() && normalized_terminal(&index["terminal"])? != *terminal {
        return Err("historical source disagrees with retained terminal evidence".into());
    }
    for key in ["branch", "worktree"] {
        if !index[key].is_null() && index[key] != terminal[key] {
            return Err("historical source topology disagrees with terminal evidence".into());
        }
    }
    // Pre-topology records retain their binding in claim instead of top-level fields.
    if !index["claim"].is_null()
        && ["branch", "worktree"]
            .iter()
            .any(|key| index["claim"][*key] != terminal[*key])
    {
        return Err("historical claim differs from terminated execution".into());
    }
    Ok((pr, sha.to_owned()))
}

pub(super) fn terminal(
    common: &Path,
    checkout: &Path,
    issue: u64,
    repository: &str,
    index: &Value,
) -> Result<LegacyTerminal> {
    let path = common.join(format!("csdlc-v2/closeout/{issue}.json"));
    safe(&path)?;
    let receipt: Value = read(&path)?;
    let (pull_request, head_sha) = evidence(issue, repository, index, &receipt)?;
    let artifacts = receipt["authored_artifacts"]
        .as_object()
        .ok_or("legacy authored artifacts missing")?;
    if artifacts.len() != 2 {
        return Err("exact retained design and diagram required".into());
    }
    let mut authored_paths = BTreeSet::new();
    for (key, relative) in [
        ("design_path", "retained/design.md"),
        ("diagram_path", "retained/diagram.mmd"),
    ] {
        let full = format!(".csdlc/issues/{issue}/{relative}");
        if receipt["record"][key] != full {
            return Err("legacy retained authored path mismatch".into());
        }
        let expected = artifacts
            .get(&full)
            .and_then(Value::as_str)
            .ok_or("retained authored content missing")?;
        let source = checkout.join(&full);
        safe(&source)?;
        if source.exists() {
            if fs::read(&source).map_err(err)? != expected.as_bytes() {
                return Err("historical authored content differs from terminal receipt".into());
            }
            authored_paths.insert(relative.to_owned());
        }
    }
    Ok(LegacyTerminal {
        pull_request,
        head_sha,
        receipt_digest: receipt["digest"]
            .as_str()
            .ok_or("legacy digest missing")?
            .to_owned(),
        authored_paths,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn seal(v: &mut Value) {
        v["digest"] = json!("");
        v["digest"] = json!(blake3::hash(&serde_json::to_vec(v).unwrap())
            .to_hex()
            .to_string());
    }
    fn fixture() -> (Value, Value) {
        let repo = "danielbaustin/agent-design-language";
        let sha = "a".repeat(40);
        let index = json!({"schema":"csdlc.issue.index.v1","issue":7,"repository":repo,"initialization_digest":"init","phase":"bound","generation":1});
        let mut record = json!({"schema":"csdlc.issue.index.v1","issue":7,"repository":repo,"initialization_digest":"init","phase":"closed_out","generation":2,
            "publication":{"repository":repo,"issue":7,"pull_request":8,"url":format!("https://github.com/{repo}/pull/8"),"head":"codex/7","revision":format!("git-blake3:{sha}:digest"),"draft":false,"observed_state":"merged"},
            "terminal":{"pull_request":8,"disposition":"merged","observed_sha":sha,"observed_state":"merged","receipt_path":"csdlc-v2/closeout/7.json","branch":"codex/7","worktree":".worktrees/7"}});
        seal(&mut record);
        let mut receipt = json!({"schema":"csdlc.terminal_receipt.v1","issue":7,"repository":repo,"initialization_digest":"init","receipt_ref":"csdlc-v2/closeout/7.json","record":record});
        seal(&mut receipt);
        (index, receipt)
    }
    #[test]
    fn terminal_identity_and_digests_bind_stale_source() {
        let (mut index, mut receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        assert_eq!(evidence(7, repo, &index, &receipt).unwrap().0, 8);
        index["initialization_digest"] = json!("different");
        assert!(evidence(7, repo, &index, &receipt).is_err());
        index["initialization_digest"] = json!("init");
        receipt["record"]["terminal"]["observed_sha"] = json!("b".repeat(40));
        assert!(evidence(7, repo, &index, &receipt).is_err());
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert!(evidence(7, repo, &index, &receipt).is_err());
    }
    #[test]
    fn source_topology_and_publication_cannot_override_terminal_receipt() {
        let (mut index, receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        index["claim"] = json!({"branch":"other","worktree":".worktrees/7"});
        assert!(evidence(7, repo, &index, &receipt).is_err());
        index["claim"] = Value::Null;
        index["publication"] = json!({"pull_request":9});
        assert!(evidence(7, repo, &index, &receipt).is_err());
        assert!(evidence(7, "agent-logic/agent-design-language", &index, &receipt).is_err());
    }
    #[test]
    fn published_copy_requires_same_delivery_but_may_precede_merge_observation() {
        let (mut index, receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        index["phase"] = json!("published");
        index["publication"] = receipt["record"]["publication"].clone();
        index["publication"]["observed_state"] = json!("open");
        assert!(evidence(7, repo, &index, &receipt).is_ok());
        index["publication"]["revision"] = json!("git-blake3:other:other");
        assert!(evidence(7, repo, &index, &receipt).is_err());
    }
    #[test]
    fn old_release_projection_matches_receipt_but_ambiguous_aliases_fail() {
        let (mut index, receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        let mut t = receipt["record"]["terminal"].clone();
        let object = t.as_object_mut().unwrap();
        let branch = object.remove("branch").unwrap();
        let worktree = object.remove("worktree").unwrap();
        object.insert("released_branch".into(), branch);
        object.insert("released_worktree".into(), worktree);
        object.insert("released_protected_paths".into(), json!(["source.rs"]));
        index["terminal"] = t;
        assert!(evidence(7, repo, &index, &receipt).is_ok());
        index["terminal"]["branch"] = json!("codex/7");
        assert!(evidence(7, repo, &index, &receipt).is_err());
    }
}
