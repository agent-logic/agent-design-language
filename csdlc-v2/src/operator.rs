use crate::error::{ErrorCode, Result, V2Error};
use crate::{select_generation, Generation, GenerationSelector};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

const SKILLS: &str = include_str!("../operator/skills.json");
const COEXISTENCE: &str = include_str!("../operator/coexistence.json");

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SkillManifest {
    pub schema: String,
    pub generation: String,
    pub generation_selector: String,
    #[serde(default)]
    pub operational_binaries: Vec<String>,
    pub operation_routes: Vec<OperationRoute>,
    pub skills: Vec<SkillRoute>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OperationRoute {
    pub binary: String,
    pub operation: String,
    pub mutates_state: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct SkillRoute {
    pub name: String,
    pub binary: String,
    pub subcommand: Option<String>,
    pub mutates_state: bool,
    pub auxiliary_binaries: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CoexistenceInventory {
    pub schema: String,
    pub v1_sunset: bool,
    pub required_v1_paths: Vec<RequiredPath>,
    pub forbidden_v1_paths: Vec<PathBuf>,
    pub required_v2_binaries: Vec<String>,
    pub forbidden_v2_binaries: Vec<String>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RequiredPath {
    pub path: PathBuf,
    pub executable: bool,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CoexistenceReport {
    pub schema: String,
    pub pass: bool,
    pub default_generation: Generation,
    pub missing_v1_paths: Vec<PathBuf>,
    pub present_forbidden_v1_paths: Vec<PathBuf>,
    pub missing_v2_binaries: Vec<String>,
    pub present_forbidden_v2_binaries: Vec<String>,
    pub skill_count: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstallReceipt {
    pub schema: String,
    pub destination: PathBuf,
    pub source_revision: String,
    pub source_set_schema: String,
    pub source_set_digest: String,
    pub binaries: Vec<InstalledBinary>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstalledBinary {
    pub name: String,
    pub blake3: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
struct OwnerSourceSet {
    schema: String,
    entries: Vec<PathBuf>,
}

const OWNER_SOURCE_SET: &str = include_str!("../operator/owner-source-set.json");

pub fn verify_installed_owner_preflight(repo: &Path) -> Result<()> {
    #[cfg(debug_assertions)]
    if std::env::var_os("ADL_CSDLC_TEST_ALLOW_UNINSTALLED_OWNER_BINARY").as_deref()
        == Some(std::ffi::OsStr::new("1"))
        && executable_is_in_cargo_debug_profile_directory()?
    {
        return Ok(());
    }
    if !is_regular_file(&repo.join(".adl/worktree-policy.json")) {
        return Ok(());
    }
    let executable = std::env::current_exe().map_err(io_error)?;
    let Some(name) = executable.file_name().and_then(|value| value.to_str()) else {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "owner executable identity is unavailable",
        ));
    };
    let manifest = SkillManifest::load()?;
    if !manifest.required_binaries().contains(name) {
        return Ok(());
    }
    let bin_dir = executable.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ValidationFailed,
            "owner executable directory is unavailable",
        )
    })?;
    let receipt_path = bin_dir.join("install-receipt.json");
    if !is_regular_file(&receipt_path) {
        return Err(stale_owner_error("installed receipt is missing"));
    }
    let receipt: InstallReceipt =
        serde_json::from_slice(&fs::read(&receipt_path).map_err(io_error)?)
            .map_err(|_| stale_owner_error("installed receipt is malformed"))?;
    verify_receipt_identity(repo, bin_dir, &manifest, &receipt)
}

#[cfg(debug_assertions)]
fn executable_is_in_cargo_debug_profile_directory() -> Result<bool> {
    let executable = std::env::current_exe().map_err(io_error)?;
    let executable = fs::canonicalize(executable).map_err(io_error)?;
    let Some(profile_directory) = executable.parent() else {
        return Ok(false);
    };
    Ok(
        profile_directory.file_name() == Some(std::ffi::OsStr::new("debug"))
            && profile_directory.join("deps").is_dir()
            && profile_directory.join(".fingerprint").is_dir()
            && profile_directory.join("build").is_dir(),
    )
}

pub fn verify_installed_owner_operation(repo: &Path, operation: &str) -> Result<()> {
    let executable = std::env::current_exe().map_err(io_error)?;
    let name = executable
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ValidationFailed,
                "owner executable identity is unavailable",
            )
        })?;
    let manifest = SkillManifest::load()?;
    let routes = manifest
        .operation_routes
        .iter()
        .filter(|route| route.binary == name)
        .collect::<Vec<_>>();
    if routes.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            format!("owner operation classification is unavailable: {name}/{operation}"),
        ));
    }
    let route = routes
        .into_iter()
        .find(|route| route.operation == operation)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidManifest,
                format!("owner operation is unclassified: {name}/{operation}"),
            )
        })?;
    if route.mutates_state {
        verify_installed_owner_preflight(repo)?;
    }
    Ok(())
}

