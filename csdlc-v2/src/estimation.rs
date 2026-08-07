use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path};

use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use time::format_description::well_known::Rfc3339;
use time::OffsetDateTime;

use crate::error::{ErrorCode, Result, V2Error};
use crate::model::IssueRecord;
use crate::publication::{
    body_has_github_closing_keyword, body_has_qualified_github_closing_keyword,
};
use crate::pvf::ExecutionReport;
use crate::store::record_digest;

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
    pub operator_wait_seconds: MetricObservation,
    pub reconnect_actions: MetricObservation,
    pub total_tokens: MetricObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ObservationManifest {
    pub schema: String,
    pub issue: u64,
    pub lifecycle: Option<ArtifactReference>,
    pub github: Option<ArtifactReference>,
    pub validation: Option<ArtifactReference>,
    pub session: Option<ArtifactReference>,
    pub operator: Option<ArtifactReference>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ValidationSourceManifest {
    pub schema: String,
    pub issue_record: ArtifactReference,
    pub execution_report: ArtifactReference,
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
    pub source_manifest: ArtifactReference,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedCalibration {
    artifact: ArtifactReference,
    report: CalibrationReport,
}

impl VerifiedCalibration {
    pub fn artifact(&self) -> &ArtifactReference {
        &self.artifact
    }

    pub fn report(&self) -> &CalibrationReport {
        &self.report
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalibrationManifest {
    pub schema: String,
    pub cases: Vec<CalibrationCaseArtifacts>,
    #[serde(default)]
    pub context_sources: Vec<ArtifactReference>,
    pub tolerance_basis_points: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CalibrationCaseArtifacts {
    pub issue: u64,
    pub forecast: ArtifactReference,
    pub actual_observation: ArtifactReference,
    pub outcome: ArtifactReference,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CycleTimeCohort {
    pub id: String,
    pub issue_count: usize,
    pub elapsed_seconds: u64,
    pub active_work_seconds: u64,
    pub validation_seconds: u64,
    pub pr_lifecycle_seconds: u64,
    pub ci_seconds: u64,
    pub wait_seconds: u64,
    pub reconnect_actions: u64,
    pub measured: bool,
    #[serde(default)]
    pub unknown_components: Vec<String>,
    pub provenance: Vec<ArtifactReference>,
    pub comparison_key: ComparableKey,
    pub preserved_gates: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CycleTimeEvidence {
    pub schema: String,
    pub observations: Vec<ArtifactReference>,
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

#[derive(Debug, Deserialize)]
struct GithubSource {
    number: u64,
    body: String,
    created_at: String,
    closed_at: Option<String>,
    merged_at: Option<String>,
    ci_started_at: Option<String>,
    ci_completed_at: Option<String>,
    base: GithubBase,
}

#[derive(Debug, Deserialize)]
struct GithubBase {
    repo: GithubRepository,
}

#[derive(Debug, Deserialize)]
struct GithubRepository {
    full_name: String,
}

#[derive(Debug, Deserialize)]
struct SessionSource {
    schema_version: String,
    issue_number: u64,
    data_source: String,
    model_ref: String,
    started_at: String,
    completed_at: String,
    elapsed_seconds: Option<u64>,
    active_work_seconds: Option<u64>,
    elapsed_availability: String,
    active_work_availability: String,
    token_usage: SessionTokens,
}

#[derive(Debug, Deserialize)]
struct SessionTokens {
    availability: String,
    total_tokens: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct OperatorSource {
    schema: String,
    issue: u64,
    source: String,
    wait_intervals: Vec<OperatorInterval>,
    reconnect_actions: u64,
    interrupted: bool,
}

#[derive(Debug, Deserialize)]
struct OperatorInterval {
    started_unix_seconds: u64,
    completed_unix_seconds: u64,
}

pub fn load_observation_manifest(root: &Path, artifact: &ArtifactReference) -> Result<Observation> {
    let manifest: ObservationManifest = load_verified_json(root, artifact)?;
    if manifest.schema != "csdlc.estimation_observation_manifest.v1" || manifest.issue == 0 {
        return invalid("observation manifest identity is invalid");
    }
    let key = derive_observation_key(root, &manifest)?;
    let mut fragments = Vec::new();
    if let Some(source) = &manifest.lifecycle {
        fragments.push(adapt_lifecycle(root, source, &manifest, &key)?);
    }
    if let Some(source) = &manifest.github {
        fragments.push(adapt_github(root, source, &manifest, &key)?);
    }
    if let Some(source) = &manifest.validation {
        fragments.push(adapt_validation(root, source, &manifest, &key)?);
    }
    if let Some(source) = &manifest.session {
        fragments.push(adapt_session(root, source, &manifest, &key)?);
    }
    if let Some(source) = &manifest.operator {
        fragments.push(adapt_operator(root, source, &manifest, &key)?);
    }
    join_observations(fragments)
}

fn derive_observation_key(root: &Path, manifest: &ObservationManifest) -> Result<ComparableKey> {
    let lifecycle_ref = manifest.lifecycle.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "observation identity requires lifecycle evidence",
        )
    })?;
    let session_ref = manifest.session.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "observation identity requires session evidence",
        )
    })?;
    let lifecycle: IssueRecord = load_verified_json(root, lifecycle_ref)?;
    verify_issue_record(&lifecycle)?;
    let session: SessionSource = load_verified_json(root, session_ref)?;
    if lifecycle.issue != manifest.issue
        || session.issue_number != manifest.issue
        || lifecycle.repository.trim().is_empty()
        || lifecycle.schema.trim().is_empty()
        || session.model_ref.trim().is_empty()
    {
        return invalid(
            "retained lifecycle and session sources do not prove one observation identity",
        );
    }
    Ok(ComparableKey {
        cohort: lifecycle.repository,
        workflow_version: lifecycle.schema,
        model_era: session.model_ref,
    })
}

fn adapt_lifecycle(
    root: &Path,
    artifact: &ArtifactReference,
    manifest: &ObservationManifest,
    key: &ComparableKey,
) -> Result<Observation> {
    let source: IssueRecord = load_verified_json(root, artifact)?;
    verify_issue_record(&source)?;
    if source.issue != manifest.issue || source.repository.trim().is_empty() {
        return invalid("lifecycle source does not match observation manifest");
    }
    fragment(
        manifest.issue,
        key.clone(),
        artifact.reference.clone(),
        ObservationSource::Lifecycle,
        FragmentMetrics::default(),
    )
}

fn adapt_github(
    root: &Path,
    artifact: &ArtifactReference,
    manifest: &ObservationManifest,
    key: &ComparableKey,
) -> Result<Observation> {
    let source: GithubSource = load_verified_json(root, artifact)?;
    let lifecycle_ref = manifest.lifecycle.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "GitHub evidence requires lifecycle publication authority",
        )
    })?;
    let lifecycle: IssueRecord = load_verified_json(root, lifecycle_ref)?;
    verify_issue_record(&lifecycle)?;
    let publication_repository = lifecycle
        .publication
        .as_ref()
        .map(|publication| publication.repository.as_str())
        .filter(|repository| repository.split_once('/').is_some())
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::InvalidInput,
                "GitHub evidence requires a verified publication repository",
            )
        })?;
    let closing_linkage_ok = if publication_repository != lifecycle.repository {
        body_has_qualified_github_closing_keyword(
            &source.body,
            manifest.issue,
            &lifecycle.repository,
        )
    } else {
        body_has_github_closing_keyword(&source.body, manifest.issue, &lifecycle.repository)
    };
    if lifecycle.issue != manifest.issue
        || source.base.repo.full_name != publication_repository
        || !closing_linkage_ok
        || source.number == 0
    {
        return invalid("GitHub packet does not match adapter issue");
    }
    let opened = parse_rfc3339(&source.created_at)?;
    let terminal = source
        .merged_at
        .as_deref()
        .or(source.closed_at.as_deref())
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "GitHub source is not terminal"))?;
    let pr_wait = checked_elapsed(opened, parse_rfc3339(terminal)?)?;
    let ci_wait = match (&source.ci_started_at, &source.ci_completed_at) {
        (Some(start), Some(end)) => {
            Some(checked_elapsed(parse_rfc3339(start)?, parse_rfc3339(end)?)?)
        }
        (None, None) => None,
        _ => return invalid("GitHub source has incomplete CI timing"),
    };
    fragment(
        manifest.issue,
        key.clone(),
        artifact.reference.clone(),
        ObservationSource::Github,
        FragmentMetrics {
            pr_wait_seconds: Some(pr_wait),
            ci_wait_seconds: ci_wait,
            ..FragmentMetrics::default()
        },
    )
}

