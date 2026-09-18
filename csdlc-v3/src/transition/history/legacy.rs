//! Read-only admission of retained v2 terminal evidence; never a v2 lifecycle route.
use super::*;

pub(super) struct LegacyTerminal {
    pub(super) pull_request: u64,
    pub(super) head_sha: String,
    pub(super) receipt_digest: String,
    pub(super) publication_repository: String,
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

fn evidence(
    issue: u64,
    repository: &str,
    index: &Value,
    receipt: &Value,
) -> Result<(u64, String, String)> {
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
    let publication_repository = match &record["code_repository"] {
        Value::Null => repository,
        Value::String(s)
            if [
                "agent-logic/agent-design-language",
                "danielbaustin/agent-design-language",
            ]
            .contains(&s.as_str()) =>
        {
            s
        }
        _ => return Err("unsupported historical code repository".into()),
    };
    if !index["code_repository"].is_null() && index["code_repository"] != record["code_repository"]
    {
        return Err("historical code repository identity disagrees".into());
    }
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
        || publication["repository"] != publication_repository
        || publication["issue"] != issue
        || publication["pull_request"] != pr
        || publication["observed_state"] != "merged"
        || publication["draft"] != false
        || publication["url"] != format!("https://github.com/{publication_repository}/pull/{pr}")
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
        if !record[key].is_null() && record[key] != terminal[key] {
            return Err("retained terminal topology disagrees with its record".into());
        }
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
    if record["branch"].is_null() != record["worktree"].is_null() {
        return Err("partial retained terminal topology".into());
    }
    Ok((pr, sha.to_owned(), publication_repository.to_owned()))
}

fn authored_path(issue: u64, key: &str, record: &Value) -> Result<(String, Option<String>)> {
    let filename = match key {
        "design_path" => "design.md",
        "diagram_path" => "diagram.mmd",
        _ => return Err("unknown historical authored kind".into()),
    };
    let retained = format!(".csdlc/issues/{issue}/retained/{filename}");
    let prepared = format!(".csdlc/prepared/issues/{issue}/{filename}");
    if record[key] == retained {
        Ok((retained, Some(format!("retained/{filename}"))))
    } else if record[key] == prepared {
        Ok((prepared, None))
    } else {
        Err("legacy retained authored path mismatch".into())
    }
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
    let (pull_request, head_sha, publication_repository) =
        evidence(issue, repository, index, &receipt)?;
    let artifacts = receipt["authored_artifacts"]
        .as_object()
        .ok_or("legacy authored artifacts missing")?;
    if artifacts.len() != 2 {
        return Err("exact retained design and diagram required".into());
    }
    let mut authored_paths = BTreeSet::new();
    for key in ["design_path", "diagram_path"] {
        let (full, relative) = authored_path(issue, key, &receipt["record"])?;
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
            if let Some(relative) = relative {
                authored_paths.insert(relative);
            }
        }
    }
    Ok(LegacyTerminal {
        pull_request,
        head_sha,
        publication_repository,
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
    #[test]
    fn closed_record_may_retain_only_its_exact_terminal_topology() {
        let (index, mut receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        receipt["record"]["branch"] = json!("codex/7");
        receipt["record"]["worktree"] = json!(".worktrees/7");
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert!(evidence(7, repo, &index, &receipt).is_ok());
        receipt["record"]["worktree"] = json!(".worktrees/other");
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert!(evidence(7, repo, &index, &receipt).is_err());
    }
    #[test]
    fn cross_repository_publication_requires_explicit_matching_code_repository() {
        let (mut index, mut receipt) = fixture();
        let repo = "danielbaustin/agent-design-language";
        let code = "agent-logic/agent-design-language";
        receipt["record"]["publication"]["repository"] = json!(code);
        receipt["record"]["publication"]["url"] =
            json!(format!("https://github.com/{code}/pull/8"));
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert!(evidence(7, repo, &index, &receipt).is_err());
        receipt["record"]["code_repository"] = json!(code);
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert_eq!(evidence(7, repo, &index, &receipt).unwrap().2, code);
        index["code_repository"] = json!(repo);
        assert!(evidence(7, repo, &index, &receipt).is_err());
        index["code_repository"] = Value::Null;
        receipt["record"]["code_repository"] = json!("untrusted/project");
        seal(&mut receipt["record"]);
        seal(&mut receipt);
        assert!(evidence(7, repo, &index, &receipt).is_err());
    }
    #[test]
    fn authored_path_variants_are_issue_bound_and_do_not_admit_other_files() {
        let mut record = json!({"design_path":".csdlc/prepared/issues/7/design.md"});
        assert_eq!(authored_path(7, "design_path", &record).unwrap().1, None);
        record["design_path"] = json!(".csdlc/issues/7/retained/design.md");
        assert_eq!(
            authored_path(7, "design_path", &record).unwrap().1,
            Some("retained/design.md".into())
        );
        for wrong in [
            ".csdlc/prepared/issues/8/design.md",
            ".csdlc/prepared/issues/7/../design.md",
            "/design.md",
            ".csdlc/issues/7/retained/other.md",
        ] {
            record["design_path"] = json!(wrong);
            assert!(authored_path(7, "design_path", &record).is_err());
        }
    }
}
