use std::{env, fs, path::PathBuf};

use csdlc_v3::{
    adapters::{EnvironmentCredentialResolver, RealProcessAdapter},
    application::FoundationState,
    commands::local::{
        discover_operational_local_context, execute_local_route, execute_operational_local_route,
        finding, initialize_v3_local_state, inspect_local_lifecycle_state, inspect_v3_local_state,
        local_route_command, local_route_status, prepare_local_workflow, LocalPreparationRequest,
        PlanStatus, WorktreeRegistration, LOCAL_ROUTE_NAMES,
    },
    commands::proof::{classify_route, ProofRouteRequest, ProofRouteStatus, PROOF_ROUTE_NAMES},
    commands::remote::{
        canonical_authority_selector_digest, dispatch_operational_remote,
        load_remote_route_receipts, observe_github_pr_readback,
        prepare_remote_publication_route_with_receipts, GithubMutation, GithubMutationRequest,
        OperationalRemoteDispatchRequest, OperationalRemoteOperation, RemoteRouteReceipts,
        RemoteRouteRequest, REMOTE_PUBLICATION_ROUTE_NAMES,
    },
    commands::sprint::{parse_request as parse_sprint_request, verify_sprint_readiness},
    commands::terminal::{
        prepare_terminal_cutover_with_github_observation,
        prepare_terminal_finish_with_github_observation, prepare_terminal_route, CleanupDecision,
        CutoverOperation, FinishDecision, TerminalRouteRequest, TerminalRouteStatus,
        TERMINAL_ROUTE_NAMES,
    },
    repository::RepositoryContext,
};
use serde::Serialize;

const AUTHORITY_HELP: &str = "C-SDLC v3 is operational after #505 / PR #591; authenticated canonical selector and reconciliation receipt validation are required. Missing or stale proof suspends authority.";

const ROOT_USAGE: &str =
    "usage: csdlc <command>\n\nCommands:\n  foundation --repo-root <path>\n  local --request <path> --registry <path> --registrations <path>\n  bind --request <path> --registry <path> --registrations <path>\n  clean --request <path>\n  cutover --request <path>\n  doctor --request <path> --registry <path> --registrations <path>\n  edit --request <path> --registry <path> --registrations <path>\n  eligibility --request <path> --registry <path> --registrations <path>\n  finish --request <path>\n  github --request <path> [--observe-github] [--execute]\n  github-issue create --repo <owner/name> --title <title> (--body <body>|--body-file <path>) --expected-head <sha> [--label <label>] [--assignee <login>] [--milestone <number>] [--execute]\n  github-issue --request <path> [--observe-github] [--execute]\n  github-pr --request <path> [--observe-github] [--execute]\n  install --request <path>\n  issue --request <path> --registry <path> --registrations <path>\n  pr-state --request <path> [--observe-github]\n  proof --request <path>\n  publish --request <path> [--observe-github]\n  remote --help\n  review --request <path>\n  schedule --request <path> --registry <path> --registrations <path>\n  shadow --request <path>\n  shepherd --request <path> --registry <path> --registrations <path>\n  soak --request <path>\n  sprint --repo-root <path> --request <path>\n  validate --request <path> --registry <path> --registrations <path>";
const FOUNDATION_USAGE: &str = "usage: csdlc foundation --repo-root <path>";
const LOCAL_USAGE: &str =
    "usage: csdlc local --request <path> --registry <path> --registrations <path>";
const REMOTE_USAGE: &str =
    "usage: csdlc <github|github-issue|github-pr|pr-state|publish|review> --request <path> [--observe-github] [--execute]";