fn adapt_validation(
    root: &Path,
    artifact: &ArtifactReference,
    manifest: &ObservationManifest,
    key: &ComparableKey,
) -> Result<Observation> {
    let source: ValidationSourceManifest = load_verified_json(root, artifact)?;
    if source.schema != "csdlc.validation_source_manifest.v1" {
        return invalid("validation source manifest schema is invalid");
    }
    let identity: IssueRecord = load_verified_json(root, &source.issue_record)?;
    verify_issue_record(&identity)?;
    if identity.issue != manifest.issue {
        return invalid("validation source identity does not match observation manifest");
    }
    let report: ExecutionReport = load_verified_json(root, &source.execution_report)?;
    if report.schema.trim().is_empty() || report.evidence.is_empty() {
        return invalid("validation source has no retained lane evidence");
    }
    let validation = report.evidence.iter().try_fold(0_u64, |total, lane| {
        let seconds = u64::try_from(lane.duration_ms.div_ceil(1000))
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "validation duration overflow"))?;
        Ok::<u64, V2Error>(total.saturating_add(seconds))
    })?;
    let passed = report.evidence.iter().all(|lane| {
        matches!(
            lane.status,
            crate::pvf::LaneStatus::Passed | crate::pvf::LaneStatus::AcceptedNonGoal
        )
    });
    fragment(
        manifest.issue,
        key.clone(),
        source.execution_report.reference.clone(),
        ObservationSource::Validation,
        FragmentMetrics {
            validation_seconds: passed.then_some(validation),
            interrupted: !passed,
            ..FragmentMetrics::default()
        },
    )
}

