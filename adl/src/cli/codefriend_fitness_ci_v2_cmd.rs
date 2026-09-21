use crate::cli::codefriend_fitness_cmd::{safe_path, FitnessExit};
use adl::codefriend::{
    evidence::store::Store,
    governance::{
        artifact::{FitnessArtifact, PolicyArtifact},
        ci::Expected,
        ci_v2::{self, Receipt},
        language,
    },
};
use anyhow::{ensure, Result};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::Path,
    process::{Child, Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};
const REPORT_LIMIT: usize = 4 * 1024 * 1024;
const LOG_LIMIT: usize = 256 * 1024;
fn now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn bytes(path: &Path, limit: usize) -> Result<Vec<u8>> {
    let path = safe_path(path)?;
    ensure!(fs::metadata(&path)?.is_file(), "ci_regular_file_required");
    let mut b = Vec::new();
    File::open(path)?
        .take(limit as u64 + 1)
        .read_to_end(&mut b)?;
    ensure!(b.len() <= limit, "ci_input_limit");
    Ok(b)
}
pub(super) fn selected(args: &[String], execute: bool) -> Result<bool> {
    let key = if execute { "--policy" } else { "--input" };
    let path = args
        .windows(2)
        .find(|p| p[0] == key)
        .ok_or_else(|| anyhow::anyhow!("ci_input_required"))?;
    let b = bytes(Path::new(&path[1]), 16 * 1024 * 1024)?;
    if execute {
        Ok(matches!(
            serde_json::from_slice::<PolicyArtifact>(&b)?,
            PolicyArtifact::V2(_)
        ))
    } else {
        Ok(matches!(
            serde_json::from_slice::<FitnessArtifact>(&b)?,
            FitnessArtifact::V2(_)
        ))
    }
}
fn flags(args: &[String], execute: bool) -> Result<BTreeMap<&str, &str>> {
    let expected: &[&str] = if execute {
        &[
            "--store",
            "--packet-id",
            "--policy",
            "--candidate",
            "--policy-digest",
            "--out",
        ]
    } else {
        &[
            "--store",
            "--input",
            "--candidate",
            "--packet-id",
            "--policy-digest",
            "--runner-exit",
            "--out",
        ]
    };
    let mut map = BTreeMap::new();
    for p in args.chunks(2) {
        ensure!(
            p.len() == 2 && expected.contains(&p[0].as_str()) && !p[1].starts_with("--"),
            "invalid_ci_arguments"
        );
        ensure!(
            map.insert(p[0].as_str(), p[1].as_str()).is_none(),
            "duplicate_ci_argument"
        );
    }
    ensure!(map.len() == expected.len(), "missing_ci_argument");
    Ok(map)
}
fn new_file(path: &Path) -> Result<File> {
    let path = safe_path(path)?;
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    Ok(options.open(path)?)
}
fn write_new(path: &Path, data: &[u8]) -> Result<()> {
    let mut f = new_file(path)?;
    f.write_all(data)?;
    f.sync_all()?;
    Ok(())
}
fn owned(path: &Path, identity: &same_file::Handle) -> bool {
    safe_path(path).is_ok() && same_file::Handle::from_path(path).is_ok_and(|h| &h == identity)
}
fn report(path: &Path) -> Result<language::Report> {
    match serde_json::from_slice::<FitnessArtifact>(&bytes(path, REPORT_LIMIT)?)? {
        FitnessArtifact::V2(r) => Ok(r),
        _ => anyhow::bail!("ci_report_schema_mismatch"),
    }
}
fn verify_write(
    store_path: &Path,
    input: &Path,
    expected: &Expected,
    original: i32,
    output: &Path,
) -> Result<Receipt> {
    let result = (|| {
        ensure!(
            store_path.join(".codefriend-store-v1").is_file(),
            "existing_store_required"
        );
        let r = report(input)?;
        let store = Store::open(store_path, now)?;
        let receipt = ci_v2::verify(&store, &r, expected, original, now())?;
        Ok::<_, anyhow::Error>((receipt, store, r))
    })();
    let receipt = result
        .as_ref()
        .map(|v| v.0.clone())
        .unwrap_or_else(|_| Receipt::rejected(original));
    let b = serde_json::to_vec_pretty(&receipt)?;
    ensure!(b.len() <= 32768, "ci_receipt_limit");
    let mut f = new_file(output)?;
    let identity = same_file::Handle::from_file(f.try_clone()?)?;
    let saved = (|| {
        f.write_all(&b)?;
        f.sync_all()?;
        ensure!(owned(output, &identity), "ci_receipt_replaced");
        let saved: Receipt = serde_json::from_slice(&bytes(output, 32768)?)?;
        ensure!(saved == receipt, "ci_receipt_readback_failed");
        if let Ok((_, store, r)) = &result {
            let current = report(input)?;
            ensure!(current == *r, "ci_report_changed");
            ensure!(
                store.get(&expected.packet_id)?.digest == r.record.admission.digest,
                "ci_admission_changed"
            );
        }
        ensure!(owned(output, &identity), "ci_receipt_replaced");
        Ok(saved)
    })();
    if saved.is_err() && owned(output, &identity) {
        fs::remove_file(output)?;
    }
    saved
}
struct OwnedChild(Child);
impl Drop for OwnedChild {
    fn drop(&mut self) {
        if self.0.try_wait().ok().flatten().is_none() {
            let _ = self.0.kill();
            let _ = self.0.wait();
        }
    }
}
fn capture(mut input: impl Read, mut file: File, cap: usize, failed: &AtomicBool) -> Result<()> {
    let result = (|| {
        let mut count = 0;
        let mut b = [0u8; 8192];
        loop {
            let n = input.read(&mut b)?;
            if n == 0 {
                break;
            }
            ensure!(count + n <= cap, "ci_runner_output_limit");
            file.write_all(&b[..n])?;
            count += n;
        }
        file.sync_all()?;
        Ok(())
    })();
    if result.is_err() {
        failed.store(true, Ordering::SeqCst);
    }
    result
}
fn child_run(argv: &[String], out: &Path) -> Result<i32> {
    let stdout = new_file(&out.join("runner.stdout.json"))?;
    let stderr = new_file(&out.join("runner.stderr.log"))?;
    let mut child = OwnedChild(
        Command::new(std::env::current_exe()?)
            .args(argv)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?,
    );
    let input = child
        .0
        .stdout
        .take()
        .ok_or_else(|| anyhow::anyhow!("ci_stdout_missing"))?;
    let errors = child
        .0
        .stderr
        .take()
        .ok_or_else(|| anyhow::anyhow!("ci_stderr_missing"))?;
    let failed = Arc::new(AtomicBool::new(false));
    let f = failed.clone();
    let g = failed.clone();
    let a = thread::spawn(move || capture(input, stdout, REPORT_LIMIT, &f));
    let b = thread::spawn(move || capture(errors, stderr, LOG_LIMIT, &g));
    // No work-duration cutoff. Only an observed stream/resource failure stops our child.
    let status = loop {
        if failed.load(Ordering::SeqCst) {
            let _ = child.0.kill();
            break child.0.wait();
        }
        match child.0.try_wait() {
            Ok(Some(s)) => break Ok(s),
            Ok(None) => thread::sleep(Duration::from_millis(10)),
            Err(e) => {
                let _ = child.0.kill();
                let _ = child.0.wait();
                break Err(e);
            }
        }
    };
    let left = a
        .join()
        .map_err(|_| anyhow::anyhow!("ci_stdout_capture_failed"))?;
    let right = b
        .join()
        .map_err(|_| anyhow::anyhow!("ci_stderr_capture_failed"))?;
    left?;
    right?;
    Ok(status?.code().unwrap_or(-1))
}
fn inner(args: &[String], execute: bool) -> Result<Receipt> {
    let f = flags(args, execute)?;
    let expected = Expected {
        candidate: f["--candidate"].into(),
        packet_id: f["--packet-id"].into(),
        policy_digest: f["--policy-digest"].into(),
    };
    expected.validate()?;
    let store = safe_path(Path::new(f["--store"]))?;
    let out = safe_path(Path::new(f["--out"]))?;
    ensure!(
        !out.starts_with(&store) && !store.starts_with(&out),
        "ci_roots_overlap"
    );
    if !execute {
        return verify_write(
            &store,
            Path::new(f["--input"]),
            &expected,
            f["--runner-exit"].parse()?,
            &out,
        );
    }
    let policy: PolicyArtifact =
        serde_json::from_slice(&bytes(Path::new(f["--policy"]), REPORT_LIMIT)?)?;
    let PolicyArtifact::V2(policy) = policy else {
        anyhow::bail!("ci_policy_schema_mismatch")
    };
    policy.validate()?;
    ensure!(
        adl::codefriend::evidence::hash(&policy)? == expected.policy_digest,
        "ci_policy_mismatch"
    );
    ensure!(
        store.join(".codefriend-store-v1").is_file(),
        "existing_store_required"
    );
    {
        let owner = Store::open(&store, now)?;
        let a = owner.get(&expected.packet_id)?;
        ensure!(
            a.packet.revision == expected.candidate,
            "ci_candidate_mismatch"
        );
    }
    let mut directory = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        directory.mode(0o700);
    }
    directory.create(&out)?;
    write_new(
        &out.join("expectations.json"),
        &serde_json::to_vec_pretty(&expected)?,
    )?;
    let report = out.join("report.json");
    let argv = vec![
        "codefriend".into(),
        "fitness".into(),
        "run".into(),
        "--store".into(),
        f["--store"].into(),
        "--packet-id".into(),
        f["--packet-id"].into(),
        "--policy".into(),
        f["--policy"].into(),
        "--out".into(),
        report.to_string_lossy().into_owned(),
    ];
    let original = child_run(&argv, &out).unwrap_or(-1);
    write_new(
        &out.join("runner-exit.txt"),
        format!("{original}\n").as_bytes(),
    )?;
    verify_write(
        &store,
        &report,
        &expected,
        original,
        &out.join("receipt.json"),
    )
}
pub(super) fn run(args: &[String], execute: bool) -> Result<()> {
    let (exit, value) = match inner(args, execute) {
        Ok(r) => (r.exit_code, serde_json::to_value(r)?),
        Err(_) => (
            2,
            serde_json::json!({"schema":ci_v2::VERSION,"exit_code":2,"artifact_valid":false,"error":"fitness_ci_command_failed"}),
        ),
    };
    println!("{}", serde_json::to_string(&value)?);
    eprintln!("adl_event component=codefriend_fitness_ci exit_code={exit}");
    if exit == 0 {
        Ok(())
    } else {
        Err(FitnessExit(exit).into())
    }
}
