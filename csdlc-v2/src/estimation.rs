use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

use crate::cards::{EvidenceOutcome, ValidationResult};
use crate::error::{ErrorCode, Result, V2Error};
use crate::github::PrStatePacket;
use crate::model::IssueRecord;

pub const OBSERVATION_SCHEMA: &str = "csdlc.estimation_observation.v1";
pub const FORECAST_SCHEMA: &str = "csdlc.estimation_forecast.v1";
pub const OUTCOME_SCHEMA: &str = "csdlc.estimation_outcome.v1";

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ObservationSource {
    Lifecycle,
    Github,
    Validation,
    Session,
    OperatorAnnotation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Availability {
    Known,
    Unknown,
    Interrupted,
    SchemaDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, JsonSchema)]
pub struct Provenance {
    pub source: ObservationSource,
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MetricObservation {
    pub availability: Availability,
    pub value: Option<u64>,
    #[serde(default)]
    pub provenance: Vec<Provenance>,
}

impl MetricObservation {
    pub fn known(value: u64, source: ObservationSource, reference: impl Into<String>) -> Self {
        Self {
            availability: Availability::Known,
            value: Some(value),
            provenance: vec![Provenance {
                source,
                reference: reference.into(),
            }],
        }
    }

    pub fn unavailable(
        availability: Availability,
        source: ObservationSource,
        reference: impl Into<String>,
    ) -> Self {
        Self {
            availability,
            value: None,
            provenance: vec![Provenance {
                source,
                reference: reference.into(),
            }],
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ComparableKey {
    pub cohort: String,
    pub workflow_version: String,
    pub model_era: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Observation {
    pub schema: String,
    pub issue: u64,
    pub key: ComparableKey,
    pub elapsed_seconds: MetricObservation,
    pub active_work_seconds: MetricObservation,
    pub validation_seconds: MetricObservation,
    pub pr_wait_seconds: MetricObservation,
    pub ci_wait_seconds: MetricObservation,
    pub total_tokens: MetricObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AdapterContext {
    pub issue: u64,
    pub key: ComparableKey,
    pub reference: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct LifecycleTiming {
    pub bound_unix_seconds: u64,
    pub observed_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct GithubTiming {
    pub pr_opened_unix_seconds: u64,
    pub terminal_unix_seconds: u64,
    pub ci_started_unix_seconds: Option<u64>,
    pub ci_completed_unix_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationTiming {
    pub started_unix_seconds: u64,
    pub completed_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct SessionTiming {
    pub started_unix_seconds: u64,
    pub observed_unix_seconds: u64,
    pub active_work_seconds: Option<u64>,
    pub total_tokens: Option<u64>,
    pub interrupted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct OperatorTiming {
    pub wait_seconds: Option<u64>,
    pub interrupted: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ArtifactReference {
    pub reference: String,
    pub digest: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EstimateMethod {
    ComparableMedian,
    StaticPlanningProfileFallback,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum Confidence {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum DriftState {
    Stable,
    InsufficientData,
    SchemaDrift,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ForecastRange {
    pub point: u64,
    pub lower: u64,
    pub upper: u64,
    pub median_absolute_deviation: u64,
    pub cohort_size: usize,
    pub source_provenance: Vec<Provenance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct StaticEstimate {
    pub elapsed_seconds: u64,
    pub validation_seconds: u64,
    pub total_tokens: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct Forecast {
    pub schema: String,
    pub target_issue: u64,
    pub key: ComparableKey,
    pub method: EstimateMethod,
    pub elapsed_seconds: ForecastRange,
    pub validation_seconds: ForecastRange,
    pub total_tokens: ForecastRange,
    pub confidence: Confidence,
    pub drift: DriftState,
    pub outlier_factors: Vec<String>,
    pub comparable_issues: Vec<u64>,
    pub calibration: Option<ArtifactReference>,
    pub fallback_reason: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum EstimateDisposition {
    Accepted,
    Adjusted,
    Rejected,
    Deferred,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct AcceptedEstimate {
    pub forecast_ref: String,
    pub forecast_digest: String,
    pub disposition: EstimateDisposition,
    pub advisory_only: bool,
    pub adjusted: Option<StaticEstimate>,
    pub operator_rationale: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct MetricComparison {
    pub forecast: u64,
    pub actual: Option<u64>,
    pub absolute_error: Option<u64>,
    pub error_basis_points: Option<u64>,
    pub actual_provenance: Vec<Provenance>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct TerminalOutcome {
    pub schema: String,
    pub issue: u64,
    pub forecast_ref: String,
    pub forecast_digest: String,
    pub elapsed_seconds: MetricComparison,
    pub validation_seconds: MetricComparison,
    pub total_tokens: MetricComparison,
    pub unknown_actuals: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct BacktestCase {
    pub issue: u64,
    pub forecast: Forecast,
    pub outcome: TerminalOutcome,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalibrationReport {
    pub schema: String,
    pub case_count: usize,
    pub elapsed_median_error_basis_points: Option<u64>,
    pub validation_median_error_basis_points: Option<u64>,
    pub token_median_error_basis_points: Option<u64>,
    pub calibrated: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct VerifiedCalibration {
    pub artifact: ArtifactReference,
    pub report: CalibrationReport,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CycleTimeCohort {
    pub id: String,
    pub issue_count: usize,
    pub active_work_seconds: u64,
    pub validation_seconds: u64,
    pub review_seconds: u64,
    pub ci_seconds: u64,
    pub wait_seconds: u64,
    pub reconnect_actions: u64,
    pub measured: bool,
    #[serde(default)]
    pub unknown_components: Vec<String>,
    pub provenance: Vec<ArtifactReference>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum CycleTimeComparisonStatus {
    Comparable,
    NotComparable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CycleTimeComparison {
    pub schema: String,
    pub baseline_cohort: CycleTimeCohort,
    pub candidate_cohort: CycleTimeCohort,
    pub status: CycleTimeComparisonStatus,
    pub comparison_basis_equal: bool,
    pub gates_preserved: bool,
    pub baseline_total_seconds: u64,
    pub candidate_total_seconds: u64,
    pub elapsed_reduction_seconds: i64,
    pub reconnect_action_reduction: i64,
}

pub fn adapt_lifecycle(
    context: AdapterContext,
    record: &IssueRecord,
    timing: LifecycleTiming,
) -> Result<Observation> {
    if record.issue != context.issue {
        return invalid("lifecycle record does not match adapter issue");
    }
    let elapsed = checked_elapsed(timing.bound_unix_seconds, timing.observed_unix_seconds)?;
    Ok(fragment(
        context,
        ObservationSource::Lifecycle,
        Some(elapsed),
        None,
        None,
        None,
        None,
        None,
        false,
    )?)
}

pub fn adapt_github(
    context: AdapterContext,
    packet: &PrStatePacket,
    timing: GithubTiming,
) -> Result<Observation> {
    if packet.linked_issue != Some(context.issue)
        || packet.repository.trim().is_empty()
        || packet.pull_request == 0
    {
        return invalid("GitHub packet does not match adapter issue");
    }
    let pr_wait = checked_elapsed(timing.pr_opened_unix_seconds, timing.terminal_unix_seconds)?;
    let ci_wait = match (
        timing.ci_started_unix_seconds,
        timing.ci_completed_unix_seconds,
    ) {
        (Some(start), Some(end)) => Some(checked_elapsed(start, end)?),
        (None, None) => None,
        _ => return invalid("GitHub CI timing requires both start and completion timestamps"),
    };
    fragment(
        context,
        ObservationSource::Github,
        None,
        None,
        None,
        Some(pr_wait),
        ci_wait,
        None,
        false,
    )
}

pub fn adapt_validation(
    context: AdapterContext,
    result: &ValidationResult,
    timing: ValidationTiming,
) -> Result<Observation> {
    let validation = checked_elapsed(timing.started_unix_seconds, timing.completed_unix_seconds)?;
    fragment(
        context,
        ObservationSource::Validation,
        None,
        None,
        (result.outcome == EvidenceOutcome::Passed).then_some(validation),
        None,
        None,
        None,
        result.outcome != EvidenceOutcome::Passed,
    )
}

pub fn adapt_session(context: AdapterContext, timing: SessionTiming) -> Result<Observation> {
    let elapsed = checked_elapsed(timing.started_unix_seconds, timing.observed_unix_seconds)?;
    fragment(
        context,
        ObservationSource::Session,
        Some(elapsed),
        timing.active_work_seconds,
        None,
        None,
        None,
        timing.total_tokens,
        timing.interrupted,
    )
}

pub fn adapt_operator(context: AdapterContext, timing: OperatorTiming) -> Result<Observation> {
    fragment(
        context,
        ObservationSource::OperatorAnnotation,
        None,
        None,
        None,
        timing.wait_seconds,
        None,
        None,
        timing.interrupted,
    )
}

pub fn validate_observation(observation: &Observation) -> Result<()> {
    if observation.schema != OBSERVATION_SCHEMA
        || observation.issue == 0
        || observation.key.cohort.trim().is_empty()
        || observation.key.workflow_version.trim().is_empty()
        || observation.key.model_era.trim().is_empty()
    {
        return invalid("observation identity is incomplete");
    }
    for metric in metrics(observation) {
        if (metric.availability == Availability::Known) != metric.value.is_some() {
            return invalid("known metrics require values and unavailable metrics forbid values");
        }
        if metric.provenance.is_empty() {
            return invalid("every metric requires provenance, including unknown metrics");
        }
        for source in &metric.provenance {
            validate_reference(&source.reference)?;
        }
    }
    Ok(())
}

pub fn join_observations(mut fragments: Vec<Observation>) -> Result<Observation> {
    if fragments.is_empty() {
        return invalid("at least one observation fragment is required");
    }
    fragments.sort_by_key(|fragment| canonical_digest(fragment).unwrap_or_default());
    for fragment in &fragments {
        validate_observation(fragment)?;
    }
    let first = fragments.remove(0);
    let mut joined = first.clone();
    for fragment in fragments {
        if fragment.issue != joined.issue || fragment.key != joined.key {
            return invalid("observation fragments have conflicting identities");
        }
        merge_metric(&mut joined.elapsed_seconds, fragment.elapsed_seconds)?;
        merge_metric(
            &mut joined.active_work_seconds,
            fragment.active_work_seconds,
        )?;
        merge_metric(&mut joined.validation_seconds, fragment.validation_seconds)?;
        merge_metric(&mut joined.pr_wait_seconds, fragment.pr_wait_seconds)?;
        merge_metric(&mut joined.ci_wait_seconds, fragment.ci_wait_seconds)?;
        merge_metric(&mut joined.total_tokens, fragment.total_tokens)?;
    }
    validate_observation(&joined)?;
    Ok(joined)
}

pub fn forecast(
    target_issue: u64,
    key: ComparableKey,
    observations: &[Observation],
    fallback: StaticEstimate,
    calibration: Option<&VerifiedCalibration>,
) -> Result<Forecast> {
    if target_issue == 0
        || fallback.elapsed_seconds == 0
        || fallback.validation_seconds == 0
        || fallback.total_tokens == 0
    {
        return invalid("target and static fallback must be non-zero");
    }
    let unique = consolidate_observations(observations)?;
    let mut comparable: Vec<&Observation> = unique
        .iter()
        .filter(|item| item.issue != target_issue && item.key == key)
        .collect();
    comparable.sort_by_key(|item| item.issue);
    let issues: Vec<u64> = comparable.iter().map(|item| item.issue).collect();
    let schema_drift = comparable.iter().any(|item| {
        metrics(item)
            .iter()
            .any(|metric| metric.availability == Availability::SchemaDrift)
    });
    let elapsed = known_values(&comparable, |item| &item.elapsed_seconds);
    let validation = known_values(&comparable, |item| &item.validation_seconds);
    let tokens = known_values(&comparable, |item| &item.total_tokens);
    let sufficient =
        comparable.len() >= 3 && elapsed.len() >= 3 && validation.len() >= 3 && tokens.len() >= 3;
    let calibration_valid = calibration.is_some_and(|value| {
        value.report.calibrated
            && validate_artifact_reference(&value.artifact).is_ok()
            && canonical_digest(&value.report).ok().as_deref()
                == Some(value.artifact.digest.as_str())
    });
    if !sufficient || schema_drift || !calibration_valid {
        let reason = if schema_drift {
            "comparable cohort contains schema drift"
        } else if !calibration_valid {
            "calibration is missing, invalid, or failed"
        } else {
            "fewer than three unique comparable known issues"
        };
        return Ok(Forecast {
            schema: FORECAST_SCHEMA.into(),
            target_issue,
            key,
            method: EstimateMethod::StaticPlanningProfileFallback,
            elapsed_seconds: fallback_range(fallback.elapsed_seconds),
            validation_seconds: fallback_range(fallback.validation_seconds),
            total_tokens: fallback_range(fallback.total_tokens),
            confidence: Confidence::Low,
            drift: if schema_drift {
                DriftState::SchemaDrift
            } else {
                DriftState::InsufficientData
            },
            outlier_factors: Vec::new(),
            comparable_issues: issues,
            calibration: calibration.map(|value| value.artifact.clone()),
            fallback_reason: Some(reason.into()),
        });
    }
    let elapsed_range = robust_range(
        &elapsed,
        provenance_for(&comparable, |item| &item.elapsed_seconds),
    );
    let validation_range = robust_range(
        &validation,
        provenance_for(&comparable, |item| &item.validation_seconds),
    );
    let token_range = robust_range(
        &tokens,
        provenance_for(&comparable, |item| &item.total_tokens),
    );
    let mut outliers = Vec::new();
    if is_wide(&elapsed_range) {
        outliers.push("elapsed_dispersion".into());
    }
    if is_wide(&validation_range) {
        outliers.push("validation_dispersion".into());
    }
    if is_wide(&token_range) {
        outliers.push("token_dispersion".into());
    }
    let confidence = if outliers.is_empty() && comparable.len() >= 5 {
        Confidence::High
    } else if outliers.len() <= 1 {
        Confidence::Medium
    } else {
        Confidence::Low
    };
    Ok(Forecast {
        schema: FORECAST_SCHEMA.into(),
        target_issue,
        key,
        method: EstimateMethod::ComparableMedian,
        elapsed_seconds: elapsed_range,
        validation_seconds: validation_range,
        total_tokens: token_range,
        confidence,
        drift: DriftState::Stable,
        outlier_factors: outliers,
        comparable_issues: issues,
        calibration: calibration.map(|value| value.artifact.clone()),
        fallback_reason: None,
    })
}

pub fn validate_accepted_estimate(accepted: &AcceptedEstimate) -> Result<()> {
    validate_artifact_reference(&ArtifactReference {
        reference: accepted.forecast_ref.clone(),
        digest: accepted.forecast_digest.clone(),
    })?;
    if !accepted.advisory_only || accepted.operator_rationale.trim().is_empty() {
        return invalid("accepted estimate must be advisory, digested, and dispositioned");
    }
    if (accepted.disposition == EstimateDisposition::Adjusted) != accepted.adjusted.is_some() {
        return invalid("only adjusted estimates may contain operator-adjusted values");
    }
    Ok(())
}

pub fn terminal_outcome(
    forecast: &Forecast,
    forecast_ref: impl Into<String>,
    actual: &Observation,
) -> Result<TerminalOutcome> {
    validate_observation(actual)?;
    if forecast.schema != FORECAST_SCHEMA || forecast.target_issue != actual.issue {
        return invalid("terminal actual does not match forecast target");
    }
    let forecast_ref = forecast_ref.into();
    validate_reference(&forecast_ref)?;
    let mut unknown_actuals = Vec::new();
    let elapsed = compare(
        "elapsed_seconds",
        forecast.elapsed_seconds.point,
        &actual.elapsed_seconds,
        &mut unknown_actuals,
    );
    let validation = compare(
        "validation_seconds",
        forecast.validation_seconds.point,
        &actual.validation_seconds,
        &mut unknown_actuals,
    );
    let tokens = compare(
        "total_tokens",
        forecast.total_tokens.point,
        &actual.total_tokens,
        &mut unknown_actuals,
    );
    Ok(TerminalOutcome {
        schema: OUTCOME_SCHEMA.into(),
        issue: actual.issue,
        forecast_ref,
        forecast_digest: canonical_digest(forecast)?,
        elapsed_seconds: elapsed,
        validation_seconds: validation,
        total_tokens: tokens,
        unknown_actuals,
    })
}

pub fn calibration_report(
    cases: &[BacktestCase],
    tolerance_basis_points: u64,
) -> Result<CalibrationReport> {
    let mut unique_issues = BTreeSet::new();
    let mut elapsed = Vec::new();
    let mut validation = Vec::new();
    let mut tokens = Vec::new();
    for case in cases {
        if !unique_issues.insert(case.issue) {
            return invalid("backtest calibration contains a duplicate issue ID");
        }
        if case.issue != case.forecast.target_issue || case.issue != case.outcome.issue {
            return invalid("backtest case identity is inconsistent");
        }
        collect_error(&mut elapsed, &case.outcome.elapsed_seconds);
        collect_error(&mut validation, &case.outcome.validation_seconds);
        collect_error(&mut tokens, &case.outcome.total_tokens);
    }
    let elapsed_median = optional_median(&mut elapsed);
    let validation_median = optional_median(&mut validation);
    let token_median = optional_median(&mut tokens);
    let calibrated = cases.len() >= 3
        && [elapsed_median, validation_median, token_median]
            .iter()
            .all(|value| value.is_some_and(|value| value <= tolerance_basis_points));
    Ok(CalibrationReport {
        schema: "csdlc.estimation_calibration.v1".into(),
        case_count: cases.len(),
        elapsed_median_error_basis_points: elapsed_median,
        validation_median_error_basis_points: validation_median,
        token_median_error_basis_points: token_median,
        calibrated,
    })
}

pub fn compare_cycle_time(
    baseline: CycleTimeCohort,
    candidate: CycleTimeCohort,
    comparison_basis_equal: bool,
    gates_preserved: bool,
) -> Result<CycleTimeComparison> {
    if baseline.issue_count == 0
        || candidate.issue_count == 0
        || baseline.provenance.is_empty()
        || candidate.provenance.is_empty()
        || !baseline.measured
        || !candidate.measured
    {
        return invalid("cycle-time cohorts require measured provenance-backed inputs");
    }
    for artifact in baseline.provenance.iter().chain(&candidate.provenance) {
        validate_artifact_reference(artifact)?;
    }
    let baseline_total = cohort_total(&baseline);
    let candidate_total = cohort_total(&candidate);
    let comparable = comparison_basis_equal
        && gates_preserved
        && baseline.issue_count == candidate.issue_count
        && baseline.unknown_components.is_empty()
        && candidate.unknown_components.is_empty();
    Ok(CycleTimeComparison {
        schema: "csdlc.cycle_time_comparison.v1".into(),
        elapsed_reduction_seconds: if comparable {
            signed_difference(baseline_total, candidate_total)
        } else {
            0
        },
        reconnect_action_reduction: if comparable {
            signed_difference(baseline.reconnect_actions, candidate.reconnect_actions)
        } else {
            0
        },
        baseline_cohort: baseline,
        candidate_cohort: candidate,
        status: if comparable {
            CycleTimeComparisonStatus::Comparable
        } else {
            CycleTimeComparisonStatus::NotComparable
        },
        comparison_basis_equal,
        gates_preserved,
        baseline_total_seconds: baseline_total,
        candidate_total_seconds: candidate_total,
    })
}

pub fn load_verified_json<T: DeserializeOwned + Serialize>(
    root: &Path,
    artifact: &ArtifactReference,
) -> Result<T> {
    validate_artifact_reference(artifact)?;
    let path = root.join(&artifact.reference);
    let canonical_root = fs::canonicalize(root)?;
    let canonical_path = fs::canonicalize(&path)?;
    if !canonical_path.starts_with(&canonical_root) {
        return invalid("artifact reference escapes the repository root");
    }
    let value: T = serde_json::from_slice(&fs::read(canonical_path)?)?;
    let observed = canonical_digest(&value)?;
    if observed != artifact.digest {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            format!("artifact digest mismatch for {}", artifact.reference),
        ));
    }
    Ok(value)
}

pub fn verified_calibration(
    root: &Path,
    artifact: ArtifactReference,
) -> Result<VerifiedCalibration> {
    let report = load_verified_json(root, &artifact)?;
    Ok(VerifiedCalibration { artifact, report })
}

pub fn validate_artifact_reference(artifact: &ArtifactReference) -> Result<()> {
    validate_reference(&artifact.reference)?;
    if artifact.digest.len() != 64 || !artifact.digest.bytes().all(|byte| byte.is_ascii_hexdigit())
    {
        return invalid("artifact digest must be a 64-character hexadecimal BLAKE3 digest");
    }
    Ok(())
}

pub fn canonical_digest<T: Serialize>(value: &T) -> Result<String> {
    let bytes = serde_json::to_vec(value)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?;
    Ok(blake3::hash(&bytes).to_hex().to_string())
}

pub fn estimation_schema_bundle() -> serde_json::Value {
    serde_json::json!({
        "schema": "csdlc.estimation_schema_bundle.v1",
        "observation": schemars::schema_for!(Observation),
        "forecast": schemars::schema_for!(Forecast),
        "accepted_estimate": schemars::schema_for!(AcceptedEstimate),
        "terminal_outcome": schemars::schema_for!(TerminalOutcome),
        "calibration_report": schemars::schema_for!(CalibrationReport),
        "cycle_time_comparison": schemars::schema_for!(CycleTimeComparison),
    })
}

fn metrics(observation: &Observation) -> [&MetricObservation; 6] {
    [
        &observation.elapsed_seconds,
        &observation.active_work_seconds,
        &observation.validation_seconds,
        &observation.pr_wait_seconds,
        &observation.ci_wait_seconds,
        &observation.total_tokens,
    ]
}

pub fn validate_reference(reference: &str) -> Result<()> {
    let lower = reference.to_ascii_lowercase();
    let normalized = reference.replace('\\', "/");
    let windows_absolute = normalized
        .as_bytes()
        .get(1)
        .is_some_and(|byte| *byte == b':')
        && normalized
            .as_bytes()
            .first()
            .is_some_and(u8::is_ascii_alphabetic);
    let parent_traversal = Path::new(&normalized)
        .components()
        .any(|component| component == Component::ParentDir);
    if reference.trim().is_empty()
        || reference.starts_with('/')
        || reference.starts_with("\\\\")
        || windows_absolute
        || parent_traversal
        || lower.contains("%2e%2e")
        || reference.contains('\n')
        || reference.contains('\r')
        || lower.contains("transcript")
        || lower.contains("/users/")
        || lower.contains("private/tmp")
        || lower.contains("secret")
    {
        return invalid("provenance references must be bounded, relative, and non-sensitive");
    }
    Ok(())
}

fn fragment(
    context: AdapterContext,
    source: ObservationSource,
    elapsed_seconds: Option<u64>,
    active_work_seconds: Option<u64>,
    validation_seconds: Option<u64>,
    pr_wait_seconds: Option<u64>,
    ci_wait_seconds: Option<u64>,
    total_tokens: Option<u64>,
    interrupted: bool,
) -> Result<Observation> {
    if context.issue == 0 {
        return invalid("adapter issue must be non-zero");
    }
    validate_reference(&context.reference)?;
    let metric = |value| match value {
        Some(value) => MetricObservation::known(value, source, context.reference.clone()),
        None => MetricObservation::unavailable(
            if interrupted {
                Availability::Interrupted
            } else {
                Availability::Unknown
            },
            source,
            context.reference.clone(),
        ),
    };
    let observation = Observation {
        schema: OBSERVATION_SCHEMA.into(),
        issue: context.issue,
        key: context.key,
        elapsed_seconds: metric(elapsed_seconds),
        active_work_seconds: metric(active_work_seconds),
        validation_seconds: metric(validation_seconds),
        pr_wait_seconds: metric(pr_wait_seconds),
        ci_wait_seconds: metric(ci_wait_seconds),
        total_tokens: metric(total_tokens),
    };
    validate_observation(&observation)?;
    Ok(observation)
}

fn checked_elapsed(start: u64, end: u64) -> Result<u64> {
    end.checked_sub(start)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "timing ends before it starts"))
}

fn consolidate_observations(observations: &[Observation]) -> Result<Vec<Observation>> {
    let mut grouped: BTreeMap<u64, Vec<Observation>> = BTreeMap::new();
    for observation in observations {
        validate_observation(observation)?;
        grouped
            .entry(observation.issue)
            .or_default()
            .push(observation.clone());
    }
    grouped
        .into_values()
        .map(join_observations)
        .collect::<Result<Vec<_>>>()
}

fn merge_metric(target: &mut MetricObservation, incoming: MetricObservation) -> Result<()> {
    if let (Some(left), Some(right)) = (target.value, incoming.value) {
        if left != right {
            return invalid("conflicting known metric values cannot be joined");
        }
    }
    if target.value.is_none() && incoming.value.is_some() {
        target.value = incoming.value;
        target.availability = Availability::Known;
    } else if target.value.is_none() {
        target.availability = unavailable_precedence(target.availability, incoming.availability);
    }
    target.provenance.extend(incoming.provenance);
    target.provenance.sort();
    target.provenance.dedup();
    Ok(())
}

fn unavailable_precedence(left: Availability, right: Availability) -> Availability {
    let rank = |value| match value {
        Availability::Known => 0,
        Availability::Unknown => 1,
        Availability::Interrupted => 2,
        Availability::SchemaDrift => 3,
    };
    if rank(left) >= rank(right) {
        left
    } else {
        right
    }
}

fn known_values<F>(items: &[&Observation], field: F) -> Vec<u64>
where
    F: Fn(&Observation) -> &MetricObservation,
{
    items.iter().filter_map(|item| field(item).value).collect()
}

fn provenance_for<F>(items: &[&Observation], field: F) -> Vec<Provenance>
where
    F: Fn(&Observation) -> &MetricObservation,
{
    let mut provenance: Vec<Provenance> = items
        .iter()
        .filter_map(|item| {
            let metric = field(item);
            metric.value.map(|_| metric.provenance.clone())
        })
        .flatten()
        .collect();
    provenance.sort();
    provenance.dedup();
    provenance
}

fn robust_range(values: &[u64], source_provenance: Vec<Provenance>) -> ForecastRange {
    let mut sorted = values.to_vec();
    sorted.sort_unstable();
    let point = median(&sorted);
    let mut deviations: Vec<u64> = sorted.iter().map(|value| value.abs_diff(point)).collect();
    deviations.sort_unstable();
    let mad = median(&deviations);
    ForecastRange {
        point,
        lower: point.saturating_sub(mad.saturating_mul(2)),
        upper: point.saturating_add(mad.saturating_mul(2)),
        median_absolute_deviation: mad,
        cohort_size: sorted.len(),
        source_provenance,
    }
}

fn fallback_range(point: u64) -> ForecastRange {
    ForecastRange {
        point,
        lower: point,
        upper: point,
        median_absolute_deviation: 0,
        cohort_size: 0,
        source_provenance: vec![Provenance {
            source: ObservationSource::Lifecycle,
            reference: "csdlc-v2/src/cards.rs#PlanningProfile".into(),
        }],
    }
}

fn median(values: &[u64]) -> u64 {
    let middle = values.len() / 2;
    if values.len() % 2 == 0 {
        values[middle - 1].saturating_add(values[middle]) / 2
    } else {
        values[middle]
    }
}

fn is_wide(range: &ForecastRange) -> bool {
    range.point > 0 && range.median_absolute_deviation.saturating_mul(2) > range.point
}

fn compare(
    name: &str,
    expected: u64,
    actual: &MetricObservation,
    unknowns: &mut Vec<String>,
) -> MetricComparison {
    match actual.value {
        Some(value) => {
            let absolute = expected.abs_diff(value);
            MetricComparison {
                forecast: expected,
                actual: Some(value),
                absolute_error: Some(absolute),
                error_basis_points: (value > 0).then_some(absolute.saturating_mul(10_000) / value),
                actual_provenance: actual.provenance.clone(),
            }
        }
        None => {
            unknowns.push(name.into());
            MetricComparison {
                forecast: expected,
                actual: None,
                absolute_error: None,
                error_basis_points: None,
                actual_provenance: actual.provenance.clone(),
            }
        }
    }
}

fn collect_error(target: &mut Vec<u64>, comparison: &MetricComparison) {
    if let Some(value) = comparison.error_basis_points {
        target.push(value);
    }
}

fn optional_median(values: &mut [u64]) -> Option<u64> {
    if values.is_empty() {
        None
    } else {
        values.sort_unstable();
        Some(median(values))
    }
}

fn cohort_total(cohort: &CycleTimeCohort) -> u64 {
    cohort
        .active_work_seconds
        .saturating_add(cohort.validation_seconds)
        .saturating_add(cohort.review_seconds)
        .saturating_add(cohort.ci_seconds)
        .saturating_add(cohort.wait_seconds)
}

fn signed_difference(left: u64, right: u64) -> i64 {
    let difference = i128::from(left) - i128::from(right);
    difference.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(V2Error::new(ErrorCode::InvalidInput, message))
}