fn adapt_session(
    root: &Path,
    artifact: &ArtifactReference,
    manifest: &ObservationManifest,
    key: &ComparableKey,
) -> Result<Observation> {
    let source: SessionSource = load_verified_json(root, artifact)?;
    if source.issue_number != manifest.issue
        || source.data_source != "codex_goal_tool"
        || source.model_ref.trim().is_empty()
    {
        return invalid("session source is not authoritative for the observation manifest");
    }
    let schema_drift = source.schema_version != "issue_goal_metrics.v1";
    let elapsed_availability = availability(&source.elapsed_availability, schema_drift);
    let active_work_availability = availability(&source.active_work_availability, schema_drift);
    let token_availability = availability(&source.token_usage.availability, schema_drift);
    let elapsed = if elapsed_availability == Availability::Known {
        Some(checked_elapsed(
            parse_rfc3339(&source.started_at)?,
            parse_rfc3339(&source.completed_at)?,
        )?)
    } else {
        None
    };
    if elapsed_availability == Availability::Known && source.elapsed_seconds != elapsed {
        return invalid("session source elapsed value disagrees with retained timestamps");
    }
    let mut observation = fragment(
        manifest.issue,
        key.clone(),
        artifact.reference.clone(),
        ObservationSource::Session,
        FragmentMetrics {
            elapsed_seconds: elapsed,
            active_work_seconds: (active_work_availability == Availability::Known)
                .then_some(source.active_work_seconds)
                .flatten(),
            total_tokens: (token_availability == Availability::Known)
                .then_some(source.token_usage.total_tokens)
                .flatten(),
            ..FragmentMetrics::default()
        },
    )?;
    observation.elapsed_seconds.availability = elapsed_availability;
    observation.active_work_seconds.availability = active_work_availability;
    observation.total_tokens.availability = token_availability;
    validate_observation(&observation)?;
    Ok(observation)
}

