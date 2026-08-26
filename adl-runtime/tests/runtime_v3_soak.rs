use std::collections::BTreeMap;

use adl_runtime::runtime_v3_soak::{
    build_evidence, build_runner_plan, evaluate_soak, validate_config, BinaryIdentity,
    CleanupContract, CleanupOutcome, EvidenceClock, FaultContract, FaultKind, FaultRecord,
    PlatformIdentity, RunOwner, RunnerActionKind, SoakBounds, SoakConfig, SoakSample, SoakStatus,
    SoakThresholds, WorkloadContract, MAX_BOUNDED_SOAK_DURATION_SECONDS,
    MAX_BOUNDED_SOAK_SAMPLE_COUNT, SOAK_CONTRACT_SCHEMA, SOAK_EVIDENCE_SCHEMA,
};

fn valid_config() -> SoakConfig {
    let mut tags = BTreeMap::new();
    tags.insert("adl:issue".to_owned(), "266".to_owned());
    tags.insert("adl:owner".to_owned(), "runtime-v3-soak".to_owned());
    let mut binaries = BTreeMap::new();
    binaries.insert(
        "adl-runtime-guardian".to_owned(),
        BinaryIdentity {
            path: "adl-runtime/target/release/adl-runtime-guardian".to_owned(),
            sha256: "a".repeat(64),
        },
    );
    SoakConfig {
        schema: SOAK_CONTRACT_SCHEMA.to_owned(),
        issue: 266,
        revision: "5783730f3e1d4a2cac8ce8e55886e313575bc8ae".to_owned(),
        owner: RunOwner {
            account_profile: "agent-logic-admin".to_owned(),
            run_id: "runtime-v3-soak-local-contract".to_owned(),
            operator: "codex".to_owned(),
            tags,
        },
        bounds: SoakBounds {
            duration_seconds: 60,
            sample_interval_seconds: 10,
            max_hourly_cost_cents: 1,
            max_total_cost_cents: 1,
            deadline_unix_seconds: 1_800_000_000,
            kill_switch: "touch .adl/runtime-v3/soak/cancel".to_owned(),
        },
        workload: WorkloadContract {
            connection_count: 50,
            authenticated_https: true,
            authenticated_wss: true,
            guardian_kernel_cycle: true,
            operation_mix: BTreeMap::from([
                ("continuity_tick".to_owned(), 4),
                ("guardian_health".to_owned(), 2),
                ("recovery_probe".to_owned(), 1),
            ]),
        },
        faults: vec![
            FaultContract {
                name: "transport-drop".to_owned(),
                kind: FaultKind::TransportDrop,
                inject_after_seconds: 10,
                expected_recovery_seconds: 2,
            },
            FaultContract {
                name: "guardian-restart".to_owned(),
                kind: FaultKind::GuardianRestart,
                inject_after_seconds: 20,
                expected_recovery_seconds: 3,
            },
        ],
        thresholds: SoakThresholds {
            max_observability_staleness_seconds: 5,
            max_missing_sample_count: 0,
            max_restart_count: 1,
            max_backoff_seconds: 2,
            max_transport_error_count: 0,
            max_recovery_seconds: 3,
            max_resource_growth_percent: 10,
        },
        cleanup: CleanupContract {
            required: true,
            zero_residue_paths: vec![
                ".adl/runtime-v3/soak/instances".to_owned(),
                ".adl/runtime-v3/soak/locks".to_owned(),
            ],
            cancellation_receipt_required: true,
        },
        binaries,
        platform: PlatformIdentity {
            os: "linux".to_owned(),
            arch: "x86_64".to_owned(),
            runner_class: "local-contract".to_owned(),
        },
    }
}

fn passing_samples() -> Vec<SoakSample> {
    (0..6)
        .map(|sequence| SoakSample {
            sequence,
            observed_unix_seconds: 1_700_000_000 + sequence * 10,
            observability_cursor_unix_seconds: Some(1_700_000_000 + sequence * 10),
            resource_growth_percent: 1,
            restart_count: 0,
            backoff_seconds: 0,
            transport_error_count: 0,
            recovery_seconds: Some(1),
        })
        .collect()
}

