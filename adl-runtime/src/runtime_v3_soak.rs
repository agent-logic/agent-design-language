//! Bounded Runtime v3 soak contracts.
//!
//! This module is intentionally provider-free.  It defines the deterministic
//! configuration, sampling, threshold, evidence, cancellation, and cleanup
//! contracts that a later runner can consume without turning local validation
//! into an AWS or paid-resource qualification.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const SOAK_CONTRACT_SCHEMA: &str = "adl.runtime_v3.bounded_soak_contract.v1";
pub const SOAK_EVIDENCE_SCHEMA: &str = "adl.runtime_v3.bounded_soak_evidence.v1";
pub const MAX_BOUNDED_SOAK_DURATION_SECONDS: u64 = 24 * 60 * 60;
pub const MAX_BOUNDED_SOAK_SAMPLE_COUNT: u64 = 100_000;
pub const AGENT_LOGIC_ACCOUNT_PROFILE: &str = "agent-logic-admin";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakConfig {
    pub schema: String,
    pub issue: u64,
    pub revision: String,
    pub owner: RunOwner,
    pub bounds: SoakBounds,
    pub workload: WorkloadContract,
    pub faults: Vec<FaultContract>,
    pub thresholds: SoakThresholds,
    pub cleanup: CleanupContract,
    pub binaries: BTreeMap<String, BinaryIdentity>,
    pub platform: PlatformIdentity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunOwner {
    pub account_profile: String,
    pub run_id: String,
    pub operator: String,
    pub tags: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakBounds {
    pub duration_seconds: u64,
    pub sample_interval_seconds: u64,
    pub max_hourly_cost_cents: u64,
    pub max_total_cost_cents: u64,
    pub deadline_unix_seconds: u64,
    pub kill_switch: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct WorkloadContract {
    pub connection_count: u64,
    pub authenticated_https: bool,
    pub authenticated_wss: bool,
    pub guardian_kernel_cycle: bool,
    pub operation_mix: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultContract {
    pub name: String,
    pub kind: FaultKind,
    pub inject_after_seconds: u64,
    pub expected_recovery_seconds: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FaultKind {
    TransportDrop,
    GuardianRestart,
    ObservabilityStall,
    ResourcePressure,
    RecoveryReplay,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakThresholds {
    pub max_observability_staleness_seconds: u64,
    pub max_missing_sample_count: u64,
    pub max_restart_count: u64,
    pub max_backoff_seconds: u64,
    pub max_transport_error_count: u64,
    pub max_recovery_seconds: u64,
    pub max_resource_growth_percent: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupContract {
    pub required: bool,
    pub zero_residue_paths: Vec<String>,
    pub cancellation_receipt_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryIdentity {
    pub path: String,
    pub sha256: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlatformIdentity {
    pub os: String,
    pub arch: String,
    pub runner_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakSample {
    pub sequence: u64,
    pub observed_unix_seconds: u64,
    pub observability_cursor_unix_seconds: Option<u64>,
    pub resource_growth_percent: u64,
    pub restart_count: u64,
    pub backoff_seconds: u64,
    pub transport_error_count: u64,
    pub recovery_seconds: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FaultRecord {
    pub name: String,
    pub injected_unix_seconds: u64,
    pub recovered_unix_seconds: Option<u64>,
    pub notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CleanupOutcome {
    pub cancellation_requested: bool,
    pub cancellation_receipt: Option<String>,
    pub residue: BTreeMap<String, u64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakEvaluation {
    pub status: SoakStatus,
    pub violations: Vec<SoakViolation>,
    pub sample_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SoakStatus {
    Pass,
    FailClosed,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakViolation {
    pub code: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SoakEvidence {
    pub schema: String,
    pub config_sha256: String,
    pub revision: String,
    pub binaries: BTreeMap<String, BinaryIdentity>,
    pub platform: PlatformIdentity,
    pub clock: EvidenceClock,
    pub samples: Vec<SoakSample>,
    pub faults: Vec<FaultRecord>,
    pub cleanup: CleanupOutcome,
    pub evaluation: SoakEvaluation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct EvidenceClock {
    pub started_unix_seconds: u64,
    pub finished_unix_seconds: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunnerPlan {
    pub schema: String,
    pub revision: String,
    pub run_id: String,
    pub actions: Vec<RunnerAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RunnerAction {
    pub sequence: u64,
    pub at_seconds: u64,
    pub kind: RunnerActionKind,
    pub detail: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RunnerActionKind {
    StartWorkload,
    Sample,
    InjectFault,
    EvaluateThresholds,
    Cleanup,
}

pub fn validate_config(config: &SoakConfig) -> Result<(), Vec<SoakViolation>> {
    let mut violations = Vec::new();
    if config.schema != SOAK_CONTRACT_SCHEMA {
        violations.push(violation(
            "schema",
            format!("expected {SOAK_CONTRACT_SCHEMA}, got {}", config.schema),
        ));
    }
    if config.issue == 0 {
        violations.push(violation("issue", "issue must be nonzero"));
    }
    if config.revision.trim().is_empty() {
        violations.push(violation("revision", "revision must be present"));
    }
    if config.owner.account_profile.trim().is_empty() {
        violations.push(violation("owner", "account profile is required"));
    }
    if config.owner.account_profile != AGENT_LOGIC_ACCOUNT_PROFILE {
        violations.push(violation(
            "owner",
            format!(
                "account profile {} is not approved profile {AGENT_LOGIC_ACCOUNT_PROFILE}",
                config.owner.account_profile
            ),
        ));
    }
    if config.owner.run_id.trim().is_empty() {
        violations.push(violation("owner", "run id is required"));
    }
    if config.owner.operator.trim().is_empty() {
        violations.push(violation("owner", "operator is required"));
    }
    match config.owner.tags.get("adl:issue") {
        Some(issue) if issue == &config.issue.to_string() => {}
        Some(issue) => violations.push(violation(
            "owner",
            format!(
                "adl:issue tag {issue} does not match issue {}",
                config.issue
            ),
        )),
        None => violations.push(violation("owner", "adl:issue tag is required")),
    }
    if config.bounds.duration_seconds == 0 {
        violations.push(violation("bounds", "duration must be nonzero"));
    }
    if config.bounds.duration_seconds > MAX_BOUNDED_SOAK_DURATION_SECONDS {
        violations.push(violation(
            "bounds",
            format!(
                "duration {} exceeds bounded maximum {}",
                config.bounds.duration_seconds, MAX_BOUNDED_SOAK_DURATION_SECONDS
            ),
        ));
    }
    if config.bounds.sample_interval_seconds == 0 {
        violations.push(violation("bounds", "sample interval must be nonzero"));
    }
    if config.bounds.duration_seconds < config.bounds.sample_interval_seconds {
        violations.push(violation(
            "bounds",
            "duration must be at least one sample interval",
        ));
    }
    if config.bounds.max_hourly_cost_cents == 0 || config.bounds.max_total_cost_cents == 0 {
        violations.push(violation(
            "cost",
            "hourly and total cost ceilings are required",
        ));
    }
    if config.bounds.deadline_unix_seconds == 0 {
        violations.push(violation("deadline", "deadline must be present"));
    }
    if let Some(required_samples) = required_sample_count(config) {
        if required_samples > MAX_BOUNDED_SOAK_SAMPLE_COUNT {
            violations.push(violation(
                "bounds",
                format!(
                    "planned sample count {required_samples} exceeds bounded maximum {}",
                    MAX_BOUNDED_SOAK_SAMPLE_COUNT
                ),
            ));
        }
        if config.thresholds.max_missing_sample_count >= required_samples {
            violations.push(violation(
                "thresholds",
                format!(
                    "missing sample tolerance {} must be below required sample count {required_samples}",
                    config.thresholds.max_missing_sample_count
                ),
            ));
        }
    }
    if config.bounds.kill_switch.trim().is_empty() {
        violations.push(violation("kill_switch", "kill switch must be declared"));
    }
    if config.workload.connection_count == 0 {
        violations.push(violation("workload", "connection count must be nonzero"));
    }
    if !config.workload.authenticated_https || !config.workload.authenticated_wss {
        violations.push(violation(
            "workload",
            "https and wss workload legs must be authenticated",
        ));
    }
    if !config.workload.guardian_kernel_cycle {
        violations.push(violation(
            "workload",
            "guardian/kernel workload cycle must be declared",
        ));
    }
    if config.workload.operation_mix.is_empty() {
        violations.push(violation("workload", "operation mix must be nonempty"));
    }
    for (operation, weight) in &config.workload.operation_mix {
        if operation.trim().is_empty() || *weight == 0 {
            violations.push(violation(
                "workload",
                "operation mix names and weights must be nonempty",
            ));
        }
    }
    if config.faults.is_empty() {
        violations.push(violation(
            "faults",
            "at least one deterministic fault is required",
        ));
    }
    let mut fault_names = BTreeSet::new();
    for fault in &config.faults {
        if fault.name.trim().is_empty() {
            violations.push(violation("faults", "fault name must be nonempty"));
        }
        if !fault_names.insert(fault.name.clone()) {
            violations.push(violation(
                "faults",
                format!("duplicate fault name {}", fault.name),
            ));
        }
        if fault.inject_after_seconds >= config.bounds.duration_seconds {
            violations.push(violation(
                "faults",
                format!("fault {} injects outside duration", fault.name),
            ));
        }
        if fault.expected_recovery_seconds > config.thresholds.max_recovery_seconds {
            violations.push(violation(
                "faults",
                format!("fault {} expected recovery exceeds threshold", fault.name),
            ));
        }
    }
    if !config.cleanup.required {
        violations.push(violation("cleanup", "cleanup contract must be required"));
    }
    if config.cleanup.zero_residue_paths.is_empty() {
        violations.push(violation(
            "cleanup",
            "at least one zero-residue path must be declared",
        ));
    }
    let mut cleanup_paths = BTreeSet::new();
    for path in &config.cleanup.zero_residue_paths {
        let normalized = path.trim();
        if normalized.is_empty() {
            violations.push(violation("cleanup", "zero-residue paths must be nonblank"));
        } else if !cleanup_paths.insert(normalized) {
            violations.push(violation(
                "cleanup",
                format!("duplicate zero-residue path {normalized}"),
            ));
        }
    }
    if !config.cleanup.cancellation_receipt_required {
        violations.push(violation(
            "cleanup",
            "cancellation receipt must be required",
        ));
    }
    if config.binaries.is_empty() {
        violations.push(violation(
            "binaries",
            "at least one binary identity is required",
        ));
    }
    for (name, binary) in &config.binaries {
        if binary.path.trim().is_empty() {
            violations.push(violation("binaries", format!("{name} path is empty")));
        }
        if !is_sha256(&binary.sha256) {
            violations.push(violation(
                "binaries",
                format!("{name} sha256 must be a lowercase 64-hex digest"),
            ));
        }
    }
    if config.platform.os.trim().is_empty()
        || config.platform.arch.trim().is_empty()
        || config.platform.runner_class.trim().is_empty()
    {
        violations.push(violation("platform", "platform identity must be complete"));
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

pub fn build_runner_plan(config: &SoakConfig) -> Result<RunnerPlan, Vec<SoakViolation>> {
    validate_config(config)?;
    let mut actions = Vec::new();
    actions.push(RunnerAction {
        sequence: 0,
        at_seconds: 0,
        kind: RunnerActionKind::StartWorkload,
        detail: format!(
            "connections={} https={} wss={} guardian_kernel_cycle={}",
            config.workload.connection_count,
            config.workload.authenticated_https,
            config.workload.authenticated_wss,
            config.workload.guardian_kernel_cycle
        ),
    });
    let mut next_sequence = 1;
    let mut sample_at = 0;
    while sample_at < config.bounds.duration_seconds {
        actions.push(RunnerAction {
            sequence: next_sequence,
            at_seconds: sample_at,
            kind: RunnerActionKind::Sample,
            detail: "collect observability/resource/transport/recovery sample".to_owned(),
        });
        next_sequence += 1;
        sample_at += config.bounds.sample_interval_seconds;
    }
    for fault in &config.faults {
        actions.push(RunnerAction {
            sequence: next_sequence,
            at_seconds: fault.inject_after_seconds,
            kind: RunnerActionKind::InjectFault,
            detail: format!("{}:{:?}", fault.name, fault.kind),
        });
        next_sequence += 1;
    }
    actions.push(RunnerAction {
        sequence: next_sequence,
        at_seconds: config.bounds.duration_seconds,
        kind: RunnerActionKind::EvaluateThresholds,
        detail: "evaluate bounded soak thresholds".to_owned(),
    });
    next_sequence += 1;
    actions.push(RunnerAction {
        sequence: next_sequence,
        at_seconds: config.bounds.duration_seconds,
        kind: RunnerActionKind::Cleanup,
        detail: config.cleanup.zero_residue_paths.join(","),
    });
    actions.sort_by_key(|action| (action.at_seconds, action.sequence));
    for (index, action) in actions.iter_mut().enumerate() {
        action.sequence = index as u64;
    }

    Ok(RunnerPlan {
        schema: "adl.runtime_v3.bounded_soak_runner_plan.v1".to_owned(),
        revision: config.revision.clone(),
        run_id: config.owner.run_id.clone(),
        actions,
    })
}

pub fn evaluate_soak(
    config: &SoakConfig,
    samples: &[SoakSample],
    cleanup: &CleanupOutcome,
) -> SoakEvaluation {
    let mut violations = match validate_config(config) {
        Ok(()) => Vec::new(),
        Err(violations) => violations,
    };
    if let Some(required_samples) = required_sample_count(config) {
        if (samples.len() as u64).saturating_add(config.thresholds.max_missing_sample_count)
            < required_samples
        {
            violations.push(violation(
                "missing_samples",
                format!(
                    "sample count {} below required {} with tolerance {}",
                    samples.len(),
                    required_samples,
                    config.thresholds.max_missing_sample_count
                ),
            ));
        }
    }
    let mut seen = BTreeSet::new();
    for sample in samples {
        if !seen.insert(sample.sequence) {
            violations.push(violation(
                "sample_sequence",
                format!("duplicate sample sequence {}", sample.sequence),
            ));
        }
        match sample.observability_cursor_unix_seconds {
            Some(cursor) => {
                if cursor > sample.observed_unix_seconds {
                    violations.push(violation(
                        "future_observability",
                        format!(
                            "sample {} cursor {} is after observed time {}",
                            sample.sequence, cursor, sample.observed_unix_seconds
                        ),
                    ));
                    continue;
                }
                let staleness = sample.observed_unix_seconds.saturating_sub(cursor);
                if staleness > config.thresholds.max_observability_staleness_seconds {
                    violations.push(violation(
                        "stale_observability",
                        format!(
                            "sample {} staleness {} exceeds {}",
                            sample.sequence,
                            staleness,
                            config.thresholds.max_observability_staleness_seconds
                        ),
                    ));
                }
            }
            None => violations.push(violation(
                "missing_observability",
                format!("sample {} has no observability cursor", sample.sequence),
            )),
        }
        if sample.resource_growth_percent > config.thresholds.max_resource_growth_percent {
            violations.push(violation(
                "resource_growth",
                format!(
                    "sample {} resource growth {} exceeds {}",
                    sample.sequence,
                    sample.resource_growth_percent,
                    config.thresholds.max_resource_growth_percent
                ),
            ));
        }
        if sample.restart_count > config.thresholds.max_restart_count {
            violations.push(violation(
                "restart_count",
                format!(
                    "sample {} restart count {} exceeds {}",
                    sample.sequence, sample.restart_count, config.thresholds.max_restart_count
                ),
            ));
        }
        if sample.backoff_seconds > config.thresholds.max_backoff_seconds {
            violations.push(violation(
                "backoff",
                format!(
                    "sample {} backoff {} exceeds {}",
                    sample.sequence, sample.backoff_seconds, config.thresholds.max_backoff_seconds
                ),
            ));
        }
        if sample.transport_error_count > config.thresholds.max_transport_error_count {
            violations.push(violation(
                "transport",
                format!(
                    "sample {} transport errors {} exceeds {}",
                    sample.sequence,
                    sample.transport_error_count,
                    config.thresholds.max_transport_error_count
                ),
            ));
        }
        if let Some(recovery_seconds) = sample.recovery_seconds {
            if recovery_seconds > config.thresholds.max_recovery_seconds {
                violations.push(violation(
                    "recovery",
                    format!(
                        "sample {} recovery {} exceeds {}",
                        sample.sequence, recovery_seconds, config.thresholds.max_recovery_seconds
                    ),
                ));
            }
        }
    }
    if cleanup.cancellation_requested
        && config.cleanup.cancellation_receipt_required
        && cleanup
            .cancellation_receipt
            .as_deref()
            .unwrap_or("")
            .trim()
            .is_empty()
    {
        violations.push(violation(
            "cancellation",
            "cancellation requested without receipt",
        ));
    }
    for required_path in &config.cleanup.zero_residue_paths {
        match cleanup.residue.get(required_path).copied() {
            Some(0) => {}
            Some(residue) => {
                violations.push(violation(
                    "cleanup_residue",
                    format!("{required_path} retained {residue} residue entries"),
                ));
            }
            None => {
                violations.push(violation(
                    "cleanup_residue",
                    format!("{required_path} missing required zero-residue evidence"),
                ));
            }
        }
    }

    SoakEvaluation {
        status: if violations.is_empty() {
            SoakStatus::Pass
        } else {
            SoakStatus::FailClosed
        },
        violations,
        sample_count: samples.len(),
    }
}

pub fn build_evidence(
    config: &SoakConfig,
    clock: EvidenceClock,
    samples: Vec<SoakSample>,
    faults: Vec<FaultRecord>,
    cleanup: CleanupOutcome,
) -> Result<SoakEvidence, Vec<SoakViolation>> {
    if clock.finished_unix_seconds < clock.started_unix_seconds {
        return Err(vec![violation(
            "clock",
            "finished clock must not be before started clock",
        )]);
    }
    let mut evaluation = evaluate_soak(config, &samples, &cleanup);
    evaluation
        .violations
        .extend(evaluate_schedule(config, &clock, &samples, &faults));
    evaluation
        .violations
        .extend(evaluate_fault_records(config, &faults));
    if !evaluation.violations.is_empty() {
        evaluation.status = SoakStatus::FailClosed;
    }
    Ok(SoakEvidence {
        schema: SOAK_EVIDENCE_SCHEMA.to_owned(),
        config_sha256: canonical_sha256(config)?,
        revision: config.revision.clone(),
        binaries: config.binaries.clone(),
        platform: config.platform.clone(),
        clock,
        samples,
        faults,
        cleanup,
        evaluation,
    })
}

fn required_sample_count(config: &SoakConfig) -> Option<u64> {
    let interval = config.bounds.sample_interval_seconds;
    if interval == 0 {
        return None;
    }
    Some(config.bounds.duration_seconds.div_ceil(interval))
}

fn evaluate_fault_records(config: &SoakConfig, faults: &[FaultRecord]) -> Vec<SoakViolation> {
    let mut violations = Vec::new();
    let mut configured = BTreeMap::new();
    for fault in &config.faults {
        configured.insert(fault.name.as_str(), fault);
    }
    let mut seen = BTreeSet::new();
    for record in faults {
        if !seen.insert(record.name.as_str()) {
            violations.push(violation(
                "fault_evidence",
                format!("duplicate fault evidence {}", record.name),
            ));
            continue;
        }
        match configured.get(record.name.as_str()) {
            Some(contract) => match record.recovered_unix_seconds {
                Some(recovered) => {
                    if recovered < record.injected_unix_seconds {
                        violations.push(violation(
                            "fault_evidence",
                            format!("fault {} recovered before injection", record.name),
                        ));
                        continue;
                    }
                    let recovery = recovered.saturating_sub(record.injected_unix_seconds);
                    if recovery > contract.expected_recovery_seconds {
                        violations.push(violation(
                            "fault_evidence",
                            format!(
                                "fault {} recovered in {}s, expected <= {}s",
                                record.name, recovery, contract.expected_recovery_seconds
                            ),
                        ));
                    }
                }
                None => violations.push(violation(
                    "fault_evidence",
                    format!("fault {} has no recovery evidence", record.name),
                )),
            },
            None => violations.push(violation(
                "fault_evidence",
                format!("unknown fault evidence {}", record.name),
            )),
        }
    }
    for name in configured.keys() {
        if !seen.contains(name) {
            violations.push(violation(
                "fault_evidence",
                format!("missing configured fault evidence {name}"),
            ));
        }
    }
    violations
}

fn evaluate_schedule(
    config: &SoakConfig,
    clock: &EvidenceClock,
    samples: &[SoakSample],
    faults: &[FaultRecord],
) -> Vec<SoakViolation> {
    let mut violations = Vec::new();
    let Some(required_samples) = required_sample_count(config) else {
        return violations;
    };
    let elapsed = clock
        .finished_unix_seconds
        .saturating_sub(clock.started_unix_seconds);
    if clock.finished_unix_seconds > config.bounds.deadline_unix_seconds {
        violations.push(violation(
            "schedule",
            format!(
                "finished clock {} exceeds deadline {}",
                clock.finished_unix_seconds, config.bounds.deadline_unix_seconds
            ),
        ));
    }
    match clock
        .started_unix_seconds
        .checked_add(config.bounds.duration_seconds)
    {
        Some(required_finish) => {
            if required_finish > config.bounds.deadline_unix_seconds {
                violations.push(violation(
                    "schedule",
                    format!(
                        "configured duration from start exceeds deadline {}",
                        config.bounds.deadline_unix_seconds
                    ),
                ));
            }
        }
        None => violations.push(violation(
            "schedule",
            "configured duration overflows evidence clock",
        )),
    }
    if elapsed < config.bounds.duration_seconds {
        violations.push(violation(
            "schedule",
            format!(
                "clock elapsed {elapsed}s below configured duration {}s",
                config.bounds.duration_seconds
            ),
        ));
    }
    let mut by_sequence = BTreeMap::new();
    for sample in samples {
        if sample.sequence >= required_samples {
            violations.push(violation(
                "schedule",
                format!("unscheduled sample sequence {}", sample.sequence),
            ));
            continue;
        }
        if by_sequence.insert(sample.sequence, sample).is_some() {
            violations.push(violation(
                "schedule",
                format!("duplicate scheduled sample {}", sample.sequence),
            ));
        }
    }
    let mut missing_samples = 0;
    for expected_sequence in 0..required_samples {
        match by_sequence.get(&expected_sequence) {
            Some(sample) => {
                let expected_offset =
                    expected_sequence.checked_mul(config.bounds.sample_interval_seconds);
                let expected_time = expected_offset
                    .and_then(|offset| clock.started_unix_seconds.checked_add(offset));
                match expected_time {
                    Some(expected_time) => {
                        if sample.observed_unix_seconds != expected_time {
                            violations.push(violation(
                                "schedule",
                                format!(
                                    "sample {} observed at {}, expected {}",
                                    sample.sequence, sample.observed_unix_seconds, expected_time
                                ),
                            ));
                        }
                    }
                    None => violations.push(violation(
                        "schedule",
                        format!("sample {expected_sequence} schedule overflows evidence clock"),
                    )),
                }
            }
            None => {
                missing_samples += 1;
            }
        }
    }
    if missing_samples > config.thresholds.max_missing_sample_count {
        violations.push(violation(
            "schedule",
            format!(
                "missing scheduled samples {missing_samples} exceeds tolerance {}",
                config.thresholds.max_missing_sample_count
            ),
        ));
    }
    for record in faults {
        if let Some(contract) = config
            .faults
            .iter()
            .find(|contract| contract.name == record.name)
        {
            match clock
                .started_unix_seconds
                .checked_add(contract.inject_after_seconds)
            {
                Some(expected_injection) => {
                    if record.injected_unix_seconds != expected_injection {
                        violations.push(violation(
                            "schedule",
                            format!(
                                "fault {} injected at {}, expected {}",
                                record.name, record.injected_unix_seconds, expected_injection
                            ),
                        ));
                    }
                }
                None => violations.push(violation(
                    "schedule",
                    format!("fault {} schedule overflows evidence clock", record.name),
                )),
            }
        }
    }
    violations
}

pub fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, Vec<SoakViolation>> {
    let bytes = serde_jcs::to_vec(value).map_err(|error| {
        vec![violation(
            "canonical_json",
            format!("could not canonicalize value: {error}"),
        )]
    })?;
    let digest = Sha256::digest(bytes);
    Ok(hex::encode(digest))
}

fn violation(code: impl Into<String>, detail: impl Into<String>) -> SoakViolation {
    SoakViolation {
        code: code.into(),
        detail: detail.into(),
    }
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
