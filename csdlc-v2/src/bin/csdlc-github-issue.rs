use std::{fs, path::PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::{
    execute_github_action, public_schema_bundle, verify_installed_owner_preflight,
    write_json_stdout, ErrorCode, GithubAction, GithubActionRequest, V2Error,
};

#[derive(Parser)]
#[command(name = "csdlc-github-issue")]
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
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Run { request } => run(&request).await,
        Command::Schema => Ok(public_schema_bundle()),
    };
    match result {
        Ok(value) => {
            if let Err(error) = write_json_stdout(&value, true) {
                eprintln!(
                    "csdlc-github-issue: failed writing stdout: {}",
                    error.message
                );
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
                    "csdlc-github-issue: failed writing error payload: {}",
                    output_error.message
                );
            }
            std::process::exit(exit_code);
        }
    }
}

async fn run(path: &PathBuf) -> csdlc_v2::Result<serde_json::Value> {
    let request: GithubActionRequest = serde_json::from_slice(&fs::read(path)?)?;
    if matches!(request.action, GithubAction::PrState) {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "csdlc-github-issue only accepts issue actions; use csdlc-github-pr for PR state",
        ));
    }
    if !matches!(request.action, GithubAction::IssueRead) {
        verify_installed_owner_preflight(&std::env::current_dir()?)?;
    }
    serde_json::to_value(execute_github_action(&request).await?).map_err(Into::into)
}
