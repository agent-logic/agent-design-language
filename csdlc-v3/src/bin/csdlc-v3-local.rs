use std::{env, fs, path::PathBuf};

use csdlc_v3::commands::local::{
    prepare_local_workflow, LocalPreparationRequest, WorktreeRegistration,
};
use serde::Serialize;

#[derive(Debug, Serialize)]
struct LocalCommandReport<T> {
    schema: &'static str,
    read_only: bool,
    operational_authority: bool,
    result: T,
}

fn main() {
    match run() {
        Ok(()) => {}
        Err(error) => {
            eprintln!("csdlc-v3-local: {error}");
            std::process::exit(2);
        }
    }
}

fn run() -> Result<(), String> {
    let args = Args::parse(env::args().skip(1))?;
    let request_bytes =
        fs::read(&args.request).map_err(|error| format!("failed to read request: {error}"))?;
    let registry_bytes =
        fs::read(&args.registry).map_err(|error| format!("failed to read registry: {error}"))?;
    let registrations_bytes = fs::read(&args.registrations)
        .map_err(|error| format!("failed to read registrations: {error}"))?;

    let request = LocalPreparationRequest::from_json(&request_bytes)
        .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?;
    let registry = csdlc_v3::commands::local::PromptRegistry::from_current_json(&registry_bytes)
        .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?;
    let registrations: Vec<WorktreeRegistration> = serde_json::from_slice(&registrations_bytes)
        .map_err(|error| format!("invalid registrations json: {error}"))?;
    let result = prepare_local_workflow(&request, &registry, &registrations)
        .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?;
    let report = LocalCommandReport {
        schema: "csdlc.v3.local_preparation.v1",
        read_only: true,
        operational_authority: false,
        result,
    };
    println!(
        "{}",
        serde_json::to_string(&report).map_err(|error| error.to_string())?
    );
    Ok(())
}

#[derive(Debug)]
struct Args {
    request: PathBuf,
    registry: PathBuf,
    registrations: PathBuf,
}

impl Args {
    fn parse<I>(mut args: I) -> Result<Self, String>
    where
        I: Iterator<Item = String>,
    {
        let mut request = None;
        let mut registry = None;
        let mut registrations = None;
        while let Some(arg) = args.next() {
            let target = match arg.as_str() {
                "--request" => &mut request,
                "--registry" => &mut registry,
                "--registrations" => &mut registrations,
                _ => {
                    return Err(format!(
                        "usage: csdlc-v3-local --request <path> --registry <path> --registrations <path>; unexpected argument {arg}"
                    ));
                }
            };
            if target.is_some() {
                return Err(format!("duplicate argument {arg}"));
            }
            *target = Some(PathBuf::from(args.next().ok_or_else(|| {
                format!(
                    "usage: csdlc-v3-local --request <path> --registry <path> --registrations <path>; missing value for {arg}"
                )
            })?));
        }
        Ok(Self {
            request: request.ok_or_else(usage)?,
            registry: registry.ok_or_else(usage)?,
            registrations: registrations.ok_or_else(usage)?,
        })
    }
}

fn usage() -> String {
    "usage: csdlc-v3-local --request <path> --registry <path> --registrations <path>".into()
}
