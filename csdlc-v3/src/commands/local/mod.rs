//! Non-authoritative V3-D local preparation workflow model.
//!
//! These types model the local issue-input → card-rendering → topology-bind →
//! doctor/PVF-plan path for V3-D. They do not mutate repository state, create
//! worktrees, execute PVF lanes, publish PRs, write GitHub state, finish,
//! cleanup, migrate v2, or grant operational authority.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicU64, Ordering},
};

use fs2::FileExt;
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
    /// Six-dimensional v2-compatible readiness denominator for `schedule`.
    #[serde(default)]
    pub schedule_readiness: Option<ScheduleReadinessInput>,
    /// v2-compatible state-routing denominator for `shepherd`.
    #[serde(default)]
    pub shepherd_routing: Option<ShepherdRoutingInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ScheduleReadinessInput {
    pub phase_ready: bool,
    pub cards_ready: bool,
    pub design_ready: bool,
    pub dependencies_ready: bool,
    pub paths_clear: bool,
    pub budget_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ShepherdRoutingInput {
    #[serde(default)]
    pub validation: Option<String>,
    #[serde(default)]
    pub dependency_wait: bool,
    #[serde(default)]
    pub retryable_failure: bool,
    #[serde(default)]
    pub repair_needed: bool,
    #[serde(default)]
    pub operator_decision_needed: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct OperationalRoutingReport {
    pub schema: String,
    pub state: String,
    pub blockers: Vec<String>,
    pub eligible_operations: Vec<String>,
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

pub fn discover_operational_local_context(
    repository_root: &Path,
    request: &LocalPreparationRequest,
) -> Result<Option<OperationalLocalContext>, Vec<DoctorFinding>> {
    let repository_root = repository_root.canonicalize().map_err(|_| {
        vec![finding(
            PlanStatus::Failed,
            "repository_root_unavailable",
            "operational repository root must be an existing canonical directory",
        )]
    })?;
    let selector_path = repository_root.join("csdlc-v2/operator/generation-selector.json");
    let selector_bytes = match fs::read(&selector_path) {
        Ok(bytes) => bytes,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(error) => {
            return Err(vec![finding(
                PlanStatus::Failed,
                "canonical_authority_selector_unreadable",
                &error.to_string(),
            )])
        }
    };
    let selector: Value = serde_json::from_slice(&selector_bytes).map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "canonical_authority_selector_invalid",
            "canonical generation selector must be typed JSON",
        )]
    })?;
    if selector.get("schema").and_then(Value::as_str) != Some("csdlc.generation_selector.v2")
        || selector.get("default_generation").and_then(Value::as_str) != Some("v3")
        || selector
            .get("operational_authority")
            .and_then(Value::as_str)
            != Some("csdlc-v3")
    {
        return Ok(None);
    }
    if crate::authority::canonical_v3_authority(&repository_root)
        .map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_invalid",
                &error,
            )]
        })?
        .is_none()
    {
        return Ok(None);
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(&repository_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(io_finding("repository_head_unreadable"))?;
    if !head.status.success() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "repository_head_unreadable",
            "operational context requires an exact checkout head",
        )]);
    }
    let expected_head_sha = String::from_utf8_lossy(&head.stdout).trim().to_owned();
    let approval_path = PathBuf::from(crate::authority::SELECTOR_PATH);
    let approval_digest = blake3::hash(&selector_bytes).to_hex().to_string();
    let worktree_policy: Value = serde_json::from_slice(
        &fs::read(repository_root.join(".adl/worktree-policy.json"))
            .map_err(io_finding("worktree_policy_unreadable"))?,
    )
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "worktree_policy_invalid",
            "tracked worktree policy must be typed JSON",
        )]
    })?;
    if worktree_policy["schema"] != "adl.worktree_policy.v1" {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "worktree_policy_invalid",
            "tracked worktree policy schema is not supported",
        )]);
    }
    let allowed_worktree_parent = PathBuf::from(
        worktree_policy["required_parent"]
            .as_str()
            .filter(|value| !value.trim().is_empty())
            .ok_or_else(|| {
                vec![finding(
                    PlanStatus::Blocked,
                    "worktree_policy_parent_missing",
                    "tracked worktree policy must declare required_parent",
                )]
            })?,
    )
    .canonicalize()
    .map_err(|_| {
        vec![finding(
            PlanStatus::Blocked,
            "worktree_parent_unavailable",
            "operational worktree parent must already exist",
        )]
    })?;
    Ok(Some(OperationalLocalContext {
        state_root: repository_root.join(".csdlc"),
        allowed_worktree_parent,
        expected_authority_selector_digest: blake3::hash(&selector_bytes).to_hex().to_string(),
        cutover_approval_path: approval_path,
        expected_cutover_approval_digest: approval_digest,
        expected_head_sha,
        expected_lifecycle_digest: request.expected_lifecycle_digest.clone(),
        repository_root,
    }))
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing: Option<OperationalRoutingReport>,
    pub findings: Vec<DoctorFinding>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LocalMutationJournal {
    schema: String,
    issue: u64,
    route: String,
    request_digest: String,
    result: OperationalLocalResult,
    #[serde(default)]
    bind_branch: Option<String>,
    #[serde(default)]
    bind_worktree: Option<PathBuf>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct LocalMutationCompletion {
    schema: String,
    issue: u64,
    route: String,
    request_digest: String,
    result: OperationalLocalResult,
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
    validate_context(request, context)?;
    let _issue_lock = acquire_issue_mutation_lock(&context.state_root, request.issue)?;
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

fn acquire_issue_mutation_lock(
    state_root: &Path,
    issue: u64,
) -> Result<fs::File, Vec<DoctorFinding>> {
    let lock_root = state_root.join("locks");
    fs::create_dir_all(&lock_root).map_err(io_finding("issue_lock_parent_create_failed"))?;
    let lock_path = lock_root.join(format!("{issue}.lock"));
    if lock_path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err(vec![finding(
            PlanStatus::Failed,
            "issue_lock_symlink_denied",
            "issue mutation lock must not be a symlink",
        )]);
    }
    let file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(lock_path)
        .map_err(io_finding("issue_lock_open_failed"))?;
    file.lock_exclusive()
        .map_err(io_finding("issue_lock_acquire_failed"))?;
    Ok(file)
}

fn local_request_digest(request: &LocalPreparationRequest) -> Result<String, Vec<DoctorFinding>> {
    serde_json::to_vec(request)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
        .map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "local_request_digest_failed",
                &error.to_string(),
            )]
        })
}

