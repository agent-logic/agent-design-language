use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    execute_github_action, inspect_runner_eligibility, public_schema_bundle,
    verify_installed_owner_operation, GithubAction, GithubActionRequest, RunnerPreflightRequest,
};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Run {
        #[arg(long)]
        request: PathBuf,
    },
    RunnerPreflight {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Run { request } => run(&request).await,
        Command::RunnerPreflight { request } => match runner_preflight(&request).await {
            Ok(packet) => {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&packet).expect("runner preflight JSON")
                );
                if !packet.is_dispatch_eligible() {
                    std::process::exit(2);
                }
                return;
            }
            Err(error) => Err(error),
        },
        Command::Schema => Ok(public_schema_bundle()),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({
                    "schema": "csdlc.error.v1",
                    "code": error.code.to_string(),
                    "message": error.message
                })
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn runner_preflight(path: &PathBuf) -> csdlc_v2::Result<csdlc_v2::RunnerPreflightPacket> {
    let request: RunnerPreflightRequest = serde_json::from_slice(&fs::read(path)?)?;
    inspect_runner_eligibility(&request).await
}

async fn run(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: GithubActionRequest = serde_json::from_slice(&fs::read(path)?)?;
    let operation = if matches!(
        request.action,
        GithubAction::IssueRead | GithubAction::PrState
    ) {
        "run-read"
    } else {
        "run-write"
    };
    verify_installed_owner_operation(&std::env::current_dir()?, operation)?;
    serde_json::to_value(execute_github_action(&request).await?).map_err(Into::into)
}