const TERMINAL_USAGE: &str =
    "usage: csdlc <finish|clean|cutover> --request <path> [--observe-github]";
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
        "--help" | "-h" => Ok(format!("{ROOT_USAGE}\n\nauthority: {AUTHORITY_HELP}")),
        "foundation" => run_foundation(rest),
        "local" => run_local(rest),
        "remote" => run_remote_overview(rest),
        "sprint" => run_sprint(rest),
        route if PROOF_ROUTE_NAMES.contains(&route) => run_proof_route(route, rest),
        route if LOCAL_ROUTE_NAMES.contains(&route) => run_local_route(route, rest),
        "github-issue" if rest.first().is_some_and(|arg| arg == "create") => {
            run_simple_issue_create(&rest[1..])
        }
        route if REMOTE_PUBLICATION_ROUTE_NAMES.contains(&route) => run_remote(route, rest),
        route if TERMINAL_ROUTE_NAMES.contains(&route) => run_terminal(route, rest),
        "rollback" => run_terminal("rollback", rest),
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
            "{usage}\n\nstatus: implemented\nauthority: {AUTHORITY_HELP}"
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
    if route != "local" {
        let repository_root = args
            .repo_root
            .clone()
            .or_else(|| env::current_dir().ok())
            .ok_or_else(|| "operational repository root is unavailable".to_string())?;
        match discover_operational_local_context(&repository_root, &request) {
            Ok(Some(context)) => {
                let operational =
                    match execute_operational_local_route(route, &request, &registry, &context) {
                        Ok(operational) => operational,
                        Err(findings)
                            if can_fallback_from_read_only_operational_context(route)
                                && findings.iter().any(|finding| {
                                    read_only_discovery_fallback_code(&finding.code)
                                }) =>
                        {
                            // Read-only diagnostic routes are safe in ordinary issue worktrees
                            // whose parent is the required bind parent.  The operational mutation
                            // context is intentionally invalid there, but the construction report
                            // remains useful and non-mutating.
                            return run_local_construction_report(
                                route,
                                args,
                                request,
                                registry,
                                registrations,
                            );
                        }
                        Err(findings) => {
                            return Err(
                                serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into())
                            );
                        }
                    };
                return serde_json::to_string(&serde_json::json!({
                    "schema": "csdlc.v3.operational_local.v1",
                    "command": route,
                    "read_only": !operational.mutated,
                    "operational_read_only": !operational.mutated,
                    "operational_authority": true,
                    "writes_v3_state": operational.mutated,
                    "result": operational,
                }))
                .map_err(|error| error.to_string());
            }
            Ok(None) => {}
            Err(findings)
                if can_fallback_from_read_only_operational_context(route)
                    && findings
                        .iter()
                        .any(|finding| read_only_discovery_fallback_code(&finding.code)) => {}
            Err(findings) => {
                return Err(serde_json::to_string(&findings).unwrap_or_else(|_| "[]".into()));
            }
        }
    }
    run_local_construction_report(route, args, request, registry, registrations)
}

fn can_fallback_from_read_only_operational_context(route: &str) -> bool {
    matches!(route, "doctor" | "eligibility")
}

fn read_only_discovery_fallback_code(code: &str) -> bool {
    matches!(
        code,
        "invalid_operational_roots" | "worktree_parent_unavailable"
    )
}

fn run_local_construction_report(
    route: &str,
    args: LocalArgs,
    request: LocalPreparationRequest,
    registry: csdlc_v3::commands::local::PromptRegistry,
    registrations: Vec<WorktreeRegistration>,
) -> Result<String, String> {
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
            "usage: csdlc {command} --request <path>\n\nstatus: implemented_construction\nauthority: {AUTHORITY_HELP}"
        ));
    }
    let request_path = RequestOnlyArgs::parse(command, args)?.request;
    let request_bytes =
        fs::read(&request_path).map_err(|error| format!("failed to read request: {error}"))?;
    let request: ProofRouteRequest = serde_json::from_slice(&request_bytes)
        .map_err(|error| format!("invalid request json: {error}"))?;
    let repo_root = discover_binary_checkout_repo_root();
    let report = classify_route(command, request, repo_root.as_deref());
    let serialized = serde_json::to_string(&report).map_err(|error| error.to_string())?;
    if report.status == ProofRouteStatus::Blocked {
        Err(serialized)
    } else {
        Ok(serialized)
    }
}