fn local_transaction_root(context: &OperationalLocalContext) -> PathBuf {
    context.state_root.join("transactions")
}

fn local_transaction_journal_path(context: &OperationalLocalContext, issue: u64) -> PathBuf {
    local_transaction_root(context).join(format!("{issue}.json"))
}

fn local_transaction_paths(
    context: &OperationalLocalContext,
    issue: u64,
    route: &str,
    request_digest: &str,
) -> (PathBuf, PathBuf, PathBuf) {
    let issue_parent = context.state_root.join("issues");
    let suffix = format!(".issue-{issue}-{route}-{request_digest}");
    let stage = issue_parent.join(format!("{suffix}.stage"));
    let backup = issue_parent.join(format!("{suffix}.backup"));
    let completion = local_transaction_root(context)
        .join("completed")
        .join(issue.to_string())
        .join(format!("{route}-{request_digest}.json"));
    (stage, backup, completion)
}

fn prepare_local_transaction_stage(
    context: &OperationalLocalContext,
    issue: u64,
    route: &str,
    request_digest: &str,
) -> Result<PathBuf, Vec<DoctorFinding>> {
    let issue_root = context.state_root.join(format!("issues/{issue}"));
    let (stage, backup, _) = local_transaction_paths(context, issue, route, request_digest);
    if backup.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_orphaned_backup",
            "an unjournaled lifecycle backup requires operator recovery",
        )]);
    }
    if stage.exists() {
        fs::remove_dir_all(&stage).map_err(io_finding("local_transaction_stage_cleanup_failed"))?;
    }
    copy_local_issue_tree(&issue_root, &stage)?;
    Ok(stage)
}

