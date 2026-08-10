use clap::{Parser, Subcommand};
use csdlc_v2::{
    initialize_native_json, migrate_bound_topology, migrate_code_repository,
    BoundTopologyMigrationRequest, CodeRepositoryMigrationRequest, Store,
};
use std::{fs, path::PathBuf};

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Create {
        #[arg(long)]
        request: PathBuf,
    },
    MigrateBoundTopology {
        #[arg(long)]
        request: PathBuf,
    },
    MigrateCodeRepository {
        #[arg(long)]
        request: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();
    let result = match cli.command {
        Command::Create { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| initialize_native_json(&Store::new(cli.root), &bytes))
            .and_then(|record| serde_json::to_value(record).map_err(csdlc_v2::V2Error::from)),
        Command::MigrateBoundTopology { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<BoundTopologyMigrationRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| migrate_bound_topology(&Store::new(cli.root), request))
            .and_then(|report| serde_json::to_value(report).map_err(csdlc_v2::V2Error::from)),
        Command::MigrateCodeRepository { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<CodeRepositoryMigrationRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| migrate_code_repository(&Store::new(cli.root), request))
            .and_then(|report| serde_json::to_value(report).map_err(csdlc_v2::V2Error::from)),
    };
    match result {
        Ok(value) => println!("{}", serde_json::to_string(&value).expect("JSON")),
        Err(error) => {
            eprintln!("csdlc-issue: {}", error);
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code,"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}
