//! Nonmutating candidate consistency checks. A gate is evidence input, never release authority.
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs,
    path::{Component, Path},
    process::Command,
};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Request {
    pub repository: String,
    pub version: String,
    pub candidate_sha: String,
    pub notes_path: String,
    pub notes_digest: String,
    pub gate_path: String,
    pub gate_digest: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Gate {
    schema: String,
    repository: String,
    version: String,
    candidate_sha: String,
    notes_path: String,
    notes_digest: String,
    inventory_digest: String,
    status: String,
    evidence: Vec<Evidence>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Evidence {
    issue: u64,
    path: String,
    digest: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Inventory {
    schema: String,
    version: String,
    packages: Vec<Package>,
    lockfiles: Vec<String>,
    historical_lockfiles: Vec<HistoricalLock>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct HistoricalLock {
    path: String,
    rationale: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Package {
    manifest: String,
    package: String,
    version: String,
    workspace: Option<String>,
    policy: String,
    rationale: String,
}
#[derive(Serialize)]
pub struct Report {
    schema: &'static str,
    pub eligible: bool,
    read_only: bool,
    mutation_allowed: bool,
    release_authorized: bool,
    candidate_sha: String,
    notes_digest: String,
    gate_digest: String,
    pub findings: Vec<String>,
}

pub fn preflight(root: &Path, request: &Request) -> Report {
    let result = check(root, request);
    Report {
        schema: "csdlc.v3.release_preflight.v1",
        eligible: result.is_ok(),
        read_only: true,
        mutation_allowed: false,
        release_authorized: false,
        candidate_sha: request.candidate_sha.clone(),
        notes_digest: request.notes_digest.clone(),
        gate_digest: request.gate_digest.clone(),
        findings: result.err().into_iter().collect(),
    }
}
fn git(root: &Path, args: &[&str]) -> Result<Vec<u8>, String> {
    let out = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        // Even `git status` can refresh index metadata by default. Candidate
        // observation must not take optional write locks or persist that refresh.
        .env("GIT_OPTIONAL_LOCKS", "0")
        .output()
        .map_err(|_| "git_unavailable")?;
    if !out.status.success() {
        return Err("candidate_git_read_failed".into());
    }
    Ok(out.stdout)
}
fn relative(path: &str) -> Result<(), String> {
    if path.is_empty()
        || !Path::new(path)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
    {
        return Err("invalid_repository_relative_path".into());
    }
    Ok(())
}
fn blob(root: &Path, sha: &str, path: &str) -> Result<Vec<u8>, String> {
    relative(path)?;
    let bytes = git(root, &["show", &format!("{sha}:{path}")])?;
    if fs::read(root.join(path)).map_err(|_| "candidate_file_missing")? != bytes {
        return Err(format!("candidate_working_bytes_differ: {path}"));
    }
    Ok(bytes)
}
fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}
fn toml(bytes: &[u8]) -> Result<toml::Value, String> {
    std::str::from_utf8(bytes)
        .map_err(|_| "invalid_toml_utf8")?
        .parse()
        .map_err(|_| "invalid_toml".into())
}
fn check(root: &Path, r: &Request) -> Result<(), String> {
    if r.repository != "agent-logic/agent-design-language" || r.version != "v0.92.1" {
        return Err("unsupported_release_identity".into());
    }
    if r.candidate_sha.len() != 40
        || !r
            .candidate_sha
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("full_candidate_sha_required".into());
    }
    if git(root, &["rev-parse", "HEAD"])?.strip_suffix(b"\n") != Some(r.candidate_sha.as_bytes()) {
        return Err("candidate_head_mismatch".into());
    }
    if !git(root, &["status", "--porcelain", "--untracked-files=normal"])?.is_empty() {
        return Err("candidate_checkout_dirty".into());
    }
    if crate::operational_authority(root)? != "csdlc-v3" {
        return Err("native_v3_authority_suspended".into());
    }
    let expected_notes = format!("docs/milestones/{0}/RELEASE_NOTES_{0}.md", r.version);
    if r.notes_path != expected_notes {
        return Err("release_notes_path_mismatch".into());
    }
    if digest(&blob(root, &r.candidate_sha, &r.notes_path)?) != r.notes_digest {
        return Err("release_notes_digest_mismatch".into());
    }
    let gate_bytes = fs::read(&r.gate_path).map_err(|_| "native_v3_gate_missing")?;
    if digest(&gate_bytes) != r.gate_digest {
        return Err("native_v3_gate_digest_mismatch".into());
    }
    let gate: Gate = serde_json::from_slice(&gate_bytes).map_err(|_| "native_v3_gate_malformed")?;
    if gate.schema != "csdlc.v3.release_gate.v1"
        || gate.repository != r.repository
        || gate.version != r.version
        || gate.candidate_sha != r.candidate_sha
        || gate.notes_path != r.notes_path
        || gate.notes_digest != r.notes_digest
        || gate.status != "ready_for_preflight"
    {
        return Err("native_v3_gate_stale_or_ineligible".into());
    }
    let evidence_issues: BTreeSet<_> = gate.evidence.iter().map(|e| e.issue).collect();
    if evidence_issues != BTreeSet::from([522, 525, 526]) || gate.evidence.len() != 3 {
        return Err("release_tail_evidence_required".into());
    }
    let mut evidence_paths = BTreeSet::new();
    for e in &gate.evidence {
        if !e.path.starts_with("docs/milestones/v0.92.1/")
            || !evidence_paths.insert(&e.path)
            || digest(&blob(root, &r.candidate_sha, &e.path)?) != e.digest
        {
            return Err("release_tail_evidence_mismatch".into());
        }
    }
    let inventory_bytes = blob(
        root,
        &r.candidate_sha,
        &format!("docs/milestones/{}/RELEASE_ARTIFACTS.json", r.version),
    )?;
    if digest(&inventory_bytes) != gate.inventory_digest {
        return Err("release_inventory_digest_mismatch".into());
    }
    let inventory: Inventory =
        serde_json::from_slice(&inventory_bytes).map_err(|_| "release_inventory_malformed")?;
    if inventory.schema != "csdlc.v3.release_artifacts.v1"
        || inventory.version != r.version
        || inventory.packages.is_empty()
    {
        return Err("release_inventory_identity_mismatch".into());
    }
    let files = String::from_utf8(git(
        root,
        &["ls-tree", "-r", "--name-only", &r.candidate_sha],
    )?)
    .map_err(|_| "invalid_git_paths")?;
    let manifests: BTreeSet<_> = files
        .lines()
        .filter(|p| p.ends_with("/Cargo.toml"))
        .collect();
    let locks: BTreeSet<_> = files
        .lines()
        .filter(|p| p.ends_with("/Cargo.lock"))
        .collect();
    let mut seen = BTreeSet::new();
    let mut names = BTreeSet::new();
    for p in &inventory.packages {
        if !seen.insert(p.manifest.as_str())
            || !names.insert(p.package.as_str())
            || p.rationale.trim().is_empty()
        {
            return Err("release_inventory_duplicate_or_unjustified".into());
        }
        let manifest = toml(&blob(root, &r.candidate_sha, &p.manifest)?)?;
        let package = manifest.get("package").ok_or("release_package_missing")?;
        let actual_version = if let Some(workspace) = &p.workspace {
            if package
                .get("version")
                .and_then(|v| v.get("workspace"))
                .and_then(|v| v.as_bool())
                != Some(true)
            {
                return Err("workspace_version_not_inherited".into());
            }
            let workspace = toml(&blob(root, &r.candidate_sha, workspace)?)?;
            workspace
                .get("workspace")
                .and_then(|v| v.get("package"))
                .and_then(|v| v.get("version"))
                .and_then(|v| v.as_str())
                .ok_or("workspace_version_missing")?
                .to_string()
        } else {
            package
                .get("version")
                .and_then(|v| v.as_str())
                .ok_or("package_version_missing")?
                .to_string()
        };
        if package.get("name").and_then(|v| v.as_str()) != Some(&p.package)
            || actual_version != p.version
            || (p.policy == "coordinated" && p.version != r.version.trim_start_matches('v'))
            || !matches!(p.policy.as_str(), "coordinated" | "independent")
        {
            return Err(format!("release_package_version_mismatch: {}", p.manifest));
        }
    }
    for path in manifests {
        let data = toml(&blob(root, &r.candidate_sha, path)?)?;
        if data.get("package").is_some() && !seen.contains(path) {
            return Err(format!("release_inventory_omits_manifest: {path}"));
        }
    }
    let mut covered_locks: BTreeSet<&str> =
        inventory.lockfiles.iter().map(String::as_str).collect();
    for historical in &inventory.historical_lockfiles {
        if !(historical.path.starts_with(".csdlc/prepared/issues/")
            || historical.path.starts_with("adl-v2/crates/"))
            || historical.rationale.trim().is_empty()
            || !covered_locks.insert(&historical.path)
        {
            return Err("invalid_historical_lock_disposition".into());
        }
        blob(root, &r.candidate_sha, &historical.path)?;
    }
    if covered_locks != locks
        || inventory.lockfiles.iter().collect::<BTreeSet<_>>().len() != inventory.lockfiles.len()
    {
        return Err("release_inventory_lockfile_coverage".into());
    }
    let mut locked_local_packages = BTreeSet::new();
    for path in &inventory.lockfiles {
        let data = toml(&blob(root, &r.candidate_sha, path)?)?;
        let mut local_names = BTreeSet::new();
        for entry in data
            .get("package")
            .and_then(|v| v.as_array())
            .ok_or("lockfile_packages_missing")?
        {
            if entry.get("source").is_some() {
                continue;
            }
            let name = entry
                .get("name")
                .and_then(|v| v.as_str())
                .ok_or("lockfile_package_name_missing")?;
            local_names.insert(name.to_string());
            locked_local_packages.insert(name.to_string());
            let pkg = inventory
                .packages
                .iter()
                .find(|p| p.package == name)
                .ok_or("unlisted_local_lock_package")?;
            if entry.get("version").and_then(|v| v.as_str()) != Some(&pkg.version) {
                return Err(format!("release_lock_version_mismatch: {path}: {name}"));
            }
        }
        let owner_manifest = path
            .strip_suffix("Cargo.lock")
            .ok_or("invalid_lockfile_path")?
            .to_string()
            + "Cargo.toml";
        let owners: Vec<_> = inventory
            .packages
            .iter()
            .filter(|p| {
                p.manifest == owner_manifest
                    || p.workspace.as_deref() == Some(owner_manifest.as_str())
            })
            .collect();
        if owners.is_empty() || owners.iter().any(|p| !local_names.contains(&p.package)) {
            return Err(format!("release_lock_owner_missing: {path}"));
        }
    }
    if inventory
        .packages
        .iter()
        .any(|p| p.policy == "coordinated" && !locked_local_packages.contains(&p.package))
    {
        return Err("release_local_package_missing_from_locks".into());
    }
    // Check the checkout identity again after all reads; this report authorizes no later action.
    if git(root, &["rev-parse", "HEAD"])?.strip_suffix(b"\n") != Some(r.candidate_sha.as_bytes())
        || !git(root, &["status", "--porcelain", "--untracked-files=normal"])?.is_empty()
    {
        return Err("candidate_changed_during_preflight".into());
    }
    Ok(())
}
