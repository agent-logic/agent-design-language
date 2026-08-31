use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    github::{collect_pr_state, PrStateRequest},
    public_schema_bundle, write_json_stdout, ErrorCode, GithubAction, GithubActionRequest, V2Error,
};

#[derive(Parser)]
#[command(name = "csdlc-github-pr")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    State {
        #[arg(long)]
        request: PathBuf,
    },
    Run {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::State { request } => state(&request).await,
        Command::Run { request } => run(&request).await,
        Command::Schema => Ok(public_schema_bundle()),
    };
    match result {
        Ok(value) => {
            if let Err(error) = write_json_stdout(&value, true) {
                eprintln!("csdlc-github-pr: failed writing stdout: {}", error.message);
                std::process::exit(error.code.exit_code());
            }
        }
        Err(error) => {
            let exit_code = error.code.exit_code();
            let payload = serde_json::json!({
                "schema": "csdlc.error.v1",
                "code": error.code.to_string(),
                "message": error.message
            });
            if let Err(output_error) = write_json_stdout(&payload, true) {
                eprintln!(
                    "csdlc-github-pr: failed writing error payload: {}",
                    output_error.message
                );
            }
            std::process::exit(exit_code);
        }
    }
}

async fn state(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: PrStateRequest = serde_json::from_slice(&fs::read(path)?)?;
    serde_json::to_value(collect_pr_state(&request).await?).map_err(Into::into)
}

async fn run(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: GithubActionRequest = serde_json::from_slice(&fs::read(path)?)?;
    if !matches!(
        request.action,
        GithubAction::PrState | GithubAction::PrUpdate
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "csdlc-github-pr only accepts pr_state or pr_update actions; use csdlc-github-issue for issue actions",
        ));
    }
    if matches!(request.action, GithubAction::PrUpdate) {
        return serde_json::to_value(csdlc_v2::execute_github_action(&request).await?)
            .map_err(Into::into);
    }
    let pr_request = PrStateRequest::try_from(&request)?;
    serde_json::to_value(collect_pr_state(&pr_request).await?).map_err(Into::into)
}
