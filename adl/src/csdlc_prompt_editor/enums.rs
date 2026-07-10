use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PromptCardEnumParseError {
    enum_name: &'static str,
    value: String,
    allowed: &'static [&'static str],
}

impl PromptCardEnumParseError {
    fn new(enum_name: &'static str, value: &str, allowed: &'static [&'static str]) -> Self {
        Self {
            enum_name,
            value: value.to_string(),
            allowed,
        }
    }
}

impl fmt::Display for PromptCardEnumParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} must be one of: {}; actual: {}",
            self.enum_name,
            self.allowed.join(", "),
            if self.value.trim().is_empty() {
                "<empty>"
            } else {
                &self.value
            }
        )
    }
}

impl std::error::Error for PromptCardEnumParseError {}

macro_rules! finite_prompt_enum {
    (
        $(#[$meta:meta])*
        pub enum $name:ident {
            $($variant:ident => $value:literal),+ $(,)?
        }
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub enum $name {
            $($variant),+
        }

        impl $name {
            pub const VALUES: &'static [&'static str] = &[$($value),+];
            pub const VARIANTS: &'static [Self] = &[$(Self::$variant),+];

            pub fn as_str(self) -> &'static str {
                match self {
                    $(Self::$variant => $value),+
                }
            }

            pub fn parse(value: &str) -> Result<Self, PromptCardEnumParseError> {
                match value {
                    $($value => Ok(Self::$variant),)+
                    other => Err(PromptCardEnumParseError::new(stringify!($name), other, Self::VALUES)),
                }
            }

            pub fn allowed_values() -> &'static [&'static str] {
                Self::VALUES
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
    };
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PromptCardKind {
    Sip,
    Stp,
    Spp,
    Vpp,
    Srp,
    Sor,
}

impl PromptCardKind {
    pub const VALUES: &'static [&'static str] = &["sip", "stp", "spp", "vpp", "srp", "sor"];
    pub const VARIANTS: &'static [Self] = &[
        Self::Sip,
        Self::Stp,
        Self::Spp,
        Self::Vpp,
        Self::Srp,
        Self::Sor,
    ];

    pub fn all_for_template_set(template_set: &str) -> Vec<Self> {
        let mut kinds = vec![Self::Sip, Self::Stp, Self::Spp];
        if template_set_supports_vpp(template_set) {
            kinds.push(Self::Vpp);
        }
        kinds.push(Self::Srp);
        kinds.push(Self::Sor);
        kinds
    }

    pub fn key(self) -> &'static str {
        self.as_str()
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sip => "sip",
            Self::Stp => "stp",
            Self::Spp => "spp",
            Self::Vpp => "vpp",
            Self::Srp => "srp",
            Self::Sor => "sor",
        }
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::Sip => "Structured Issue Prompt",
            Self::Stp => "Structured Task Prompt",
            Self::Spp => "Structured Plan Prompt",
            Self::Vpp => "Structured Validation Planning Prompt",
            Self::Srp => "Structured Review Prompt",
            Self::Sor => "Structured Outcome Record",
        }
    }

    pub fn output_file(self) -> &'static str {
        match self {
            Self::Sip => "sip.md",
            Self::Stp => "stp.md",
            Self::Spp => "spp.md",
            Self::Vpp => "vpp.md",
            Self::Srp => "srp.md",
            Self::Sor => "sor.md",
        }
    }

    pub fn validate_type(self) -> &'static str {
        self.key()
    }

    pub fn parse_key(value: &str) -> anyhow::Result<Self> {
        Self::parse(value).map_err(|err| anyhow::anyhow!("{err}"))
    }

    pub fn parse(value: &str) -> Result<Self, PromptCardEnumParseError> {
        match value {
            "sip" => Ok(Self::Sip),
            "stp" => Ok(Self::Stp),
            "spp" => Ok(Self::Spp),
            "vpp" => Ok(Self::Vpp),
            "srp" => Ok(Self::Srp),
            "sor" => Ok(Self::Sor),
            other => Err(PromptCardEnumParseError::new(
                "PromptCardKind",
                other,
                Self::VALUES,
            )),
        }
    }

    pub fn allowed_values() -> &'static [&'static str] {
        Self::VALUES
    }
}

impl fmt::Display for PromptCardKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

