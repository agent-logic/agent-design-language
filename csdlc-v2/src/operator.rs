use crate::error::{ErrorCode, Result, V2Error};
use crate::{select_generation, Generation, GenerationSelector};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
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
    pub skills: Vec<SkillRoute>,
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
    pub required_v1_paths: Vec<RequiredPath>,
    pub required_v2_binaries: Vec<String>,
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
    pub missing_v2_binaries: Vec<String>,
    pub skill_count: usize,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstallReceipt {
    pub schema: String,
    pub destination: PathBuf,
    pub binaries: Vec<InstalledBinary>,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct InstalledBinary {
    pub name: String,
    pub blake3: String,
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
            || self.skills.len() != 9
        {
            return Err(V2Error::new(
                ErrorCode::InvalidManifest,
                "operator manifest must declare nine v2 skills bound to the tracked generation selector",
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
        Ok(())
    }

    fn binaries(&self) -> BTreeSet<&str> {
        self.skills
            .iter()
            .flat_map(|route| {
                std::iter::once(route.binary.as_str())
                    .chain(route.auxiliary_binaries.iter().map(String::as_str))
            })
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
    if inventory.schema != "csdlc.coexistence_inventory.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidManifest,
            "coexistence requires schema v1",
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
    let missing_v1_paths = inventory
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
        .collect::<Vec<_>>();
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
    missing_v2_binaries.extend(verify_install_receipt(bin_dir, &manifest)?);
    Ok(CoexistenceReport {
        schema: "csdlc.coexistence_report.v1".into(),
        pass: missing_v1_paths.is_empty() && missing_v2_binaries.is_empty(),
        default_generation: selector.default_generation,
        missing_v1_paths,
        missing_v2_binaries: missing_v2_binaries.into_iter().collect(),
        skill_count: manifest.skills.len(),
    })
}

pub fn resolve_operator_generation(
    repo: &Path,
    issue: u64,
    requested: Option<Generation>,
) -> Result<Generation> {
    let manifest = SkillManifest::load()?;
    let selector_path = checked_repo_path(repo, Path::new(&manifest.generation_selector))?;
    if !is_regular_file(&selector_path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "tracked generation selector must be a regular file, not a symlink",
        ));
    }
    let selector: GenerationSelector =
        serde_json::from_slice(&fs::read(selector_path).map_err(io_error)?)?;
    select_generation(&selector, issue, requested)
}

pub fn install_binaries(source: &Path, destination: &Path) -> Result<InstallReceipt> {
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
    let mut receipt = InstallReceipt {
        schema: "csdlc.install_receipt.v1".into(),
        destination: destination.to_path_buf(),
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

fn verify_install_receipt(bin_dir: &Path, manifest: &SkillManifest) -> Result<BTreeSet<String>> {
    let path = bin_dir.join("install-receipt.json");
    if !is_regular_file(&path) {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt must be a regular non-symlink file",
        ));
    }
    let receipt: InstallReceipt = serde_json::from_slice(&fs::read(&path).map_err(io_error)?)?;
    if receipt.schema != "csdlc.install_receipt.v1" || receipt.destination != bin_dir {
        return Err(V2Error::new(
            ErrorCode::ValidationFailed,
            "install receipt schema or destination does not match the verified generation directory",
        ));
    }
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
