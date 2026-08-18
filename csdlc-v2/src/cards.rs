use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use markdown::mdast::Node;
use markdown::{to_mdast, ParseOptions};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::estimation::{validate_accepted_estimate, AcceptedEstimate};
use crate::model::LifecyclePhase;

macro_rules! closed_enum {
    ($name:ident { $($variant:ident),+ $(,)? }) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display,
            EnumString, AsRefStr, EnumIter,
        )]
        #[serde(rename_all = "snake_case")]
        #[strum(serialize_all = "snake_case")]
        pub enum $name { $($variant),+ }
    };
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "lowercase")]
#[strum(serialize_all = "lowercase")]
pub enum CardKind {
    Sip,
    Stp,
    Spp,
    Vpp,
    Srp,
    Sor,
}

impl CardKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::Sip => "Structured Intent Prompt",
            Self::Stp => "Structured Task Prompt",
            Self::Spp => "Structured Planning Prompt",
            Self::Vpp => "Validation Planning Prompt",
            Self::Srp => "Structured Review Prompt",
            Self::Sor => "Structured Output Record",
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CardStatus {
    PrePhase,
    Draft,
    Ready,
    Approved,
    Blocked,
    Superseded,
    Complete,
}

impl CardStatus {
    fn allows(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::PrePhase, Self::Draft | Self::Ready)
                | (Self::Draft, Self::Ready | Self::Blocked | Self::Superseded)
                | (
                    Self::Ready,
                    Self::Approved | Self::Blocked | Self::Superseded | Self::Complete
                )
                | (
                    Self::Approved,
                    Self::Blocked | Self::Superseded | Self::Complete
                )
                | (Self::Blocked, Self::Draft | Self::Ready | Self::Superseded)
        )
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum StepStatus {
    Pending,
    InProgress,
    Completed,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlanningProfile {
    Small,
    Medium,
    Large,
    Migration,
}

impl PlanningProfile {
    pub fn estimates(self) -> (ExecutionEstimates, u64) {
        match self {
            Self::Small => (
                ExecutionEstimates {
                    elapsed_seconds: 7_200,
                    total_tokens: 40_000,
                    validation_seconds: 1_200,
                    advisory: None,
                },
                10_000,
            ),
            Self::Medium => (
                ExecutionEstimates {
                    elapsed_seconds: 21_600,
                    total_tokens: 80_000,
                    validation_seconds: 3_600,
                    advisory: None,
                },
                25_000,
            ),
            Self::Large => (
                ExecutionEstimates {
                    elapsed_seconds: 43_200,
                    total_tokens: 140_000,
                    validation_seconds: 7_200,
                    advisory: None,
                },
                50_000,
            ),
            Self::Migration => (
                ExecutionEstimates {
                    elapsed_seconds: 86_400,
                    total_tokens: 240_000,
                    validation_seconds: 21_600,
                    advisory: None,
                },
                100_000,
            ),
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum FindingDisposition {
    Open,
    Fixed,
    AcceptedRisk,
    OutOfScope,
}

closed_enum!(ResourceProfile {
    Small,
    Medium,
    Large
});
closed_enum!(FindingSeverity { P0, P1, P2, P3 });
closed_enum!(ReviewResult {
    PreReview,
    Pass,
    ChangesRequired,
    Blocked
});
closed_enum!(EvidenceOutcome {
    Passed,
    Failed,
    Blocked,
    Waiting,
    Deferred,
    SkippedNonGoal
});
closed_enum!(IntegrationState {
    NotStarted,
    WorktreeOnly,
    PrOpen,
    Merged,
    ClosedNoPr
});
closed_enum!(PublicationState {
    NotPublished,
    Draft,
    Ready,
    Closed
});
closed_enum!(MergeState {
    NotMerged,
    Pending,
    Merged,
    ClosedUnmerged
});
closed_enum!(CloseoutState {
    NotStarted,
    InProgress,
    Complete,
    Blocked
});

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CardIdentity {
    pub schema_version: String,
    pub template_version: String,
    pub issue: u64,
    pub repository: String,
    pub title: String,
    pub slug: String,
    pub version: String,
    pub generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SipValues {
    pub goal: String,
    pub required_outcome: String,
    pub declared_scope: Vec<String>,
    pub authority_boundary: Vec<String>,
    pub initial_assumptions: Vec<String>,
    pub operator_constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StpValues {
    pub task_boundary: String,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<String>,
    pub repo_inputs: Vec<String>,
    pub non_goals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PlanStep {
    pub id: String,
    pub action: String,
    pub acceptance_ids: Vec<String>,
    pub status: StepStatus,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ExecutionEstimates {
    pub elapsed_seconds: u64,
    pub total_tokens: u64,
    pub validation_seconds: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub advisory: Option<Box<AcceptedEstimate>>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SppValues {
    pub plan_revision: u64,
    pub summary: String,
    pub steps: Vec<PlanStep>,
    pub affected_areas: Vec<String>,
    pub invariants: Vec<String>,
    pub risks: Vec<String>,
    pub execution_estimates: ExecutionEstimates,
    pub stop_conditions: Vec<String>,
    pub replan_triggers: Vec<String>,
    pub design_ref: String,
    pub design_digest: String,
    pub diagram_ref: String,
    pub diagram_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationLane {
    pub lane: String,
    pub proof_role: String,
    pub acceptance_ids: Vec<String>,
    pub deterministic: bool,
    pub resource_profile: ResourceProfile,
    pub budget_seconds: u64,
    pub budget_tokens: u64,
    pub argv: Vec<String>,
    pub parallel_group: String,
    pub defer_reason: Option<String>,
}

closed_enum!(RustTestSelectorPosture {
    ExactTarget,
    IntentionalBroad,
    Invalid,
});

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RustTestSelectorClassification {
    pub posture: RustTestSelectorPosture,
    pub diagnostic: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct VppValues {
    pub summary: String,
    pub lanes: Vec<ValidationLane>,
    pub planned_validation_seconds: u64,
    pub planned_validation_tokens: u64,
    pub failure_policy: String,
    pub design_ref: String,
    pub design_digest: String,
    pub diagram_ref: String,
    pub diagram_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReviewFinding {
    pub id: String,
    pub severity: FindingSeverity,
    pub summary: String,
    pub actionable: bool,
    #[serde(default = "default_true")]
    pub in_scope: bool,
    pub disposition: FindingDisposition,
    #[serde(default)]
    pub fix_revision: Option<String>,
    #[serde(default)]
    pub route: Option<String>,
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SrpValues {
    pub review_scope: String,
    pub review_revision: Option<String>,
    pub reviewer: Option<String>,
    pub review_prompts: Vec<String>,
    pub findings: Vec<ReviewFinding>,
    pub residual_risk: Vec<String>,
    pub review_result: ReviewResult,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationResult {
    pub command: Vec<String>,
    pub purpose: String,
    pub outcome: EvidenceOutcome,
    pub evidence_ref: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SorValues {
    pub summary: String,
    pub actual_changes: Vec<String>,
    pub artifacts: Vec<String>,
    pub actual_validation: Vec<ValidationResult>,
    pub integration_state: IntegrationState,
    pub publication_state: PublicationState,
    pub merge_state: MergeState,
    pub closeout_state: CloseoutState,
    pub follow_ups: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "card_kind", content = "values", rename_all = "lowercase")]
pub enum CardContent {
    Sip(SipValues),
    Stp(StpValues),
    Spp(SppValues),
    Vpp(VppValues),
    Srp(SrpValues),
    Sor(SorValues),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CardValues {
    pub identity: CardIdentity,
    pub status: CardStatus,
    pub content: CardContent,
}

impl CardValues {
    pub fn kind(&self) -> CardKind {
        match self.content {
            CardContent::Sip(_) => CardKind::Sip,
            CardContent::Stp(_) => CardKind::Stp,
            CardContent::Spp(_) => CardKind::Spp,
            CardContent::Vpp(_) => CardKind::Vpp,
            CardContent::Srp(_) => CardKind::Srp,
            CardContent::Sor(_) => CardKind::Sor,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct InitialCardInput {
    pub title: String,
    pub slug: String,
    pub version: String,
    pub goal: String,
    pub required_outcome: String,
    pub declared_scope: Vec<String>,
    pub authority_boundary: Vec<String>,
    #[serde(default = "explicit_none_list")]
    pub operator_constraints: Vec<String>,
    pub task_boundary: String,
    pub deliverables: Vec<String>,
    pub acceptance_criteria: Vec<String>,
    pub dependencies: Vec<String>,
    pub repo_inputs: Vec<String>,
    pub non_goals: Vec<String>,
    pub plan_summary: String,
    pub steps: Vec<PlanStep>,
    pub affected_areas: Vec<String>,
    pub invariants: Vec<String>,
    pub risks: Vec<String>,
    pub planning_profile: PlanningProfile,
    pub stop_conditions: Vec<String>,
    pub validation_lanes: Vec<ValidationLane>,
    pub failure_policy: String,
    pub review_prompts: Vec<String>,
    #[serde(default = "explicit_none")]
    pub review_scope: String,
}

fn explicit_none_list() -> Vec<String> {
    vec!["none".into()]
}

fn explicit_none() -> String {
    "none".into()
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum TextField {
    Goal,
    RequiredOutcome,
    TaskBoundary,
    PlanSummary,
    FailurePolicy,
    ReviewScope,
    SorSummary,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PlanningCollectionField {
    DeclaredScope,
    AuthorityBoundary,
    InitialAssumptions,
    Deliverables,
    Dependencies,
    RepoInputs,
    NonGoals,
    AffectedAreas,
    Invariants,
    Risks,
    StopConditions,
    ReplanTriggers,
    ReviewPrompts,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "operation", rename_all = "snake_case")]
pub enum SemanticOperation {
    UpdateIdentityVersion {
        version: String,
    },
    CorrectIdentityTitleSlugAfterDecomposition {
        title: String,
        slug: String,
        live_issue_title: String,
        live_issue_url: String,
        live_issue_body_digest: String,
    },
    Replan {
        field: TextField,
        value: String,
    },
    ReplaceOperatorConstraints {
        values: Vec<String>,
    },
    ReplaceAcceptanceCriteria {
        values: Vec<String>,
    },
    ReplacePlanningCollection {
        field: PlanningCollectionField,
        values: Vec<String>,
    },
    CorrectReviewPromptsAfterRecovery {
        values: Vec<String>,
    },
    CorrectDeclaredScopeBeforePublication {
        values: Vec<String>,
    },
    CorrectStpDeliverablesAfterRecovery {
        values: Vec<String>,
    },
    CorrectStpDependenciesAfterRecovery {
        values: Vec<String>,
    },
    CorrectStpRepoInputsAfterRecovery {
        values: Vec<String>,
    },
    CorrectPlanSummaryAfterRecovery {
        value: String,
    },
    CorrectValidationSummaryAfterRecovery {
        value: String,
    },
    CorrectValidationFailurePolicyAfterRecovery {
        value: String,
    },
    CorrectSorFollowUpsAfterRecovery {
        values: Vec<String>,
    },
    CorrectGoalAfterRecovery {
        value: String,
    },
    CorrectRequiredOutcomeAfterRecovery {
        value: String,
    },
    ReplaceSorFollowUpsAfterRecovery {
        values: Vec<String>,
    },
    CorrectOperatorConstraintsBeforeBind {
        values: Vec<String>,
    },
    RefreshAuthoredDesignAfterRecovery,
    CorrectPlanStepsAfterRecovery {
        steps: Vec<PlanStep>,
    },
    ReplacePlanSteps {
        steps: Vec<PlanStep>,
    },
    ReplaceAcceptancePlan {
        acceptance_criteria: Vec<String>,
        steps: Vec<PlanStep>,
        validation_lanes: Vec<ValidationLane>,
    },
    SetField {
        field: TextField,
        value: String,
    },
    AppendReference {
        value: String,
    },
    UpdatePlanStep {
        step_id: String,
        status: StepStatus,
    },
    ReplaceValidationLanes {
        lanes: Vec<ValidationLane>,
    },
    RecordAdvisoryEstimate {
        estimate: AcceptedEstimate,
    },
    RecordValidation {
        result: ValidationResult,
    },
    RecordFinding {
        finding: ReviewFinding,
    },
    RecordReview {
        reviewer: String,
        revision: String,
        result: ReviewResult,
        residual_risk: Vec<String>,
    },
    DisposeFinding {
        finding_id: String,
        disposition: FindingDisposition,
    },
    AdvanceStatus {
        status: CardStatus,
    },
    RecordExecution {
        summary: String,
        changes: Vec<String>,
        artifacts: Vec<String>,
    },
    ReplaceExecution {
        summary: String,
        changes: Vec<String>,
        artifacts: Vec<String>,
        validation: Vec<ValidationResult>,
    },
    RecordCloseout {
        integration_state: IntegrationState,
        publication_state: PublicationState,
        merge_state: MergeState,
        closeout_state: CloseoutState,
    },
    RecordPublication {
        state: PublicationState,
    },
    RecordMerge {
        state: MergeState,
    },
    AdvancePhase {
        phase: LifecyclePhase,
    },
}

#[derive(Debug, Clone)]
pub struct RenderedCard {
    pub markdown: String,
    pub values_digest: String,
    pub rendered_digest: String,
    pub ast_digest: String,
}

struct CardTemplate {
    version: &'static str,
    headings: &'static [&'static str],
}

fn template_for(kind: CardKind) -> CardTemplate {
    let headings = match kind {
        CardKind::Sip => &[
            "Goal",
            "Required Outcome",
            "Scope",
            "Authority",
            "Assumptions",
            "Operator Constraints",
        ][..],
        CardKind::Stp => &[
            "Task",
            "Deliverables",
            "Acceptance",
            "Dependencies",
            "Inputs",
            "Non Goals",
        ][..],
        CardKind::Spp => &[
            "Summary",
            "Plan",
            "Steps",
            "Invariants",
            "Risks",
            "Estimates",
            "Design",
            "Diagram",
            "Stop Conditions",
            "Handoff",
        ][..],
        CardKind::Vpp => &[
            "Summary",
            "Lane Inputs",
            "Selected Lanes",
            "Parallelization",
            "Budgets",
            "Commands",
            "Failure Semantics",
            "Handoff",
        ][..],
        CardKind::Srp => &[
            "Scope",
            "Prompts",
            "Findings",
            "Dispositions",
            "Residual Risk",
            "Review Result",
        ][..],
        CardKind::Sor => &[
            "Summary",
            "Artifacts",
            "Execution",
            "Validation",
            "Integration",
            "Publication",
            "Closeout",
            "Follow Ups",
        ][..],
    };
    CardTemplate {
        version: "1.0.0",
        headings,
    }
}

pub(crate) fn compiled_headings(kind: CardKind) -> Vec<&'static str> {
    template_for(kind).headings.to_vec()
}

pub fn initial_cards(
    issue: u64,
    repository: &str,
    design_ref: &str,
    design_digest: &str,
    diagram_ref: &str,
    diagram_digest: &str,
    input: InitialCardInput,
) -> Result<BTreeMap<CardKind, CardValues>> {
    require_input(&input)?;
    let identity = CardIdentity {
        schema_version: "1.0.0".into(),
        template_version: "1.0.0".into(),
        issue,
        repository: repository.into(),
        title: input.title.clone(),
        slug: input.slug.clone(),
        version: input.version.clone(),
        generation: 0,
    };
    let (estimates, planned_validation_tokens) = input.planning_profile.estimates();
    let planned_seconds = estimates.validation_seconds;
    let cards = [
        (
            CardKind::Sip,
            CardStatus::Ready,
            CardContent::Sip(SipValues {
                goal: input.goal,
                required_outcome: input.required_outcome,
                declared_scope: input.declared_scope,
                authority_boundary: input.authority_boundary,
                initial_assumptions: Vec::new(),
                operator_constraints: input.operator_constraints,
            }),
        ),
        (
            CardKind::Stp,
            CardStatus::Ready,
            CardContent::Stp(StpValues {
                task_boundary: input.task_boundary,
                deliverables: input.deliverables,
                acceptance_criteria: input.acceptance_criteria,
                dependencies: input.dependencies,
                repo_inputs: input.repo_inputs,
                non_goals: input.non_goals,
            }),
        ),
        (
            CardKind::Spp,
            CardStatus::Ready,
            CardContent::Spp(SppValues {
                plan_revision: 1,
                summary: input.plan_summary,
                steps: input.steps,
                affected_areas: input.affected_areas,
                invariants: input.invariants,
                risks: input.risks,
                execution_estimates: estimates,
                stop_conditions: input.stop_conditions,
                replan_triggers: Vec::new(),
                design_ref: design_ref.into(),
                design_digest: design_digest.into(),
                diagram_ref: diagram_ref.into(),
                diagram_digest: diagram_digest.into(),
            }),
        ),
        (
            CardKind::Vpp,
            CardStatus::Ready,
            CardContent::Vpp(VppValues {
                summary: "Execute the smallest proving validation DAG.".into(),
                lanes: input.validation_lanes,
                planned_validation_seconds: planned_seconds,
                planned_validation_tokens,
                failure_policy: input.failure_policy,
                design_ref: design_ref.into(),
                design_digest: design_digest.into(),
                diagram_ref: diagram_ref.into(),
                diagram_digest: diagram_digest.into(),
            }),
        ),
        (
            CardKind::Srp,
            CardStatus::PrePhase,
            CardContent::Srp(SrpValues {
                review_scope: input.review_scope,
                review_revision: None,
                reviewer: None,
                review_prompts: input.review_prompts,
                findings: Vec::new(),
                residual_risk: Vec::new(),
                review_result: ReviewResult::PreReview,
            }),
        ),
        (
            CardKind::Sor,
            CardStatus::PrePhase,
            CardContent::Sor(SorValues {
                summary: "Pre-execution output record.".into(),
                actual_changes: Vec::new(),
                artifacts: Vec::new(),
                actual_validation: Vec::new(),
                integration_state: IntegrationState::NotStarted,
                publication_state: PublicationState::NotPublished,
                merge_state: MergeState::NotMerged,
                closeout_state: CloseoutState::NotStarted,
                follow_ups: Vec::new(),
            }),
        ),
    ]
    .into_iter()
    .map(|(kind, status, content)| {
        (
            kind,
            CardValues {
                identity: identity.clone(),
                status,
                content,
            },
        )
    })
    .collect::<BTreeMap<_, _>>();
    validate_cross_card(
        &cards,
        design_ref,
        design_digest,
        diagram_ref,
        diagram_digest,
    )?;
    Ok(cards)
}

pub fn apply(
    values: &mut CardValues,
    operation: &SemanticOperation,
) -> Result<Option<LifecyclePhase>> {
    match operation {
        SemanticOperation::UpdateIdentityVersion { version } => {
            validate_identity_version(version)?;
            values.identity.version = version.clone();
            Ok(None)
        }
        SemanticOperation::CorrectIdentityTitleSlugAfterDecomposition { .. } => Err(V2Error::new(
            ErrorCode::FieldOwnership,
            "correct_identity_title_slug_after_decomposition is a cross-card operation",
        )),
        SemanticOperation::Replan { field, value } => {
            set_text(values, *field, value.clone())?;
            Ok(None)
        }
        SemanticOperation::ReplaceOperatorConstraints {
            values: replacement,
        } => {
            if replacement.is_empty() || replacement.iter().any(|value| value.trim().is_empty()) {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "operator constraints cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Sip(v) => v.operator_constraints = replacement.clone(),
                _ => return ownership(values.kind(), "replace_operator_constraints"),
            }
            Ok(None)
        }
        SemanticOperation::ReplaceAcceptanceCriteria {
            values: replacement,
        } => {
            validate_replacement(replacement, "acceptance criteria")?;
            match &mut values.content {
                CardContent::Stp(v) => v.acceptance_criteria = replacement.clone(),
                _ => return ownership(values.kind(), "replace_acceptance_criteria"),
            }
            Ok(None)
        }
        SemanticOperation::ReplacePlanningCollection {
            field,
            values: replacement,
        } => {
            validate_replacement(replacement, field.as_ref())?;
            replace_planning_collection(values, *field, replacement.clone())?;
            Ok(None)
        }
        SemanticOperation::CorrectReviewPromptsAfterRecovery {
            values: replacement,
        } => {
            validate_replacement(replacement, "review prompts")?;
            match &mut values.content {
                CardContent::Srp(value) => value.review_prompts = replacement.clone(),
                _ => return ownership(values.kind(), "correct_review_prompts_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectDeclaredScopeBeforePublication {
            values: replacement,
        } => {
            validate_replacement(replacement, "declared scope")?;
            match &mut values.content {
                CardContent::Sip(value) => value.declared_scope = replacement.clone(),
                _ => return ownership(values.kind(), "correct_declared_scope_before_publication"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectStpDeliverablesAfterRecovery {
            values: replacement,
        } => {
            validate_unique_replacement(replacement, "STP deliverables")?;
            match &mut values.content {
                CardContent::Stp(value) => value.deliverables = replacement.clone(),
                _ => return ownership(values.kind(), "correct_stp_deliverables_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectStpDependenciesAfterRecovery {
            values: replacement,
        } => {
            validate_unique_replacement(replacement, "STP dependencies")?;
            match &mut values.content {
                CardContent::Stp(value) => value.dependencies = replacement.clone(),
                _ => return ownership(values.kind(), "correct_stp_dependencies_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectStpRepoInputsAfterRecovery {
            values: replacement,
        } => {
            validate_unique_replacement(replacement, "STP repo inputs")?;
            match &mut values.content {
                CardContent::Stp(value) => value.repo_inputs = replacement.clone(),
                _ => return ownership(values.kind(), "correct_stp_repo_inputs_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectPlanSummaryAfterRecovery { value } => {
            if value.trim().is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "plan summary cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Spp(v) => v.summary = value.clone(),
                _ => return ownership(values.kind(), "correct_plan_summary_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectValidationSummaryAfterRecovery { value } => {
            if value.trim().is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "validation summary cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Vpp(v) => v.summary = value.clone(),
                _ => return ownership(values.kind(), "correct_validation_summary_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectValidationFailurePolicyAfterRecovery { value } => {
            if value.trim().is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "validation failure policy cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Vpp(v) => v.failure_policy = value.clone(),
                _ => {
                    return ownership(
                        values.kind(),
                        "correct_validation_failure_policy_after_recovery",
                    )
                }
            }
            Ok(None)
        }
        SemanticOperation::CorrectSorFollowUpsAfterRecovery {
            values: replacement,
        } => {
            if replacement.iter().any(|value| value.trim().is_empty()) {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "SOR follow-up replacement cannot contain blank entries",
                ));
            }
            match &mut values.content {
                CardContent::Sor(v) => v.follow_ups = replacement.clone(),
                _ => return ownership(values.kind(), "correct_sor_follow_ups_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectRequiredOutcomeAfterRecovery { value } => {
            if value.trim().is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "required outcome cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Sip(v) => v.required_outcome = value.clone(),
                _ => return ownership(values.kind(), "correct_required_outcome_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectGoalAfterRecovery { value } => {
            if value.trim().is_empty() {
                return Err(V2Error::new(ErrorCode::CardInvalid, "goal cannot be empty"));
            }
            match &mut values.content {
                CardContent::Sip(v) => v.goal = value.clone(),
                _ => return ownership(values.kind(), "correct_goal_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::ReplaceSorFollowUpsAfterRecovery {
            values: replacement,
        } => {
            validate_replacement(replacement, "SOR follow-ups")?;
            match &mut values.content {
                CardContent::Sor(v) => v.follow_ups = replacement.clone(),
                _ => return ownership(values.kind(), "replace_sor_follow_ups_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::CorrectOperatorConstraintsBeforeBind {
            values: replacement,
        } => {
            if replacement.is_empty() || replacement.iter().any(|value| value.trim().is_empty()) {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "operator constraints cannot be empty",
                ));
            }
            match &mut values.content {
                CardContent::Sip(v) => v.operator_constraints = replacement.clone(),
                _ => return ownership(values.kind(), "correct_operator_constraints_before_bind"),
            }
            Ok(None)
        }
        SemanticOperation::RefreshAuthoredDesignAfterRecovery => match values.content {
            CardContent::Spp(_) => Ok(None),
            _ => ownership(values.kind(), "refresh_authored_design_after_recovery"),
        },
        SemanticOperation::CorrectPlanStepsAfterRecovery { steps } => {
            validate_recovery_plan_steps(steps)?;
            match &mut values.content {
                CardContent::Spp(v) => {
                    v.steps = steps.clone();
                    v.plan_revision += 1;
                }
                _ => return ownership(values.kind(), "correct_plan_steps_after_recovery"),
            }
            Ok(None)
        }
        SemanticOperation::ReplacePlanSteps { steps } => {
            validate_plan_steps(steps)?;
            match &mut values.content {
                CardContent::Spp(v) => {
                    v.steps = steps.clone();
                    v.plan_revision += 1;
                }
                _ => return ownership(values.kind(), "replace_plan_steps"),
            }
            Ok(None)
        }
        SemanticOperation::ReplaceAcceptancePlan { .. } => Err(V2Error::new(
            ErrorCode::FieldOwnership,
            "replace_acceptance_plan is a cross-card operation",
        )),
        SemanticOperation::SetField { field, value } => {
            set_text(values, *field, value.clone())?;
            Ok(None)
        }
        SemanticOperation::AppendReference { value } => {
            append_reference(values, value.clone())?;
            Ok(None)
        }
        SemanticOperation::UpdatePlanStep { step_id, status } => match &mut values.content {
            CardContent::Spp(v) => {
                let step = v
                    .steps
                    .iter_mut()
                    .find(|step| step.id == *step_id)
                    .ok_or_else(|| {
                        V2Error::new(ErrorCode::InvalidInput, "plan step does not exist")
                    })?;
                let allowed = matches!(
                    (step.status, status),
                    (
                        StepStatus::Pending,
                        StepStatus::InProgress | StepStatus::Completed
                    ) | (StepStatus::InProgress, StepStatus::Completed)
                );
                if !allowed {
                    return Err(V2Error::new(
                        ErrorCode::InvalidTransition,
                        format!("plan step {} -> {} is not allowed", step.status, status),
                    ));
                }
                step.status = *status;
                if *status == StepStatus::InProgress
                    && v.steps
                        .iter()
                        .filter(|step| step.status == StepStatus::InProgress)
                        .count()
                        > 1
                {
                    return Err(V2Error::new(
                        ErrorCode::InvalidTransition,
                        "only one plan step may be in progress",
                    ));
                }
                Ok(None)
            }
            _ => ownership(values.kind(), "update_plan_step"),
        },
        SemanticOperation::ReplaceValidationLanes { lanes } => match &mut values.content {
            CardContent::Vpp(v) => {
                validate_validation_lanes(lanes)?;
                v.lanes = lanes.clone();
                Ok(None)
            }
            _ => ownership(values.kind(), "replace_validation_lanes"),
        },
        SemanticOperation::RecordAdvisoryEstimate { estimate } => {
            validate_accepted_estimate(estimate)?;
            match &mut values.content {
                CardContent::Spp(v) => {
                    v.execution_estimates.advisory = Some(Box::new(estimate.clone()));
                    Ok(None)
                }
                _ => ownership(values.kind(), "record_advisory_estimate"),
            }
        }
        SemanticOperation::RecordValidation { result } => match &mut values.content {
            CardContent::Sor(v) => {
                validate_result(result)?;
                v.actual_validation.push(result.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_validation"),
        },
        SemanticOperation::RecordFinding { finding } => match &mut values.content {
            CardContent::Srp(v) => {
                v.findings.push(finding.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_finding"),
        },
        SemanticOperation::RecordReview {
            reviewer,
            revision,
            result,
            residual_risk,
        } => match &mut values.content {
            CardContent::Srp(v) => {
                if reviewer.trim().is_empty() || revision.trim().is_empty() {
                    return Err(V2Error::new(
                        ErrorCode::InvalidInput,
                        "reviewer and exact revision are required",
                    ));
                }
                v.reviewer = Some(reviewer.clone());
                v.review_revision = Some(revision.clone());
                v.review_result = *result;
                v.residual_risk = residual_risk.clone();
                Ok(None)
            }
            _ => ownership(values.kind(), "record_review"),
        },
        SemanticOperation::DisposeFinding {
            finding_id,
            disposition,
        } => match &mut values.content {
            CardContent::Srp(v) => {
                let finding = v
                    .findings
                    .iter_mut()
                    .find(|f| &f.id == finding_id)
                    .ok_or_else(|| {
                        V2Error::new(ErrorCode::InvalidInput, "finding does not exist")
                    })?;
                finding.disposition = *disposition;
                Ok(None)
            }
            _ => ownership(values.kind(), "dispose_finding"),
        },
        SemanticOperation::AdvanceStatus { status } => {
            if !values.status.allows(*status) {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    format!("card status {} -> {} is not allowed", values.status, status),
                ));
            }
            validate_status_guard(values, *status)?;
            values.status = *status;
            Ok(None)
        }
        SemanticOperation::RecordExecution {
            summary,
            changes,
            artifacts,
        } => match &mut values.content {
            CardContent::Sor(v) => {
                v.summary = summary.clone();
                v.actual_changes.extend(changes.clone());
                v.artifacts.extend(artifacts.clone());
                Ok(None)
            }
            _ => ownership(values.kind(), "record_execution"),
        },
        SemanticOperation::ReplaceExecution {
            summary,
            changes,
            artifacts,
            validation,
        } => {
            if summary.trim().is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "execution summary cannot be empty",
                ));
            }
            validate_replacement(changes, "execution changes")?;
            validate_replacement(artifacts, "execution artifacts")?;
            if validation.is_empty() {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    "execution validation cannot be empty",
                ));
            }
            for result in validation {
                validate_result(result)?;
            }
            match &mut values.content {
                CardContent::Sor(v) => {
                    v.summary = summary.clone();
                    v.actual_changes = changes.clone();
                    v.artifacts = artifacts.clone();
                    v.actual_validation = validation.clone();
                    Ok(None)
                }
                _ => ownership(values.kind(), "replace_execution"),
            }
        }
        SemanticOperation::RecordCloseout {
            integration_state,
            publication_state,
            merge_state,
            closeout_state,
        } => match &mut values.content {
            CardContent::Sor(v) => {
                v.integration_state = *integration_state;
                v.publication_state = *publication_state;
                v.merge_state = *merge_state;
                v.closeout_state = *closeout_state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_closeout"),
        },
        SemanticOperation::RecordPublication { state } => match &mut values.content {
            CardContent::Sor(v) => {
                v.publication_state = *state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_publication"),
        },
        SemanticOperation::RecordMerge { state } => match &mut values.content {
            CardContent::Sor(v) => {
                v.merge_state = *state;
                Ok(None)
            }
            _ => ownership(values.kind(), "record_merge"),
        },
        SemanticOperation::AdvancePhase { phase } => Ok(Some(*phase)),
    }
}

pub fn validate_identity_version(value: &str) -> Result<()> {
    let Some(rest) = value.strip_prefix('v') else {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "identity version must use vMAJOR.MINOR.PATCH form",
        ));
    };
    let parts: Vec<_> = rest.split('.').collect();
    if parts.len() != 3
        || parts
            .iter()
            .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "identity version must use vMAJOR.MINOR.PATCH form",
        ));
    }
    Ok(())
}

fn validate_status_guard(values: &CardValues, next: CardStatus) -> Result<()> {
    if matches!(next, CardStatus::Approved | CardStatus::Complete) {
        if let CardContent::Srp(srp) = &values.content {
            if srp.review_result != ReviewResult::Pass
                || srp
                    .review_revision
                    .as_deref()
                    .unwrap_or_default()
                    .is_empty()
                || srp.reviewer.as_deref().unwrap_or_default().is_empty()
                || srp.findings.iter().any(|finding| {
                    finding.actionable && finding.disposition == FindingDisposition::Open
                })
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "SRP cannot be approved/complete without current resolved review evidence",
                ));
            }
        }
    }
    if next == CardStatus::Complete {
        if let CardContent::Sor(sor) = &values.content {
            let terminal_integration = matches!(
                sor.integration_state,
                IntegrationState::Merged | IntegrationState::ClosedNoPr
            );
            let terminal_merge = matches!(
                sor.merge_state,
                MergeState::Merged | MergeState::ClosedUnmerged
            );
            let terminal_validation = terminal_validation_passed(&sor.actual_validation);
            if !terminal_integration
                || !terminal_merge
                || sor.closeout_state != CloseoutState::Complete
                || !terminal_validation
            {
                return Err(V2Error::new(
                    ErrorCode::InvalidTransition,
                    "SOR cannot be complete before terminal validation/integration/closeout truth",
                ));
            }
        }
    }
    Ok(())
}

pub(crate) fn validate_result(result: &ValidationResult) -> Result<()> {
    if result.command.is_empty()
        || result.command.iter().any(|part| part.trim().is_empty())
        || result.purpose.trim().is_empty()
        || result.evidence_ref.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "validation result requires argv, purpose, and evidence reference",
        ));
    }
    Ok(())
}

pub(crate) fn terminal_validation_passed(results: &[ValidationResult]) -> bool {
    if results.is_empty() {
        return false;
    }
    let mut latest = BTreeMap::new();
    for result in results {
        if validate_result(result).is_err() {
            return false;
        }
        latest.insert(
            (
                result.command.clone(),
                result.purpose.clone(),
                result.evidence_ref.clone(),
            ),
            result.outcome,
        );
    }
    latest.values().all(|outcome| {
        matches!(
            outcome,
            EvidenceOutcome::Passed | EvidenceOutcome::SkippedNonGoal
        )
    })
}

pub fn render(values: &CardValues) -> Result<RenderedCard> {
    validate_values(values)?;
    let kind = values.kind();
    let sections = sections(values);
    let template = template_for(kind);
    if values.identity.template_version != template.version
        || sections
            .iter()
            .map(|(heading, _)| *heading)
            .collect::<Vec<_>>()
            != template.headings
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "card template version/shape mismatch",
        ));
    }
    let mut markdown = format!(
        "# {}\n\nTemplate: {}\n\nIssue: {}\n\nRepository: {}\n\nCard: {}\n\nStatus: {}\n",
        kind.title(),
        template.version,
        values.identity.issue,
        values.identity.repository,
        kind,
        values.status
    );
    for (heading, body) in &sections {
        markdown.push_str(&format!("\n## {heading}\n\n{}\n", body.trim()));
    }
    let ast = to_mdast(&markdown, &ParseOptions::gfm()).map_err(|message| {
        V2Error::new(
            ErrorCode::CardInvalid,
            format!("markdown parse failed: {message}"),
        )
    })?;
    validate_mdast(&ast, &sections)?;
    let values_bytes = serde_json::to_vec(values)?;
    Ok(RenderedCard {
        values_digest: digest(&values_bytes),
        rendered_digest: digest(markdown.as_bytes()),
        ast_digest: digest(format!("{ast:?}").as_bytes()),
        markdown,
    })
}

pub fn validate_cross_card(
    cards: &BTreeMap<CardKind, CardValues>,
    design_ref: &str,
    design_digest: &str,
    diagram_ref: &str,
    diagram_digest: &str,
) -> Result<()> {
    if cards.len() != 6 {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "all six cards are required",
        ));
    }
    let first = cards.values().next().expect("six cards");
    if cards.iter().any(|(kind, card)| {
        card.kind() != *kind
            || card.identity.issue != first.identity.issue
            || card.identity.repository != first.identity.repository
            || card.identity.slug != first.identity.slug
            || card.identity.version != first.identity.version
    }) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "cross-card identity mismatch",
        ));
    }
    let (stp, spp, vpp) = match (
        &cards[&CardKind::Stp].content,
        &cards[&CardKind::Spp].content,
        &cards[&CardKind::Vpp].content,
    ) {
        (CardContent::Stp(stp), CardContent::Spp(spp), CardContent::Vpp(vpp)) => (stp, spp, vpp),
        _ => unreachable!(),
    };
    if spp.design_ref != design_ref
        || spp.design_digest != design_digest
        || spp.diagram_ref != diagram_ref
        || spp.diagram_digest != diagram_digest
        || vpp.design_ref != design_ref
        || vpp.design_digest != design_digest
        || vpp.diagram_ref != diagram_ref
        || vpp.diagram_digest != diagram_digest
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "design/diagram references are stale",
        ));
    }
    let acceptance_ids: Vec<String> = (1..=stp.acceptance_criteria.len())
        .map(|n| format!("AC-{n}"))
        .collect();
    let mapped: BTreeSet<_> = spp
        .steps
        .iter()
        .flat_map(|step| step.acceptance_ids.iter())
        .cloned()
        .collect();
    let accepted: BTreeSet<_> = acceptance_ids.iter().cloned().collect();
    if spp.steps.iter().any(|step| {
        step.acceptance_ids.iter().collect::<BTreeSet<_>>().len() != step.acceptance_ids.len()
    }) || acceptance_ids.iter().any(|id| !mapped.contains(id))
        || mapped.iter().any(|id| !accepted.contains(id))
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "plan-step acceptance coverage is incomplete or stale",
        ));
    }
    let proven: BTreeSet<_> = vpp
        .lanes
        .iter()
        .flat_map(|lane| lane.acceptance_ids.iter())
        .cloned()
        .collect();
    if vpp.lanes.iter().any(|lane| {
        lane.acceptance_ids.iter().collect::<BTreeSet<_>>().len() != lane.acceptance_ids.len()
    }) || acceptance_ids.iter().any(|id| !proven.contains(id))
        || proven.iter().any(|id| !accepted.contains(id))
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "VPP acceptance coverage is incomplete or stale",
        ));
    }
    let lane_seconds: u64 = vpp.lanes.iter().map(|lane| lane.budget_seconds).sum();
    let lane_tokens: u64 = vpp.lanes.iter().map(|lane| lane.budget_tokens).sum();
    if vpp.lanes.is_empty()
        || vpp
            .lanes
            .iter()
            .any(|lane| lane.argv.is_empty() || lane.budget_seconds == 0 || lane.budget_tokens == 0)
        || lane_seconds > vpp.planned_validation_seconds
        || lane_tokens > vpp.planned_validation_tokens
        || vpp.planned_validation_tokens == 0
        || spp.execution_estimates.elapsed_seconds == 0
        || spp.execution_estimates.total_tokens == 0
        || spp.execution_estimates.validation_seconds == 0
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "SPP/VPP automatic budgets or lanes are incomplete",
        ));
    }
    Ok(())
}

