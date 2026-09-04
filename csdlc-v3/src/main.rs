use std::{env, fs, path::PathBuf};

use csdlc_v3::{
    application::FoundationState,
    commands::local::{
        execute_local_route, finding, initialize_v3_local_state, inspect_local_lifecycle_state,
        inspect_v3_local_state, local_route_command, local_route_status, prepare_local_workflow,
        LocalPreparationRequest, PlanStatus, WorktreeRegistration, LOCAL_ROUTE_NAMES,
    },
    commands::remote::{
        verify_remote_bridge_request, RemoteCommandOperation, RemoteCommandRequest,
    },
    commands::replacement::{
        command_from_route, verify_replacement_request, ReplacementVerifierRequest,
    },
    commands::sprint::{parse_request as parse_sprint_request, verify_sprint_readiness},
    repository::RepositoryContext,
};
use serde::Serialize;

const ROOT_USAGE: &str =
    "usage: csdlc <command>\n\nCommands:\n  foundation --repo-root <path>\n  local --request <path> --registry <path> --registrations <path>\n  bind --request <path> --registry <path> --registrations <path>\n  clean --repo-root <path> --request <path>\n  cutover --repo-root <path> --request <path>\n  doctor --request <path> --registry <path> --registrations <path>\n  edit --request <path> --registry <path> --registrations <path>\n  eligibility --request <path> --registry <path> --registrations <path>\n  finish --repo-root <path> --request <path>\n  github --repo-root <path> --request <path>\n  github-issue --repo-root <path> --request <path>\n  github-pr --repo-root <path> --request <path>\n  install --repo-root <path> --request <path>\n  issue --request <path> --registry <path> --registrations <path>\n  pr-state --repo-root <path> --request <path>\n  proof --repo-root <path> --request <path>\n  publish --repo-root <path> --request <path>\n  review --repo-root <path> --request <path>\n  remote --repo-root <path> --request <path>\n  schedule --request <path> --registry <path> --registrations <path>\n  shadow --repo-root <path> --request <path>\n  shepherd --request <path> --registry <path> --registrations <path>\n  soak --repo-root <path> --request <path>\n  sprint --repo-root <path> --request <path>\n  validate --request <path> --registry <path> --registrations <path>";
const FOUNDATION_USAGE: &str = "usage: csdlc foundation --repo-root <path>";
const LOCAL_USAGE: &str =
    "usage: csdlc local --request <path> --registry <path> --registrations <path>";
const REMOTE_USAGE: &str = "usage: csdlc remote --repo-root <path> --request <path>";
const REPLACEMENT_USAGE: &str =
    "usage: csdlc <cutover|install|proof|shadow|soak> --repo-root <path> --request <path>";
const SPRINT_USAGE: &str = "usage: csdlc sprint --repo-root <path> --request <path>";
const REMOTE_BRIDGE_ROUTE_NAMES: &[&str] = &[
    "github",
    "github-issue",
    "github-pr",
    "pr-state",
    "publish",
    "review",
    "finish",
    "clean",
];

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
        route if LOCAL_ROUTE_NAMES.contains(&route) => run_local_route(route, rest),
        route if REMOTE_BRIDGE_ROUTE_NAMES.contains(&route) => run_remote_bridge_route(route, rest),
        route if command_from_route(route).is_some() => run_replacement_route(route, rest),
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
    run_local_report("local", args)
}

fn run_local_route(route: &str, args: &[String]) -> Result<String, String> {
    let usage =
        format!("usage: csdlc {route} --request <path> --registry <path> --registrations <path>");
    if args == ["--help"] || args == ["-h"] {
        return Ok(format!(
            "{usage}\n\nstatus: implemented\nauthority: C-SDLC v3 is not live authority before #505 cutover."
        ));
    }
    run_local_report(route, args)
}

