use anyhow::{Context, Result};
use std::path::PathBuf;

use super::agent_cmd::real_csm_daemon;
use super::csm_service_cmd::real_service;
use ::adl::csm_backpressure::{prove_backpressure, BackpressureProofOptions};
use ::adl::csm_continuity_capsule::{
    capture_capsule, fire_drill_capsule, restore_capsule, stage_capsule, ContinuityCaptureOptions,
    ContinuityFireDrillOptions, ContinuityRestoreOptions, ContinuityStageOptions,
};
use ::adl::csm_observatory::{write_observatory_outputs, ObservatoryFormat};
use ::adl::csm_polis_storage::{prove_polis_storage, PolisStorageProofOptions};
use ::adl::csm_runtime_api::{serve_runtime_api, CsmRuntimeApiOptions};
use ::adl::wp08_acip_sns_proof::run_wp08_acip_sns_live_proof;

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
        eprintln!(
            "csm requires subcommand: daemon | service | continuity | backpressure | api | aws-signal | storage | observatory"
        );
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
        "backpressure" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_backpressure(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm backpressure is owned by the standalone csm runtime binary; use `csm backpressure`, not `adl csm backpressure`"
            )),
        },
        "api" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_api(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm api is owned by the standalone csm runtime binary; use `csm api`, not `adl csm api`"
            )),
        },
        "aws-signal" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_aws_signal(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm aws-signal is owned by the standalone csm runtime binary; use `csm aws-signal`, not `adl csm aws-signal`"
            )),
        },
        "storage" => match mode {
            CsmDispatchMode::StandaloneRuntime => real_storage(&args[1..]),
            CsmDispatchMode::AdlControlPlane => Err(anyhow::anyhow!(
                "csm storage is owned by the standalone csm runtime binary; use `csm storage`, not `adl csm storage`"
            )),
        },
        "observatory" => real_observatory(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm subcommand: {other} (expected daemon, service, continuity, backpressure, api, aws-signal, storage, or observatory)"
            );
            std::process::exit(2);
        }
    }
}

fn real_aws_signal(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm aws-signal requires subcommand: acip-sns-proof");
        std::process::exit(2);
    };
    match cmd {
        "acip-sns-proof" => run_wp08_acip_sns_live_proof(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm aws-signal subcommand: {other} (expected acip-sns-proof)");
            std::process::exit(2);
        }
    }
}

fn real_storage(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm storage requires subcommand: prove-s3");
        std::process::exit(2);
    };
    match cmd {
        "prove-s3" => real_storage_prove_s3(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm storage subcommand: {other} (expected prove-s3)");
            std::process::exit(2);
        }
    }
}

fn real_storage_prove_s3(args: &[String]) -> Result<()> {
    let mut out_dir: Option<PathBuf> = None;
    let mut bucket: Option<String> = None;
    let mut prefix = "community-memory/".to_string();
    let mut profile = std::env::var("ADL_AWS_PROFILE")
        .or_else(|_| std::env::var("AWS_PROFILE"))
        .unwrap_or_else(|_| "agent-logic-admin".to_string());
    let mut region = std::env::var("ADL_AWS_REGION").unwrap_or_else(|_| "us-west-2".to_string());
    let mut expected_account_sha256 =
        std::env::var("ADL_AWS_POLIS_STORAGE_ACCOUNT_SHA256").unwrap_or_default();
    let mut run_id = "wp08-4913-polis-storage".to_string();
    let mut aws_bin = std::env::var("AWS_BIN").unwrap_or_else(|_| "aws".to_string());
    let mut json_output = false;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--out" => {
                out_dir = Some(PathBuf::from(required_value(args, i, "--out")?));
                i += 1;
            }
            "--bucket" => {
                bucket = Some(required_value(args, i, "--bucket")?.to_string());
                i += 1;
            }
            "--prefix" => {
                prefix = required_value(args, i, "--prefix")?.to_string();
                i += 1;
            }
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--region" => {
                region = required_value(args, i, "--region")?.to_string();
                i += 1;
            }
            "--expected-account-sha256" => {
                expected_account_sha256 =
                    required_value(args, i, "--expected-account-sha256")?.to_string();
                i += 1;
            }
            "--run-id" => {
                run_id = required_value(args, i, "--run-id")?.to_string();
                i += 1;
            }
            "--aws-bin" => {
                aws_bin = required_value(args, i, "--aws-bin")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm storage prove-s3 arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }

    let result = prove_polis_storage(PolisStorageProofOptions {
        out_dir: out_dir.context("csm storage prove-s3 requires --out <proof-dir>")?,
        bucket: bucket.context("csm storage prove-s3 requires --bucket <bucket>")?,
        prefix,
        profile,
        region,
        expected_account_sha256,
        run_id,
        aws_bin,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "CSM_POLIS_STORAGE ok status={} key={}",
            result.status, result.object.key
        );
    }
    Ok(())
}

fn real_backpressure(args: &[String]) -> Result<()> {
    let Some(cmd) = args.first().map(|value| value.as_str()) else {
        eprintln!("csm backpressure requires subcommand: prove");
        std::process::exit(2);
    };
    match cmd {
        "prove" => real_backpressure_prove(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!("unknown csm backpressure subcommand: {other} (expected prove)");
            std::process::exit(2);
        }
    }
}

