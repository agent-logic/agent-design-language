use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use csdlc_v2::finish::{
    as_merge_request, derive_terminal, lock_finish_operation, retain_cached_terminal,
    validate_canonical_identity, FinishRequest, FinishResult, IssueTerminalObservation,
};
use csdlc_v2::github::{
    collect_pr_state, execute_github_action, GithubAction, GithubActionRequest, PrStateRequest,
};
use csdlc_v2::github_token;
use csdlc_v2::merge::{execute_remote_merge, validate_canonical, validate_remote};
use csdlc_v2::{ErrorCode, Store, V2Error};

#[derive(Parser)]
#[command(about = "Finish one C-SDLC v2 issue from exact live GitHub terminal truth")]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(long)]
    request: PathBuf,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    match run(cli).await {
        Ok(result) => println!("{}", serde_json::to_string_pretty(&result).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(cli: Cli) -> csdlc_v2::Result<FinishResult> {
    let request: FinishRequest = serde_json::from_slice(&fs::read(cli.request)?)?;
    let store = Store::new(&cli.root);
    let _finish_lock = lock_finish_operation(store.root(), request.issue)?;
    let record = store.load_record(request.issue)?;
    validate_canonical_identity(&record, &request)?;
    let issue = read_issue(&request).await?;
    let observation = issue_observation(issue, now_unix_seconds()?);
    let packet = match request.pull_request {
        Some(pull_request) => Some(
            collect_pr_state(&PrStateRequest {
                repository: request.repository.clone(),
                pull_request,
                required_checks: request.required_checks.clone(),
                require_review: request.require_review,
                token_file: request.token_file.clone(),
                linked_issue: Some(request.issue),
            })
            .await?,
        ),
        None => None,
    };

    if let Some(terminal) = derive_terminal(&record, &request, &observation, packet.as_ref())? {
        retain_cached_terminal(store.root(), &terminal)?;
        return Ok(FinishResult {
            schema: "csdlc.finish_result.v1".into(),
            terminal,
            already_terminal: true,
        });
    }

    let state = packet.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "issue is open and has no PR terminal authority",
        )
    })?;
    let merge_request = as_merge_request(&request)?;
    validate_canonical(&record, &merge_request, now_unix_seconds()?)?;
    validate_remote(state, &merge_request)?;

    let token = github_token::resolve(request.token_file.as_deref())?;
    let _ = execute_remote_merge(&merge_request, token).await?;
    let observed_issue = read_issue(&request).await?;
    let observed_issue = issue_observation(observed_issue, now_unix_seconds()?);
    let observed_pr = collect_pr_state(&PrStateRequest {
        repository: request.repository.clone(),
        pull_request: merge_request.pull_request,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: Some(request.issue),
    })
    .await?;
    let terminal = derive_terminal(&record, &request, &observed_issue, Some(&observed_pr))?
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "merge returned success but GitHub did not re-observe terminal PR state",
            )
        })?;
    retain_cached_terminal(store.root(), &terminal)?;
    Ok(FinishResult {
        schema: "csdlc.finish_result.v1".into(),
        terminal,
        already_terminal: false,
    })
}

async fn read_issue(request: &FinishRequest) -> csdlc_v2::Result<csdlc_v2::GithubIssuePacket> {
    let result = execute_github_action(&GithubActionRequest {
        repository: request.repository.clone(),
        action: GithubAction::IssueRead,
        operation_key: None,
        token_file: request.token_file.clone(),
        issue: Some(request.issue),
        pull_request: None,
        title: None,
        body: None,
        labels: Vec::new(),
        assignees: Vec::new(),
        milestone: None,
        state: None,
        comment_body: None,
        required_checks: Vec::new(),
        require_review: false,
        linked_issue: None,
    })
    .await?;
    result
        .issue
        .ok_or_else(|| V2Error::new(ErrorCode::RemoteFailure, "issue read returned no issue"))
}

fn issue_observation(
    issue: csdlc_v2::GithubIssuePacket,
    observed_unix_seconds: u64,
) -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: issue.state,
        labels: issue.labels,
        observed_unix_seconds,
    }
}

fn now_unix_seconds() -> csdlc_v2::Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| V2Error::new(ErrorCode::InvalidClaim, error.to_string()))
}
