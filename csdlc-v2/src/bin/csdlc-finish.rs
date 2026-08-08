use std::fs;
use std::path::PathBuf;

use clap::Parser;
use csdlc_v2::finish::{
    envelope_matches_record, execute_finish, execute_historical_finish, load_cached_terminal,
    FinishRequest, HistoricalFinishRequest,
};
use csdlc_v2::{ErrorCode, Store, V2Error};

#[derive(Parser)]
#[command(about = "Finish one C-SDLC v2 issue from exact live GitHub terminal truth")]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[arg(
        long,
        conflicts_with_all = ["validate_cached_issue", "historical_request"],
        required_unless_present_any = ["validate_cached_issue", "historical_request"]
    )]
    request: Option<PathBuf>,
    #[arg(
        long,
        conflicts_with_all = ["request", "historical_request"],
        required_unless_present_any = ["request", "historical_request"]
    )]
    validate_cached_issue: Option<u64>,
    #[arg(
        long,
        conflicts_with_all = ["request", "validate_cached_issue"],
        required_unless_present_any = ["request", "validate_cached_issue"]
    )]
    historical_request: Option<PathBuf>,
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

async fn run(cli: Cli) -> csdlc_v2::Result<serde_json::Value> {
    if let Some(issue) = cli.validate_cached_issue {
        let store = Store::new(&cli.root);
        let record = store.load_record(issue)?;
        let terminal = load_cached_terminal(&cli.root, issue)?.ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal cache is missing",
            )
        })?;
        if !envelope_matches_record(&terminal, &record)? {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "derived terminal envelope does not match canonical issue truth",
            ));
        }
        return Ok(serde_json::json!({
            "schema": "csdlc.derived_terminal_validation.v1",
            "canonical_match": true,
            "terminal": terminal,
        }));
    }
    if let Some(request) = cli.historical_request {
        let request: HistoricalFinishRequest = serde_json::from_slice(&fs::read(request)?)?;
        return serde_json::to_value(execute_historical_finish(&cli.root, &request).await?)
            .map_err(Into::into);
    }
    let request = cli.request.expect("clap requires finish request");
    let request: FinishRequest = serde_json::from_slice(&fs::read(request)?)?;
    serde_json::to_value(execute_finish(&cli.root, &request).await?).map_err(Into::into)
}
