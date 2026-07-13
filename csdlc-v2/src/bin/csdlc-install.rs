use clap::{Parser, Subcommand};
use csdlc_v2::{install_binaries, verify_coexistence, CoexistenceInventory, SkillManifest};
use std::fs;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "csdlc-install")]
struct Args {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Manifest,
    Install {
        #[arg(long)]
        source: PathBuf,
        #[arg(long)]
        destination: PathBuf,
    },
    Verify {
        #[arg(long)]
        repo: PathBuf,
        #[arg(long)]
        bin_dir: PathBuf,
        #[arg(long)]
        inventory: PathBuf,
    },
}
fn main() {
    let result = match Args::parse().command {
        Command::Manifest => SkillManifest::load().and_then(json),
        Command::Install {
            source,
            destination,
        } => install_binaries(&source, &destination).and_then(json),
        Command::Verify {
            repo,
            bin_dir,
            inventory,
        } => fs::read(inventory)
            .map_err(io_error)
            .and_then(|b| {
                serde_json::from_slice::<CoexistenceInventory>(&b).map_err(|e| {
                    csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::CorruptRecord, e.to_string())
                })
            })
            .and_then(|v| verify_coexistence(&repo, &bin_dir, &v))
            .and_then(|r| {
                let pass = r.pass;
                json(r)?;
                if pass {
                    Ok(())
                } else {
                    Err(csdlc_v2::V2Error::new(
                        csdlc_v2::ErrorCode::ValidationFailed,
                        "coexistence proof failed",
                    ))
                }
            }),
    };
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(2);
    }
}
fn json(value: impl serde::Serialize) -> csdlc_v2::Result<()> {
    println!(
        "{}",
        serde_json::to_string_pretty(&value)
            .map_err(|e| csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::Io, e.to_string()))?
    );
    Ok(())
}
fn io_error(error: std::io::Error) -> csdlc_v2::V2Error {
    csdlc_v2::V2Error::new(csdlc_v2::ErrorCode::Io, error.to_string())
}
