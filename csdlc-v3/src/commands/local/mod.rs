//! Non-authoritative V3-D local preparation workflow model.
//!
//! These types model the local issue-input → card-rendering → topology-bind →
//! doctor/PVF-plan path for V3-D. They do not mutate repository state, create
//! worktrees, execute PVF lanes, publish PRs, write GitHub state, finish,
//! cleanup, migrate v2, or grant operational authority.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use serde::{Deserialize, Serialize};
use serde_json::Value;

const REQUIRED_CARD_KINDS: [&str; 6] = ["sip", "stp", "spp", "vpp", "srp", "sor"];
pub const LOCAL_ROUTE_NAMES: [&str; 8] = [
    "issue",
    "bind",
    "edit",
    "validate",
    "doctor",
    "schedule",
    "shepherd",
    "eligibility",
];

/// Typed local commands constructed by V3-D.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum LocalCommand {
    PrepareIssue,
    BindWorktree,
    EditCards,
    PlanPvf,
    Doctor,
    Schedule,
    Shepherd,
    Eligibility,
}

/// Local preparation request accepted by the construction-only workflow.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalPreparationRequest {
    pub issue: u64,
    pub title: String,
    pub repository: String,
    pub branch: String,
    pub worktree: String,
    pub registry_version: String,
    #[serde(default)]
    pub expected_lifecycle_digest: Option<String>,
    pub commands: Vec<LocalCommand>,
    /// Values merged by the native `edit` route, keyed by card kind.
    #[serde(default)]
    pub card_updates: BTreeMap<String, Value>,
}

/// Filesystem authority supplied to the native local command executor.
///
/// The explicit roots keep writes reviewable and make root-main denial and
/// worktree containment independently testable.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperationalLocalContext {
    pub repository_root: PathBuf,
    pub state_root: PathBuf,
    pub allowed_worktree_parent: PathBuf,
    pub expected_authority_selector_digest: String,
    pub cutover_approval_path: PathBuf,
    pub expected_cutover_approval_digest: String,
    pub expected_head_sha: String,
    #[serde(default)]
    pub expected_lifecycle_digest: Option<String>,
}

#[derive(Debug, Deserialize)]
struct CutoverApprovalEvidence {
    schema: String,
    authority_issue: u64,
    repository: String,
    decision: String,
    exact_head: String,
    selector_metadata_digest: String,
}

/// Result of a native local mutation or diagnostic.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OperationalLocalResult {
    pub route: String,
    pub issue: u64,
    pub mutated: bool,
    pub phase: Option<String>,
    pub generation: Option<u64>,
    pub digest: Option<String>,
    pub next_route: Option<String>,
    pub findings: Vec<DoctorFinding>,
}

/// Exact Git worktree registration observed by a caller-owned adapter.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorktreeRegistration {
    pub branch: String,
    pub worktree: String,
    pub primary: bool,
}

/// Prompt registry observation used by the card-rendering plan.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PromptRegistry {
    pub version: String,
    pub card_kinds: BTreeSet<String>,
    pub template_paths: BTreeMap<String, String>,
}

/// The planned render set for an issue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardRenderPlan {
    pub registry_version: String,
    pub issue: u64,
    pub card_kinds: Vec<String>,
    pub rendered_cards: Vec<RenderedCard>,
}

/// One concrete, template-derived card render target.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RenderedCard {
    pub kind: String,
    pub template_ref: String,
    pub rendered_ref: String,
    pub render_manifest_digest: String,
}

/// Bind authorization is separate from command construction.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BindAuthorization {
    pub issue: u64,
    pub branch: String,
    pub worktree: String,
}

/// Distinct doctor/PVF planning states.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlanStatus {
    Ready,
    Blocked,
    Failed,
    Deferred,
    Skipped,
    Passed,
}

/// One explicit doctor/PVF finding.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorFinding {
    pub status: PlanStatus,
    pub code: String,
    pub message: String,
}

/// Read-only lifecycle-state observation for local v3 route planning.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalLifecycleStateObservation {
    pub issue: u64,
    pub phase: Option<String>,
    pub generation: Option<u64>,
    pub digest: Option<String>,
    pub status: PlanStatus,
    pub code: String,
    pub message: String,
    pub cards_present: Vec<String>,
    pub missing_cards: Vec<String>,
    pub ready_to_execute: bool,
}

/// Route-specific local command planning status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalRouteStatus {
    pub route: String,
    pub command: LocalCommand,
    pub status: PlanStatus,
    pub code: String,
    pub message: String,
    pub issue_start_minutes_max: u64,
}

