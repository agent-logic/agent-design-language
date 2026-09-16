//! Operational local-route dispatch and routing reports.

use std::path::PathBuf;

use super::binding::bind_operational_issue;
use super::cards::{edit_operational_cards, validate_operational_issue, validation_findings};
use super::context::{require_operational_cas, validate_context};
use super::issue::initialize_operational_issue;
use super::lifecycle::inspect_lifecycle_issue_root;
use super::planning::{plan_cards, validate_contract};
use super::storage::operational_result;
use super::transactions::{
    acquire_issue_mutation_lock, load_local_completion, local_request_digest,
    local_transaction_journal_path, recover_pending_local_transaction,
};
use super::worktree::git_worktree_registration;
use super::{
    finding, is_observation_route, DoctorFinding, LocalPreparationRequest, OperationalLocalContext,
    OperationalLocalResult, OperationalRoutingReport, PlanStatus, PromptRegistry,
    LOCAL_ROUTE_NAMES,
};

/// Execute one local route with native v3 state and Git operations.
///
/// This entry point deliberately has no v2 subprocess fallback. Callers may
/// expose it only after the v3 writer fence is active.
#[cfg(unix)]
pub fn execute_operational_local_route(
    route: &str,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if !LOCAL_ROUTE_NAMES.contains(&route) {
        return Err(vec![finding(
            PlanStatus::Failed,
            "unknown_local_route",
            "unknown native v3 local route",
        )]);
    }
    validate_contract(request)?;
    plan_cards(request.issue, &request.registry_version, registry)?;
    validate_context(route, request, context)?;
    // Observation must never create locks, recover a journal, or reuse a
    // mutation completion. A concurrent writer may make observation fail;
    // inspection is not authority to repair or complete that writer's work.
    if is_observation_route(route) {
        if local_transaction_journal_path(context, request.issue)
            .symlink_metadata()
            .is_ok()
        {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_recovery_required",
                "a pending local journal requires explicit recovery: inspect its bind/edit identity and reissue the exact original mutating request; diagnostics never replay it",
            )]);
        }
        let issue_root = context.state_root.join(format!("issues/{}", request.issue));
        let before = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
        require_operational_cas(route, request, &before)?;
        return match route {
            "validate" => validate_operational_issue(request, registry, &issue_root),
            "doctor" | "eligibility" => {
                diagnose_operational_issue(route, request, registry, context)
            }
            "schedule" | "shepherd" => route_operational_issue(route, request, registry, context),
            _ => unreachable!("observation route membership checked above"),
        };
    }
    let _issue_lock = acquire_issue_mutation_lock(&context.state_root, request.issue)?;
    // Binding can transfer ownership while this invocation waits for the lock.
    validate_context(route, request, context)?;
    recover_pending_local_transaction(context, request.issue)?;
    let request_digest = local_request_digest(request)?;
    if let Some(result) = load_local_completion(context, request, route, &request_digest)? {
        return Ok(result);
    }
    let issue_root = context.state_root.join(format!("issues/{}", request.issue));
    let before = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
    require_operational_cas(route, request, &before)?;

    match route {
        "issue" => initialize_operational_issue(request, registry, &issue_root),
        "bind" => bind_operational_issue(request, context, &issue_root),
        "edit" => edit_operational_cards(request, registry, context, &issue_root),
        "validate" => validate_operational_issue(request, registry, &issue_root),
        "doctor" | "eligibility" => diagnose_operational_issue(route, request, registry, context),
        "schedule" | "shepherd" => route_operational_issue(route, request, registry, context),
        _ => unreachable!("route membership checked above"),
    }
}

#[cfg(not(unix))]
pub fn execute_operational_local_route(
    _route: &str,
    _request: &LocalPreparationRequest,
    _registry: &PromptRegistry,
    _context: &OperationalLocalContext,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    Err(vec![finding(
        PlanStatus::Blocked,
        "operational_local_unsupported_platform",
        "native v3 operational mutation is supported only on Unix platforms",
    )])
}