fn availability(value: &str, schema_drift: bool) -> Availability {
    if schema_drift {
        return Availability::SchemaDrift;
    }
    match value {
        "known" => Availability::Known,
        "unknown" => Availability::Unknown,
        "interrupted" => Availability::Interrupted,
        "schema_drift" => Availability::SchemaDrift,
        _ => Availability::SchemaDrift,
    }
}

fn adapt_operator(
    root: &Path,
    artifact: &ArtifactReference,
    manifest: &ObservationManifest,
    key: &ComparableKey,
) -> Result<Observation> {
    let source: OperatorSource = load_verified_json(root, artifact)?;
    if source.schema != "csdlc.operator_timing_source.v1"
        || source.issue != manifest.issue
        || source.source != "operator_annotation"
    {
        return invalid("operator source is not authoritative for the observation manifest");
    }
    let wait = source
        .wait_intervals
        .iter()
        .try_fold(0_u64, |total, interval| {
            Ok::<u64, V2Error>(total.saturating_add(checked_elapsed(
                interval.started_unix_seconds,
                interval.completed_unix_seconds,
            )?))
        })?;
    fragment(
        manifest.issue,
        key.clone(),
        artifact.reference.clone(),
        ObservationSource::OperatorAnnotation,
        FragmentMetrics {
            operator_wait_seconds: Some(wait),
            reconnect_actions: Some(source.reconnect_actions),
            interrupted: source.interrupted,
            ..FragmentMetrics::default()
        },
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
        merge_metric(
            &mut joined.operator_wait_seconds,
            fragment.operator_wait_seconds,
        )?;
        merge_metric(&mut joined.reconnect_actions, fragment.reconnect_actions)?;
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
        value.report.calibrated && validate_artifact_reference(&value.artifact).is_ok()
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
    forecast_artifact: ArtifactReference,
    actual: &Observation,
) -> Result<TerminalOutcome> {
    validate_observation(actual)?;
    if forecast.schema != FORECAST_SCHEMA || forecast.target_issue != actual.issue {
        return invalid("terminal actual does not match forecast target");
    }
    validate_artifact_reference(&forecast_artifact)?;
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
        forecast_ref: forecast_artifact.reference,
        forecast_digest: forecast_artifact.digest,
        elapsed_seconds: elapsed,
        validation_seconds: validation,
        total_tokens: tokens,
        unknown_actuals,
    })
}

pub fn calibration_report(
    cases: &[BacktestCase],
    tolerance_basis_points: u64,
    source_manifest: ArtifactReference,
) -> Result<CalibrationReport> {
    validate_artifact_reference(&source_manifest)?;
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
        source_manifest,
    })
}

pub fn compare_cycle_time(
    root: &Path,
    baseline_artifact: &ArtifactReference,
    candidate_artifact: &ArtifactReference,
) -> Result<CycleTimeComparison> {
    let baseline = load_cycle_time_evidence(root, baseline_artifact)?;
    let candidate = load_cycle_time_evidence(root, candidate_artifact)?;
    let comparison_basis_equal = baseline.comparison_key == candidate.comparison_key;
    let required_gates: Vec<String> = ["validation", "review", "publication", "merge", "closeout"]
        .into_iter()
        .map(str::to_string)
        .collect();
    let gates_preserved =
        baseline.preserved_gates == required_gates && candidate.preserved_gates == required_gates;
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

pub fn load_cycle_time_evidence(
    root: &Path,
    artifact: &ArtifactReference,
) -> Result<CycleTimeCohort> {
    let source: CycleTimeEvidence = load_verified_json(root, artifact)?;
    derive_cycle_cohort(root, source)
}

pub fn load_verified_json<T: DeserializeOwned>(
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
    let bytes = fs::read(canonical_path)?;
    let observed = blake3::hash(&bytes).to_hex().to_string();
    if observed != artifact.digest {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            format!("artifact digest mismatch for {}", artifact.reference),
        ));
    }
    Ok(serde_json::from_slice(&bytes)?)
}