pub(crate) fn template_set_supports_vpp(template_set: &str) -> bool {
    let mut parts = template_set.split('.');
    let Some(major) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    let Some(minor) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    let Some(patch) = parts.next().and_then(|value| value.parse::<u64>().ok()) else {
        return false;
    };
    (major, minor, patch) >= (1, 0, 3)
}

finite_prompt_enum! {
    pub enum CardStatus {
        Draft => "draft",
        Ready => "ready",
        Reviewed => "reviewed",
        Approved => "approved",
        Completed => "completed",
        Blocked => "blocked",
        Superseded => "superseded",
    }
}

finite_prompt_enum! {
    pub enum StpStatus {
        Draft => "draft",
        Active => "active",
        Complete => "complete",
    }
}

finite_prompt_enum! {
    pub enum StpAction {
        Create => "create",
        Edit => "edit",
        Close => "close",
        Split => "split",
        Supersede => "supersede",
    }
}

finite_prompt_enum! {
    pub enum IssueOutcomeType {
        Code => "code",
        Docs => "docs",
        Tests => "tests",
        Demo => "demo",
        Combination => "combination",
    }
}

finite_prompt_enum! {
    pub enum SppStatus {
        Draft => "draft",
        Ready => "ready",
        Reviewed => "reviewed",
        Approved => "approved",
    }
}

finite_prompt_enum! {
    pub enum ActivationState {
        Draft => "draft",
        Ready => "ready",
        Reviewed => "reviewed",
        Approved => "approved",
    }
}

impl ActivationState {
    pub const ALIASES: &'static [(&'static str, Self)] = &[
        ("design_time_ready", Self::Ready),
        ("ready_for_execution", Self::Ready),
        ("ready_for_execution_binding", Self::Approved),
        ("active", Self::Approved),
    ];

    pub fn parse_with_alias(value: &str) -> Result<Self, PromptCardEnumParseError> {
        Self::parse(value).or_else(|_| {
            Self::ALIASES
                .iter()
                .find_map(|(alias, variant)| (*alias == value).then_some(*variant))
                .ok_or_else(|| {
                    PromptCardEnumParseError::new("ActivationState", value, Self::VALUES)
                })
        })
    }
}

finite_prompt_enum! {
    pub enum CodexPlanStatus {
        Pending => "pending",
        InProgress => "in_progress",
        Completed => "completed",
    }
}

