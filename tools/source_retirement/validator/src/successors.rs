use crate::*;
pub const RUNTIME_REMNANTS_PIN: &str =
    "323599a131098ee4367adfbade76454db9b00d64c70d9abef813f5c1278c3f13";
pub const CLEANUP_PIN: &str = "762c27b2d844ba3f166ea5bf05a1eb65b2f752b7d4e4f341f107b7a9ebf15915";
pub const READINESS_PIN: &str = "4b1cc0ef6b60d7cc0ec8b2b9d3354489d0036328dcfa9f1d1e43543ffc7bf560";
pub const PODCAST_PIN: &str = "a7026948d2934bb4cef4071034a168d61d51df3c96c7a6131ffd56d8ea80fd3f";
pub fn transition(candidate: &Tree, baseline: &Tree, manifest: &Value) -> Result<Tree> {
    let mut result = candidate.clone();
    let mut seen = Paths::new();
    for row in rows(&manifest["changes"])? {
        let p = string(&row["path"])?;
        ensure!(
            seen.insert(p.to_owned())
                && row["before"] == *at(baseline, p)
                && row["after"] == *at(candidate, p),
            "transition mismatch: {p}"
        );
        replace(&mut result, p, &row["before"]);
    }
    Ok(result)
}
pub fn language(repo: &Repo, candidate: &Tree, manifest: &Value) -> Result<Tree> {
    let baseline = repo.tree(string(&manifest["source_commit"])?)?;
    let result = transition(candidate, &baseline, manifest)?;
    let mut removed = Paths::new();
    for r in rows(&manifest["changes"])? {
        ensure!(
            sha(&repo.blob(string(&r["before"][1])?)?) == r["before_sha256"],
            "language preimage custody mismatch"
        );
        if r["after"].is_null() {
            removed.insert(string(&r["path"])?.to_owned());
        }
    }
    ensure!(
        removed == strings(&manifest["removed"])?,
        "language removal set mismatch"
    );
    Ok(result)
}
pub fn csdlc_untracking(repo: &Repo, candidate: &Tree, manifest: &Value) -> Result<Tree> {
    ensure!(
        manifest["schema"] == "adl.csdlc_untracking.v1" && manifest["path"] == ".csdlc",
        "invalid local-state transition scope"
    );
    let revision = string(&manifest["source_commit"])?;
    ensure!(
        revision.len() == 40 && revision.bytes().all(|b| b.is_ascii_hexdigit()),
        "immutable source commit required"
    );
    repo.git(&["merge-base", "--is-ancestor", revision, "HEAD"])?;
    ensure!(
        repo.text(&["rev-parse", &format!("{revision}:.csdlc")])? == string(&manifest["tree_oid"])?,
        "local-state custody tree mismatch"
    );
    ensure!(
        !candidate
            .keys()
            .any(|p| p == ".csdlc" || p.starts_with(".csdlc/")),
        "local state reintroduced into tracked source"
    );
    let historical = repo.tree(revision)?;
    let entries: Vec<_> = historical
        .iter()
        .filter(|(p, _)| p.starts_with(".csdlc/"))
        .collect();
    ensure!(
        !entries.is_empty() && entries.len() == number(&manifest["entry_count"])?,
        "local-state custody count mismatch"
    );
    let mut result = candidate.clone();
    for (path, identity) in entries {
        result.insert(path.clone(), identity.clone());
    }
    Ok(result)
}
pub fn normalize_language(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let manifest = repo.pinned(
        "docs/language-cleanup/csdlc-untracking.json",
        super::constants::CSDLC_UNTRACKING_PIN,
    )?;
    let mut result = csdlc_untracking(repo, candidate, &manifest)?;
    for (name, pin) in super::constants::LANGUAGE_TRANSITIONS {
        let manifest = repo.pinned(&format!("docs/language-cleanup/{name}"), pin)?;
        result = language(repo, &result, &manifest)?;
    }
    Ok(result)
}
pub fn cleanup(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let t = repo.pinned("tools/repository_cleanup/transition.json", CLEANUP_PIN)?;
    let archive = repo.pinned(
        "docs/archives/POST_SPLIT_PAYLOADS_1284.json",
        string(&t["archive_manifest_sha256"])?,
    )?;
    ensure!(
        archive["source_commit"] == t["source_commit"],
        "cleanup archive source changed"
    );
    let entries = rows(&archive["entries"])?;
    let map = entries
        .iter()
        .map(|r| Ok((string(&r["path"])?, r)))
        .collect::<Result<BTreeMap<_, _>>>()?;
    ensure!(map.len() == entries.len(), "duplicate archived path");
    let result = transition(candidate, &repo.tree(string(&t["source_commit"])?)?, &t)?;
    let mut removed = Paths::new();
    for r in rows(&t["changes"])? {
        if r["after"].is_null() {
            let p = string(&r["path"])?;
            let e = map
                .get(p)
                .context("cleanup removal lacks archive identity")?;
            ensure!(
                json!([e["mode"], e["git_blob"]]) == r["before"],
                "cleanup removal lacks exact archive identity"
            );
            removed.insert(p.to_owned());
        }
    }
    ensure!(
        removed == map.keys().map(|p| p.to_string()).collect(),
        "archive and cleanup removal sets differ"
    );
    Ok(result)
}
pub fn readiness(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let t = repo.pinned(
        "tools/public_readiness_archive/transition.json",
        READINESS_PIN,
    )?;
    readiness_manifest(repo, candidate, &t)
}
pub fn readiness_manifest(repo: &Repo, candidate: &Tree, t: &Value) -> Result<Tree> {
    let base = repo.tree(string(&t["source_commit"])?)?;
    let mut result = candidate.clone();
    let mut seen = Paths::new();
    for r in rows(&t["entries"])? {
        let p = string(&r["path"])?;
        let id = json!([r["mode"], r["git_blob"]]);
        ensure!(
            seen.insert(p.to_owned()) && *at(&base, p) == id && !candidate.contains_key(p),
            "archive removal identity mismatch: {p}"
        );
        let raw = repo.blob(string(&r["git_blob"])?)?;
        ensure!(
            raw.len() == number(&r["bytes"])? && sha(&raw) == r["sha256"],
            "archive preimage custody mismatch"
        );
        result.insert(p.to_owned(), id);
    }
    let c = &t["caller_change"];
    let p = string(&c["path"])?;
    ensure!(
        *at(&base, p) == c["before"] && *at(candidate, p) == c["after"],
        "archive guard caller identity mismatch"
    );
    replace(&mut result, p, &c["before"]);
    Ok(result)
}
pub fn podcast(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let t = repo.pinned("tools/podcast_migration/transition.json", PODCAST_PIN)?;
    let manifest = repo.pinned(
        "tools/podcast_migration/source-manifest.json",
        string(&t["manifest_sha256"])?,
    )?;
    let imported = rows(&manifest["entries"])?
        .iter()
        .filter(|r| r["source_kind"] == "adl_git")
        .map(|r| Ok(string(&r["source_path"])?.to_owned()))
        .collect::<Result<Paths>>()?;
    let removed = strings(&t["removed"])?;
    ensure!(
        removed.len() == rows(&t["removed"])?.len(),
        "duplicate podcast removal"
    );
    let result = transition(candidate, &repo.tree(string(&t["source_commit"])?)?, &t)?;
    let mut actual = Paths::new();
    for r in rows(&t["changes"])? {
        let p = string(&r["path"])?;
        if r["after"].is_null() {
            ensure!(
                removed.contains(p) && imported.contains(p),
                "podcast removal lacks destination provenance"
            );
            actual.insert(p.to_owned());
        } else {
            ensure!(!removed.contains(p), "podcast removal still present");
        }
    }
    ensure!(removed == actual, "podcast removal set mismatch");
    Ok(result)
}
pub fn normalized(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let candidate = obsolete_v2_tools(repo, candidate)?;
    let candidate = retired_authority(repo, &candidate)?;
    let candidate = local_adl_state(repo, &candidate)?;
    let candidate = runtime_remnants(repo, &candidate)?;
    let candidate = normalize_language(repo, &candidate)?;
    let candidate = readiness(repo, &candidate)?;
    let candidate = cleanup(repo, &candidate)?;
    let successor = repo.pinned(
        "docs/language-cleanup/podcast-successor-transition.json",
        super::constants::PODCAST_SUCCESSOR_PIN,
    )?;
    let candidate = language(repo, &candidate, &successor)?;
    let candidate = podcast(repo, &candidate)?;
    let retained = repo.pinned(
        "docs/language-cleanup/retained-successor-transition.json",
        super::constants::RETAINED_SUCCESSOR_PIN,
    )?;
    retained_successors(repo, &candidate, &retained)
}