fn diagnose_operational_issue(
    route: &str,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let issue_root = context.state_root.join(format!("issues/{}", request.issue));
    let mut findings = validation_findings(request, registry, &issue_root);
    let observed = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
    if observed.phase.as_deref() == Some("bound") {
        let target = PathBuf::from(&request.worktree);
        if git_worktree_registration(&context.repository_root, &request.branch, &target)? {
            findings.push(finding(
                PlanStatus::Passed,
                "binding_live",
                "exact Git worktree registration is live",
            ));
        } else {
            findings.push(finding(
                PlanStatus::Blocked,
                "binding_not_live",
                "recorded binding is absent from live Git worktree topology",
            ));
        }
    }
    let ready = !findings
        .iter()
        .any(|item| matches!(item.status, PlanStatus::Blocked | PlanStatus::Failed));
    Ok(operational_result(
        route,
        request.issue,
        false,
        observed.phase.as_deref().unwrap_or("unknown"),
        observed.generation.unwrap_or(0),
        observed.digest.unwrap_or_default(),
        if ready {
            Some(if observed.phase.as_deref() == Some("bound") {
                "shepherd"
            } else {
                "bind"
            })
        } else {
            None
        },
        findings,
    ))
}

fn route_operational_issue(
    route: &str,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let mut result = diagnose_operational_issue(route, request, registry, context)?;
    let diagnostics_blocked = result
        .findings
        .iter()
        .any(|item| matches!(item.status, PlanStatus::Blocked | PlanStatus::Failed));
    if route == "schedule" {
        let readiness = request.schedule_readiness.as_ref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "schedule_readiness_missing",
                "schedule requires all six explicit readiness dimensions",
            )]
        })?;
        let dimensions = [
            ("phase_ready", readiness.phase_ready),
            ("cards_ready", readiness.cards_ready),
            ("design_ready", readiness.design_ready),
            ("dependencies_ready", readiness.dependencies_ready),
            ("paths_clear", readiness.paths_clear),
            ("budget_available", readiness.budget_available),
        ];
        let mut blockers: Vec<String> = dimensions
            .into_iter()
            .filter_map(|(name, ready)| (!ready).then_some(name.to_owned()))
            .collect();
        if diagnostics_blocked {
            blockers.push("operational_diagnostics".into());
        }
        let ready = blockers.is_empty();
        result.next_route = Some(if ready { "validate" } else { "doctor" }.into());
        result.routing = Some(OperationalRoutingReport {
            schema: "csdlc.scheduler.report.v1".into(),
            state: if ready { "ready" } else { "blocked" }.into(),
            blockers,
            eligible_operations: if ready {
                vec!["validate".into()]
            } else {
                Vec::new()
            },
        });
    } else {
        let routing = request.shepherd_routing.as_ref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "shepherd_routing_missing",
                "shepherd requires explicit waiting, retry, repair, and operator-decision state",
            )]
        })?;
        let state = if routing.operator_decision_needed {
            "operator_required"
        } else if routing.repair_needed {
            "repair_required"
        } else if routing.retryable_failure {
            "retryable"
        } else if routing.dependency_wait || routing.validation.is_none() {
            "waiting"
        } else if diagnostics_blocked {
            "repair_required"
        } else {
            "ready"
        };
        let eligible_operations = match state {
            "ready" => vec!["schedule".into()],
            "retryable" => vec!["retry".into()],
            "repair_required" => vec!["repair".into()],
            "operator_required" => vec!["operator_decision".into()],
            _ => Vec::new(),
        };
        result.next_route = Some(
            match state {
                "ready" => "schedule",
                "retryable" => "retry",
                "repair_required" => "doctor",
                "operator_required" => "operator",
                _ => "shepherd",
            }
            .into(),
        );
        result.routing = Some(OperationalRoutingReport {
            schema: "csdlc.shepherd.report.v1".into(),
            state: state.into(),
            blockers: Vec::new(),
            eligible_operations,
        });
    }
    Ok(result)
}
