use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::Instant,
};

use adl_runtime::guardian::{run_guardian, GuardianConfig, GuardianOutcome, GuardianTerminalState};
use sha2::{Digest, Sha256};
use tokio_util::sync::CancellationToken;

const REPORT_SCHEMA: &str = "adl.runtime_v3.lifecycle_soak.v1";
const REQUIRED_CYCLES: u64 = 10_000;

#[tokio::main]
async fn main() -> ExitCode {
    let args = match Args::parse(std::env::args().skip(1)) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    if let Err(error) = prepare_state_root(&args.state_root) {
        eprintln!("{error}");
        return ExitCode::from(64);
    }

    let started = Instant::now();
    let capsule = args.state_root.join("runtime-continuity.json");
    let kernel_sha256 = match file_sha256(&args.kernel) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("failed hashing Runtime v3 kernel: {error}");
            return ExitCode::from(66);
        }
    };

    for cycle in 1..=args.cycles {
        let config = GuardianConfig {
            program: args.kernel.clone(),
            args: vec!["demo".to_owned(), capsule.to_string_lossy().into_owned()],
            env: Vec::new(),
            restart_budget: 0,
            backoff_base_ms: 1,
            backoff_cap_ms: 1,
            shutdown_grace_ms: 10_000,
            configuration_exit_codes: vec![64, 78],
        };
        let outcome = match run_guardian(config, CancellationToken::new()).await {
            Ok(outcome) => outcome,
            Err(error) => {
                return fail(&args, &kernel_sha256, started, cycle, format!("{error:?}"));
            }
        };
        if let Err(error) = validate_cycle(&outcome) {
            return fail(&args, &kernel_sha256, started, cycle, error);
        }
        if cycle % 1_000 == 0 {
            eprintln!(
                "guardian_runtime_lifecycle_progress={cycle}/{}",
                args.cycles
            );
        }
    }

    let generation = match continuity_generation(&capsule) {
        Ok(generation) => generation,
        Err(error) => {
            return fail(
                &args,
                &kernel_sha256,
                started,
                args.cycles,
                format!("continuity verification failed: {error}"),
            );
        }
    };
    if generation != args.cycles {
        return fail(
            &args,
            &kernel_sha256,
            started,
            args.cycles,
            format!(
                "continuity generation {generation} did not equal completed cycles {}",
                args.cycles
            ),
        );
    }

    let report = report(
        &args,
        &kernel_sha256,
        started,
        "pass",
        args.cycles,
        generation,
        None,
    );
    if let Err(error) = write_report(&args.report, &report) {
        eprintln!("failed writing lifecycle report: {error}");
        return ExitCode::from(66);
    }
    println!("{report}");
    ExitCode::SUCCESS
}

struct Args {
    kernel: PathBuf,
    state_root: PathBuf,
    report: PathBuf,
    revision: String,
    cycles: u64,
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut kernel = None;
        let mut state_root = None;
        let mut report = None;
        let mut revision = None;
        let mut cycles = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--kernel" => kernel = Some(PathBuf::from(value(&mut args, "--kernel")?)),
                "--state-root" => {
                    state_root = Some(PathBuf::from(value(&mut args, "--state-root")?))
                }
                "--report" => report = Some(PathBuf::from(value(&mut args, "--report")?)),
                "--revision" => revision = Some(value(&mut args, "--revision")?),
                "--cycles" => {
                    cycles = Some(
                        value(&mut args, "--cycles")?
                            .parse::<u64>()
                            .map_err(|_| "--cycles must be an integer".to_owned())?,
                    )
                }
                _ => return Err(format!("unknown lifecycle soak option: {argument}")),
            }
        }
        let kernel = kernel.ok_or_else(|| "--kernel is required".to_owned())?;
        let state_root = state_root.ok_or_else(|| "--state-root is required".to_owned())?;
        let report = report.ok_or_else(|| "--report is required".to_owned())?;
        let revision = revision.ok_or_else(|| "--revision is required".to_owned())?;
        let cycles = cycles.unwrap_or(REQUIRED_CYCLES);
        if !kernel.is_absolute() || !kernel.is_file() {
            return Err("--kernel must be an absolute existing file".to_owned());
        }
        if !state_root.is_absolute() || !report.is_absolute() {
            return Err("--state-root and --report must be absolute paths".to_owned());
        }
        if cycles != REQUIRED_CYCLES {
            return Err(format!(
                "--cycles must be exactly {REQUIRED_CYCLES} for acceptance proof"
            ));
        }
        if revision.len() != 40
            || !revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err("--revision must be a lowercase 40-character Git SHA".to_owned());
        }
        Ok(Self {
            kernel,
            state_root,
            report,
            revision,
            cycles,
        })
    }
}