fn stale_owner_error(detail: &str) -> V2Error {
    V2Error::new(
        ErrorCode::ValidationFailed,
        format!("stale owner-binary provenance: {detail}; run csdlc-install resolve, then reinstall the selected generation"),
    )
}

pub fn owner_source_set_digest(repo: &Path) -> Result<String> {
    let source_set: OwnerSourceSet = serde_json::from_str(OWNER_SOURCE_SET).map_err(|error| {
        V2Error::new(
            ErrorCode::InvalidManifest,
            format!("invalid owner source set: {error}"),
        )
    })?;
    if source_set.schema != "csdlc.owner_source_set.v1" || source_set.entries.is_empty() {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "owner source set identity is invalid",
        ));
    }
    let mut entries = Vec::new();
    for entry in source_set.entries {
        let raw = entry.to_str().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidManifest, "owner source path is not UTF-8")
        })?;
        if entry.is_absolute()
            || entry
                .components()
                .any(|part| matches!(part, std::path::Component::ParentDir))
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "owner source path escapes the repository",
            ));
        }
        entries.push(raw.to_string());
    }
    let mut status_args = vec!["status", "--porcelain=v1", "--untracked-files=all", "--"];
    status_args.extend(entries.iter().map(String::as_str));
    if !crate::git::run(repo, &status_args)?
        .stdout
        .trim()
        .is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "owner source set contains modified, deleted, or untracked files",
        ));
    }
    let mut ls_args = vec!["ls-files", "--stage", "--"];
    ls_args.extend(entries.iter().map(String::as_str));
    let output = crate::git::run(repo, &ls_args)?.stdout;
    let mut files = BTreeMap::<Vec<u8>, (PathBuf, String, String)>::new();
    for line in output.lines() {
        let (metadata, path) = line.split_once('\t').ok_or_else(|| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                "owner source Git row is malformed",
            )
        })?;
        let mut fields = metadata.split_whitespace();
        let mode = fields.next().unwrap_or_default();
        let object = fields.next().unwrap_or_default();
        let stage = fields.next().unwrap_or_default();
        if (mode != "100644" && mode != "100755") || object.len() < 40 || stage != "0" {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                format!("owner source is not a tracked regular stage-zero file: {path}"),
            ));
        }
        if files
            .insert(
                path.as_bytes().to_vec(),
                (PathBuf::from(path), mode.to_string(), object.to_string()),
            )
            .is_some()
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "owner source set contains duplicate files",
            ));
        }
    }
    for entry in &entries {
        let prefix = format!("{entry}/");
        if !files.values().any(|(path, _, _)| {
            let path = path.to_string_lossy();
            path == entry.as_str() || path.starts_with(&prefix)
        }) {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                format!("owner source entry is unavailable: {entry}"),
            ));
        }
    }
    let mut hasher = blake3::Hasher::new();
    hasher.update(b"csdlc.owner_source_set.v1\0");
    for (_, (path, mode, object)) in files {
        let path = path.to_str().ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidManifest, "owner source path is not UTF-8")
        })?;
        hasher.update(&(path.len() as u64).to_be_bytes());
        hasher.update(path.as_bytes());
        hasher.update(&(mode.len() as u64).to_be_bytes());
        hasher.update(mode.as_bytes());
        hasher.update(&(object.len() as u64).to_be_bytes());
        hasher.update(object.as_bytes());
    }
    Ok(hasher.finalize().to_hex().to_string())
}