fn run_remote(command: &str, args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(remote_usage(command));
    }
    let args = RemoteArgs::parse(command, args)?;
    let request_bytes =
        fs::read(&args.request).map_err(|error| format!("failed to read request: {error}"))?;
    if command != "pr-state" && args.execute {
        if args.observe_github {
            return Err(format!(
                "{}; --observe-github is reserved for the read-only pr-state route",
                remote_usage(command)
            ));
        }
        let dispatch: OperationalRemoteDispatchRequest = serde_json::from_slice(&request_bytes)
            .map_err(|error| format!("typed_operational_remote_request_invalid_json: {error}"))?;
        validate_operational_remote_route(command, &dispatch.operation)?;
        let repo_root = discover_repo_root(env::current_dir().map_err(|error| error.to_string())?)
            .ok_or_else(|| {
                "repository_root_unavailable: could not find containing .git".to_string()
            })?;
        let mut adapter = RealProcessAdapter::new(EnvironmentCredentialResolver);
        let result = dispatch_operational_remote(&repo_root, &dispatch, &mut adapter)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
        let read_only = !matches!(
            dispatch.operation,
            OperationalRemoteOperation::GithubMutation(_)
        );
        return serde_json::to_string(&RemoteCommandReport {
            schema: "csdlc.v3.operational_remote.v1",
            command: command.to_owned(),
            read_only,
            operational_authority: true,
            cutover_issue: 505,
            result,
        })
        .map_err(|error| error.to_string());
    }
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

fn run_simple_issue_create(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] {
        return Ok(SimpleIssueCreateArgs::usage());
    }
    let args = SimpleIssueCreateArgs::parse(args)?;
    let repo_root = discover_repo_root(env::current_dir().map_err(|error| error.to_string())?)
        .ok_or_else(|| "repository_root_unavailable: could not find containing .git".to_string())?;
    let body = match (&args.body, &args.body_file) {
        (Some(body), None) if !body.trim().is_empty() => body.clone(),
        (None, Some(path)) => {
            let body = fs::read_to_string(path)
                .map_err(|error| format!("failed to read --body-file: {error}"))?;
            if body.trim().is_empty() {
                return Err("issue body must not be empty".into());
            }
            body
        }
        (Some(_), Some(_)) => return Err("use exactly one of --body or --body-file".into()),
        _ => return Err("use exactly one non-empty --body or --body-file".into()),
    };
    if args.title.trim().is_empty() {
        return Err("issue title must not be empty".into());
    }
    if !is_exact_git_sha(&args.expected_head) {
        return Err("--expected-head must be an exact 40-hex Git SHA".into());
    }
    let dispatch = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: canonical_authority_selector_digest(&repo_root)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?,
        exact_review_sha: args.expected_head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(GithubMutationRequest {
            repository: args.repository,
            issue: 0,
            pull_request: None,
            cutover_issue: None,
            operator_approval: None,
            expected_head_sha: args.expected_head,
            credential_names: vec![args.credential_name],
            mutation: GithubMutation::IssueCreate {
                title: args.title,
                body,
                labels: args.labels,
                assignees: args.assignees,
                milestone: args.milestone,
            },
        }),
    };
    if !args.execute {
        return serde_json::to_string(&dispatch).map_err(|error| error.to_string());
    }
    let mut adapter = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let result = dispatch_operational_remote(&repo_root, &dispatch, &mut adapter)
        .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?;
    serde_json::to_string(&RemoteCommandReport {
        schema: "csdlc.v3.operational_remote.v1",
        command: "github-issue".to_owned(),
        read_only: false,
        operational_authority: true,
        cutover_issue: 505,
        result,
    })
    .map_err(|error| error.to_string())
}

fn is_exact_git_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn validate_operational_remote_route(
    command: &str,
    operation: &OperationalRemoteOperation,
) -> Result<(), String> {
    if matches!(
        (command, operation),
        ("review", OperationalRemoteOperation::Review(_))
            | ("publish", OperationalRemoteOperation::Publish(_))
    ) {
        Ok(())
    } else if let OperationalRemoteOperation::GithubMutation(mutation) = operation {
        if github_mutation_route_matches(command, &mutation.mutation) {
            Ok(())
        } else {
            Err(format!(
                "operational_remote_route_mismatch: {command} does not own the requested operation"
            ))
        }
    } else {
        Err(format!(
            "operational_remote_route_mismatch: {command} does not own the requested operation"
        ))
    }
}

