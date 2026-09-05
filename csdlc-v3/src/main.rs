use std::{env, fs, path::PathBuf};

use csdlc_v3::{
    adapters::{EnvironmentCredentialResolver, RealProcessAdapter},
    application::FoundationState,
    commands::local::{
        execute_local_route, finding, initialize_v3_local_state, inspect_local_lifecycle_state,
        inspect_v3_local_state, local_route_command, local_route_status, prepare_local_workflow,
        LocalPreparationRequest, PlanStatus, WorktreeRegistration, LOCAL_ROUTE_NAMES,
    },
    commands::proof::{classify_route, ProofRouteRequest, PROOF_ROUTE_NAMES},
    commands::remote::{
        load_remote_route_receipts, observe_github_pr_readback,
        prepare_remote_publication_route_with_receipts, RemoteRouteReceipts, RemoteRouteRequest,
        REMOTE_PUBLICATION_ROUTE_NAMES,
    },
    commands::terminal::{prepare_terminal_route, TerminalRouteRequest, TERMINAL_ROUTE_NAMES},
    repository::RepositoryContext,
};
use serde::Serialize;

const ROOT_USAGE: &str =
    "usage: csdlc <command>\n\nCommands:\n  foundation --repo-root <path>\n  local --request <path> --registry <path> --registrations <path>\n  bind --request <path> --registry <path> --registrations <path>\n  clean --request <path>\n  cutover --request <path>\n  doctor --request <path> --registry <path> --registrations <path>\n  edit --request <path> --registry <path> --registrations <path>\n  eligibility --request <path> --registry <path> --registrations <path>\n  finish --request <path>\n  github --request <path> [--observe-github]\n  github-issue --request <path> [--observe-github]\n  github-pr --request <path> [--observe-github]\n  install --request <path>\n  issue --request <path> --registry <path> --registrations <path>\n  pr-state --request <path> [--observe-github]\n  proof --request <path>\n  publish --request <path> [--observe-github]\n  review --request <path>\n  schedule --request <path> --registry <path> --registrations <path>\n  shadow --request <path>\n  shepherd --request <path> --registry <path> --registrations <path>\n  soak --request <path>\n  validate --request <path> --registry <path> --registrations <path>";
const FOUNDATION_USAGE: &str = "usage: csdlc foundation --repo-root <path>";
const LOCAL_USAGE: &str =
    "usage: csdlc local --request <path> --registry <path> --registrations <path>";
const REMOTE_USAGE: &str =
    "usage: csdlc <github|github-issue|github-pr|pr-state|publish|review> --request <path> [--observe-github]";
const TERMINAL_USAGE: &str = "usage: csdlc <finish|clean|cutover> --request <path>";

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
        route if PROOF_ROUTE_NAMES.contains(&route) => run_proof_route(route, rest),
        route if LOCAL_ROUTE_NAMES.contains(&route) => run_local_route(route, rest),
        route if REMOTE_PUBLICATION_ROUTE_NAMES.contains(&route) => run_remote(route, rest),
        route if TERMINAL_ROUTE_NAMES.contains(&route) => run_terminal(route, rest),
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

fn run_proof_route(command: &str, args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(format!(
            "usage: csdlc {command} --request <path>\n\nstatus: implemented_construction\nauthority: C-SDLC v3 is not live authority before #505 cutover."
        ));
    }
    let request_path = RequestOnlyArgs::parse(command, args)?.request;
    let request_bytes =
        fs::read(&request_path).map_err(|error| format!("failed to read request: {error}"))?;
    let request: ProofRouteRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("invalid request json: {error}"))?;
    let repo_root = discover_binary_checkout_repo_root();
    let report = classify_route(command, request, repo_root.as_deref());
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn run_remote(command: &str, args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(remote_usage(command));
    }
    let args = RemoteArgs::parse(command, args)?;
    let request_bytes =
        fs::read(&args.request).map_err(|error| format!("failed to read request: {error}"))?;
    let mut request: RemoteRouteRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("typed_remote_request_invalid_json: {error}"))?;
    let repo_root = discover_repo_root(env::current_dir().map_err(|error| error.to_string())?)
        .ok_or_else(|| "repository_root_unavailable: could not find containing .git".to_string())?;
    let mut receipts = load_remote_route_receipts(&repo_root, &request)
        .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
    if args.observe_github {
        let mut adapter = RealProcessAdapter::new(EnvironmentCredentialResolver);
        let observed = observe_github_pr_readback(&request, &mut adapter)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
        request = observed.request;
        merge_observed_receipts(&mut receipts, observed.receipts);
    }
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