impl SkillManifest {
    pub fn load() -> Result<Self> {
        let manifest: Self = serde_json::from_str(SKILLS).map_err(|e| {
            V2Error::new(
                ErrorCode::CorruptRecord,
                format!("invalid embedded skill manifest: {e}"),
            )
        })?;
        manifest.validate()?;
        Ok(manifest)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != "csdlc.operator_skills.v1"
            || self.generation != "v2"
            || self.generation_selector != "csdlc-v2/operator/generation-selector.json"
            || self.skills.len() != 11
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "operator manifest must declare eleven v2 skills bound to the tracked generation selector",
            ));
        }
        let mut names = BTreeSet::new();
        for route in &self.skills {
            if !names.insert(&route.name)
                || !route.binary.starts_with("csdlc-")
                || route.binary.contains("python")
                || route.binary.ends_with(".sh")
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "skills must be unique typed C-SDLC binary routes",
                ));
            }
            for binary in std::iter::once(&route.binary).chain(&route.auxiliary_binaries) {
                if !binary.starts_with("csdlc-") || binary.contains('/') {
                    return Err(V2Error::new(
                        ErrorCode::InvalidManifest,
                        "all skill executables must be simple typed C-SDLC binary names",
                    ));
                }
            }
        }
        let observed_skills = self
            .skills
            .iter()
            .map(|route| {
                (
                    route.binary.as_str(),
                    route.subcommand.as_deref(),
                    route.mutates_state,
                )
            })
            .collect::<BTreeSet<_>>();
        let expected_skills = [
            ("csdlc-issue", Some("create"), true),
            ("csdlc-bind", None, true),
            ("csdlc-edit", Some("apply"), true),
            ("csdlc-validate", None, true),
            ("csdlc-review", None, true),
            ("csdlc-publish", None, true),
            ("csdlc-finish", None, true),
            ("csdlc-clean", None, true),
            ("csdlc-github", Some("run"), true),
            ("csdlc-shepherd", None, false),
            ("csdlc-doctor", None, false),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        if observed_skills != expected_skills {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "skill command classification differs from the canonical owner inventory",
            ));
        }
        for binary in &self.operational_binaries {
            if !binary.starts_with("csdlc-") || binary.contains('/') {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "all operational executables must be simple typed C-SDLC binary names",
                ));
            }
        }
        let expected_routes = [
            ("csdlc-github", "run-write", true),
            ("csdlc-github", "run-read", false),
            ("csdlc-github", "runner-preflight", false),
            ("csdlc-github", "schema", false),
            ("csdlc-clean", "cleanup", true),
            ("csdlc-clean", "compatibility-index", true),
            ("csdlc-clean", "materialize-terminal", true),
            ("csdlc-clean", "validate-census", false),
            ("csdlc-clean", "schema", false),
            ("csdlc-finish", "finish", true),
            ("csdlc-finish", "historical-finish", true),
            ("csdlc-finish", "recordless-closeout", true),
            ("csdlc-finish", "validate-cached-issue", false),
            ("csdlc-publish", "publish", true),
            ("csdlc-publish", "status", false),
            ("csdlc-publish", "ready", true),
            ("csdlc-publish", "reconcile-ready", true),
            ("csdlc-publish", "schema", false),
            ("csdlc-github-issue", "issue-write", true),
            ("csdlc-github-issue", "issue-read", false),
            ("csdlc-github-pr", "state", false),
            ("csdlc-github-pr", "pr-write", true),
            ("csdlc-pr-state", "state", false),
            ("csdlc-shadow", "compare", false),
            ("csdlc-shadow", "generate-view", true),
            ("csdlc-shadow", "schema", false),
            ("csdlc-soak", "generate-samples", true),
            ("csdlc-soak", "decide-with-output", true),
            ("csdlc-soak", "decide-without-output", false),
            ("csdlc-soak", "schema", false),
            ("csdlc-proof", "run", true),
            ("csdlc-cutover", "run", true),
        ]
        .into_iter()
        .collect::<BTreeSet<_>>();
        let observed_routes = self
            .operation_routes
            .iter()
            .map(|route| {
                (
                    route.binary.as_str(),
                    route.operation.as_str(),
                    route.mutates_state,
                )
            })
            .collect::<BTreeSet<_>>();
        if observed_routes != expected_routes {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "operation classification differs from the canonical command inventory",
            ));
        }
        let installed = self.required_binaries();
        let mut routes = BTreeSet::new();
        for route in &self.operation_routes {
            if !installed.contains(&route.binary)
                || route.operation.is_empty()
                || !routes.insert((route.binary.clone(), route.operation.clone()))
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "operation routes must be unique and bind installed owner binaries",
                ));
            }
        }
        for binary in &self.operational_binaries {
            if !self
                .operation_routes
                .iter()
                .any(|route| &route.binary == binary)
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidManifest,
                    "every operational binary must declare operation routes",
                ));
            }
        }
        Ok(())
    }

    fn binaries(&self) -> BTreeSet<&str> {
        self.skills
            .iter()
            .flat_map(|route| {
                std::iter::once(route.binary.as_str())
                    .chain(route.auxiliary_binaries.iter().map(String::as_str))
            })
            .chain(self.operational_binaries.iter().map(String::as_str))
            .collect()
    }

    pub fn required_binaries(&self) -> BTreeSet<String> {
        self.binaries().into_iter().map(str::to_owned).collect()
    }
}

impl CoexistenceInventory {
    pub fn load() -> Result<Self> {
        serde_json::from_str(COEXISTENCE).map_err(Into::into)
    }
}

