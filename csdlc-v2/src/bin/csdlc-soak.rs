use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use csdlc_v2::{decide_from_evidence, generate_sample_packets, SoakEvidenceInput};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    GenerateSamples {
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    Decide {
        #[arg(long, default_value = ".")]
        repo: PathBuf,
        #[arg(long)]
        evidence: PathBuf,
        #[arg(long)]
        output: Option<PathBuf>,
    },
    Schema,
}

fn main() {
    let result: csdlc_v2::Result<serde_json::Value> = match Cli::parse().command {
        Command::GenerateSamples { repo, output } => {
            csdlc_v2::verify_installed_owner_operation(&repo, "generate-samples")
                .and_then(|_| generate_sample_packets(&repo, &output))
                .and_then(|packets| serde_json::to_value(packets).map_err(Into::into))
        }
        Command::Decide {
            repo,
            evidence,
            output,
        } => (|| {
            if output.is_some() {
                csdlc_v2::verify_installed_owner_operation(&repo, "decide-with-output")?;
            }
            fs::read(evidence)
                .map_err(Into::into)
                .and_then(|bytes| {
                    serde_json::from_slice::<SoakEvidenceInput>(&bytes).map_err(Into::into)
                })
                .and_then(decide_from_evidence)
                .and_then(|packet| serde_json::to_value(packet).map_err(Into::into))
                .and_then(|value| {
                    if let Some(path) = output {
                        let mut bytes = serde_json::to_vec_pretty(&value)?;
                        bytes.push(b'\n');
                        fs::write(path, bytes)?;
                    }
                    Ok(value)
                })
        })(),
        Command::Schema => Ok(serde_json::json!({
            "generation_selector": schemars::schema_for!(csdlc_v2::GenerationSelector),
            "scenario_evidence": schemars::schema_for!(csdlc_v2::ScenarioEvidence),
            "budget_evidence": schemars::schema_for!(csdlc_v2::BudgetEvidence),
            "soak_evidence": schemars::schema_for!(csdlc_v2::SoakEvidenceInput),
            "decision_packet": schemars::schema_for!(csdlc_v2::SoakDecisionPacket),
        })),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-soak: {error}");
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
