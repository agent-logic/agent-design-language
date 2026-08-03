use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use clap::Parser;
use csdlc_v2::github::{collect_pr_state, PrStateRequest};
use csdlc_v2::github_token;
use csdlc_v2::merge::{
    build_result, execute_remote_merge, validate_canonical, validate_remote, MergeRequest,
    MergeResult,
};
use csdlc_v2::{ErrorCode, Store, V2Error};

#[derive(Parser)]
#[command(about = "Perform one fail-closed exact-head C-SDLC v2 GitHub merge")]
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

async fn run(cli: Cli) -> csdlc_v2::Result<MergeResult> {
    let request: MergeRequest = serde_json::from_slice(&fs::read(cli.request)?)?;
    csdlc_v2::merge::validate_request(&request)?;
    let store = Store::new(&cli.root);
    let record = store.load_record(request.issue)?;
    validate_canonical(&record, &request, now_unix_seconds()?)?;
    let token = github_token::resolve(request.token_file.as_deref())?;
    let state = collect_pr_state(&PrStateRequest {
        repository: request.repository.clone(),
        pull_request: request.pull_request,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
        linked_issue: Some(request.issue),
    })
    .await?;
    validate_remote(&state, &request)?;
    let (merge_sha, already_merged) = execute_remote_merge(&request, token).await?;
    Ok(build_result(&request, merge_sha, already_merged))
}

fn now_unix_seconds() -> csdlc_v2::Result<u64> {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .map_err(|error| {
            V2Error::new(
                ErrorCode::InvalidClaim,
                format!("clock is before Unix epoch: {error}"),
            )
        })
}
