use clap::Parser;
use csdlc_v2::{run_pre_switch_proof, ProofManifest};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    #[arg(long)]
    repo: PathBuf,
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    output: PathBuf,
}

fn main() {
    let args = Args::parse();
    let result = csdlc_v2::verify_installed_owner_operation(&args.repo, "run")
        .and_then(|_| fs::read(&args.manifest).map_err(Into::into))
        .and_then(|bytes| serde_json::from_slice::<ProofManifest>(&bytes).map_err(Into::into))
        .and_then(|manifest| run_pre_switch_proof(&args.repo, &manifest))
        .and_then(|evidence| {
            csdlc_v2::proof::write_evidence_atomic(&args.output, &evidence)?;
            if evidence.passed {
                Ok(evidence)
            } else {
                Err(csdlc_v2::V2Error::new(
                    csdlc_v2::ErrorCode::ValidationFailed,
                    "pre-switch proof failed",
                ))
            }
        });
    match result {
        Ok(evidence) => println!("{}", serde_json::to_string_pretty(&evidence).expect("JSON")),
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(error.code.exit_code());
        }
    }
}