pub fn artifact_reference(root: &Path, reference: impl Into<String>) -> Result<ArtifactReference> {
    let reference = reference.into();
    validate_reference(&reference)?;
    let canonical_root = fs::canonicalize(root)?;
    let canonical_path = fs::canonicalize(root.join(&reference))?;
    if !canonical_path.starts_with(&canonical_root) {
        return invalid("artifact reference escapes the repository root");
    }
    let digest = blake3::hash(&fs::read(canonical_path)?)
        .to_hex()
        .to_string();
    Ok(ArtifactReference { reference, digest })
}

pub fn verified_calibration(
    root: &Path,
    artifact: ArtifactReference,
) -> Result<VerifiedCalibration> {
    let report: CalibrationReport = load_verified_json(root, &artifact)?;
    let manifest: CalibrationManifest = load_verified_json(root, &report.source_manifest)?;
    if manifest.schema != "csdlc.estimation_calibration_manifest.v1" {
        return invalid("calibration source manifest schema is invalid");
    }
    for source in &manifest.context_sources {
        let _: serde_json::Value = load_verified_json(root, source)?;
    }
    let mut cases = Vec::new();
    for entry in &manifest.cases {
        let forecast: Forecast = load_verified_json(root, &entry.forecast)?;
        let actual = load_observation_manifest(root, &entry.actual_observation)?;
        let outcome: TerminalOutcome = load_verified_json(root, &entry.outcome)?;
        if entry.issue != forecast.target_issue
            || entry.issue != actual.issue
            || entry.issue != outcome.issue
        {
            return invalid("calibration source case identity is inconsistent");
        }
        if outcome.forecast_digest != entry.forecast.digest
            || outcome.forecast_ref != entry.forecast.reference
        {
            return invalid("calibration outcome is not linked to its forecast bytes");
        }
        let derived = terminal_outcome(&forecast, entry.forecast.clone(), &actual)?;
        if derived != outcome {
            return Err(V2Error::new(
                ErrorCode::CorruptRecord,
                "calibration outcome does not reproduce from its verified actual sources",
            ));
        }
        cases.push(BacktestCase {
            issue: entry.issue,
            forecast,
            outcome,
        });
    }
    let recomputed = calibration_report(
        &cases,
        manifest.tolerance_basis_points,
        report.source_manifest.clone(),
    )?;
    if recomputed != report {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "calibration report is not reproducible from its retained source manifest",
        ));
    }
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