/// Command-specific read-only result for one local lifecycle route.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LocalRouteResult {
    IssueInitialization {
        issue: u64,
        repository: String,
        card_paths: Vec<String>,
        template_registry_version: String,
        initialized_state: Option<LocalLifecycleStateObservation>,
    },
    BindWorktree {
        issue: u64,
        branch: String,
        worktree: String,
        primary_worktree_denied: bool,
    },
    CardEdit {
        issue: u64,
        editable_cards: Vec<String>,
        values_first: bool,
        render_required: bool,
    },
    ValidationPlan {
        issue: u64,
        required_cards: Vec<String>,
        pvf_required: bool,
        fail_closed: bool,
    },
    Doctor {
        issue: u64,
        findings: Vec<DoctorFinding>,
        ready: bool,
    },
    Schedule {
        issue: u64,
        ordered_routes: Vec<String>,
        next_route: String,
    },
    Shepherd {
        issue: u64,
        branch: String,
        worktree: String,
        execution_authority: bool,
        bounded_by_spp: bool,
    },
    Eligibility {
        issue: u64,
        ready_to_execute: bool,
        lifecycle_state: Option<LocalLifecycleStateObservation>,
        issue_start_minutes_max: u64,
    },
}

/// Non-authoritative local preparation result.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LocalPreparationPlan {
    pub issue: u64,
    pub commands: Vec<LocalCommand>,
    pub cards: CardRenderPlan,
    pub bind: BindAuthorization,
    pub findings: Vec<DoctorFinding>,
    pub lifecycle_state: Option<LocalLifecycleStateObservation>,
}

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

pub fn inspect_local_lifecycle_state(root: &Path, issue: u64) -> LocalLifecycleStateObservation {
    let issue_root = root.join(format!(".csdlc/issues/{issue}"));
    inspect_lifecycle_issue_root(&issue_root, issue, "local")
}

pub fn inspect_v3_local_state(root: &Path, issue: u64) -> LocalLifecycleStateObservation {
    let issue_root = root.join(format!("issues/{issue}"));
    inspect_lifecycle_issue_root(&issue_root, issue, "v3-local")
}

pub fn initialize_v3_local_state(
    root: &Path,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
) -> Result<LocalLifecycleStateObservation, Vec<DoctorFinding>> {
    let cards = plan_cards(request.issue, &request.registry_version, registry)?;
    let issue_root = root.join(format!("issues/{}", request.issue));
    let cards_root = issue_root.join("cards");
    fs::create_dir_all(&cards_root).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_create_failed",
            &format!("could not create v3 local state directory: {error}"),
        )]
    })?;
    let index = serde_json::json!({
        "schema": "csdlc.v3.local_state.v1",
        "issue": request.issue,
        "phase": "ready",
        "repository": request.repository,
        "branch": request.branch,
        "worktree": request.worktree,
        "template_registry_version": registry.version,
        "operational_authority": false
    });
    write_json(&issue_root.join("index.json"), &index)?;
    for card in cards.card_kinds {
        write_json(
            &cards_root.join(format!("{card}.values.json")),
            &serde_json::json!({
                "schema": "csdlc.v3.local_card_values.v1",
                "issue": request.issue,
                "card": card,
                "operational_authority": false
            }),
        )?;
        fs::write(
            cards_root.join(format!("{card}.md")),
            format!(
                "# {card}\n\nNon-authoritative v3 local preparation fixture for issue #{}.\n",
                request.issue
            ),
        )
        .map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "v3_local_card_write_failed",
                &format!("could not write v3 local card fixture: {error}"),
            )]
        })?;
    }
    Ok(inspect_v3_local_state(root, request.issue))
}

