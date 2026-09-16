//! Typed contract and non-authoritative local route planning.

use std::collections::{BTreeMap, BTreeSet};

use serde_json::Value;

use super::{
    finding, BindAuthorization, CardRenderPlan, DoctorFinding, LocalCommand,
    LocalLifecycleStateObservation, LocalPreparationPlan, LocalPreparationRequest,
    LocalRouteResult, LocalRouteStatus, PlanStatus, PromptRegistry, RenderedCard,
    WorktreeRegistration, LOCAL_ROUTE_NAMES, REQUIRED_CARD_KINDS,
};

impl LocalPreparationRequest {
    /// Parse a typed local-preparation request from JSON bytes.
    pub fn from_json(bytes: &[u8]) -> Result<Self, Vec<DoctorFinding>> {
        serde_json::from_slice(bytes).map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "typed_contract_invalid_json",
                &error.to_string(),
            )]
        })
    }
}

impl PromptRegistry {
    /// Read the active prompt-template registry shape from `current.json`.
    pub fn from_current_json(bytes: &[u8]) -> Result<Self, Vec<DoctorFinding>> {
        let value: Value = serde_json::from_slice(bytes).map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "registry_invalid_json",
                &error.to_string(),
            )]
        })?;
        let schema = value.get("schema").and_then(Value::as_str);
        let status = value.get("status").and_then(Value::as_str);
        let version = value
            .get("semver")
            .and_then(Value::as_str)
            .or_else(|| {
                value
                    .get("csdlc_prompt_template_set")
                    .and_then(Value::as_str)
            })
            .ok_or_else(|| {
                vec![finding(
                    PlanStatus::Blocked,
                    "registry_version_missing",
                    "active registry must declare semver",
                )]
            })?;
        if schema != Some("adl.csdlc.prompt_template_registry.v1") || status != Some("active") {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "registry_not_active",
                "prompt registry must be active and use the expected schema",
            )]);
        }
        let templates = value
            .get("templates")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                vec![finding(
                    PlanStatus::Blocked,
                    "registry_templates_missing",
                    "active registry must declare templates",
                )]
            })?;
        let mut template_paths = BTreeMap::new();
        for (kind, entry) in templates {
            let Some(path) = entry.get("path").and_then(Value::as_str) else {
                return Err(vec![finding(
                    PlanStatus::Blocked,
                    "registry_template_path_missing",
                    "active registry template entries must declare paths",
                )]);
            };
            template_paths.insert(kind.clone(), path.to_owned());
        }
        Ok(Self {
            version: version.into(),
            card_kinds: templates.keys().cloned().collect(),
            template_paths,
        })
    }
}

/// Validate that the caller supplied a typed local preparation contract.
pub fn validate_contract(request: &LocalPreparationRequest) -> Result<(), Vec<DoctorFinding>> {
    let mut findings = Vec::new();
    if request.issue == 0 {
        findings.push(finding(
            PlanStatus::Failed,
            "issue_missing",
            "issue identity must be non-zero",
        ));
    }
    for (code, value) in [
        ("title_missing", &request.title),
        ("repository_missing", &request.repository),
        ("branch_missing", &request.branch),
        ("worktree_missing", &request.worktree),
        ("registry_version_missing", &request.registry_version),
    ] {
        if value.trim().is_empty() {
            findings.push(finding(PlanStatus::Failed, code, "typed field is required"));
        }
    }
    if !request.repository.trim().is_empty()
        && !is_valid_github_repository_name(&request.repository)
    {
        findings.push(finding(
            PlanStatus::Failed,
            "repository_invalid",
            "repository must use owner/name GitHub syntax",
        ));
    }
    let unique = request.commands.iter().copied().collect::<BTreeSet<_>>();
    for required in required_local_commands() {
        if !unique.contains(&required) {
            findings.push(finding(
                PlanStatus::Blocked,
                "command_missing",
                "local preparation requires every v3 local command route",
            ));
        }
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(findings)
    }
}

fn is_valid_github_repository_name(repository: &str) -> bool {
    let Some((owner, name)) = repository.split_once('/') else {
        return false;
    };
    !owner.is_empty()
        && !name.is_empty()
        && !name.contains('/')
        && [owner, name].into_iter().all(|component| {
            component
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.'))
        })
}