pub fn verify_coexistence(
    repo: &Path,
    bin_dir: &Path,
    inventory: &CoexistenceInventory,
) -> Result<CoexistenceReport> {
    let manifest = SkillManifest::load()?;
    let reviewed = CoexistenceInventory::load()?;
    if inventory != &reviewed {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "coexistence input must exactly match the embedded reviewed inventory",
        ));
    }
    if inventory.schema != "csdlc.coexistence_inventory.v2" || !inventory.v1_sunset {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "final v2 installation requires the reviewed v1-sunset inventory",
        ));
    }
    let selector_path = checked_repo_path(repo, Path::new(&manifest.generation_selector))?;
    if !is_regular_file(&selector_path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "tracked generation selector must be a regular file, not a symlink",
        ));
    }
    let selector: GenerationSelector =
        serde_json::from_slice(&fs::read(&selector_path).map_err(io_error)?)?;
    select_generation(&selector, 0, None)?;
    let missing_v1_paths = if inventory.v1_sunset {
        Vec::new()
    } else {
        inventory
            .required_v1_paths
            .iter()
            .filter_map(|required| match checked_repo_path(repo, &required.path) {
                Ok(path)
                    if is_regular_file(&path) && (!required.executable || is_executable(&path)) =>
                {
                    None
                }
                _ => Some(required.path.clone()),
            })
            .collect::<Vec<_>>()
    };
    let mut present_forbidden_v1_paths = inventory
        .forbidden_v1_paths
        .iter()
        .filter(|path| {
            checked_repo_path(repo, path).is_ok_and(|path| fs::symlink_metadata(path).is_ok())
        })
        .cloned()
        .collect::<Vec<_>>();
    for path in discover_forbidden_v1_paths(repo)? {
        if !present_forbidden_v1_paths.contains(&path) {
            present_forbidden_v1_paths.push(path);
        }
    }
    let missing_v2_binaries = inventory
        .required_v2_binaries
        .iter()
        .filter(|n| {
            let path = bin_dir.join(n);
            !is_regular_file(&path) || !is_executable(&path)
        })
        .cloned()
        .collect::<BTreeSet<_>>();
    let mut missing_v2_binaries = missing_v2_binaries;
    missing_v2_binaries.extend(verify_install_receipt(repo, bin_dir, &manifest)?);
    let present_forbidden_v2_binaries = inventory
        .forbidden_v2_binaries
        .iter()
        .filter(|name| fs::symlink_metadata(bin_dir.join(name)).is_ok())
        .cloned()
        .collect::<Vec<_>>();
    Ok(CoexistenceReport {
        schema: "csdlc.coexistence_report.v2".into(),
        pass: missing_v1_paths.is_empty()
            && present_forbidden_v1_paths.is_empty()
            && missing_v2_binaries.is_empty()
            && present_forbidden_v2_binaries.is_empty(),
        default_generation: selector.default_generation,
        missing_v1_paths,
        present_forbidden_v1_paths,
        missing_v2_binaries: missing_v2_binaries.into_iter().collect(),
        present_forbidden_v2_binaries,
        skill_count: manifest.skills.len(),
    })
}

fn discover_forbidden_v1_paths(repo: &Path) -> Result<Vec<PathBuf>> {
    let mut found = Vec::new();
    let roots = [
        repo.join("adl/tools"),
        repo.join("adl/src/cli"),
        repo.join("adl/src/bin"),
    ];
    for root in roots {
        if !root.exists() {
            continue;
        }
        let mut stack = vec![root];
        while let Some(path) = stack.pop() {
            for entry in fs::read_dir(&path).map_err(io_error)? {
                let entry = entry.map_err(io_error)?;
                let candidate = entry.path();
                let relative = candidate.strip_prefix(repo).unwrap_or(&candidate);
                if candidate.is_dir() {
                    stack.push(candidate);
                    continue;
                }
                let text = relative.to_string_lossy();
                let forbidden = (text.starts_with("adl/tools/")
                    && (text.ends_with("/pr.sh")
                        || text.contains("/check_pr_")
                        || text.contains("/test_pr_")
                        || (text.contains("/test_prompt_template") && text.ends_with(".sh"))
                        || text.ends_with("/prompt_template.sh")
                        || text.ends_with("/validate_structured_prompt.sh")
                        || text.ends_with("/card_paths.sh")
                        || text.ends_with("/pr_cards.sh")
                        || text.ends_with("/pr_delegate.sh")
                        || text.ends_with("/pr_usage.sh")))
                    || text.starts_with("adl/src/cli/pr_cmd")
                    || text == "adl/src/csdlc_prompt_editor.rs"
                    || text == "adl/src/pr_dispatch_support.rs"
                    || (text.starts_with("adl/src/bin/")
                        && (text.contains("/adl_pr_")
                            || matches!(
                                text.as_ref(),
                                "adl/src/bin/adl_csdlc.rs"
                                    | "adl/src/bin/csdlc.rs"
                                    | "adl/src/bin/adl_issue.rs"
                                    | "adl/src/bin/adl_session.rs"
                            )))
                    || text == "csdlc-v2/src/bin/csdlc-import.rs";
                if forbidden {
                    found.push(relative.to_path_buf());
                }
            }
        }
    }
    Ok(found)
}