fn github_mutation_route_matches(command: &str, mutation: &GithubMutation) -> bool {
    match command {
        "github" => true,
        "github-issue" => matches!(
            mutation,
            GithubMutation::IssueCreate { .. }
                | GithubMutation::IssueComment { .. }
                | GithubMutation::IssueEdit { .. }
        ),
        "github-pr" => matches!(
            mutation,
            GithubMutation::PullRequestCreate { .. }
                | GithubMutation::PullRequestUpdate { .. }
                | GithubMutation::PullRequestReady
        ),
        _ => false,
    }
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
    let terminal_route = if command == "rollback" {
        if request.cutover.as_ref().map(|cutover| cutover.operation)
            != Some(CutoverOperation::Rollback)
        {
            return Err(
                "rollback_requires_typed_rollback_operation: cutover.operation must be rollback"
                    .into(),
            );
        }
        "cutover"
    } else {
        command
    };
    let result = if terminal_route == "finish" && args.observe_github {
        let mut adapter = RealProcessAdapter::new(EnvironmentCredentialResolver);
        prepare_terminal_finish_with_github_observation(&request, &mut adapter)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?
    } else if terminal_route == "cutover" && args.observe_github {
        let mut adapter = RealProcessAdapter::new(EnvironmentCredentialResolver);
        prepare_terminal_cutover_with_github_observation(&request, &mut adapter)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?
    } else {
        prepare_terminal_route(terminal_route, &request)
            .map_err(|finding| serde_json::to_string(&finding).unwrap_or_else(|_| "{}".into()))?
    };
    let blocked = result.status == TerminalRouteStatus::Blocked;
    let performed_mutation = result.cutover.as_ref().is_some_and(|decision| {
        decision.executes_cutover
            || decision.operation == CutoverOperation::Rollback
                && decision.rollback_receipt_path.is_some()
    }) || result
        .cleanup
        .as_ref()
        .is_some_and(|decision| matches!(decision, CleanupDecision::Removed { .. }))
        || result.finish.as_ref().is_some_and(|decision| {
            matches!(decision, FinishDecision::TerminalClosedOut { .. })
                && request.terminal_state.is_some()
        });
    let operational_authority = result.operational_authority;
    let report = TerminalCommandReport {
        schema: "csdlc.v3.terminal_cleanup_cutover.v1",
        command: command.to_owned(),
        read_only: !performed_mutation,
        requested_mutation: command == "clean"
            && request
                .cleanup
                .as_ref()
                .is_some_and(|cleanup| cleanup.remove)
            || request
                .cutover
                .as_ref()
                .is_some_and(|cutover| cutover.execute),
        performed_mutation,
        operational_authority,
        cutover_issue: 505,
        result,
    };
    let serialized = serde_json::to_string(&report).map_err(|error| error.to_string())?;
    if blocked {
        Err(serialized)
    } else {
        Ok(serialized)
    }
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
        "usage: csdlc {command} --request <path> [--observe-github] [--execute]\n\nstatus: implemented\nauthority: {AUTHORITY_HELP}"
    )
}

fn terminal_usage(command: &str) -> String {
    format!(
        "usage: csdlc {command} --request <path>\n\nstatus: implemented\nauthority: {AUTHORITY_HELP}"
    )
}

fn run_remote_overview(args: &[String]) -> Result<String, String> {
    if args == ["--help"] || args == ["-h"] || args.is_empty() {
        return Ok(format!(
            "usage: csdlc remote --help\n\nstatus: implemented\nauthority: {AUTHORITY_HELP}\nroutes: github, github-issue, github-pr, pr-state, publish, review"
        ));
    }
    Err("usage: csdlc remote --help".into())
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
    execute: bool,
}