/// Build the card-rendering plan from an active registry observation.
pub fn plan_cards(
    issue: u64,
    expected_registry_version: &str,
    registry: &PromptRegistry,
) -> Result<CardRenderPlan, Vec<DoctorFinding>> {
    let mut findings = Vec::new();
    if registry.version != expected_registry_version {
        findings.push(finding(
            PlanStatus::Blocked,
            "registry_version_mismatch",
            "active prompt registry version does not match the request",
        ));
    }
    for kind in REQUIRED_CARD_KINDS {
        if !registry.card_kinds.contains(kind) {
            findings.push(finding(
                PlanStatus::Blocked,
                "card_kind_missing",
                "active registry must provide every lifecycle card",
            ));
        }
    }
    if findings.is_empty() {
        let mut rendered_cards = Vec::new();
        let issue_text = issue.to_string();
        for kind in REQUIRED_CARD_KINDS {
            let Some(template_ref) = registry.template_paths.get(kind) else {
                return Err(vec![finding(
                    PlanStatus::Blocked,
                    "registry_template_path_missing",
                    "active registry template entries must declare paths",
                )]);
            };
            rendered_cards.push(RenderedCard {
                kind: kind.to_owned(),
                template_ref: template_ref.clone(),
                rendered_ref: format!(".csdlc/issues/{issue}/cards/{kind}.md"),
                render_manifest_digest: stable_digest(&[
                    "csdlc-v3.local.render-manifest.v1",
                    template_ref,
                    &issue_text,
                    kind,
                ]),
            });
        }
        Ok(CardRenderPlan {
            registry_version: registry.version.clone(),
            issue,
            card_kinds: REQUIRED_CARD_KINDS
                .into_iter()
                .map(str::to_string)
                .collect(),
            rendered_cards,
        })
    } else {
        Err(findings)
    }
}

fn stable_digest(values: &[&str]) -> String {
    let mut hasher = blake3::Hasher::new();
    for value in values {
        hasher.update(value.as_bytes());
        hasher.update(b"\0");
    }
    hasher.finalize().to_hex().to_string()
}

/// Authorize bind only from exact registered topology, never branch name alone.
pub fn authorize_bind(
    request: &LocalPreparationRequest,
    registrations: &[WorktreeRegistration],
) -> Result<BindAuthorization, Vec<DoctorFinding>> {
    let matching = registrations
        .iter()
        .filter(|entry| entry.branch == request.branch && entry.worktree == request.worktree)
        .collect::<Vec<_>>();
    match matching.as_slice() {
        [entry] if !entry.primary => Ok(BindAuthorization {
            issue: request.issue,
            branch: entry.branch.clone(),
            worktree: entry.worktree.clone(),
        }),
        [] => Err(vec![finding(
            PlanStatus::Blocked,
            "registered_topology_missing",
            "bind requires exact registered branch and worktree",
        )]),
        [_] => Err(vec![finding(
            PlanStatus::Blocked,
            "primary_worktree_denied",
            "bind cannot target the primary checkout",
        )]),
        _ => Err(vec![finding(
            PlanStatus::Failed,
            "registered_topology_ambiguous",
            "bind requires one exact registered worktree",
        )]),
    }
}

/// Combine typed contract, registry, and topology observations into a PVF plan.
pub fn prepare_local_workflow(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    registrations: &[WorktreeRegistration],
) -> Result<LocalPreparationPlan, Vec<DoctorFinding>> {
    let mut findings = Vec::new();
    if let Err(mut errors) = validate_contract(request) {
        findings.append(&mut errors);
    }
    let cards = match plan_cards(request.issue, &request.registry_version, registry) {
        Ok(cards) => Some(cards),
        Err(mut errors) => {
            findings.append(&mut errors);
            None
        }
    };
    let bind = match authorize_bind(request, registrations) {
        Ok(bind) => Some(bind),
        Err(mut errors) => {
            findings.append(&mut errors);
            None
        }
    };
    if findings.is_empty() {
        Ok(LocalPreparationPlan {
            issue: request.issue,
            commands: request.commands.clone(),
            cards: cards.expect("cards checked"),
            bind: bind.expect("bind checked"),
            lifecycle_state: None,
            findings: vec![finding(
                PlanStatus::Ready,
                "doctor_ready",
                "typed local preparation reached a doctor-validated PVF plan",
            )],
        })
    } else {
        Err(findings)
    }
}

/// Reject any attempt to classify non-authoritative V3-D planning as authority.
pub fn grants_operational_authority(command: LocalCommand) -> bool {
    match command {
        LocalCommand::PrepareIssue
        | LocalCommand::BindWorktree
        | LocalCommand::EditCards
        | LocalCommand::PlanPvf
        | LocalCommand::Doctor
        | LocalCommand::Schedule
        | LocalCommand::Shepherd
        | LocalCommand::Eligibility => false,
    }
}

pub fn required_local_commands() -> [LocalCommand; 8] {
    [
        LocalCommand::PrepareIssue,
        LocalCommand::BindWorktree,
        LocalCommand::EditCards,
        LocalCommand::PlanPvf,
        LocalCommand::Doctor,
        LocalCommand::Schedule,
        LocalCommand::Shepherd,
        LocalCommand::Eligibility,
    ]
}