fn inspect_lifecycle_issue_root(
    issue_root: &Path,
    issue: u64,
    source: &str,
) -> LocalLifecycleStateObservation {
    let index_path = issue_root.join("index.json");
    if !index_path.is_file() {
        return LocalLifecycleStateObservation {
            issue,
            phase: None,
            generation: None,
            digest: None,
            status: PlanStatus::Blocked,
            code: "missing_local_lifecycle_state".into(),
            message: format!("{source} lifecycle state is missing; initialize or repair the issue record before executing local routes"),
            cards_present: Vec::new(),
            missing_cards: REQUIRED_CARD_KINDS.into_iter().map(str::to_string).collect(),
            ready_to_execute: false,
        };
    }
    let index = match read_local_lifecycle_index(&index_path) {
        Ok(index) => index,
        Err(message) => {
            return LocalLifecycleStateObservation {
                issue,
                phase: None,
                generation: None,
                digest: None,
                status: PlanStatus::Blocked,
                code: "invalid_local_lifecycle_state".into(),
                message,
                cards_present: Vec::new(),
                missing_cards: REQUIRED_CARD_KINDS
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                ready_to_execute: false,
            };
        }
    };
    let phase = index.phase.clone();
    let mut cards_present = Vec::new();
    let mut missing_cards = Vec::new();
    for card in REQUIRED_CARD_KINDS {
        if issue_root
            .join("cards")
            .join(format!("{card}.values.json"))
            .is_file()
            && issue_root
                .join("cards")
                .join(format!("{card}.md"))
                .is_file()
        {
            cards_present.push(card.to_owned());
        } else {
            missing_cards.push(card.to_owned());
        }
    }
    if missing_cards.is_empty() && index.operational_authority {
        let index_value = match fs::read(&index_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        {
            Some(value) => value,
            None => {
                return invalid_lifecycle_observation(
                    issue,
                    "local lifecycle index could not be recomputed",
                );
            }
        };
        let computed = match lifecycle_digest(issue_root, &index_value) {
            Ok(digest) => digest,
            Err(_) => {
                return invalid_lifecycle_observation(
                    issue,
                    "local lifecycle digest could not be recomputed from index and cards",
                );
            }
        };
        if index.digest.as_deref() != Some(computed.as_str()) {
            return invalid_lifecycle_observation(
                issue,
                "stored lifecycle digest does not match the current index and card bytes",
            );
        }
    }
    if !matches!(phase.as_str(), "ready" | "bound") {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase.clone()),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Blocked,
            code: "unsupported_local_lifecycle_phase".into(),
            message: format!(
                "local lifecycle state is in phase {phase}; local execution readiness requires ready or bound"
            ),
            cards_present,
            missing_cards,
            ready_to_execute: false,
        }
    } else if missing_cards.is_empty() {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase.clone()),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Ready,
            code: "local_lifecycle_state_ready".into(),
            message: format!(
                "local lifecycle state is {phase} and contains the six-card denominator"
            ),
            cards_present,
            missing_cards,
            ready_to_execute: true,
        }
    } else {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Blocked,
            code: "missing_lifecycle_cards".into(),
            message: "local lifecycle state exists but one or more lifecycle cards are missing"
                .into(),
            cards_present,
            missing_cards,
            ready_to_execute: false,
        }
    }
}

fn invalid_lifecycle_observation(issue: u64, message: &str) -> LocalLifecycleStateObservation {
    LocalLifecycleStateObservation {
        issue,
        phase: None,
        generation: None,
        digest: None,
        status: PlanStatus::Blocked,
        code: "invalid_local_lifecycle_digest".into(),
        message: message.into(),
        cards_present: Vec::new(),
        missing_cards: Vec::new(),
        ready_to_execute: false,
    }
}

fn write_json(path: &Path, value: &Value) -> Result<(), Vec<DoctorFinding>> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_serialize_failed",
            &error.to_string(),
        )]
    })?;
    fs::write(path, bytes).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_write_failed",
            &format!("could not write v3 local state: {error}"),
        )]
    })
}

struct LocalLifecycleIndex {
    phase: String,
    generation: Option<u64>,
    digest: Option<String>,
    operational_authority: bool,
}