pub fn resolve_operator_generation(
    repo: &Path,
    issue: u64,
    requested: Option<Generation>,
) -> Result<Generation> {
    let coexistence: CoexistenceInventory = serde_json::from_str(COEXISTENCE).map_err(|e| {
        V2Error::new(
            ErrorCode::CorruptRecord,
            format!("invalid embedded coexistence inventory: {e}"),
        )
    })?;
    if coexistence.v1_sunset && requested == Some(Generation::V1) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "explicit v1 generation selection is forbidden after v1 sunset",
        ));
    }
    let manifest = SkillManifest::load()?;
    let selector_path = checked_repo_path(repo, Path::new(&manifest.generation_selector))?;
    if !is_regular_file(&selector_path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "tracked generation selector must be a regular file, not a symlink",
        ));
    }
    let selector_bytes = fs::read(selector_path).map_err(io_error)?;
    let mut selector: GenerationSelector = serde_json::from_slice(&selector_bytes)?;
    if selector.default_generation == Generation::V3 {
        let value: serde_json::Value = serde_json::from_slice(&selector_bytes)?;
        let contract_valid = value["schema"] == "csdlc.generation_selector.v2"
            && value["operational_authority"] == "csdlc-v3"
            && value["authority_issue"] == 505
            && value["authority_pull_request"] == 591
            && value["review_authority"] == "typed-v2-exact-head"
            && value["approval_authority"] == "merged-pr-591-closed-issue-505";
        if !contract_valid {
            return Err(V2Error::new(
                ErrorCode::ValidationFailed,
                "v3 generation selector does not satisfy the canonical #505/#591 authority contract",
            ));
        }
        let remote = std::process::Command::new("git")
            .arg("-C")
            .arg(repo)
            .args([
                "show",
                "refs/remotes/origin/main:csdlc-v2/operator/generation-selector.json",
            ])
            .output()
            .map_err(io_error)?;
        if !remote.status.success() || remote.stdout != selector_bytes {
            selector.default_generation = Generation::V2;
        }
    }
    select_generation(&selector, issue, requested)
}

pub fn install_binaries(source: &Path, destination: &Path) -> Result<InstallReceipt> {
    install_binaries_with_revision(source, destination, None, None)
}

pub fn build_and_install_binaries(repo: &Path, destination: &Path) -> Result<InstallReceipt> {
    let manifest_path = repo.join("csdlc-v2/Cargo.toml");
    if !is_regular_file(&manifest_path) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "repository is missing csdlc-v2/Cargo.toml",
        ));
    }
    let before = crate::git::run(repo, &["rev-parse", "HEAD"])?;
    if !owner_binary_sources_are_clean(repo)? {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "refusing to stamp owner binaries from dirty C-SDLC owner sources",
        ));
    }
    if destination.is_dir() {
        let inventory = CoexistenceInventory::load()?;
        if verify_coexistence(repo, destination, &inventory)
            .map(|report| report.pass)
            .unwrap_or(false)
        {
            let receipt: InstallReceipt = serde_json::from_slice(
                &fs::read(destination.join("install-receipt.json")).map_err(io_error)?,
            )?;
            if receipt.source_set_digest == owner_source_set_digest(repo)? {
                return Ok(receipt);
            }
        }
    }
    let target = external_cargo_target(repo)?;
    let status = std::process::Command::new("cargo")
        .args(["build", "--locked", "--manifest-path"])
        .arg(&manifest_path)
        .arg("--bins")
        .env("CARGO_TARGET_DIR", &target)
        .current_dir(repo)
        .status()
        .map_err(io_error)?;
    if !status.success() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "cargo failed to build the typed C-SDLC binaries",
        ));
    }
    let after = crate::git::run(repo, &["rev-parse", "HEAD"])?;
    if before.stdout != after.stdout || !owner_binary_sources_are_clean(repo)? {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "C-SDLC owner source revision changed or became dirty during the build",
        ));
    }
    let source_set_digest = owner_source_set_digest(repo)?;
    install_binaries_with_revision(
        &target.join("debug"),
        destination,
        Some(format!("git:{}", after.stdout)),
        Some(source_set_digest),
    )
}

pub(crate) fn external_cargo_target(repo: &Path) -> Result<PathBuf> {
    let target = std::env::var_os("CARGO_TARGET_DIR").ok_or_else(|| {
        V2Error::new(
            ErrorCode::ValidationFailed,
            "CARGO_TARGET_DIR must name the validated external Cargo target",
        )
    })?;
    validate_external_cargo_target(repo, Path::new(&target))
}