pub fn local_route_command(route: &str) -> Option<LocalCommand> {
    match route {
        "issue" => Some(LocalCommand::PrepareIssue),
        "bind" => Some(LocalCommand::BindWorktree),
        "edit" => Some(LocalCommand::EditCards),
        "validate" => Some(LocalCommand::PlanPvf),
        "doctor" => Some(LocalCommand::Doctor),
        "schedule" => Some(LocalCommand::Schedule),
        "shepherd" => Some(LocalCommand::Shepherd),
        "eligibility" => Some(LocalCommand::Eligibility),
        _ => None,
    }
}

pub fn local_route_status(
    route: &str,
    lifecycle_state: Option<&LocalLifecycleStateObservation>,
) -> Option<LocalRouteStatus> {
    let command = local_route_command(route)?;
    let (status, code, message) = match command {
        LocalCommand::PrepareIssue => match lifecycle_state {
            Some(state) if state.code == "missing_local_lifecycle_state" => (
                PlanStatus::Ready,
                "issue_preparation_ready",
                "issue route observed no existing local lifecycle state and can prepare one",
            ),
            Some(state) if state.code == "local_lifecycle_state_ready" => (
                PlanStatus::Ready,
                "issue_preparation_ready",
                "issue route initialized non-authoritative v3 local lifecycle state",
            ),
            Some(state) => (
                PlanStatus::Blocked,
                "issue_already_initialized",
                state.message.as_str(),
            ),
            None => (
                PlanStatus::Blocked,
                "lifecycle_observation_missing",
                "issue route requires --repo-root or --v3-state-root lifecycle-state observation",
            ),
        },
        LocalCommand::BindWorktree => route_state_status(
            lifecycle_state,
            &["ready"],
            "bind_topology_authorized",
            "bind route observed ready lifecycle state and exact non-primary worktree registration evidence",
        ),
        LocalCommand::EditCards => route_state_status(
            lifecycle_state,
            &["ready", "bound"],
            "edit_plan_ready",
            "edit route observed editable ready/bound lifecycle state and six-card denominator",
        ),
        LocalCommand::PlanPvf => route_state_status(
            lifecycle_state,
            &["ready", "bound"],
            "pvf_plan_ready",
            "validate route observed ready/bound lifecycle state with VPP/PVF denominator",
        ),
        LocalCommand::Doctor => match lifecycle_state {
            Some(state) if state.phase.is_some() => (
                PlanStatus::Ready,
                "doctor_ready",
                "doctor route observed local lifecycle state for read-only diagnosis",
            ),
            Some(state) => (state.status, state.code.as_str(), state.message.as_str()),
            None => (
                PlanStatus::Blocked,
                "lifecycle_observation_missing",
                "doctor route requires --repo-root lifecycle-state observation",
            ),
        },
        LocalCommand::Schedule => route_state_status(
            lifecycle_state,
            &["ready", "bound"],
            "schedule_plan_ready",
            "schedule route observed local lifecycle state suitable for ordered route planning",
        ),
        LocalCommand::Shepherd => route_state_status(
            lifecycle_state,
            &["bound"],
            "shepherd_plan_ready",
            "shepherd route observed bound lifecycle state for bounded execution guidance",
        ),
        LocalCommand::Eligibility => match lifecycle_state {
            Some(state) if state.ready_to_execute => (
                PlanStatus::Ready,
                "ready_to_execute",
                "eligibility route observed ready or bound lifecycle state and all six cards",
            ),
            Some(state) => (
                state.status,
                state.code.as_str(),
                "eligibility route is blocked by local lifecycle-state observation",
            ),
            None => (
                PlanStatus::Blocked,
                "lifecycle_observation_missing",
                "eligibility route requires --repo-root lifecycle-state observation",
            ),
        },
    };
    Some(LocalRouteStatus {
        route: route.to_owned(),
        command,
        status,
        code: code.to_owned(),
        message: message.to_owned(),
        issue_start_minutes_max: 3,
    })
}

fn route_state_status<'a>(
    lifecycle_state: Option<&'a LocalLifecycleStateObservation>,
    allowed_phases: &[&str],
    ready_code: &'static str,
    ready_message: &'static str,
) -> (PlanStatus, &'a str, &'a str) {
    match lifecycle_state {
        Some(state)
            if state.ready_to_execute
                && state
                    .phase
                    .as_deref()
                    .is_some_and(|phase| allowed_phases.contains(&phase)) =>
        {
            (PlanStatus::Ready, ready_code, ready_message)
        }
        Some(state) if state.ready_to_execute => (
            PlanStatus::Blocked,
            "unsupported_local_route_transition",
            "route is not valid for the observed local lifecycle phase",
        ),
        Some(state) => (state.status, state.code.as_str(), state.message.as_str()),
        None => (
            PlanStatus::Blocked,
            "lifecycle_observation_missing",
            "local route requires --repo-root or --v3-state-root lifecycle-state observation",
        ),
    }
}