fn read_local_lifecycle_index(index_path: &Path) -> Result<LocalLifecycleIndex, String> {
    let bytes = fs::read(index_path)
        .map_err(|err| format!("local lifecycle index could not be read: {err}"))?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|err| format!("local lifecycle index is not valid JSON: {err}"))?;
    let phase = value
        .get("phase")
        .and_then(Value::as_str)
        .filter(|phase| !phase.trim().is_empty())
        .map(str::to_string)
        .ok_or_else(|| "local lifecycle index is missing nonempty phase".to_string())?;
    Ok(LocalLifecycleIndex {
        phase,
        generation: value.get("generation").and_then(Value::as_u64),
        digest: value
            .get("digest")
            .and_then(Value::as_str)
            .filter(|digest| !digest.trim().is_empty())
            .map(str::to_string),
        operational_authority: value
            .get("operational_authority")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}

/// Execute one local route with native v3 state and Git operations.
///
/// This entry point deliberately has no v2 subprocess fallback. Callers may
/// expose it only after the v3 writer fence is active.
pub fn execute_operational_local_route(
    route: &str,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    validate_contract(request)?;
    plan_cards(request.issue, &request.registry_version, registry)?;
    validate_context(request, context)?;
    let issue_root = context.state_root.join(format!("issues/{}", request.issue));
    let before = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
    require_operational_cas(route, request, &before)?;

    match route {
        "issue" => initialize_operational_issue(request, registry, &issue_root),
        "bind" => bind_operational_issue(request, context, &issue_root),
        "edit" => edit_operational_cards(request, registry, &issue_root),
        "validate" => validate_operational_issue(request, registry, &issue_root),
        "doctor" | "eligibility" => diagnose_operational_issue(route, request, registry, context),
        "schedule" | "shepherd" => route_operational_issue(route, request, registry, context),
        _ => Err(vec![finding(
            PlanStatus::Failed,
            "unknown_local_route",
            "unknown native v3 local route",
        )]),
    }
}

fn validate_context(
    request: &LocalPreparationRequest,
    context: &OperationalLocalContext,
) -> Result<(), Vec<DoctorFinding>> {
    let repository_root = context.repository_root.canonicalize().map_err(|_| {
        vec![finding(
            PlanStatus::Failed,
            "repository_root_unavailable",
            "operational repository root must be an existing canonical directory",
        )]
    })?;
    let state_root = &context.state_root;
    if state_root.components().any(|component| {
        matches!(
            component,
            std::path::Component::ParentDir | std::path::Component::Prefix(_)
        )
    }) || !state_root.starts_with(repository_root.join(".csdlc"))
        || !canonical_existing_ancestor_local(state_root)
            .is_some_and(|ancestor| ancestor.starts_with(&repository_root))
    {
        return Err(vec![finding(
            PlanStatus::Failed,
            "state_root_outside_repository",
            "operational state root must resolve beneath the repository .csdlc directory",
        )]);
    }
    if repository_root == context.allowed_worktree_parent
        || repository_root.starts_with(&context.allowed_worktree_parent)
    {
        return Err(vec![finding(
            PlanStatus::Failed,
            "invalid_operational_roots",
            "repository root and worktree parent must be distinct",
        )]);
    }
    if request.expected_lifecycle_digest != context.expected_lifecycle_digest {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "operational_lifecycle_digest_mismatch",
            "serialized operational context and typed request must bind the same lifecycle digest",
        )]);
    }

    let selector_path = context
        .repository_root
        .join("csdlc-v2/operator/generation-selector.json");
    let selector_bytes = fs::read(&selector_path).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_v3_authority_missing",
            "canonical generation selector is unavailable; v3 local mutations remain denied",
        )]
    })?;
    let selector_digest = blake3::hash(&selector_bytes).to_hex().to_string();
    if context.expected_authority_selector_digest.trim().is_empty()
        || context.expected_authority_selector_digest != selector_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_digest_mismatch",
            "serialized authority digest does not match the canonical generation selector",
        )]);
    }
    let selector: Value = serde_json::from_slice(&selector_bytes).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_invalid",
            "canonical generation selector must be typed JSON",
        )]
    })?;
    if !matches!(
        selector.get("schema").and_then(Value::as_str),
        Some("csdlc.generation_selector.v1" | "csdlc.generation_selector.v2")
    ) || selector.get("default_generation").and_then(Value::as_str) != Some("v3")
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "canonical_v3_authority_inactive",
            "canonical generation selector does not grant v3 operational authority",
        )]);
    }

    let approval_path = &context.cutover_approval_path;
    if approval_path.is_absolute()
        || !approval_path.starts_with(Path::new(".csdlc/evidence/505"))
        || approval_path.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir
                    | std::path::Component::RootDir
                    | std::path::Component::Prefix(_)
            )
        })
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_approval_path_not_canonical",
            "cutover approval evidence must be a repository-relative #505 evidence path",
        )]);
    }
    let approval_evidence_root = repository_root
        .join(".csdlc/evidence/505")
        .canonicalize()
        .map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "cutover_approval_missing",
                "canonical #505 approval evidence directory is unavailable",
            )]
        })?;
    let approval_file = repository_root
        .join(approval_path)
        .canonicalize()
        .map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "cutover_approval_missing",
                "digest-bound #505 cutover approval evidence is unavailable",
            )]
        })?;
    if !approval_file.starts_with(&approval_evidence_root) || !approval_file.is_file() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_approval_path_not_canonical",
            "cutover approval evidence must resolve inside the canonical #505 evidence directory",
        )]);
    }
    let approval_bytes =
        fs::read(&approval_file).map_err(io_finding("cutover_approval_unreadable"))?;
    let approval_digest = blake3::hash(&approval_bytes).to_hex().to_string();
    if context.expected_cutover_approval_digest.trim().is_empty()
        || context.expected_cutover_approval_digest != approval_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_approval_digest_mismatch",
            "serialized approval digest does not match retained #505 evidence",
        )]);
    }
    let approval: CutoverApprovalEvidence =
        serde_json::from_slice(&approval_bytes).map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "cutover_approval_invalid",
                "cutover approval evidence must be typed JSON",
            )]
        })?;
    let expected_head = context.expected_head_sha.trim();
    let selector_is_v2 =
        selector.get("schema").and_then(Value::as_str) == Some("csdlc.generation_selector.v2");
    let selector_authority_exact = selector
        .get("operational_authority")
        .and_then(Value::as_str)
        == Some("csdlc-v3")
        && selector.get("authority_issue").and_then(Value::as_u64) == Some(505)
        && selector.get("exact_review_sha").and_then(Value::as_str) == Some(expected_head)
        && selector
            .get("approval_evidence_digest")
            .and_then(Value::as_str)
            == Some(approval_digest.as_str());
    if approval.schema != "csdlc.v3.cutover_approval.v1"
        || approval.authority_issue != 505
        || approval.repository != request.repository
        || approval.decision != "approved"
        || approval.exact_head != expected_head
        || (!selector_is_v2 && approval.selector_metadata_digest != selector_digest)
        || (selector_is_v2 && !selector_authority_exact)
        || expected_head.len() != 40
        || !expected_head.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_approval_not_exact",
            "typed #505 approval must bind repository, exact head, and canonical selector digest",
        )]);
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(&repository_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(io_finding("git_head_observation_failed"))?;
    if !head.status.success() || String::from_utf8_lossy(&head.stdout).trim() != expected_head {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "operational_exact_head_mismatch",
            "live repository HEAD does not match digest-bound #505 approval evidence",
        )]);
    }

    fs::create_dir_all(&context.state_root).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "state_root_create_failed",
            &error.to_string(),
        )]
    })
}