fn require_input(input: &InitialCardInput) -> Result<()> {
    if [
        &input.title,
        &input.slug,
        &input.version,
        &input.goal,
        &input.required_outcome,
        &input.task_boundary,
        &input.plan_summary,
        &input.failure_policy,
    ]
    .iter()
    .any(|v| v.trim().is_empty())
        || input.declared_scope.is_empty()
        || input
            .declared_scope
            .iter()
            .any(|value| value.trim().is_empty())
        || input.authority_boundary.is_empty()
        || input.operator_constraints.is_empty()
        || input.deliverables.is_empty()
        || input.acceptance_criteria.is_empty()
        || input.steps.is_empty()
        || input.affected_areas.is_empty()
        || input.affected_areas.iter().any(|value| {
            placeholder(value)
                || !Path::new(value)
                    .components()
                    .all(|component| matches!(component, Component::Normal(_)))
        })
        || input.stop_conditions.is_empty()
        || input.review_prompts.is_empty()
        || input.review_scope.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "initial typed card input is incomplete",
        ));
    }
    validate_plan_steps(&input.steps)?;
    validate_validation_lanes(&input.validation_lanes)?;
    Ok(())
}

pub(crate) fn validate_initial_input(input: &InitialCardInput) -> Result<()> {
    require_input(input)
}

