use anyhow::{Context, Result};
use std::path::PathBuf;

use super::agent_cmd::real_csm_daemon;
use super::csm_service_cmd::real_service;
use ::adl::csm_continuity_capsule::{
    capture_capsule, restore_capsule, stage_capsule, ContinuityCaptureOptions,
    ContinuityRestoreOptions, ContinuityStageOptions,
};
use ::adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};
use ::adl::csm_runtime_api::{serve_runtime_api, CsmRuntimeApiOptions};

pub(crate) enum CsmDispatchMode {
    StandaloneRuntime,
    AdlControlPlane,
}

pub(crate) fn real_csm(args: &[String]) -> Result<()> {
    real_csm_with_mode(args, CsmDispatchMode::AdlControlPlane)
}

pub(crate) fn real_csm_standalone(args: &[String]) -> Result<()> {
    real_csm_with_mode(args, CsmDispatchMode::StandaloneRuntime)
}

fn real_csm_with_mode(args: &[String], mode: CsmDispatchMode) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm requires subcommand: daemon | service | continuity | api | observatory");
        std::process::exit(2);
    };

    match cmd {
        "daemon" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_csm_daemon(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm daemon is owned by the standalone csm runtime binary; use `csm daemon`, not `adl csm daemon`"
            )),
        },
        "service" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_service(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm service is owned by the standalone csm runtime binary; use `csm service`, not `adl csm service`"
            )),
        },
        "continuity" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_continuity(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm continuity is owned by the standalone csm runtime binary; use `csm continuity`, not `adl csm continuity`"
            )),
        },
        "api" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_api(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm api is owned by the standalone csm runtime binary; use `csm api`, not `adl csm api`"
            )),
        },
        "observatory" => real_observatory(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm subcommand: {other} (expected daemon, service, continuity, api, or observatory)"
            );
            std::process::exit(2);
        }
    }
}

fn real_api(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm api requires subcommand: serve");
        std::process::exit(2);
    };
    match cmd {
        "serve" => real_api_serve(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm api subcommand: {other} (expected serve)");
            std::process::exit(2);
        }
    }
}

fn real_api_serve(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut bind = "127.0.0.1:0".to_string();
    let mut max_requests = 1usize;
    let mut idle_timeout_ms: Option<u64> = None;
    let mut otel_status_path: Option<PathBuf> = None;
    let mut otel_log_path: Option<PathBuf> = None;
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--bind" => {
                bind = required_value(args, i, "--bind")?.to_string();
                i += 1;
            }
            "--max-requests" => {
                max_requests = required_value(args, i, "--max-requests")?
                    .parse()
                    .context("csm api serve --max-requests must be an integer")?;
                i += 1;
            }
            "--once" => {
                max_requests = 1;
            }
            "--idle-timeout-ms" => {
                idle_timeout_ms = Some(
                    required_value(args, i, "--idle-timeout-ms")?
                        .parse()
                        .context("csm api serve --idle-timeout-ms must be an integer")?,
                );
                i += 1;
            }
            "--otel-status" => {
                otel_status_path = Some(PathBuf::from(required_value(args, i, "--otel-status")?));
                i += 1;
            }
            "--otel-log" => {
                otel_log_path = Some(PathBuf::from(required_value(args, i, "--otel-log")?));
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm api serve arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = serve_runtime_api(CsmRuntimeApiOptions {
        spec_path: spec.context("csm api serve requires --spec <agent-spec.yaml>")?,
        bind,
        max_requests,
        idle_timeout_ms,
        otel_status_path,
        otel_log_path,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    }
    Ok(())
}

fn real_continuity(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm continuity requires subcommand: capture | stage | restore");
        std::process::exit(2);
    };
    match cmd {
        "capture" => real_continuity_capture(&args[1..]),
        "stage" => real_continuity_stage(&args[1..]),
        "restore" => real_continuity_restore(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm continuity subcommand: {other} (expected capture, stage, or restore)"
            );
            std::process::exit(2);
        }
    }
}

