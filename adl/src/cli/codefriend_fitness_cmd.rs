use adl::codefriend::{
    evidence::store::Store,
    governance::{
        artifact::{self, FitnessArtifact, PolicyArtifact},
        local::VERSION,
    },
};
use anyhow::{ensure, Result};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

pub(super) const USAGE: &str = "adl codefriend fitness run --store <dir> --packet-id <id> --policy <policy.json> --out <new-report.json>\n       adl codefriend fitness read --store <dir> --input <report.json>\n       adl codefriend fitness ci-run --store <dir> --packet-id <id> --policy <policy.json> --candidate <sha> --policy-digest <digest> --out <new-directory>\n       adl codefriend fitness ci-verify --store <dir> --input <report.json> --candidate <sha> --packet-id <id> --policy-digest <digest> --runner-exit <0|1|2> --out <new-receipt.json>";
#[derive(Debug)]
pub(super) struct FitnessExit(pub i32);
impl std::fmt::Display for FitnessExit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "fitness_exit_{}", self.0)
    }
}
impl std::error::Error for FitnessExit {}
pub(super) fn safe_path(path: &Path) -> Result<PathBuf> {
    let absolute = std::path::absolute(path)?;
    let mut current = PathBuf::new();
    for component in absolute.components() {
        ensure!(
            !matches!(component, Component::ParentDir),
            "fitness_parent_path_rejected"
        );
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(meta) => ensure!(!meta.file_type().is_symlink(), "fitness_symlink_rejected"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
            Err(e) => return Err(e.into()),
        }
    }
    Ok(absolute)
}
pub(super) fn read<T: serde::de::DeserializeOwned>(path: &Path, limit: u64) -> Result<T> {
    let path = safe_path(path)?;
    ensure!(
        fs::metadata(&path)?.is_file(),
        "fitness_regular_file_required"
    );
    let mut bytes = Vec::new();
    fs::File::open(path)?
        .take(limit + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "fitness_input_limit");
    Ok(serde_json::from_slice(&bytes)?)
}
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn read_bytes(path: &Path, limit: u64) -> Result<Vec<u8>> {
    let path = safe_path(path)?;
    ensure!(
        fs::metadata(&path)?.is_file(),
        "fitness_regular_file_required"
    );
    let mut b = Vec::new();
    fs::File::open(path)?.take(limit + 1).read_to_end(&mut b)?;
    ensure!(b.len() as u64 <= limit, "fitness_input_limit");
    Ok(b)
}
fn read_report(path: &Path, schema: &mut &'static str) -> Result<FitnessArtifact> {
    let bytes = read_bytes(path, 16 * 1024 * 1024)?;
    let value: FitnessArtifact = serde_json::from_slice(&bytes)?;
    *schema = value.schema();
    ensure!(bytes.len() <= value.byte_limit(), "fitness_input_limit");
    Ok(value)
}
fn same_output(path: &Path, handle: &same_file::Handle) -> bool {
    safe_path(path).is_ok()
        && fs::symlink_metadata(path).is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
        && same_file::Handle::from_path(path).is_ok_and(|current| &current == handle)
}
fn cleanup_created(path: &Path, handle: &same_file::Handle) -> Result<()> {
    if same_output(path, handle) {
        fs::remove_file(path)?;
    }
    Ok(())
}
fn persist<T>(output: &Path, bytes: &[u8], verify: impl FnOnce() -> Result<T>) -> Result<T> {
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(output)?;
    let handle = same_file::Handle::from_file(file.try_clone()?)?;
    let result = (|| {
        file.write_all(bytes)?;
        file.sync_all()?;
        ensure!(same_output(output, &handle), "fitness_output_replaced");
        let result = verify()?;
        ensure!(same_output(output, &handle), "fitness_output_replaced");
        Ok(result)
    })();
    if result.is_err() {
        cleanup_created(output, &handle)?;
    }
    result
}
fn response(report: &FitnessArtifact, store: &Store) -> Result<(i32, String)> {
    let text = String::from_utf8(report.json_bytes(false)?)?;
    ensure!(
        store.get(&report.record().run.packet_id)?.digest == report.record().admission.digest,
        "fitness_admission_changed"
    );
    Ok((report.exit_code(), text))
}
fn inner(args: &[String], error_schema: &mut &'static str) -> Result<(i32, String)> {
    let expected: &[&str] = match args.first().map(String::as_str) {
        Some("run") => &["--store", "--packet-id", "--policy", "--out"],
        Some("read") => &["--store", "--input"],
        _ => anyhow::bail!("invalid_fitness_command"),
    };
    let mut flags = BTreeMap::new();
    for pair in args[1..].chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_fitness_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_fitness_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_fitness_argument");
    let store_path = safe_path(Path::new(flags["--store"]))?;
    ensure!(
        store_path.join(".codefriend-store-v1").is_file(),
        "existing_evidence_store_required"
    );
    let store = Store::open(&store_path, || {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs()
    })?;
    if args[0] == "read" {
        let report = read_report(Path::new(flags["--input"]), error_schema)?;
        report.validate(&store, now())?;
        return response(&report, &store);
    }
    let policy_bytes = read_bytes(Path::new(flags["--policy"]), 4 * 1024 * 1024)?;
    let policy: PolicyArtifact = serde_json::from_slice(&policy_bytes)?;
    match &policy {
        PolicyArtifact::V1(_) => {
            ensure!(policy_bytes.len() <= 128 * 1024, "fitness_input_limit");
        }
        PolicyArtifact::V2(_) => {
            *error_schema = adl::codefriend::governance::language::VERSION;
        }
    }
    let report = artifact::evaluate(&store, flags["--packet-id"], policy, now())?;
    *error_schema = report.schema();
    let output = safe_path(Path::new(flags["--out"]))?;
    ensure!(
        !output.starts_with(&store_path),
        "fitness_output_inside_store"
    );
    let bytes = report.json_bytes(true)?;
    ensure!(
        store.get(flags["--packet-id"])?.digest == report.record().admission.digest,
        "fitness_admission_changed"
    );
    persist(&output, &bytes, || {
        let saved = read_report(&output, error_schema)?;
        saved.validate(&store, now())?;
        ensure!(saved.digest() == report.digest(), "fitness_output_changed");
        response(&saved, &store)
    })
}
pub(super) fn run(args: &[String]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "ci-verify") {
        return super::codefriend_fitness_ci_cmd::run(&args[1..], false);
    }
    if args.first().is_some_and(|arg| arg == "ci-run") {
        return super::codefriend_fitness_ci_cmd::run(&args[1..], true);
    }
    let mut error_schema = VERSION;
    let code = match inner(args, &mut error_schema) {
        Ok((code, text)) => {
            println!("{text}");
            code
        }
        Err(_) => {
            println!(
                "{}",
                serde_json::json!({"schema":error_schema,"status":"error","error":"fitness_command_failed","report_available":false})
            );
            2
        }
    };
    eprintln!("adl_event component=codefriend_fitness exit_code={code}");
    if code == 0 {
        Ok(())
    } else {
        Err(FitnessExit(code).into())
    }
}

#[cfg(test)]
mod output_cleanup_tests {
    use super::*;
    #[test]
    fn cleanup_preserves_replacement_and_removes_only_owned_inode() {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let path = temp.path().join("fresh.json");
        let old = temp.path().join("old.json");
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .unwrap();
        let handle = same_file::Handle::from_file(file).unwrap();
        fs::rename(&path, &old).unwrap();
        fs::write(&path, b"replacement").unwrap();
        cleanup_created(&path, &handle).unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"replacement");
        let fresh = temp.path().join("owned.json");
        let file = OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&fresh)
            .unwrap();
        let handle = same_file::Handle::from_file(file).unwrap();
        cleanup_created(&fresh, &handle).unwrap();
        assert!(!fresh.exists());
    }
    #[test]
    fn failed_verification_cleans_owned_output() {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let path = temp.path().join("invalid.json");
        let result: Result<()> = persist(&path, b"retained report", || {
            assert_eq!(fs::read(&path)?, b"retained report");
            anyhow::bail!("fitness_admission_changed")
        });
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("fitness_admission_changed"));
        assert!(!path.exists());
    }
}