fn copy_local_issue_tree(source: &Path, destination: &Path) -> Result<(), Vec<DoctorFinding>> {
    let metadata = source
        .symlink_metadata()
        .map_err(io_finding("local_transaction_source_unavailable"))?;
    if !metadata.is_dir() || metadata.file_type().is_symlink() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_source_invalid",
            "lifecycle transaction source must be a real directory",
        )]);
    }
    fs::create_dir(destination).map_err(io_finding("local_transaction_stage_create_failed"))?;
    for entry in fs::read_dir(source).map_err(io_finding("local_transaction_source_read_failed"))? {
        let entry = entry.map_err(io_finding("local_transaction_entry_read_failed"))?;
        let file_type = entry
            .file_type()
            .map_err(io_finding("local_transaction_entry_metadata_failed"))?;
        if file_type.is_symlink() {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_symlink_denied",
                "lifecycle transaction images cannot contain symlinks",
            )]);
        }
        let target = destination.join(entry.file_name());
        if file_type.is_dir() {
            copy_local_issue_tree(&entry.path(), &target)?;
        } else if file_type.is_file() {
            fs::copy(entry.path(), target)
                .map_err(io_finding("local_transaction_file_copy_failed"))?;
        } else {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_special_file_denied",
                "lifecycle transaction images may contain only files and directories",
            )]);
        }
    }
    Ok(())
}

fn begin_local_transaction(
    context: &OperationalLocalContext,
    journal: LocalMutationJournal,
) -> Result<(), Vec<DoctorFinding>> {
    let journal_path = local_transaction_journal_path(context, journal.issue);
    if journal_path.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_already_pending",
            "a lifecycle mutation journal must be recovered before another mutation starts",
        )]);
    }
    fs::create_dir_all(
        journal_path
            .parent()
            .expect("transaction journal has a parent"),
    )
    .map_err(io_finding("local_transaction_root_create_failed"))?;
    let value = serde_json::to_value(&journal).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "local_transaction_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write_json(&journal_path, &value)
}

fn recover_pending_local_transaction(
    context: &OperationalLocalContext,
    issue: u64,
) -> Result<(), Vec<DoctorFinding>> {
    let journal_path = local_transaction_journal_path(context, issue);
    if !journal_path.exists() {
        return Ok(());
    }
    if journal_path
        .symlink_metadata()
        .is_ok_and(|metadata| !metadata.is_file() || metadata.file_type().is_symlink())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_invalid",
            "lifecycle mutation journal must be a regular file",
        )]);
    }
    let journal: LocalMutationJournal = serde_json::from_slice(
        &fs::read(&journal_path).map_err(io_finding("local_transaction_journal_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_invalid",
            &error.to_string(),
        )]
    })?;
    if journal.schema != "csdlc.v3.local_mutation_journal.v1"
        || journal.issue != issue
        || !matches!(journal.route.as_str(), "bind" | "edit")
        || journal.request_digest.len() != 64
        || !journal
            .request_digest
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_journal_mismatch",
            "lifecycle mutation journal identity is invalid",
        )]);
    }
    if journal.route == "bind" {
        let branch = journal.bind_branch.as_deref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_transaction_bind_identity_missing",
                "bind recovery requires its exact branch identity",
            )]
        })?;
        let target = journal.bind_worktree.as_deref().ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "local_transaction_bind_identity_missing",
                "bind recovery requires its exact worktree identity",
            )]
        })?;
        ensure_bind_registration(
            &context.repository_root,
            branch,
            target,
            &context.expected_head_sha,
        )?;
    }
    commit_local_transaction(context, &journal)
}

fn commit_pending_local_transaction(
    context: &OperationalLocalContext,
    issue: u64,
) -> Result<(), Vec<DoctorFinding>> {
    recover_pending_local_transaction(context, issue)
}

fn commit_local_transaction(
    context: &OperationalLocalContext,
    journal: &LocalMutationJournal,
) -> Result<(), Vec<DoctorFinding>> {
    let issue_root = context.state_root.join(format!("issues/{}", journal.issue));
    let journal_path = local_transaction_journal_path(context, journal.issue);
    let (stage, backup, completion) = local_transaction_paths(
        context,
        journal.issue,
        &journal.route,
        &journal.request_digest,
    );
    if issue_root.exists() && stage.exists() && !backup.exists() {
        fs::rename(&issue_root, &backup)
            .map_err(io_finding("local_transaction_backup_commit_failed"))?;
        local_transaction_failpoint("after_backup_rename");
    }
    if !issue_root.exists() && stage.exists() && backup.exists() {
        fs::rename(&stage, &issue_root)
            .map_err(io_finding("local_transaction_stage_commit_failed"))?;
        local_transaction_failpoint("after_stage_rename");
    }
    if !issue_root.exists() || stage.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_state_ambiguous",
            "lifecycle transaction cannot reconcile its issue, stage, and backup state",
        )]);
    }
    let completed = LocalMutationCompletion {
        schema: "csdlc.v3.local_mutation_completion.v1".into(),
        issue: journal.issue,
        route: journal.route.clone(),
        request_digest: journal.request_digest.clone(),
        result: journal.result.clone(),
    };
    fs::create_dir_all(completion.parent().expect("completion has a parent"))
        .map_err(io_finding("local_transaction_completion_parent_failed"))?;
    let value = serde_json::to_value(completed).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "local_transaction_completion_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write_json(&completion, &value)?;
    if backup.exists() {
        fs::remove_dir_all(&backup)
            .map_err(io_finding("local_transaction_backup_cleanup_failed"))?;
    }
    fs::remove_file(&journal_path).map_err(io_finding("local_transaction_journal_cleanup_failed"))
}