fn run_terminal(command: &str, args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(terminal_usage(command));
    }
    let args = TerminalArgs::parse(command, args)?;
    let request_bytes =
        fs::read(&args.request).map_err(|error| format!("failed to read request: {error}"))?;
    let request: TerminalRouteRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("typed_terminal_request_invalid_json: {error}"))?;
    let result = prepare_terminal_route(command, &request)
        .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
    let report = TerminalCommandReport {
        schema: "csdlc.v3.terminal_cleanup_cutover.v1",
        command: command.to_owned(),
        read_only: true,
        requested_mutation: command == "clean"
            && request
                .cleanup
                .as_ref()
                .is_some_and(|cleanup| cleanup.remove),
        performed_mutation: false,
        operational_authority: false,
        cutover_issue: 505,
        result,
    };
    serde_json::to_string(&report).map_err(|error| error.to_string())
}

fn merge_observed_receipts(receipts: &mut RemoteRouteReceipts, observed: RemoteRouteReceipts) {
    receipts.github_readback = observed.github_readback;
    receipts.adapter = observed.adapter;
}

fn discover_repo_root(start: PathBuf) -> Option<PathBuf> {
    for candidate in start.ancestors() {
        if candidate.join(".git").exists() {
            return Some(candidate.to_path_buf());
        }
    }
    None
}

fn discover_binary_checkout_repo_root() -> Option<PathBuf> {
    env::current_exe()
        .ok()
        .and_then(discover_repo_root)
        .or_else(|| {
            option_env!("CARGO_MANIFEST_DIR")
                .map(PathBuf::from)
                .and_then(discover_repo_root)
        })
}

fn remote_usage(command: &str) -> String {
    format!(
        "usage: csdlc {command} --request <path> [--observe-github]\n\nstatus: implemented\nauthority: C-SDLC v3 is not live authority before #505 cutover."
    )
}

fn terminal_usage(command: &str) -> String {
    format!(
        "usage: csdlc {command} --request <path>\n\nstatus: implemented\nauthority: C-SDLC v3 is not live authority before #505 cutover."
    )
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

#[derive(Debug, Serialize)]
struct RemoteCommandReport<T> {
    schema: &'static str,
    command: String,
    read_only: bool,
    operational_authority: bool,
    cutover_issue: u64,
    result: T,
}

#[derive(Debug, Serialize)]
struct TerminalCommandReport<T> {
    schema: &'static str,
    command: String,
    read_only: bool,
    requested_mutation: bool,
    performed_mutation: bool,
    operational_authority: bool,
    cutover_issue: u64,
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

#[derive(Debug)]
struct RemoteArgs {
    request: PathBuf,
    observe_github: bool,
}

#[derive(Debug)]
struct TerminalArgs {
    request: PathBuf,
}

#[derive(Debug)]
struct RequestOnlyArgs {
    request: PathBuf,
}

impl RequestOnlyArgs {
    fn parse(command: &str, args: &[String]) -> Result<Self, String> {
        let usage = format!("usage: csdlc {command} --request <path>");
        let [flag, path] = args else {
            return Err(usage);
        };
        if flag != "--request" {
            return Err(format!("{usage}; unexpected argument {flag}"));
        }
        Ok(Self {
            request: PathBuf::from(path),
        })
    }
}

impl RemoteArgs {
    fn parse(command: &str, args: &[String]) -> Result<Self, String> {
        let mut request = None;
        let mut observe_github = false;
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
                "--observe-github" => {
                    if observe_github {
                        return Err("duplicate argument --observe-github".into());
                    }
                    observe_github = true;
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
            observe_github,
        })
    }
}

impl TerminalArgs {
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
                        format!("{}; missing value for --request", terminal_usage(command))
                    })?));
                }
                _ => {
                    return Err(format!(
                        "{}; unexpected argument {arg}",
                        terminal_usage(command)
                    ))
                }
            }
        }
        Ok(Self {
            request: request.ok_or_else(|| TERMINAL_USAGE.to_string())?,
        })
    }
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
