use std::{env, fs, path::PathBuf};

use csdlc_v3::{
    application::FoundationState,
    commands::local::{prepare_local_workflow, LocalPreparationRequest, WorktreeRegistration},
    commands::remote::{
        load_remote_route_receipts, prepare_remote_publication_route_with_receipts,
        RemoteRouteRequest, REMOTE_PUBLICATION_ROUTE_NAMES,
    },
    repository::RepositoryContext,
};
use serde::Serialize;

const ROOT_USAGE: &str =
    "usage: csdlc <command>\n\nCommands:\n  foundation --repo-root <path>\n  local --request <path> --registry <path> --registrations <path>\n  bind --help\n  clean --help\n  cutover --help\n  doctor --help\n  edit --help\n  eligibility --help\n  finish --help\n  github --request <path>\n  github-issue --request <path>\n  github-pr --request <path>\n  install --help\n  issue --help\n  pr-state --request <path>\n  proof --help\n  publish --request <path>\n  review --request <path>\n  schedule --help\n  shadow --help\n  shepherd --help\n  soak --help\n  validate --help";
const FOUNDATION_USAGE: &str = "usage: csdlc foundation --repo-root <path>";
const LOCAL_USAGE: &str =
    "usage: csdlc local --request <path> --registry <path> --registrations <path>";
const REMOTE_USAGE: &str =
    "usage: csdlc <github|github-issue|github-pr|pr-state|publish|review> --request <path>";

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
        route if REMOTE_PUBLICATION_ROUTE_NAMES.contains(&route) => run_remote(route, rest),
        "bind" | "clean" | "cutover" | "doctor" | "edit" | "eligibility" | "finish" | "install"
        | "issue" | "proof" | "schedule" | "shepherd" | "soak" => {
            if rest == ["--help"] || rest == ["-h"] {
                return Ok(reserved_usage(command, "fail_closed"));
            }
            Err(format!(
                "fail_closed: csdlc {command} is reserved for C-SDLC v3 replacement work and is not implemented as live authority in #627. C-SDLC v3 is not live authority before #505 cutover."
            ))
        }
        "shadow" | "validate" => {
            if rest == ["--help"] || rest == ["-h"] {
                return Ok(reserved_usage(command, "partial"));
            }
            Err(format!(
                "partial: csdlc {command} has construction evidence only and is not implemented as live authority in #627. C-SDLC v3 is not live authority before #505 cutover."
            ))
        }
        _ => Err(format!("{ROOT_USAGE}; unexpected command {command}")),
    }
}

fn reserved_usage(command: &str, status: &str) -> String {
    format!(
        "usage: csdlc {command} [--help]\n\nstatus: {status}\nauthority: C-SDLC v3 is not live authority before #505 cutover."
    )
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

fn run_remote(command: &str, args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(remote_usage(command));
    }
    let args = RemoteArgs::parse(command, args)?;
    let request_bytes =
        fs::read(&args.request).map_err(|error| format!("failed to read request: {error}"))?;
    let request: RemoteRouteRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("typed_remote_request_invalid_json: {error}"))?;
    let repo_root = discover_repo_root(env::current_dir().map_err(|error| error.to_string())?)
        .ok_or_else(|| "repository_root_unavailable: could not find containing .git".to_string())?;
    let receipts = load_remote_route_receipts(&repo_root, &request)
        .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
    let result = prepare_remote_publication_route_with_receipts(command, &request, &receipts)
        .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
    let report = RemoteCommandReport {
        schema: "csdlc.v3.remote_publication.v1",
        command: command.to_owned(),
        read_only: true,
        operational_authority: false,
        cutover_issue: 505,
        result,
    };
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn discover_repo_root(start: PathBuf) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join(".git").exists() {
            return Some(candidate.to_path_buf());
        }
    }
    None
}

fn remote_usage(command: &str) -> String {
    format!(
        "usage: csdlc {command} --request <path>\n\nstatus: implemented\nauthority: C-SDLC v3 is not live authority before #505 cutover."
    )
}

#[derive(Debug, Serialize)]
struct LocalCommandReport<T> {
    schema: &'static str,
    read_only: bool,
    operational_authority: bool,
    result: T,
}

#[derive(Debug, Serialize)]
struct RemoteCommandReport<T> {
    schema: &'static str,
    command: String,
    read_only: bool,
    operational_authority: bool,
    cutover_issue: u64,
    result: T,
}

#[derive(Debug)]
struct LocalArgs {
    request: PathBuf,
    registry: PathBuf,
    registrations: PathBuf,
}

#[derive(Debug)]
struct RemoteArgs {
    request: PathBuf,
}

impl RemoteArgs {
    fn parse(command: &str, args: &[String]) -> Result<Self, String> {
        let mut request = None;
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--request" => {
                    if request.is_some() {
                        return Err("duplicate argument --request".into());
                    }
                    request = Some(PathBuf::from(iter.next().ok_or_else(|| {
                        format!("{}; missing value for --request", remote_usage(command))
                    })?));
                }
                _ => {
                    return Err(format!(
                        "{}; unexpected argument {arg}",
                        remote_usage(command)
                    ))
                }
            }
        }
        Ok(Self {
            request: request.ok_or_else(|| REMOTE_USAGE.to_string())?,
        })
    }
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