pub fn validate_external_cargo_target(repo: &Path, target: &Path) -> Result<PathBuf> {
    if !target.is_absolute() || target.as_os_str().is_empty() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "CARGO_TARGET_DIR must be an absolute existing directory",
        ));
    }
    let canonical_repo = fs::canonicalize(repo).map_err(io_error)?;
    let canonical_target = fs::canonicalize(target).map_err(|_| {
        V2Error::new(
            ErrorCode::ValidationFailed,
            "CARGO_TARGET_DIR must be an absolute existing directory",
        )
    })?;
    if canonical_target != target
        || canonical_target == canonical_repo
        || canonical_target.starts_with(&canonical_repo)
    {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "CARGO_TARGET_DIR must resolve exactly outside the repository",
        ));
    }
    if !fs::metadata(&canonical_target).map_err(io_error)?.is_dir() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "CARGO_TARGET_DIR must be an absolute existing directory",
        ));
    }
    Ok(canonical_target)
}

fn owner_binary_sources_are_clean(repo: &Path) -> Result<bool> {
    let status = crate::git::run(
        repo,
        &[
            "status",
            "--porcelain=v1",
            "--untracked-files=all",
            "--",
            "csdlc-v2",
            "adl-resilience",
        ],
    )?;
    Ok(status.stdout.trim().is_empty())
}

fn install_binaries_with_revision(
    source: &Path,
    destination: &Path,
    trusted_revision: Option<String>,
    trusted_source_set_digest: Option<String>,
) -> Result<InstallReceipt> {
    let manifest = SkillManifest::load()?;
    if destination.file_name().and_then(|value| value.to_str()) != Some("csdlc-v2") {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "v2 binaries must install into a dedicated generation directory named csdlc-v2 (normally .adl/bin/csdlc-v2); shared directories such as .adl/bin are forbidden",
        ));
    }
    let names = manifest.binaries();
    let mut prepared = Vec::new();
    for name in &names {
        let input = source.join(name);
        if !is_regular_file(&input) || !is_executable(&input) {
            return Err(V2Error::new(
                ErrorCode::Io,
                format!(
                    "built binary is missing, non-regular, or non-executable: {}",
                    input.display()
                ),
            ));
        }
        let bytes = fs::read(&input).map_err(io_error)?;
        let permissions = fs::metadata(&input).map_err(io_error)?.permissions();
        prepared.push((name.to_string(), bytes, permissions));
    }
    let binaries = prepared
        .iter()
        .map(|(name, bytes, _)| InstalledBinary {
            name: name.clone(),
            blake3: blake3::hash(bytes).to_hex().to_string(),
        })
        .collect();
    let source_revision = trusted_revision.unwrap_or_else(|| content_provenance(&prepared));
    let mut receipt = InstallReceipt {
        schema: "csdlc.install_receipt.v2".into(),
        destination: destination.to_path_buf(),
        source_revision,
        source_set_schema: "csdlc.owner_source_set.v1".into(),
        source_set_digest: trusted_source_set_digest.unwrap_or_else(|| "content-only".into()),
        binaries,
    };
    let parent = destination.parent().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "installation destination needs a parent",
        )
    })?;
    fs::create_dir_all(parent).map_err(io_error)?;
    let leaf = destination
        .file_name()
        .and_then(|value| value.to_str())
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "invalid installation destination"))?;
    let stage = parent.join(format!(".{leaf}.stage-{}", std::process::id()));
    let backup = parent.join(format!(".{leaf}.backup-{}", std::process::id()));
    if stage.exists() || backup.exists() {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "stale install stage or backup requires reconciliation",
        ));
    }
    fs::create_dir(&stage).map_err(io_error)?;
    receipt.destination = destination.to_path_buf();
    for (name, bytes, permissions) in prepared {
        let output = stage.join(name);
        fs::write(&output, bytes).map_err(io_error)?;
        fs::set_permissions(&output, permissions).map_err(io_error)?;
    }
    fs::write(
        stage.join("install-receipt.json"),
        serde_json::to_vec_pretty(&receipt)
            .map_err(|e| V2Error::new(ErrorCode::Io, e.to_string()))?,
    )
    .map_err(io_error)?;
    verify_staged_install(&stage, &manifest, &receipt)?;
    let had_destination = destination.exists();
    if had_destination {
        fs::rename(destination, &backup).map_err(io_error)?;
    }
    if let Err(error) = fs::rename(&stage, destination) {
        if had_destination {
            let _ = fs::rename(&backup, destination);
        }
        return Err(io_error(error));
    }
    if had_destination {
        fs::remove_dir_all(&backup).map_err(io_error)?;
    }
    Ok(receipt)
}