fn metrics(observation: &Observation) -> [&MetricObservation; 8] {
    [
        &observation.elapsed_seconds,
        &observation.active_work_seconds,
        &observation.validation_seconds,
        &observation.pr_wait_seconds,
        &observation.ci_wait_seconds,
        &observation.operator_wait_seconds,
        &observation.reconnect_actions,
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

#[derive(Default)]
struct FragmentMetrics {
    elapsed_seconds: Option<u64>,
    active_work_seconds: Option<u64>,
    validation_seconds: Option<u64>,
    pr_wait_seconds: Option<u64>,
    ci_wait_seconds: Option<u64>,
    total_tokens: Option<u64>,
    operator_wait_seconds: Option<u64>,
    reconnect_actions: Option<u64>,
    interrupted: bool,
}

fn fragment(
    issue: u64,
    key: ComparableKey,
    reference: String,
    source: ObservationSource,
    metrics: FragmentMetrics,
) -> Result<Observation> {
    if issue == 0 {
        return invalid("adapter issue must be non-zero");
    }
    validate_reference(&reference)?;
    let metric = |value| match value {
        Some(value) => MetricObservation::known(value, source, reference.clone()),
        None => MetricObservation::unavailable(
            if metrics.interrupted {
                Availability::Interrupted
            } else {
                Availability::Unknown
            },
            source,
            reference.clone(),
        ),
    };
    let observation = Observation {
        schema: OBSERVATION_SCHEMA.into(),
        issue,
        key,
        elapsed_seconds: metric(metrics.elapsed_seconds),
        active_work_seconds: metric(metrics.active_work_seconds),
        validation_seconds: metric(metrics.validation_seconds),
        pr_wait_seconds: metric(metrics.pr_wait_seconds),
        ci_wait_seconds: metric(metrics.ci_wait_seconds),
        operator_wait_seconds: metric(metrics.operator_wait_seconds),
        reconnect_actions: metric(metrics.reconnect_actions),
        total_tokens: metric(metrics.total_tokens),
    };
    validate_observation(&observation)?;
    Ok(observation)
}

fn checked_elapsed(start: u64, end: u64) -> Result<u64> {
    end.checked_sub(start)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "timing ends before it starts"))
}

fn parse_rfc3339(value: &str) -> Result<u64> {
    let parsed = OffsetDateTime::parse(value, &Rfc3339)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?;
    u64::try_from(parsed.unix_timestamp())
        .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "timestamp predates Unix epoch"))
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
    cohort.elapsed_seconds
}

fn derive_cycle_cohort(root: &Path, source: CycleTimeEvidence) -> Result<CycleTimeCohort> {
    if source.schema != "csdlc.cycle_time_evidence.v2" || source.observations.is_empty() {
        return invalid("cycle-time source is incomplete");
    }
    let mut issue_ids = BTreeSet::new();
    let mut observations = Vec::new();
    let mut provenance = Vec::new();
    let mut comparison_key = None;
    let mut preserved_gates = None;
    for artifact in &source.observations {
        let manifest: ObservationManifest = load_verified_json(root, artifact)?;
        let observation = load_observation_manifest(root, artifact)?;
        if !issue_ids.insert(observation.issue) {
            return invalid("cycle-time source contains an invalid or duplicate issue");
        }
        if comparison_key
            .as_ref()
            .is_some_and(|key| key != &observation.key)
        {
            return invalid("cycle-time observations do not share a source-derived identity");
        }
        comparison_key.get_or_insert_with(|| observation.key.clone());
        let gates = derive_preserved_gates(root, &manifest)?;
        if preserved_gates
            .as_ref()
            .is_some_and(|known| known != &gates)
        {
            return invalid(
                "cycle-time observations do not preserve identical source-derived gates",
            );
        }
        preserved_gates.get_or_insert(gates);
        provenance.push(artifact.clone());
        observations.push(observation);
    }
    let mut unknown = BTreeSet::new();
    let sum = |name: &str, values: Vec<Option<u64>>, unknown: &mut BTreeSet<String>| {
        if values.iter().any(Option::is_none) {
            unknown.insert(name.to_string());
        }
        values
            .into_iter()
            .flatten()
            .fold(0_u64, u64::saturating_add)
    };
    let active_work_seconds = sum(
        "active_work_seconds",
        observations
            .iter()
            .map(|item| item.active_work_seconds.value)
            .collect(),
        &mut unknown,
    );
    let elapsed_seconds = sum(
        "elapsed_seconds",
        observations
            .iter()
            .map(|item| item.elapsed_seconds.value)
            .collect(),
        &mut unknown,
    );
    let validation_seconds = sum(
        "validation_seconds",
        observations
            .iter()
            .map(|item| item.validation_seconds.value)
            .collect(),
        &mut unknown,
    );
    let pr_lifecycle_seconds = sum(
        "pr_lifecycle_seconds",
        observations
            .iter()
            .map(|item| item.pr_wait_seconds.value)
            .collect(),
        &mut unknown,
    );
    let ci_seconds = sum(
        "ci_seconds",
        observations
            .iter()
            .map(|item| item.ci_wait_seconds.value)
            .collect(),
        &mut unknown,
    );
    let wait_seconds = sum(
        "wait_seconds",
        observations
            .iter()
            .map(|item| item.operator_wait_seconds.value)
            .collect(),
        &mut unknown,
    );
    let reconnect_actions = sum(
        "reconnect_actions",
        observations
            .iter()
            .map(|item| item.reconnect_actions.value)
            .collect(),
        &mut unknown,
    );
    Ok(CycleTimeCohort {
        id: format!(
            "issues-{}",
            issue_ids
                .iter()
                .map(u64::to_string)
                .collect::<Vec<_>>()
                .join("-")
        ),
        issue_count: issue_ids.len(),
        elapsed_seconds,
        active_work_seconds,
        validation_seconds,
        pr_lifecycle_seconds,
        ci_seconds,
        wait_seconds,
        reconnect_actions,
        measured: true,
        unknown_components: unknown.into_iter().collect(),
        provenance,
        comparison_key: comparison_key.expect("non-empty observations establish a key"),
        preserved_gates: preserved_gates.expect("non-empty observations establish gates"),
    })
}

