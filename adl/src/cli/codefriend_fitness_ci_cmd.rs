use super::codefriend_fitness_cmd::{read, safe_path, FitnessExit};
use adl::codefriend::{
    evidence::store::Store,
    governance::{
        ci::{self, Expected, Receipt},
        local::Report,
    },
};
use anyhow::{ensure, Result};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    io::Write,
    path::Path,
    process::{Command, Stdio},
};

fn inner(args: &[String]) -> Result<Receipt> {
    let expected = [
        "--store",
        "--input",
        "--candidate",
        "--packet-id",
        "--policy-digest",
        "--runner-exit",
        "--out",
    ];
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_ci_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_ci_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_ci_argument");
    let original_exit = flags["--runner-exit"].parse::<i32>()?;
    let output = safe_path(Path::new(flags["--out"]))?;
    let store_path = safe_path(Path::new(flags["--store"]))?;
    ensure!(!output.starts_with(&store_path), "ci_output_inside_store");
    let result = (|| -> Result<Receipt> {
        ensure!(
            store_path.join(".codefriend-store-v1").is_file(),
            "existing_store_required"
        );
        let report: Report = read(Path::new(flags["--input"]), 16 * 1024 * 1024)?;
        let store = Store::open(&store_path, || {
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs()
        })?;
        ci::verify(
            &store,
            &report,
            &Expected {
                candidate: flags["--candidate"].into(),
                packet_id: flags["--packet-id"].into(),
                policy_digest: flags["--policy-digest"].into(),
            },
            original_exit,
        )
    })();
    let receipt = result.unwrap_or_else(|_| Receipt::rejected(original_exit));
    let mut options = OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(&output)?;
    file.write_all(&serde_json::to_vec_pretty(&receipt)?)?;
    file.sync_all()?;
    let saved: Receipt = serde_json::from_slice(&fs::read(output)?)?;
    ensure!(saved == receipt, "ci_receipt_readback_failed");
    Ok(receipt)
}

fn execute(args: &[String]) -> Result<Receipt> {
    let expected = [
        "--store",
        "--packet-id",
        "--policy",
        "--candidate",
        "--policy-digest",
        "--out",
    ];
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_ci_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_ci_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_ci_argument");
    let output = safe_path(Path::new(flags["--out"]))?;
    let pins = Expected {
        candidate: flags["--candidate"].into(),
        packet_id: flags["--packet-id"].into(),
        policy_digest: flags["--policy-digest"].into(),
    };
    pins.validate()?;
    let store = safe_path(Path::new(flags["--store"]))?;
    ensure!(
        !output.starts_with(&store) && !store.starts_with(&output),
        "ci_roots_overlap"
    );
    let mut builder = fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder.create(&output)?;
    // Declared expectations remain available even when the runner emits no report.
    fs::write(
        output.join("expectations.json"),
        serde_json::to_vec_pretty(&pins)?,
    )?;
    let report = output.join("report.json");
    let status = Command::new(std::env::current_exe()?)
        .args([
            "codefriend",
            "fitness",
            "run",
            "--store",
            flags["--store"],
            "--packet-id",
            flags["--packet-id"],
            "--policy",
            flags["--policy"],
            "--out",
        ])
        .arg(&report)
        .stdin(Stdio::null())
        .stdout(fs::File::create(output.join("runner.stdout.json"))?)
        .stderr(fs::File::create(output.join("runner.stderr.log"))?)
        .status()?;
    let original = status.code().unwrap_or(-1);
    fs::write(output.join("runner-exit.txt"), format!("{original}\n"))?;
    let verify_args = [
        "--store".into(),
        flags["--store"].into(),
        "--input".into(),
        report.to_string_lossy().into_owned(),
        "--candidate".into(),
        flags["--candidate"].into(),
        "--packet-id".into(),
        flags["--packet-id"].into(),
        "--policy-digest".into(),
        flags["--policy-digest"].into(),
        "--runner-exit".into(),
        original.to_string(),
        "--out".into(),
        output.join("receipt.json").to_string_lossy().into_owned(),
    ];
    inner(&verify_args)
}

pub(super) fn run(args: &[String], execute_runner: bool) -> Result<()> {
    let receipt = if execute_runner {
        execute(args)
    } else {
        inner(args)
    };
    let exit = match receipt {
        Ok(receipt) => {
            println!("{}", serde_json::to_string(&receipt)?);
            receipt.exit_code
        }
        Err(_) => {
            println!(
                "{}",
                serde_json::json!({"schema":ci::VERSION,"exit_code":2,"artifact_valid":false,"error":"fitness_ci_command_failed"})
            );
            2
        }
    };
    eprintln!("adl_event component=codefriend_fitness_ci exit_code={exit}");
    if exit == 0 {
        Ok(())
    } else {
        Err(FitnessExit(exit).into())
    }
}
