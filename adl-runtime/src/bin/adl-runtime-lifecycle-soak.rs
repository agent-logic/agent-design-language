use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    process::ExitCode,
    sync::Arc,
    time::{Duration, Instant},
};

use adl_runtime::guardian::{run_guardian, GuardianConfig, GuardianOutcome, GuardianTerminalState};
use ed25519_dalek::SigningKey;
use rcgen::{date_time_ymd, BasicConstraints, CertificateParams, CertifiedIssuer, IsCa, KeyPair};
use sha2::{Digest, Sha256};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_rustls::rustls::{
    pki_types::{CertificateDer, ServerName},
    ClientConfig, RootCertStore,
};
use tokio_util::sync::CancellationToken;

const REPORT_SCHEMA: &str = "adl.runtime_v3.lifecycle_soak.v1";
const CONTROL_ADDRESS: &str = "127.0.0.1:20997";
const REQUIRED_CYCLES: u64 = 10_000;
const STRESS_RUNS: u64 = 100;
const STRESS_SECONDS: u64 = 10;
const ENDURANCE_RUNS: u64 = 10;
const ENDURANCE_SECONDS: u64 = 600;
const PLATFORM_PROOF_SCHEMA: &str = "adl.wp12.platform_proof.v1";

#[tokio::main]
async fn main() -> ExitCode {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();
    if raw_args.iter().any(|arg| arg == "--aggregate-platform") {
        return match AggregateArgs::parse(raw_args.into_iter()) {
            Ok(args) => aggregate_platform(&args),
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(64)
            }
        };
    }

    let args = match Args::parse(raw_args.into_iter()) {
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
    let fixture = match ProductionFixture::create(&args.state_root) {
        Ok(fixture) => fixture,
        Err(error) => {
            eprintln!("failed preparing production Runtime v3 launch: {error}");
            return ExitCode::from(66);
        }
    };

    let mut execution = match execute_suite(&args, &fixture, started).await {
        Ok(execution) => execution,
        Err(failure) => return fail(&args, &kernel_sha256, started, failure),
    };
    execution.log_proof = match verify_master_log(&args, &fixture) {
        Ok(proof) => Some(proof),
        Err(error) => {
            return fail(
                &args,
                &kernel_sha256,
                started,
                Failure {
                    run: execution.completed_runs,
                    cycle: execution.completed_cycles,
                    completed_runs: execution.completed_runs,
                    completed_cycles: execution.completed_cycles,
                    error,
                },
            )
        }
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

struct ProductionFixture {
    address: SocketAddr,
    init: PathBuf,
    continuity_root: PathBuf,
    local_state_root: PathBuf,
    tls_connector: tokio_rustls::TlsConnector,
    control_public_key: String,
    continuity_signing_key: String,
    operation_public_key: String,
    observatory_token: String,
}

impl ProductionFixture {
    fn create(state_root: &Path) -> Result<Self, String> {
        let address = CONTROL_ADDRESS
            .parse::<SocketAddr>()
            .map_err(|error| format!("invalid control address: {error}"))?;
        let config_root = state_root.join("config");
        let continuity_root = state_root.join("continuity");
        let local_state_root = state_root.join("local-state");
        for path in [&config_root, &continuity_root, &local_state_root] {
            std::fs::create_dir_all(path)
                .map_err(|error| format!("could not create {}: {error}", path.display()))?;
        }

        let mut ca_params = CertificateParams::new(["adl-runtime-v3-wp12-ca".to_owned()])
            .map_err(|error| error.to_string())?;
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        ca_params.not_before = date_time_ymd(2026, 1, 1);
        ca_params.not_after = date_time_ymd(2036, 1, 1);
        let ca_key = KeyPair::generate().map_err(|error| error.to_string())?;
        let ca =
            CertifiedIssuer::self_signed(ca_params, ca_key).map_err(|error| error.to_string())?;
        let leaf_key = KeyPair::generate().map_err(|error| error.to_string())?;
        let mut leaf_params = CertificateParams::new([
            "localhost".to_owned(),
            "127.0.0.1".to_owned(),
            "::1".to_owned(),
        ])
        .map_err(|error| error.to_string())?;
        leaf_params.not_before = date_time_ymd(2026, 1, 1);
        leaf_params.not_after = date_time_ymd(2036, 1, 1);
        let leaf = leaf_params
            .signed_by(&leaf_key, &ca)
            .map_err(|error| error.to_string())?;
        let certificate = config_root.join("cert.pem");
        let private_key = config_root.join("key.pem");
        std::fs::write(&certificate, leaf.pem()).map_err(|error| error.to_string())?;
        std::fs::write(&private_key, leaf_key.serialize_pem())
            .map_err(|error| error.to_string())?;

        let init = config_root.join("runtime-init.toml");
        std::fs::write(
            &init,
            format!(
                r#"schema = "adl.runtime_v3.init.v1"
[api]
address = "{address}"
public_base_url = "https://localhost:{}"
[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
[observatory]
allowed_origins = ["https://localhost:20997"]
[agents]
count = 1
sample_limit = 1
"#,
                address.port(),
                toml_path(&certificate)?,
                toml_path(&private_key)?,
            ),
        )
        .map_err(|error| error.to_string())?;

        let mut roots = RootCertStore::empty();
        roots
            .add(CertificateDer::from(ca.der().to_vec()))
            .map_err(|error| error.to_string())?;
        let client_config = ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth();
        let control_key = SigningKey::from_bytes(&[17_u8; 32]);
        let operation_key = SigningKey::from_bytes(&[29_u8; 32]);

        Ok(Self {
            address,
            init,
            continuity_root,
            local_state_root: local_state_root
                .canonicalize()
                .map_err(|error| error.to_string())?,
            tls_connector: tokio_rustls::TlsConnector::from(Arc::new(client_config)),
            control_public_key: hex::encode(control_key.verifying_key().as_bytes()),
            continuity_signing_key: hex::encode([23_u8; 32]),
            operation_public_key: hex::encode(operation_key.verifying_key().as_bytes()),
            observatory_token: "wp12-observatory-token-000000000001".to_owned(),
        })
    }
}

#[derive(Clone, Copy)]
enum Suite {
    Preflight,
    Lifecycle { cycles: u64 },
    Timed { runs: u64, seconds: u64 },
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Preflight => "preflight_1x",
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
        let mut preflight = false;
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
                "--preflight" => preflight = true,
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
        let suite = match (preflight, cycles, runs, duration_seconds) {
            (true, None, None, None) => Suite::Preflight,
            (false, None, None, None) | (false, Some(REQUIRED_CYCLES), None, None) => {
                Suite::Lifecycle {
                    cycles: REQUIRED_CYCLES,
                }
            }
            (false, Some(_), None, None) => {
                return Err(format!(
                    "--cycles must be exactly {REQUIRED_CYCLES} for acceptance proof"
                ))
            }
            (false, None, Some(STRESS_RUNS), Some(STRESS_SECONDS)) => Suite::Timed {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS,
            },
            (false, None, Some(ENDURANCE_RUNS), Some(ENDURANCE_SECONDS)) => Suite::Timed {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS,
            },
            _ => {
                return Err(
                    "use exactly --preflight, --cycles 10000, --runs 100 --duration-seconds 10, or --runs 10 --duration-seconds 600"
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

struct AggregateArgs {
    preflight_report: PathBuf,
    lifecycle_report: PathBuf,
    stress_report: PathBuf,
    endurance_report: PathBuf,
    output: PathBuf,
}

impl AggregateArgs {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut aggregate = false;
        let mut preflight_report = None;
        let mut lifecycle_report = None;
        let mut stress_report = None;
        let mut endurance_report = None;
        let mut output = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--aggregate-platform" => aggregate = true,
                "--preflight-report" => {
                    preflight_report = Some(PathBuf::from(value(&mut args, "--preflight-report")?))
                }
                "--lifecycle-report" => {
                    lifecycle_report = Some(PathBuf::from(value(&mut args, "--lifecycle-report")?))
                }
                "--stress-report" => {
                    stress_report = Some(PathBuf::from(value(&mut args, "--stress-report")?))
                }
                "--endurance-report" => {
                    endurance_report = Some(PathBuf::from(value(&mut args, "--endurance-report")?))
                }
                "--output" => output = Some(PathBuf::from(value(&mut args, "--output")?)),
                _ => return Err(format!("unknown platform aggregation option: {argument}")),
            }
        }
        if !aggregate {
            return Err("--aggregate-platform is required".to_owned());
        }
        let args = Self {
            preflight_report: preflight_report
                .ok_or_else(|| "--preflight-report is required".to_owned())?,
            lifecycle_report: lifecycle_report
                .ok_or_else(|| "--lifecycle-report is required".to_owned())?,
            stress_report: stress_report.ok_or_else(|| "--stress-report is required".to_owned())?,
            endurance_report: endurance_report
                .ok_or_else(|| "--endurance-report is required".to_owned())?,
            output: output.ok_or_else(|| "--output is required".to_owned())?,
        };
        for path in [
            &args.preflight_report,
            &args.lifecycle_report,
            &args.stress_report,
            &args.endurance_report,
        ] {
            if !path.is_file() {
                return Err(format!("report does not exist: {}", path.display()));
            }
        }
        if !args.output.is_absolute() {
            return Err(
                "--output must be an absolute path for atomic platform proof writes".to_owned(),
            );
        }
        Ok(args)
    }
}

struct Execution {
    completed_runs: u64,
    completed_cycles: u64,
    continuity_generation: u64,
    minimum_cycles_per_run: u64,
    log_proof: Option<LogProof>,
}

struct LogProof {
    master_log_ref: String,
    master_log_sha256: String,
    master_log_records: u64,
    log_audit_ref: String,
    log_audit_sha256: String,
}

struct Failure {
    run: u64,
    cycle: u64,
    completed_runs: u64,
    completed_cycles: u64,
    error: String,
}

async fn execute_suite(
    args: &Args,
    fixture: &ProductionFixture,
    started: Instant,
) -> Result<Execution, Failure> {
    match args.suite {
        Suite::Preflight => {
            let continuity = fixture.continuity_root.join("preflight");
            execute_cycle(args, fixture, &continuity, 1, 1, true)
                .await
                .map_err(|error| Failure {
                    run: 1,
                    cycle: 1,
                    completed_runs: 0,
                    completed_cycles: 0,
                    error,
                })?;
            Ok(Execution {
                completed_runs: 1,
                completed_cycles: 1,
                continuity_generation: 1,
                minimum_cycles_per_run: 1,
                log_proof: None,
            })
        }
        Suite::Lifecycle { cycles } => {
            let continuity = fixture.continuity_root.join("lifecycle");
            for cycle in 1..=cycles {
                execute_cycle(args, fixture, &continuity, 1, cycle, cycle == cycles)
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
            verify_generation(&continuity, cycles).map_err(|error| Failure {
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
                log_proof: None,
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
                let continuity = fixture.continuity_root.join(format!("run-{run:03}"));
                let deadline = Instant::now() + Duration::from_secs(seconds);
                let mut run_cycles = 0_u64;
                while run_cycles == 0 || Instant::now() < deadline {
                    run_cycles = run_cycles.saturating_add(1);
                    execute_cycle(args, fixture, &continuity, run, run_cycles, false)
                        .await
                        .map_err(|error| Failure {
                            run,
                            cycle: run_cycles,
                            completed_runs: run.saturating_sub(1),
                            completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                            error,
                        })?;
                }
                run_cycles = run_cycles.saturating_add(1);
                execute_cycle(args, fixture, &continuity, run, run_cycles, true)
                    .await
                    .map_err(|error| Failure {
                        run,
                        cycle: run_cycles,
                        completed_runs: run.saturating_sub(1),
                        completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                        error,
                    })?;
                verify_generation(&continuity, run_cycles).map_err(|error| Failure {
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
                log_proof: None,
            })
        }
    }
}

async fn execute_cycle(
    args: &Args,
    fixture: &ProductionFixture,
    continuity_root: &Path,
    run: u64,
    cycle: u64,
    retain_log: bool,
) -> Result<(), String> {
    std::fs::create_dir_all(continuity_root)
        .map_err(|error| format!("could not create continuity root: {error}"))?;
    let config = GuardianConfig {
        program: args.kernel.clone(),
        args: vec![
            "serve".to_owned(),
            "--init".to_owned(),
            fixture.init.to_string_lossy().into_owned(),
            "--continuity-root".to_owned(),
            continuity_root.to_string_lossy().into_owned(),
        ],
        env: vec![
            (
                "ADL_RUNTIME_V3_LOCAL_STATE_DIR".to_owned(),
                fixture.local_state_root.to_string_lossy().into_owned(),
            ),
            (
                "ADL_RUNTIME_CONTROL_PUBLIC_KEY_HEX".to_owned(),
                fixture.control_public_key.clone(),
            ),
            (
                "ADL_RUNTIME_CONTROL_KEY_ID".to_owned(),
                "wp12-control".to_owned(),
            ),
            (
                "ADL_RUNTIME_CONTROL_PRINCIPAL".to_owned(),
                "wp12-control".to_owned(),
            ),
            (
                "ADL_RUNTIME_CONTINUITY_SIGNING_KEY_HEX".to_owned(),
                fixture.continuity_signing_key.clone(),
            ),
            (
                "ADL_RUNTIME_CONTINUITY_KEY_ID".to_owned(),
                "wp12-continuity".to_owned(),
            ),
            (
                "ADL_RUNTIME_CONTINUITY_MIN_GENERATION".to_owned(),
                cycle.saturating_sub(1).to_string(),
            ),
            (
                "ADL_RUNTIME_OPERATION_PUBLIC_KEY_HEX".to_owned(),
                fixture.operation_public_key.clone(),
            ),
            (
                "ADL_RUNTIME_OPERATION_KEY_ID".to_owned(),
                "wp12-operations".to_owned(),
            ),
            (
                "ADL_RUNTIME_OBSERVATORY_TOKEN".to_owned(),
                fixture.observatory_token.clone(),
            ),
            (
                "ADL_RUNTIME_LIFECYCLE_SUITE".to_owned(),
                args.suite.name().to_owned(),
            ),
            ("ADL_RUNTIME_LIFECYCLE_RUN".to_owned(), run.to_string()),
            ("ADL_RUNTIME_LIFECYCLE_CYCLE".to_owned(), cycle.to_string()),
            ("ADL_RUNTIME_REVISION".to_owned(), args.revision.clone()),
        ],
        restart_budget: 0,
        backoff_base_ms: 1,
        backoff_cap_ms: 1,
        shutdown_grace_ms: 45_000,
        configuration_exit_codes: vec![64, 78],
    };
    let shutdown = CancellationToken::new();
    let guardian_shutdown = shutdown.clone();
    let guardian = tokio::spawn(async move { run_guardian(config, guardian_shutdown).await });
    let ready = wait_for_authenticated_observatory(fixture, &guardian).await;
    shutdown.cancel();
    let outcome = tokio::time::timeout(Duration::from_secs(50), guardian)
        .await
        .map_err(|_| "Guardian did not complete production shutdown".to_owned())?
        .map_err(|error| format!("Guardian task failed: {error}"))?
        .map_err(|error| format!("{error:?}"))?;
    validate_guardian_outcome(&outcome)?;
    verify_generation(continuity_root, cycle).map_err(|error| {
        format!(
            "{error}; guardian_stderr={}",
            diagnostic_tail(&outcome.attempts_detail[0].stderr, &args.state_root)
        )
    })?;
    verify_writer_lock_released(&fixture.local_state_root)?;
    ready?;
    validate_guardian_output(&outcome)?;
    verify_master_log(args, fixture).map_err(|error| {
        format!(
            "{error}; guardian_stderr={}",
            diagnostic_tail(&outcome.attempts_detail[0].stderr, &args.state_root)
        )
    })?;
    if !retain_log {
        std::fs::remove_dir_all(fixture.local_state_root.join("observability"))
            .map_err(|error| format!("checked Vector log could not be discarded: {error}"))?;
    }
    Ok(())
}

fn diagnostic_tail(output: &str, state_root: &Path) -> String {
    let redacted = output.replace(&state_root.to_string_lossy().to_string(), "<state-root>");
    let tail = redacted
        .char_indices()
        .rev()
        .nth(4_095)
        .map_or(redacted.as_str(), |(index, _)| &redacted[index..]);
    tail.replace(['\n', '\r'], " | ")
}

async fn wait_for_authenticated_observatory(
    fixture: &ProductionFixture,
    guardian: &tokio::task::JoinHandle<
        Result<GuardianOutcome, adl_runtime::guardian::GuardianConfigError>,
    >,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        if guardian.is_finished() {
            return Err("Runtime v3 exited before its authenticated API became ready".to_owned());
        }
        match authenticated_observatory(fixture).await {
            Ok(observatory) => return validate_observatory(&observatory),
            Err(error) if Instant::now() < deadline => {
                let _ = error;
                tokio::time::sleep(Duration::from_millis(10)).await;
            }
            Err(error) => {
                return Err(format!(
                    "Runtime v3 authenticated API was not ready on {CONTROL_ADDRESS}: {error}"
                ))
            }
        }
    }
}

async fn authenticated_observatory(
    fixture: &ProductionFixture,
) -> Result<serde_json::Value, String> {
    let stream = tokio::net::TcpStream::connect(fixture.address)
        .await
        .map_err(|error| error.to_string())?;
    let server_name = ServerName::try_from("localhost").map_err(|error| error.to_string())?;
    let mut stream = fixture
        .tls_connector
        .connect(server_name, stream)
        .await
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /v1/observatory HTTP/1.1\r\nHost: localhost\r\nAuthorization: Bearer {}\r\nConnection: close\r\n\r\n",
        fixture.observatory_token
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|error| error.to_string())?;
    let response = String::from_utf8(response).map_err(|error| error.to_string())?;
    if !response.starts_with("HTTP/1.1 200 OK") {
        return Err("authenticated Observatory request did not return HTTP 200".to_owned());
    }
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .ok_or_else(|| "authenticated Observatory response had no body".to_owned())?;
    serde_json::from_str(body).map_err(|error| error.to_string())
}

fn validate_observatory(observatory: &serde_json::Value) -> Result<(), String> {
    if observatory["schema"] != "adl.runtime_v3.observatory_feed.v2"
        || observatory["runtime_selection"] != "runtime_v3_explicit_opt_in"
        || observatory["control"]["websocket_full_duplex"] != true
    {
        return Err(
            "Runtime v3 Observatory did not expose the production control contract".to_owned(),
        );
    }
    reject_non_operational_state(observatory, "$")
}

fn reject_non_operational_state(value: &serde_json::Value, path: &str) -> Result<(), String> {
    match value {
        serde_json::Value::String(value) => {
            let normalized = value.to_ascii_lowercase();
            if ["degraded", "unavailable", "failed", "fatal", "panic"]
                .iter()
                .any(|marker| {
                    normalized == *marker || normalized.starts_with(&format!("{marker}:"))
                })
            {
                return Err(format!(
                    "Runtime v3 Observatory reported non-operational state at {path}: {value}"
                ));
            }
        }
        serde_json::Value::Array(values) => {
            for (index, value) in values.iter().enumerate() {
                reject_non_operational_state(value, &format!("{path}[{index}]"))?;
            }
        }
        serde_json::Value::Object(values) => {
            for (key, value) in values {
                reject_non_operational_state(value, &format!("{path}.{key}"))?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn verify_writer_lock_released(local_state_root: &Path) -> Result<(), String> {
    let writer_lock = local_state_root.join("writer.lock");
    if writer_lock.exists() {
        return Err(format!(
            "production adapter writer lock survived clean shutdown: {}",
            writer_lock.display()
        ));
    }
    Ok(())
}

fn verify_master_log(args: &Args, fixture: &ProductionFixture) -> Result<LogProof, String> {
    let observability_root = fixture.local_state_root.join("observability");
    let master_log = observability_root.join("durable/master.log.jsonl");
    let audit = observability_root.join("audit/master-log-audit.json");
    let master_log_sha256 =
        file_sha256(&master_log).map_err(|error| format!("master log unavailable: {error}"))?;
    let master_log_bytes =
        std::fs::read(&master_log).map_err(|error| format!("master log unreadable: {error}"))?;
    let master_log_records = u64::try_from(
        String::from_utf8(master_log_bytes)
            .map_err(|_| "master log is not UTF-8 JSONL".to_owned())?
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count(),
    )
    .map_err(|_| "master log record count overflowed".to_owned())?;
    let audit_bytes =
        std::fs::read(&audit).map_err(|error| format!("master log audit unavailable: {error}"))?;
    let audit_value: serde_json::Value = serde_json::from_slice(&audit_bytes)
        .map_err(|error| format!("master log audit is invalid JSON: {error}"))?;
    let expected_platform = std::env::consts::OS;
    let expected_suite = args.suite.name();
    let zero_counters = [
        "malformed_records",
        "missing_required_fields",
        "sequence_gaps",
        "error_events",
        "degraded_events",
        "unexplained_restarts",
        "incomplete_drains",
    ]
    .iter()
    .all(|field| audit_value[*field].as_u64() == Some(0));
    if audit_value["schema"] != "adl.runtime.master_log_audit.v1"
        || audit_value["status"] != "pass"
        || audit_value["platform"] != expected_platform
        || audit_value["suite"] != expected_suite
        || audit_value["revision"] != args.revision
        || audit_value["master_log_sha256"] != master_log_sha256
        || audit_value["record_count"].as_u64() != Some(master_log_records)
        || !zero_counters
    {
        return Err(format!(
            "Vector master log audit did not prove a clean {expected_platform}/{expected_suite} lifecycle"
        ));
    }
    Ok(LogProof {
        master_log_ref: repo_relative(&master_log)?,
        master_log_sha256,
        master_log_records,
        log_audit_ref: repo_relative(&audit)?,
        log_audit_sha256: file_sha256(&audit)
            .map_err(|error| format!("master log audit hash failed: {error}"))?,
    })
}

fn aggregate_platform(args: &AggregateArgs) -> ExitCode {
    match build_platform_proof(args) {
        Ok(proof) => {
            if let Err(error) = write_report(&args.output, &proof) {
                eprintln!("failed writing platform proof: {error}");
                return ExitCode::from(66);
            }
            println!("{proof}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

struct SoakReport {
    value: serde_json::Value,
    revision: String,
    kernel_sha256: String,
    platform: String,
    architecture: String,
}

fn build_platform_proof(args: &AggregateArgs) -> Result<serde_json::Value, String> {
    let preflight = read_soak_report(&args.preflight_report, "preflight_1x")?;
    let lifecycle = read_soak_report(&args.lifecycle_report, "lifecycle_10000")?;
    let stress = read_soak_report(&args.stress_report, "stress_100x10s")?;
    let endurance = read_soak_report(&args.endurance_report, "endurance_10x600s")?;
    let reports = [&preflight, &lifecycle, &stress, &endurance];
    let first = reports[0];
    for report in reports {
        if report.revision != first.revision {
            return Err("platform reports do not share one exact Git revision".to_owned());
        }
        if report.kernel_sha256 != first.kernel_sha256 {
            return Err("platform reports do not share one Runtime v3 kernel digest".to_owned());
        }
        if report.platform != first.platform || report.architecture != first.architecture {
            return Err(
                "platform reports mix native platform or architecture identities".to_owned(),
            );
        }
    }
    let platform_id = platform_proof_id(&first.platform, &first.architecture)?;
    Ok(serde_json::json!({
        "schema": PLATFORM_PROOF_SCHEMA,
        "issue": 5344,
        "platform": platform_id,
        "native_os": first.platform,
        "architecture": first.architecture,
        "status": "pass",
        "guardian_process_zero": true,
        "native_execution": true,
        "wsl_used": false,
        "docker_used": false,
        "lifecycle_acceptance": {
            "revision": first.revision,
            "kernel_sha256": first.kernel_sha256,
            "all_logs_clean": true,
            "preflight": suite_summary(&preflight.value),
            "lifecycle_10000": suite_summary(&lifecycle.value),
            "stress_100x10s": suite_summary(&stress.value),
            "endurance_10x600s": suite_summary(&endurance.value),
        },
    }))
}

fn read_soak_report(path: &Path, expected_suite: &str) -> Result<SoakReport, String> {
    let value: serde_json::Value = serde_json::from_slice(
        &std::fs::read(path).map_err(|error| format!("{} unreadable: {error}", path.display()))?,
    )
    .map_err(|error| format!("{} is invalid JSON: {error}", path.display()))?;
    require_string(&value, "schema", REPORT_SCHEMA)?;
    require_string(&value, "status", "pass")?;
    require_string(&value, "suite", expected_suite)?;
    let platform = string_field(&value, "platform")?;
    let architecture = string_field(&value, "architecture")?;
    let native_platform = std::env::consts::OS;
    let native_architecture = std::env::consts::ARCH;
    if platform != native_platform || architecture != native_architecture {
        return Err(format!(
            "{expected_suite} was collected for {platform}/{architecture}, not native {native_platform}/{native_architecture}"
        ));
    }
    let revision = string_field(&value, "revision")?;
    if !is_lower_hex(&revision, 40) {
        return Err(format!("{expected_suite} has invalid revision identity"));
    }
    let kernel_sha256 = string_field(&value, "kernel_sha256")?;
    if !is_lower_hex(&kernel_sha256, 64) {
        return Err(format!("{expected_suite} has invalid kernel digest"));
    }
    if u64_field(&value, "failed_cycles")? != 0 || u64_field(&value, "degraded_cycles")? != 0 {
        return Err(format!("{expected_suite} reported failure or degradation"));
    }
    require_bool(&value, "guardian_owned", true)?;
    require_bool(&value, "runtime_kernel_process_per_cycle", true)?;
    require_bool(&value, "logging_complete", true)?;
    require_string(&value, "master_log_status", "clean")?;
    if u64_field(&value, "master_log_records")? == 0 {
        return Err(format!("{expected_suite} retained no master log records"));
    }
    validate_suite_counts(&value, expected_suite)?;
    validate_log_proof(&value, expected_suite)?;
    Ok(SoakReport {
        value,
        revision,
        kernel_sha256,
        platform,
        architecture,
    })
}

fn validate_suite_counts(value: &serde_json::Value, suite: &str) -> Result<(), String> {
    match suite {
        "preflight_1x" => {
            require_bool(value, "acceptance_eligible", false)?;
            require_u64(value, "requested_cycles", 1)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", 1)?;
        }
        "lifecycle_10000" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_cycles", REQUIRED_CYCLES)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", REQUIRED_CYCLES)?;
        }
        "stress_100x10s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", STRESS_RUNS)?;
            require_u64(value, "duration_seconds_per_run", STRESS_SECONDS)?;
            require_u64(value, "completed_runs", STRESS_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        "endurance_10x600s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", ENDURANCE_RUNS)?;
            require_u64(value, "duration_seconds_per_run", ENDURANCE_SECONDS)?;
            require_u64(value, "completed_runs", ENDURANCE_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        _ => return Err(format!("unsupported suite identity: {suite}")),
    }
    Ok(())
}

fn validate_log_proof(value: &serde_json::Value, suite: &str) -> Result<(), String> {
    let log_records = u64_field(value, "master_log_records")?;
    let master_log = safe_repo_path(&string_field(value, "master_log_ref")?)?;
    let master_log_sha256 = string_field(value, "master_log_sha256")?;
    if !is_lower_hex(&master_log_sha256, 64)
        || file_sha256(&master_log).map_err(|e| e.to_string())? != master_log_sha256
    {
        return Err(format!(
            "{suite} master log digest did not match retained file"
        ));
    }
    let master_log_text = std::fs::read_to_string(&master_log)
        .map_err(|error| format!("{suite} master log unreadable: {error}"))?;
    let counted_records = u64::try_from(
        master_log_text
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count(),
    )
    .map_err(|_| format!("{suite} master log count overflowed"))?;
    if counted_records != log_records {
        return Err(format!(
            "{suite} master log record count does not match report"
        ));
    }
    for line in master_log_text
        .lines()
        .filter(|line| !line.trim().is_empty())
    {
        serde_json::from_str::<serde_json::Value>(line)
            .map_err(|error| format!("{suite} master log contains invalid JSONL: {error}"))?;
    }

    let audit = safe_repo_path(&string_field(value, "log_audit_ref")?)?;
    let audit_sha256 = string_field(value, "log_audit_sha256")?;
    if !is_lower_hex(&audit_sha256, 64)
        || file_sha256(&audit).map_err(|e| e.to_string())? != audit_sha256
    {
        return Err(format!("{suite} audit digest did not match retained file"));
    }
    let audit_value: serde_json::Value =
        serde_json::from_slice(&std::fs::read(&audit).map_err(|error| error.to_string())?)
            .map_err(|error| format!("{suite} audit is invalid JSON: {error}"))?;
    let zero_counters = [
        "malformed_records",
        "missing_required_fields",
        "sequence_gaps",
        "error_events",
        "degraded_events",
        "unexplained_restarts",
        "incomplete_drains",
    ]
    .iter()
    .all(|field| audit_value[*field].as_u64() == Some(0));
    if audit_value["schema"] != "adl.runtime.master_log_audit.v1"
        || audit_value["status"] != "pass"
        || audit_value["platform"] != value["platform"]
        || audit_value["suite"] != suite
        || audit_value["revision"] != value["revision"]
        || audit_value["master_log_sha256"] != master_log_sha256
        || audit_value["record_count"].as_u64() != Some(log_records)
        || !zero_counters
    {
        return Err(format!("{suite} audit did not prove clean compact logging"));
    }
    Ok(())
}

fn suite_summary(value: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": value["status"],
        "suite": value["suite"],
        "requested_cycles": value["requested_cycles"],
        "requested_runs": value["requested_runs"],
        "duration_seconds_per_run": value["duration_seconds_per_run"],
        "completed_runs": value["completed_runs"],
        "completed_cycles": value["completed_cycles"],
        "minimum_cycles_per_run": value["minimum_cycles_per_run"],
        "failed_cycles": value["failed_cycles"],
        "degraded_cycles": value["degraded_cycles"],
        "guardian_owned": value["guardian_owned"],
        "runtime_kernel_process_per_cycle": value["runtime_kernel_process_per_cycle"],
        "acceptance_eligible": value["acceptance_eligible"],
        "logging_complete": value["logging_complete"],
        "log_checked_cycles": value["log_checked_cycles"],
        "master_log_status": value["master_log_status"],
        "master_log_ref": value["master_log_ref"],
        "master_log_sha256": value["master_log_sha256"],
        "master_log_records": value["master_log_records"],
        "log_audit_ref": value["log_audit_ref"],
        "log_audit_sha256": value["log_audit_sha256"],
    })
}

fn platform_proof_id(platform: &str, architecture: &str) -> Result<&'static str, String> {
    match (platform, architecture) {
        ("macos", "aarch64") => Ok("macos-arm64"),
        ("linux", "x86_64") => Ok("linux-x86_64"),
        ("windows", "x86_64") => Ok("windows-x86_64-msvc"),
        _ => Err(format!(
            "unsupported native WP-12 platform identity: {platform}/{architecture}"
        )),
    }
}

fn safe_repo_path(path: &str) -> Result<PathBuf, String> {
    let candidate = Path::new(path);
    if candidate.is_absolute()
        || candidate
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(format!("evidence path is not repo-relative: {path}"));
    }
    let root = std::env::current_dir()
        .map_err(|error| format!("current checkout unavailable: {error}"))?
        .canonicalize()
        .map_err(|error| format!("current checkout cannot be canonicalized: {error}"))?;
    let resolved = root.join(candidate);
    let canonical = resolved
        .canonicalize()
        .map_err(|error| format!("retained evidence path unavailable: {path}: {error}"))?;
    canonical
        .strip_prefix(&root)
        .map_err(|_| format!("evidence path escaped repository checkout: {path}"))?;
    Ok(canonical)
}

fn require_string(value: &serde_json::Value, field: &str, expected: &str) -> Result<(), String> {
    let actual = string_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_bool(value: &serde_json::Value, field: &str, expected: bool) -> Result<(), String> {
    let actual = value[field]
        .as_bool()
        .ok_or_else(|| format!("{field} must be boolean"))?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_u64(value: &serde_json::Value, field: &str, expected: u64) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_positive(value: &serde_json::Value, field: &str) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual == 0 {
        return Err(format!("{field} must be greater than zero"));
    }
    Ok(())
}

fn string_field(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value[field]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("{field} must be string"))
}

fn u64_field(value: &serde_json::Value, field: &str) -> Result<u64, String> {
    value[field]
        .as_u64()
        .ok_or_else(|| format!("{field} must be unsigned integer"))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn repo_relative(path: &Path) -> Result<String, String> {
    let root = std::env::current_dir()
        .map_err(|error| format!("current checkout unavailable: {error}"))?
        .canonicalize()
        .map_err(|error| format!("current checkout cannot be canonicalized: {error}"))?;
    let path = path
        .canonicalize()
        .map_err(|error| format!("evidence path cannot be canonicalized: {error}"))?;
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| "lifecycle evidence escaped the repository checkout".to_owned())?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn verify_generation(continuity_root: &Path, expected: u64) -> Result<(), String> {
    let manifest = continuity_root
        .join(format!("generation-{expected}"))
        .join("manifest.json");
    let generation = continuity_generation(&manifest)
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

fn validate_guardian_outcome(outcome: &GuardianOutcome) -> Result<(), String> {
    if outcome.terminal_state != GuardianTerminalState::ShutdownForwarded
        || outcome.attempts != 1
        || outcome.restarts != 0
        || outcome.attempts_detail.len() != 1
    {
        return Err(format!("unexpected guardian outcome: {outcome:?}"));
    }
    let attempt = &outcome.attempts_detail[0];
    if attempt.reason_code != "shutdown_signal_forwarded" || attempt.pid.is_none() {
        return Err(format!("unexpected guardian attempt: {attempt:?}"));
    }
    Ok(())
}

fn validate_guardian_output(outcome: &GuardianOutcome) -> Result<(), String> {
    let attempt = &outcome.attempts_detail[0];
    let output = format!("{}\n{}", attempt.stdout, attempt.stderr).to_ascii_lowercase();
    if ["degraded", "unavailable", "panic", "fatal"]
        .iter()
        .any(|marker| output.contains(marker))
    {
        return Err("runtime reported degraded, unavailable, panic, or fatal state".to_owned());
    }
    Ok(())
}

fn toml_path(path: &Path) -> Result<String, String> {
    let value = path
        .to_str()
        .ok_or_else(|| "runtime configuration path is not UTF-8".to_owned())?;
    if value.contains(['\n', '\r']) {
        return Err("runtime configuration path contains a line break".to_owned());
    }
    Ok(value.replace('\\', "\\\\").replace('"', "\\\""))
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
        log_proof: None,
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
        Suite::Preflight => (Some(1), Some(1), None),
        Suite::Lifecycle { cycles } => (Some(cycles), Some(1), None),
        Suite::Timed { runs, seconds } => (None, Some(runs), Some(seconds)),
    };
    let logging_complete = execution.log_proof.is_some();
    let master_log_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_ref.as_str());
    let master_log_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_sha256.as_str());
    let master_log_records = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_records);
    let log_audit_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_ref.as_str());
    let log_audit_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_sha256.as_str());
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
        "acceptance_eligible": !matches!(args.suite, Suite::Preflight),
        "logging_complete": logging_complete,
        "log_checked_cycles": if logging_complete {
            Some(execution.completed_cycles)
        } else {
            None
        },
        "master_log_status": if logging_complete { "clean" } else { "incomplete" },
        "master_log_ref": master_log_ref,
        "master_log_sha256": master_log_sha256,
        "master_log_records": master_log_records,
        "log_audit_ref": log_audit_ref,
        "log_audit_sha256": log_audit_sha256,
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

#[cfg(test)]
mod tests {
    use super::*;

    fn arguments(mode: &[&str]) -> Vec<String> {
        let root = std::env::current_dir().expect("current directory");
        let mut values = vec![
            "--kernel".to_owned(),
            std::env::current_exe()
                .expect("current executable")
                .to_string_lossy()
                .into_owned(),
            "--state-root".to_owned(),
            root.join("state").to_string_lossy().into_owned(),
            "--report".to_owned(),
            root.join("report.json").to_string_lossy().into_owned(),
            "--revision".to_owned(),
            "0123456789abcdef0123456789abcdef01234567".to_owned(),
        ];
        values.extend(mode.iter().map(|value| (*value).to_owned()));
        values
    }

    #[test]
    fn accepts_only_the_three_exact_acceptance_suites() {
        let lifecycle = Args::parse(arguments(&["--cycles", "10000"]).into_iter())
            .expect("10k lifecycle suite");
        assert!(matches!(
            lifecycle.suite,
            Suite::Lifecycle {
                cycles: REQUIRED_CYCLES
            }
        ));

        let stress =
            Args::parse(arguments(&["--runs", "100", "--duration-seconds", "10"]).into_iter())
                .expect("100x10s stress suite");
        assert!(matches!(
            stress.suite,
            Suite::Timed {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS
            }
        ));

        let endurance =
            Args::parse(arguments(&["--runs", "10", "--duration-seconds", "600"]).into_iter())
                .expect("10x600s endurance suite");
        assert!(matches!(
            endurance.suite,
            Suite::Timed {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS
            }
        ));
    }

    #[test]
    fn preflight_is_real_but_never_acceptance_eligible() {
        let preflight =
            Args::parse(arguments(&["--preflight"]).into_iter()).expect("one-cycle preflight");
        assert!(matches!(preflight.suite, Suite::Preflight));
        let execution = Execution {
            completed_runs: 1,
            completed_cycles: 1,
            continuity_generation: 1,
            minimum_cycles_per_run: 1,
            log_proof: Some(LogProof {
                master_log_ref: ".csdlc/evidence/5344/work/master.jsonl".to_owned(),
                master_log_sha256: "b".repeat(64),
                master_log_records: 2,
                log_audit_ref: ".csdlc/evidence/5344/work/audit.json".to_owned(),
                log_audit_sha256: "c".repeat(64),
            }),
        };
        let value = report(
            &preflight,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );
        assert_eq!(value["acceptance_eligible"], false);
        assert_eq!(value["logging_complete"], true);
        assert_eq!(value["master_log_status"], "clean");
        assert_eq!(value["master_log_records"], 2);
    }

    #[test]
    fn rejects_partial_or_mixed_acceptance_suites() {
        for mode in [
            vec!["--cycles", "9999"],
            vec!["--runs", "100", "--duration-seconds", "9"],
            vec!["--runs", "9", "--duration-seconds", "600"],
            vec![
                "--cycles",
                "10000",
                "--runs",
                "100",
                "--duration-seconds",
                "10",
            ],
            vec!["--preflight", "--cycles", "10000"],
        ] {
            assert!(
                Args::parse(arguments(&mode).into_iter()).is_err(),
                "unexpectedly accepted {mode:?}"
            );
        }
    }

    #[test]
    fn aggregates_four_exact_lifecycle_reports_with_compact_clean_logs() {
        let root = std::env::current_dir().expect("current directory");
        let temp = tempfile::tempdir_in(&root).expect("repo-local temp evidence");
        let revision = "0123456789abcdef0123456789abcdef01234567";
        let kernel_sha256 = &"a".repeat(64);
        let preflight = write_sample_report(
            &root,
            temp.path(),
            "preflight_1x",
            revision,
            kernel_sha256,
            1,
        );
        let lifecycle = write_sample_report(
            &root,
            temp.path(),
            "lifecycle_10000",
            revision,
            kernel_sha256,
            REQUIRED_CYCLES,
        );
        let stress = write_sample_report(
            &root,
            temp.path(),
            "stress_100x10s",
            revision,
            kernel_sha256,
            42,
        );
        let endurance = write_sample_report(
            &root,
            temp.path(),
            "endurance_10x600s",
            revision,
            kernel_sha256,
            24,
        );
        let output = temp.path().join("platform-proof.json");
        let args = AggregateArgs {
            preflight_report: preflight,
            lifecycle_report: lifecycle,
            stress_report: stress,
            endurance_report: endurance,
            output: output.clone(),
        };

        let proof = build_platform_proof(&args).expect("platform proof");
        write_report(&output, &proof).expect("atomic proof write");
        let written: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output).expect("written proof"))
                .expect("proof JSON");

        assert_eq!(written["schema"], PLATFORM_PROOF_SCHEMA);
        assert_eq!(written["status"], "pass");
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["completed_cycles"],
            REQUIRED_CYCLES
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["master_log_records"],
            3
        );
        assert_eq!(
            written["lifecycle_acceptance"]["stress_100x10s"]["master_log_records"],
            3
        );
    }

    #[test]
    fn rejects_compact_log_reports_when_audit_count_drifts() {
        let root = std::env::current_dir().expect("current directory");
        let temp = tempfile::tempdir_in(&root).expect("repo-local temp evidence");
        let revision = "0123456789abcdef0123456789abcdef01234567";
        let kernel_sha256 = &"a".repeat(64);
        let preflight = write_sample_report(
            &root,
            temp.path(),
            "preflight_1x",
            revision,
            kernel_sha256,
            1,
        );
        let lifecycle = write_sample_report(
            &root,
            temp.path(),
            "lifecycle_10000",
            revision,
            kernel_sha256,
            REQUIRED_CYCLES,
        );
        let stress = write_sample_report(
            &root,
            temp.path(),
            "stress_100x10s",
            revision,
            kernel_sha256,
            42,
        );
        let endurance = write_sample_report(
            &root,
            temp.path(),
            "endurance_10x600s",
            revision,
            kernel_sha256,
            24,
        );
        let lifecycle_value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&lifecycle).expect("lifecycle report"))
                .expect("JSON report");
        let audit = root.join(
            lifecycle_value["log_audit_ref"]
                .as_str()
                .expect("audit ref"),
        );
        let mut audit_value: serde_json::Value =
            serde_json::from_slice(&std::fs::read(&audit).expect("audit")).expect("audit JSON");
        audit_value["record_count"] = serde_json::json!(4);
        std::fs::write(
            &audit,
            serde_json::to_vec_pretty(&audit_value).expect("audit bytes"),
        )
        .expect("write drifted audit");
        let drifted_audit_sha256 = file_sha256(&audit).expect("audit sha");
        let mut lifecycle_value = lifecycle_value;
        lifecycle_value["log_audit_sha256"] = serde_json::json!(drifted_audit_sha256);
        std::fs::write(
            &lifecycle,
            serde_json::to_vec_pretty(&lifecycle_value).expect("report bytes"),
        )
        .expect("write drifted report");
        let args = AggregateArgs {
            preflight_report: preflight,
            lifecycle_report: lifecycle,
            stress_report: stress,
            endurance_report: endurance,
            output: temp.path().join("platform-proof.json"),
        };

        let error = build_platform_proof(&args).expect_err("drifted audit rejected");
        assert!(error.contains("audit did not prove clean compact logging"));
    }

    fn write_sample_report(
        root: &Path,
        temp: &Path,
        suite: &str,
        revision: &str,
        kernel_sha256: &str,
        completed_cycles: u64,
    ) -> PathBuf {
        let suite_dir = temp.join(suite);
        std::fs::create_dir_all(&suite_dir).expect("suite dir");
        let log = suite_dir.join("master.log.jsonl");
        std::fs::write(
            &log,
            b"{\"sequence\":1,\"level\":\"info\"}\n{\"sequence\":2,\"level\":\"info\"}\n{\"sequence\":3,\"level\":\"info\"}\n",
        )
        .expect("master log");
        let log_sha256 = file_sha256(&log).expect("log sha");
        let audit = suite_dir.join("master-log-audit.json");
        let audit_value = serde_json::json!({
            "schema": "adl.runtime.master_log_audit.v1",
            "status": "pass",
            "platform": std::env::consts::OS,
            "suite": suite,
            "revision": revision,
            "master_log_sha256": log_sha256,
            "record_count": 3,
            "malformed_records": 0,
            "missing_required_fields": 0,
            "sequence_gaps": 0,
            "error_events": 0,
            "degraded_events": 0,
            "unexplained_restarts": 0,
            "incomplete_drains": 0,
        });
        std::fs::write(
            &audit,
            serde_json::to_vec_pretty(&audit_value).expect("audit bytes"),
        )
        .expect("audit");
        let audit_sha256 = file_sha256(&audit).expect("audit sha");
        let (requested_runs, requested_cycles, duration_seconds) = match suite {
            "preflight_1x" => (1, Some(1), None),
            "lifecycle_10000" => (1, Some(REQUIRED_CYCLES), None),
            "stress_100x10s" => (STRESS_RUNS, None, Some(STRESS_SECONDS)),
            "endurance_10x600s" => (ENDURANCE_RUNS, None, Some(ENDURANCE_SECONDS)),
            _ => panic!("unsupported sample suite"),
        };
        let report = serde_json::json!({
            "schema": REPORT_SCHEMA,
            "status": "pass",
            "suite": suite,
            "platform": std::env::consts::OS,
            "architecture": std::env::consts::ARCH,
            "revision": revision,
            "requested_cycles": requested_cycles,
            "requested_runs": requested_runs,
            "duration_seconds_per_run": duration_seconds,
            "completed_runs": requested_runs,
            "completed_cycles": completed_cycles,
            "minimum_cycles_per_run": completed_cycles.max(1),
            "failed_cycles": 0,
            "degraded_cycles": 0,
            "guardian_owned": true,
            "runtime_kernel_process_per_cycle": true,
            "acceptance_eligible": suite != "preflight_1x",
            "logging_complete": true,
            "log_checked_cycles": completed_cycles,
            "master_log_status": "clean",
            "master_log_ref": rel(root, &log),
            "master_log_sha256": log_sha256,
            "master_log_records": 3,
            "log_audit_ref": rel(root, &audit),
            "log_audit_sha256": audit_sha256,
            "continuity_generation": completed_cycles,
            "kernel_sha256": kernel_sha256,
            "duration_millis": 1,
            "failure": null,
        });
        let report_path = suite_dir.join("report.json");
        std::fs::write(
            &report_path,
            serde_json::to_vec_pretty(&report).expect("report bytes"),
        )
        .expect("report");
        report_path
    }

    fn rel(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .expect("repo-relative test path")
            .to_string_lossy()
            .replace('\\', "/")
    }
}
