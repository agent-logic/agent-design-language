//! Native C-SDLC v3 local preparation, binding and card operations.
//!
//! Operational execution requires authenticated canonical selector and receipt
//! validation, plus exact checkout, registration and lifecycle digest checks. The retained planning-only
//! functions remain non-mutating models and do not grant operational authority.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    path::PathBuf,
};

const REQUIRED_CARD_KINDS: [&str; 6] = ["sip", "stp", "spp", "vpp", "srp", "sor"];
mod binding;
mod cards;
mod context;
pub mod intent;
mod issue;
mod lifecycle;
mod planning;
mod routing;
mod storage;
mod transactions;
mod worktree;

pub use cards::render_semantic_card_projection;
pub use context::{discover_operational_local_context, operational_state_root};
pub use lifecycle::{
    initialize_v3_local_state, inspect_local_lifecycle_state, inspect_v3_local_state,
};
pub use planning::{
    authorize_bind, execute_local_route, grants_operational_authority, local_route_command,
    local_route_status, plan_cards, prepare_local_workflow, required_local_commands,
    validate_contract,
};
pub use routing::execute_operational_local_route;
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

pub fn is_observation_route(route: &str) -> bool {
    matches!(
        route,
        "doctor" | "eligibility" | "validate" | "schedule" | "shepherd"
    )
}

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

/// Typed request shared by retained planning and guarded operational routes.
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

pub fn finding(status: PlanStatus, code: &str, message: &str) -> DoctorFinding {
    DoctorFinding {
        status,
        code: code.into(),
        message: message.into(),
    }
}