fn require_operational_cas(
    route: &str,
    request: &LocalPreparationRequest,
    observed: &LocalLifecycleStateObservation,
) -> Result<(), Vec<DoctorFinding>> {
    if observed.code == "missing_local_lifecycle_state" {
        if route == "issue" && request.expected_lifecycle_digest.is_none() {
            return Ok(());
        }
        return Err(vec![finding(
            PlanStatus::Blocked,
            "missing_local_lifecycle_state",
            "only issue initialization may operate without existing lifecycle state",
        )]);
    }
    let expected = request
        .expected_lifecycle_digest
        .as_deref()
        .ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_lifecycle_digest_required",
                "an existing lifecycle record requires expected_lifecycle_digest",
            )]
        })?;
    if observed.digest.as_deref() != Some(expected) {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "stale_local_lifecycle_digest",
            "observed local lifecycle digest does not match the typed request",
        )]);
    }
    Ok(())
}

fn initialize_operational_issue(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if issue_root.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "issue_already_initialized",
            "native issue initialization is create-only",
        )]);
    }
    let cards_root = issue_root.join("cards");
    fs::create_dir_all(&cards_root).map_err(io_finding("issue_state_create_failed"))?;
    for kind in REQUIRED_CARD_KINDS {
        let values = initial_card_values(request, registry, kind);
        let template_path = registry
            .template_paths
            .get(kind)
            .expect("registry validated");
        let template =
            fs::read_to_string(template_path).map_err(io_finding("template_read_failed"))?;
        let rendered = render_template(&template, &values);
        atomic_write_json(&cards_root.join(format!("{kind}.values.json")), &values)?;
        atomic_write(&cards_root.join(format!("{kind}.md")), rendered.as_bytes())?;
    }
    let (generation, digest) = persist_index(issue_root, request, registry, "ready", 1)?;
    Ok(operational_result(
        "issue",
        request.issue,
        true,
        "ready",
        generation,
        digest,
        Some("bind"),
        vec![],
    ))
}

fn initial_card_values(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    kind: &str,
) -> Value {
    serde_json::json!({
        "schema": "csdlc.v3.card_values.v1",
        "issue": request.issue,
        "issue_padded": format!("{:04}", request.issue),
        "issue_url": format!("https://github.com/{}/issues/{}", request.repository, request.issue),
        "title": request.title,
        "branch": request.branch,
        "version": registry.version,
        "card": kind,
        "card_status": "ready",
        "repository": request.repository,
        "worktree": request.worktree
    })
}