fn set_text(values: &mut CardValues, field: TextField, value: String) -> Result<()> {
    if value.trim().is_empty() {
        return Err(V2Error::new(ErrorCode::CardInvalid, "empty field value"));
    }
    match (&mut values.content, field) {
        (CardContent::Sip(v), TextField::Goal) => v.goal = value,
        (CardContent::Sip(v), TextField::RequiredOutcome) => v.required_outcome = value,
        (CardContent::Stp(v), TextField::TaskBoundary) => v.task_boundary = value,
        (CardContent::Spp(v), TextField::PlanSummary) => {
            v.summary = value;
            v.plan_revision += 1;
        }
        (CardContent::Vpp(v), TextField::PlanSummary) => v.summary = value,
        (CardContent::Vpp(v), TextField::FailurePolicy) => v.failure_policy = value,
        (CardContent::Srp(v), TextField::ReviewScope) => v.review_scope = value,
        (CardContent::Sor(v), TextField::SorSummary) => v.summary = value,
        _ => return ownership(values.kind(), field.as_ref()),
    }
    Ok(())
}

fn validate_replacement(values: &[String], field: &str) -> Result<()> {
    if values.is_empty() || values.iter().any(|value| value.trim().is_empty()) {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            format!("{field} replacement cannot be empty"),
        ));
    }
    Ok(())
}