fn run_local_report(route: &str, args: &[String]) -> Result<String, String> {
    let args = LocalArgs::parse(args, route)?;
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
    if let Some(command) = local_route_command(route) {
        if !request.commands.contains(&command) {
            return Err(format!(
                "local route {route} is not present in the typed request"
            ));
        }
    }
    let mut result = prepare_local_workflow(&request, &registry, &registrations)
        .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?;
    let observed_v3_issue_state = match (route, args.v3_state_root.as_ref()) {
        ("issue", Some(root)) => Some(inspect_v3_local_state(root, request.issue)),
        _ => None,
    };
    if let Some(observed) = observed_v3_issue_state.as_ref() {
        if request.expected_lifecycle_digest.is_none()
            && observed.code != "missing_local_lifecycle_state"
        {
            return Err(
                serde_json::to_string(&vec![finding(
                    PlanStatus::Blocked,
                    "v3_local_state_digest_required",
                    "existing v3 local state requires an expected lifecycle digest before the issue route may write",
                )])
                .unwrap_or_else(|_| "[]".into()),
            );
        }
    }
    let prechecked_issue_route_result = if route == "issue"
        && request.expected_lifecycle_digest.is_some()
    {
        Some(
            execute_local_route(
                route,
                &request,
                &registry,
                &registrations,
                observed_v3_issue_state.clone(),
            )
            .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?,
        )
    } else {
        None
    };
    result.lifecycle_state = match (route, args.v3_state_root.as_ref(), args.repo_root.as_ref()) {
        ("issue", Some(root), _) => Some(
            initialize_v3_local_state(root, &request, &registry).map_err(|findings| {
                serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into())
            })?,
        ),
        ("eligibility", Some(root), _) => Some(inspect_v3_local_state(root, request.issue)),
        (_, _, Some(root)) => Some(inspect_local_lifecycle_state(root, request.issue)),
        _ => None,
    };
    let route_status = local_route_status(route, result.lifecycle_state.as_ref());
    let writes_v3_state = route == "issue" && args.v3_state_root.is_some();
    let route_result = if route == "local" {
        None
    } else if let Some(route_result) = prechecked_issue_route_result {
        Some(route_result)
    } else {
        Some(
            execute_local_route(
                route,
                &request,
                &registry,
                &registrations,
                result.lifecycle_state.clone(),
            )
            .map_err(|findings| serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()))?,
        )
    };
    let report = LocalCommandReport {
        schema: "csdlc.v3.local_preparation.v1",
        command: route.to_owned(),
        read_only: !writes_v3_state,
        operational_read_only: true,
        operational_authority: false,
        writes_v3_state,
        route_status,
        route_result,
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

fn run_remote_bridge_route(route: &str, args: &[String]) -> Result<String, String> {
    let usage = format!("usage: csdlc {route} --repo-root <path> --request <path>");
    if args == ["--help"] || args == ["-h"] {
        return Ok(format!(
            "{usage}\n\nstatus: implemented\nauthority: C-SDLC v3 is not live authority before #505 cutover.\ntransport: structured pre-cutover bridge evidence only."
        ));
    }
    let expected = expected_remote_operation(route);
    let [root_flag, root, request_flag, request] = args else {
        return Err(usage);
    };
    if root_flag != "--repo-root" {
        return Err(format!("{usage}; unexpected argument {root_flag}"));
    }
    if request_flag != "--request" {
        return Err(format!("{usage}; unexpected argument {request_flag}"));
    }
    let request_bytes =
        fs::read(request).map_err(|error| format!("failed to read request: {error}"))?;
    let request: RemoteCommandRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("invalid remote request: {error}"))?;
    if request.operation != expected {
        return Err(format!(
            "route_operation_mismatch: csdlc {route} requires request operation {expected:?}, got {:?}",
            request.operation
        ));
    }
    let report = verify_remote_bridge_request(&PathBuf::from(root), request)
        .map_err(|error| format!("{error:?}"))?;
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn run_replacement_route(route: &str, args: &[String]) -> Result<String, String> {
    let usage = format!("usage: csdlc {route} --repo-root <path> --request <path>");
    if args == ["--help"] || args == ["-h"] {
        return Ok(format!(
            "{usage}\n\nstatus: implemented_pre_cutover_verifier\nauthority: C-SDLC v3 is not live authority before #505 cutover.\nmutation: disabled until explicit operator approval."
        ));
    }
    let Some(command) = command_from_route(route) else {
        return Err(REPLACEMENT_USAGE.into());
    };
    let [root_flag, root, request_flag, request] = args else {
        return Err(usage);
    };
    if root_flag != "--repo-root" {
        return Err(format!("{usage}; unexpected argument {root_flag}"));
    }
    if request_flag != "--request" {
        return Err(format!("{usage}; unexpected argument {request_flag}"));
    }
    let request_bytes =
        fs::read(request).map_err(|error| format!("failed to read request: {error}"))?;
    let request: ReplacementVerifierRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("invalid replacement request: {error}"))?;
    let report = verify_replacement_request(&PathBuf::from(root), command, request)
        .map_err(|error| format!("{}: {}", error.code, error.message))?;
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn expected_remote_operation(route: &str) -> RemoteCommandOperation {
    match route {
        "finish" => RemoteCommandOperation::Finish,
        "clean" => RemoteCommandOperation::CleanupPreview,
        "publish" => RemoteCommandOperation::Publish,
        "github" | "github-issue" | "github-pr" | "pr-state" | "review" => {
            RemoteCommandOperation::VerifyBridgeEvidence
        }
        _ => RemoteCommandOperation::VerifyBridgeEvidence,
    }
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
    command: String,
    read_only: bool,
    operational_read_only: bool,
    operational_authority: bool,
    writes_v3_state: bool,
    route_status: Option<csdlc_v3::commands::local::LocalRouteStatus>,
    route_result: Option<csdlc_v3::commands::local::LocalRouteResult>,
    result: T,
}

#[derive(Debug)]
struct LocalArgs {
    request: PathBuf,
    registry: PathBuf,
    registrations: PathBuf,
    repo_root: Option<PathBuf>,
    v3_state_root: Option<PathBuf>,
}

impl LocalArgs {
    fn parse(args: &[String], route: &str) -> Result<Self, String> {
        let usage = format!(
            "usage: csdlc {route} --request <path> --registry <path> --registrations <path>"
        );
        let mut request = None;
        let mut registry = None;
        let mut registrations = None;
        let mut repo_root = None;
        let mut v3_state_root = None;
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            let target = match arg.as_str() {
                "--request" => &mut request,
                "--registry" => &mut registry,
                "--registrations" => &mut registrations,
                "--repo-root" => &mut repo_root,
                "--v3-state-root" => &mut v3_state_root,
                _ => return Err(format!("{usage}; unexpected argument {arg}")),
            };
            if target.is_some() {
                return Err(format!("duplicate argument {arg}"));
            }
            *target =
                Some(PathBuf::from(iter.next().ok_or_else(|| {
                    format!("{usage}; missing value for {arg}")
                })?));
        }
        Ok(Self {
            request: request.ok_or_else(|| usage.clone())?,
            registry: registry.ok_or_else(|| usage.clone())?,
            registrations: registrations.ok_or(usage)?,
            repo_root,
            v3_state_root,
        })
    }
}
