use std::{env, fs, path::PathBuf};

use csdlc_v3::{
    application::FoundationState,
    commands::local::{prepare_local_workflow, LocalPreparationRequest, WorktreeRegistration},
    commands::remote::{verify_remote_bridge_request, RemoteCommandRequest},
    commands::sprint::{parse_request as parse_sprint_request, verify_sprint_readiness},
    repository::RepositoryContext,
};
use serde::Serialize;

const ROOT_USAGE: &str =
    "usage: csdlc <command>\n\nCommands:\n  foundation --repo-root <path>\n  local --request <path> --registry <path> --registrations <path>\n  remote --repo-root <path> --request <path>\n  sprint --repo-root <path> --request <path>";
const FOUNDATION_USAGE: &str = "usage: csdlc foundation --repo-root <path>";
const LOCAL_USAGE: &str =
    "usage: csdlc local --request <path> --registry <path> --registrations <path>";
const REMOTE_USAGE: &str = "usage: csdlc remote --repo-root <path> --request <path>";
const SPRINT_USAGE: &str = "usage: csdlc sprint --repo-root <path> --request <path>";

fn main() {
    match run(env::args().skip(1).collect()) {
        Ok(output) => {
            if !output.is_empty() {
                println!("{output}");
            }
        }
        Err(error) => {
            eprintln!("csdlc: {error}");
            std::process::exit(2);
        }
    }
}

fn run(args: Vec<String>) -> Result<String, String> {
    let Some((command, rest)) = args.split_first() else {
        return Err(ROOT_USAGE.into());
    };
    match command.as_str() {
        "--help" | "-h" => Ok(ROOT_USAGE.into()),
        "foundation" => run_foundation(rest),
        "local" => run_local(rest),
        "remote" => run_remote(rest),
        "sprint" => run_sprint(rest),
        _ => Err(format!("{ROOT_USAGE}; unexpected command {command}")),
    }
}

fn run_foundation(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(FOUNDATION_USAGE.into());
    }
    let [flag, root] = args else {
        return Err(FOUNDATION_USAGE.into());
    };
    if flag != "--repo-root" {
        return Err(format!("{FOUNDATION_USAGE}; unexpected argument {flag}"));
    }
    let context =
        RepositoryContext::discover(PathBuf::from(root)).map_err(|error| error.to_string())?;
    let state = FoundationState::load(&context).map_err(|error| error.to_string())?;
    Ok(state.to_machine_json())
}

fn run_local(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(LOCAL_USAGE.into());
    }
    let args = LocalArgs::parse(args)?;
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
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn run_remote(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(REMOTE_USAGE.into());
    }
    let [root_flag, root, request_flag, request] = args else {
        return Err(REMOTE_USAGE.into());
    };
    if root_flag != "--repo-root" {
        return Err(format!("{REMOTE_USAGE}; unexpected argument {root_flag}"));
    }
    if request_flag != "--request" {
        return Err(format!(
            "{REMOTE_USAGE}; unexpected argument {request_flag}"
        ));
    }
    let request_bytes =
        fs::read(request).map_err(|error| format!("failed to read request: {error}"))?;
    let request: RemoteCommandRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("invalid remote request: {error}"))?;
    let report = verify_remote_bridge_request(&PathBuf::from(root), request)
        .map_err(|error| format!("{error:?}"))?;
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn run_sprint(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(SPRINT_USAGE.into());
    }
    let [root_flag, root, request_flag, request] = args else {
        return Err(SPRINT_USAGE.into());
    };
    if root_flag != "--repo-root" {
        return Err(format!("{SPRINT_USAGE}; unexpected argument {root_flag}"));
    }
    if request_flag != "--request" {
        return Err(format!(
            "{SPRINT_USAGE}; unexpected argument {request_flag}"
        ));
    }
    let request_bytes =
        fs::read(request).map_err(|error| format!("failed to read request: {error}"))?;
    let request = parse_sprint_request(&request_bytes).map_err(|error| format!("{error:?}"))?;
    let report = verify_sprint_readiness(&PathBuf::from(root), request)
        .map_err(|error| format!("{error:?}"))?;
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

#[derive(Debug, Serialize)]
struct LocalCommandReport<T> {
    schema: &'static str,
    read_only: bool,
    operational_authority: bool,
    result: T,
}

#[derive(Debug)]
struct LocalArgs {
    request: PathBuf,
    registry: PathBuf,
    registrations: PathBuf,
}

impl LocalArgs {
    fn parse(args: &[String]) -> Result<Self, String> {
        let mut request = None;
        let mut registry = None;
        let mut registrations = None;
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            let target = match arg.as_str() {
                "--request" => &mut request,
                "--registry" => &mut registry,
                "--registrations" => &mut registrations,
                _ => return Err(format!("{LOCAL_USAGE}; unexpected argument {arg}")),
            };
            if target.is_some() {
                return Err(format!("duplicate argument {arg}"));
            }
            *target =
                Some(PathBuf::from(iter.next().ok_or_else(|| {
                    format!("{LOCAL_USAGE}; missing value for {arg}")
                })?));
        }
        Ok(Self {
            request: request.ok_or_else(|| LOCAL_USAGE.to_string())?,
            registry: registry.ok_or_else(|| LOCAL_USAGE.to_string())?,
            registrations: registrations.ok_or_else(|| LOCAL_USAGE.to_string())?,
        })
    }
}
