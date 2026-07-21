use std::path::PathBuf;

use anyhow::Result;
use clap::{Parser, Subcommand};

use adl_characterization::{capture_corpus, load_corpus, verify_corpus};

#[derive(Debug, Parser)]
#[command(
    name = "adl-characterize",
    version,
    about = "Independent ADL v1 characterization harness"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Capture {
        #[arg(long)]
        binary: PathBuf,
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
    },
    Verify {
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
        #[arg(long)]
        report: Option<PathBuf>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let report = match cli.command {
        Command::Capture {
            binary,
            corpus,
            observations,
            report,
        } => {
            let manifest = load_corpus(&corpus)?;
            let result = capture_corpus(&binary, &corpus, &manifest, &observations)?;
            write_report(report.as_ref(), &result)?;
            result
        }
        Command::Verify {
            corpus,
            observations,
            report,
        } => {
            let manifest = load_corpus(&corpus)?;
            let result = verify_corpus(&manifest, &observations)?;
            write_report(report.as_ref(), &result)?;
            result
        }
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}

fn write_report(
    path: Option<&PathBuf>,
    report: &adl_characterization::VerificationReport,
) -> Result<()> {
    if let Some(path) = path {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let mut bytes = serde_json::to_vec_pretty(report)?;
        bytes.push(b'\n');
        std::fs::write(path, bytes)?;
    }
    Ok(())
}
