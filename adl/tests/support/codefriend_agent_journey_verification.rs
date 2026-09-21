//! Pure verifier accepts actual paired native snapshots and rejects rewritten identities.
use crate as adl;
include!("codefriend_local_publication_case.rs");
use crate::codefriend::agent::journey::verification as verifier;
use crate::codefriend::{
    agent::journey as relay, architecture::structure, governance::local as fitness,
};

#[test]
fn actual_paired_snapshot_verification_binds_owner_candidate_and_native_stages() {
    actual_snapshot(false);
}

#[test]
fn actual_v2_paired_snapshot_preserves_version_and_replay() {
    actual_snapshot(true);
}

static OWNER_TESTS: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn actual_snapshot(v2: bool) {
    let _guard = OWNER_TESTS.lock().unwrap_or_else(|e| e.into_inner());
    assert_eq!(
        env!("CODEFRIEND_BUILD_CLEAN"),
        "true",
        "clean native candidate required"
    );
    let now = crate::codefriend::agent::clock();
    let case = Case::with_format_at("none", 0, false, PublicationFormat::Markdown, now);
    *case.state.lock().unwrap() = Value::Null;
    let root = case.temp.path().join("state/run-run1");
    let report: RunReport =
        serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
    private(
        &root.join("local-consent.json"),
        &json!({
            "path":case.temp.path().join("consent.json").canonicalize().unwrap(),
            "digest":report.consent_digest
        }),
    );
    let legacy_request = relay::Request::Prepare {
        boundary_policy: structure::BoundaryPolicy {
            schema: structure::VERSION.into(),
            crate_root: "lib.rs".into(),
            manifest_path: None,
            layers: [("lib.rs".into(), "core".into())].into(),
            allowed: Default::default(),
            coupling_threshold: 2,
        }
        .into(),
        fitness_policy: fitness::Policy {
            schema: fitness::VERSION.into(),
            rules: vec![fitness::Rule {
                id: "no_network".into(),
                kind: "forbidden_declared_use".into(),
                source_path: "lib.rs".into(),
                forbidden_prefix: "reqwest".into(),
            }],
        }
        .into(),
    };
    let request = if v2 {
        use crate::codefriend::{
            architecture::structure_v2,
            governance::language as fitness_v2,
            language::{AnalysisPolicy, Language, Limits, ProjectRoot},
        };
        let analysis = AnalysisPolicy {
            schema: crate::codefriend::language::VERSION.into(),
            files: [("lib.rs".into(), Language::Rust)].into(),
            roots: vec![ProjectRoot {
                language: Language::Rust,
                root: ".".into(),
                manifest: None,
            }],
            layers: [("lib.rs".into(), "core".into())].into(),
            allowed: Default::default(),
            limits: Limits {
                max_nodes: 10000,
                max_depth: 128,
                max_facts: 1000,
                max_output_bytes: 1024 * 1024,
            },
        };
        relay::Request::Prepare {
            boundary_policy: structure_v2::BoundaryPolicyV2 {
                schema: structure_v2::VERSION.into(),
                analysis: analysis.clone(),
                coupling_threshold: 2,
            }
            .into(),
            fitness_policy: fitness_v2::Policy {
                schema: fitness_v2::VERSION.into(),
                analysis,
                rules: vec![fitness_v2::Rule::ForbiddenStaticImport {
                    id: "no_network".into(),
                    source_path: "lib.rs".into(),
                    selector: fitness_v2::StaticSelector::RustUse {
                        prefix: vec!["reqwest".into()],
                    },
                }],
            }
            .into(),
        }
    } else {
        legacy_request
    };
    let job = relay::Job {
        binding: relay::Binding {
            schema: if v2 {
                "codefriend.agent_journey_job.v2"
            } else {
                "codefriend.agent_journey_job.v1"
            }
            .into(),
            job_id: "prepare1".into(),
            subject: report.subject.clone(),
            agent_id: report.agent_id.clone(),
            run_id: report.run_id.clone(),
            consent_digest: report.consent_digest.clone(),
            report_digest: report.digest.clone(),
            received_digest: "d".repeat(64),
            request_digest: request.digest().unwrap(),
            expires_at: report.expires_at,
        },
        request,
        permitted_agent_candidate: env!("CODEFRIEND_BUILD_REVISION").into(),
    };
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    case.poll().unwrap();
    let result: relay::StageResult =
        serde_json::from_value(case.journey_results.lock().unwrap()[0].clone()).unwrap();
    let mut context = verifier::VerificationContext {
        schema: "codefriend.agent_journey_verifier_context.v1".into(),
        job,
        receipt: crate::codefriend::agent::ForwardReceipt {
            schema: "codefriend.agent_report_receipt.v1".into(),
            subject: report.subject.clone(),
            agent_id: report.agent_id.clone(),
            run_id: report.run_id.clone(),
            report_digest: report.digest.clone(),
            received_digest: "d".repeat(64),
            consent_digest: report.consent_digest.clone(),
            expires_at: report.expires_at,
        },
        report,
        previous: None,
        now,
    };
    verifier::verify_stage(&result, &context, now).unwrap();
    assert_eq!(
        result.schema,
        if v2 {
            "codefriend.agent_journey_result.v2"
        } else {
            "codefriend.agent_journey_result.v1"
        }
    );
    assert_eq!(
        result.manifest.schema,
        if v2 {
            "codefriend.journey.v2"
        } else {
            "codefriend.journey.v1"
        }
    );
    // Redelivery observes the same committed native effect and forwards identical bytes.
    case.poll().unwrap();
    assert_eq!(
        case.journey_results.lock().unwrap().last().unwrap(),
        &serde_json::to_value(&result).unwrap()
    );

    for variant in 0..11 {
        let mut changed = result.clone();
        match variant {
            0 => changed.manifest.revision = "f".repeat(40),
            1 => changed.binding.received_digest = "f".repeat(64),
            2 => {
                changed.manifest.stages.remove("structure");
            }
            3 => changed.manifest.stages.get_mut("review").unwrap().digest = Some("f".repeat(64)),
            4 => changed.agent_candidate_revision = "f".repeat(40),
            5 => changed.payload = Some(json!({"unrequested":"artifact"})),
            6 => {
                changed.manifest.stages.get_mut("impact").unwrap().artifact =
                    Some("../../escape".into())
            }
            7 => changed.checkpoint_sequence = 0,
            8 => {
                changed.manifest.status =
                    crate::codefriend::integration::journey::StageStatus::Complete
            }
            9 => {
                changed.manifest.stages.get_mut("review").unwrap().reason =
                    Some("analysis_gaps_reported".into())
            }
            10 => {
                changed.manifest.stages.get_mut("structure").unwrap().reason =
                    Some("unrecognized_gap".into())
            }
            _ => unreachable!(),
        }
        changed.digest.clear();
        changed.digest = hash(&changed).unwrap();
        assert!(
            verifier::verify_stage(&changed, &context, now).is_err(),
            "variant {variant}"
        );
    }
    if !v2 {
        let mut legacy_gap = result.clone();
        legacy_gap
            .manifest
            .stages
            .get_mut("structure")
            .unwrap()
            .reason = Some("analysis_gaps_reported".into());
        legacy_gap.digest.clear();
        legacy_gap.digest = hash(&legacy_gap).unwrap();
        assert!(verifier::verify_stage(&legacy_gap, &context, now).is_err());
    }
    context.receipt.agent_id = "another-agent".into();
    assert!(verifier::verify_stage(&result, &context, now).is_err());
    context.receipt.agent_id = context.report.agent_id.clone();
    assert!(verifier::verify_stage(&result, &context, context.report.expires_at).is_err());
    context.previous = Some(result.clone());
    verifier::verify_stage(&result, &context, now).unwrap();
    context.previous = None;
    context.job.binding.job_id = "graph1".into();
    context.job.request = relay::Request::Graph;
    context.job.binding.request_digest = context.job.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&context.job).unwrap();
    case.poll().unwrap();
    let graph: relay::StageResult =
        serde_json::from_value(case.journey_results.lock().unwrap().last().unwrap().clone())
            .unwrap();
    verifier::verify_stage(&graph, &context, now).unwrap();
    if v2 {
        assert_eq!(
            graph.payload.as_ref().unwrap()["schema"],
            "codefriend.structure.v2"
        );
        for failure in ["failed", "cancelled", "withheld"] {
            let mut failed = graph.clone();
            failed.payload.as_mut().unwrap()["record"]["run"]["completion"] = json!(failure);
            failed.manifest.stages.get_mut("structure").unwrap().digest = Some(
                super::super::artifact_digest(
                    &relay::Artifact::Structure,
                    failed.payload.as_ref().unwrap(),
                )
                .unwrap(),
            );
            failed.digest.clear();
            failed.digest = hash(&failed).unwrap();
            let error = verifier::verify_stage(&failed, &context, now).unwrap_err();
            assert_eq!(error.to_string(), "agent_journey_payload_outcome");
        }
        let mut inconsistent = graph.clone();
        inconsistent.payload.as_mut().unwrap()["analysis_complete"] = json!(true);
        inconsistent
            .manifest
            .stages
            .get_mut("structure")
            .unwrap()
            .digest = Some(
            super::super::artifact_digest(
                &relay::Artifact::Structure,
                inconsistent.payload.as_ref().unwrap(),
            )
            .unwrap(),
        );
        inconsistent.digest.clear();
        inconsistent.digest = hash(&inconsistent).unwrap();
        assert_eq!(
            verifier::verify_stage(&inconsistent, &context, now)
                .unwrap_err()
                .to_string(),
            "agent_journey_payload_completeness"
        );
        let mut erased = graph.clone();
        erased.manifest.stages.get_mut("structure").unwrap().reason = None;
        erased.digest.clear();
        erased.digest = hash(&erased).unwrap();
        assert!(verifier::verify_stage(&erased, &context, now).is_err());

        let mut downgrade = graph.clone();
        downgrade.schema = "codefriend.agent_journey_result.v1".into();
        downgrade.digest.clear();
        downgrade.digest = hash(&downgrade).unwrap();
        assert!(verifier::verify_stage(&downgrade, &context, now).is_err());
        context.job.binding.schema = "codefriend.agent_journey_job.v1".into();
        assert!(verifier::verify_stage(&graph, &context, now).is_err());
        context.job.binding.schema = "codefriend.agent_journey_job.v2".into();
    }

    let mut changed = graph.clone();
    changed.payload.as_mut().unwrap()["record"]["admission"]["digest"] = json!("f".repeat(64));
    changed.manifest.stages.get_mut("structure").unwrap().digest = Some(
        super::super::artifact_digest(
            &relay::Artifact::Structure,
            changed.payload.as_ref().unwrap(),
        )
        .unwrap(),
    );
    changed.digest.clear();
    changed.digest = hash(&changed).unwrap();
    assert!(verifier::verify_stage(&changed, &context, now).is_err());
    // Transport-only projection derived from original native graph references;
    // this exercises schema pairing, not production of a drift comparison.
    let record: adl::codefriend::evidence::contracts::ReviewRecord =
        serde_json::from_value(graph.payload.as_ref().unwrap()["record"].clone()).unwrap();
    let reference = adl::codefriend::memory::baseline::BaselineRef::from_record(&record).unwrap();
    let delta = adl::codefriend::memory::comparison::DeltaReport {
        schema: "codefriend.delta.v1".into(),
        baseline: reference.clone(),
        current: reference,
        comparable: true,
        reasons: vec![],
        changes: vec![],
        digest: "a".repeat(64),
    };
    let projection = json!({"schema":if v2 {"codefriend.owned_drift.v2"}else{"codefriend.owned_drift.v1"}, "original_report_digest":"a".repeat(64), "expires_at":context.report.expires_at, "graph_comparison":delta, "structural_comparison":delta, "current_traces":[]});
    let original_request = context.job.request.clone();
    context.job.request = relay::Request::Artifact {
        artifact: relay::Artifact::Drift,
    };
    context.job.binding.request_digest = context.job.request.digest().unwrap();
    let mut drift = graph.clone();
    drift.binding = context.job.binding.clone();
    drift.payload = Some(projection);
    let stage = drift.manifest.stages.get_mut("drift").unwrap();
    stage.status = adl::codefriend::integration::journey::StageStatus::Complete;
    stage.reason = None;
    stage.artifact = Some("drift.json".into());
    stage.digest = Some(
        super::super::artifact_digest(&relay::Artifact::Drift, drift.payload.as_ref().unwrap())
            .unwrap(),
    );
    drift.digest.clear();
    drift.digest = hash(&drift).unwrap();
    verifier::verify_stage(&drift, &context, now).unwrap();
    drift.payload.as_mut().unwrap()["schema"] = json!(if v2 {
        "codefriend.owned_drift.v1"
    } else {
        "codefriend.owned_drift.v2"
    });
    drift.manifest.stages.get_mut("drift").unwrap().digest = Some(
        super::super::artifact_digest(&relay::Artifact::Drift, drift.payload.as_ref().unwrap())
            .unwrap(),
    );
    drift.digest.clear();
    drift.digest = hash(&drift).unwrap();
    assert_eq!(
        verifier::verify_stage(&drift, &context, now)
            .unwrap_err()
            .to_string(),
        "agent_journey_payload_version"
    );
    context.job.request = original_request;
    context.job.binding.request_digest = context.job.request.digest().unwrap();
    let mut later = graph.clone();
    later.checkpoint_sequence += 1;
    later.digest.clear();
    later.digest = hash(&later).unwrap();
    context.previous = Some(later);
    assert!(verifier::verify_stage(&graph, &context, now).is_err());
    assert_eq!(case.posts(), 0);
}
