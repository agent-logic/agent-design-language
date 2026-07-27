use std::{
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Duration, Instant},
};

use adl_runtime::guardian::{run_guardian, GuardianConfig, GuardianOutcome, GuardianTerminalState};
use sha2::{Digest, Sha256};
use tokio_util::sync::CancellationToken;

const REPORT_SCHEMA: &str = "adl.runtime_v3.lifecycle_soak.v1";
const REQUIRED_CYCLES: u64 = 10_000;
const STRESS_RUNS: u64 = 100;
const STRESS_SECONDS: u64 = 10;
const ENDURANCE_RUNS: u64 = 10;
const ENDURANCE_SECONDS: u64 = 600;

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
    let kernel_sha256 = match file_sha256(&args.kernel) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("failed hashing Runtime v3 kernel: {error}");
            return ExitCode::from(66);
        }
    };

    let execution = match execute_suite(&args, started).await {
        Ok(execution) => execution,
        Err(failure) => return fail(&args, &kernel_sha256, started, failure),
    };

    let report = report(&args, &kernel_sha256, started, "pass", &execution, None);
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
    suite: Suite,
}

#[derive(Clone, Copy)]
enum Suite {
    Lifecycle { cycles: u64 },
    Timed { runs: u64, seconds: u64 },
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Lifecycle { .. } => "lifecycle_10000",
            Self::Timed {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS,
            } => "stress_100x10s",
            Self::Timed {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS,
            } => "endurance_10x600s",
            Self::Timed { .. } => unreachable!("argument parsing rejects other timed suites"),
        }
    }
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut kernel = None;
        let mut state_root = None;
        let mut report = None;
        let mut revision = None;
        let mut cycles = None;
        let mut runs = None;
        let mut duration_seconds = None;
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
                "--runs" => {
                    runs = Some(
                        value(&mut args, "--runs")?
                            .parse::<u64>()
                            .map_err(|_| "--runs must be an integer".to_owned())?,
                    )
                }
                "--duration-seconds" => {
                    duration_seconds = Some(
                        value(&mut args, "--duration-seconds")?
                            .parse::<u64>()
                            .map_err(|_| "--duration-seconds must be an integer".to_owned())?,
                    )
                }
                _ => return Err(format!("unknown lifecycle soak option: {argument}")),
            }
        }
        let kernel = kernel.ok_or_else(|| "--kernel is required".to_owned())?;
        let state_root = state_root.ok_or_else(|| "--state-root is required".to_owned())?;
        let report = report.ok_or_else(|| "--report is required".to_owned())?;
        let revision = revision.ok_or_else(|| "--revision is required".to_owned())?;
        if !kernel.is_absolute() || !kernel.is_file() {
            return Err("--kernel must be an absolute existing file".to_owned());
        }
        if !state_root.is_absolute() || !report.is_absolute() {
            return Err("--state-root and --report must be absolute paths".to_owned());
        }
        let suite = match (cycles, runs, duration_seconds) {
            (None, None, None) | (Some(REQUIRED_CYCLES), None, None) => {
                Suite::Lifecycle {
                    cycles: REQUIRED_CYCLES,
                }
            }
            (Some(_), None, None) => {
                return Err(format!(
                    "--cycles must be exactly {REQUIRED_CYCLES} for acceptance proof"
                ))
            }
            (None, Some(STRESS_RUNS), Some(STRESS_SECONDS)) => Suite::Timed {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS,
            },
            (None, Some(ENDURANCE_RUNS), Some(ENDURANCE_SECONDS)) => Suite::Timed {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS,
            },
            _ => {
                return Err(
                    "use exactly --cycles 10000, --runs 100 --duration-seconds 10, or --runs 10 --duration-seconds 600"
                        .to_owned(),
                )
            }
        };
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
            suite,
        })
    }
}

struct Execution {
    completed_runs: u64,
    completed_cycles: u64,
    continuity_generation: u64,
    minimum_cycles_per_run: u64,
}

struct Failure {
    run: u64,
    cycle: u64,
    completed_runs: u64,
    completed_cycles: u64,
    error: String,
}

