//! Explicit preservation and reconciliation of unbound, closed historical copies.
use super::*;
use crate::adapters::{
    CommandInvocation, EnvironmentCredentialResolver, ProcessAdapter, ProcessStatus,
    RealProcessAdapter,
};

const REPOSITORY: &str = "agent-logic/agent-design-language";
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Spec {
    primary: PathBuf,
    checkout: PathBuf,
    expected_head: String,
    issues: Vec<u64>,
    operator: String,
    approval_reference: String,
}
fn common(primary: &Path) -> Result<PathBuf> {
    safe(primary)?;
    let p = PathBuf::from(git(
        primary,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?);
    if p != primary.join(".git") {
        return Err("historical reconciliation requires canonical primary".into());
    }
    Ok(p)
}
fn location(common: &Path, checkout: &Path, issue: u64) -> PathBuf {
    common
        .join("csdlc-v3/local/historical-copies")
        .join(
            blake3::hash(checkout.as_os_str().as_encoded_bytes())
                .to_hex()
                .to_string(),
        )
        .join(issue.to_string())
}
fn closed(process: &mut impl ProcessAdapter, issue: u64) -> Result<Value> {
    let command = CommandInvocation::new(
        "github-api-read-only",
        ["issue".into(), REPOSITORY.into(), issue.to_string()],
    )
    .and_then(|c| c.with_child_credential("GITHUB_TOKEN"))
    .map_err(err)?;
    let out = process.run(command);
    if out.status != ProcessStatus::Exit(0) || out.truncated {
        return Err(format!(
            "issue {issue}: authenticated closed readback unavailable"
        ));
    }
    let v: Value =
        serde_json::from_str(&out.stdout).map_err(|_| "invalid closed issue readback")?;
    if v["number"] != issue
        || v["state"] != "closed"
        || !v["pull_request"].is_null()
        || v["html_url"] != format!("https://github.com/{REPOSITORY}/issues/{issue}")
        || v["closed_at"].as_str().is_none_or(str::is_empty)
        || v["id"].as_u64().is_none()
    {
        return Err(format!(
            "issue {issue}: exact closed issue identity required"
        ));
    }
    Ok(
        json!({"id":v["id"],"number":issue,"url":v["html_url"],"state":"closed","closed_at":v["closed_at"]}),
    )
}
fn source(common: &Path, checkout: &Path, issue: u64) -> Result<(PathBuf, String)> {
    for path in [
        common.join(format!("csdlc-v3/local/issues/{issue}")),
        common.join(format!("csdlc-v3/semantic/issues/{issue}")),
        common.join(format!("csdlc-v3/local/bindings/{issue}.json")),
    ] {
        safe(&path)?;
        if path.exists() {
            return Err(format!(
                "issue {issue}: operational record cannot be classified as history"
            ));
        }
    }
    for root in [
        common.join("csdlc-v3/local/transactions"),
        checkout.join(".csdlc/transactions"),
    ] {
        for dir in [root.clone(), root.join("pending")] {
            safe(&dir)?;
            if dir.exists() {
                for entry in fs::read_dir(dir).map_err(err)? {
                    let p = entry.map_err(err)?.path();
                    safe(&p)?;
                    if p.is_file() && p.extension().is_some_and(|s| s == "json") {
                        return Err(
                            "pending native transaction forbids historical reconciliation".into(),
                        );
                    }
                }
            }
        }
    }
    let rel = format!(".csdlc/issues/{issue}");
    let p = checkout.join(&rel);
    safe(&p)?;
    let index: Value = read(&p.join("index.json"))?;
    if index["schema"] != "csdlc.issue.index.v1"
        || index["issue"] != issue
        || index["repository"] != REPOSITORY
        || index["phase"] != "initialized"
        || !index["branch"].is_null()
        || !index["worktree"].is_null()
        || !index["publication"].is_null()
        || !index["terminal"].is_null()
        || index["transitions"] != json!([])
    {
        return Err(format!(
            "issue {issue}: only unbound initialized historical copies are admitted"
        ));
    }
    let mut expected = BTreeSet::from([format!("{rel}/index.json"), format!("{rel}/audit.jsonl")]);
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        expected.insert(format!("{rel}/cards/{card}.md"));
        expected.insert(format!("{rel}/cards/{card}.values.json"));
    }
    let actual: BTreeSet<_> = git(checkout, &["ls-files", "--others", "-z", "--", &rel])?
        .split('\0')
        .filter(|s| !s.is_empty())
        .map(str::to_owned)
        .collect();
    if actual != expected || !git(checkout, &["ls-files", "-z", "--", &rel])?.is_empty() {
        return Err(format!("issue {issue}: exact untracked card bundle required; extra/missing/tracked files refused"));
    }
    for file in expected {
        let f = checkout.join(file);
        safe(&f)?;
        if !f.is_file() {
            return Err("non-regular historical member".into());
        }
    }
    let digest = fingerprint(&p)?;
    Ok((p, digest))
}
fn topology(spec: &Spec) -> Result<PathBuf> {
    if spec.operator.trim().is_empty()
        || spec.approval_reference.trim().is_empty()
        || spec.issues.is_empty()
        || spec.issues.len() > 100
        || spec.issues.contains(&0)
        || spec.issues.iter().collect::<BTreeSet<_>>().len() != spec.issues.len()
    {
        return Err("explicit bounded historical reconciliation decision required".into());
    }
    safe(&spec.checkout)?;
    if fs::canonicalize(&spec.checkout).map_err(err)? != spec.checkout
        || fs::canonicalize(&spec.primary).map_err(err)? != spec.primary
    {
        return Err("canonical absolute historical paths required".into());
    }
    let c = common(&spec.primary)?;
    let checkout_common = git(
        &spec.checkout,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?;
    if Path::new(&checkout_common) != c
        || git(&spec.checkout, &["rev-parse", "HEAD"])? != spec.expected_head
        || !git(&spec.primary, &["worktree", "list", "--porcelain"])?
            .lines()
            .any(|l| l == format!("worktree {}", spec.checkout.display()))
    {
        return Err("historical checkout registration/head mismatch".into());
    }
    let origin = git(&spec.primary, &["remote", "get-url", "origin"])?;
    if origin != format!("https://github.com/{REPOSITORY}.git")
        && origin != format!("git@github.com:{REPOSITORY}.git")
    {
        return Err("historical repository identity mismatch".into());
    }
    Ok(c)
}
fn prepare(spec: &Spec, process: &mut impl ProcessAdapter) -> Result<Value> {
    let common = topology(spec)?;
    let mut records = Vec::new();
    for issue in &spec.issues {
        let (_, digest) = source(&common, &spec.checkout, *issue)?;
        let remote = closed(process, *issue)?;
        if source(&common, &spec.checkout, *issue)?.1 != digest {
            return Err("historical source changed during readback".into());
        }
        records.push(json!({"issue":issue,"source_digest":digest,"closed_readback":remote}));
    }
    Ok(json!({"schema":"csdlc.v3.historical_copy_preview.v1","spec":spec,"records":records}))
}
fn apply(spec: &Spec, preview: &str, process: &mut impl ProcessAdapter) -> Result<Value> {
    let common = topology(spec)?;
    let lock_root = common.join("csdlc-v3/local/locks");
    safe(&lock_root)?;
    for directory in [common.join("csdlc-v3/local"), lock_root.clone()] {
        if directory.exists()
            && fs::metadata(&directory).map_err(err)?.permissions().mode() & 0o222 == 0
        {
            return Err("historical reconciliation denied while native namespace is fenced".into());
        }
    }
    fs::create_dir_all(&lock_root).map_err(err)?;
    let mut locks = Vec::new();
    for issue in spec.issues.iter().copied().collect::<BTreeSet<_>>() {
        let path = lock_root.join(format!("{issue}.lock"));
        safe(&path)?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(path)
            .map_err(err)?;
        FileExt::try_lock_exclusive(&lock).map_err(|_| {
            format!("issue {issue}: native writer busy; historical reconciliation refused")
        })?;
        locks.push(lock);
    }
    let packet = prepare(spec, process)?;
    let digest = blake3::hash(&bytes(&packet)?).to_hex().to_string();
    if digest != preview {
        return Err("historical preview changed; no disposition written".into());
    }
    let common = topology(spec)?;
    for row in packet["records"].as_array().ok_or("missing records")? {
        let issue = row["issue"].as_u64().ok_or("missing issue")?;
        let (src, before) = source(&common, &spec.checkout, issue)?;
        if row["source_digest"] != before {
            return Err("historical source changed".into());
        }
        let dest = location(&common, &spec.checkout, issue);
        tree(&src, &dest.join("snapshot"))?;
        if fingerprint(&dest.join("snapshot"))? != before
            || source(&common, &spec.checkout, issue)?.1 != before
        {
            return Err("historical snapshot verification failed; original preserved".into());
        }
        let receipt = json!({"schema":"csdlc.v3.historical_copy_disposition.v1","spec":spec,"record":row,"preview_digest":preview});
        put(&dest.join("disposition.json"), &bytes(&receipt)?)?;
    }
    Ok(
        json!({"status":"historical_copies_reconciled","preview_digest":preview,"issues":spec.issues,"originals_preserved":true}),
    )
}
/// Preview is read-only. Execution preserves source and creates immutable evidence only.
pub fn reconcile(path: &Path, preview: Option<&str>) -> Result<Value> {
    let spec: Spec = read(path)?;
    if canonical_v3_authority(&spec.primary)?.is_none() {
        return Err("canonical native authority unavailable".into());
    }
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    if let Some(digest) = preview {
        apply(&spec, digest, &mut process)
    } else {
        let packet = prepare(&spec, &mut process)?;
        Ok(
            json!({"status":"preview","read_only":true,"preview_digest":blake3::hash(&bytes(&packet)?).to_hex().to_string(),"packet":packet}),
        )
    }
}
fn disposition_with_process(
    common: &Path,
    checkout: &Path,
    issue: u64,
    process: &mut impl ProcessAdapter,
) -> Result<Option<Value>> {
    let root = location(common, checkout, issue);
    let path = root.join("disposition.json");
    safe(&path)?;
    if !path.exists() {
        return Ok(None);
    }
    let receipt: Value = read(&path)?;
    let spec: Spec = serde_json::from_value(receipt["spec"].clone()).map_err(err)?;
    if receipt["schema"] != "csdlc.v3.historical_copy_disposition.v1"
        || spec.checkout != checkout
        || !spec.issues.contains(&issue)
        || topology(&spec)? != common
        || receipt["record"]["issue"] != issue
    {
        return Err("historical disposition identity mismatch".into());
    }
    let (_, digest) = source(common, checkout, issue)?;
    if receipt["record"]["source_digest"] != digest
        || fingerprint(&root.join("snapshot"))? != digest
    {
        return Err(format!(
            "issue {issue}: historical source or preserved snapshot changed"
        ));
    }
    let current = closed(process, issue)?;
    if current != receipt["record"]["closed_readback"] {
        return Err(format!("issue {issue}: historical closure changed"));
    }
    Ok(Some(
        json!({"issue":issue,"checkout":checkout,"revision":spec.expected_head,"disposition":"authenticated_preserved_historical_copy","source_digest":digest,"receipt":path}),
    ))
}

pub(super) fn disposition(common: &Path, checkout: &Path, issue: u64) -> Result<Option<Value>> {
    disposition_with_process(
        common,
        checkout,
        issue,
        &mut RealProcessAdapter::new(EnvironmentCredentialResolver),
    )
}
pub(super) fn retained_issues(common: &Path, checkout: &Path) -> Result<BTreeSet<u64>> {
    let root = location(common, checkout, 1)
        .parent()
        .ok_or("missing history parent")?
        .to_path_buf();
    safe(&root)?;
    let mut ids = BTreeSet::new();
    if root.exists() {
        for entry in fs::read_dir(root).map_err(err)? {
            let path = entry.map_err(err)?.path();
            safe(&path)?;
            if path.join("disposition.json").exists() {
                ids.insert(
                    path.file_name()
                        .and_then(|s| s.to_str())
                        .and_then(|s| s.parse::<u64>().ok())
                        .filter(|n| *n > 0)
                        .ok_or("invalid historical disposition issue")?,
                );
            }
        }
    }
    Ok(ids)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adapters::ProcessOutput;
    struct Remote {
        response: Value,
        status: ProcessStatus,
        truncated: bool,
    }
    impl Remote {
        fn closed() -> Self {
            Self {
                response: json!({"id":99,"number":3,"state":"closed","html_url":format!("https://github.com/{REPOSITORY}/issues/3"),"closed_at":"2026-08-01T00:00:00Z"}),
                status: ProcessStatus::Exit(0),
                truncated: false,
            }
        }
    }
    impl ProcessAdapter for Remote {
        fn run(&mut self, cmd: CommandInvocation) -> ProcessOutput {
            assert_eq!(cmd.program, "github-api-read-only");
            assert_eq!(cmd.argv(), &["issue", REPOSITORY, "3"]);
            assert_eq!(cmd.child_credential_name(), Some("GITHUB_TOKEN"));
            ProcessOutput {
                status: self.status,
                stdout: self.response.to_string(),
                stderr: String::new(),
                truncated: self.truncated,
            }
        }
    }
    struct Fixture {
        spec: Spec,
        common: PathBuf,
        source: PathBuf,
    }
    impl Fixture {
        fn new() -> Self {
            let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target/history-unit")
                .join(format!(
                    "{}-{}-{}",
                    std::process::id(),
                    SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_nanos(),
                    WRITE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(&root).unwrap();
            let root = fs::canonicalize(root).unwrap();
            git(&root, &["init", "-q"]).unwrap();
            git(&root, &["config", "user.email", "fixture@example.invalid"]).unwrap();
            git(&root, &["config", "user.name", "Fixture"]).unwrap();
            put(&root.join("README"), b"fixture").unwrap();
            git(&root, &["add", "README"]).unwrap();
            git(&root, &["commit", "-qm", "fixture"]).unwrap();
            git(
                &root,
                &[
                    "remote",
                    "add",
                    "origin",
                    &format!("https://github.com/{REPOSITORY}.git"),
                ],
            )
            .unwrap();
            let source = root.join(".csdlc/issues/3");
            put(&source.join("index.json"),&bytes(&json!({"schema":"csdlc.issue.index.v1","issue":3,"repository":REPOSITORY,"phase":"initialized","transitions":[]})).unwrap()).unwrap();
            put(&source.join("audit.jsonl"), b"historical audit").unwrap();
            for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
                for suffix in ["md", "values.json"] {
                    put(
                        &source.join(format!("cards/{card}.{suffix}")),
                        b"historical card",
                    )
                    .unwrap();
                }
            }
            let spec = Spec {
                primary: root.clone(),
                checkout: root.clone(),
                expected_head: git(&root, &["rev-parse", "HEAD"]).unwrap(),
                issues: vec![3],
                operator: "fixture".into(),
                approval_reference: "fixture explicit historical disposition".into(),
            };
            Self {
                spec,
                common: root.join(".git"),
                source,
            }
        }
        fn preview(&self) -> String {
            blake3::hash(&bytes(&prepare(&self.spec, &mut Remote::closed()).unwrap()).unwrap())
                .to_hex()
                .to_string()
        }
        fn apply(&self) {
            apply(&self.spec, &self.preview(), &mut Remote::closed()).unwrap();
        }
        fn read(&self, remote: &mut Remote) -> Result<Option<Value>> {
            disposition_with_process(&self.common, &self.spec.checkout, 3, remote)
        }
    }
    #[test]
    fn history_preview_reconcile_replay_preserve_sources_and_no_remote_mutation() {
        let f = Fixture::new();
        let before = fingerprint(&f.source).unwrap();
        let digest = f.preview();
        assert!(!location(&f.common, &f.spec.checkout, 3).exists());
        assert!(apply(&f.spec, "stale", &mut Remote::closed()).is_err());
        assert!(!location(&f.common, &f.spec.checkout, 3).exists());
        apply(&f.spec, &digest, &mut Remote::closed()).unwrap();
        f.apply();
        assert_eq!(fingerprint(&f.source).unwrap(), before);
        assert!(f.read(&mut Remote::closed()).unwrap().is_some());
        assert_eq!(
            retained_issues(&f.common, &f.spec.checkout).unwrap(),
            BTreeSet::from([3])
        );
    }
    #[test]
    fn history_source_snapshot_deletion_and_reopened_remote_fail_closed() {
        for case in ["source", "snapshot", "deleted", "reopened", "reclosed"] {
            let f = Fixture::new();
            f.apply();
            let mut remote = Remote::closed();
            match case {
                "source" => fs::write(f.source.join("audit.jsonl"), b"changed").unwrap(),
                "snapshot" => fs::write(
                    location(&f.common, &f.spec.checkout, 3).join("snapshot/audit.jsonl"),
                    b"changed",
                )
                .unwrap(),
                "deleted" => fs::remove_dir_all(&f.source).unwrap(),
                "reopened" => remote.response["state"] = json!("open"),
                "reclosed" => remote.response["closed_at"] = json!("2026-09-01T00:00:00Z"),
                _ => unreachable!(),
            }
            assert!(f.read(&mut remote).is_err(), "{case}");
            assert_eq!(
                retained_issues(&f.common, &f.spec.checkout).unwrap(),
                BTreeSet::from([3])
            );
        }
    }
    #[test]
    fn history_requires_exact_authenticated_closed_issue() {
        for case in [
            "open",
            "wrong_issue",
            "wrong_repo",
            "pr",
            "unauthenticated",
            "truncated",
        ] {
            let f = Fixture::new();
            let mut remote = Remote::closed();
            match case {
                "open" => remote.response["state"] = json!("open"),
                "wrong_issue" => remote.response["number"] = json!(4),
                "wrong_repo" => {
                    remote.response["html_url"] = json!("https://github.com/other/repo/issues/3")
                }
                "pr" => remote.response["pull_request"] = json!({}),
                "unauthenticated" => remote.status = ProcessStatus::Exit(126),
                "truncated" => remote.truncated = true,
                _ => unreachable!(),
            }
            assert!(prepare(&f.spec, &mut remote).is_err(), "{case}");
            assert!(!location(&f.common, &f.spec.checkout, 3).exists());
        }
    }
    #[test]
    fn history_refuses_active_extra_symlink_pending_and_topology_drift() {
        for case in [
            "native", "semantic", "binding", "bound", "extra", "symlink", "pending", "head",
            "approval",
        ] {
            let mut f = Fixture::new();
            match case {
                "native" => fs::create_dir_all(f.common.join("csdlc-v3/local/issues/3")).unwrap(),
                "semantic" => {
                    fs::create_dir_all(f.common.join("csdlc-v3/semantic/issues/3")).unwrap()
                }
                "binding" => put(&f.common.join("csdlc-v3/local/bindings/3.json"), b"{}").unwrap(),
                "bound" => {
                    let mut v: Value = read(&f.source.join("index.json")).unwrap();
                    v["worktree"] = json!("another-owner");
                    fs::write(f.source.join("index.json"), bytes(&v).unwrap()).unwrap();
                }
                "extra" => put(&f.source.join("unknown"), b"preserve").unwrap(),
                "symlink" => {
                    fs::remove_file(f.source.join("audit.jsonl")).unwrap();
                    std::os::unix::fs::symlink(
                        f.spec.primary.join("README"),
                        f.source.join("audit.jsonl"),
                    )
                    .unwrap();
                }
                "pending" => put(
                    &f.spec.checkout.join(".csdlc/transactions/pending/3.json"),
                    b"{}",
                )
                .unwrap(),
                "head" => f.spec.expected_head = "0".repeat(40),
                "approval" => f.spec.approval_reference.clear(),
                _ => unreachable!(),
            }
            assert!(prepare(&f.spec, &mut Remote::closed()).is_err(), "{case}");
            assert!(!location(&f.common, &f.spec.checkout, 3).exists());
        }
    }
    #[test]
    fn history_cannot_downgrade_retained_disposition_to_committed_history() {
        let f = Fixture::new();
        f.apply();
        git(&f.spec.checkout, &["add", ".csdlc/issues/3"]).unwrap();
        git(
            &f.spec.checkout,
            &["commit", "-qm", "commit preserved copy"],
        )
        .unwrap();
        let rejected = super::super::census_inventory(&f.spec.primary, &f.common)
            .err()
            .unwrap();
        assert!(
            rejected.contains("registration/head mismatch"),
            "{rejected}"
        );
    }
    #[test]
    fn history_execution_refuses_busy_writer_and_sealed_namespace() {
        let f = Fixture::new();
        let digest = f.preview();
        let root = f.common.join("csdlc-v3/local/locks");
        fs::create_dir_all(&root).unwrap();
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(root.join("3.lock"))
            .unwrap();
        FileExt::lock_exclusive(&lock).unwrap();
        assert!(apply(&f.spec, &digest, &mut Remote::closed())
            .unwrap_err()
            .contains("writer busy"));
        drop(lock);
        fs::set_permissions(&root, fs::Permissions::from_mode(0o555)).unwrap();
        assert!(apply(&f.spec, &digest, &mut Remote::closed())
            .unwrap_err()
            .contains("namespace is fenced"));
        fs::set_permissions(&root, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(!location(&f.common, &f.spec.checkout, 3).exists());
    }
}