fn real_backpressure_prove(args: &[String]) -> Result<()> {
    let mut spec: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut profile = "local".to_string();
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
            "--profile" => {
                profile = required_value(args, i, "--profile")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm backpressure prove arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = prove_backpressure(BackpressureProofOptions {
        spec_path: spec.context("csm backpressure prove requires --spec <agent-spec.yaml>")?,
        out_dir: out_dir.context("csm backpressure prove requires --out <proof-dir>")?,
        profile,
    })?;
    if json_output {
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!(
            "CSM_BACKPRESSURE ok status={} report={}",
            result.status, result.report_ref
        );
    }
    Ok(())
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
        eprintln!("csm continuity requires subcommand: capture | stage | restore | drill");
        std::process::exit(2);
    };
    match cmd {
        "capture" => real_continuity_capture(&args[1..]),
        "stage" => real_continuity_stage(&args[1..]),
        "restore" => real_continuity_restore(&args[1..]),
        "drill" => real_continuity_drill(&args[1..]),
        "--help" | "-h" => {
            println!("{}", csm_usage());
            Ok(())
        }
        other => {
            eprintln!(
                "unknown csm continuity subcommand: {other} (expected capture, stage, restore, or drill)"
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

fn real_continuity_drill(args: &[String]) -> Result<()> {
    let mut bundle: Option<PathBuf> = None;
    let mut out_dir: Option<PathBuf> = None;
    let mut target_host = "local".to_string();
    let mut cadence = "manual".to_string();
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
            "--cadence" => {
                cadence = required_value(args, i, "--cadence")?.to_string();
                i += 1;
            }
            "--json" => json_output = true,
            "--help" | "-h" => {
                println!("{}", csm_usage());
                return Ok(());
            }
            other => {
                eprintln!("unknown csm continuity drill arg: {other}");
                std::process::exit(2);
            }
        }
        i += 1;
    }
    let result = fire_drill_capsule(ContinuityFireDrillOptions {
        bundle_dir: bundle.context("csm continuity drill requires --bundle <bundle-dir>")?,
        out_dir: out_dir.context("csm continuity drill requires --out <drill-dir>")?,
        target_host,
        cadence,
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
  csm aws-signal acip-sns-proof --out <proof-dir> [--run-id <id>] [--projection-level delivery_metadata|content_summary]
  csm backpressure prove --spec <agent-spec.yaml> --out <proof-dir> [--profile local|soak2|pre-v0.92] [--json]
  csm storage prove-s3 --out <proof-dir> --bucket <bucket> --expected-account-sha256 <sha256> [--prefix community-memory/] [--profile agent-logic-admin] [--region us-west-2] [--run-id <id>] [--json]
  csm continuity capture --spec <agent-spec.yaml> --out <bundle-dir> [--source-host wuji] [--target-host ec2-staging|ec2|local] [--json]
  csm continuity stage --bundle <bundle-dir> --out <stage-dir> [--target-host ec2-staging|ec2|local] [--json]
  csm continuity restore --bundle <bundle-dir> --out <runtime-dir> [--target-host ec2-staging|ec2|local] [--json]
  csm continuity drill --bundle <bundle-dir> --out <drill-dir> [--target-host local|ec2-staging] [--cadence daily|per-release|pre-v0.92|manual] [--json]
  adl csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]  # read-only control-plane inspection
  csm observatory --packet <visibility-packet.json> [--format bundle|json|report] [--out <dir>]

Semantics:
  - csm is the dedicated runtime owner binary.
  - csm daemon owns long-lived runtime execution, partial checkpoints, restart accounting, recoverable terminal state, and runtime observability.
  - csm service owns host service-manager installation/status around csm daemon; launchd is the primary macOS target and local mode is a bounded proof fallback.
  - csm api exposes local-by-default /status, /health, /ready, /metrics, and /events endpoints from retained runtime artifacts without leaking host-private paths or secrets.
  - csm aws-signal owns runtime AWS signal proof execution, including ACIP-to-SNS live publication under the Agent Logic account guard.
  - csm backpressure proves bounded overload policy, retained metrics, and safe-fail serialization triggers for capacity-degraded runtime paths.
  - csm storage proves Polis durable-state write/read/restore semantics against the approved S3 backend with checksum, immutable reference, and negative-case evidence.
  - csm continuity captures, stages, restores, and fire-drills portable continuity capsules with secrets excluded and host bindings explicit.
  - csm daemon emits ADL_OBSERVABILITY_LOG, ADL_OTEL_LOG, and ADL_OTEL_STATUS records through the shared observability contract.
  - Read-only CSM Observatory inspection.
  - Validates the visibility packet before emitting artifacts.
  - bundle writes visibility_packet.json, operator_report.md, console_reference.md, and demo_manifest.json.
  - json writes visibility_packet.json.
  - report writes operator_report.md.
  - No live Runtime v2 mutation is performed."
}

#[cfg(test)]
mod tests {
    use super::{real_csm, real_csm_standalone};

    #[test]
    fn standalone_csm_accepts_aws_signal_help() {
        let args = vec!["aws-signal".to_string(), "--help".to_string()];
        real_csm_standalone(&args).expect("standalone csm owns aws-signal");
    }

    #[test]
    fn adl_control_plane_rejects_aws_signal_runtime_surface() {
        let args = vec!["aws-signal".to_string(), "--help".to_string()];
        let error = real_csm(&args).expect_err("adl csm must not own aws-signal runtime surface");
        assert!(
            error.to_string().contains("standalone csm runtime binary"),
            "{error}"
        );
    }

    #[test]
    fn standalone_csm_accepts_storage_help() {
        let args = vec!["storage".to_string(), "--help".to_string()];
        real_csm_standalone(&args).expect("standalone csm owns storage");
    }

    #[test]
    fn adl_control_plane_rejects_storage_runtime_surface() {
        let args = vec!["storage".to_string(), "--help".to_string()];
        let error = real_csm(&args).expect_err("adl csm must not own storage runtime surface");
        assert!(
            error.to_string().contains("standalone csm runtime binary"),
            "{error}"
        );
    }
}
