use clap::{Parser, Subcommand};
use csdlc_v2::{
    classify_preserved_projection, initialize_native_json, migrate_bound_issue_identity,
    migrate_bound_topology, migrate_code_repository, migrate_initialized_code_repository,
    recover_preserved_projection, BoundIssueIdentityMigrationRequest,
    BoundTopologyMigrationRequest, CodeRepositoryMigrationRequest,
    InitializedCodeRepositoryMigrationRequest, ProjectionClassifyRequest, ProjectionRecoverRequest,
    Store,
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
    MigrateInitializedCodeRepository {
        #[arg(long)]
        request: PathBuf,
    },
    MigrateBoundIssueIdentity {
        #[arg(long)]
        request: PathBuf,
    },
    ClassifyPreservedProjection {
        #[arg(long)]
        request: PathBuf,
    },
    RecoverPreservedProjection {
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
        Command::MigrateInitializedCodeRepository { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<InitializedCodeRepositoryMigrationRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| migrate_initialized_code_repository(&Store::new(cli.root), request))
            .and_then(|report| serde_json::to_value(report).map_err(csdlc_v2::V2Error::from)),
        Command::MigrateBoundIssueIdentity { request } => fs::read(request)
            .map_err(csdlc_v2::V2Error::from)
            .and_then(|bytes| {
                serde_json::from_slice::<BoundIssueIdentityMigrationRequest>(&bytes)
                    .map_err(csdlc_v2::V2Error::from)
            })
            .and_then(|request| migrate_bound_issue_identity(&Store::new(cli.root), request))
            .and_then(|report| serde_json::to_value(report).map_err(csdlc_v2::V2Error::from)),
        Command::ClassifyPreservedProjection { request } => {
            read::<ProjectionClassifyRequest>(&request)
                .and_then(|request| classify_preserved_projection(&Store::new(cli.root), request))
                .and_then(|value| serde_json::to_value(value).map_err(csdlc_v2::V2Error::from))
        }
        Command::RecoverPreservedProjection { request } => {
            read::<ProjectionRecoverRequest>(&request)
                .and_then(|request| recover_preserved_projection(&Store::new(cli.root), request))
                .and_then(|value| serde_json::to_value(value).map_err(csdlc_v2::V2Error::from))
        }
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

fn read<T: serde::de::DeserializeOwned>(path: &PathBuf) -> csdlc_v2::Result<T> {
    Ok(serde_json::from_slice(&fs::read(path)?)?)
}
