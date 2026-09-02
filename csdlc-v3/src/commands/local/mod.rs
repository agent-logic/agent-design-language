//! Non-authoritative V3-D local preparation workflow model.
//!
//! These types model the local issue-input → card-rendering → topology-bind →
//! doctor/PVF-plan path for V3-D. They do not mutate repository state, create
//! worktrees, execute PVF lanes, publish PRs, write GitHub state, finish,
//! cleanup, migrate v2, or grant operational authority.

use std::{collections::BTreeSet, path::Path};

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
    pub commands: Vec<LocalCommand>,
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
}

/// The planned render set for an issue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardRenderPlan {
    pub registry_version: String,
    pub issue: u64,
    pub card_kinds: Vec<String>,
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
    pub status: PlanStatus,
    pub code: String,
    pub message: String,
    pub cards_present: Vec<String>,
    pub missing_cards: Vec<String>,
    pub ready_to_execute: bool,
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
        Ok(Self {
            version: version.into(),
            card_kinds: templates.keys().cloned().collect(),
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
        Ok(CardRenderPlan {
            registry_version: registry.version.clone(),
            issue,
            card_kinds: REQUIRED_CARD_KINDS
                .into_iter()
                .map(str::to_string)
                .collect(),
        })
    } else {
        Err(findings)
    }
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

pub fn inspect_local_lifecycle_state(root: &Path, issue: u64) -> LocalLifecycleStateObservation {
    let issue_root = root.join(format!(".csdlc/issues/{issue}"));
    let index_path = issue_root.join("index.json");
    if !index_path.is_file() {
        return LocalLifecycleStateObservation {
            issue,
            status: PlanStatus::Blocked,
            code: "missing_local_lifecycle_state".into(),
            message: "local lifecycle state is missing; initialize or repair the issue record before executing local routes".into(),
            cards_present: Vec::new(),
            missing_cards: REQUIRED_CARD_KINDS.into_iter().map(str::to_string).collect(),
            ready_to_execute: false,
        };
    }
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
    if missing_cards.is_empty() {
        LocalLifecycleStateObservation {
            issue,
            status: PlanStatus::Ready,
            code: "local_lifecycle_state_ready".into(),
            message: "local lifecycle state contains the six-card denominator".into(),
            cards_present,
            missing_cards,
            ready_to_execute: true,
        }
    } else {
        LocalLifecycleStateObservation {
            issue,
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

pub fn finding(status: PlanStatus, code: &str, message: &str) -> DoctorFinding {
    DoctorFinding {
        status,
        code: code.into(),
        message: message.into(),
    }
}