fn checked_repo_path(repo: &Path, relative: &Path) -> Result<PathBuf> {
    if relative.is_absolute()
        || relative
            .components()
            .any(|part| matches!(part, std::path::Component::ParentDir))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "coexistence paths must be repo-relative without parent traversal",
        ));
    }
    Ok(repo.join(relative))
}

fn verify_install_receipt(
    repo: &Path,
    bin_dir: &Path,
    manifest: &SkillManifest,
) -> Result<BTreeSet<String>> {
    let path = bin_dir.join("install-receipt.json");
    if !is_regular_file(&path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt must be a regular non-symlink file",
        ));
    }
    let receipt: InstallReceipt = serde_json::from_slice(&fs::read(&path).map_err(io_error)?)?;
    if receipt.schema != "csdlc.install_receipt.v2" || receipt.destination != bin_dir {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt schema or destination does not match the verified generation directory",
        ));
    }
    verify_receipt_identity(repo, bin_dir, manifest, &receipt)?;
    let expected = manifest.required_binaries();
    let observed = receipt
        .binaries
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<BTreeSet<_>>();
    if observed != expected || observed.len() != receipt.binaries.len() {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt must contain the exact unique executable set",
        ));
    }
    let mut failures = BTreeSet::new();
    for entry in receipt.binaries {
        let binary = bin_dir.join(&entry.name);
        if !is_regular_file(&binary)
            || !is_executable(&binary)
            || fs::read(&binary)
                .map(|bytes| blake3::hash(&bytes).to_hex().as_str() != entry.blake3)
                .unwrap_or(true)
        {
            failures.insert(entry.name);
        }
    }
    Ok(failures)
}

fn verify_receipt_identity(
    repo: &Path,
    bin_dir: &Path,
    manifest: &SkillManifest,
    receipt: &InstallReceipt,
) -> Result<()> {
    if receipt.schema != "csdlc.install_receipt.v2"
        || receipt.destination != bin_dir
        || receipt.source_set_schema != "csdlc.owner_source_set.v1"
    {
        return Err(stale_owner_error("installed receipt identity is invalid"));
    }
    let current = owner_source_set_digest(repo)
        .map_err(|_| stale_owner_error("current owner source identity is unavailable"))?;
    if receipt.source_set_digest != current {
        return Err(stale_owner_error(&format!(
            "installed source set {} differs from current {}",
            receipt.source_set_digest, current
        )));
    }
    let expected = manifest.required_binaries();
    let observed = receipt
        .binaries
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<BTreeSet<_>>();
    if observed != expected || observed.len() != receipt.binaries.len() {
        return Err(stale_owner_error(
            "installed executable denominator is incomplete",
        ));
    }
    for entry in &receipt.binaries {
        let binary = bin_dir.join(&entry.name);
        if !is_regular_file(&binary)
            || !is_executable(&binary)
            || fs::read(&binary)
                .map(|bytes| blake3::hash(&bytes).to_hex().as_str() != entry.blake3)
                .unwrap_or(true)
        {
            return Err(stale_owner_error(&format!(
                "installed executable digest differs: {}",
                entry.name
            )));
        }
    }
    Ok(())
}

fn verify_staged_install(
    stage: &Path,
    manifest: &SkillManifest,
    receipt: &InstallReceipt,
) -> Result<()> {
    let expected = manifest.required_binaries();
    let observed = receipt
        .binaries
        .iter()
        .map(|entry| entry.name.clone())
        .collect::<BTreeSet<_>>();
    if observed != expected || observed.len() != receipt.binaries.len() {
        return Err(stale_owner_error(
            "staged executable denominator is incomplete",
        ));
    }
    for entry in &receipt.binaries {
        let binary = stage.join(&entry.name);
        if !is_regular_file(&binary)
            || !is_executable(&binary)
            || fs::read(&binary)
                .map(|bytes| blake3::hash(&bytes).to_hex().as_str() != entry.blake3)
                .unwrap_or(true)
        {
            return Err(stale_owner_error(&format!(
                "staged executable digest differs: {}",
                entry.name
            )));
        }
    }
    Ok(())
}

fn content_provenance(prepared: &[(String, Vec<u8>, fs::Permissions)]) -> String {
    let mut hasher = blake3::Hasher::new();
    for (name, bytes, _) in prepared {
        hasher.update(name.as_bytes());
        hasher.update(bytes);
    }
    format!("content:{}", hasher.finalize().to_hex())
}

fn is_regular_file(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_file())
        .unwrap_or(false)
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    fs::metadata(path)
        .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}
