#[path = "codefriend_github_cmd.rs"]
mod github_command;
use adl::codefriend::ingestion::{local, AdmissionInput, Scope};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};
const USAGE: &str = "Usage: adl codefriend ingest local --checkout <directory> --repository <https://host/owner/repo> --revision <full-commit-id> --scope <scope.json> --out <new-packet.json>\n       adl codefriend packet read --input <packet.json>\n       adl codefriend review run --store <store-dir> --packet-id <id> --provider-request <request.json> --out <dir> [--run-id <id>]";
pub(super) fn real_codefriend(args: &[String]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "memory") {
        return super::codefriend_memory_cmd::run(&args[1..]);
    }
    if args.first().is_some_and(|arg| arg == "architecture") {
        return super::codefriend_structure_cmd::run(&args[1..]);
    }
    if args.len() >= 2 && args[0] == "ingest" && args[1] == "github" {
        return github_command::run(&args[2..]);
    }
    if args.first().is_some_and(|arg| arg == "evidence") {
        return super::codefriend_evidence_cmd::evidence(&args[1..]);
    }
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        println!(
            "{USAGE}\n{}\n{}\n{}",
            github_command::USAGE,
            super::codefriend_memory_cmd::USAGE,
            super::codefriend_structure_cmd::USAGE
        );
        return Ok(());
    }
    if args.len() >= 2 && args[0] == "review" && args[1] == "run" {
        return review_run(&args[2..]);
    }
    ensure!(args.len() >= 2, "{USAGE}");
    if args[0] == "ingest" && args[1] == "ci" {
        return super::codefriend_ci_cmd::run(&args[2..]);
    }
    let read = args[0] == "packet" && args[1] == "read";
    ensure!(
        read || (args[0] == "ingest" && args[1] == "local"),
        "unsupported_codefriend_command"
    );
    let expected: &[&str] = if read {
        &["--input"]
    } else {
        &[
            "--checkout",
            "--repository",
            "--revision",
            "--scope",
            "--out",
        ]
    };
    let mut flags = BTreeMap::new();
    for pair in args[2..].chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_codefriend_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_codefriend_argument"
        );
    }
    ensure!(flags.len() == expected.len(), "missing_codefriend_argument");
    let packet = if read {
        AdmissionInput::read(Path::new(flags["--input"]))?
            .packet()
            .clone()
    } else {
        use std::io::Read;
        let mut bytes = Vec::new();
        std::fs::File::open(flags["--scope"])
            .map_err(|_| anyhow::anyhow!("scope_open_failed"))?
            .take(128 * 1024 + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("scope_read_failed"))?;
        ensure!(bytes.len() <= 128 * 1024, "scope_byte_limit_exceeded");
        let scope: Scope =
            serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_scope_json"))?;
        let root = Path::new(flags["--checkout"]);
        let packet = local::acquire(root, flags["--repository"], flags["--revision"], scope)?;
        local::write_packet(root, Path::new(flags["--out"]), &packet)?;
        // Actual production reader validates the just-published acquisition artifact.
        AdmissionInput::read(Path::new(flags["--out"]))?
            .packet()
            .clone()
    };
    println!(
        "{}",
        serde_json::to_string(
            &serde_json::json!({"schema":"codefriend.acquisition_result.v1","packet_id":packet.packet_id,"revision":packet.revision,"objects":packet.objects.len(),"completeness":packet.completeness,"review_state":packet.review_state})
        )?
    );
    Ok(())
}

fn review_run(args: &[String]) -> Result<()> {
    let mut flags = BTreeMap::new();
    let mut optional = BTreeMap::new();
    let mut i = 0;
    while i < args.len() {
        let flag = args[i].as_str();
        ensure!(
            i + 1 < args.len() && !args[i + 1].starts_with("--"),
            "invalid_review_arguments"
        );
        let value = args[i + 1].as_str();
        match flag {
            "--store" | "--packet-id" | "--provider-request" | "--out" => {
                ensure!(
                    flags.insert(flag, value).is_none(),
                    "duplicate_review_argument"
                );
            }
            "--run-id" => {
                ensure!(
                    optional.insert(flag, value).is_none(),
                    "duplicate_review_argument"
                );
            }
            _ => anyhow::bail!("unsupported_review_argument"),
        }
        i += 2;
    }
    ensure!(
        ["--store", "--packet-id", "--provider-request", "--out"]
            .iter()
            .all(|flag| flags.contains_key(flag)),
        "missing_review_argument"
    );
    let run_id = optional
        .get("--run-id")
        .copied()
        .map(str::to_string)
        .unwrap_or_else(|| {
            format!(
                "codefriend-review-{}",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
            )
        });
    let request = adl::codefriend::review::runner::read_provider_request(Path::new(
        flags["--provider-request"],
    ))?;
    let output = adl::codefriend::review::runner::run_from_store(
        adl::codefriend::review::runner::ReviewRunOptions {
            store: Path::new(flags["--store"]).to_path_buf(),
            packet_id: flags["--packet-id"].to_string(),
            provider_request: request,
            out: Path::new(flags["--out"]).to_path_buf(),
            run_id,
        },
    )?;
    println!(
        "{}",
        serde_json::to_string(&adl::codefriend::review::runner::review_run_summary(
            &output
        )?)?
    );
    Ok(())
}