async fn execute_suite(args: &Args, started: Instant) -> Result<Execution, Failure> {
    match args.suite {
        Suite::Lifecycle { cycles } => {
            let capsule = args.state_root.join("runtime-continuity.json");
            for cycle in 1..=cycles {
                execute_cycle(&args.kernel, &capsule)
                    .await
                    .map_err(|error| Failure {
                        run: 1,
                        cycle,
                        completed_runs: 0,
                        completed_cycles: cycle.saturating_sub(1),
                        error,
                    })?;
                if cycle % 1_000 == 0 {
                    eprintln!("guardian_runtime_lifecycle_progress={cycle}/{cycles}");
                }
            }
            verify_generation(&capsule, cycles).map_err(|error| Failure {
                run: 1,
                cycle: cycles,
                completed_runs: 0,
                completed_cycles: cycles,
                error,
            })?;
            Ok(Execution {
                completed_runs: 1,
                completed_cycles: cycles,
                continuity_generation: cycles,
                minimum_cycles_per_run: cycles,
            })
        }
        Suite::Timed { runs, seconds } => {
            let mut total_cycles = 0_u64;
            let mut minimum_cycles_per_run = u64::MAX;
            for run in 1..=runs {
                let run_root = args.state_root.join(format!("run-{run:03}"));
                std::fs::create_dir_all(&run_root).map_err(|error| Failure {
                    run,
                    cycle: 0,
                    completed_runs: run.saturating_sub(1),
                    completed_cycles: total_cycles,
                    error: format!("could not create lifecycle run state: {error}"),
                })?;
                let capsule = run_root.join("runtime-continuity.json");
                let deadline = Instant::now() + Duration::from_secs(seconds);
                let mut run_cycles = 0_u64;
                while run_cycles == 0 || Instant::now() < deadline {
                    run_cycles = run_cycles.saturating_add(1);
                    execute_cycle(&args.kernel, &capsule)
                        .await
                        .map_err(|error| Failure {
                            run,
                            cycle: run_cycles,
                            completed_runs: run.saturating_sub(1),
                            completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                            error,
                        })?;
                }
                verify_generation(&capsule, run_cycles).map_err(|error| Failure {
                    run,
                    cycle: run_cycles,
                    completed_runs: run.saturating_sub(1),
                    completed_cycles: total_cycles + run_cycles,
                    error,
                })?;
                total_cycles = total_cycles.saturating_add(run_cycles);
                minimum_cycles_per_run = minimum_cycles_per_run.min(run_cycles);
                eprintln!(
                    "guardian_runtime_window_progress={run}/{runs} run_cycles={run_cycles} total_cycles={total_cycles} elapsed_millis={}",
                    started.elapsed().as_millis()
                );
            }
            Ok(Execution {
                completed_runs: runs,
                completed_cycles: total_cycles,
                continuity_generation: total_cycles,
                minimum_cycles_per_run,
            })
        }
    }
}

async fn execute_cycle(kernel: &Path, capsule: &Path) -> Result<(), String> {
    let config = GuardianConfig {
        program: kernel.to_path_buf(),
        args: vec!["demo".to_owned(), capsule.to_string_lossy().into_owned()],
        env: Vec::new(),
        restart_budget: 0,
        backoff_base_ms: 1,
        backoff_cap_ms: 1,
        shutdown_grace_ms: 10_000,
        configuration_exit_codes: vec![64, 78],
    };
    let outcome = run_guardian(config, CancellationToken::new())
        .await
        .map_err(|error| format!("{error:?}"))?;
    validate_cycle(&outcome)
}

fn verify_generation(capsule: &Path, expected: u64) -> Result<(), String> {
    let generation = continuity_generation(capsule)
        .map_err(|error| format!("continuity verification failed: {error}"))?;
    if generation != expected {
        return Err(format!(
            "continuity generation {generation} did not equal completed cycles {expected}"
        ));
    }
    Ok(())
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
    if ["degraded", "unavailable", "panic", "fatal"]
        .iter()
        .any(|marker| output.contains(marker))
    {
        return Err("runtime reported degraded, unavailable, panic, or fatal state".to_owned());
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

fn fail(args: &Args, kernel_sha256: &str, started: Instant, failure: Failure) -> ExitCode {
    let execution = Execution {
        completed_runs: failure.completed_runs,
        completed_cycles: failure.completed_cycles,
        continuity_generation: 0,
        minimum_cycles_per_run: 0,
    };
    let report = report(
        args,
        kernel_sha256,
        started,
        "fail",
        &execution,
        Some(failure),
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
    execution: &Execution,
    failure: Option<Failure>,
) -> serde_json::Value {
    let (requested_cycles, requested_runs, duration_seconds) = match args.suite {
        Suite::Lifecycle { cycles } => (Some(cycles), Some(1), None),
        Suite::Timed { runs, seconds } => (None, Some(runs), Some(seconds)),
    };
    serde_json::json!({
        "schema": REPORT_SCHEMA,
        "status": status,
        "suite": args.suite.name(),
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "revision": args.revision,
        "requested_cycles": requested_cycles,
        "requested_runs": requested_runs,
        "duration_seconds_per_run": duration_seconds,
        "completed_runs": execution.completed_runs,
        "completed_cycles": execution.completed_cycles,
        "minimum_cycles_per_run": execution.minimum_cycles_per_run,
        "failed_cycles": u64::from(failure.is_some()),
        "degraded_cycles": 0,
        "guardian_owned": true,
        "runtime_kernel_process_per_cycle": true,
        "continuity_generation": execution.continuity_generation,
        "kernel_sha256": kernel_sha256,
        "duration_millis": u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "failure": failure.map(|failure| serde_json::json!({
            "run": failure.run,
            "cycle": failure.cycle,
            "error": failure.error,
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
