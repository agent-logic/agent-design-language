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
    },
    Verify {
        #[arg(long)]
        corpus: PathBuf,
        #[arg(long)]
        observations: PathBuf,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let report = match cli.command {
        Command::Capture {
            binary,
            corpus,
            observations,
        } => {
            let manifest = load_corpus(&corpus)?;
            capture_corpus(&binary, &corpus, &manifest, &observations)?
        }
        Command::Verify {
            corpus,
            observations,
        } => {
            let manifest = load_corpus(&corpus)?;
            verify_corpus(&manifest, &observations)?
        }
    };
    println!("{}", serde_json::to_string_pretty(&report)?);
    Ok(())
}