fn prepare_state_root(path: &Path) -> Result<(), String> {
    if path.exists() {
        let mut entries =
            std::fs::read_dir(path).map_err(|error| format!("state root unreadable: {error}"))?;
        if entries.next().is_some() {
            return Err("state root must be empty for an exact lifecycle soak".to_owned());
        }
    } else {
        std::fs::create_dir_all(path)
            .map_err(|error| format!("state root could not be created: {error}"))?;
    }
    Ok(())
}

fn validate_cycle(outcome: &GuardianOutcome) -> Result<(), String> {
    if outcome.terminal_state != GuardianTerminalState::ExitedSuccessfully
        || outcome.attempts != 1
        || outcome.restarts != 0
        || outcome.attempts_detail.len() != 1
    {
        return Err(format!("unexpected guardian outcome: {outcome:?}"));
    }
    let attempt = &outcome.attempts_detail[0];
    if attempt.exit_code != Some(0)
        || attempt.reason_code != "child_exited_successfully"
        || attempt.pid.is_none()
    {
        return Err(format!("unexpected guardian attempt: {attempt:?}"));
    }
    let output = format!("{}\n{}", attempt.stdout, attempt.stderr).to_ascii_lowercase();
    if output.contains("degraded") || output.contains("unavailable") {
        return Err("runtime reported degraded or unavailable state".to_owned());
    }
    Ok(())
}

fn continuity_generation(path: &Path) -> Result<u64, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    value["generation"]
        .as_u64()
        .ok_or_else(|| "continuity generation is missing".to_owned())
}

fn file_sha256(path: &Path) -> std::io::Result<String> {
    let mut hash = Sha256::new();
    hash.update(std::fs::read(path)?);
    Ok(format!("{:x}", hash.finalize()))
}

fn fail(
    args: &Args,
    kernel_sha256: &str,
    started: Instant,
    failed_cycle: u64,
    error: String,
) -> ExitCode {
    let completed = failed_cycle.saturating_sub(1);
    let generation =
        continuity_generation(&args.state_root.join("runtime-continuity.json")).unwrap_or_default();
    let report = report(
        args,
        kernel_sha256,
        started,
        "fail",
        completed,
        generation,
        Some((failed_cycle, error)),
    );
    let _ = write_report(&args.report, &report);
    eprintln!("{report}");
    ExitCode::from(1)
}

fn report(
    args: &Args,
    kernel_sha256: &str,
    started: Instant,
    status: &str,
    completed_cycles: u64,
    continuity_generation: u64,
    failure: Option<(u64, String)>,
) -> serde_json::Value {
    serde_json::json!({
        "schema": REPORT_SCHEMA,
        "status": status,
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "revision": args.revision,
        "requested_cycles": args.cycles,
        "completed_cycles": completed_cycles,
        "failed_cycles": u64::from(failure.is_some()),
        "degraded_cycles": 0,
        "guardian_owned": true,
        "runtime_kernel_process_per_cycle": true,
        "continuity_generation": continuity_generation,
        "kernel_sha256": kernel_sha256,
        "duration_millis": u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "failure": failure.map(|(cycle, error)| serde_json::json!({
            "cycle": cycle,
            "error": error,
        })),
    })
}

fn write_report(path: &Path, report: &serde_json::Value) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(report)?)?;
    std::fs::rename(temporary, path)
}