fn real_continuity_capture(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut source_host = "wuji".to_string();
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--spec" => {
                spec = Some(PathBuf::from(required_value(args, i, "--spec")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--source-host" => {
                source_host = required_value(args, i, "--source-host")?.to_string();
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity capture arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = capture_capsule(ContinuityCaptureOptions {
        spec_path: spec.context("csm continuity capture requires --spec <agent-spec.yaml>")?,
        out_dir: out_dir.context("csm continuity capture requires --out <bundle-dir>")?,
        source_host,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn real_continuity_stage(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                bundle = Some(PathBuf::from(required_value(args, i, "--bundle")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity stage arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = stage_capsule(ContinuityStageOptions {
        bundle_dir: bundle.context("csm continuity stage requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity stage requires --out <stage-dir>")?,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn real_continuity_restore(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "ec2-staging".to_string();
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--bundle" => {
                bundle = Some(PathBuf::from(required_value(args, i, "--bundle")?));
                i += 1;
            }
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--target-host" => {
                target_host = required_value(args, i, "--target-host")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity restore arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = restore_capsule(ContinuityRestoreOptions {
        bundle_dir: bundle.context("csm continuity restore requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity restore requires --out <runtime-dir>")?,
        target_host,
    })?;
    print_continuity_result(&result, json_output)
}

fn required_value<'a>(args: &'a [String], i: usize, flag: &str) -> Result<&'a str> {
    args.get(i + 1)
        .map(|value| value.as_str())
        .ok_or_else(|| anyhow::anyhow!("{flag} requires a value"))
}

fn print_continuity_result(
    result: &::adl::csm_continuity_capsule::ContinuityCommandResult,
    json_output: bool,
) -> Result<()> {
    if json_output {
        println!("{}", serde_json::to_string_pretty(result)?);
    } else {
        println!(
            "CSM_CONTINUITY ok operation={} status={} bundle={}",
            result.operation,
            result.status,
            result.bundle_dir.display()
        );
    }
    Ok(())
}

fn real_observatory(args: &[String]) -> Result<()> {
    let mut packet: Option<PathBuf> = None;
    let mut out_dir = PathBuf::from("out/csm-observatory");
    let mut format = ObservatoryFormat::Bundle;

    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--packet" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --packet <visibility-packet.json>");
                    std::process::exit(2);
                };
                packet = Some(PathBuf::from(value));
                i += 1;
            }
            "--out" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --out <dir>");
                    std::process::exit(2);
                };
                out_dir = PathBuf::from(value);
                i += 1;
            }
            "--format" => {
                let Some(value) = args.get(i + 1) else {
                    eprintln!("csm observatory requires --format <bundle|json|report>");
                    std::process::exit(2);
                };
                format = ObservatoryFormat::parse(value)?;
                i += 1;
            }
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm observatory arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let packet = packet.context("csm observatory requires --packet <visibility-packet.json>")?;
    let output = write_observatory_outputs(&packet, &out_dir, format)?;

    println!(
        "CSM_OBSERVATORY ok format={format:?} out={}",
        out_dir.display()
    );
    if let Some(path) = output.packet_path {
        println!("  packet={}", path.display());
    }
    if let Some(path) = output.report_path {
        println!("  report={}", path.display());
    }
    if let Some(path) = output.console_reference_path {
        println!("  console_reference={}", path.display());
    }
    if let Some(path) = output.manifest_path {
        println!("  manifest={}", path.display());
    }
    Ok(())
}

pub(crate) fn csm_usage() -> &'static str {
    "Usage:
  csm daemon --spec <agent-spec.yaml> [--max-restarts <n>] [--checkpoint-interval-secs <n>] [--interval-secs <n>] [--recover-stale-lease] [--no-sleep] [--json]
  csm service install --spec <agent-spec.yaml> [--service-root <dir>] [--manager launchd|local] [--label <label>] [--csm-bin <path>] [--json]
  csm service start|status|stop|remove [--service-root <dir>] [--json]
  csm api serve --spec <agent-spec.yaml> [--bind 127.0.0.1:0] [--once|--max-requests <n>] [--idle-timeout-ms <n>] [--otel-status <path>] [--otel-log <path>] [--json]
  csm continuity capture --spec <agent-spec.yaml> --out <bundle-dir> [--source-host wuji] [--target-host ec2-staging|ec2|local] [--json]
  csm continuity stage --bundle <bundle-dir> --out <stage-dir> [--target-host ec2-staging|ec2|local] [--json]
  csm continuity restore --bundle <bundle-dir> --out <runtime-dir> [--target-host ec2-staging|ec2|local] [--json]
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]  # read-only control-plane inspection
  csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]

Semantics:
  - csm is the dedicated runtime owner binary.
  - csm daemon owns long-lived runtime execution, partial checkpoints, restart accounting, recoverable terminal state, and runtime observability.
  - csm service owns host service-manager installation/status around csm daemon; launchd is the primary macOS target and local mode is a bounded proof fallback.
  - csm api exposes local-by-default /status, /health, /ready, /metrics, and /events endpoints from retained runtime artifacts without leaking host-private paths or secrets.
  - csm continuity captures, stages, and restores portable continuity capsules with secrets excluded and host bindings explicit.
  - csm daemon emits ADL_OBSERVABILITY_LOG, ADL_OTEL_LOG, and ADL_OTEL_STATUS records through the shared observability contract.
  - Read-only CSM Observatory inspection.
  - Validates the visibility packet before emitting artifacts.
  - bundle writes visibility_packet.json, operator_report.md, console_reference.md, and demo_manifest.json.
  - json writes visibility_packet.json.
  - report writes operator_report.md.
  - No live Runtime v2 mutation is performed."
}
