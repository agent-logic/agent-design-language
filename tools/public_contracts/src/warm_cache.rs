//! Conservative local dependency-cache acceleration; never validation proof.
use anyhow::{bail, ensure, Context, Result};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
#[derive(Clone)]
pub struct Options {
    pub source: PathBuf,
    pub destination: PathBuf,
    pub manifest: PathBuf,
    pub profile: String,
    pub dry_run: bool,
    pub replace: bool,
}
#[derive(Default)]
struct Counts {
    candidates: usize,
    linked: usize,
    dep_info: usize,
    fingerprints: usize,
    existing: usize,
    unmatched: usize,
    errors: usize,
    rolled_back: usize,
}
fn read_toml(path: &Path) -> Result<toml::Value> {
    toml::from_str(&fs::read_to_string(path)?).with_context(|| format!("parse {}", path.display()))
}
fn package_name(v: &toml::Value) -> Option<&str> {
    v.get("package")?.get("name")?.as_str()
}
fn prefixes(manifest: &Path) -> Result<BTreeSet<String>> {
    let document = read_toml(manifest)?;
    let directory = manifest.parent().context("manifest has no parent")?;
    let mut workspace = BTreeSet::new();
    if let Some(name) = package_name(&document) {
        workspace.insert(name.to_owned());
    }
    if let Some(members) = document.get("workspace").and_then(|v| v.get("members")) {
        for member in members
            .as_array()
            .context("workspace members must be array")?
        {
            let member = member.as_str().context("workspace member must be string")?;
            // Do not guess membership patterns: reject instead of linking a local crate.
            ensure!(
                !member.contains(['*', '?', '[']),
                "wildcard workspace members require explicit cache classification"
            );
            let path = directory.join(member).join("Cargo.toml");
            if path.exists() {
                if let Some(name) = package_name(&read_toml(&path)?) {
                    workspace.insert(name.to_owned());
                }
            }
        }
    }
    let mut lock = directory.join("Cargo.lock");
    if !lock.exists() {
        lock = directory
            .parent()
            .context("Cargo.lock missing")?
            .join("Cargo.lock");
    }
    let lock = read_toml(&lock)?;
    let mut result = BTreeSet::new();
    for package in lock
        .get("package")
        .and_then(|v| v.as_array())
        .context("Cargo.lock packages missing")?
    {
        let name = package
            .get("name")
            .and_then(|v| v.as_str())
            .context("package name missing")?;
        if !workspace.contains(name) {
            result.insert(name.replace('-', "_"));
        }
    }
    Ok(result)
}
fn matches(path: &Path, prefixes: &BTreeSet<String>) -> bool {
    let Some(name) = path.file_name().and_then(|s| s.to_str()) else {
        return false;
    };
    let stem = name.split('.').next().unwrap_or(name);
    let stem = stem.strip_prefix("lib").unwrap_or(stem);
    prefixes.iter().any(|prefix| {
        stem == prefix
            || stem
                .strip_prefix(prefix)
                .is_some_and(|s| s.starts_with('-'))
    })
}
fn entries(path: &Path) -> Result<Vec<PathBuf>> {
    if !path.exists() {
        return Ok(vec![]);
    }
    ensure!(
        !fs::symlink_metadata(path)?.file_type().is_symlink(),
        "cache directory must not be a symlink: {}",
        path.display()
    );
    let mut paths = fs::read_dir(path)?
        .map(|e| e.map(|e| e.path()))
        .collect::<std::io::Result<Vec<_>>>()?;
    paths.sort();
    Ok(paths)
}
fn fingerprint_files(path: &Path, out: &mut Vec<PathBuf>) -> Result<()> {
    for entry in entries(path)? {
        let meta = fs::symlink_metadata(&entry)?;
        ensure!(
            !meta.file_type().is_symlink(),
            "cache artifact must not be a symlink: {}",
            entry.display()
        );
        if meta.is_dir() {
            fingerprint_files(&entry, out)?;
        } else if meta.is_file() {
            out.push(entry);
        }
    }
    Ok(())
}
fn candidates(
    profile: &Path,
    prefixes: &BTreeSet<String>,
    counts: &mut Counts,
) -> Result<Vec<(PathBuf, PathBuf)>> {
    let mut result = vec![];
    for source in entries(&profile.join("deps"))? {
        let meta = fs::symlink_metadata(&source)?;
        ensure!(
            !meta.file_type().is_symlink(),
            "cache artifact must not be a symlink: {}",
            source.display()
        );
        if !meta.is_file() {
            continue;
        }
        if source.extension().is_some_and(|s| s == "d") {
            counts.dep_info += 1;
        } else if matches(&source, prefixes) {
            result.push((source.clone(), source.strip_prefix(profile)?.to_path_buf()));
        } else {
            counts.unmatched += 1;
        }
    }
    for directory in entries(&profile.join(".fingerprint"))? {
        ensure!(
            !fs::symlink_metadata(&directory)?.file_type().is_symlink(),
            "fingerprint directory must not be a symlink"
        );
        if !directory.is_dir() || !matches(&directory, prefixes) {
            continue;
        }
        let mut files = vec![];
        fingerprint_files(&directory, &mut files)?;
        for source in files {
            if source.extension().is_some_and(|s| s == "d") {
                counts.dep_info += 1;
                continue;
            }
            counts.fingerprints += 1;
            result.push((source.clone(), source.strip_prefix(profile)?.to_path_buf()));
        }
    }
    Ok(result)
}
fn resolve(path: &Path) -> Result<PathBuf> {
    if path.exists() {
        return Ok(path.canonicalize()?);
    }
    let path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()?.join(path)
    };
    Ok(resolve(path.parent().context("invalid destination")?)?
        .join(path.file_name().context("invalid destination name")?))
}
fn safe_parent(parent: &Path, root: &Path) -> Result<()> {
    let mut ancestor = parent;
    while !ancestor.exists() {
        ancestor = ancestor.parent().context("destination parent missing")?;
    }
    ensure!(
        ancestor.canonicalize()?.starts_with(root),
        "destination parent escapes selected target"
    );
    fs::create_dir_all(parent)?;
    ensure!(
        parent.canonicalize()?.starts_with(root),
        "destination parent escapes selected target"
    );
    Ok(())
}
#[cfg(unix)]
fn same_inode(a: &Path, b: &Path) -> bool {
    use std::os::unix::fs::MetadataExt;
    match (fs::metadata(a), fs::metadata(b)) {
        (Ok(a), Ok(b)) => (a.dev(), a.ino()) == (b.dev(), b.ino()),
        _ => false,
    }
}
#[cfg(not(unix))]
fn same_inode(_a: &Path, _b: &Path) -> bool {
    false
}
pub fn warm(options: &Options) -> Result<Value> {
    ensure!(
        matches!(
            Path::new(&options.profile).components().next(),
            Some(std::path::Component::Normal(_))
        ) && Path::new(&options.profile).components().count() == 1,
        "profile must be one directory name"
    );
    let manifest = options
        .manifest
        .canonicalize()
        .context("manifest path missing")?;
    let source = options
        .source
        .canonicalize()
        .context("source target missing")?;
    let destination = resolve(&options.destination)?;
    ensure!(
        source.join(&options.profile).is_dir(),
        "source profile directory missing"
    );
    ensure!(
        !fs::symlink_metadata(source.join(&options.profile))?
            .file_type()
            .is_symlink(),
        "source profile must not be a symlink"
    );
    let prefixes = prefixes(&manifest)?;
    let mut counts = Counts::default();
    let candidates = candidates(&source.join(&options.profile), &prefixes, &mut counts)?;
    let mut linked = vec![];
    let mut replacements: Vec<(PathBuf, tempfile::TempDir)> = vec![];
    if !options.dry_run {
        fs::create_dir_all(&destination)?;
    }
    for (source_file, relative) in candidates {
        counts.candidates += 1;
        let dest = destination.join(&options.profile).join(relative);
        let exists = fs::symlink_metadata(&dest).is_ok();
        if exists && (same_inode(&source_file, &dest) || !options.replace) {
            counts.existing += 1;
            continue;
        }
        if options.dry_run {
            counts.linked += 1;
            continue;
        }
        let result = (|| -> Result<()> {
            let parent = dest.parent().context("artifact parent missing")?;
            safe_parent(parent, &destination)?;
            if exists {
                ensure!(
                    !fs::symlink_metadata(&dest)?.is_dir(),
                    "artifact destination is a directory"
                );
                let backup = tempfile::Builder::new()
                    .prefix(".adl-warm-backup-")
                    .tempdir_in(parent)?;
                fs::rename(&dest, backup.path().join("original"))?;
                replacements.push((dest.clone(), backup));
            }
            fs::hard_link(&source_file, &dest)?;
            linked.push(dest.clone());
            counts.linked += 1;
            Ok(())
        })();
        if let Err(error) = result {
            counts.errors += 1;
            eprintln!(
                "error: failed to hardlink {} -> {}: {error:#}",
                source_file.display(),
                dest.display()
            );
        }
    }
    if counts.errors > 0 {
        for dest in linked.iter().rev() {
            match fs::remove_file(dest) {
                Ok(()) => counts.rolled_back += 1,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
                Err(e) => {
                    counts.errors += 1;
                    eprintln!("error: rollback {}: {e}", dest.display());
                }
            }
        }
        for (dest, backup) in replacements.into_iter().rev() {
            // Preserve the only original copy if restoration itself fails.
            if let Err(e) = fs::rename(backup.path().join("original"), &dest) {
                counts.errors += 1;
                let retained = backup.keep();
                eprintln!(
                    "error: restore {}: {e}; original retained at {}",
                    dest.display(),
                    retained.display()
                );
            }
        }
    }
    Ok(
        json!({"status":if counts.errors==0 {"ok"} else {"completed_with_errors"},"source_target":source,"dest_target":destination,"manifest_path":manifest,"profile":options.profile,"dry_run":options.dry_run,"replace":options.replace,"external_packages":prefixes.len(),"external_target_prefixes":prefixes.len(),"candidate_files":counts.candidates,"linked_files":counts.linked,"skipped_dep_info":counts.dep_info,"linked_fingerprint_files":counts.fingerprints,"skipped_existing":counts.existing,"skipped_unmatched":counts.unmatched,"errors":counts.errors,"rolled_back_files":counts.rolled_back,"validation_proof":false}),
    )
}
pub fn cli(args: &[String]) -> i32 {
    let json_output = args.iter().any(|s| s == "--json");
    let result = (|| -> Result<Value> {
        let mut options = Options {
            source: PathBuf::new(),
            destination: PathBuf::new(),
            manifest: "adl/Cargo.toml".into(),
            profile: "debug".into(),
            dry_run: false,
            replace: false,
        };
        let mut seen = BTreeSet::new();
        let mut args = args.iter();
        while let Some(arg) = args.next() {
            ensure!(seen.insert(arg), "duplicate option {arg}");
            match arg.as_str() {
                "--json" => (),
                "--dry-run" => options.dry_run = true,
                "--replace" => options.replace = true,
                "--source-target" => {
                    options.source = args.next().context("source target missing")?.into()
                }
                "--dest-target" => {
                    options.destination = args.next().context("destination target missing")?.into()
                }
                "--manifest-path" => {
                    options.manifest = args.next().context("manifest path missing")?.into()
                }
                "--profile" => options.profile = args.next().context("profile missing")?.into(),
                _ => bail!("unknown cache option {arg}"),
            }
        }
        ensure!(
            !options.source.as_os_str().is_empty() && !options.destination.as_os_str().is_empty(),
            "--source-target and --dest-target are required"
        );
        warm(&options)
    })();
    match result {
        Ok(report) => {
            if json_output {
                println!("{report}");
            } else {
                println!(
                    "status={} candidate_files={} linked_files={} skipped_existing={} errors={}",
                    report["status"].as_str().unwrap(),
                    report["candidate_files"],
                    report["linked_files"],
                    report["skipped_existing"],
                    report["errors"]
                );
            }
            if report["errors"] == 0 {
                0
            } else {
                2
            }
        }
        Err(error) => {
            if json_output {
                println!("{}", json!({"status":"error","error":format!("{error:#}")}));
            } else {
                eprintln!("error: {error:#}");
            }
            1
        }
    }
}