fn load_local_completion(
    context: &OperationalLocalContext,
    request: &LocalPreparationRequest,
    route: &str,
    request_digest: &str,
) -> Result<Option<OperationalLocalResult>, Vec<DoctorFinding>> {
    if !matches!(route, "bind" | "edit") {
        return Ok(None);
    }
    let (_, _, path) = local_transaction_paths(context, request.issue, route, request_digest);
    if !path.exists() {
        return Ok(None);
    }
    let completion: LocalMutationCompletion = serde_json::from_slice(
        &fs::read(path).map_err(io_finding("local_transaction_completion_read_failed"))?,
    )
    .map_err(|error| {
        vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_invalid",
            &error.to_string(),
        )]
    })?;
    if completion.schema != "csdlc.v3.local_mutation_completion.v1"
        || completion.issue != request.issue
        || completion.route != route
        || completion.request_digest != request_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_mismatch",
            "lifecycle mutation completion does not match the exact request",
        )]);
    }
    let issue_root = context.state_root.join(format!("issues/{}", request.issue));
    let observed = inspect_lifecycle_issue_root(&issue_root, request.issue, "v3");
    if completion.result.route != route
        || completion.result.issue != request.issue
        || !completion.result.mutated
        || completion.result.phase != observed.phase
        || completion.result.generation != observed.generation
        || completion.result.digest != observed.digest
        || !observed.ready_to_execute
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "local_transaction_completion_state_mismatch",
            "lifecycle mutation completion does not reconcile to the live issue state",
        )]);
    }
    if route == "bind" {
        if !git_worktree_registration(
            &context.repository_root,
            &request.branch,
            Path::new(&request.worktree),
        )? {
            return Err(vec![finding(
                PlanStatus::Blocked,
                "local_transaction_completion_bind_mismatch",
                "bind completion does not reconcile to the exact Git worktree registration",
            )]);
        }
        verify_bound_worktree(
            &request.branch,
            Path::new(&request.worktree),
            &context.expected_head_sha,
        )?;
    }
    Ok(Some(completion.result))
}

fn ensure_bind_registration(
    repository_root: &Path,
    branch: &str,
    target: &Path,
    expected_head: &str,
) -> Result<(), Vec<DoctorFinding>> {
    if git_worktree_registration(repository_root, branch, target)? {
        return verify_bound_worktree(branch, target, expected_head);
    }
    if target.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "unregistered_worktree_exists",
            "target path exists but is not registered for the requested branch",
        )]);
    }
    let branch_ref = format!("refs/heads/{branch}");
    let branch_exists = Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(["show-ref", "--verify", "--quiet", &branch_ref])
        .status()
        .map_err(io_finding("git_branch_probe_failed"))?
        .success();
    if !branch_exists {
        let branch_output = Command::new("git")
            .arg("-C")
            .arg(repository_root)
            .args(["branch", "--"])
            .arg(branch)
            .arg(expected_head)
            .output()
            .map_err(io_finding("git_branch_create_failed"))?;
        if !branch_output.status.success() {
            return Err(vec![finding(
                PlanStatus::Failed,
                "git_branch_create_failed",
                String::from_utf8_lossy(&branch_output.stderr).trim(),
            )]);
        }
        local_transaction_failpoint("bind_after_branch_creation");
    }
    let output = Command::new("git")
        .arg("-C")
        .arg(repository_root)
        .args(["worktree", "add", "--"])
        .arg(target)
        .arg(branch)
        .output()
        .map_err(io_finding("git_worktree_add_failed"))?;
    if !output.status.success() || !git_worktree_registration(repository_root, branch, target)? {
        return Err(vec![finding(
            PlanStatus::Failed,
            "git_worktree_add_failed",
            String::from_utf8_lossy(&output.stderr).trim(),
        )]);
    }
    verify_bound_worktree(branch, target, expected_head)
}

