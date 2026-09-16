use adl::codefriend::{
    evidence::store::Store,
    governance::local::{local_fitness_runner, Policy, Report, VERSION},
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
fn inner(args: &[String]) -> Result<Report> {
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
        let report: Report = read(Path::new(flags["--input"]), 16 * 1024 * 1024)?;
        report.validate(&store)?;
        return Ok(report);
    }
    let policy: Policy = read(Path::new(flags["--policy"]), 128 * 1024)?;
    let report = local_fitness_runner(&store, flags["--packet-id"], policy)?;
    let output = safe_path(Path::new(flags["--out"]))?;
    ensure!(
        !output.starts_with(&store_path),
        "fitness_output_inside_store"
    );
    let bytes = serde_json::to_vec_pretty(&report)?;
    ensure!(bytes.len() <= 16 * 1024 * 1024, "fitness_output_limit");
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&output)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    let saved: Report = read(&output, 16 * 1024 * 1024)?;
    saved.validate(&store)?;
    Ok(saved)
}
pub(super) fn run(args: &[String]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "ci-verify") {
        return super::codefriend_fitness_ci_cmd::run(&args[1..], false);
    }
    if args.first().is_some_and(|arg| arg == "ci-run") {
        return super::codefriend_fitness_ci_cmd::run(&args[1..], true);
    }
    let code = match inner(args) {
        Ok(report) => {
            println!("{}", serde_json::to_string(&report)?);
            report.status.exit_code()
        }
        Err(_) => {
            println!(
                "{}",
                serde_json::json!({"schema":VERSION,"status":"error","error":"fitness_command_failed","report_available":false})
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