pub fn execute_local_route(
    route: &str,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    registrations: &[WorktreeRegistration],
    lifecycle_state: Option<LocalLifecycleStateObservation>,
) -> Result<LocalRouteResult, Vec<DoctorFinding>> {
    require_lifecycle_digest(request, lifecycle_state.as_ref())?;
    require_route_state(route, lifecycle_state.as_ref())?;
    match local_route_command(route) {
        Some(LocalCommand::PrepareIssue) => Ok(LocalRouteResult::IssueInitialization {
            issue: request.issue,
            repository: request.repository.clone(),
            card_paths: REQUIRED_CARD_KINDS
                .into_iter()
                .map(|card| format!(".csdlc/issues/{}/cards/{card}.md", request.issue))
                .collect(),
            template_registry_version: registry.version.clone(),
            initialized_state: lifecycle_state,
        }),
        Some(LocalCommand::BindWorktree) => {
            let bind = authorize_bind(request, registrations)?;
            Ok(LocalRouteResult::BindWorktree {
                issue: bind.issue,
                branch: bind.branch,
                worktree: bind.worktree,
                primary_worktree_denied: true,
            })
        }
        Some(LocalCommand::EditCards) => Ok(LocalRouteResult::CardEdit {
            issue: request.issue,
            editable_cards: REQUIRED_CARD_KINDS
                .into_iter()
                .map(str::to_string)
                .collect(),
            values_first: true,
            render_required: true,
        }),
        Some(LocalCommand::PlanPvf) => Ok(LocalRouteResult::ValidationPlan {
            issue: request.issue,
            required_cards: REQUIRED_CARD_KINDS
                .into_iter()
                .map(str::to_string)
                .collect(),
            pvf_required: true,
            fail_closed: true,
        }),
        Some(LocalCommand::Doctor) => {
            let plan = prepare_local_workflow(request, registry, registrations)?;
            Ok(LocalRouteResult::Doctor {
                issue: request.issue,
                findings: plan.findings,
                ready: true,
            })
        }
        Some(LocalCommand::Schedule) => Ok(LocalRouteResult::Schedule {
            issue: request.issue,
            ordered_routes: LOCAL_ROUTE_NAMES.into_iter().map(str::to_string).collect(),
            next_route: "issue".into(),
        }),
        Some(LocalCommand::Shepherd) => {
            let bind = authorize_bind(request, registrations)?;
            Ok(LocalRouteResult::Shepherd {
                issue: request.issue,
                branch: bind.branch,
                worktree: bind.worktree,
                execution_authority: false,
                bounded_by_spp: true,
            })
        }
        Some(LocalCommand::Eligibility) => {
            let ready_to_execute = lifecycle_state
                .as_ref()
                .is_some_and(|state| state.ready_to_execute);
            Ok(LocalRouteResult::Eligibility {
                issue: request.issue,
                ready_to_execute,
                lifecycle_state,
                issue_start_minutes_max: 3,
            })
        }
        None => Err(vec![finding(
            PlanStatus::Failed,
            "unknown_local_route",
            "local route is not owned by #628",
        )]),
    }
}

fn require_lifecycle_digest(
    request: &LocalPreparationRequest,
    lifecycle_state: Option<&LocalLifecycleStateObservation>,
) -> Result<(), Vec<DoctorFinding>> {
    let Some(expected) = request.expected_lifecycle_digest.as_deref() else {
        return Ok(());
    };
    let Some(actual) = lifecycle_state.and_then(|state| state.digest.as_deref()) else {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_lifecycle_digest_missing",
            "expected lifecycle digest requires observed local lifecycle state digest",
        )]);
    };
    if actual != expected {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "stale_local_lifecycle_digest",
            "observed local lifecycle digest does not match the typed request",
        )]);
    }
    Ok(())
}

fn require_route_state(
    route: &str,
    lifecycle_state: Option<&LocalLifecycleStateObservation>,
) -> Result<(), Vec<DoctorFinding>> {
    let Some(status) = local_route_status(route, lifecycle_state) else {
        return Err(vec![finding(
            PlanStatus::Failed,
            "unknown_local_route",
            "local route is not owned by #628",
        )]);
    };
    if status.status == PlanStatus::Ready {
        Ok(())
    } else {
        Err(vec![finding(status.status, &status.code, &status.message)])
    }
}