fn verify_bound_worktree(
    branch: &str,
    target: &Path,
    expected_head: &str,
) -> Result<(), Vec<DoctorFinding>> {
    let output = Command::new("git")
        .arg("-C")
        .arg(target)
        .args(["rev-parse", "--is-inside-work-tree", "HEAD"])
        .output()
        .map_err(io_finding("bound_worktree_health_check_failed"))?;
    let lines: Vec<_> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_owned)
        .collect();
    if !output.status.success()
        || lines.first().map(String::as_str) != Some("true")
        || lines.get(1).map(String::as_str) != Some(expected_head)
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bound_worktree_head_mismatch",
            "registered worktree must be a healthy checkout at the exact expected head",
        )]);
    }
    let branch_output = Command::new("git")
        .arg("-C")
        .arg(target)
        .args(["symbolic-ref", "--quiet", "--short", "HEAD"])
        .output()
        .map_err(io_finding("bound_worktree_branch_check_failed"))?;
    if !branch_output.status.success()
        || String::from_utf8_lossy(&branch_output.stdout).trim() != branch
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bound_worktree_branch_mismatch",
            "registered worktree branch does not match the requested branch",
        )]);
    }
    let status = Command::new("git")
        .arg("-C")
        .arg(target)
        .args(["status", "--porcelain"])
        .output()
        .map_err(io_finding("bound_worktree_status_failed"))?;
    if !status.status.success() || !status.stdout.is_empty() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "bound_worktree_not_clean",
            "newly bound worktree must be clean before lifecycle completion is published",
        )]);
    }
    Ok(())
}

