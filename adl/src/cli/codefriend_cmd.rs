#[path = "codefriend_github_cmd.rs"]
mod github_command;
use adl::codefriend::ingestion::{local, AdmissionInput, Scope};
use anyhow::{ensure, Result};
use std::{collections::BTreeMap, path::Path};
const USAGE: &str = "Usage: adl codefriend journey resume --output <journey-directory> --request <step.json>\n       adl codefriend journey admitted --request <acquisition-options.json>\n        adl codefriend journey local --request <journey.json> [--provider-request <request.json> --run-id <id> --destination <existing-directory>]\n       adl codefriend ingest local --checkout <directory> --repository <https://host/owner/repo> --revision <full-commit-id> --scope <scope.json> --out <new-packet.json>\n       adl codefriend packet read --input <packet.json>\n       adl codefriend review run --store <store-dir> --packet-id <id> --provider-request <request.json> --out <dir> [--run-id <id>]\n       adl codefriend review synthesize --input <review-record.json> --out <new-dir>\n       adl codefriend plan remediation --input <synthesis.json> --out <new-dir>\n       adl codefriend plan remediation read --input <remediation-plan.json>\n       adl codefriend plan tests --input <synthesis.json> --out <new-dir>\n       adl codefriend plan tests read --input <test-plan.json>\n       adl codefriend publication prepare|approve|withhold|invalidate|inspect|admit ...\n       adl codefriend export markdown|html --review-record <review-record.json> --publication <publication.json> --approval-store <dir> --artifact-root <dir> --synthesis <relative-path> --remediation-plan <relative-path> --test-plan <relative-path> --destination-root <dir> --out <new-dir>\n       adl codefriend export pdf --review-record <review-record.json> --publication <publication.json> --approval-store <dir> --artifact-root <dir> --synthesis <relative-path> --remediation-plan <relative-path> --test-plan <relative-path> --destination-root <dir> --out <new-dir> --font <font.ttf>\n       adl codefriend review shell start|inspect|cancel|retry|withhold-publication ...";
pub(super) fn real_codefriend(args: &[String]) -> Result<()> {
    if args.len() >= 2 && args[0] == "journey" && args[1] == "resume" {
        return journey_resume(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "journey" && args[1] == "admitted" {
        return journey_admitted(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "journey" && args[1] == "local" {
        return journey_local(&args[2..]);
    }
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
    if args.first().is_some_and(|arg| arg == "publication") {
        return super::codefriend_publication_cmd::run(&args[1..]);
    }
    if args.len() >= 2 && args[0] == "export" && args[1] == "markdown" {
        return export_markdown(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "export" && args[1] == "pdf" {
        return export_pdf(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "export" && args[1] == "html" {
        return export_html(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "plan" && args[1] == "remediation" {
        return plan_remediation(&args[2..]);
    }
    if args.len() >= 2 && args[0] == "plan" && args[1] == "tests" {
        return plan_tests(&args[2..]);
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

fn export_markdown(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--publication",
            "--approval-store",
            "--artifact-root",
            "--synthesis",
            "--remediation-plan",
            "--test-plan",
            "--destination-root",
            "--out",
        ],
        "markdown_export",
    )?;
    let result = adl::codefriend::publication::render_markdown(
        adl::codefriend::publication::MarkdownRenderOptions {
            review_record: Path::new(flags["--review-record"]).to_path_buf(),
            publication: Path::new(flags["--publication"]).to_path_buf(),
            approval_store: Path::new(flags["--approval-store"]).to_path_buf(),
            artifact_root: Path::new(flags["--artifact-root"]).to_path_buf(),
            synthesis: Path::new(flags["--synthesis"]).to_path_buf(),
            remediation_plan: Path::new(flags["--remediation-plan"]).to_path_buf(),
            test_plan: Path::new(flags["--test-plan"]).to_path_buf(),
            destination_root: Path::new(flags["--destination-root"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
        },
    )?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn export_pdf(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--publication",
            "--approval-store",
            "--artifact-root",
            "--synthesis",
            "--remediation-plan",
            "--test-plan",
            "--destination-root",
            "--out",
            "--font",
        ],
        "pdf_export",
    )?;
    let result =
        adl::codefriend::publication::render_pdf(adl::codefriend::publication::PdfRenderOptions {
            review_record: Path::new(flags["--review-record"]).to_path_buf(),
            publication: Path::new(flags["--publication"]).to_path_buf(),
            approval_store: Path::new(flags["--approval-store"]).to_path_buf(),
            artifact_root: Path::new(flags["--artifact-root"]).to_path_buf(),
            synthesis: Path::new(flags["--synthesis"]).to_path_buf(),
            remediation_plan: Path::new(flags["--remediation-plan"]).to_path_buf(),
            test_plan: Path::new(flags["--test-plan"]).to_path_buf(),
            destination_root: Path::new(flags["--destination-root"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
            font: Path::new(flags["--font"]).to_path_buf(),
        })?;
    println!("{}", serde_json::to_string(&result)?);
    Ok(())
}

fn export_html(args: &[String]) -> Result<()> {
    let flags = exact_flags(
        args,
        &[
            "--review-record",
            "--publication",
            "--approval-store",
            "--artifact-root",
            "--synthesis",
            "--remediation-plan",
            "--test-plan",
            "--destination-root",
            "--out",
        ],
        "html_export",
    )?;
    let result = adl::codefriend::publication::render_html(
        adl::codefriend::publication::HtmlRenderOptions {
            review_record: Path::new(flags["--review-record"]).to_path_buf(),
            publication: Path::new(flags["--publication"]).to_path_buf(),
            approval_store: Path::new(flags["--approval-store"]).to_path_buf(),
            artifact_root: Path::new(flags["--artifact-root"]).to_path_buf(),
            synthesis: Path::new(flags["--synthesis"]).to_path_buf(),
            remediation_plan: Path::new(flags["--remediation-plan"]).to_path_buf(),
            test_plan: Path::new(flags["--test-plan"]).to_path_buf(),
            destination_root: Path::new(flags["--destination-root"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
        },
    )?;
    println!("{}", serde_json::to_string(&result)?);
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

fn plan_tests(args: &[String]) -> Result<()> {
    if args.first().is_some_and(|arg| arg == "read") {
        let flags = exact_flags(&args[1..], &["--input"], "test_plan_read")?;
        let plan =
            adl::codefriend::actions::test_plan::read_plan_from_file(Path::new(flags["--input"]))?;
        println!("{}", serde_json::to_string(&plan)?);
        return Ok(());
    }
    let flags = exact_flags(args, &["--input", "--out"], "test_plan")?;
    let plan = adl::codefriend::actions::test_plan::plan_from_file(
        adl::codefriend::actions::test_plan::TestPlanOptions {
            input: Path::new(flags["--input"]).to_path_buf(),
            out: Path::new(flags["--out"]).to_path_buf(),
        },
    )?;
    println!(
        "{}",
        serde_json::to_string(&serde_json::json!({
            "schema": adl::codefriend::actions::test_plan::TEST_PLAN_SCHEMA,
            "synthesis_digest": plan.synthesis_digest,
            "run_id": plan.run_id,
            "test_case_count": plan.test_cases.len(),
            "omitted_finding_count": plan.omitted_findings.len(),
            "test_plan": "test-plan.json",
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

fn journey_local(args: &[String]) -> Result<()> {
    use adl::codefriend::integration::{
        journey::{prepare_local, LocalJourneyOptions, StageStatus},
        PublicationFormat,
    };
    use std::io::Read;
    let with_provider = args.iter().any(|arg| arg == "--provider-request");
    let expected: &[&str] = if with_provider {
        &[
            "--request",
            "--provider-request",
            "--run-id",
            "--destination",
        ]
    } else {
        &["--request"]
    };
    let flags = exact_flags(args, expected, "journey")?;
    let mut bytes = Vec::new();
    std::fs::File::open(flags["--request"])
        .map_err(|_| anyhow::anyhow!("journey_request_open_failed"))?
        .take(131073)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 131072, "journey_request_too_large");
    let options: LocalJourneyOptions =
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_journey_request"))?;
    // Validate provider input before acquisition, but dispatch only after native preparation.
    let provider = if with_provider {
        Some(adl::codefriend::review::runner::read_provider_request(
            Path::new(flags["--provider-request"]),
        )?)
    } else {
        None
    };
    let mut journey = prepare_local(options)?;
    if let Some(request) = provider {
        if journey.manifest().status != StageStatus::Failed {
            journey.run_review(request, flags["--run-id"].into(), None)?;
            if journey.manifest().status != StageStatus::Failed {
                for format in [
                    PublicationFormat::Markdown,
                    PublicationFormat::Html,
                    PublicationFormat::Pdf,
                ] {
                    journey.prepare_publication(Path::new(flags["--destination"]), format)?;
                    if journey.manifest().status == StageStatus::Failed {
                        break;
                    }
                }
            }
        }
    }
    println!("{}", serde_json::to_string(journey.manifest())?);
    ensure!(
        journey.manifest().status != StageStatus::Failed,
        "journey_stage_failed"
    );
    Ok(())
}

fn journey_resume(args: &[String]) -> Result<()> {
    use adl::codefriend::integration::journey::{resume, Continuation, StageStatus};
    let flags = exact_flags(args, &["--output", "--request"], "journey resume")?;
    let step: Continuation = journey_read_request(Path::new(flags["--request"]))?;
    let mut journey = resume(Path::new(flags["--output"]))?;
    journey.continue_with(step)?;
    println!("{}", serde_json::to_string(journey.manifest())?);
    ensure!(
        journey.manifest().status != StageStatus::Failed,
        "journey_stage_failed"
    );
    Ok(())
}
fn journey_read_request<T: serde::de::DeserializeOwned>(path: &Path) -> Result<T> {
    use std::io::Read;
    let mut bytes = Vec::new();
    std::fs::File::open(path)?
        .take(131073)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 131072, "journey_request_too_large");
    Ok(serde_json::from_slice(&bytes)?)
}
fn journey_admitted(args: &[String]) -> Result<()> {
    use adl::codefriend::integration::journey::{
        prepare_source, AcquisitionSource, LocalJourneyOptions, StageStatus,
    };
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Input {
        options: LocalJourneyOptions,
        acquisition: AcquisitionSource,
    }
    let flags = exact_flags(args, &["--request"], "journey admitted")?;
    let request: Input = journey_read_request(Path::new(flags["--request"]))?;
    let journey = prepare_source(request.options, request.acquisition)?;
    println!("{}", serde_json::to_string(journey.manifest())?);
    ensure!(
        journey.manifest().status != StageStatus::Failed,
        "journey_stage_failed"
    );
    Ok(())
}