fn validate_unique_replacement(values: &[String], field: &str) -> Result<()> {
    validate_replacement(values, field)?;
    let mut normalized = BTreeSet::new();
    if values
        .iter()
        .map(|value| value.trim())
        .any(|value| !normalized.insert(value))
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            format!("{field} replacement cannot contain duplicates"),
        ));
    }
    Ok(())
}

fn replace_planning_collection(
    values: &mut CardValues,
    field: PlanningCollectionField,
    replacement: Vec<String>,
) -> Result<()> {
    match (&mut values.content, field) {
        (CardContent::Sip(v), PlanningCollectionField::DeclaredScope) => {
            v.declared_scope = replacement
        }
        (CardContent::Sip(v), PlanningCollectionField::AuthorityBoundary) => {
            v.authority_boundary = replacement
        }
        (CardContent::Sip(v), PlanningCollectionField::InitialAssumptions) => {
            v.initial_assumptions = replacement
        }
        (CardContent::Stp(v), PlanningCollectionField::Deliverables) => {
            v.deliverables = replacement
        }
        (CardContent::Stp(v), PlanningCollectionField::Dependencies) => {
            v.dependencies = replacement
        }
        (CardContent::Stp(v), PlanningCollectionField::RepoInputs) => v.repo_inputs = replacement,
        (CardContent::Stp(v), PlanningCollectionField::NonGoals) => v.non_goals = replacement,
        (CardContent::Spp(v), PlanningCollectionField::AffectedAreas) => {
            v.affected_areas = replacement;
            v.plan_revision += 1;
        }
        (CardContent::Spp(v), PlanningCollectionField::Invariants) => {
            v.invariants = replacement;
            v.plan_revision += 1;
        }
        (CardContent::Spp(v), PlanningCollectionField::Risks) => {
            v.risks = replacement;
            v.plan_revision += 1;
        }
        (CardContent::Spp(v), PlanningCollectionField::StopConditions) => {
            v.stop_conditions = replacement;
            v.plan_revision += 1;
        }
        (CardContent::Spp(v), PlanningCollectionField::ReplanTriggers) => {
            v.replan_triggers = replacement;
            v.plan_revision += 1;
        }
        (CardContent::Srp(v), PlanningCollectionField::ReviewPrompts) => {
            v.review_prompts = replacement
        }
        _ => return ownership(values.kind(), field.as_ref()),
    }
    Ok(())
}