fn passing_faults() -> Vec<FaultRecord> {
    vec![
        FaultRecord {
            name: "transport-drop".to_owned(),
            injected_unix_seconds: 1_700_000_010,
            recovered_unix_seconds: Some(1_700_000_011),
            notes: "deterministic fixture".to_owned(),
        },
        FaultRecord {
            name: "guardian-restart".to_owned(),
            injected_unix_seconds: 1_700_000_020,
            recovered_unix_seconds: Some(1_700_000_022),
            notes: "deterministic fixture".to_owned(),
        },
    ]
}

fn clean_cleanup() -> CleanupOutcome {
    let mut residue = BTreeMap::new();
    residue.insert(".adl/runtime-v3/soak/instances".to_owned(), 0);
    residue.insert(".adl/runtime-v3/soak/locks".to_owned(), 0);
    CleanupOutcome {
        cancellation_requested: false,
        cancellation_receipt: None,
        residue,
    }
}

#[test]
fn config_rejects_missing_owner_cost_revision_and_cleanup_inputs() {
    let mut config = valid_config();
    config.revision.clear();
    config.owner.account_profile.clear();
    config.owner.tags.remove("adl:issue");
    config.bounds.max_hourly_cost_cents = 0;
    config.bounds.max_total_cost_cents = 0;
    config.cleanup.required = false;
    config.cleanup.zero_residue_paths.clear();
    config.cleanup.cancellation_receipt_required = false;

    let violations = validate_config(&config).expect_err("invalid config must fail closed");
    let codes = violations
        .iter()
        .map(|violation| violation.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"revision"));
    assert!(codes.contains(&"owner"));
    assert!(codes.contains(&"cost"));
    assert!(codes.contains(&"cleanup"));
}

#[test]
fn config_rejects_mismatched_issue_tag_and_blank_or_duplicate_cleanup_paths() {
    let mut config = valid_config();
    config
        .owner
        .tags
        .insert("adl:issue".to_owned(), "999".to_owned());
    config.cleanup.zero_residue_paths = vec![
        ".adl/runtime-v3/soak/instances".to_owned(),
        "   ".to_owned(),
        ".adl/runtime-v3/soak/instances".to_owned(),
    ];

    let violations = validate_config(&config).expect_err("invalid owner/cleanup config");
    let codes = violations
        .iter()
        .map(|violation| violation.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"owner"));
    assert!(codes.contains(&"cleanup"));
}

#[test]
fn config_rejects_non_agent_logic_account_profile() {
    let mut config = valid_config();
    config.owner.account_profile = "default".to_owned();

    let violations = validate_config(&config).expect_err("wrong profile must fail");
    assert!(violations.iter().any(|violation| violation.code == "owner"));
}

#[test]
fn evaluator_passes_complete_bounded_local_contract_without_provider_use() {
    let config = valid_config();
    let evaluation = evaluate_soak(&config, &passing_samples(), &clean_cleanup());

    assert_eq!(evaluation.status, SoakStatus::Pass);
    assert_eq!(evaluation.sample_count, 6);
    assert!(evaluation.violations.is_empty());
}

#[test]
fn runner_plan_declares_workload_sampling_fault_evaluation_and_cleanup_actions() {
    let config = valid_config();
    let plan = build_runner_plan(&config).expect("runner plan");
    let action_kinds = plan
        .actions
        .iter()
        .map(|action| action.kind)
        .collect::<Vec<_>>();

    assert_eq!(plan.revision, config.revision);
    assert_eq!(plan.run_id, config.owner.run_id);
    assert_eq!(action_kinds.first(), Some(&RunnerActionKind::StartWorkload));
    assert_eq!(
        action_kinds
            .iter()
            .filter(|kind| **kind == RunnerActionKind::Sample)
            .count(),
        6
    );
    assert_eq!(
        action_kinds
            .iter()
            .filter(|kind| **kind == RunnerActionKind::InjectFault)
            .count(),
        2
    );
    assert!(action_kinds.contains(&RunnerActionKind::EvaluateThresholds));
    assert_eq!(action_kinds.last(), Some(&RunnerActionKind::Cleanup));
}