pub fn retained_successors(repo: &Repo, candidate: &Tree, manifest: &Value) -> Result<Tree> {
    ensure!(
        rows(&manifest["removed"])?.is_empty(),
        "retained reconciliation cannot authorize removal"
    );
    let result = language(repo, candidate, manifest)?;
    let accepted = string(&manifest["accepted_main"])?;
    for row in rows(&manifest["changes"])? {
        ensure!(
            !row["before"].is_null() && !row["after"].is_null(),
            "retained reconciliation requires modification"
        );
        let path = string(&row["path"])?;
        let commit = string(&row["successor_commit"])?;
        repo.git(&["merge-base", "--is-ancestor", commit, accepted])?;
        ensure!(
            *at(&repo.tree(commit)?, path) == row["after"],
            "retained successor commit identity mismatch: {path}"
        );
        ensure!(
            sha(&repo.blob(string(&row["after"][1])?)?) == row["after_sha256"],
            "retained successor bytes changed: {path}"
        );
    }
    Ok(result)
}

/// Reconstruct the historical audit view only after verifying exact retired custody.
pub fn runtime_remnants(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let manifest = repo.pinned(
        "docs/archives/RUNTIME_REMNANTS_1370.json",
        RUNTIME_REMNANTS_PIN,
    )?;
    let revision = string(&manifest["source_commit"])?;
    repo.git(&["merge-base", "--is-ancestor", revision, "HEAD"])?;
    let baseline = repo.tree(revision)?;
    let prefixes = [
        "adl-v2/crates/adl-engine/",
        "adl-v2/crates/adl-cli/",
        "adl-v2/crates/adl-records/",
        "csdlc-v2/",
        "infra/runtime-v3/",
    ];
    let expected: Paths = baseline
        .keys()
        .filter(|p| prefixes.iter().any(|prefix| p.starts_with(prefix)))
        .cloned()
        .collect();
    let mut seen = Paths::new();
    let mut result = candidate.clone();
    for row in rows(&manifest["entries"])? {
        let path = string(&row["path"])?;
        let id = json!([row["mode"], row["git_blob"]]);
        ensure!(
            seen.insert(path.to_owned())
                && expected.contains(path)
                && *at(&baseline, path) == id
                && !candidate.contains_key(path),
            "runtime retirement identity mismatch: {path}"
        );
        let raw = repo.blob(string(&row["git_blob"])?)?;
        ensure!(
            raw.len() == number(&row["bytes"])? && sha(&raw) == row["sha256"],
            "runtime retirement custody mismatch"
        );
        result.insert(path.to_owned(), id);
    }
    ensure!(
        seen == expected && !seen.is_empty(),
        "runtime retirement inventory incomplete"
    );
    ensure!(
        !candidate
            .keys()
            .any(|p| prefixes.iter().any(|prefix| p.starts_with(prefix))),
        "retired source reintroduced"
    );
    for row in rows(&manifest["consumer_changes"])? {
        let path = string(&row["path"])?;
        ensure!(
            *at(&baseline, path) == row["before"] && *at(candidate, path) == row["after"],
            "retirement consumer drift: {path}"
        );
        replace(&mut result, path, &row["before"]);
    }
    Ok(result)
}