fn validate_plan_steps(steps: &[PlanStep]) -> Result<()> {
    let ids: BTreeSet<_> = steps.iter().map(|step| step.id.as_str()).collect();
    if steps.is_empty()
        || ids.len() != steps.len()
        || steps.iter().any(|step| {
            let acceptance_ids: BTreeSet<_> =
                step.acceptance_ids.iter().map(String::as_str).collect();
            step.id.trim().is_empty()
                || step.action.trim().is_empty()
                || step.acceptance_ids.is_empty()
                || acceptance_ids.len() != step.acceptance_ids.len()
                || step.acceptance_ids.iter().any(|id| id.trim().is_empty())
                || step.status != StepStatus::Pending
        })
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "replacement plan steps must be unique, pending, and complete",
        ));
    }
    Ok(())
}

fn validate_recovery_plan_steps(steps: &[PlanStep]) -> Result<()> {
    let ids: BTreeSet<_> = steps.iter().map(|step| step.id.as_str()).collect();
    if steps.is_empty()
        || ids.len() != steps.len()
        || steps.iter().any(|step| {
            let acceptance_ids: BTreeSet<_> =
                step.acceptance_ids.iter().map(String::as_str).collect();
            step.id.trim().is_empty()
                || step.action.trim().is_empty()
                || step.acceptance_ids.is_empty()
                || acceptance_ids.len() != step.acceptance_ids.len()
                || step.acceptance_ids.iter().any(|id| id.trim().is_empty())
        })
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "post-recovery plan steps must be unique and complete",
        ));
    }
    if steps
        .iter()
        .filter(|step| step.status == StepStatus::InProgress)
        .count()
        > 1
    {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "only one plan step may be in progress",
        ));
    }
    Ok(())
}

fn validate_validation_lanes(lanes: &[ValidationLane]) -> Result<()> {
    if lanes.is_empty() {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "validation lanes cannot be empty",
        ));
    }
    let unique: BTreeSet<_> = lanes.iter().map(|lane| lane.lane.as_str()).collect();
    if unique.len() != lanes.len()
        || lanes.iter().any(|lane| {
            let acceptance_ids: BTreeSet<_> =
                lane.acceptance_ids.iter().map(String::as_str).collect();
            lane.lane.trim().is_empty()
                || lane.proof_role.trim().is_empty()
                || lane.acceptance_ids.is_empty()
                || acceptance_ids.len() != lane.acceptance_ids.len()
                || lane.acceptance_ids.iter().any(|id| id.trim().is_empty())
                || !proving_lane(lane)
                || lane.budget_seconds == 0
                || lane.budget_tokens == 0
        })
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "validation lanes must be unique and complete",
        ));
    }
    for lane in lanes {
        if let Some(classification) = classify_rust_test_selector(&lane.argv) {
            if classification.posture == RustTestSelectorPosture::Invalid {
                return Err(V2Error::new(
                    ErrorCode::CardInvalid,
                    classification
                        .diagnostic
                        .unwrap_or_else(|| "invalid Rust test selector".into()),
                ));
            }
        }
    }
    Ok(())
}

pub fn classify_rust_test_selector(argv: &[String]) -> Option<RustTestSelectorClassification> {
    let executable = argv
        .first()
        .and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str());
    if executable != Some("cargo") {
        return None;
    }
    let mut test_index = 1;
    if argv
        .get(test_index)
        .is_some_and(|value| value.starts_with('+') && value.len() > 1)
    {
        test_index += 1;
    }
    let global_flags = [
        "-v",
        "--verbose",
        "-q",
        "--quiet",
        "--frozen",
        "--locked",
        "--offline",
    ];
    let global_value_flags = ["--color", "--config", "-C", "-Z"];
    loop {
        let value = argv.get(test_index)?;
        if value == "test" || value == "t" {
            break;
        }
        if global_flags.contains(&value.as_str())
            || (value.starts_with('-')
                && value.len() > 2
                && value[1..].chars().all(|character| character == 'v'))
        {
            test_index += 1;
            continue;
        }
        if global_value_flags.contains(&value.as_str()) {
            if argv
                .get(test_index + 1)
                .is_none_or(|next| next.starts_with('-'))
            {
                return Some(invalid_rust_selector(format!(
                    "cargo global option `{value}` requires a value before `test`"
                )));
            }
            test_index += 2;
            continue;
        }
        if global_value_flags
            .iter()
            .any(|flag| value.starts_with(&format!("{flag}=")))
            || ["-C", "-Z"]
                .iter()
                .any(|flag| value.starts_with(flag) && value.len() > flag.len())
        {
            test_index += 1;
            continue;
        }
        if argv[test_index + 1..]
            .iter()
            .any(|value| value == "test" || value == "t")
        {
            return Some(invalid_rust_selector(format!(
                "unrecognized cargo global option `{value}` before `test`/`t`; use a supported typed argv shape"
            )));
        }
        return None;
    }

    let exact_target_flags = ["--lib", "--doc"];
    let broad_target_flags = [
        "--bins",
        "--tests",
        "--examples",
        "--benches",
        "--all-targets",
    ];
    let named_target_flags = ["--test", "--bin", "--example", "--bench"];
    let value_flags = [
        "--manifest-path",
        "--package",
        "-p",
        "--exclude",
        "--features",
        "-F",
        "--target",
        "--target-dir",
        "--lockfile-path",
        "--profile",
        "--jobs",
        "-j",
        "--color",
        "--config",
        "--message-format",
        "-Z",
    ];
    let mut target_boundaries = 0_u8;
    let mut broad_target_sets = 0_u8;
    let mut filter = None;
    let mut libtest_filter = None;
    let mut index = test_index + 1;
    while index < argv.len() {
        let value = &argv[index];
        if value == "--" {
            index += 1;
            let libtest_value_flags = ["--skip", "--test-threads", "--format", "--color", "-Z"];
            while index < argv.len() {
                let value = &argv[index];
                if libtest_value_flags.contains(&value.as_str()) {
                    if argv.get(index + 1).is_none_or(|next| next.starts_with('-')) {
                        return Some(invalid_rust_selector(format!(
                            "cargo test libtest option `{value}` requires a value"
                        )));
                    }
                    index += 2;
                    continue;
                }
                if libtest_value_flags
                    .iter()
                    .any(|flag| value.starts_with(&format!("{flag}=")))
                {
                    index += 1;
                    continue;
                }
                if value.starts_with('-') {
                    index += 1;
                    continue;
                }
                if libtest_filter.replace(value.as_str()).is_some() {
                    return Some(invalid_rust_selector(
                        "cargo test lane contains multiple libtest substring selectors; declare one exact Cargo target boundary"
                            .into(),
                    ));
                }
                index += 1;
            }
            break;
        }
        if exact_target_flags.contains(&value.as_str()) {
            target_boundaries += 1;
            index += 1;
            continue;
        }
        if broad_target_flags.contains(&value.as_str()) {
            broad_target_sets += 1;
            index += 1;
            continue;
        }
        if named_target_flags.contains(&value.as_str()) {
            let Some(name) = argv.get(index + 1).filter(|name| !name.starts_with('-')) else {
                return Some(invalid_rust_selector(format!(
                    "cargo test selector `{value}` requires a target name"
                )));
            };
            if name.trim().is_empty() {
                return Some(invalid_rust_selector(format!(
                    "cargo test selector `{value}` requires a non-empty target name"
                )));
            }
            target_boundaries += 1;
            index += 2;
            continue;
        }
        if named_target_flags
            .iter()
            .any(|flag| value.starts_with(&format!("{flag}=")))
        {
            if value.ends_with('=') {
                return Some(invalid_rust_selector(format!(
                    "cargo test selector `{value}` requires a target name"
                )));
            }
            target_boundaries += 1;
            index += 1;
            continue;
        }
        if value_flags.contains(&value.as_str()) {
            if argv.get(index + 1).is_none_or(|next| next.starts_with('-')) {
                return Some(invalid_rust_selector(format!(
                    "cargo test option `{value}` requires a value"
                )));
            }
            index += 2;
            continue;
        }
        if value_flags
            .iter()
            .any(|flag| value.starts_with(&format!("{flag}=")))
            || ["-p", "-F", "-j", "-Z"]
                .iter()
                .any(|flag| value.starts_with(flag) && value.len() > flag.len())
        {
            index += 1;
            continue;
        }
        if value.starts_with('-') {
            index += 1;
            continue;
        }
        if filter.replace(value.as_str()).is_some() {
            return Some(invalid_rust_selector(
                "cargo test lane contains multiple free substring selectors; declare one exact target with `--lib`, `--test <name>`, `--bin <name>`, `--example <name>`, or `--bench <name>`"
                    .into(),
            ));
        }
        index += 1;
    }

    if target_boundaries + broad_target_sets > 1 {
        return Some(invalid_rust_selector(
            "cargo test lane has conflicting target selectors; declare exactly one target boundary"
                .into(),
        ));
    }
    if target_boundaries == 0 {
        if let Some(filter) = filter.or(libtest_filter) {
            return Some(invalid_rust_selector(format!(
                "cargo test lane uses free substring selector `{}` without a target boundary; use `--lib {}` or `--test <target>`",
                filter, filter
            )));
        }
    }
    Some(RustTestSelectorClassification {
        posture: if target_boundaries == 0 {
            RustTestSelectorPosture::IntentionalBroad
        } else {
            RustTestSelectorPosture::ExactTarget
        },
        diagnostic: None,
    })
}

fn invalid_rust_selector(diagnostic: String) -> RustTestSelectorClassification {
    RustTestSelectorClassification {
        posture: RustTestSelectorPosture::Invalid,
        diagnostic: Some(diagnostic),
    }
}

fn placeholder(value: &str) -> bool {
    let value = value.trim().to_ascii_lowercase();
    value.is_empty()
        || matches!(
            value.as_str(),
            "none" | "n/a" | "na" | "tbd" | "todo" | "unknown" | "placeholder"
        )
        || value.contains("<placeholder>")
        || value.contains("${")
}