fn derive_preserved_gates(root: &Path, manifest: &ObservationManifest) -> Result<Vec<String>> {
    let lifecycle_ref = manifest.lifecycle.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "cycle evidence requires lifecycle proof",
        )
    })?;
    let validation_ref = manifest.validation.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::InvalidInput,
            "cycle evidence requires validation proof",
        )
    })?;
    let lifecycle: IssueRecord = load_verified_json(root, lifecycle_ref)?;
    verify_issue_record(&lifecycle)?;
    let validation_manifest: ValidationSourceManifest = load_verified_json(root, validation_ref)?;
    let validation_identity: IssueRecord =
        load_verified_json(root, &validation_manifest.issue_record)?;
    verify_issue_record(&validation_identity)?;
    let report: ExecutionReport = load_verified_json(root, &validation_manifest.execution_report)?;
    if lifecycle.issue != manifest.issue || validation_identity.issue != manifest.issue {
        return invalid("cycle gate evidence does not match the observation issue");
    }
    let validation_passed = !report.evidence.is_empty()
        && report.evidence.iter().all(|lane| {
            matches!(
                lane.status,
                crate::pvf::LaneStatus::Passed | crate::pvf::LaneStatus::AcceptedNonGoal
            )
        });
    let terminal_merged = lifecycle.terminal.as_ref().is_some_and(|terminal| {
        terminal.disposition == crate::readiness::TerminalDisposition::Merged
    });
    let closed_out = lifecycle.phase == crate::model::LifecyclePhase::ClosedOut;
    let mut gates = Vec::new();
    if validation_passed {
        gates.push("validation".into());
    }
    if lifecycle
        .review
        .as_ref()
        .is_some_and(|review| review.completed)
    {
        gates.push("review".into());
    }
    if lifecycle.publication.is_some() {
        gates.push("publication".into());
    }
    if terminal_merged {
        gates.push("merge".into());
    }
    if closed_out {
        gates.push("closeout".into());
    }
    Ok(gates)
}

fn signed_difference(left: u64, right: u64) -> i64 {
    let difference = i128::from(left) - i128::from(right);
    difference.clamp(i128::from(i64::MIN), i128::from(i64::MAX)) as i64
}

fn verify_issue_record(record: &IssueRecord) -> Result<()> {
    if record.issue == 0 || record.digest != record_digest(record)? {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "retained issue record failed its canonical digest",
        ));
    }
    Ok(())
}

fn invalid<T>(message: impl Into<String>) -> Result<T> {
    Err(V2Error::new(ErrorCode::InvalidInput, message))
}