#[test]
fn runner_plan_rejects_unauthenticated_workload_and_out_of_bounds_faults() {
    let mut config = valid_config();
    config.workload.authenticated_wss = false;
    config.faults.push(FaultContract {
        name: "late-fault".to_owned(),
        kind: FaultKind::RecoveryReplay,
        inject_after_seconds: config.bounds.duration_seconds,
        expected_recovery_seconds: 99,
    });

    let violations = build_runner_plan(&config).expect_err("invalid runner plan must fail");
    let codes = violations
        .iter()
        .map(|violation| violation.code.as_str())
        .collect::<Vec<_>>();
    assert!(codes.contains(&"workload"));
    assert!(codes.contains(&"faults"));
}

#[test]
fn runner_plan_rejects_unbounded_duration_and_sample_count() {
    let mut huge_duration = valid_config();
    huge_duration.bounds.duration_seconds = MAX_BOUNDED_SOAK_DURATION_SECONDS + 1;
    huge_duration.bounds.deadline_unix_seconds = u64::MAX;
    assert!(build_runner_plan(&huge_duration)
        .expect_err("huge duration must fail")
        .iter()
        .any(|violation| violation.code == "bounds"));

    let mut huge_samples = valid_config();
    huge_samples.bounds.duration_seconds = MAX_BOUNDED_SOAK_SAMPLE_COUNT + 1;
    huge_samples.bounds.sample_interval_seconds = 1;
    huge_samples.bounds.deadline_unix_seconds = u64::MAX;
    assert!(build_runner_plan(&huge_samples)
        .expect_err("huge sample count must fail")
        .iter()
        .any(|violation| violation.code == "bounds"));
}

#[test]
fn evaluator_classifies_stale_missing_resource_restart_backoff_transport_and_recovery_failures() {
    let config = valid_config();
    let samples = vec![
        SoakSample {
            sequence: 0,
            observed_unix_seconds: 1_700_000_100,
            observability_cursor_unix_seconds: Some(1_700_000_000),
            resource_growth_percent: 99,
            restart_count: 2,
            backoff_seconds: 9,
            transport_error_count: 1,
            recovery_seconds: Some(99),
        },
        SoakSample {
            sequence: 1,
            observed_unix_seconds: 1_700_000_110,
            observability_cursor_unix_seconds: None,
            resource_growth_percent: 0,
            restart_count: 0,
            backoff_seconds: 0,
            transport_error_count: 0,
            recovery_seconds: None,
        },
    ];

    let evaluation = evaluate_soak(&config, &samples, &clean_cleanup());
    let codes = evaluation
        .violations
        .iter()
        .map(|violation| violation.code.as_str())
        .collect::<Vec<_>>();
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(codes.contains(&"missing_samples"));
    assert!(codes.contains(&"stale_observability"));
    assert!(codes.contains(&"missing_observability"));
    assert!(codes.contains(&"resource_growth"));
    assert!(codes.contains(&"restart_count"));
    assert!(codes.contains(&"backoff"));
    assert!(codes.contains(&"transport"));
    assert!(codes.contains(&"recovery"));
}

#[test]
fn evaluator_rejects_future_observability_cursor() {
    let config = valid_config();
    let mut samples = passing_samples();
    samples[0].observability_cursor_unix_seconds = Some(samples[0].observed_unix_seconds + 1);

    let evaluation = evaluate_soak(&config, &samples, &clean_cleanup());
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "future_observability"));
}