fn proving_lane(lane: &ValidationLane) -> bool {
    let executable = lane
        .argv
        .first()
        .and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    !placeholder(&lane.proof_role)
        && !lane.proof_role.to_ascii_lowercase().contains("non-proving")
        && !lane.argv.iter().any(|value| placeholder(value))
        && !lane
            .argv
            .iter()
            .any(|value| matches!(value.as_str(), "--help" | "-h" | "--version" | "-V"))
        && !matches!(
            executable.as_str(),
            "true" | "false" | "echo" | "printf" | "sleep" | ":" | "sh" | "zsh" | "env"
        )
}

fn executable_file(path: &Path) -> bool {
    let Ok(metadata) = std::fs::metadata(path) else {
        return false;
    };
    if !metadata.is_file() {
        return false;
    }
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        metadata.permissions().mode() & 0o111 != 0
    }
    #[cfg(not(unix))]
    {
        true
    }
}

fn proving_lane_at(root: &Path, lane: &ValidationLane, owned_paths: &[String]) -> bool {
    let executable = lane
        .argv
        .first()
        .and_then(|value| Path::new(value).file_name())
        .and_then(|value| value.to_str())
        .unwrap_or_default()
        .to_ascii_lowercase();
    let executable_exists = if lane.argv.first().is_none_or(String::is_empty) {
        false
    } else if lane.argv[0].contains('/') {
        executable_file(&root.join(&lane.argv[0]))
    } else {
        std::env::var_os("PATH").is_some_and(|path| {
            std::env::split_paths(&path)
                .any(|directory| executable_file(&directory.join(&lane.argv[0])))
        })
    };
    let governed_shell_script = executable != "bash"
        || lane.argv.get(1).is_some_and(|script| {
            script.ends_with(".sh")
                && owned_paths.iter().any(|owned| owned == script)
                && owned_path_at(root, script)
        });
    executable_exists && governed_shell_script && proving_lane(lane)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExecutionReadinessFinding {
    pub code: &'static str,
    pub message: String,
}

fn cargo_package_root(root: &Path, manifest: Option<&str>, package: &str) -> Option<PathBuf> {
    let canonical_root = root.canonicalize().ok()?;
    let manifest = manifest.unwrap_or("Cargo.toml");
    let manifest_path = canonical_root.join(manifest);
    if !manifest_path.is_file() {
        return None;
    }
    let output = Command::new("cargo")
        .current_dir(&canonical_root)
        .args([
            "metadata",
            "--no-deps",
            "--format-version",
            "1",
            "--manifest-path",
        ])
        .arg(&manifest_path)
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let package_manifest = metadata
        .get("packages")?
        .as_array()?
        .iter()
        .find(|candidate| candidate.get("name").and_then(|name| name.as_str()) == Some(package))?
        .get("manifest_path")?
        .as_str()?;
    Path::new(package_manifest)
        .parent()?
        .strip_prefix(&canonical_root)
        .ok()
        .map(Path::to_path_buf)
}

fn validator_targets(root: &Path, lane: &ValidationLane) -> Vec<String> {
    let Some(executable) = lane.argv.first().map(String::as_str) else {
        return Vec::new();
    };
    if executable == "cargo" {
        let manifest = lane
            .argv
            .windows(2)
            .find_map(|pair| (pair[0] == "--manifest-path").then_some(pair[1].as_str()))
            .or_else(|| {
                lane.argv
                    .iter()
                    .find_map(|value| value.strip_prefix("--manifest-path="))
            });
        let package = lane
            .argv
            .windows(2)
            .find_map(|pair| {
                matches!(pair[0].as_str(), "-p" | "--package").then_some(pair[1].as_str())
            })
            .or_else(|| {
                lane.argv
                    .iter()
                    .find_map(|value| value.strip_prefix("--package="))
            });
        let crate_root = if let Some(package) = package {
            let Some(crate_root) = cargo_package_root(root, manifest, package) else {
                return Vec::new();
            };
            crate_root
        } else {
            manifest
                .map(Path::new)
                .and_then(Path::parent)
                .unwrap_or_else(|| Path::new(""))
                .to_path_buf()
        };
        let mut targets = Vec::new();
        for (index, value) in lane.argv.iter().enumerate() {
            let name = if value == "--test" {
                lane.argv.get(index + 1).map(String::as_str)
            } else {
                value.strip_prefix("--test=")
            };
            if let Some(name) = name {
                targets.push(
                    crate_root
                        .join("tests")
                        .join(format!("{name}.rs"))
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
        return targets;
    }
    let executable_name = Path::new(executable)
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or_default();
    if matches!(
        executable_name,
        "bash" | "zsh" | "node" | "ruby" | "python" | "python3"
    ) {
        return lane
            .argv
            .get(1)
            .filter(|value| !value.starts_with('-'))
            .cloned()
            .into_iter()
            .collect();
    }
    executable
        .contains('/')
        .then(|| executable.to_owned())
        .into_iter()
        .collect()
}

fn explicitly_deferred_validator(
    root: &Path,
    path: &str,
    lane: &ValidationLane,
    owned_paths: &[String],
    deliverables: &[String],
    failure_policy: &str,
    allow_deferred: bool,
) -> bool {
    allow_deferred
        && validator_targets(root, lane)
            .iter()
            .any(|target| target == path)
        && owned_paths.iter().any(|owned| owned == path)
        && deliverables.iter().any(|deliverable| deliverable == path)
        && lane
            .defer_reason
            .as_deref()
            .is_some_and(|reason| !placeholder(reason))
        && fail_closed_policy(failure_policy)
}

fn fail_closed_policy(value: &str) -> bool {
    let words = value
        .split(|character: char| !character.is_ascii_alphanumeric())
        .filter(|word| !word.is_empty())
        .map(str::to_ascii_lowercase)
        .collect::<Vec<_>>();
    matches!(words.as_slice(), [first, second, ..] if first == "fail" && second == "closed")
        && !words[2..].iter().any(|word| {
            matches!(
                word.as_str(),
                "unless" | "except" | "excepting" | "however" | "but"
            )
        })
}

fn required_validator_deliverable(path: &str) -> bool {
    if path.chars().any(char::is_whitespace)
        || !Path::new(path)
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        || !(path.contains('/') || Path::new(path).extension().is_some())
    {
        return false;
    }
    let path = Path::new(path);
    let validator_words = [
        "test",
        "tests",
        "spec",
        "validate",
        "validation",
        "validator",
        "verify",
        "check",
        "proof",
    ];
    path.components().any(|component| {
        component
            .as_os_str()
            .to_str()
            .is_some_and(|value| validator_words.contains(&value.to_ascii_lowercase().as_str()))
    }) || path
        .file_stem()
        .and_then(|value| value.to_str())
        .is_some_and(|stem| {
            stem.split(|character: char| !character.is_ascii_alphanumeric())
                .any(|word| validator_words.contains(&word.to_ascii_lowercase().as_str()))
        })
}

fn rust_source_crate_root(root: &Path, path: &str) -> Option<PathBuf> {
    let relative = Path::new(path);
    if relative.extension().and_then(|value| value.to_str()) != Some("rs") {
        return None;
    }
    let components = relative.components().collect::<Vec<_>>();
    let src_index = components
        .iter()
        .enumerate()
        .find_map(|(index, component)| {
            if component.as_os_str() != "src" || index + 1 >= components.len() {
                return None;
            }
            let crate_root = components[..index]
                .iter()
                .fold(PathBuf::new(), |mut path, part| {
                    path.push(part.as_os_str());
                    path
                });
            root.join(&crate_root)
                .join("Cargo.toml")
                .is_file()
                .then_some(crate_root)
        });
    src_index
}

fn explicitly_deferred_rust_path_harness(
    root: &Path,
    source_path: &str,
    lanes: &[ValidationLane],
    owned_paths: &[String],
    deliverables: &[String],
    failure_policy: &str,
    allow_deferred: bool,
) -> bool {
    if !allow_deferred
        || owned_paths
            .iter()
            .filter(|path| !rust_module_route_owned(root, path, owned_paths))
            .count()
            != 1
        || root.join(source_path).exists()
        || !owned_paths.iter().any(|owned| owned == source_path)
        || !deliverables
            .iter()
            .any(|deliverable| deliverable == source_path)
    {
        return false;
    }
    let Some(crate_root) = rust_source_crate_root(root, source_path) else {
        return false;
    };
    let tests_root = crate_root.join("tests");
    lanes.iter().any(|lane| {
        let is_exact_nextest_lane = matches!(lane.argv.as_slice(), [cargo, nextest, run, ..]
            if cargo == "cargo" && nextest == "nextest" && run == "run")
            && lane
                .argv
                .iter()
                .any(|argument| argument == "--no-tests=fail");
        let targets = validator_targets(root, lane);
        let Some(target) = targets.first() else {
            return false;
        };
        let target_path = Path::new(target);
        let exact_same_crate_test = targets.len() == 1
            && target_path.parent() == Some(tests_root.as_path())
            && target_path.extension().and_then(|value| value.to_str()) == Some("rs");
        let explicit_harness = lane.defer_reason.as_deref().is_some_and(|reason| {
            reason.contains("#[path") && reason.contains(source_path) && !placeholder(reason)
        });
        is_exact_nextest_lane
            && exact_same_crate_test
            && explicit_harness
            && proving_lane_at(root, lane, owned_paths)
            && explicitly_deferred_validator(
                root,
                target,
                lane,
                owned_paths,
                deliverables,
                failure_policy,
                allow_deferred,
            )
    })
}

fn rust_module_route_owned(root: &Path, path: &str, owned_paths: &[String]) -> bool {
    let relative = Path::new(path);
    if root.join(relative).is_file()
        || relative.extension().and_then(|value| value.to_str()) != Some("rs")
    {
        return true;
    }
    let components = relative.components().collect::<Vec<_>>();
    let Some(src_index) = components
        .iter()
        .enumerate()
        .find_map(|(index, component)| {
            if component.as_os_str() != "src" {
                return None;
            }
            let crate_root =
                components[..index]
                    .iter()
                    .fold(PathBuf::new(), |mut path, component| {
                        path.push(component.as_os_str());
                        path
                    });
            root.join(crate_root)
                .join("Cargo.toml")
                .is_file()
                .then_some(index)
        })
    else {
        return true;
    };
    let after_src = &components[src_index + 1..];
    if after_src.is_empty()
        || (after_src.len() == 1
            && matches!(
                after_src[0].as_os_str().to_str(),
                Some("lib.rs" | "main.rs")
            ))
    {
        return true;
    }
    if after_src
        .first()
        .is_some_and(|part| part.as_os_str() == "bin")
        && (after_src.len() == 2
            || (after_src.len() == 3
                && after_src
                    .last()
                    .is_some_and(|part| part.as_os_str() == "main.rs")))
    {
        return true;
    }
    let src_root = components[..=src_index]
        .iter()
        .fold(PathBuf::new(), |mut path, component| {
            path.push(component.as_os_str());
            path
        });
    let is_mod_root = after_src
        .last()
        .is_some_and(|part| part.as_os_str() == "mod.rs");
    let parent_end = after_src.len() - usize::from(is_mod_root) - 1;
    let parent_parts = &after_src[..parent_end];
    let mut candidates = if parent_parts.is_empty() {
        vec![src_root.join("lib.rs"), src_root.join("main.rs")]
    } else {
        let parent_root = parent_parts[..parent_parts.len() - 1].iter().fold(
            src_root.clone(),
            |mut path, component| {
                path.push(component.as_os_str());
                path
            },
        );
        let parent = parent_parts
            .iter()
            .fold(PathBuf::new(), |mut path, component| {
                path.push(component.as_os_str());
                path
            });
        let module = parent_parts[parent_parts.len() - 1].as_os_str();
        vec![
            parent_root.join(format!("{}.rs", module.to_string_lossy())),
            src_root.join(&parent).join("mod.rs"),
        ]
    };
    if after_src
        .first()
        .is_some_and(|part| part.as_os_str() == "bin")
        && parent_parts.len() >= 2
    {
        candidates.push(
            src_root
                .join(
                    parent_parts
                        .iter()
                        .fold(PathBuf::new(), |mut path, component| {
                            path.push(component.as_os_str());
                            path
                        }),
                )
                .join("main.rs"),
        );
    }
    candidates.iter().any(|candidate| {
        let candidate = candidate.to_string_lossy();
        root.join(candidate.as_ref()).is_file()
            && owned_paths.iter().any(|owned| owned == candidate.as_ref())
    })
}

fn owned_path_at(root: &Path, value: &str) -> bool {
    let relative = Path::new(value);
    if placeholder(value)
        || !relative
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
    {
        return false;
    }
    match fs::symlink_metadata(root) {
        Ok(metadata) if metadata.is_dir() && !metadata.file_type().is_symlink() => {}
        _ => return false,
    }
    let canonical_root = match root.canonicalize() {
        Ok(root) => root,
        Err(_) => return false,
    };
    let components: Vec<_> = relative.components().collect();
    let mut current = root.to_path_buf();
    let mut saw_existing_relative_prefix = false;
    for (index, component) in components.iter().enumerate() {
        let Component::Normal(part) = component else {
            return false;
        };
        current.push(part);
        match fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink()
                    || (index + 1 < components.len() && !metadata.is_dir())
                {
                    return false;
                }
                saw_existing_relative_prefix = true;
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                if !saw_existing_relative_prefix {
                    return false;
                }
                let Some(existing) = current.parent() else {
                    return false;
                };
                return existing
                    .canonicalize()
                    .is_ok_and(|ancestor| ancestor.starts_with(&canonical_root));
            }
            Err(_) => return false,
        }
    }
    saw_existing_relative_prefix
        && current
            .canonicalize()
            .is_ok_and(|owned| owned.starts_with(&canonical_root))
}

pub(crate) fn execution_readiness_findings_for_cards(
    root: &Path,
    cards: &BTreeMap<CardKind, CardValues>,
    phase: LifecyclePhase,
) -> Result<Vec<ExecutionReadinessFinding>> {
    let affected_areas = match &cards
        .get(&CardKind::Spp)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "SPP projection missing"))?
        .content
    {
        CardContent::Spp(values) => &values.affected_areas,
        _ => unreachable!("SPP projection"),
    };
    let deliverables = match &cards
        .get(&CardKind::Stp)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "STP projection missing"))?
        .content
    {
        CardContent::Stp(values) => &values.deliverables,
        _ => unreachable!("STP projection"),
    };
    let vpp = match &cards
        .get(&CardKind::Vpp)
        .ok_or_else(|| V2Error::new(ErrorCode::CardInvalid, "VPP projection missing"))?
        .content
    {
        CardContent::Vpp(values) => values,
        _ => unreachable!("VPP projection"),
    };
    let owned_paths_are_valid = !affected_areas.is_empty()
        && affected_areas
            .iter()
            .all(|value| owned_path_at(root, value));
    Ok(execution_readiness_findings(
        root,
        affected_areas,
        deliverables,
        &vpp.lanes,
        &vpp.failure_policy,
        owned_paths_are_valid,
        matches!(phase, LifecyclePhase::Initialized | LifecyclePhase::Ready),
    ))
}

