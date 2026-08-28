use clap::Parser;
use csdlc_v2::{run_cutover, CutoverRequest};
use std::fs;
use std::path::PathBuf;

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
    let result = csdlc_v2::verify_installed_owner_operation(&args.repo, "run")
        .and_then(|_| fs::read(&args.request).map_err(Into::into))
        .and_then(|bytes| serde_json::from_slice::<CutoverRequest>(&bytes).map_err(Into::into))
        .and_then(|request| run_cutover(&args.repo, &request))
        .and_then(|evidence| {
            csdlc_v2::cutover::write_evidence_atomic(&args.output, &evidence)?;
            if evidence.passed {
                Ok(evidence)
            } else {
                Err(csdlc_v2::V2Error::new(
                    csdlc_v2::ErrorCode::ValidationFailed,
                    "cutover smoke failed; v1 restored",
                ))
            }
        });
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.code.exit_code());
        }
    }
}