#[test]
fn evidence_receipt_binds_revision_binaries_platform_clocks_samples_faults_and_cleanup() {
    let config = valid_config();
    let faults = passing_faults();

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        passing_samples(),
        faults.clone(),
        clean_cleanup(),
    )
    .expect("evidence");

    assert_eq!(evidence.schema, SOAK_EVIDENCE_SCHEMA);
    assert_eq!(evidence.revision, config.revision);
    assert_eq!(evidence.binaries, config.binaries);
    assert_eq!(evidence.platform, config.platform);
    assert_eq!(evidence.clock.finished_unix_seconds, 1_700_000_060);
    assert_eq!(evidence.samples.len(), 6);
    assert_eq!(evidence.faults, faults);
    assert_eq!(evidence.cleanup, clean_cleanup());
    assert_eq!(evidence.evaluation.status, SoakStatus::Pass);
    assert_eq!(evidence.config_sha256.len(), 64);
}

#[test]
fn evidence_fails_closed_for_missing_unknown_duplicate_or_unrecovered_fault_receipts() {
    let config = valid_config();
    let faults = vec![
        FaultRecord {
            name: "transport-drop".to_owned(),
            injected_unix_seconds: 1_700_000_010,
            recovered_unix_seconds: None,
            notes: "missing recovery".to_owned(),
        },
        FaultRecord {
            name: "transport-drop".to_owned(),
            injected_unix_seconds: 1_700_000_010,
            recovered_unix_seconds: Some(1_700_000_011),
            notes: "duplicate".to_owned(),
        },
        FaultRecord {
            name: "unknown".to_owned(),
            injected_unix_seconds: 1_700_000_030,
            recovered_unix_seconds: Some(1_700_000_031),
            notes: "unknown".to_owned(),
        },
    ];

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        passing_samples(),
        faults,
        clean_cleanup(),
    )
    .expect("evidence with fail-closed evaluation");
    let fault_violations = evidence
        .evaluation
        .violations
        .iter()
        .filter(|violation| violation.code == "fault_evidence")
        .count();

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(fault_violations >= 4);
}

#[test]
fn cancellation_and_cleanup_fail_closed_without_receipt_or_zero_residue() {
    let config = valid_config();
    let mut residue = BTreeMap::new();
    residue.insert(".adl/runtime-v3/soak/instances".to_owned(), 1);
    let cleanup = CleanupOutcome {
        cancellation_requested: true,
        cancellation_receipt: None,
        residue,
    };

    let evaluation = evaluate_soak(&config, &passing_samples(), &cleanup);
    let codes = evaluation
        .violations
        .iter()
        .map(|violation| violation.code.as_str())
        .collect::<Vec<_>>();
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(codes.contains(&"cancellation"));
    assert!(codes.contains(&"cleanup_residue"));
}

#[test]
fn cleanup_missing_required_path_fails_closed() {
    let config = valid_config();
    let mut cleanup = clean_cleanup();
    cleanup.residue.remove(".adl/runtime-v3/soak/locks");

    let evaluation = evaluate_soak(&config, &passing_samples(), &cleanup);
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "cleanup_residue"));
}

#[test]
fn zero_sample_interval_fails_closed_without_panic() {
    let mut config = valid_config();
    config.bounds.sample_interval_seconds = 0;

    let evaluation = evaluate_soak(&config, &passing_samples(), &clean_cleanup());
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "bounds"));

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        passing_samples(),
        passing_faults(),
        clean_cleanup(),
    )
    .expect("invalid config is represented as fail-closed evidence");
    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
}

#[test]
fn non_divisible_duration_requires_ceil_aligned_sample_count() {
    let mut config = valid_config();
    config.bounds.duration_seconds = 61;

    let six_samples = passing_samples();
    let evaluation = evaluate_soak(&config, &six_samples, &clean_cleanup());
    assert_eq!(evaluation.status, SoakStatus::FailClosed);
    assert!(evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "missing_samples"));

    let plan = build_runner_plan(&config).expect("runner plan");
    assert_eq!(
        plan.actions
            .iter()
            .filter(|action| action.kind == RunnerActionKind::Sample)
            .count(),
        7
    );
}