fn execution_readiness_findings(
    root: &Path,
    affected_areas: &[String],
    deliverables: &[String],
    lanes: &[ValidationLane],
    failure_policy: &str,
    owned_paths_are_valid: bool,
    allow_deferred: bool,
) -> Vec<ExecutionReadinessFinding> {
    let mut findings = Vec::new();
    if !owned_paths_are_valid {
        findings.push(ExecutionReadinessFinding {
            code: "owned_paths_invalid",
            message: "execution readiness requires non-empty safe repository-owned paths".into(),
        });
    }
    for path in affected_areas {
        if !rust_module_route_owned(root, path, affected_areas)
            && !explicitly_deferred_rust_path_harness(
                root,
                path,
                lanes,
                affected_areas,
                deliverables,
                failure_policy,
                allow_deferred,
            )
        {
            findings.push(ExecutionReadinessFinding {
                code: "owned_rust_module_unroutable",
                message: format!("new Rust module requires an owned existing module route: {path}"),
            });
        }
    }
    let mut issue_specific_denominator = false;
    if lanes.is_empty() {
        findings.push(ExecutionReadinessFinding {
            code: "validation_lanes_missing",
            message: "at least one proving validation lane is required".into(),
        });
    }
    for lane in lanes {
        if !proving_lane_at(root, lane, affected_areas) {
            findings.push(ExecutionReadinessFinding {
                code: "validation_lane_non_proving",
                message: format!(
                    "validation lane is not executable and governed: {}",
                    lane.lane
                ),
            });
        }
        let targets = validator_targets(root, lane);
        for target in &targets {
            let exists = root.join(target).is_file();
            let deferred = explicitly_deferred_validator(
                root,
                target,
                lane,
                affected_areas,
                deliverables,
                failure_policy,
                allow_deferred,
            );
            if !exists && !deferred {
                findings.push(ExecutionReadinessFinding {
                    code: "validator_target_missing",
                    message: format!(
                        "validation lane {} requires unavailable target {target}",
                        lane.lane
                    ),
                });
            }
            if affected_areas.iter().any(|owned| owned == target) && (exists || deferred) {
                issue_specific_denominator = true;
            }
        }
        if targets.is_empty()
            && lane.argv.iter().any(|argument| {
                affected_areas.iter().any(|owned| owned == argument)
                    && root.join(argument).is_file()
            })
        {
            issue_specific_denominator = true;
        }
    }
    let selected_targets = lanes
        .iter()
        .flat_map(|lane| validator_targets(root, lane))
        .collect::<BTreeSet<_>>();
    for validator in deliverables
        .iter()
        .filter(|path| required_validator_deliverable(path))
    {
        if !affected_areas.iter().any(|owned| owned == validator) {
            findings.push(ExecutionReadinessFinding {
                code: "validator_deliverable_unowned",
                message: format!(
                    "required validator deliverable is not an issue-owned path: {validator}"
                ),
            });
            continue;
        }
        let exists = root.join(validator).is_file();
        let deferred = lanes.iter().any(|lane| {
            explicitly_deferred_validator(
                root,
                validator,
                lane,
                affected_areas,
                deliverables,
                failure_policy,
                allow_deferred,
            )
        });
        if !exists && !deferred && !selected_targets.contains(validator) {
            findings.push(ExecutionReadinessFinding {
                code: "validator_target_missing",
                message: format!("required validator deliverable is unavailable: {validator}"),
            });
        }
    }
    if !issue_specific_denominator {
        findings.push(ExecutionReadinessFinding {
            code: "issue_specific_denominator_missing",
            message:
                "validation lanes select no available or explicitly deferred issue-owned target"
                    .into(),
        });
    }
    findings
}

pub(crate) fn replace_acceptance_plan(
    cards: &mut BTreeMap<CardKind, CardValues>,
    acceptance_criteria: &[String],
    steps: &[PlanStep],
    validation_lanes: &[ValidationLane],
) -> Result<()> {
    validate_replacement(acceptance_criteria, "acceptance criteria")?;
    validate_plan_steps(steps)?;
    validate_validation_lanes(validation_lanes)?;

    match &mut cards
        .get_mut(&CardKind::Stp)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "STP projection missing"))?
        .content
    {
        CardContent::Stp(values) => values.acceptance_criteria = acceptance_criteria.to_vec(),
        _ => unreachable!("STP card content"),
    }
    match &mut cards
        .get_mut(&CardKind::Spp)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "SPP projection missing"))?
        .content
    {
        CardContent::Spp(values) => {
            values.steps = steps.to_vec();
            values.plan_revision += 1;
        }
        _ => unreachable!("SPP card content"),
    }
    match &mut cards
        .get_mut(&CardKind::Vpp)
        .ok_or_else(|| V2Error::new(ErrorCode::CorruptRecord, "VPP projection missing"))?
        .content
    {
        CardContent::Vpp(values) => values.lanes = validation_lanes.to_vec(),
        _ => unreachable!("VPP card content"),
    }
    Ok(())
}

fn append_reference(values: &mut CardValues, value: String) -> Result<()> {
    match &mut values.content {
        CardContent::Stp(v) => v.repo_inputs.push(value),
        CardContent::Srp(v) => v.residual_risk.push(value),
        CardContent::Sor(v) => v.follow_ups.push(value),
        _ => return ownership(values.kind(), "append_reference"),
    }
    Ok(())
}

fn ownership<T>(kind: CardKind, operation: &str) -> Result<T> {
    Err(V2Error::new(
        ErrorCode::FieldOwnership,
        format!("{kind} does not own {operation}"),
    ))
}

fn validate_values(values: &CardValues) -> Result<()> {
    if values.identity.schema_version != "1.0.0"
        || values.identity.template_version != "1.0.0"
        || values.identity.issue == 0
        || values.identity.repository.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "invalid card identity/schema",
        ));
    }
    if let CardContent::Sor(sor) = &values.content {
        for result in &sor.actual_validation {
            validate_result(result)?;
        }
    }
    if let CardContent::Spp(spp) = &values.content {
        if let Some(estimate) = &spp.execution_estimates.advisory {
            validate_accepted_estimate(estimate)?;
        }
    }
    Ok(())
}

