use std::fs;
use std::path::PathBuf;

use clap::Parser;
use csdlc_v2::{evaluate_deletion_eligibility, DeletionEligibilityRequest};

#[derive(Parser)]
struct Args {
    #[arg(long)]
    repo: PathBuf,
    #[arg(long)]
    request: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let result = fs::read(&args.request)
        .map_err(Into::into)
        .and_then(|bytes| {
            serde_json::from_slice::<DeletionEligibilityRequest>(&bytes).map_err(Into::into)
        })
        .and_then(|request| evaluate_deletion_eligibility(&args.repo, &request))
        .and_then(|decision| {
            csdlc_v2::eligibility::write_decision_atomic(&args.output, &decision)?;
            Ok(decision)
        });
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.code.exit_code());
        }
    }
}