fn local_transaction_failpoint(name: &str) {
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref() == Ok(name) {
        std::process::exit(91);
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
    if selector.get("schema").and_then(Value::as_str) != Some("csdlc.generation_selector.v2")
        || selector.get("default_generation").and_then(Value::as_str) != Some("v3")
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "canonical_v3_authority_inactive",
            "canonical generation selector does not grant v3 operational authority",
        )]);
    }

    let _authority = crate::authority::canonical_v3_authority(&repository_root)
        .map_err(|error| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_invalid",
                &error,
            )]
        })?
        .ok_or_else(|| {
            vec![finding(
                PlanStatus::Blocked,
                "canonical_v3_authority_inactive",
                "tracked v3 selector is not active on origin/main",
            )]
        })?;
    if context.cutover_approval_path != Path::new(crate::authority::SELECTOR_PATH)
        || context.expected_cutover_approval_digest != context.expected_authority_selector_digest
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "cutover_authority_mismatch",
            "operational context does not bind the canonical merge-derived selector authority",
        )]);
    }
    let head = Command::new("git")
        .arg("-C")
        .arg(&repository_root)
        .args(["rev-parse", "HEAD"])
        .output()
        .map_err(io_finding("git_head_observation_failed"))?;
    if !head.status.success()
        || String::from_utf8_lossy(&head.stdout).trim() != context.expected_head_sha
    {
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
    let parent = issue_root.parent().ok_or_else(|| {
        vec![finding(
            PlanStatus::Failed,
            "issue_state_parent_missing",
            "issue state path must have a parent",
        )]
    })?;
    fs::create_dir_all(parent).map_err(io_finding("issue_state_parent_create_failed"))?;
    let stage = parent.join(format!(
        ".issue-{}.init-{}-{}",
        request.issue,
        std::process::id(),
        next_local_temp_sequence()
    ));
    fs::create_dir(&stage).map_err(io_finding("issue_stage_create_failed"))?;
    let staged_result = (|| {
        let cards_root = stage.join("cards");
        fs::create_dir(&cards_root).map_err(io_finding("issue_state_create_failed"))?;
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
        persist_index(&stage, request, registry, "ready", 1)
    })();
    let (generation, digest) = match staged_result {
        Ok(result) => result,
        Err(mut findings) => {
            append_directory_rollback_finding(&stage, &mut findings);
            return Err(findings);
        }
    };
    if let Err(error) = fs::rename(&stage, issue_root) {
        let mut findings = io_finding("issue_state_commit_failed")(error);
        append_directory_rollback_finding(&stage, &mut findings);
        return Err(findings);
    }
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

fn append_directory_rollback_finding(path: &Path, findings: &mut Vec<DoctorFinding>) {
    if let Err(error) = fs::remove_dir_all(path) {
        findings.push(finding(
            PlanStatus::Failed,
            "issue_state_rollback_failed",
            &error.to_string(),
        ));
    }
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
    if !git_worktree_registration(&context.repository_root, &request.branch, &target)?
        && target.exists()
    {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "unregistered_worktree_exists",
            "target path exists but is not registered for the requested branch",
        )]);
    }
    let request_digest = local_request_digest(request)?;
    let stage = prepare_local_transaction_stage(context, request.issue, "bind", &request_digest)?;
    atomic_write_json(
        &stage.join("binding.json"),
        &serde_json::json!({
            "schema": "csdlc.v3.binding.v1",
            "issue": request.issue,
            "branch": request.branch,
            "worktree": request.worktree
        }),
    )?;
    let index = read_index_value(&stage)?;
    let registry_version = index["template_registry_version"]
        .as_str()
        .unwrap_or(&request.registry_version);
    let registry = PromptRegistry {
        version: registry_version.to_owned(),
        card_kinds: BTreeSet::new(),
        template_paths: BTreeMap::new(),
    };
    let generation = observed.generation.unwrap_or(1) + 1;
    let (generation, digest) = persist_index(&stage, request, &registry, "bound", generation)?;
    let result = operational_result(
        "bind",
        request.issue,
        true,
        "bound",
        generation,
        digest,
        Some("edit"),
        vec![],
    );
    begin_local_transaction(
        context,
        LocalMutationJournal {
            schema: "csdlc.v3.local_mutation_journal.v1".into(),
            issue: request.issue,
            route: "bind".into(),
            request_digest,
            result: result.clone(),
            bind_branch: Some(request.branch.clone()),
            bind_worktree: Some(target.clone()),
        },
    )?;
    ensure_bind_registration(
        &context.repository_root,
        &request.branch,
        &target,
        &context.expected_head_sha,
    )?;
    local_transaction_failpoint("bind_after_git");
    commit_pending_local_transaction(context, request.issue)?;
    Ok(result)
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
    context: &OperationalLocalContext,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if request.card_updates.is_empty() {
        return Err(vec![finding(
            PlanStatus::Failed,
            "card_updates_missing",
            "edit requires at least one typed card update",
        )]);
    }
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    let phase = observed.phase.as_deref().unwrap_or("ready");
    let request_digest = local_request_digest(request)?;
    let stage = prepare_local_transaction_stage(context, request.issue, "edit", &request_digest)?;
    let staged = (|| {
        for (kind, update) in &request.card_updates {
            if !REQUIRED_CARD_KINDS.contains(&kind.as_str()) || !update.is_object() {
                return Err(vec![finding(
                    PlanStatus::Failed,
                    "card_update_invalid",
                    "card updates must be objects keyed by canonical card kind",
                )]);
            }
            let values_path = stage.join(format!("cards/{kind}.values.json"));
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
            atomic_write_json(&values_path, &values)?;
            atomic_write(
                &stage.join(format!("cards/{kind}.md")),
                render_template(&template, &values).as_bytes(),
            )?;
        }
        persist_index(
            &stage,
            request,
            registry,
            phase,
            observed.generation.unwrap_or(1) + 1,
        )
    })();
    let (generation, digest) = match staged {
        Ok(value) => value,
        Err(mut findings) => {
            append_directory_rollback_finding(&stage, &mut findings);
            return Err(findings);
        }
    };
    let result = operational_result(
        "edit",
        request.issue,
        true,
        phase,
        generation,
        digest,
        Some("validate"),
        vec![],
    );
    if let Err(mut findings) = begin_local_transaction(
        context,
        LocalMutationJournal {
            schema: "csdlc.v3.local_mutation_journal.v1".into(),
            issue: request.issue,
            route: "edit".into(),
            request_digest,
            result: result.clone(),
            bind_branch: None,
            bind_worktree: None,
        },
    ) {
        append_directory_rollback_finding(&stage, &mut findings);
        return Err(findings);
    }
    commit_pending_local_transaction(context, request.issue)?;
    Ok(result)
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
        ".{}.tmp-{}-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("state"),
        std::process::id(),
        next_local_temp_sequence()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(io_finding("temporary_create_failed"))?;
    file.write_all(bytes)
        .map_err(io_finding("temporary_write_failed"))?;
    file.sync_all()
        .map_err(io_finding("temporary_sync_failed"))?;
    fs::rename(&temporary, path).map_err(io_finding("atomic_rename_failed"))
}

fn next_local_temp_sequence() -> u64 {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    SEQUENCE.fetch_add(1, Ordering::Relaxed)
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
        routing: None,
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