pub fn wrapper(
    root: &Path,
    environment: &std::collections::BTreeMap<String, String>,
) -> Result<Value> {
    let adl = root.join("adl");
    let value = |key: &str, default: String| {
        environment
            .get(key)
            .filter(|s| !s.is_empty())
            .cloned()
            .unwrap_or(default)
    };
    let default_source = root
        .to_str()
        .and_then(|s| s.split_once("/.worktrees/adl-wp-"))
        .map(|(primary, _)| Path::new(primary).join("adl/target"))
        .filter(|p| p.is_dir())
        .unwrap_or_else(|| adl.join("target"));
    let source = PathBuf::from(value(
        "ADL_RUST_WARM_CACHE_SOURCE_TARGET",
        default_source.display().to_string(),
    ));
    let destination = PathBuf::from(value(
        "ADL_RUST_WARM_CACHE_DEST_TARGET",
        value("CARGO_TARGET_DIR", adl.join("target").display().to_string()),
    ));
    let manifest = PathBuf::from(value(
        "ADL_RUST_WARM_CACHE_MANIFEST_PATH",
        adl.join("Cargo.toml").display().to_string(),
    ));
    let profile = value("ADL_RUST_WARM_CACHE_PROFILE", "debug".into());
    let skipped = |reason: &str| json!({"status":"skipped","reason":reason,"source_target":source,"dest_target":destination,"validation_proof":false});
    if value("ADL_RUST_WARM_CACHE", "1".into()) == "0" {
        return Ok(skipped("disabled"));
    }
    if !manifest.is_file() {
        return Ok(skipped("manifest missing"));
    }
    if !source.join(&profile).join("deps").is_dir() {
        return Ok(skipped("source target cache missing"));
    }
    fs::create_dir_all(&destination)?;
    let source_real = source.canonicalize()?;
    let dest_real = destination.canonicalize()?;
    if source_real == dest_real {
        return Ok(skipped("source target is destination target"));
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::MetadataExt;
        if fs::metadata(&source_real)?.dev() != fs::metadata(&dest_real)?.dev() {
            return Ok(skipped("source and destination target are on different filesystems; hardlink warm cache skipped"));
        }
    }
    warm(&Options {
        source: source_real,
        destination: dest_real,
        manifest,
        profile,
        dry_run: false,
        replace: value("ADL_RUST_WARM_CACHE_REPLACE", "1".into()) != "0",
    })
}