/// Restore exact historical audit inputs without tracking local workstation state.
pub fn local_adl_state(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let manifest = repo.pinned("docs/archives/ADL_LOCAL_STATE_1372.json", "4dc49dddf1433c7a415e7e400589bd1b907b6e64daeb96f29179ef0768a2fbde")?;
    let revision = string(&manifest["source_commit"])?;
    repo.git(&["merge-base", "--is-ancestor", revision, "HEAD"])?;
    ensure!(
        !candidate.keys().any(|p| p == ".adl" || p.starts_with(".adl/")),
        "local .adl state must remain untracked"
    );
    let baseline = repo.tree(revision)?;
    let expected: Paths = baseline.keys().filter(|p| p.starts_with(".adl/")).cloned().collect();
    ensure!(expected == strings(&manifest["removed"])? && !expected.is_empty(), "local .adl inventory mismatch");
    language(repo, candidate, &manifest)
}

/// Preserve historical cutover evidence without retaining it as project authority.
pub fn retired_authority(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let manifest = repo.pinned("docs/archives/CSDLC_AUTHORITY_1373.json", "7dd84e53950d847fd13655e11ec57dcf509ffa2ce22c7869aeffa64ad0a42337")?;
    let revision = string(&manifest["source_commit"])?;
    repo.git(&["merge-base", "--is-ancestor", revision, "HEAD"])?;
    ensure!(!candidate.keys().any(|p| p.starts_with("csdlc-v3/operator/")), "retired project authority reintroduced");
    let baseline = repo.tree(revision)?;
    let expected: Paths = baseline.keys().filter(|p| p.starts_with("csdlc-v3/operator/")).cloned().collect();
    ensure!(expected == strings(&manifest["removed"])? && expected.len() == 3, "authority retirement inventory mismatch");
    language(repo, candidate, &manifest)
}

/// Restore exact historical script and documentary inputs for the retirement audit.
pub fn obsolete_v2_tools(repo: &Repo, candidate: &Tree) -> Result<Tree> {
    let manifest = repo.pinned("docs/archives/ADL_V2_TOOLS_1376.json", "ed539c6dc83aa42a6dca627ad74be9b00c686d2feec070d3a7fa9c7adb9be39e")?;
    let revision = string(&manifest["source_commit"])?;
    repo.git(&["merge-base", "--is-ancestor", revision, "HEAD"])?;
    let expected: Paths = ["adl-v2/tools/install-adl-v2.sh", "adl-v2/tools/prove-rollback.sh", "adl-v2/tools/run-soak.sh"].into_iter().map(str::to_owned).collect();
    ensure!(expected == strings(&manifest["removed"])?, "obsolete v2 tool scope mismatch");
    language(repo, candidate, &manifest)
}