fn bind_operational_issue(
    request: &LocalPreparationRequest,
    context: &OperationalLocalContext,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    if observed.phase.as_deref() != Some("ready") {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bind_phase_invalid",
            "bind requires ready lifecycle state",
        )]);
    }
    let target = PathBuf::from(&request.worktree);
    let allowed_parent = context
        .allowed_worktree_parent
        .canonicalize()
        .map_err(|_| {
            vec![finding(
                PlanStatus::Blocked,
                "worktree_parent_unavailable",
                "allowed worktree parent must be an existing canonical directory",
            )]
        })?;
    let target_parent_inside_policy = target
        .parent()
        .and_then(canonical_existing_ancestor_local)
        .is_some_and(|ancestor| ancestor.starts_with(&allowed_parent));
    if !target.is_absolute()
        || target.components().any(|component| {
            matches!(
                component,
                std::path::Component::ParentDir | std::path::Component::Prefix(_)
            )
        })
        || !target.starts_with(&context.allowed_worktree_parent)
        || !target_parent_inside_policy
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "worktree_outside_policy",
            "operational bind requires an absolute path below the allowed worktree parent",
        )]);
    }
    if target == context.repository_root {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "primary_worktree_denied",
            "bind cannot target the primary checkout",
        )]);
    }
    let registration =
        git_worktree_registration(&context.repository_root, &request.branch, &target)?;
    if !registration {
        if target.exists() {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "unregistered_worktree_exists",
                "target path exists but is not registered for the requested branch",
            )]);
        }
        let output = Command::new("git")
            .arg("-C")
            .arg(&context.repository_root)
            .args(["worktree", "add", "-b"])
            .arg(&request.branch)
            .arg(&target)
            .arg("HEAD")
            .output()
            .map_err(io_finding("git_worktree_add_failed"))?;
        if !output.status.success() {
            return Err(vec![finding(
                PlanStatus::Failed,
                "git_worktree_add_failed",
                String::from_utf8_lossy(&output.stderr).trim(),
            )]);
        }
    }
    atomic_write_json(
        &issue_root.join("binding.json"),
        &serde_json::json!({
            "schema": "csdlc.v3.binding.v1",
            "issue": request.issue,
            "branch": request.branch,
            "worktree": request.worktree
        }),
    )?;
    let index = read_index_value(issue_root)?;
    let registry_version = index["template_registry_version"]
        .as_str()
        .unwrap_or(&request.registry_version);
    let registry = PromptRegistry {
        version: registry_version.to_owned(),
        card_kinds: BTreeSet::new(),
        template_paths: BTreeMap::new(),
    };
    let generation = observed.generation.unwrap_or(1) + 1;
    let (generation, digest) = persist_index(issue_root, request, &registry, "bound", generation)?;
    Ok(operational_result(
        "bind",
        request.issue,
        true,
        "bound",
        generation,
        digest,
        Some("edit"),
        vec![],
    ))
}

fn canonical_existing_ancestor_local(path: &Path) -> Option<PathBuf> {
    let mut candidate = path;
    loop {
        if let Ok(canonical) = candidate.canonicalize() {
            return Some(canonical);
        }
        candidate = candidate.parent()?;
    }
}

fn edit_operational_cards(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if request.card_updates.is_empty() {
        return Err(vec![finding(
            PlanStatus::Failed,
            "card_updates_missing",
            "edit requires at least one typed card update",
        )]);
    }
    let mut prepared = Vec::new();
    for (kind, update) in &request.card_updates {
        if !REQUIRED_CARD_KINDS.contains(&kind.as_str()) || !update.is_object() {
            return Err(vec![finding(
                PlanStatus::Failed,
                "card_update_invalid",
                "card updates must be objects keyed by canonical card kind",
            )]);
        }
        let values_path = issue_root.join(format!("cards/{kind}.values.json"));
        let mut values: Value = serde_json::from_slice(
            &fs::read(&values_path).map_err(io_finding("card_values_read_failed"))?,
        )
        .map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "card_values_invalid",
                &error.to_string(),
            )]
        })?;
        merge_json_object(&mut values, update);
        let template = fs::read_to_string(
            registry
                .template_paths
                .get(kind)
                .expect("registry validated"),
        )
        .map_err(io_finding("template_read_failed"))?;
        let rendered_path = issue_root.join(format!("cards/{kind}.md"));
        let old_values = fs::read(&values_path).map_err(io_finding("card_values_read_failed"))?;
        let old_rendered =
            fs::read(&rendered_path).map_err(io_finding("card_render_read_failed"))?;
        let new_values = serde_json::to_vec_pretty(&values).map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "card_values_serialize_failed",
                &error.to_string(),
            )]
        })?;
        let new_rendered = render_template(&template, &values).into_bytes();
        prepared.push((values_path, old_values, new_values));
        prepared.push((rendered_path, old_rendered, new_rendered));
    }
    for (applied, (path, _, bytes)) in prepared.iter().enumerate() {
        if let Err(findings) = atomic_write(path, bytes) {
            for (rollback_path, old_bytes, _) in prepared[..applied].iter().rev() {
                let _ = atomic_write(rollback_path, old_bytes);
            }
            return Err(findings);
        }
    }
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    let phase = observed.phase.as_deref().unwrap_or("ready");
    let persisted = persist_index(
        issue_root,
        request,
        registry,
        phase,
        observed.generation.unwrap_or(1) + 1,
    );
    let (generation, digest) = match persisted {
        Ok(result) => result,
        Err(findings) => {
            for (path, old_bytes, _) in prepared.iter().rev() {
                let _ = atomic_write(path, old_bytes);
            }
            return Err(findings);
        }
    };
    Ok(operational_result(
        "edit",
        request.issue,
        true,
        phase,
        generation,
        digest,
        Some("validate"),
        vec![],
    ))
}