#[test]
fn excessive_missing_sample_tolerance_fails_closed_even_with_zero_samples() {
    let mut config = valid_config();
    config.thresholds.max_missing_sample_count = 6;

    let violations = validate_config(&config).expect_err("excessive tolerance must be invalid");
    assert!(violations
        .iter()
        .any(|violation| violation.code == "thresholds"));

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        Vec::new(),
        passing_faults(),
        clean_cleanup(),
    )
    .expect("invalid tolerance is fail-closed evidence");
    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
}

#[test]
fn evidence_fails_closed_after_configured_deadline() {
    let mut config = valid_config();
    config.bounds.deadline_unix_seconds = 1_700_000_050;

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        passing_samples(),
        passing_faults(),
        clean_cleanup(),
    )
    .expect("deadline violation is fail-closed evidence");

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(evidence
        .evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "schedule"));
}

#[test]
fn evidence_allows_missing_sample_within_declared_tolerance() {
    let mut config = valid_config();
    config.thresholds.max_missing_sample_count = 1;
    let mut samples = passing_samples();
    samples.remove(3);

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        samples,
        passing_faults(),
        clean_cleanup(),
    )
    .expect("missing sample within tolerance is valid evidence");

    assert_eq!(evidence.evaluation.status, SoakStatus::Pass);
}

#[test]
fn evidence_rejects_extra_unscheduled_sample() {
    let config = valid_config();
    let mut samples = passing_samples();
    samples.push(SoakSample {
        sequence: 99,
        observed_unix_seconds: 1_700_000_099,
        observability_cursor_unix_seconds: Some(1_700_000_099),
        resource_growth_percent: 0,
        restart_count: 0,
        backoff_seconds: 0,
        transport_error_count: 0,
        recovery_seconds: None,
    });

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        samples,
        passing_faults(),
        clean_cleanup(),
    )
    .expect("extra sample is fail-closed evidence");

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(evidence
        .evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "schedule"));
}

#[test]
fn evidence_near_overflow_clock_fails_closed_without_panic() {
    let mut config = valid_config();
    config.bounds.deadline_unix_seconds = u64::MAX;
    let start = u64::MAX - 30;
    let mut samples = passing_samples();
    samples[0].observed_unix_seconds = start;
    let mut faults = passing_faults();
    faults[0].injected_unix_seconds = u64::MAX;

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: start,
            finished_unix_seconds: u64::MAX,
        },
        samples,
        faults,
        clean_cleanup(),
    )
    .expect("overflow schedule is fail-closed evidence");

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(evidence
        .evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "schedule"));
}

#[test]
fn evidence_fails_closed_for_short_clock_shifted_samples_and_shifted_faults() {
    let config = valid_config();
    let mut samples = passing_samples();
    samples[1].sequence = 42;
    samples[2].observed_unix_seconds += 1;
    let mut faults = passing_faults();
    faults[0].injected_unix_seconds += 1;

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_001,
        },
        samples,
        faults,
        clean_cleanup(),
    )
    .expect("timing errors are fail-closed evidence");
    let schedule_violations = evidence
        .evaluation
        .violations
        .iter()
        .filter(|violation| violation.code == "schedule")
        .count();

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(schedule_violations >= 4);
}

#[test]
fn evidence_fails_closed_when_fault_recovery_precedes_injection() {
    let config = valid_config();
    let mut faults = passing_faults();
    faults[0].recovered_unix_seconds = Some(faults[0].injected_unix_seconds - 1);

    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 1_700_000_000,
            finished_unix_seconds: 1_700_000_060,
        },
        passing_samples(),
        faults,
        clean_cleanup(),
    )
    .expect("fault timing errors are fail-closed evidence");

    assert_eq!(evidence.evaluation.status, SoakStatus::FailClosed);
    assert!(evidence
        .evaluation
        .violations
        .iter()
        .any(|violation| violation.code == "fault_evidence"));
}

#[test]
fn evidence_rejects_reversed_clocks_before_receipt_claim() {
    let config = valid_config();
    let violations = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: 2,
            finished_unix_seconds: 1,
        },
        passing_samples(),
        Vec::new(),
        clean_cleanup(),
    )
    .expect_err("reversed clocks fail closed");

    assert_eq!(violations[0].code, "clock");
}
