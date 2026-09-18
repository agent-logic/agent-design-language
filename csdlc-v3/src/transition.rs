//! Live, explicitly authorized transition of admitted native records.
//!
//! The qualified lifecycle executable is an input, never this executable. Native
//! records and dirty work remain in place. Two persistent guardians retain old
//! and new writer locks across converter exits. No remote mutation is performed.
use crate::authority::{canonical_v3_authority, SELECTOR_PATH};
use crate::commands::local::PromptRegistry;
use crate::conversion::{copied_card_projection, source_digest, source_plan};
use crate::storage::semantic::{
    AcceptedIntentPlan, Admission, Binding, CommitOutcome, CopiedRecordConversion, Digest,
    IssueInputs, IssueKey, LocalChange, NativeWriterFenceGuard, Observation, SemanticRoot,
    Snapshot,
};
use crate::storage::DurableTransactionStore as Store;
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::net::UdpSocket;
use std::os::unix::fs::{MetadataExt, PermissionsExt};
#[cfg(not(test))]
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::Command;
#[cfg(not(test))]
use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
static WRITE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

type Result<T> = std::result::Result<T, String>;
const QUALIFIED_SOURCE: &str = "067cb99bf5c6220f64c9faadd7da6abdca34bcc4";
const QUALIFIED_BINARY: &str = "764a2f43b4f752cae97680b3294f5d26bed7d2538821c3d1fb7d8e57691ac44e";
fn err(e: impl std::fmt::Debug) -> String {
    format!("{e:?}")
}
fn read<T: serde::de::DeserializeOwned>(p: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(p).map_err(err)?).map_err(err)
}
fn bytes(v: &impl Serialize) -> Result<Vec<u8>> {
    serde_json::to_vec_pretty(v).map_err(err)
}
fn hash(p: &Path) -> Result<String> {
    Ok(blake3::hash(&fs::read(p).map_err(err)?)
        .to_hex()
        .to_string())
}
fn git(p: &Path, args: &[&str]) -> Result<String> {
    let o = Command::new("git")
        .arg("-C")
        .arg(p)
        .args(args)
        .output()
        .map_err(err)?;
    if !o.status.success() {
        return Err(format!("git {args:?} failed"));
    }
    Ok(String::from_utf8(o.stdout).map_err(err)?.trim().into())
}
fn safe(p: &Path) -> Result<()> {
    for a in p.ancestors() {
        match fs::symlink_metadata(a) {
            Ok(m) if m.file_type().is_symlink() => {
                return Err(format!("symlink refused: {}", a.display()))
            }
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(e) => return Err(err(e)),
        }
    }
    Ok(())
}
fn sync(p: &Path) -> Result<()> {
    File::open(p).and_then(|f| f.sync_all()).map_err(err)
}
fn put(p: &Path, content: &[u8]) -> Result<()> {
    safe(p)?;
    if p.exists() {
        return if fs::read(p).map_err(err)? == content {
            Ok(())
        } else {
            Err(format!("retained identity changed: {}", p.display()))
        };
    }
    let parent = p.parent().ok_or("missing parent")?;
    fs::create_dir_all(parent).map_err(err)?;
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(err)?
        .as_nanos();
    let temporary = parent.join(format!(
        ".transition-write-{}-{nonce}-{}",
        std::process::id(),
        WRITE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
    ));
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(err)?;
    f.write_all(content).map_err(err)?;
    f.sync_all().map_err(err)?;
    let linked = fs::hard_link(&temporary, p);
    fs::remove_file(&temporary).map_err(err)?;
    match linked {
        Ok(()) => sync(parent),
        Err(e)
            if e.kind() == std::io::ErrorKind::AlreadyExists
                && fs::read(p).map_err(err)? == content =>
        {
            Ok(())
        }
        Err(e) => Err(err(e)),
    }
}
fn tree(from: &Path, to: &Path) -> Result<()> {
    safe(from)?;
    safe(to)?;
    fs::create_dir_all(to).map_err(err)?;
    for e in fs::read_dir(from).map_err(err)? {
        let e = e.map_err(err)?;
        let src = e.path();
        let dst = to.join(e.file_name());
        safe(&src)?;
        if e.file_type().map_err(err)?.is_dir() {
            tree(&src, &dst)?;
        } else if e.file_type().map_err(err)?.is_file() {
            put(&dst, &fs::read(&src).map_err(err)?)?;
            fs::set_permissions(&dst, fs::metadata(&src).map_err(err)?.permissions())
                .map_err(err)?;
        } else {
            return Err("non-regular snapshot member".into());
        }
    }
    sync(to)
}
fn fingerprint(p: &Path) -> Result<String> {
    safe(p)?;
    if !p.exists() {
        Ok("absent".into())
    } else if p.is_dir() {
        Ok(source_digest(p)?.as_str().into())
    } else {
        hash(p)
    }
}
// Empty namespace creation is not a remote effect.
fn remote_fingerprint(p: &Path) -> Result<String> {
    let value = fingerprint(p)?;
    if value == Digest::semantic_projection(&[]).as_str() {
        Ok("absent".into())
    } else {
        Ok(value)
    }
}

// Existing issue writers hold file locks. Denying directory creation also fences
// old repository-scoped owners and writers targeting previously unknown issues.
// This is a cooperative same-user fence, not protection against chmod/root.
fn require_no_pending_native(c: &Context) -> Result<()> {
    let mut roots = BTreeSet::from([c.common.join("csdlc-v3/local/transactions")]);
    for record in &c.request.records {
        if record.checkout != c.request.primary {
            roots.insert(record.checkout.join(".csdlc/transactions"));
        }
    }
    for root in roots {
        for directory in [root.clone(), root.join("pending")] {
            safe(&directory)?;
            if !directory.exists() {
                continue;
            }
            for entry in fs::read_dir(&directory).map_err(err)? {
                let path = entry.map_err(err)?.path();
                safe(&path)?;
                if path.is_file() && path.extension().is_some_and(|s| s == "json") {
                    return Err("pending native transaction; reconcile before transition".into());
                }
            }
        }
    }
    Ok(())
}

fn namespace_paths(c: &Context) -> Vec<PathBuf> {
    [
        "local/locks",
        "local/issues",
        "local/transactions/pending",
        "local/transactions",
        "remote/intents",
        "remote/mutations",
        "remote/recoveries",
        "remote/merges",
        "remote",
    ]
    .iter()
    .map(|p| c.common.join("csdlc-v3").join(p))
    .collect()
}
fn seal_namespaces(c: &Context) -> Result<()> {
    let journal = c.op.join("namespace-modes.json");
    if !journal.exists() {
        let mut modes = Vec::new();
        for path in namespace_paths(c) {
            safe(&path)?;
            fs::create_dir_all(&path).map_err(err)?;
            modes.push(fs::metadata(path).map_err(err)?.permissions().mode());
        }
        c.marker("namespace-modes.json", &modes)?;
    }
    let modes: Vec<u32> = read(&journal)?;
    if modes.len() != namespace_paths(c).len() {
        return Err("namespace journal mismatch".into());
    }
    for (path, mode) in namespace_paths(c).iter().zip(modes) {
        safe(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(mode & !0o222)).map_err(err)?;
    }
    Ok(())
}
fn check_namespaces(c: &Context) -> Result<()> {
    if !c.op.join("namespace-modes.json").exists() {
        return Ok(());
    }
    for path in namespace_paths(c) {
        safe(&path)?;
        if fs::metadata(path).map_err(err)?.permissions().mode() & 0o222 != 0 {
            return Err("writer namespace fence changed".into());
        }
    }
    Ok(())
}
fn unseal_namespaces(c: &Context) -> Result<()> {
    let journal = c.op.join("namespace-modes.json");
    if !journal.exists() {
        return Ok(());
    }
    let paths = namespace_paths(c);
    let modes: Vec<u32> = read(&journal)?;
    if paths.len() != modes.len() {
        return Err("namespace journal mismatch".into());
    }
    for (path, mode) in paths.iter().zip(modes).rev() {
        safe(path)?;
        fs::set_permissions(path, fs::Permissions::from_mode(mode)).map_err(err)?;
    }
    Ok(())
}

