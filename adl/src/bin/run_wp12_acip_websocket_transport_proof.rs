use std::fs;
use std::path::{Component, Path, PathBuf};

use adl::agent_comms::prove_acip_runtime_stream_websocket_transport_path_v1;
use anyhow::{anyhow, Context, Result};
use clap::Parser;
use serde_json::{json, Value};
use sha2::Digest;

const DEFAULT_OUT: &str =
    "docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659";

#[derive(Debug, Parser)]
#[command(name = "run_wp12_acip_websocket_transport_proof")]
#[command(about = "Generate the retained WP-12 ACIP WebSocket transport proof packet for #4659")]
struct Args {
    #[arg(long, default_value = DEFAULT_OUT)]
    out: PathBuf,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    run(args).await
}

async fn run(args: Args) -> Result<()> {
    reject_unsafe_out_path(&args.out)?;
    if args.out.exists() {
        fs::remove_dir_all(&args.out)
            .with_context(|| format!("reset existing output dir {}", args.out.display()))?;
    }
    fs::create_dir_all(args.out.join("audit"))
        .with_context(|| format!("create output dir {}", args.out.display()))?;

    let proof = prove_acip_runtime_stream_websocket_transport_path_v1().await?;
    write_json(
        &args.out.join("acip_websocket_transport_proof.json"),
        &proof,
    )?;
    write_file(&args.out.join("README.md"), &readme())?;
    write_file(
        &args.out.join("reviewer_walkthrough.md"),
        &reviewer_walkthrough(),
    )?;

    let evidence_index = json!({
        "schema_version": "wp12.acip.websocket_transport.evidence_index.v1",
        "issue": 4659,
        "proof": "acip_websocket_transport_proof.json",
        "reviewer_walkthrough": "reviewer_walkthrough.md",
        "positive_case": proof.positive_case.case_name,
        "failure_cases": proof.failure_cases.iter().map(|case| case.case_name.clone()).collect::<Vec<_>>(),
        "integration_refs": proof.integration_refs,
        "non_claims": proof.non_claims,
    });
    write_json(&args.out.join("evidence_index.json"), &evidence_index)?;

    let artifact_scan = scan_artifacts(&args.out)?;
    write_json(
        &args.out.join("audit/artifact_safety_scan.json"),
        &artifact_scan,
    )?;
    if !artifact_scan
        .get("passed")
        .and_then(Value::as_bool)
        .unwrap_or(false)
    {
        return Err(anyhow!(
            "ACIP WebSocket transport proof artifact scan failed"
        ));
    }

    println!("out={}", args.out.display());
    println!(
        "proof={}",
        args.out
            .join("acip_websocket_transport_proof.json")
            .display()
    );
    Ok(())
}

fn reject_unsafe_out_path(path: &Path) -> Result<()> {
    if path.is_absolute() {
        return Err(anyhow!("--out must be repository-relative"));
    }
    if path.as_os_str().is_empty() || path == Path::new(".") {
        return Err(anyhow!(
            "--out must name a dedicated review artifact directory"
        ));
    }
    if path
        .components()
        .any(|component| matches!(component, Component::ParentDir))
    {
        return Err(anyhow!("--out must not contain parent-directory traversal"));
    }
    let normalized = path
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    let expected = [
        "docs",
        "milestones",
        "v0.91.7",
        "review",
        "runtime",
        "wp12_acip_websocket_transport_4659",
    ];
    if normalized != expected {
        return Err(anyhow!("--out must be {}", expected.join("/")));
    }
    Ok(())
}

fn write_json<T: serde::Serialize>(path: &Path, value: &T) -> Result<()> {
    let bytes = serde_json::to_vec_pretty(value)?;
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, bytes).with_context(|| format!("write {}", path.display()))
}

fn write_file(path: &Path, contents: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(path, contents).with_context(|| format!("write {}", path.display()))
}

fn readme() -> String {
    "# WP-12 ACIP WebSocket Transport Proof (#4659)\n\n\
This retained packet proves the bounded v0.91.7 ACIP WebSocket transport path for #4659. \
It exercises `tokio-tungstenite` server/client mechanics, validates ACIP JSON envelopes at the transport boundary, applies the WP-12 fail-closed access policy, and records malformed, denied, close-before-response, and timeout failure behavior.\n\n\
Non-claims: this packet does not claim production TLS termination, production authentication, cross-polis networking, or protobuf wire encoding.\n"
        .to_string()
}

fn reviewer_walkthrough() -> String {
    "# Reviewer Walkthrough\n\n\
1. Inspect `acip_websocket_transport_proof.json`.\n\
2. Confirm `positive_case.status` is `delivered`.\n\
3. Confirm failure cases include `malformed_message`, `auth_policy_denial`, `peer_close_before_response`, and `response_timeout` with `failed_closed` status.\n\
4. Inspect `audit/artifact_safety_scan.json` for retained-artifact hygiene.\n\
5. Re-run with `cargo run --manifest-path adl/Cargo.toml --bin run_wp12_acip_websocket_transport_proof -- --out docs/milestones/v0.91.7/review/runtime/wp12_acip_websocket_transport_4659`.\n"
        .to_string()
}

fn scan_artifacts(root: &Path) -> Result<Value> {
    let mut files = Vec::new();
    let mut findings = Vec::new();
    scan_dir(root, root, &mut files, &mut findings)?;
    Ok(json!({
        "schema_version": "wp12.acip.websocket_transport.artifact_safety_scan.v1",
        "passed": findings.is_empty(),
        "files": files,
        "findings": findings,
        "self_scan_boundary": "audit/artifact_safety_scan.json is written after this scan is computed; rerunning the proof regenerates and rechecks the source artifacts deterministically.",
        "checks": [
            "no_absolute_host_paths",
            "no_secret_like_literals",
            "sha256_recorded"
        ]
    }))
}

fn scan_dir(
    root: &Path,
    dir: &Path,
    files: &mut Vec<Value>,
    findings: &mut Vec<Value>,
) -> Result<()> {
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            scan_dir(root, &path, files, findings)?;
            continue;
        }
        let rel = path
            .strip_prefix(root)
            .context("artifact path must be under root")?
            .to_string_lossy()
            .replace('\\', "/");
        let bytes = fs::read(&path)?;
        let digest = sha2::Sha256::digest(&bytes);
        files.push(json!({
            "path": rel,
            "sha256": format!("{digest:x}"),
            "bytes": bytes.len(),
        }));
        let text = String::from_utf8_lossy(&bytes);
        for marker in [
            "/Users/",
            "sk-",
            "BEGIN OPENSSH",
            "BEGIN PRIVATE KEY",
            "AWS_SECRET_ACCESS_KEY",
            "GITHUB_TOKEN",
        ] {
            if text.contains(marker) {
                findings.push(json!({
                    "path": rel,
                    "marker": marker,
                }));
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn output_path_guard_rejects_destructive_or_unscoped_paths() {
        for path in [
            Path::new("."),
            Path::new("adl/src"),
            Path::new(".git"),
            Path::new("docs/milestones/v0.91.7/review/runtime"),
            Path::new("docs/milestones/v0.91.7/review/runtime/other"),
        ] {
            let error = reject_unsafe_out_path(path)
                .expect_err(&format!("expected rejection for {}", path.display()));
            assert!(
                error.to_string().contains("--out must"),
                "unexpected error for {}: {error}",
                path.display()
            );
        }

        reject_unsafe_out_path(Path::new(DEFAULT_OUT))
            .expect("default retained proof path should be accepted");
    }
}