fn validate_operational_issue(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let findings = validation_findings(request, registry, issue_root);
    if findings
        .iter()
        .any(|item| item.status != PlanStatus::Passed)
    {
        return Err(findings);
    }
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    Ok(operational_result(
        "validate",
        request.issue,
        false,
        observed.phase.as_deref().unwrap_or("unknown"),
        observed.generation.unwrap_or(0),
        observed.digest.unwrap_or_default(),
        Some(if observed.phase.as_deref() == Some("bound") {
            "shepherd"
        } else {
            "bind"
        }),
        findings,
    ))
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
    let blocked = result
        .findings
        .iter()
        .any(|item| matches!(item.status, PlanStatus::Blocked | PlanStatus::Failed));
    result.next_route = if blocked {
        Some("doctor".into())
    } else if result.phase.as_deref() == Some("ready") {
        Some("bind".into())
    } else if request.card_updates.is_empty() {
        Some("edit".into())
    } else {
        Some("validate".into())
    };
    Ok(result)
}

fn validation_findings(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Vec<DoctorFinding> {
    let mut findings = Vec::new();
    let index = match read_index_value(issue_root) {
        Ok(index) => index,
        Err(mut errors) => {
            return {
                findings.append(&mut errors);
                findings
            }
        }
    };
    let recorded_digest = index.get("digest").and_then(Value::as_str).unwrap_or("");
    match lifecycle_digest(issue_root, &index) {
        Ok(actual) if actual == recorded_digest => findings.push(finding(
            PlanStatus::Passed,
            "lifecycle_digest_valid",
            "lifecycle digest matches canonical state",
        )),
        Ok(_) => findings.push(finding(
            PlanStatus::Blocked,
            "lifecycle_digest_mismatch",
            "lifecycle digest does not match canonical state",
        )),
        Err(mut errors) => findings.append(&mut errors),
    }
    for kind in REQUIRED_CARD_KINDS {
        let values_path = issue_root.join(format!("cards/{kind}.values.json"));
        let rendered_path = issue_root.join(format!("cards/{kind}.md"));
        let values: Value = match fs::read(&values_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        {
            Some(value) if value.is_object() => value,
            _ => {
                findings.push(finding(
                    PlanStatus::Blocked,
                    "card_values_invalid",
                    &format!("{kind} values are missing or invalid"),
                ));
                continue;
            }
        };
        let template = match registry
            .template_paths
            .get(kind)
            .and_then(|path| fs::read_to_string(path).ok())
        {
            Some(template) => template,
            None => {
                findings.push(finding(
                    PlanStatus::Blocked,
                    "template_read_failed",
                    &format!("{kind} template is unavailable"),
                ));
                continue;
            }
        };
        let expected = render_template(&template, &values);
        let actual = fs::read_to_string(&rendered_path).unwrap_or_default();
        if expected != actual {
            findings.push(finding(
                PlanStatus::Blocked,
                "rendered_card_drift",
                &format!("{kind} rendered card differs from typed values"),
            ));
            continue;
        }
        if !structure_valid(registry, kind, &actual) {
            findings.push(finding(
                PlanStatus::Blocked,
                "card_structure_invalid",
                &format!("{kind} rendered card violates its structure schema"),
            ));
        }
    }
    if findings.len() == 1 && findings[0].status == PlanStatus::Passed {
        findings.push(finding(
            PlanStatus::Passed,
            "six_card_validation_passed",
            &format!(
                "issue #{} has valid values, renders, structures, and digest",
                request.issue
            ),
        ));
    }
    findings
}

fn structure_valid(registry: &PromptRegistry, kind: &str, markdown: &str) -> bool {
    let Some(template_path) = registry.template_paths.get(kind) else {
        return false;
    };
    let schema_path = PathBuf::from(template_path)
        .parent()
        .and_then(Path::parent)
        .map(|root| root.join(format!("schemas/{kind}.structure.json")));
    let Some(schema_path) = schema_path else {
        return false;
    };
    let Ok(bytes) = fs::read(schema_path) else {
        return false;
    };
    let Ok(schema) = serde_json::from_slice::<Value>(&bytes) else {
        return false;
    };
    schema
        .get("scaffold_lines")
        .and_then(Value::as_array)
        .is_some_and(|lines| {
            lines
                .iter()
                .filter_map(Value::as_str)
                .all(|line| markdown.lines().any(|candidate| candidate.trim() == line))
        })
}

fn persist_index(
    issue_root: &Path,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    phase: &str,
    generation: u64,
) -> Result<(u64, String), Vec<DoctorFinding>> {
    let mut index = serde_json::json!({
        "schema": "csdlc.v3.local_state.v1",
        "issue": request.issue,
        "phase": phase,
        "generation": generation,
        "repository": request.repository,
        "branch": request.branch,
        "worktree": request.worktree,
        "template_registry_version": registry.version,
        "operational_authority": true
    });
    let digest = lifecycle_digest(issue_root, &index)?;
    index["digest"] = Value::String(digest.clone());
    atomic_write_json(&issue_root.join("index.json"), &index)?;
    Ok((generation, digest))
}

fn lifecycle_digest(issue_root: &Path, index: &Value) -> Result<String, Vec<DoctorFinding>> {
    let mut canonical_index = index.clone();
    canonical_index
        .as_object_mut()
        .map(|object| object.remove("digest"));
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serde_json::to_vec(&canonical_index).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "index_serialize_failed",
            &error.to_string(),
        )]
    })?);
    for kind in REQUIRED_CARD_KINDS {
        for suffix in ["values.json", "md"] {
            let path = issue_root.join(format!("cards/{kind}.{suffix}"));
            hasher.update(&fs::read(path).map_err(io_finding("card_read_failed"))?);
        }
    }
    let binding = issue_root.join("binding.json");
    if binding.is_file() {
        hasher.update(&fs::read(binding).map_err(io_finding("binding_read_failed"))?);
    }
    Ok(hasher.finalize().to_hex().to_string())
}