fn sections(values: &CardValues) -> Vec<(&'static str, String)> {
    match &values.content {
        CardContent::Sip(v) => vec![
            ("Goal", v.goal.clone()),
            ("Required Outcome", v.required_outcome.clone()),
            ("Scope", bullets(&v.declared_scope)),
            ("Authority", bullets(&v.authority_boundary)),
            ("Assumptions", bullets(&v.initial_assumptions)),
            ("Operator Constraints", bullets(&v.operator_constraints)),
        ],
        CardContent::Stp(v) => vec![
            ("Task", v.task_boundary.clone()),
            ("Deliverables", bullets(&v.deliverables)),
            ("Acceptance", numbered(&v.acceptance_criteria)),
            ("Dependencies", bullets(&v.dependencies)),
            ("Inputs", bullets(&v.repo_inputs)),
            ("Non Goals", bullets(&v.non_goals)),
        ],
        CardContent::Spp(v) => vec![
            ("Summary", v.summary.clone()),
            ("Plan", format!("Revision {}", v.plan_revision)),
            (
                "Steps",
                serde_json::to_string_pretty(&v.steps).expect("steps"),
            ),
            ("Invariants", bullets(&v.invariants)),
            ("Risks", bullets(&v.risks)),
            (
                "Estimates",
                serde_json::to_string_pretty(&v.execution_estimates).expect("estimates"),
            ),
            (
                "Design",
                format!("{}\n\nDigest: {}", v.design_ref, v.design_digest),
            ),
            (
                "Diagram",
                format!("{}\n\nDigest: {}", v.diagram_ref, v.diagram_digest),
            ),
            ("Stop Conditions", bullets(&v.stop_conditions)),
            ("Handoff", "Proceed only after doctor readiness.".into()),
        ],
        CardContent::Vpp(v) => vec![
            ("Summary", v.summary.clone()),
            (
                "Lane Inputs",
                format!("Design: {}\n\nDiagram: {}", v.design_ref, v.diagram_ref),
            ),
            (
                "Selected Lanes",
                serde_json::to_string_pretty(&v.lanes).expect("lanes"),
            ),
            (
                "Parallelization",
                "Only declared parallel groups may overlap.".into(),
            ),
            (
                "Budgets",
                format!(
                    "Seconds: {}\n\nTokens: {}",
                    v.planned_validation_seconds, v.planned_validation_tokens
                ),
            ),
            (
                "Commands",
                v.lanes
                    .iter()
                    .map(|lane| format!("- `{}`", lane.argv.join(" ")))
                    .collect::<Vec<_>>()
                    .join("\n"),
            ),
            ("Failure Semantics", v.failure_policy.clone()),
            (
                "Handoff",
                "Retain typed evidence before convergence.".into(),
            ),
        ],
        CardContent::Srp(v) => vec![
            ("Scope", v.review_scope.clone()),
            ("Prompts", bullets(&v.review_prompts)),
            (
                "Findings",
                serde_json::to_string_pretty(&v.findings).expect("findings"),
            ),
            (
                "Dispositions",
                "Every actionable finding requires a terminal disposition.".into(),
            ),
            ("Residual Risk", bullets(&v.residual_risk)),
            (
                "Review Result",
                format!(
                    "Revision: {:?}\n\nReviewer: {:?}\n\nResult: {}",
                    v.review_revision, v.reviewer, v.review_result
                ),
            ),
        ],
        CardContent::Sor(v) => vec![
            ("Summary", v.summary.clone()),
            ("Artifacts", bullets(&v.artifacts)),
            ("Execution", bullets(&v.actual_changes)),
            (
                "Validation",
                serde_json::to_string_pretty(&v.actual_validation).expect("validation"),
            ),
            ("Integration", v.integration_state.to_string()),
            (
                "Publication",
                format!(
                    "Publication: {}\n\nMerge: {}",
                    v.publication_state, v.merge_state
                ),
            ),
            ("Closeout", v.closeout_state.to_string()),
            ("Follow Ups", bullets(&v.follow_ups)),
        ],
    }
}

fn validate_mdast(ast: &Node, expected: &[(&str, String)]) -> Result<()> {
    let children = match ast {
        Node::Root(root) => &root.children,
        _ => return Err(V2Error::new(ErrorCode::CardInvalid, "mdast root missing")),
    };
    let mut headings = Vec::new();
    for child in children {
        if let Node::Heading(heading) = child {
            if heading.depth == 2 {
                let text = heading
                    .children
                    .iter()
                    .filter_map(|node| match node {
                        Node::Text(text) => Some(text.value.as_str()),
                        _ => None,
                    })
                    .collect::<String>();
                headings.push(text);
            }
        }
    }
    let expected_headings: Vec<_> = expected
        .iter()
        .map(|(heading, _)| (*heading).to_string())
        .collect();
    if headings != expected_headings {
        return Err(V2Error::new(
            ErrorCode::CardInvalid,
            "mdast semantic anchors mismatch",
        ));
    }
    Ok(())
}

fn bullets(values: &[String]) -> String {
    if values.is_empty() {
        "- none".into()
    } else {
        values
            .iter()
            .map(|value| format!("- {value}"))
            .collect::<Vec<_>>()
            .join("\n")
    }
}
fn numbered(values: &[String]) -> String {
    values
        .iter()
        .enumerate()
        .map(|(index, value)| format!("{}. {value}", index + 1))
        .collect::<Vec<_>>()
        .join("\n")
}
pub fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::{
        owned_path_at, proving_lane_at, terminal_validation_passed, validate_validation_lanes,
        ErrorCode, EvidenceOutcome, ResourceProfile, ValidationLane, ValidationResult,
    };

    fn lane(argv: &[&str]) -> ValidationLane {
        ValidationLane {
            lane: "focused".into(),
            proof_role: "execute a governed repository validation".into(),
            acceptance_ids: vec!["AC-1".into()],
            deterministic: true,
            resource_profile: ResourceProfile::Small,
            budget_seconds: 60,
            budget_tokens: 500,
            argv: argv.iter().map(|value| (*value).into()).collect(),
            parallel_group: "local".into(),
            defer_reason: None,
        }
    }

    fn result(outcome: EvidenceOutcome) -> ValidationResult {
        ValidationResult {
            command: vec!["cargo".into(), "test".into()],
            purpose: "proof".into(),
            outcome,
            evidence_ref: "evidence.json".into(),
        }
    }

    #[test]
    fn latest_pass_supersedes_waiting_for_the_same_validation() {
        assert!(terminal_validation_passed(&[
            result(EvidenceOutcome::Waiting),
            result(EvidenceOutcome::Passed),
        ]));
    }

    #[test]
    fn latest_failure_supersedes_an_earlier_pass() {
        assert!(!terminal_validation_passed(&[
            result(EvidenceOutcome::Passed),
            result(EvidenceOutcome::Failed),
        ]));
    }

    #[test]
    fn owned_paths_accept_real_files_and_safe_future_descendants() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("existing/deep")).expect("existing directory");
        fs::write(temp.path().join("existing/file.rs"), "proof\n").expect("existing file");

        assert!(owned_path_at(temp.path(), "existing/file.rs"));
        assert!(owned_path_at(temp.path(), "existing/deep/future/module.rs"));
        assert!(!owned_path_at(temp.path(), "missing/future.rs"));
        assert!(!owned_path_at(temp.path(), "existing/file.rs/future.rs"));
        assert!(!owned_path_at(temp.path(), "../escape.rs"));
        assert!(!owned_path_at(temp.path(), "/absolute/escape.rs"));
        assert!(!owned_path_at(temp.path(), "<owned-path>"));
        assert!(!owned_path_at(
            temp.path(),
            "SERIALIZATION_GATE {\"paths\":[\"existing/file.rs\"]}"
        ));
    }

    #[test]
    fn bash_lane_requires_a_governed_repository_script() {
        let temp = tempfile::tempdir().expect("tempdir");
        fs::create_dir_all(temp.path().join("tools")).expect("tools directory");

        let owned = vec!["tools/future-validation.sh".to_string()];
        assert!(proving_lane_at(
            temp.path(),
            &lane(&["bash", "tools/future-validation.sh"]),
            &owned
        ));
        assert!(!proving_lane_at(temp.path(), &lane(&["bash"]), &owned));
        assert!(!proving_lane_at(
            temp.path(),
            &lane(&["bash", "tools/unowned-validation.sh"]),
            &owned
        ));
        assert!(!proving_lane_at(
            temp.path(),
            &lane(&["bash", "-c", "echo unsafe"]),
            &owned
        ));
        assert!(!proving_lane_at(
            temp.path(),
            &lane(&["bash", "../outside.sh"]),
            &["../outside.sh".into()]
        ));
    }

    #[test]
    fn validation_lane_rejects_free_rust_test_substring_before_execution() {
        let ambiguous = lane(&[
            "cargo",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "schema",
        ]);
        let error = validate_validation_lanes(&[ambiguous]).expect_err("ambiguous selector");
        assert_eq!(error.code, ErrorCode::CardInvalid);
        assert!(error.message.contains("without a target boundary"));

        let separator_bypass = lane(&[
            "cargo",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "--",
            "schema",
            "--list",
        ]);
        let error = validate_validation_lanes(&[separator_bypass]).expect_err("separator bypass");
        assert_eq!(error.code, ErrorCode::CardInvalid);
        assert!(error.message.contains("without a target boundary"));

        let global_option_bypass = lane(&[
            "cargo",
            "--locked",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "schema",
        ]);
        let error =
            validate_validation_lanes(&[global_option_bypass]).expect_err("global option bypass");
        assert_eq!(error.code, ErrorCode::CardInvalid);
        assert!(error.message.contains("without a target boundary"));

        let broad_target_bypass = lane(&[
            "cargo",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "--tests",
            "schema",
        ]);
        let error =
            validate_validation_lanes(&[broad_target_bypass]).expect_err("broad target bypass");
        assert_eq!(error.code, ErrorCode::CardInvalid);
        assert!(error.message.contains("without a target boundary"));

        assert!(validate_validation_lanes(&[lane(&[
            "cargo",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
        ])])
        .is_ok());
        assert!(validate_validation_lanes(&[lane(&[
            "cargo",
            "test",
            "--manifest-path",
            "csdlc-v2/Cargo.toml",
            "--lib",
            "schema::tests",
        ])])
        .is_ok());
    }

    #[cfg(unix)]
    #[test]
    fn owned_paths_reject_every_symlink_component() {
        use std::os::unix::fs::symlink;

        let temp = tempfile::tempdir().expect("tempdir");
        let outside = tempfile::tempdir().expect("outside tempdir");
        fs::create_dir_all(temp.path().join("real/inside")).expect("inside directory");
        fs::write(temp.path().join("real/leaf.rs"), "proof\n").expect("leaf");
        symlink(outside.path(), temp.path().join("outside-link")).expect("outside symlink");
        symlink(
            temp.path().join("real/inside"),
            temp.path().join("inside-link"),
        )
        .expect("inside symlink");
        symlink(
            temp.path().join("real/leaf.rs"),
            temp.path().join("leaf-link.rs"),
        )
        .expect("leaf symlink");

        assert!(!owned_path_at(temp.path(), "outside-link/future.rs"));
        assert!(!owned_path_at(temp.path(), "inside-link/future.rs"));
        assert!(!owned_path_at(temp.path(), "leaf-link.rs"));
    }
}
