#[path = "codefriend_github_cmd.rs"]
mod github_command;
use adl::codefriend::ingestion::{local, AdmissionInput, Scope};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};
const USAGE: &str = "Usage: adl codefriend ingest local --checkout <directory> --repository <https://host/owner/repo> --revision <full-commit-id> --scope <scope.json> --out <new-packet.json>\n       adl codefriend packet read --input <packet.json>\n       adl codefriend review run --store <store-dir> --packet-id <id> --provider-request <request.json> --out <dir> [--run-id <id>]\n       adl codefriend review synthesize --input <review-record.json> --out <new-dir>\n       adl codefriend plan remediation --input <synthesis.json> --out <new-dir>\n       adl codefriend plan remediation read --input <remediation-plan.json>\n       adl codefriend review shell start|inspect|cancel|retry|withhold-publication ...";
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
    if args.first().is_some_and(|arg| arg == "fitness") {
        return super::codefriend_fitness_cmd::run(&args[1..]);
    }
    if args.is_empty() || matches!(args[0].as_str(), "--help" | "-h") {
        println!(
            "{USAGE}\n{}\n{}\n{}\n{}",
            github_command::USAGE,
            super::codefriend_memory_cmd::USAGE,
            super::codefriend_structure_cmd::USAGE,
            super::codefriend_fitness_cmd::USAGE
        );
        return Ok(());
    }
    if args.len() >= 2 && args[0] == "review" && args[1] == "run" {
        return review_run(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "review" && args[1] == "synthesize" {
        return review_synthesize(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "review" && args[1] == "shell" {
        return review_shell(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "plan" && args[1] == "remediation" {
        return plan_remediation(&args[2..]);
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

fn plan_remediation(args: &[String]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "read") {
        let flags = exact_flags(&args[1..], &["--input"], "remediation_plan_read")?;
        let plan = adl::codefriend::actions::remediation::read_plan_from_file(Path::new(
            flags["--input"],
        ))?;
        println!("{}", serde_json::to_string(&plan)?);
        return Ok(());
    }
    let flags = exact_flags(args, &["--input", "--out"], "remediation_plan")?;
    let plan = adl::codefriend::actions::remediation::plan_from_file(
        adl::codefriend::actions::remediation::RemediationOptions {
            input: Path::new(flags["--input"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
        },
    )?;
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "schema": adl::codefriend::actions::remediation::REMEDIATION_PLAN_SCHEMA,
            "synthesis_digest": plan.synthesis_digest,
            "run_id": plan.run_id,
            "action_count": plan.actions.len(),
            "omitted_finding_count": plan.omitted_findings.len(),
            "remediation_plan": "remediation-plan.json",
            "manifest": "manifest.json"
        }))?
    );
    Ok(())
}

fn exact_flags<'a>(
    args: &'a [String],
    expected: &[&str],
    label: &str,
) -> Result<BTreeMap<&'a str, &'a str>> {
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_{label}_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_{label}_argument"
        );
    }
    ensure!(
        expected.iter().all(|flag| flags.contains_key(flag)),
        "missing_{label}_argument"
    );
    Ok(flags)
}

fn review_synthesize(args: &[String]) -> Result<()> {
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2
                && matches!(pair[0].as_str(), "--input" | "--out")
                && !pair[1].starts_with("--"),
            "invalid_synthesis_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_synthesis_argument"
        );
    }
    ensure!(
        ["--input", "--out"]
            .iter()
            .all(|flag| flags.contains_key(flag)),
        "missing_synthesis_argument"
    );
    let synthesis = adl::codefriend::review::synthesis::synthesize_from_file(
        adl::codefriend::review::synthesis::SynthesisOptions {
            input: Path::new(flags["--input"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
        },
    )?;
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "schema": adl::codefriend::review::synthesis::SYNTHESIS_SCHEMA,
            "review_record_digest": synthesis.review_record_digest,
            "run_id": synthesis.run_id,
            "input_finding_count": synthesis.input_finding_count,
            "synthesized_finding_count": synthesis.synthesized_findings.len(),
            "synthesis": "synthesis.json",
            "manifest": "manifest.json"
        }))?
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
            cancel_file: None,
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

fn review_shell(args: &[String]) -> Result<()> {
    ensure!(!args.is_empty(), "missing_review_shell_command");
    match args[0].as_str() {
        "start" => review_shell_start(&args[1..]),
        "inspect" => review_shell_inspect(&args[1..]),
        "cancel" => review_shell_cancel(&args[1..]),
        "retry" => review_shell_retry(&args[1..]),
        "withhold-publication" => review_shell_withhold_publication(&args[1..]),
        _ => anyhow::bail!("unsupported_review_shell_command"),
    }
}

fn review_shell_flags<'a>(
    args: &'a [String],
    expected: &[&str],
) -> Result<BTreeMap<&'a str, &'a str>> {
    let mut flags = BTreeMap::new();
    for pair in args.chunks(2) {
        ensure!(
            pair.len() == 2 && expected.contains(&pair[0].as_str()) && !pair[1].starts_with("--"),
            "invalid_review_shell_arguments"
        );
        ensure!(
            flags.insert(pair[0].as_str(), pair[1].as_str()).is_none(),
            "duplicate_review_shell_argument"
        );
    }
    ensure!(
        expected.iter().all(|flag| flags.contains_key(flag)),
        "missing_review_shell_argument"
    );
    Ok(flags)
}

fn print_shell_state(state: &adl::codefriend::operator::OperatorReviewState) -> Result<()> {
    println!("{}", serde_json::to_string(state)?);
    Ok(())
}

fn review_shell_start(args: &[String]) -> Result<()> {
    let flags = review_shell_flags(
        args,
        &[
            "--store",
            "--packet-id",
            "--provider-request",
            "--out",
            "--run-id",
        ],
    )?;
    let request = adl::codefriend::review::runner::read_provider_request(Path::new(
        flags["--provider-request"],
    ))?;
    let state =
        adl::codefriend::operator::start_review(adl::codefriend::operator::OperatorStartOptions {
            store: Path::new(flags["--store"]).to_path_buf(),
            packet_id: flags["--packet-id"].to_string(),
            provider_request: request,
            out: Path::new(flags["--out"]).to_path_buf(),
            run_id: flags["--run-id"].to_string(),
        })?;
    print_shell_state(&state)
}

fn review_shell_inspect(args: &[String]) -> Result<()> {
    let flags = review_shell_flags(args, &["--out"])?;
    print_shell_state(&adl::codefriend::operator::inspect_review(Path::new(
        flags["--out"],
    ))?)
}

fn review_shell_cancel(args: &[String]) -> Result<()> {
    let flags = review_shell_flags(args, &["--out", "--reason"])?;
    print_shell_state(&adl::codefriend::operator::cancel_review(
        Path::new(flags["--out"]),
        flags["--reason"],
    )?)
}

fn review_shell_retry(args: &[String]) -> Result<()> {
    let flags = review_shell_flags(args, &["--out", "--provider-request", "--run-id"])?;
    let request = adl::codefriend::review::runner::read_provider_request(Path::new(
        flags["--provider-request"],
    ))?;
    print_shell_state(&adl::codefriend::operator::retry_review(
        Path::new(flags["--out"]),
        request,
        flags["--run-id"],
    )?)
}

fn review_shell_withhold_publication(args: &[String]) -> Result<()> {
    let flags = review_shell_flags(args, &["--out", "--reason"])?;
    print_shell_state(&adl::codefriend::operator::withhold_publication(
        Path::new(flags["--out"]),
        flags["--reason"],
    )?)
}