fn read_index_value(issue_root: &Path) -> Result<Value, Vec<DoctorFinding>> {
    serde_json::from_slice(
        &fs::read(issue_root.join("index.json")).map_err(io_finding("index_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "index_invalid",
            &error.to_string(),
        )]
    })
}

fn render_template(template: &str, values: &Value) -> String {
    let mut output = template.to_owned();
    if let Some(object) = values.as_object() {
        for (key, value) in object {
            let rendered = value
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| value.to_string());
            output = output.replace(&format!("<{key}>"), &rendered);
        }
    }
    output
}

fn merge_json_object(target: &mut Value, update: &Value) {
    if let (Some(target), Some(update)) = (target.as_object_mut(), update.as_object()) {
        for (key, value) in update {
            target.insert(key.clone(), value.clone());
        }
    }
}

fn git_worktree_registration(
    repo: &Path,
    branch: &str,
    target: &Path,
) -> Result<bool, Vec<DoctorFinding>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(["worktree", "list", "--porcelain"])
        .output()
        .map_err(io_finding("git_worktree_list_failed"))?;
    if !output.status.success() {
        return Err(vec![finding(
            PlanStatus::Failed,
            "git_worktree_list_failed",
            String::from_utf8_lossy(&output.stderr).trim(),
        )]);
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let branch_ref = format!("refs/heads/{branch}");
    Ok(text.split("\n\n").any(|record| {
        record
            .lines()
            .any(|line| line == format!("worktree {}", target.display()))
            && record
                .lines()
                .any(|line| line == format!("branch {branch_ref}"))
    }))
}

fn atomic_write_json(path: &Path, value: &Value) -> Result<(), Vec<DoctorFinding>> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "json_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write(path, &bytes)
}

fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), Vec<DoctorFinding>> {
    let parent = path.parent().ok_or_else(|| {
        vec![finding(
            PlanStatus::Failed,
            "write_parent_missing",
            "target has no parent",
        )]
    })?;
    fs::create_dir_all(parent).map_err(io_finding("write_parent_create_failed"))?;
    let temporary = parent.join(format!(
        ".{}.tmp-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("state"),
        std::process::id()
    ));
    fs::write(&temporary, bytes).map_err(io_finding("temporary_write_failed"))?;
    fs::rename(&temporary, path).map_err(io_finding("atomic_rename_failed"))
}

fn io_finding(code: &'static str) -> impl FnOnce(std::io::Error) -> Vec<DoctorFinding> {
    move |error| vec![finding(PlanStatus::Failed, code, &error.to_string())]
}

#[allow(clippy::too_many_arguments)]
fn operational_result(
    route: &str,
    issue: u64,
    mutated: bool,
    phase: &str,
    generation: u64,
    digest: String,
    next_route: Option<&str>,
    findings: Vec<DoctorFinding>,
) -> OperationalLocalResult {
    OperationalLocalResult {
        route: route.into(),
        issue,
        mutated,
        phase: Some(phase.into()),
        generation: Some(generation),
        digest: Some(digest),
        next_route: next_route.map(str::to_string),
        findings,
    }
}

pub fn finding(status: PlanStatus, code: &str, message: &str) -> DoctorFinding {
    DoctorFinding {
        status,
        code: code.into(),
        message: message.into(),
    }
}