fn io_error(error: std::io::Error) -> V2Error {
    V2Error::new(ErrorCode::Io, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::{owner_source_set_digest, resolve_operator_generation};
    use crate::Generation;
    use std::{fs, path::Path};

    #[test]
    fn v1_override_is_rejected_after_sunset() {
        let error = resolve_operator_generation(std::path::Path::new("."), 1, Some(Generation::V1))
            .expect_err("sunset must reject explicit v1");
        assert!(error.message.contains("v1 sunset"));
    }

    #[test]
    fn tracked_v3_selector_activates_only_after_origin_main_contains_it() {
        let repo = tempfile::tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(
            repo.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(repo.path(), &["config", "user.name", "C-SDLC Test"]);
        let path = repo
            .path()
            .join("csdlc-v2/operator/generation-selector.json");
        fs::create_dir_all(path.parent().unwrap()).unwrap();
        fs::write(
            &path,
            br#"{"schema":"csdlc.generation_selector.v1","default_generation":"v2","opted_in_issues":[]}"#,
        )
        .unwrap();
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-m", "v2 selector"]);
        let v2_head = git_output(repo.path(), &["rev-parse", "HEAD"]);
        git(
            repo.path(),
            &["update-ref", "refs/remotes/origin/main", &v2_head],
        );

        let canonical_v3 = br#"{"schema":"csdlc.generation_selector.v2","default_generation":"v3","operational_authority":"csdlc-v3","authority_issue":505,"authority_pull_request":591,"review_authority":"typed-v2-exact-head","approval_authority":"merged-pr-591-closed-issue-505","opted_in_issues":[]}"#;
        fs::write(&path, canonical_v3).unwrap();
        assert_eq!(
            resolve_operator_generation(repo.path(), 505, None).unwrap(),
            Generation::V2
        );
        fs::write(
            &path,
            br#"{"schema":"csdlc.generation_selector.v2","default_generation":"v3","opted_in_issues":[]}"#,
        )
        .unwrap();
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-m", "malformed v3 selector"]);
        let malformed_head = git_output(repo.path(), &["rev-parse", "HEAD"]);
        git(
            repo.path(),
            &["update-ref", "refs/remotes/origin/main", &malformed_head],
        );
        assert!(resolve_operator_generation(repo.path(), 505, None).is_err());

        fs::write(&path, canonical_v3).unwrap();
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-m", "canonical v3 selector"]);
        let canonical_head = git_output(repo.path(), &["rev-parse", "HEAD"]);
        git(
            repo.path(),
            &["update-ref", "refs/remotes/origin/main", &canonical_head],
        );
        assert_eq!(
            resolve_operator_generation(repo.path(), 505, None).unwrap(),
            Generation::V3
        );
    }

    #[test]
    fn owner_source_digest_ignores_unrelated_commits_and_detects_owner_drift() {
        let repo = tempfile::tempdir().unwrap();
        git(repo.path(), &["init", "-b", "main"]);
        git(
            repo.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(repo.path(), &["config", "user.name", "C-SDLC Test"]);
        for path in [
            "csdlc-v2/Cargo.toml",
            "csdlc-v2/Cargo.lock",
            "csdlc-v2/src/lib.rs",
            "csdlc-v2/operator/skills.json",
            "adl-resilience/Cargo.toml",
            "adl-resilience/src/lib.rs",
            "docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json",
            "docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json",
        ] {
            let path = repo.path().join(path);
            fs::create_dir_all(path.parent().unwrap()).unwrap();
            fs::write(&path, format!("fixture:{path:?}\n")).unwrap();
        }
        git(repo.path(), &["add", "."]);
        git(repo.path(), &["commit", "-m", "owner sources"]);
        let baseline = owner_source_set_digest(repo.path()).unwrap();

        fs::write(repo.path().join("README.md"), "unrelated\n").unwrap();
        git(repo.path(), &["add", "README.md"]);
        git(repo.path(), &["commit", "-m", "unrelated docs"]);
        assert_eq!(owner_source_set_digest(repo.path()).unwrap(), baseline);

        fs::write(repo.path().join("csdlc-v2/src/lib.rs"), "owner drift\n").unwrap();
        let error = owner_source_set_digest(repo.path()).unwrap_err();
        assert!(error.message.contains("modified, deleted, or untracked"));
        git(repo.path(), &["checkout", "--", "csdlc-v2/src/lib.rs"]);

        fs::write(
            repo.path().join("csdlc-v2/src/untracked_owner.rs"),
            "untracked\n",
        )
        .unwrap();
        let error = owner_source_set_digest(repo.path()).unwrap_err();
        assert!(error.message.contains("modified, deleted, or untracked"));
    }

    fn git(root: &Path, args: &[&str]) {
        let status = std::process::Command::new("git")
            .args(args)
            .current_dir(root)
            .status()
            .unwrap();
        assert!(status.success(), "git {args:?}");
    }

    fn git_output(root: &Path, args: &[&str]) -> String {
        let output = std::process::Command::new("git")
            .args(args)
            .current_dir(root)
            .output()
            .unwrap();
        assert!(output.status.success(), "git {args:?}");
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    }
}