finite_prompt_enum! {
    pub enum EstimateConfidence {
        Low => "low",
        Medium => "medium",
        High => "high",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    pub enum EstimateDataSource {
        ManualEntry => "manual_entry",
        DerivedSprintState => "derived_sprint_state",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    pub enum VppStatus {
        Draft => "draft",
        Ready => "ready",
        Reviewed => "reviewed",
        Approved => "approved",
    }
}

finite_prompt_enum! {
    pub enum ValidationSizeSplit {
        SmallOnly => "small_only",
        LargeOnly => "large_only",
        Mixed => "mixed",
        NotApplicable => "not_applicable",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    pub enum SrpStatus {
        Draft => "draft",
        Ready => "ready",
        Approved => "approved",
    }
}

finite_prompt_enum! {
    pub enum ReviewFindingsStatus {
        NotRun => "not_run",
        FindingsPresent => "findings_present",
        NoFindings => "no_findings",
        ReviewUnavailable => "review_unavailable",
        ReviewTimeout => "review_timeout",
        ReviewCancelled => "review_cancelled",
        ReviewFailed => "review_failed",
    }
}

finite_prompt_enum! {
    pub enum ReviewRecommendedOutcome {
        NotRun => "not_run",
        Pass => "pass",
        Block => "block",
        NeedsFollowup => "needs_followup",
    }
}

finite_prompt_enum! {
    pub enum AllowedReviewDisposition {
        Pass => "PASS",
        Block => "BLOCK",
        NeedsFollowup => "NEEDS_FOLLOWUP",
    }
}

finite_prompt_enum! {
    pub enum SorExecutionStatus {
        NotStarted => "NOT_STARTED",
        InProgress => "IN_PROGRESS",
        Done => "DONE",
        Failed => "FAILED",
    }
}

finite_prompt_enum! {
    pub enum CompletionState {
        Completed => "completed",
        CompletedWithFollowOn => "completed_with_follow_on",
        Blocked => "blocked",
        Failed => "failed",
        Deferred => "deferred",
        Cancelled => "cancelled",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    pub enum VarianceRequired {
        NotApplicable => "not_applicable",
        No => "no",
        Yes => "yes",
    }
}

finite_prompt_enum! {
    pub enum VarianceCompleted {
        NotApplicable => "not_applicable",
        No => "no",
        Yes => "yes",
    }
}

finite_prompt_enum! {
    pub enum VarianceCategory {
        NotApplicable => "not_applicable",
        ValidationMisclassification => "validation_misclassification",
        PrWait => "pr_wait",
        MergeConflict => "merge_conflict",
        ToolFailure => "tool_failure",
        UnclearScope => "unclear_scope",
        ModelDrift => "model_drift",
        HumanWait => "human_wait",
        ExternalApiLatency => "external_api_latency",
        OverestimatedScope => "overestimated_scope",
    }
}

finite_prompt_enum! {
    pub enum BudgetSource {
        IssueGoalBudget => "issue_goal_budget",
        SprintRollup => "sprint_rollup",
        ManualEntry => "manual_entry",
        NotApplicable => "not_applicable",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    pub enum ActualMetricsDataSource {
        CodexGoalTool => "codex_goal_tool",
        ManualEntry => "manual_entry",
        DerivedSprintState => "derived_sprint_state",
        Unknown => "unknown",
    }
}

finite_prompt_enum! {
    /// Canonical rendered SOR integration states.
    ///
    /// The v0.91.7 inventory intentionally excludes the current editor-only
    /// `failed` and `blocked` spellings. Those are completion/result concepts
    /// unless follow-on work deliberately widens structured SOR semantics.
    pub enum IntegrationState {
        WorktreeOnly => "worktree_only",
        PrOpen => "pr_open",
        Merged => "merged",
        ClosedNoPr => "closed_no_pr",
    }
}

impl IntegrationState {
    pub const ALIASES: &'static [(&'static str, Self)] = &[
        ("worktree", Self::WorktreeOnly),
        ("open_pr", Self::PrOpen),
        ("open-pr", Self::PrOpen),
        ("pr-ready", Self::PrOpen),
        ("pr_ready", Self::PrOpen),
        ("closed-no-pr", Self::ClosedNoPr),
        ("no-pr", Self::ClosedNoPr),
        ("no_pr", Self::ClosedNoPr),
        ("merged-pr", Self::Merged),
        ("merged_pr", Self::Merged),
    ];

    pub fn parse_with_alias(value: &str) -> Result<Self, PromptCardEnumParseError> {
        Self::parse(value).or_else(|_| {
            Self::ALIASES
                .iter()
                .find_map(|(alias, variant)| (*alias == value).then_some(*variant))
                .ok_or_else(|| {
                    PromptCardEnumParseError::new("IntegrationState", value, Self::VALUES)
                })
        })
    }
}

finite_prompt_enum! {
    /// Canonical rendered SOR verification scopes.
    ///
    /// The v0.91.7 inventory intentionally excludes current editor-only `ci`
    /// and `not_run` spellings. CI is validation metadata, while `not_run` is a
    /// result/status concept unless follow-on work deliberately widens the
    /// structured SOR contract.
    pub enum VerificationScope {
        Worktree => "worktree",
        PrBranch => "pr_branch",
        MainRepo => "main_repo",
    }
}

impl VerificationScope {
    pub const ALIASES: &'static [(&'static str, Self)] = &[
        ("main", Self::MainRepo),
        ("main repo", Self::MainRepo),
        ("main-repo", Self::MainRepo),
        ("repo", Self::MainRepo),
        ("pr", Self::PrBranch),
        ("pr branch", Self::PrBranch),
        ("pr-branch", Self::PrBranch),
        ("branch", Self::PrBranch),
        ("worktree-only", Self::Worktree),
        ("worktree_only", Self::Worktree),
    ];

    pub fn parse_with_alias(value: &str) -> Result<Self, PromptCardEnumParseError> {
        Self::parse(value).or_else(|_| {
            Self::ALIASES
                .iter()
                .find_map(|(alias, variant)| (*alias == value).then_some(*variant))
                .ok_or_else(|| {
                    PromptCardEnumParseError::new("VerificationScope", value, Self::VALUES)
                })
        })
    }
}

finite_prompt_enum! {
    pub enum ValidationResult {
        Pass => "PASS",
        Fail => "FAIL",
        NotRun => "NOT_RUN",
        Blocked => "BLOCKED",
    }
}

finite_prompt_enum! {
    pub enum ProofStatus {
        Pass => "PASS",
        Fail => "FAIL",
        Partial => "PARTIAL",
        NotRun => "NOT_RUN",
        Blocked => "BLOCKED",
    }
}

finite_prompt_enum! {
    pub enum VerificationSchemaChangesApproved {
        Yes => "yes",
        No => "no",
        NotApplicable => "not_applicable",
    }
}

finite_prompt_enum! {
    pub enum DemoRequired {
        True => "true",
        False => "false",
    }
}

impl DemoRequired {
    pub fn as_bool(self) -> bool {
        matches!(self, Self::True)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn assert_round_trip<E>(
        values: &[&'static str],
        parse: impl Fn(&str) -> Result<E, PromptCardEnumParseError>,
        display: impl Fn(E) -> &'static str,
    ) where
        E: Copy + fmt::Debug + PartialEq,
    {
        for value in values {
            let parsed = parse(value).unwrap_or_else(|err| panic!("{value}: {err}"));
            assert_eq!(display(parsed), *value);
        }
    }

    #[test]
    fn prompt_card_kind_preserves_keys_labels_and_template_set_order() {
        assert_round_trip(
            PromptCardKind::VALUES,
            PromptCardKind::parse,
            PromptCardKind::as_str,
        );
        assert_eq!(
            PromptCardKind::all_for_template_set("1.0.2")
                .into_iter()
                .map(PromptCardKind::as_str)
                .collect::<Vec<_>>(),
            vec!["sip", "stp", "spp", "srp", "sor"]
        );
        assert_eq!(
            PromptCardKind::all_for_template_set("1.0.3")
                .into_iter()
                .map(PromptCardKind::as_str)
                .collect::<Vec<_>>(),
            vec!["sip", "stp", "spp", "vpp", "srp", "sor"]
        );
        assert_eq!(PromptCardKind::Srp.label(), "Structured Review Prompt");
        assert_eq!(PromptCardKind::Sor.output_file(), "sor.md");
    }

    #[test]
    fn enum_now_contracts_round_trip_canonical_values() {
        assert_round_trip(CardStatus::VALUES, CardStatus::parse, CardStatus::as_str);
        assert_round_trip(StpStatus::VALUES, StpStatus::parse, StpStatus::as_str);
        assert_round_trip(StpAction::VALUES, StpAction::parse, StpAction::as_str);
        assert_round_trip(
            IssueOutcomeType::VALUES,
            IssueOutcomeType::parse,
            IssueOutcomeType::as_str,
        );
        assert_round_trip(SppStatus::VALUES, SppStatus::parse, SppStatus::as_str);
        assert_round_trip(
            ActivationState::VALUES,
            ActivationState::parse,
            ActivationState::as_str,
        );
        assert_round_trip(
            CodexPlanStatus::VALUES,
            CodexPlanStatus::parse,
            CodexPlanStatus::as_str,
        );
        assert_round_trip(
            EstimateConfidence::VALUES,
            EstimateConfidence::parse,
            EstimateConfidence::as_str,
        );
        assert_round_trip(
            EstimateDataSource::VALUES,
            EstimateDataSource::parse,
            EstimateDataSource::as_str,
        );
        assert_round_trip(VppStatus::VALUES, VppStatus::parse, VppStatus::as_str);
        assert_round_trip(
            ValidationSizeSplit::VALUES,
            ValidationSizeSplit::parse,
            ValidationSizeSplit::as_str,
        );
        assert_round_trip(SrpStatus::VALUES, SrpStatus::parse, SrpStatus::as_str);
        assert_round_trip(
            ReviewFindingsStatus::VALUES,
            ReviewFindingsStatus::parse,
            ReviewFindingsStatus::as_str,
        );
        assert_round_trip(
            ReviewRecommendedOutcome::VALUES,
            ReviewRecommendedOutcome::parse,
            ReviewRecommendedOutcome::as_str,
        );
        assert_round_trip(
            AllowedReviewDisposition::VALUES,
            AllowedReviewDisposition::parse,
            AllowedReviewDisposition::as_str,
        );
        assert_round_trip(
            SorExecutionStatus::VALUES,
            SorExecutionStatus::parse,
            SorExecutionStatus::as_str,
        );
        assert_round_trip(
            CompletionState::VALUES,
            CompletionState::parse,
            CompletionState::as_str,
        );
        assert_round_trip(
            VarianceRequired::VALUES,
            VarianceRequired::parse,
            VarianceRequired::as_str,
        );
        assert_round_trip(
            VarianceCompleted::VALUES,
            VarianceCompleted::parse,
            VarianceCompleted::as_str,
        );
        assert_round_trip(
            VarianceCategory::VALUES,
            VarianceCategory::parse,
            VarianceCategory::as_str,
        );
        assert_round_trip(
            BudgetSource::VALUES,
            BudgetSource::parse,
            BudgetSource::as_str,
        );
        assert_round_trip(
            ActualMetricsDataSource::VALUES,
            ActualMetricsDataSource::parse,
            ActualMetricsDataSource::as_str,
        );
        assert_round_trip(
            IntegrationState::VALUES,
            IntegrationState::parse,
            IntegrationState::as_str,
        );
        assert_round_trip(
            VerificationScope::VALUES,
            VerificationScope::parse,
            VerificationScope::as_str,
        );
        assert_round_trip(
            ValidationResult::VALUES,
            ValidationResult::parse,
            ValidationResult::as_str,
        );
        assert_round_trip(ProofStatus::VALUES, ProofStatus::parse, ProofStatus::as_str);
        assert_round_trip(
            VerificationSchemaChangesApproved::VALUES,
            VerificationSchemaChangesApproved::parse,
            VerificationSchemaChangesApproved::as_str,
        );
        assert_round_trip(
            DemoRequired::VALUES,
            DemoRequired::parse,
            DemoRequired::as_str,
        );
    }

    #[test]
    fn legacy_aliases_normalize_without_widening_canonical_values() {
        assert_eq!(
            ActivationState::parse_with_alias("design_time_ready").unwrap(),
            ActivationState::Ready
        );
        assert_eq!(
            ActivationState::parse_with_alias("ready_for_execution").unwrap(),
            ActivationState::Ready
        );
        assert_eq!(
            ActivationState::parse_with_alias("ready_for_execution_binding").unwrap(),
            ActivationState::Approved
        );
        assert_eq!(
            ActivationState::parse_with_alias("active").unwrap(),
            ActivationState::Approved
        );

        assert_eq!(
            IntegrationState::parse_with_alias("worktree").unwrap(),
            IntegrationState::WorktreeOnly
        );
        assert_eq!(
            IntegrationState::parse_with_alias("pr-ready").unwrap(),
            IntegrationState::PrOpen
        );
        assert_eq!(
            IntegrationState::parse_with_alias("closed-no-pr").unwrap(),
            IntegrationState::ClosedNoPr
        );
        assert_eq!(
            IntegrationState::parse_with_alias("merged_pr").unwrap(),
            IntegrationState::Merged
        );

        assert_eq!(
            VerificationScope::parse_with_alias("main repo").unwrap(),
            VerificationScope::MainRepo
        );
        assert_eq!(
            VerificationScope::parse_with_alias("pr-branch").unwrap(),
            VerificationScope::PrBranch
        );
        assert_eq!(
            VerificationScope::parse_with_alias("worktree-only").unwrap(),
            VerificationScope::Worktree
        );
    }

    #[test]
    fn invalid_values_report_allowed_canonical_spellings() {
        let err = CardStatus::parse("almost_ready").unwrap_err();
        assert_eq!(
            err.to_string(),
            "CardStatus must be one of: draft, ready, reviewed, approved, completed, blocked, superseded; actual: almost_ready"
        );
        let err = IntegrationState::parse_with_alias("blocked").unwrap_err();
        assert!(err.to_string().contains(
            "IntegrationState must be one of: worktree_only, pr_open, merged, closed_no_pr"
        ));
        assert!(!IntegrationState::VALUES.contains(&"blocked"));
        assert!(!VerificationScope::VALUES.contains(&"not_run"));
    }

    #[test]
    fn selected_helpers_expose_values_for_follow_on_integration() {
        assert_eq!(
            CardStatus::allowed_values(),
            &[
                "draft",
                "ready",
                "reviewed",
                "approved",
                "completed",
                "blocked",
                "superseded"
            ]
        );
        assert_eq!(ProofStatus::Partial.as_str(), "PARTIAL");
        assert!(DemoRequired::True.as_bool());
        assert!(!DemoRequired::False.as_bool());
    }
}