fn rename(from: &Path, to: &Path) -> Result<()> {
    safe(from)?;
    safe(to)?;
    fs::create_dir_all(to.parent().ok_or("missing parent")?).map_err(err)?;
    fs::rename(from, to).map_err(err)?;
    sync(from.parent().ok_or("missing parent")?)?;
    sync(to.parent().ok_or("missing parent")?)
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Record {
    pub issue: u64,
    pub checkout: PathBuf,
    pub checkout_head: String,
    pub checkout_branch: String,
    pub plan: Option<PathBuf>,
    pub plan_digest: Option<String>,
    pub semantic_digest: String,
    pub source_digest: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub schema: String,
    pub repository: String,
    pub primary: PathBuf,
    pub operation_id: String,
    pub operator: String,
    pub approval_reference: String,
    pub pause_expires_unix: u64,
    pub acknowledged_issues: Vec<u64>,
    pub candidate: PathBuf,
    pub candidate_blake3: String,
    pub candidate_source_revision: String,
    pub authority_blake3: String,
    pub owner_blake3: String,
    pub remote_digest: String,
    pub installed: PathBuf,
    pub prior_blake3: String,
    pub records: Vec<Record>,
}
#[derive(Clone)]
struct Context {
    request: Request,
    common: PathBuf,
    op: PathBuf,
    digest: String,
}
impl Context {
    fn load(path: &Path) -> Result<Self> {
        let request: Request = read(path)?;
        Self::from_request(request)
    }
    fn from_request(request: Request) -> Result<Self> {
        if request.schema != "csdlc.v3.live_transition.v1"
            || request.repository != "agent-logic/agent-design-language"
            || request.operation_id.is_empty()
            || request.operation_id.len() > 80
            || !request
                .operation_id
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || c == b'-' || c == b'_')
            || request.operator.trim().is_empty()
            || request.approval_reference.trim().is_empty()
        {
            return Err("invalid transition identity or missing explicit operator decision".into());
        }
        safe(&request.primary)?;
        let common = PathBuf::from(git(
            &request.primary,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?);
        if common != request.primary.join(".git") {
            return Err("primary checkout required".into());
        }
        if canonical_v3_authority(&request.primary)?.is_none() {
            return Err("canonical native authority unavailable".into());
        }
        let issues = request
            .records
            .iter()
            .map(|r| r.issue)
            .collect::<BTreeSet<_>>();
        if issues.is_empty()
            || issues.contains(&0)
            || issues.len() != request.records.len()
            || request
                .acknowledged_issues
                .iter()
                .copied()
                .collect::<BTreeSet<_>>()
                != issues
            || request.acknowledged_issues.len() != issues.len()
        {
            return Err("incomplete owner acknowledgments or duplicate census".into());
        }
        for p in [&request.candidate, &request.installed] {
            safe(p)?;
            if !p.is_absolute() || !p.is_file() {
                return Err("absolute regular executable paths required".into());
            }
        }
        if request.installed != request.primary.join(".adl/bin/native-v3/csdlc") {
            return Err("canonical stable native-v3 destination required".into());
        }
        if hash(&std::env::current_exe().map_err(err)?)? != request.owner_blake3 {
            return Err("transition owner executable changed".into());
        }
        if hash(&request.candidate)? != request.candidate_blake3 {
            return Err("qualified candidate identity changed".into());
        }
        if request.candidate_blake3 != QUALIFIED_BINARY
            || request.candidate_source_revision != QUALIFIED_SOURCE
        {
            return Err("candidate is not the SIM07-qualified executable/source pair".into());
        }
        if hash(&request.primary.join(SELECTOR_PATH))? != request.authority_blake3 {
            return Err("authority changed".into());
        }
        if request.candidate_source_revision.len() != 40
            || !request
                .candidate_source_revision
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return Err("qualified source revision required".into());
        }
        let digest = blake3::hash(&bytes(&request)?).to_hex().to_string();
        let op = common
            .join("csdlc-v3/local/live-transitions")
            .join(&request.operation_id);
        safe(&op)?;
        Ok(Self {
            request,
            common,
            op,
            digest,
        })
    }
    fn legacy(&self) -> impl Iterator<Item = &Record> {
        self.request
            .records
            .iter()
            .filter(|r| r.semantic_digest == "absent")
    }
    fn live(&self, id: u64) -> PathBuf {
        self.common
            .join("csdlc-v3/semantic/issues")
            .join(id.to_string())
    }
    fn stage(&self) -> PathBuf {
        self.op.join("stage.git")
    }
    fn staged(&self, id: u64) -> PathBuf {
        self.stage()
            .join("csdlc-v3/semantic/issues")
            .join(id.to_string())
    }
    fn source(&self, r: &Record) -> Result<PathBuf> {
        safe(&r.checkout)?;
        if Path::new(&git(
            &r.checkout,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?) != self.common
        {
            return Err("foreign checkout".into());
        }
        Ok(if r.checkout == self.request.primary {
            self.common
                .join("csdlc-v3/local/issues")
                .join(r.issue.to_string())
        } else {
            r.checkout.join(".csdlc/issues").join(r.issue.to_string())
        })
    }
    fn projection(&self, r: &Record) -> PathBuf {
        if r.checkout == self.request.primary {
            self.common
                .join("csdlc-v3/local/projections")
                .join(r.issue.to_string())
        } else {
            r.checkout
                .join(".csdlc/v3/issues")
                .join(r.issue.to_string())
        }
    }
    fn marker(&self, name: &str, v: &impl Serialize) -> Result<()> {
        put(&self.op.join(name), &bytes(v)?)
    }
    fn check_sources(&self) -> Result<()> {
        check_namespaces(self)?;
        require_no_pending_native(self)?;
        let actual = census(&self.request.primary, &self.common)?;
        let declared = self
            .request
            .records
            .iter()
            .map(|r| r.issue)
            .collect::<BTreeSet<_>>();
        if actual != declared {
            return Err(format!(
                "incomplete census: observed {actual:?}, declared {declared:?}"
            ));
        }
        if remote_fingerprint(&self.common.join("csdlc-v3/remote"))? != self.request.remote_digest {
            return Err(
                "remote state changed or effects are uncertain; automatic restore prohibited"
                    .into(),
            );
        }
        for r in &self.request.records {
            if git(&r.checkout, &["rev-parse", "HEAD"])? != r.checkout_head
                || git(&r.checkout, &["symbolic-ref", "--quiet", "--short", "HEAD"])?
                    != r.checkout_branch
            {
                return Err("checkout head or branch changed during transition".into());
            }
            if fingerprint(&self.source(r)?)? != r.source_digest {
                return Err(format!("source changed for {}", r.issue));
            }
            if r.semantic_digest != "absent"
                && fingerprint(&self.live(r.issue))? != r.semantic_digest
            {
                return Err(format!("passthrough state changed for {}", r.issue));
            }
            if let Some(plan) = &r.plan {
                if Some(hash(plan)?) != r.plan_digest {
                    return Err("retained plan changed".into());
                }
            }
        }
        Ok(())
    }
}

struct Census {
    issues: BTreeSet<u64>,
    excluded_history: Vec<Value>,
}
fn issue_from_card_path(path: &str) -> Option<u64> {
    path.strip_prefix(".csdlc/issues/")?
        .split('/')
        .next()?
        .parse()
        .ok()
}
fn census_inventory(primary: &Path, common: &Path) -> Result<Census> {
    let mut issues = BTreeSet::new();
    for root in [
        common.join("csdlc-v3/local/issues"),
        common.join("csdlc-v3/semantic/issues"),
    ] {
        safe(&root)?;
        if !root.exists() {
            continue;
        }
        for entry in fs::read_dir(&root).map_err(err)? {
            let entry = entry.map_err(err)?;
            safe(&entry.path())?;
            if let Ok(id) = entry.file_name().to_string_lossy().parse::<u64>() {
                if id > 0 && entry.file_type().map_err(err)?.is_dir() {
                    issues.insert(id);
                }
            }
        }
    }
    // Bound legacy records live in their registered worktree, not necessarily
    // under the primary native issue directory. Retained bindings remain part
    // of the census even when a missing checkout needs recovery.
    let bindings = common.join("csdlc-v3/local/bindings");
    safe(&bindings)?;
    if bindings.exists() {
        for entry in fs::read_dir(&bindings).map_err(err)? {
            let path = entry.map_err(err)?.path();
            safe(&path)?;
            if path.extension().is_none_or(|s| s != "json") {
                continue;
            }
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .and_then(|s| s.parse::<u64>().ok())
                .filter(|id| *id > 0)
                .ok_or_else(|| format!("invalid native binding path: {}", path.display()))?;
            let binding: Value = read(&path)
                .map_err(|error| format!("issue {id} binding {}: {error}", path.display()))?;
            if binding["schema"] != "csdlc.v3.binding.v1"
                || binding["issue"] != id
                || binding["worktree"].as_str().is_none_or(str::is_empty)
            {
                return Err(format!(
                    "issue {id}: invalid native binding {}",
                    path.display()
                ));
            }
            issues.insert(id);
        }
    }
    let mut excluded_history = Vec::new();
    let mut tracked_by_head: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for block in git(primary, &["worktree", "list", "--porcelain"])?.split("\n\n") {
        let Some(checkout) = block.lines().find_map(|s| s.strip_prefix("worktree ")) else {
            continue;
        };
        let checkout = Path::new(checkout);
        let root = checkout.join(".csdlc/issues");
        safe(&root)?;
        if !root.exists() {
            continue;
        }
        let mut unknown = Vec::new();
        for entry in fs::read_dir(&root).map_err(err)? {
            let entry = entry.map_err(err)?;
            safe(&entry.path())?;
            if let Ok(id) = entry.file_name().to_string_lossy().parse::<u64>() {
                if id > 0 && entry.file_type().map_err(err)?.is_dir() && !issues.contains(&id) {
                    unknown.push(id);
                }
            }
        }
        if unknown.is_empty() {
            continue;
        }
        let head = block
            .lines()
            .find_map(|s| s.strip_prefix("HEAD "))
            .ok_or_else(|| {
                format!(
                    "cannot classify historical cards in {}: missing registered HEAD",
                    checkout.display()
                )
            })?;
        if !tracked_by_head.contains_key(head) {
            let tracked = git(
                primary,
                &[
                    "ls-tree",
                    "-r",
                    "--name-only",
                    "-z",
                    head,
                    "--",
                    ".csdlc/issues",
                ],
            )?;
            tracked_by_head.insert(
                head.into(),
                tracked
                    .split('\0')
                    .filter(|p| !p.is_empty())
                    .map(str::to_owned)
                    .collect(),
            );
        }
        let tracked = &tracked_by_head[head];
        let changed = git(
            checkout,
            &[
                "diff",
                "--no-ext-diff",
                "--no-textconv",
                "--name-only",
                "-z",
                "HEAD",
                "--",
                ".csdlc/issues",
            ],
        )?;
        // Include ignored residue too: exclusion requires an entirely unchanged
        // historical projection, not merely a clean default Git status.
        let untracked = git(
            checkout,
            &["ls-files", "--others", "-z", "--", ".csdlc/issues"],
        )?;
        let changed: BTreeSet<u64> = changed
            .split('\0')
            .chain(untracked.split('\0'))
            .filter_map(issue_from_card_path)
            .collect();
        unknown.sort_unstable();
        for id in &unknown {
            let path = root.join(id.to_string());
            if !tracked.contains(&format!(".csdlc/issues/{id}/index.json")) || changed.contains(id)
            {
                return Err(format!("issue {id}: unregistered or changed lifecycle residue at {}; resolve native ownership before transition", path.display()));
            }
        }
        excluded_history.push(json!({"issues":unknown,"checkout":checkout,"revision":head,"disposition":"tracked_historical_projection","basis":"committed indexes; unchanged tracked files; no untracked residue; no native/semantic/binding records"}));
    }
    Ok(Census {
        issues,
        excluded_history,
    })
}
fn census(primary: &Path, common: &Path) -> Result<BTreeSet<u64>> {
    Ok(census_inventory(primary, common)?.issues)
}
/// Census only: no writer locks, request construction, or lifecycle effects.
pub fn inventory(primary: &Path) -> Result<Value> {
    let primary = fs::canonicalize(primary).map_err(err)?;
    let common = PathBuf::from(git(
        &primary,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?);
    if common != primary.join(".git") {
        return Err("primary checkout required".into());
    }
    let census = census_inventory(&primary, &common)?;
    let root =
        SemanticRoot::from_git_common(&common, "agent-logic/agent-design-language").map_err(err)?;
    let mut rows = Vec::new();
    for issue in census.issues {
        let key = IssueKey::new("agent-logic/agent-design-language", issue).map_err(err)?;
        let observation = Store::observe_issue(&root, &key).map_err(err)?;
        let native = common.join("csdlc-v3/local/issues").join(issue.to_string());
        let row = match observation {
            Observation::Current(s) if !s.projection_required() && s.pending().is_none() => {
                let projection = root.projection_path(&s).map_err(err)?;
                let checkout = if projection.starts_with(&common) {
                    primary.clone()
                } else {
                    s.inputs()
                        .binding()
                        .ok_or("missing binding")?
                        .worktree
                        .clone()
                };
                json!({"issue":issue,"checkout":checkout,"plan":null,"disposition":"preserve_current"})
            }
            Observation::LegacyMigrationRequired => {
                let binding_path = common.join(format!("csdlc-v3/local/bindings/{issue}.json"));
                let (checkout, source) = if binding_path.exists() {
                    let binding: Value = read(&binding_path)?;
                    let checkout = PathBuf::from(
                        binding["worktree"]
                            .as_str()
                            .ok_or("missing native checkout")?,
                    );
                    let source = checkout.join(format!(".csdlc/issues/{issue}/index.json"));
                    (checkout, source)
                } else {
                    (primary.clone(), native.join("index.json"))
                };
                let index: Value = read(&source).map_err(|error| {
                    format!(
                        "issue {issue}: cannot read live native record {}: {error}",
                        source.display()
                    )
                })?;
                if index["issue"] != issue || (checkout == primary && index["phase"] != "ready") {
                    return Err(format!(
                        "issue {issue}: native identity/phase does not match source {}",
                        source.display()
                    ));
                }
                json!({"issue":issue,"checkout":checkout,"retained_plan_required":true,"native_phase":index["phase"],"disposition":"requires_native_admission"})
            }
            _ => json!({"issue":issue,"disposition":"recovery_required_before_transition"}),
        };
        rows.push(row);
    }
    Ok(
        json!({"schema":"csdlc.v3.live_transition_inventory.v1","repository":"agent-logic/agent-design-language","records":rows,"excluded_history":census.excluded_history,"read_only":true}),
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecordSpec {
    issue: u64,
    checkout: PathBuf,
    plan: Option<PathBuf>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RequestSpec {
    primary: PathBuf,
    operation_id: String,
    operator: String,
    approval_reference: String,
    pause_expires_unix: u64,
    acknowledged_issues: Vec<u64>,
    candidate: PathBuf,
    records: Vec<RecordSpec>,
}

/// Only a create-only request file is written. Authorization is supplied by the
/// operator, while identities and digests come from the real local repository.
pub fn build_request(spec: &Path, output: &Path) -> Result<Value> {
    let s: RequestSpec = read(spec)?;
    let primary = fs::canonicalize(&s.primary).map_err(err)?;
    let common = PathBuf::from(git(
        &primary,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?);
    let installed = primary.join(".adl/bin/native-v3/csdlc");
    let mut records = Vec::new();
    for r in s.records {
        let checkout = fs::canonicalize(&r.checkout).map_err(err)?;
        if git(&checkout, &["rev-parse", "--show-toplevel"])? != checkout.to_string_lossy() {
            return Err("exact checkout root required".into());
        }
        let source = if checkout == primary {
            common
                .join("csdlc-v3/local/issues")
                .join(r.issue.to_string())
        } else {
            checkout.join(".csdlc/issues").join(r.issue.to_string())
        };
        let semantic_digest = fingerprint(
            &common
                .join("csdlc-v3/semantic/issues")
                .join(r.issue.to_string()),
        )?;
        let plan = r
            .plan
            .map(|p| fs::canonicalize(p).map_err(err))
            .transpose()?;
        if semantic_digest == "absent" && plan.is_none() {
            return Err(format!(
                "legacy issue {} requires its retained intent plan",
                r.issue
            ));
        }
        let plan_digest = plan.as_deref().map(hash).transpose()?;
        records.push(Record {
            issue: r.issue,
            checkout_head: git(&checkout, &["rev-parse", "HEAD"])?,
            checkout_branch: git(&checkout, &["symbolic-ref", "--quiet", "--short", "HEAD"])?,
            checkout,
            plan,
            plan_digest,
            semantic_digest,
            source_digest: fingerprint(&source)?,
        });
    }
    let candidate = fs::canonicalize(s.candidate).map_err(err)?;
    let request = Request {
        schema: "csdlc.v3.live_transition.v1".into(),
        repository: "agent-logic/agent-design-language".into(),
        owner_blake3: hash(&std::env::current_exe().map_err(err)?)?,
        remote_digest: remote_fingerprint(&common.join("csdlc-v3/remote"))?,
        authority_blake3: hash(&primary.join(SELECTOR_PATH))?,
        prior_blake3: hash(&installed)?,
        candidate_blake3: hash(&candidate)?,
        primary,
        installed,
        candidate,
        candidate_source_revision: QUALIFIED_SOURCE.into(),
        operation_id: s.operation_id,
        operator: s.operator,
        approval_reference: s.approval_reference,
        pause_expires_unix: s.pause_expires_unix,
        acknowledged_issues: s.acknowledged_issues,
        records,
    };
    let c = Context::from_request(request)?;
    c.check_sources()?;
    if output.exists() {
        return Err("request output must be new".into());
    }
    put(output, &bytes(&c.request)?)?;
    Ok(
        json!({"status":"request_constructed","request_digest":c.digest,"request":output,"lifecycle_effects":"none","admission":"not_yet_checked"}),
    )
}

fn snapshot(o: CommitOutcome) -> Box<Snapshot> {
    match o {
        CommitOutcome::Committed(s) | CommitOutcome::Unchanged(s) => s,
    }
}

/// Pure admission plus the existing native compatibility census; no generalized
/// history conversion is inferred. Unsupported legacy phases fail before staging.
fn inputs(c: &Context, r: &Record, fence: &NativeWriterFenceGuard) -> Result<IssueInputs> {
    let source = c.source(r)?;
    let index: Value = read(&source.join("index.json"))?;
    let plan: AcceptedIntentPlan = read(
        r.plan
            .as_ref()
            .ok_or("legacy record requires its retained intent plan")?,
    )?;
    for (kind, values) in &plan.cards {
        if *values != read::<Value>(&source.join("cards").join(format!("{kind}.values.json")))? {
            return Err(format!("retained plan differs from {kind}"));
        }
    }
    let binding = if r.checkout == c.request.primary {
        None
    } else {
        let branch = git(&r.checkout, &["symbolic-ref", "--quiet", "--short", "HEAD"])?;
        if index["branch"].as_str() != Some(&branch) {
            return Err("bound branch changed".into());
        }
        Some(Binding {
            branch,
            head: git(&r.checkout, &["rev-parse", "HEAD"])?,
            worktree: r.checkout.clone(),
            registration: "git-worktree-list".into(),
        })
    };
    let title = plan
        .cards
        .get("sip")
        .and_then(|v| v["title"].as_str())
        .ok_or("source title missing")?
        .to_string();
    let steps = source_plan(&plan.cards)?;
    let authority =
        Digest::authority(&fs::read(c.request.primary.join(SELECTOR_PATH)).map_err(err)?);
    let inputs = IssueInputs::new(title, plan, steps, binding, authority).map_err(err)?;
    let root =
        SemanticRoot::from_git_common(&c.common, c.request.repository.clone()).map_err(err)?;
    let key = IssueKey::new(c.request.repository.clone(), r.issue).map_err(err)?;
    Store::validate_legacy_native_issue_under_writer_fence(&root, &key, &inputs, fence)
        .map_err(err)?;
    Ok(inputs)
}

// Guardians own the file descriptors; a killed command cannot release the pause.
// Readiness is authenticated by challenge AND lock inode/readback, not a PID file.
#[derive(Serialize, Deserialize)]
struct Ready {
    phase: String,
    digest: String,
    port: u16,
    files: Vec<(PathBuf, u64, u64)>,
}
fn locks(c: &Context, phase: &str) -> Result<Vec<PathBuf>> {
    match phase {
        "native" => {
            let mut p = c
                .request
                .records
                .iter()
                .map(|r| {
                    c.common
                        .join("csdlc-v3/local/locks")
                        .join(format!("{}.lock", r.issue))
                })
                .collect::<Vec<_>>();
            p.extend(
                c.request
                    .records
                    .iter()
                    .filter(|r| r.semantic_digest != "absent")
                    .map(|r| c.live(r.issue).join("state.lock")),
            );
            p.push(c.common.join("csdlc-v3/semantic/issues/state.lock"));
            Ok(p)
        }
        "staged" => Ok(c
            .legacy()
            .map(|r| c.staged(r.issue).join("state.lock"))
            .collect()),
        _ => Err("invalid guardian phase".into()),
    }
}
fn held(c: &Context, phase: &str) -> Result<bool> {
    let p = c.op.join(format!("{phase}-ready.json"));
    if !p.exists() {
        return Ok(false);
    }
    let ready: Ready = read(&p)?;
    if ready.digest != c.digest
        || ready.phase != phase
        || ready
            .files
            .iter()
            .map(|(p, _, _)| p.clone())
            .collect::<Vec<_>>()
            != locks(c, phase)?
    {
        return Err("guardian request/phase/lock denominator changed".into());
    }
    let s = UdpSocket::bind(("127.0.0.1", 0)).map_err(err)?;
    s.set_read_timeout(Some(Duration::from_millis(300)))
        .map_err(err)?;
    s.connect(("127.0.0.1", ready.port)).map_err(err)?;
    let challenge = format!(
        "{}:{phase}:{}",
        c.digest,
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(err)?
            .as_nanos()
    );
    s.send(challenge.as_bytes()).map_err(err)?;
    let mut b = [0; 128];
    let n = s
        .recv(&mut b)
        .map_err(|_| "guardian lost; preserve pause and require reconciliation".to_owned())?;
    if b[..n] != *blake3::hash(challenge.as_bytes()).to_hex().as_bytes() {
        return Err("guardian challenge failed".into());
    }
    for (p, dev, ino) in ready.files {
        let p = if p.exists() {
            p
        } else if phase == "staged" {
            let id = p
                .parent()
                .and_then(Path::file_name)
                .ok_or("invalid lock path")?;
            let live = c
                .common
                .join("csdlc-v3/semantic/issues")
                .join(id)
                .join("state.lock");
            if live.exists() {
                live
            } else {
                c.op.join(format!("restored-semantic-{}", id.to_string_lossy()))
                    .join("state.lock")
            }
        } else {
            return Err("missing native fence".into());
        };
        safe(&p)?;
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .open(p)
            .map_err(err)?;
        let m = f.metadata().map_err(err)?;
        if m.dev() != dev || m.ino() != ino {
            return Err("writer lock inode changed".into());
        }
        match f.try_lock_exclusive() {
            Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => (),
            _ => return Err("writer fence not held".into()),
        }
    }
    Ok(true)
}
fn fence(c: &Context, phase: &str) -> Result<()> {
    if held(c, phase)? {
        return Ok(());
    }
    #[cfg(test)]
    {
        let context = c.clone();
        let phase = phase.to_owned();
        std::thread::spawn(move || run_guardian(context, &phase).unwrap());
    }
    #[cfg(not(test))]
    Command::new(std::env::current_exe().map_err(err)?)
        .args(["__guardian", "--request"])
        .arg(c.op.join("request.json"))
        .args(["--phase", phase])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .process_group(0)
        .spawn()
        .map_err(err)?;
    let until = Instant::now() + Duration::from_secs(10);
    while Instant::now() < until {
        if held(c, phase)? {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    Err("guardian did not acquire complete writer fence; no conversion admitted".into())
}
pub fn guardian(request: &Path, phase: &str) -> Result<()> {
    let c = Context::load(request)?;
    if request != c.op.join("request.json") {
        return Err("guardian requires retained operation request".into());
    }
    run_guardian(c, phase)
}
fn run_guardian(c: Context, phase: &str) -> Result<()> {
    let mut files = Vec::new();
    let mut identities = Vec::new();
    for p in locks(&c, phase)? {
        safe(&p)?;
        fs::create_dir_all(p.parent().ok_or("missing parent")?).map_err(err)?;
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .open(&p)
            .map_err(err)?;
        let issue_semantic = phase == "staged"
            || (p.starts_with(c.common.join("csdlc-v3/semantic/issues"))
                && p.parent() != Some(c.common.join("csdlc-v3/semantic/issues").as_path()));
        if issue_semantic {
            FileExt::try_lock_shared(&f)
        } else {
            f.try_lock_exclusive()
        }
        .map_err(|_| "writer still active; drain it before retrying".to_owned())?;
        let m = f.metadata().map_err(err)?;
        identities.push((p, m.dev(), m.ino()));
        files.push(f);
    }
    let s = UdpSocket::bind(("127.0.0.1", 0)).map_err(err)?;
    s.set_read_timeout(Some(Duration::from_millis(200)))
        .map_err(err)?;
    c.marker(
        &format!("{phase}-ready.json"),
        &Ready {
            phase: phase.into(),
            digest: c.digest.clone(),
            port: s.local_addr().map_err(err)?.port(),
            files: identities,
        },
    )?;
    loop {
        let release = c.op.join(format!("release-{phase}.json"));
        if release.exists() && read::<Value>(&release)?["request_digest"] == c.digest {
            break;
        }
        let mut b = [0; 512];
        if let Ok((n, peer)) = s.recv_from(&mut b) {
            if peer.ip().is_loopback()
                && b[..n].starts_with(format!("{}:{phase}:", c.digest).as_bytes())
            {
                s.send_to(blake3::hash(&b[..n]).to_hex().as_bytes(), peer)
                    .map_err(err)?;
            }
        }
    }
    drop(files);
    c.marker(
        &format!("{phase}-released.json"),
        &json!({"request_digest":c.digest}),
    )?;
    Ok(())
}

fn retained_fence(c: &Context) -> Result<NativeWriterFenceGuard> {
    if !held(c, "native")? {
        return Err("native fence required".into());
    }
    NativeWriterFenceGuard::authenticated_guardian(
        &c.common,
        c.request.records.iter().map(|r| r.issue),
    )
    .map_err(err)
}
fn terminal(c: &Context) -> Result<()> {
    if c.op.join("resumed.json").exists() || c.op.join("restored.json").exists() {
        return Err("operation is terminal; no automatic rollback or reuse".into());
    }
    Ok(())
}

pub fn execute(action: &str, request: &Path, decision: Option<&Path>) -> Result<Value> {
    let c = Context::load(request)?;
    let retained = c.op.join("request.json");
    if retained.exists() && bytes(&read::<Request>(&retained)?)? != bytes(&c.request)? {
        return Err("frozen request identity changed".into());
    }
    if action == "status" {
        return Ok(
            json!({"operation_id":c.request.operation_id,"request_digest":c.digest,
            "candidate_blake3":c.request.candidate_blake3,
            "conversion_blake3":if c.op.join("converted.json").exists() { Some(hash(&c.op.join("converted.json"))?) } else {None},
            "converted":c.op.join("converted.json").exists(),"resumed":c.op.join("resumed.json").exists(),
            "restored":c.op.join("restored.json").exists(),"operation_directory":c.op}),
        );
    }
    if !["fence", "convert", "restore", "resume", "verify"].contains(&action) {
        return Err("unknown action".into());
    }
    if ["fence", "convert"].contains(&action)
        && SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(err)?
            .as_secs()
            >= c.request.pause_expires_unix
    {
        return Err("pause decision expired; restore remains available".into());
    }
    fs::create_dir_all(&c.op).map_err(err)?;
    let command_lock = c.op.join("command.lock");
    safe(&command_lock)?;
    let command_lock = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(command_lock)
        .map_err(err)?;
    command_lock
        .try_lock_exclusive()
        .map_err(|_| "another transition command is running".to_owned())?;
    if c.op.join("resumed.json").exists() && action == "resume"
        || c.op.join("restored.json").exists() && action == "restore"
    {
        release(&c)?;
        return Ok(json!({"status":"terminal_release_reconciled"}));
    }
    terminal(&c)?;
    if c.op.join("restoring.json").exists() && action != "restore" {
        return Err("restore is in progress; resume the same restore operation".into());
    }
    if action == "fence" {
        if SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(err)?
            .as_secs()
            >= c.request.pause_expires_unix
        {
            return Err("pause decision expired".into());
        }
        if hash(&c.request.installed)? != c.request.prior_blake3 {
            return Err("installed preimage changed".into());
        }
        let root =
            SemanticRoot::from_git_common(&c.common, c.request.repository.clone()).map_err(err)?;
        for r in &c.request.records {
            if fingerprint(&c.live(r.issue))? != r.semantic_digest {
                return Err("semantic census changed".into());
            }
            if r.semantic_digest != "absent" {
                let key = IssueKey::new(c.request.repository.clone(), r.issue).map_err(err)?;
                match Store::observe_issue(&root, &key).map_err(err)? {
                    Observation::Current(s)
                        if !s.projection_required()
                            && s.pending().is_none()
                            && root.projection_path(&s).map_err(err)?.parent()
                                == Some(c.projection(r).as_path()) => {}
                    _ => {
                        return Err(
                            "current record is pending or unhealthy; reconcile before transition"
                                .into(),
                        )
                    }
                }
            }
        }
        c.check_sources()?;
        c.marker("request.json", &c.request)?;
        fence(&c, "native")?;
        seal_namespaces(&c)?;
        c.check_sources()?;
        crate::commands::remote::require_settled_transition_remote(
            &c.common.join("csdlc-v3/remote"),
        )
        .map_err(err)?;
        let guard = retained_fence(&c)?;
        for r in c.legacy() {
            let input = inputs(&c, r, &guard)?;
            c.marker(&format!("inputs-{}.json", r.issue), &input)?;
        }
        c.marker(
            "admitted.json",
            &json!({"request_digest":c.digest,"native_history":"preserved","remote_effects":0}),
        )?;
        return Ok(json!({"status":"fenced","request_digest":c.digest,"operation_directory":c.op}));
    }
    if !c.op.join("request.json").exists()
        || read::<Request>(&c.op.join("request.json")).and_then(|r| bytes(&r))?
            != bytes(&c.request)?
    {
        return Err("frozen request required".into());
    }
    let guard = retained_fence(&c)?;
    if action == "restore" && !c.op.join("conversion-started.json").exists() {
        c.marker(
            "restored.json",
            &json!({"request_digest":c.digest,"status":"pause_aborted_before_conversion"}),
        )?;
        release(&c)?;
        return Ok(json!({"status":"pause_aborted_before_conversion"}));
    }
    c.check_sources()?;
    match action {
        "convert" => convert_live(&c, &guard),
        "verify" => {
            verify_final(&c)?;
            Ok(json!({"status":"verified_paused","request_digest":c.digest}))
        }
        "restore" => restore_live(&c),
        "resume" => resume_live(&c, decision.ok_or("fresh resume decision required")?),
        _ => Err("expected fence, convert, status, restore or resume".into()),
    }
}

fn convert_live(c: &Context, guard: &NativeWriterFenceGuard) -> Result<Value> {
    if !c.op.join("admitted.json").exists() {
        return Err("whole-census admission required".into());
    }
    if c.op.join("converted.json").exists() {
        verify_final(c)?;
        return Ok(json!({"status":"converted_paused","replay":true}));
    }
    let mut all = Vec::new();
    for r in c.legacy() {
        let input: IssueInputs = read(&c.op.join(format!("inputs-{}.json", r.issue)))?;
        // The admission record was sealed under this still-authenticated native fence.
        if input.cards()
            != &read::<AcceptedIntentPlan>(r.plan.as_ref().ok_or("missing plan")?)?.cards
        {
            return Err("admitted cards changed".into());
        }
        all.push(input);
    }
    c.marker(
        "conversion-started.json",
        &json!({"request_digest":c.digest}),
    )?;
    let _ = guard;
    let stage = c.stage();
    if !stage.exists()
        && !Command::new("git")
            .args(["init", "--bare", "--quiet"])
            .arg(&stage)
            .status()
            .map_err(err)?
            .success()
    {
        return Err("staging Git initialization failed".into());
    }
    let root = SemanticRoot::from_git_common(&stage, c.request.repository.clone()).map_err(err)?;
    let registry = PromptRegistry::from_current_json(
        &fs::read(
            c.request
                .primary
                .join("docs/templates/prompts/current.json"),
        )
        .map_err(err)?,
    )
    .map_err(err)?;
    let binary_backup = c.op.join("before/csdlc");
    if !binary_backup.exists() {
        if hash(&c.request.installed)? != c.request.prior_blake3 {
            return Err("missing prior executable snapshot".into());
        }
        put(
            &binary_backup,
            &fs::read(&c.request.installed).map_err(err)?,
        )?;
        fs::set_permissions(
            &binary_backup,
            fs::metadata(&c.request.installed)
                .map_err(err)?
                .permissions(),
        )
        .map_err(err)?;
    }
    if hash(&binary_backup)? != c.request.prior_blake3 {
        return Err("prior executable snapshot changed".into());
    }
    if !c.op.join("staged-ready.json").exists() {
        for (r, input) in c.legacy().zip(all) {
            let source = c.source(r)?;
            let projection = c.projection(r);
            let before = c.op.join(format!("before/projection-{}", r.issue));
            let pre = c.op.join(format!("before-{}.json", r.issue));
            if !pre.exists() {
                if projection.exists() {
                    tree(&projection, &before)?;
                }
                c.marker(
                    &format!("before-{}.json", r.issue),
                    &json!({"projection":fingerprint(&projection)?}),
                )?;
            }
            let key = IssueKey::new(c.request.repository.clone(), r.issue).map_err(err)?;
            let current = match Store::observe_issue(&root, &key).map_err(err)? {
                Observation::Absent => {
                    let index: Value = read(&source.join("index.json"))?;
                    let phase = if input.binding().is_some() {
                        crate::lifecycle::LifecycleState::Bound
                    } else {
                        crate::lifecycle::LifecycleState::Ready
                    };
                    snapshot(
                        Store::convert_copied_issue(
                            &root,
                            CopiedRecordConversion {
                                key: key.clone(),
                                inputs: input,
                                phase,
                                source_generation: index["generation"]
                                    .as_u64()
                                    .ok_or("missing native generation")?,
                                source_digest: source_digest(&source)?,
                            },
                        )
                        .map_err(err)?,
                    )
                }
                Observation::Current(s) | Observation::ProjectionRepairRequired(s) => {
                    if s.inputs() != &input {
                        return Err("staged input changed".into());
                    }
                    s
                }
                _ => {
                    return Err(
                        "staged operation requires restore or classified native recovery".into(),
                    )
                }
            };
            if current.projection_required() {
                let bundle = copied_card_projection(&current, &registry, &source)?;
                let proof = Store::write_card_projection(&root, &current, bundle).map_err(err)?;
                let next = snapshot(
                    Store::commit_issue_local(
                        &root,
                        Admission::new(
                            key,
                            current.version().clone(),
                            current.inputs().authority().clone(),
                        ),
                        LocalChange::AcknowledgeProjection(proof),
                    )
                    .map_err(err)?,
                );
                Store::write_issue_projection(&root, &next).map_err(err)?;
            }
        }
    }
    c.check_sources()?;
    fence(c, "staged")?;
    for r in c.legacy() {
        let staged = c.staged(r.issue);
        let live = c.live(r.issue);
        if staged.exists() && live.exists() {
            return Err("ambiguous stage/live identity".into());
        }
        if staged.exists() {
            rename(&staged, &live)?;
            checkpoint("semantic_activation")?;
        }
        if r.checkout == c.request.primary {
            let projection = c.projection(r);
            let staged_projection = stage
                .join("csdlc-v3/local/projections")
                .join(r.issue.to_string());
            if staged_projection.exists() {
                if projection.exists() {
                    rename(
                        &projection,
                        &c.op.join(format!("replaced-projection-{}", r.issue)),
                    )?;
                }
                rename(&staged_projection, &projection)?;
            }
        }
    }
    let installed = hash(&c.request.installed)?;
    if installed != c.request.prior_blake3 && installed != c.request.candidate_blake3 {
        return Err("installed preimage changed before activation".into());
    }
    snapshot_provenance(c)?;
    let next = c.request.installed.with_file_name(".csdlc-transition-next");
    put(&next, &fs::read(&c.request.candidate).map_err(err)?)?;
    fs::set_permissions(&next, fs::Permissions::from_mode(0o755)).map_err(err)?;
    rename(&next, &c.request.installed)?;
    checkpoint("binary_activation")?;
    install_provenance(c)?;
    let final_state=c.request.records.iter().map(|r|Ok(json!({"issue":r.issue,"semantic":fingerprint(&c.live(r.issue))?,"projection":fingerprint(&c.projection(r))?}))).collect::<Result<Vec<_>>>()?;
    c.marker("converted.json",&json!({"request_digest":c.digest,"candidate_blake3":hash(&c.request.installed)?,"records":final_state,"remote_effects":0,"converted_at_unix":SystemTime::now().duration_since(UNIX_EPOCH).map_err(err)?.as_secs()}))?;
    verify_final(c)?;
    Ok(json!({"status":"converted_paused","request_digest":c.digest,"operation_directory":c.op}))
}

fn verify_final(c: &Context) -> Result<()> {
    retained_fence(c)?;
    if !held(c, "staged")? {
        return Err("new writer fence required".into());
    }
    c.check_sources()?;
    if fingerprint(&provenance(c, "json"))?
        != blake3::hash(&installed_provenance(c)?).to_hex().to_string()
        || provenance(c, "sha256").exists()
    {
        return Err("installed provenance changed".into());
    }
    let result: Value = read(&c.op.join("converted.json"))?;
    if result["request_digest"] != c.digest
        || hash(&c.request.installed)? != c.request.candidate_blake3
    {
        return Err("candidate or conversion identity changed".into());
    }
    let root =
        SemanticRoot::from_git_common(&c.common, c.request.repository.clone()).map_err(err)?;
    for r in &c.request.records {
        let key = IssueKey::new(c.request.repository.clone(), r.issue).map_err(err)?;
        match Store::observe_issue(&root, &key).map_err(err)? {
            Observation::Current(s) if !s.projection_required() && s.pending().is_none() => (),
            _ => return Err("converted semantic/projection readback failed".into()),
        }
        let row = result["records"]
            .as_array()
            .ok_or("missing conversion denominator")?
            .iter()
            .find(|v| v["issue"] == r.issue)
            .ok_or("missing converted issue")?;
        if row["semantic"] != fingerprint(&c.live(r.issue))?
            || row["projection"] != fingerprint(&c.projection(r))?
        {
            return Err("post-conversion effect detected; automatic restore prohibited".into());
        }
    }
    Ok(())
}
fn provenance(c: &Context, suffix: &str) -> PathBuf {
    c.request
        .installed
        .parent()
        .expect("validated installed path")
        .join(".provenance")
        .join(format!("csdlc.{suffix}"))
}
fn installed_provenance(c: &Context) -> Result<Vec<u8>> {
    bytes(
        &json!({"schema":"csdlc.v3.transition_install_provenance.v1","binary":"csdlc",
        "artifact_blake3":c.request.candidate_blake3,"source_revision":c.request.candidate_source_revision,
        "request_digest":c.digest,"operation_id":c.request.operation_id,"installed_path":c.request.installed,
        "installer_source_cache":"invalidated; artifact installation did not build from the current checkout"}),
    )
}
fn snapshot_provenance(c: &Context) -> Result<()> {
    if c.op.join("provenance-before.json").exists() {
        return Ok(());
    }
    let mut before = serde_json::Map::new();
    for suffix in ["json", "sha256"] {
        let p = provenance(c, suffix);
        before.insert(suffix.into(), json!(fingerprint(&p)?));
        if p.exists() {
            let backup = c.op.join(format!("before/provenance-{suffix}"));
            put(&backup, &fs::read(&p).map_err(err)?)?;
            fs::set_permissions(&backup, fs::metadata(&p).map_err(err)?.permissions())
                .map_err(err)?;
        }
    }
    c.marker("provenance-before.json", &before)
}
fn replace(p: &Path, content: &[u8], mode: u32) -> Result<()> {
    safe(p)?;
    let next = p.with_file_name(format!(
        ".{}-transition-next",
        p.file_name().ok_or("missing filename")?.to_string_lossy()
    ));
    put(&next, content)?;
    fs::set_permissions(&next, fs::Permissions::from_mode(mode)).map_err(err)?;
    sync(&next)?;
    rename(&next, p)
}
fn install_provenance(c: &Context) -> Result<()> {
    let before: Value = read(&c.op.join("provenance-before.json"))?;
    let expected = installed_provenance(c)?;
    let json = provenance(c, "json");
    let observed = fingerprint(&json)?;
    if observed != before["json"] && observed != blake3::hash(&expected).to_hex().to_string() {
        return Err("foreign installed provenance".into());
    }
    replace(&json, &expected, 0o644)?;
    let cache = provenance(c, "sha256");
    if cache.exists() {
        if fingerprint(&cache)? != before["sha256"] {
            return Err("installer cache identity changed".into());
        }
        // Its bytes are retained in the restore snapshot. Leaving the old cache
        // digest would let a later installer falsely skip replacing this artifact.
        fs::remove_file(&cache).map_err(err)?;
        sync(cache.parent().ok_or("missing parent")?)?;
    }
    Ok(())
}
fn validate_provenance_restore(c: &Context) -> Result<()> {
    let p = c.op.join("provenance-before.json");
    if !p.exists() {
        return Ok(());
    }
    let before: Value = read(&p)?;
    for suffix in ["json", "sha256"] {
        let observed = fingerprint(&provenance(c, suffix))?;
        let installed = if suffix == "json" {
            blake3::hash(&installed_provenance(c)?).to_hex().to_string()
        } else {
            "absent".into()
        };
        if observed != before[suffix] && observed != installed {
            return Err("foreign provenance prevents restore".into());
        }
        if before[suffix] != "absent"
            && fingerprint(&c.op.join(format!("before/provenance-{suffix}")))? != before[suffix]
        {
            return Err("provenance snapshot changed".into());
        }
    }
    Ok(())
}
fn restore_provenance(c: &Context) -> Result<()> {
    let p = c.op.join("provenance-before.json");
    if !p.exists() {
        return Ok(());
    }
    let before: Value = read(&p)?;
    for suffix in ["json", "sha256"] {
        let target = provenance(c, suffix);
        if before[suffix] == "absent" {
            if target.exists() {
                fs::remove_file(&target).map_err(err)?;
                sync(target.parent().ok_or("missing parent")?)?;
            }
        } else {
            let backup = c.op.join(format!("before/provenance-{suffix}"));
            replace(
                &target,
                &fs::read(&backup).map_err(err)?,
                fs::metadata(backup).map_err(err)?.permissions().mode(),
            )?;
        }
        if fingerprint(&target)? != before[suffix] {
            return Err("provenance restore readback failed".into());
        }
    }
    Ok(())
}
fn release(c: &Context) -> Result<()> {
    unseal_namespaces(c)?;
    for phase in ["staged", "native"] {
        if c.op.join(format!("{phase}-ready.json")).exists() {
            c.marker(
                &format!("release-{phase}.json"),
                &json!({"request_digest":c.digest}),
            )?;
        }
    }
    let until = Instant::now() + Duration::from_secs(5);
    loop {
        let done = ["staged", "native"].iter().all(|phase| {
            !c.op.join(format!("{phase}-ready.json")).exists()
                || read::<Value>(&c.op.join(format!("{phase}-released.json")))
                    .is_ok_and(|v| v["request_digest"] == c.digest)
        });
        if done {
            return Ok(());
        }
        if Instant::now() >= until {
            return Err("terminal decision retained; fence release confirmation pending".into());
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}
fn restore_live(c: &Context) -> Result<Value> {
    let continuing = c.op.join("restoring.json").exists();
    if !continuing && c.op.join("converted.json").exists() {
        verify_final(c)?;
    } else if c.op.join("staged-ready.json").exists() && !held(c, "staged")? {
        return Err("staged guardian lost; restoration prohibited".into());
    }
    let observed = hash(&c.request.installed)?;
    if observed != c.request.prior_blake3 && observed != c.request.candidate_blake3 {
        return Err("foreign installed bytes; restore prohibited".into());
    }
    let backup_binary = c.op.join("before/csdlc");
    if observed != c.request.prior_blake3 && hash(&backup_binary)? != c.request.prior_blake3 {
        return Err("prior executable snapshot changed".into());
    }
    validate_provenance_restore(c)?;
    // Validate ALL restore material before changing any live path.
    for r in c.legacy() {
        let pre = c.op.join(format!("before-{}.json", r.issue));
        if pre.exists() {
            let v: Value = read(&pre)?;
            if v["projection"] != "absent"
                && fingerprint(&c.op.join(format!("before/projection-{}", r.issue)))?
                    != v["projection"]
            {
                return Err("restore snapshot changed".into());
            }
        }
    }
    c.marker("restoring.json", &json!({"request_digest":c.digest}))?;
    for r in c.legacy() {
        let pre = c.op.join(format!("before-{}.json", r.issue));
        if !pre.exists() {
            continue;
        }
        let before: Value = read(&pre)?;
        let projection = c.projection(r);
        let done = c.op.join(format!("restored-record-{}.json", r.issue));
        if done.exists() {
            if c.live(r.issue).exists() || fingerprint(&projection)? != before["projection"] {
                return Err("restored state changed; forward reconciliation required".into());
            }
            continue;
        }
        let intent_path = c.op.join(format!("restore-intent-{}.json", r.issue));
        if !intent_path.exists() {
            c.marker(&format!("restore-intent-{}.json",r.issue),&json!({"semantic":fingerprint(&c.live(r.issue))?,"projection":fingerprint(&projection)?}))?;
        }
        let intent: Value = read(&intent_path)?;
        let saved_semantic = c.op.join(format!("restored-semantic-{}", r.issue));
        if saved_semantic.exists() {
            if c.live(r.issue).exists() || fingerprint(&saved_semantic)? != intent["semantic"] {
                return Err("ambiguous restored semantic identity".into());
            }
        } else if c.live(r.issue).exists() {
            if fingerprint(&c.live(r.issue))? != intent["semantic"] {
                return Err("semantic state changed during restore".into());
            }
            rename(&c.live(r.issue), &saved_semantic)?;
            checkpoint("restore_semantic")?;
        }
        let saved_projection = c.op.join(format!("restored-projection-{}", r.issue));
        if saved_projection.exists() {
            if fingerprint(&saved_projection)? != intent["projection"] {
                return Err("saved projection changed".into());
            }
        } else if projection.exists() {
            if fingerprint(&projection)? != intent["projection"] {
                return Err("projection changed during restore".into());
            }
            rename(&projection, &saved_projection)?;
        }
        if before["projection"] != "absent" {
            tree(
                &c.op.join(format!("before/projection-{}", r.issue)),
                &projection,
            )?;
        }
        if fingerprint(&projection)? != before["projection"] {
            return Err("restore readback differs from original projection".into());
        }
        c.marker(&format!("restored-record-{}.json", r.issue), &before)?;
    }
    if observed != c.request.prior_blake3 {
        let next = c.request.installed.with_file_name(".csdlc-restore-next");
        put(&next, &fs::read(&backup_binary).map_err(err)?)?;
        fs::set_permissions(
            &next,
            fs::metadata(&backup_binary).map_err(err)?.permissions(),
        )
        .map_err(err)?;
        rename(&next, &c.request.installed)?;
    }
    restore_provenance(c)?;
    c.check_sources()?;
    c.marker(
        "restored.json",
        &json!({"request_digest":c.digest,"prior_blake3":hash(&c.request.installed)?}),
    )?;
    release(c)?;
    Ok(json!({"status":"restored","operation_directory":c.op}))
}
fn resume_live(c: &Context, path: &Path) -> Result<Value> {
    verify_final(c)?;
    let decision: Value = read(path)?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(err)?
        .as_secs();
    if decision["approved_at_unix"].as_u64().is_none_or(|t| {
        t == 0
            || t > now
            || t < read::<Value>(&c.op.join("converted.json"))
                .ok()
                .and_then(|v| v["converted_at_unix"].as_u64())
                .unwrap_or(u64::MAX)
    }) || decision["expires_unix"].as_u64().is_none_or(|t| t <= now)
        || decision["conversion_blake3"] != hash(&c.op.join("converted.json"))?
    {
        return Err("fresh conversion-bound resume decision required".into());
    }
    if decision["schema"] != "csdlc.v3.live_transition_resume.v1"
        || decision["request_digest"] != c.digest
        || decision["candidate_blake3"] != c.request.candidate_blake3
        || decision["operator"]
            .as_str()
            .is_none_or(|s| s.trim().is_empty())
        || decision["approval_reference"]
            .as_str()
            .is_none_or(|s| s.trim().is_empty())
    {
        return Err("explicit identity-bound resume decision required".into());
    }
    c.marker("resumed.json", &decision)?;
    release(c)?;
    Ok(json!({"status":"resumed","restore":"prohibited_after_resume","operation_directory":c.op}))
}

#[cfg(test)]
thread_local! {static STOP:std::cell::RefCell<Option<&'static str>>=const {std::cell::RefCell::new(None)};}
fn checkpoint(_name: &str) -> Result<()> {
    #[cfg(test)]
    if STOP.with(|s| *s.borrow() == Some(_name)) {
        return Err(format!("injected stop: {_name}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    // PVF tooling/unit; deterministic local Git/files/locks, no providers or live
    // repository. Exercises owner internals, not an operational runbook rehearsal.
    use super::*;
    struct Fixture(Context);
    impl Fixture {
        fn new() -> Self {
            Self::with_binding(false)
        }
        fn with_binding(bound: bool) -> Self {
            let nonce = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let primary = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("target/transition-unit")
                .join(format!(
                    "{}-{nonce}-{}",
                    std::process::id(),
                    WRITE_SEQUENCE.fetch_add(1, Ordering::Relaxed)
                ));
            fs::create_dir_all(primary.parent().unwrap()).unwrap();
            fs::create_dir(&primary).unwrap();
            git(&primary, &["init", "--quiet"]).unwrap();
            git(
                &primary,
                &[
                    "-c",
                    "user.name=Unit",
                    "-c",
                    "user.email=unit@example.invalid",
                    "commit",
                    "--allow-empty",
                    "-m",
                    "unit initial",
                ],
            )
            .unwrap();
            let common = primary.join(".git");
            let mut source = common.join("csdlc-v3/local/issues/875");
            let future = primary.join("future-worktree");
            let mut cards = serde_json::Map::new();
            for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
                let mut values = json!({"card":kind,"title":"Transition unit","issue":875,"repository":"agent-logic/agent-design-language","branch":"codex/875-transition-unit","worktree":future});
                if kind == "spp" {
                    for field in [
                        "dependencies_inline",
                        "repo_inputs_inline",
                        "target_files_surfaces_inline",
                        "deliverables_inline",
                        "validation_plan_inline",
                        "acceptance_criteria_inline",
                        "notes_risks_inline",
                    ] {
                        values[field] = json!("unit contract");
                    }
                }
                put(
                    &source.join(format!("cards/{kind}.values.json")),
                    &bytes(&values).unwrap(),
                )
                .unwrap();
                put(
                    &source.join(format!("cards/{kind}.md")),
                    b"retained native card\n",
                )
                .unwrap();
                cards.insert(kind.into(), values);
            }
            let plan = primary.join("plan.json");
            put(&plan,&bytes(&json!({"schema":"csdlc.v3.intent_plan.v1","slug":"transition-unit","cards":cards,"validators":[],"publication":{"base":"main","title":"Unit","body":"Closes #875","draft":true}})).unwrap()).unwrap();
            let mut index = json!({"schema":"csdlc.v3.local_state.v1","issue":875,"repository":"agent-logic/agent-design-language","phase":"ready","generation":4,"branch":"codex/875-transition-unit","worktree":future});
            let mut digest = blake3::Hasher::new();
            digest.update(&serde_json::to_vec(&index).unwrap());
            for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
                for suffix in ["values.json", "md"] {
                    digest
                        .update(&fs::read(source.join(format!("cards/{kind}.{suffix}"))).unwrap());
                }
            }
            index["digest"] = json!(digest.finalize().to_hex().to_string());
            put(&source.join("index.json"), &bytes(&index).unwrap()).unwrap();
            let checkout = if bound {
                git(
                    &primary,
                    &[
                        "-c",
                        "user.name=Unit",
                        "-c",
                        "user.email=unit@example.invalid",
                        "commit",
                        "--allow-empty",
                        "-m",
                        "unit",
                    ],
                )
                .unwrap();
                git(
                    &primary,
                    &[
                        "worktree",
                        "add",
                        "-b",
                        "codex/875-transition-unit",
                        future.to_str().unwrap(),
                        "HEAD",
                    ],
                )
                .unwrap();
                let linked_source = future.join(".csdlc/issues/875");
                tree(&source, &linked_source).unwrap();
                source = linked_source;
                let binding = json!({"schema":"csdlc.v3.binding.v1","issue":875,"branch":"codex/875-transition-unit","worktree":future});
                put(&source.join("binding.json"), &bytes(&binding).unwrap()).unwrap();
                put(
                    &common.join("csdlc-v3/local/bindings/875.json"),
                    &bytes(&binding).unwrap(),
                )
                .unwrap();
                index["phase"] = json!("bound");
                index.as_object_mut().unwrap().remove("digest");
                let mut digest = blake3::Hasher::new();
                digest.update(&serde_json::to_vec(&index).unwrap());
                for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
                    for suffix in ["values.json", "md"] {
                        digest.update(
                            &fs::read(source.join(format!("cards/{kind}.{suffix}"))).unwrap(),
                        );
                    }
                }
                digest.update(&fs::read(source.join("binding.json")).unwrap());
                index["digest"] = json!(digest.finalize().to_hex().to_string());
                fs::write(source.join("index.json"), bytes(&index).unwrap()).unwrap();
                let request_digest = "a".repeat(64);
                let completion = json!({"schema":"csdlc.v3.local_mutation_completion.v1","issue":875,"route":"bind","request_digest":request_digest,
                    "result":{"route":"bind","issue":875,"mutated":true,"phase":"bound","generation":4,"digest":index["digest"],"next_route":"validate","findings":[]}});
                put(
                    &future.join(format!(
                        ".csdlc/transactions/completed/875/bind-{request_digest}.json"
                    )),
                    &bytes(&completion).unwrap(),
                )
                .unwrap();
                put(&future.join("user-dirty-work.txt"), b"preserve me").unwrap();
                future.clone()
            } else {
                primary.clone()
            };
            put(&primary.join(SELECTOR_PATH), b"{}").unwrap();
            let registry = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .unwrap()
                .join("docs/templates/prompts/current.json");
            put(
                &primary.join("docs/templates/prompts/current.json"),
                &fs::read(registry).unwrap(),
            )
            .unwrap();
            let installed = primary.join(".adl/bin/native-v3/csdlc");
            let candidate = primary.join("candidate");
            put(&installed, b"prior unit executable").unwrap();
            put(&candidate, b"candidate unit executable").unwrap();
            put(
                &installed.parent().unwrap().join(".provenance/csdlc.json"),
                b"old provenance",
            )
            .unwrap();
            put(
                &installed.parent().unwrap().join(".provenance/csdlc.sha256"),
                b"old source cache",
            )
            .unwrap();
            let request = Request {
                schema: "csdlc.v3.live_transition.v1".into(),
                repository: "agent-logic/agent-design-language".into(),
                primary: primary.clone(),
                operation_id: "unit-operation".into(),
                operator: "unit-operator".into(),
                approval_reference: "unit-only".into(),
                pause_expires_unix: u64::MAX,
                acknowledged_issues: vec![875],
                candidate_blake3: hash(&candidate).unwrap(),
                candidate,
                prior_blake3: hash(&installed).unwrap(),
                installed,
                candidate_source_revision: QUALIFIED_SOURCE.into(),
                authority_blake3: hash(&primary.join(SELECTOR_PATH)).unwrap(),
                owner_blake3: "unit".into(),
                remote_digest: "absent".into(),
                records: vec![Record {
                    issue: 875,
                    checkout_head: git(&checkout, &["rev-parse", "HEAD"]).unwrap(),
                    checkout_branch: git(
                        &checkout,
                        &["symbolic-ref", "--quiet", "--short", "HEAD"],
                    )
                    .unwrap(),
                    checkout,
                    plan: Some(plan.clone()),
                    plan_digest: Some(hash(&plan).unwrap()),
                    semantic_digest: "absent".into(),
                    source_digest: fingerprint(&source).unwrap(),
                }],
            };
            // Tests call internals with synthetic authority; the public CLI keeps
            // canonical authority and qualified artifact checks mandatory.
            let digest = blake3::hash(&bytes(&request).unwrap()).to_hex().to_string();
            let op = common.join("csdlc-v3/local/live-transitions/unit-operation");
            let c = Context {
                request,
                common,
                op,
                digest,
            };
            c.marker("request.json", &c.request).unwrap();
            fence(&c, "native").unwrap();
            let guard = retained_fence(&c).unwrap();
            let input = inputs(&c, &c.request.records[0], &guard).unwrap();
            c.marker("inputs-875.json", &input).unwrap();
            c.marker("admitted.json", &json!({"request_digest":c.digest}))
                .unwrap();
            Self(c)
        }
        fn convert(&self) -> Result<Value> {
            convert_live(&self.0, &retained_fence(&self.0)?)
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            STOP.with(|s| *s.borrow_mut() = None);
            let _ = release(&self.0);
            let _ = fs::remove_dir_all(&self.0.request.primary);
        }
    }
    #[test]
    fn repository_namespaces_remain_sealed_until_release() {
        let f = Fixture::new();
        let before = remote_fingerprint(&f.0.common.join("csdlc-v3/remote")).unwrap();
        seal_namespaces(&f.0).unwrap();
        check_namespaces(&f.0).unwrap();
        assert_eq!(
            before,
            remote_fingerprint(&f.0.common.join("csdlc-v3/remote")).unwrap()
        );
        f.0.check_sources().unwrap();
        f.convert().unwrap();
        verify_final(&f.0).unwrap();
        restore_live(&f.0).unwrap();
        let modes: Vec<u32> = read(&f.0.op.join("namespace-modes.json")).unwrap();
        for (path, mode) in namespace_paths(&f.0).iter().zip(modes) {
            assert_eq!(fs::metadata(path).unwrap().permissions().mode(), mode);
        }
    }

    #[test]
    fn pending_native_transaction_blocks_current_and_legacy_census() {
        let f = Fixture::new();
        let pending = f.0.common.join("csdlc-v3/local/transactions/999.json");
        put(&pending, b"{}").unwrap();
        assert!(f
            .0
            .check_sources()
            .unwrap_err()
            .contains("pending native transaction"));
    }

    #[test]
    fn unsettled_repository_remote_intent_is_not_admitted() {
        let f = Fixture::new();
        let remote = f.0.common.join("csdlc-v3/remote");
        crate::commands::remote::require_settled_transition_remote(&remote).unwrap();
        fs::create_dir_all(remote.join("intents")).unwrap();
        put(
            &remote
                .join("intents")
                .join(format!("{}.json", "a".repeat(64))),
            b"{}",
        )
        .unwrap();
        assert!(crate::commands::remote::require_settled_transition_remote(&remote).is_err());
    }

    #[test]
    fn live_conversion_preserves_source_and_restores_binary_provenance_and_projection() {
        let f = Fixture::new();
        let c = &f.0;
        let r = &c.request.records[0];
        assert_eq!(f.convert().unwrap()["status"], "converted_paused");
        assert_eq!(fingerprint(&c.source(r).unwrap()).unwrap(), r.source_digest);
        verify_final(c).unwrap();
        assert!(!provenance(c, "sha256").exists());
        assert_eq!(restore_live(c).unwrap()["status"], "restored");
        assert!(!c.live(875).exists());
        assert!(!c.projection(r).exists());
        assert_eq!(hash(&c.request.installed).unwrap(), c.request.prior_blake3);
        assert_eq!(fs::read(provenance(c, "json")).unwrap(), b"old provenance");
        assert_eq!(
            fs::read(provenance(c, "sha256")).unwrap(),
            b"old source cache"
        );
    }
    #[test]
    fn bound_conversion_preserves_genuine_registration_and_dirty_work() {
        let f = Fixture::with_binding(true);
        let c = &f.0;
        let r = &c.request.records[0];
        let branch = git(&r.checkout, &["symbolic-ref", "--short", "HEAD"]).unwrap();
        let head = git(&r.checkout, &["rev-parse", "HEAD"]).unwrap();
        f.convert().unwrap();
        verify_final(c).unwrap();
        let root = SemanticRoot::from_git_common(&c.common, c.request.repository.clone()).unwrap();
        let Observation::Current(s) = Store::observe_issue(
            &root,
            &IssueKey::new(c.request.repository.clone(), 875).unwrap(),
        )
        .unwrap() else {
            panic!("missing current state")
        };
        let binding = s.inputs().binding().unwrap();
        assert_eq!(binding.worktree, r.checkout);
        assert_eq!(binding.branch, branch);
        assert_eq!(binding.head, head);
        assert_eq!(
            fs::read(r.checkout.join("user-dirty-work.txt")).unwrap(),
            b"preserve me"
        );
        restore_live(c).unwrap();
        assert_eq!(fingerprint(&c.source(r).unwrap()).unwrap(), r.source_digest);
        assert_eq!(
            fs::read(r.checkout.join("user-dirty-work.txt")).unwrap(),
            b"preserve me"
        );
    }
    #[test]
    fn old_and_new_writers_are_denied_while_readback_is_allowed() {
        let f = Fixture::new();
        let c = &f.0;
        f.convert().unwrap();
        let old = OpenOptions::new()
            .read(true)
            .write(true)
            .open(c.common.join("csdlc-v3/local/locks/875.lock"))
            .unwrap();
        assert_eq!(
            old.try_lock_exclusive().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        let new = OpenOptions::new()
            .read(true)
            .write(true)
            .open(c.live(875).join("state.lock"))
            .unwrap();
        assert_eq!(
            new.try_lock_exclusive().unwrap_err().kind(),
            std::io::ErrorKind::WouldBlock
        );
        FileExt::try_lock_shared(&new).unwrap();
        FileExt::unlock(&new).unwrap();
        verify_final(c).unwrap();
        restore_live(c).unwrap();
    }
    #[test]
    fn interrupted_activation_restarts_without_recreating_state_or_losing_fences() {
        for stop in ["semantic_activation", "binary_activation"] {
            let f = Fixture::new();
            STOP.with(|s| *s.borrow_mut() = Some(stop));
            assert!(f.convert().unwrap_err().contains("injected stop"));
            assert!(held(&f.0, "native").unwrap());
            assert!(held(&f.0, "staged").unwrap());
            STOP.with(|s| *s.borrow_mut() = None);
            f.convert().unwrap();
            verify_final(&f.0).unwrap();
            restore_live(&f.0).unwrap();
        }
    }
    #[test]
    fn interrupted_restore_resumes_from_its_retained_intent() {
        let f = Fixture::new();
        f.convert().unwrap();
        STOP.with(|s| *s.borrow_mut() = Some("restore_semantic"));
        assert!(restore_live(&f.0).unwrap_err().contains("injected stop"));
        STOP.with(|s| *s.borrow_mut() = None);
        restore_live(&f.0).unwrap();
        assert!(!f.0.live(875).exists());
    }
    #[test]
    fn later_write_refuses_restore_before_any_original_bytes_change() {
        let f = Fixture::new();
        f.convert().unwrap();
        let c = &f.0;
        put(
            &c.live(875).join("later-write.json"),
            b"post-conversion effect",
        )
        .unwrap();
        let before = fingerprint(&c.live(875)).unwrap();
        assert!(restore_live(c)
            .unwrap_err()
            .contains("post-conversion effect"));
        assert_eq!(fingerprint(&c.live(875)).unwrap(), before);
        assert_eq!(
            hash(&c.request.installed).unwrap(),
            c.request.candidate_blake3
        );
    }
    #[test]
    fn uncertain_remote_change_refuses_restore() {
        let f = Fixture::new();
        f.convert().unwrap();
        put(
            &f.0.common.join("csdlc-v3/remote/intents/unknown.json"),
            b"uncertain",
        )
        .unwrap();
        assert!(verify_final(&f.0)
            .unwrap_err()
            .contains("remote state changed"));
        assert!(restore_live(&f.0).is_err());
        assert!(f.0.live(875).exists());
    }
    #[test]
    fn explicit_resume_is_irreversible_and_requires_the_exact_decision_identity() {
        let f = Fixture::new();
        f.convert().unwrap();
        let c = &f.0;
        let bad = c.op.join("bad-resume.json");
        put(&bad, b"{}").unwrap();
        assert!(resume_live(c, &bad).is_err());
        let good = c.op.join("resume-decision.json");
        put(&good,&bytes(&json!({"schema":"csdlc.v3.live_transition_resume.v1","request_digest":c.digest,"candidate_blake3":c.request.candidate_blake3,"operator":"unit","approval_reference":"unit-resume","approved_at_unix":SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),"expires_unix":u64::MAX,"conversion_blake3":hash(&c.op.join("converted.json")).unwrap()})).unwrap()).unwrap();
        resume_live(c, &good).unwrap();
        assert!(terminal(c).is_err());
    }
    #[test]
    fn lost_guardian_readiness_does_not_authorize_another_lock_holder() {
        let f = Fixture::new();
        let c = &f.0;
        let ready = c.op.join("native-ready.json");
        let original = fs::read(&ready).unwrap();
        let mut v: Value = read(&ready).unwrap();
        v["port"] = json!(0);
        fs::write(&ready, bytes(&v).unwrap()).unwrap();
        assert!(retained_fence(c).is_err());
        fs::write(&ready, original).unwrap();
        assert!(held(c, "native").unwrap());
    }
    #[test]
    fn create_only_journal_and_snapshot_refuse_changed_bytes() {
        let f = Fixture::new();
        let p = f.0.op.join("identity.json");
        put(&p, b"one").unwrap();
        put(&p, b"one").unwrap();
        assert!(put(&p, b"two").is_err());
        assert_eq!(fs::read(p).unwrap(), b"one");
    }

    #[test]
    fn parallel_guardian_receipts_use_distinct_atomic_temporaries() {
        let f = Fixture::new();
        std::thread::scope(|scope| {
            for n in 0..16 {
                let op = &f.0.op;
                scope.spawn(move || {
                    put(&op.join(format!("parallel-{n}.json")), b"receipt").unwrap()
                });
            }
        });
        for n in 0..16 {
            assert_eq!(
                fs::read(f.0.op.join(format!("parallel-{n}.json"))).unwrap(),
                b"receipt"
            );
        }
    }

    #[test]
    fn incomplete_census_and_changed_checkout_are_rejected() {
        let f = Fixture::new();
        fs::create_dir_all(f.0.common.join("csdlc-v3/local/issues/876")).unwrap();
        assert!(f
            .0
            .check_sources()
            .unwrap_err()
            .contains("incomplete census"));
        fs::remove_dir(f.0.common.join("csdlc-v3/local/issues/876")).unwrap();
        git(
            &f.0.request.primary,
            &[
                "-c",
                "user.name=Unit",
                "-c",
                "user.email=unit@example.invalid",
                "commit",
                "--allow-empty",
                "-m",
                "changed head",
            ],
        )
        .unwrap();
        assert!(f.0.check_sources().unwrap_err().contains("checkout head"));
    }

    #[test]
    fn current_semantic_records_are_fenced_and_preserved_without_reimport() {
        let f = Fixture::new();
        f.convert().unwrap();
        release(&f.0).unwrap();
        let mut c = f.0.clone();
        c.request.operation_id = "passthrough-operation".into();
        c.request.prior_blake3 = c.request.candidate_blake3.clone();
        c.request.records[0].semantic_digest = fingerprint(&c.live(875)).unwrap();
        c.request.records[0].plan = None;
        c.request.records[0].plan_digest = None;
        c.digest = blake3::hash(&bytes(&c.request).unwrap())
            .to_hex()
            .to_string();
        c.op = c
            .common
            .join("csdlc-v3/local/live-transitions/passthrough-operation");
        c.marker("request.json", &c.request).unwrap();
        fence(&c, "native").unwrap();
        c.marker("admitted.json", &json!({"request_digest":c.digest}))
            .unwrap();
        let current = Fixture(c);
        let before = fingerprint(&current.0.live(875)).unwrap();
        current.convert().unwrap();
        verify_final(&current.0).unwrap();
        assert_eq!(fingerprint(&current.0.live(875)).unwrap(), before);
        restore_live(&current.0).unwrap();
        assert_eq!(fingerprint(&current.0.live(875)).unwrap(), before);
    }
    #[test]
    fn census_excludes_committed_history_but_rejects_changed_or_unknown_residue() {
        let f = Fixture::new();
        let historical = f.0.request.primary.join(".csdlc/issues/3/index.json");
        put(&historical, b"{\"historical\":true}").unwrap();
        git(&f.0.request.primary, &["add", ".csdlc/issues/3/index.json"]).unwrap();
        git(
            &f.0.request.primary,
            &["commit", "--quiet", "-m", "historical cards"],
        )
        .unwrap();
        let before = fingerprint(&f.0.common.join("csdlc-v3")).unwrap();
        let observed = census_inventory(&f.0.request.primary, &f.0.common).unwrap();
        assert_eq!(observed.issues, BTreeSet::from([875]));
        assert_eq!(observed.excluded_history.len(), 1);
        assert_eq!(observed.excluded_history[0]["issues"], json!([3]));
        assert_eq!(fingerprint(&f.0.common.join("csdlc-v3")).unwrap(), before);
        let packet = inventory(&f.0.request.primary).unwrap();
        assert_eq!(packet["records"].as_array().unwrap().len(), 1);
        assert_eq!(packet["excluded_history"][0]["issues"], json!([3]));
        fs::write(&historical, b"changed historical lifecycle state").unwrap();
        let rejected = census(&f.0.request.primary, &f.0.common).unwrap_err();
        assert!(rejected.contains("issue 3") && rejected.contains("changed lifecycle residue"));
        fs::write(&historical, b"{\"historical\":true}").unwrap();
        let residue = f.0.request.primary.join(".csdlc/issues/999/index.json");
        put(&residue, b"unregistered live residue").unwrap();
        let rejected = f.0.check_sources().unwrap_err();
        assert!(rejected.contains("issue 999") && rejected.contains("unregistered"));
        assert_eq!(fs::read(residue).unwrap(), b"unregistered live residue");
    }

    #[test]
    fn census_rejects_staged_and_ignored_historical_changes() {
        for change in ["modify", "add", "delete", "ignored"] {
            let f = Fixture::new();
            let primary = &f.0.request.primary;
            let historical = primary.join(".csdlc/issues/3/index.json");
            put(&historical, b"historical").unwrap();
            put(&primary.join(".csdlc/issues/3/sor.md"), b"historical card").unwrap();
            git(primary, &["add", ".csdlc/issues/3"]).unwrap();
            git(primary, &["commit", "--quiet", "-m", "historical cards"]).unwrap();
            match change {
                "modify" => fs::write(&historical, b"changed").unwrap(),
                "add" => put(&primary.join(".csdlc/issues/3/new.md"), b"new").unwrap(),
                "delete" => fs::remove_file(primary.join(".csdlc/issues/3/sor.md")).unwrap(),
                "ignored" => {
                    fs::write(f.0.common.join("info/exclude"), b"ignored-residue\n").unwrap();
                    put(&primary.join(".csdlc/issues/3/ignored-residue"), b"unknown").unwrap();
                }
                _ => unreachable!(),
            }
            if change != "ignored" {
                git(primary, &["add", "-A", ".csdlc/issues/3"]).unwrap();
            }
            let before = fingerprint(&primary.join(".csdlc/issues/3")).unwrap();
            let rejected = census(primary, &f.0.common).unwrap_err();
            assert!(
                rejected.contains("issue 3") && rejected.contains("changed lifecycle residue"),
                "{change}: {rejected}"
            );
            assert_eq!(
                fingerprint(&primary.join(".csdlc/issues/3")).unwrap(),
                before
            );
        }
    }

    #[test]
    fn census_keeps_bound_legacy_records_without_primary_index() {
        let f = Fixture::with_binding(true);
        fs::remove_dir_all(f.0.common.join("csdlc-v3/local/issues/875")).unwrap();
        let packet = inventory(&f.0.request.primary).unwrap();
        let rows = packet["records"].as_array().unwrap();
        assert_eq!(rows.len(), 1);
        assert_eq!(rows[0]["issue"], 875);
        assert_eq!(rows[0]["checkout"], json!(f.0.request.records[0].checkout));
        assert_eq!(rows[0]["native_phase"], "bound");
        assert_eq!(rows[0]["disposition"], "requires_native_admission");
        assert_eq!(packet["excluded_history"], json!([]));
        let source = f.0.request.records[0]
            .checkout
            .join(".csdlc/issues/875/index.json");
        fs::remove_file(&source).unwrap();
        let rejected = inventory(&f.0.request.primary).unwrap_err();
        assert!(rejected.contains("issue 875") && rejected.contains(source.to_str().unwrap()));
    }

    #[test]
    fn census_rejects_binding_identity_conflicts() {
        let f = Fixture::new();
        let binding = f.0.common.join("csdlc-v3/local/bindings/875.json");
        put(
            &binding,
            &bytes(
                &json!({"schema":"csdlc.v3.binding.v1","issue":876,"worktree":f.0.request.primary}),
            )
            .unwrap(),
        )
        .unwrap();
        let rejected = census(&f.0.request.primary, &f.0.common).unwrap_err();
        assert!(rejected.contains("issue 875") && rejected.contains("invalid native binding"));
    }
}