#[derive(Debug)]
struct SimpleIssueCreateArgs {
    repository: String,
    title: String,
    body: Option<String>,
    body_file: Option<PathBuf>,
    labels: Vec<String>,
    assignees: Vec<String>,
    milestone: Option<u64>,
    expected_head: String,
    credential_name: String,
    execute: bool,
}

impl SimpleIssueCreateArgs {
    fn usage() -> String {
        "usage: csdlc github-issue create --repo <owner/name> --title <title> (--body <body>|--body-file <path>) --expected-head <sha> [--label <label>] [--assignee <login>] [--milestone <number>] [--credential-name <env-name>] [--execute]\n\nThe simple form builds the same typed operational dispatch as --request. Without --execute it prints that request; mutation remains authority-gated."
            .into()
    }

    fn parse(args: &[String]) -> Result<Self, String> {
        let mut repository = None;
        let mut title = None;
        let mut body = None;
        let mut body_file = None;
        let mut labels = Vec::new();
        let mut assignees = Vec::new();
        let mut milestone = None;
        let mut expected_head = None;
        let mut credential_name = None;
        let mut execute = false;
        let mut iter = args.iter();
        while let Some(arg) = iter.next() {
            let value = |iter: &mut std::slice::Iter<'_, String>| {
                iter.next()
                    .cloned()
                    .ok_or_else(|| format!("missing value for {arg}"))
            };
            match arg.as_str() {
                "--repo" if repository.is_none() => repository = Some(value(&mut iter)?),
                "--title" if title.is_none() => title = Some(value(&mut iter)?),
                "--body" if body.is_none() => body = Some(value(&mut iter)?),
                "--body-file" if body_file.is_none() => {
                    body_file = Some(PathBuf::from(value(&mut iter)?))
                }
                "--label" => labels.push(value(&mut iter)?),
                "--assignee" => assignees.push(value(&mut iter)?),
                "--milestone" if milestone.is_none() => {
                    milestone = Some(value(&mut iter)?.parse::<u64>().map_err(|_| {
                        "--milestone must be a positive numeric milestone ID".to_string()
                    })?);
                    if milestone == Some(0) {
                        return Err("--milestone must be a positive numeric milestone ID".into());
                    }
                }
                "--expected-head" if expected_head.is_none() => {
                    expected_head = Some(value(&mut iter)?)
                }
                "--credential-name" if credential_name.is_none() => {
                    credential_name = Some(value(&mut iter)?)
                }
                "--execute" if !execute => execute = true,
                _ => {
                    return Err(format!(
                        "{}; unexpected or duplicate argument {arg}",
                        Self::usage()
                    ))
                }
            }
        }
        Ok(Self {
            repository: repository.ok_or_else(Self::usage)?,
            title: title.ok_or_else(Self::usage)?,
            body,
            body_file,
            labels,
            assignees,
            milestone,
            expected_head: expected_head.ok_or_else(Self::usage)?,
            credential_name: credential_name.unwrap_or_else(|| "GITHUB_TOKEN".into()),
            execute,
        })
    }
}

#[derive(Debug)]
struct TerminalArgs {
    request: PathBuf,
    observe_github: bool,
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
        let mut execute = false;
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
                "--execute" => {
                    if execute {
                        return Err("duplicate argument --execute".into());
                    }
                    execute = true;
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
            execute,
        })
    }
}

impl TerminalArgs {
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
                        format!("{}; missing value for --request", terminal_usage(command))
                    })?));
                }
                "--observe-github" if matches!(command, "finish" | "cutover" | "rollback") => {
                    if observe_github {
                        return Err("duplicate argument --observe-github".into());
                    }
                    observe_github = true;
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
            observe_github,
        })
    }
}

impl LocalArgs {
    fn parse(args: &[String], route: &str) -> Result<Self, String> {
        let usage = format!(
            "usage: csdlc {route} --request <path> --registry <path> --registrations <path> [--repo-root <path>] [--v3-state-root <path>]"
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
