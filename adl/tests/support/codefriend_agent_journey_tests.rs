//! Actual original review/Store and native Journey over the paired polling path.
use crate as adl;
include!("codefriend_local_publication_case.rs");
use crate::codefriend::{
    agent::journey as relay, architecture::structure, governance::local as fitness,
};

fn prepared_job() -> (Case, relay::Job) {
    assert_eq!(
        env!("CODEFRIEND_BUILD_CLEAN"),
        "true",
        "clean native candidate required"
    );
    let now = crate::codefriend::agent::clock();
    let case = Case::with_format_at("none", 0, false, PublicationFormat::Markdown, now);
    *case.state.lock().unwrap() = Value::Null;
    job_for_case(case)
}

fn job_for_case(case: Case) -> (Case, relay::Job) {
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
    let request = relay::Request::Prepare {
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
    let job = relay::Job {
        binding: relay::Binding {
            schema: "codefriend.agent_journey_job.v1".into(),
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
    (case, job)
}

#[test]
fn paired_journey_prepare_replay_and_graph_reuse_original_review_without_posts() {
    let (case, mut job) = prepared_job();
    let root = case.temp.path().join("state/run-run1");
    let original = fs::read(root.join("work/review/run.json")).unwrap();
    assert_eq!(case.poll().unwrap(), Some("run1".into()));
    let first = case.journey_results.lock().unwrap()[0].clone();
    assert_ne!(
        first["manifest"]["stages"]["structure"]["status"],
        "pending"
    );
    assert_eq!(case.poll().unwrap(), Some("run1".into()));
    assert_eq!(case.journey_results.lock().unwrap()[1], first);
    job.binding.job_id = "graph1".into();
    job.request = relay::Request::Graph;
    job.binding.request_digest = job.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    case.poll().unwrap();
    let graph = case.journey_results.lock().unwrap()[2].clone();
    assert!(graph["payload"].is_object());
    assert_eq!(graph["checkpoint_sequence"], first["checkpoint_sequence"]);
    assert_eq!(
        fs::read(root.join("work/review/run.json")).unwrap(),
        original
    );
    assert_eq!(
        case.posts(),
        0,
        "Journey must never dispatch a model or publication POST"
    );
    fs::remove_file(case.temp.path().join("consent.json")).unwrap();
    assert!(case.poll().is_err());
    assert_eq!(case.journey_results.lock().unwrap().len(), 3);
}

#[test]
fn lost_journey_ack_resends_identical_result_without_repeating_effects() {
    let (case, _) = prepared_job();
    case.drop_journey_ack.store(true, Ordering::SeqCst);
    assert!(case.poll().is_err());
    let first = case.journey_results.lock().unwrap()[0].clone();
    assert_eq!(case.poll().unwrap(), Some("run1".into()));
    assert_eq!(case.journey_results.lock().unwrap()[1], first);
    assert_eq!(case.posts(), 0);
}

#[test]
fn reserved_but_unattempted_effect_cannot_be_reported_complete_or_reexecuted() {
    use crate::codefriend::architecture::impact;
    let (case, mut job) = prepared_job();
    case.poll().unwrap();
    let root = case.temp.path().join("state/run-run1");
    let graph: structure::StructureReport =
        serde_json::from_slice(&fs::read(root.join("journey/structure.json")).unwrap()).unwrap();
    job.binding.job_id = "impact1".into();
    job.request = relay::Request::Impact {
        changes: impact::ChangeSet {
            schema: impact::VERSION.into(),
            repository: graph.record.admission.packet.repository.clone(),
            revision: graph.record.run.revision.clone(),
            graph_digest: graph.digest.clone(),
            targets: vec![],
        }
        .into(),
    };
    job.binding.request_digest = job.request.digest().unwrap();
    // Simulate a crash after the create-only reservation and before native dispatch.
    private(
        &root.join("relay-reservations/effect-impact1.json"),
        &json!({"job_digest":hash(&job).unwrap()}),
    );
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    for _ in 0..2 {
        let error = case.poll().unwrap_err();
        assert!(
            error.to_string().contains("reserved_effect_unresolved"),
            "{error}"
        );
    }
    assert_eq!(case.journey_results.lock().unwrap().len(), 1);
    assert!(!root.join("journey/impact.json").exists());
    assert_eq!(case.posts(), 0);
}

#[test]
fn paired_continuations_and_native_artifacts_preserve_owner_and_order() {
    use crate::codefriend::architecture::{impact, rationale};
    use relay::verification::{verify_stage, VerificationContext};
    let (case, mut job) = prepared_job();
    case.poll().unwrap();
    let prepared = case.journey_results.lock().unwrap()[0].clone();
    assert_eq!(
        prepared["manifest"]["stages"]["structure"]["status"], "complete",
        "prepare: {prepared}"
    );
    assert_eq!(
        prepared["manifest"]["stages"]["fitness"]["status"], "complete",
        "prepare: {prepared}"
    );
    let root = case.temp.path().join("state/run-run1");
    let report: RunReport =
        serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
    let graph: structure::StructureReport =
        serde_json::from_slice(&fs::read(root.join("journey/structure.json")).unwrap()).unwrap();
    let original = fs::read(root.join("work/review/run.json")).unwrap();
    let requests = vec![
        relay::Request::Impact {
            changes: impact::ChangeSet {
                schema: impact::VERSION.into(),
                repository: graph.record.run.repository.clone(),
                revision: graph.record.run.revision.clone(),
                graph_digest: graph.digest.clone(),
                targets: vec![impact::ChangeTarget::Module(graph.nodes[0].module.clone())],
            }
            .into(),
        },
        relay::Request::Status,
        relay::Request::Artifact {
            artifact: relay::Artifact::Structure,
        },
        relay::Request::Artifact {
            artifact: relay::Artifact::Fitness,
        },
        relay::Request::Artifact {
            artifact: relay::Artifact::Impact,
        },
    ];
    for (index, request) in requests.into_iter().enumerate() {
        job.binding.job_id = format!("continuation{index}");
        job.request = request;
        job.binding.request_digest = job.request.digest().unwrap();
        *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
        case.poll()
            .unwrap_or_else(|error| panic!("request {index} {:?}: {error:#}", job.request));
        let result: relay::StageResult =
            serde_json::from_value(case.journey_results.lock().unwrap().last().unwrap().clone())
                .unwrap();
        let mut context = VerificationContext {
            schema: "codefriend.agent_journey_verifier_context.v1".into(),
            job: job.clone(),
            report: serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap(),
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
            previous: None,
            now: crate::codefriend::agent::clock(),
        };
        verify_stage(&result, &context, context.now).unwrap();
        context.previous = Some(result.clone());
        verify_stage(&result, &context, context.now).unwrap();
        let mut changed = result.clone();
        changed.checkpoint_sequence += 1;
        changed.digest.clear();
        changed.digest = hash(&changed).unwrap();
        // A future observation cannot be manufactured merely by retaining an older receipt.
        let previous = context.previous.as_mut().unwrap();
        previous.checkpoint_sequence = changed.checkpoint_sequence + 1;
        previous.digest.clear();
        previous.digest = hash(previous).unwrap();
        assert_eq!(
            verify_stage(&changed, &context, context.now)
                .unwrap_err()
                .to_string(),
            "agent_journey_verifier_regression"
        );
        if result.payload.is_some() {
            let mut changed = result.clone();
            changed.payload = None;
            changed.digest.clear();
            changed.digest = hash(&changed).unwrap();
            context.previous = None;
            assert!(verify_stage(&changed, &context, context.now).is_err());
        }
        assert_eq!(
            fs::read(root.join("work/review/run.json")).unwrap(),
            original
        );
    }
    job.binding.job_id = "rationale_missing_evidence".into();
    job.request = relay::Request::Rationale {
        selection: rationale::RationaleSelection {
            schema: rationale::VERSION.into(),
            graph_digest: graph.digest.clone(),
            revision: graph.record.run.revision.clone(),
            boundaries: vec![rationale::BoundarySelection {
                boundary: "core".into(),
                deployment_path: "compose.yaml".into(),
                service: "app".into(),
                rationale_paths: vec![],
            }],
        }
        .into(),
    };
    job.binding.request_digest = job.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    assert_eq!(case.poll().unwrap(), Some("run1".into()));
    let delivered = case.journey_results.lock().unwrap().last().unwrap().clone();
    assert_eq!(
        delivered["manifest"]["stages"]["rationale"]["status"],
        "failed"
    );
    assert_eq!(
        delivered["manifest"]["stages"]["rationale"]["reason"],
        "stage_incomplete_or_failed"
    );
    let rationale: rationale::RationaleReport =
        serde_json::from_slice(&fs::read(root.join("journey/rationale.json")).unwrap()).unwrap();
    assert!(!rationale.analysis_complete);
    assert!(!rationale.boundaries[0].unknowns.is_empty());
    assert_eq!(case.posts(), 0);
}

#[test]
fn paired_jobs_reject_foreign_authority_and_unavailable_baselines_without_dispatch() {
    let (case, original) = prepared_job();
    case.poll().unwrap();
    for (index, field) in [
        "subject",
        "agent_id",
        "consent_digest",
        "report_digest",
        "received_digest",
        "expires_at",
    ]
    .into_iter()
    .enumerate()
    {
        let mut job = serde_json::to_value(&original).unwrap();
        job["binding"]["job_id"] = json!(format!("foreign{index}"));
        job["binding"][field] = if field == "expires_at" {
            json!(1)
        } else {
            json!("other")
        };
        *case.journey.lock().unwrap() = job;
        assert!(case.poll().is_err(), "accepted {field}");
    }
    for (index, baseline) in ["run1", "absent"].into_iter().enumerate() {
        let mut job = original.clone();
        job.binding.job_id = format!("baseline{index}");
        job.request = relay::Request::Drift {
            baseline_run: baseline.into(),
        };
        job.binding.request_digest = job.request.digest().unwrap();
        *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
        assert!(case.poll().is_err());
    }
    assert_eq!(case.journey_results.lock().unwrap().len(), 1);
    assert_eq!(case.posts(), 0);
}

#[test]
fn paired_existing_publication_attaches_without_renderer_or_provider_replay() {
    let now = crate::codefriend::agent::clock();
    let case = Case::with_format_at("none", 0, false, PublicationFormat::Markdown, now);
    fs::remove_dir_all(
        case.temp
            .path()
            .join("state/run-run1/work/publications/job1"),
    )
    .unwrap();
    case.poll().unwrap();
    {
        let mut job = case.state.lock().unwrap();
        job["decision"] = json!({"challenge_digest":job["prepared"]["challenge_digest"],"binding_digest":job["prepared"]["binding_digest"],"expected_decision_digest":job["prepared"]["expected_decision_digest"],"decision":"approved"});
        job["status"] = json!("decision_pending");
    }
    case.poll().unwrap();
    assert_eq!(case.state.lock().unwrap()["status"], "complete");
    let before_posts = case.posts();
    let (case, mut job) = job_for_case(case);
    let consent = case.temp.path().join("consent.json");
    case.transport
        .poll_journey(&case.journal, &consent)
        .unwrap();
    job.binding.job_id = "attach1".into();
    job.request = relay::Request::AttachPublication {
        publication_job: "job1".into(),
        format: PublicationFormat::Markdown,
    };
    job.binding.request_digest = job.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    case.transport
        .poll_journey(&case.journal, &consent)
        .unwrap();
    let first = case.journey_results.lock().unwrap().last().unwrap().clone();
    assert_eq!(
        first["manifest"]["stages"]["markdown"]["status"],
        "complete"
    );
    case.transport
        .poll_journey(&case.journal, &consent)
        .unwrap();
    assert_eq!(*case.journey_results.lock().unwrap().last().unwrap(), first);
    assert_eq!(case.posts(), before_posts);
    case.state.lock().unwrap()["prepared_digest"] = json!("e".repeat(64));
    assert!(case
        .transport
        .poll_journey(&case.journal, &consent)
        .is_err());
    assert_eq!(case.posts(), before_posts);
}

#[test]
fn paired_drift_reopens_both_original_run_owners_and_denies_revoked_baseline() {
    let (case, mut current) = prepared_job();
    case.poll().unwrap();
    let now = crate::codefriend::agent::clock();
    let source_root = case.temp.path().join("state/run-run1");
    let report: RunReport =
        serde_json::from_slice(&fs::read(source_root.join("report.json")).unwrap()).unwrap();
    let mut consent: Consent =
        serde_json::from_slice(&fs::read(case.temp.path().join("consent.json")).unwrap()).unwrap();
    consent.expires_at = now + 100;
    let consent_path = case.temp.path().join("baseline-consent.json");
    private(&consent_path, &consent);
    let pairing = case.journal.pairing(now).unwrap();
    let command = Command {
        schema: PROTOCOL.into(),
        agent_id: pairing.agent_id.clone(),
        subject: pairing.subject.clone(),
        run_id: "run2".into(),
        consent_digest: consent.digest().unwrap(),
        expires_at: now + 80,
        cycle: None,
    };
    let root = case
        .journal
        .reserve(&command, &pairing, &consent, now)
        .unwrap();
    let store =
        crate::codefriend::evidence::store::Store::open(&root.join("evidence"), move || now)
            .unwrap();
    // A separate original admission and native run, even for a repeated source revision.
    let original = &report.result.as_ref().unwrap().review_record;
    let admission = store
        .admit(
            original.admission.packet.clone(),
            original.admission.retention.clone(),
        )
        .unwrap();
    drop(store);
    let result = runner::run_with_executor(
        ExecutionOptions {
            out: root.join("work/review"),
            run_id: "run2".into(),
            cancel_file: None,
        },
        admission.clone(),
        original.run.provider_route.clone(),
        |_, _, _| {
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"findings\":[]}".into()),
            })
        },
    )
    .unwrap();
    let mut second = RunReport {
        schema: PROTOCOL.into(),
        subject: pairing.subject,
        agent_id: pairing.agent_id,
        run_id: "run2".into(),
        consent_digest: command.consent_digest.clone(),
        execution_location: "local_agent".into(),
        gateway_lanes: report.gateway_lanes,
        status: "complete".into(),
        expires_at: admission.expires_at,
        result: Some(result),
        cycle_result: None,
        digest: String::new(),
    };
    second.digest = hash(&second).unwrap();
    second.validate(now).unwrap();
    private(&root.join("expires.json"), &second.expires_at);
    private(&root.join("report.json"), &second);
    private(
        &root.join("local-consent.json"),
        &json!({"path":consent_path.canonicalize().unwrap(),"digest":second.consent_digest}),
    );
    case.extra_receipts.lock().unwrap().insert("run2".into(), json!({"schema":"codefriend.agent_report_receipt.v1",
        "subject":second.subject,"agent_id":second.agent_id,"run_id":"run2","report_digest":second.digest,
        "received_digest":"e".repeat(64),"consent_digest":second.consent_digest,"expires_at":second.expires_at}));
    let mut baseline = current.clone();
    baseline.binding.job_id = "prepare_run2".into();
    baseline.binding.run_id = "run2".into();
    baseline.binding.report_digest = second.digest.clone();
    baseline.binding.consent_digest = second.consent_digest.clone();
    baseline.binding.received_digest = "e".repeat(64);
    baseline.binding.expires_at = second.expires_at;
    *case.journey.lock().unwrap() = serde_json::to_value(&baseline).unwrap();
    case.transport
        .poll_journey(&case.journal, &consent_path)
        .unwrap();
    current.binding.job_id = "drift_original_owners".into();
    current.request = relay::Request::Drift {
        baseline_run: "run2".into(),
    };
    current.binding.request_digest = current.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&current).unwrap();
    case.poll().unwrap();
    let drift = case.journey_results.lock().unwrap().last().unwrap().clone();
    assert_eq!(drift["manifest"]["stages"]["drift"]["status"], "complete");
    current.binding.job_id = "status_with_saved_baseline".into();
    current.request = relay::Request::Status;
    current.binding.request_digest = current.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&current).unwrap();
    case.poll().unwrap();
    let count = case.journey_results.lock().unwrap().len();
    fs::remove_file(&consent_path).unwrap();
    assert!(case.poll().is_err());
    assert_eq!(case.journey_results.lock().unwrap().len(), count);
    assert_eq!(case.posts(), 0);
}

// #1133: actual typed cycle/report shape, distinct gateway/local IDs and admissions.
fn cycle_job() -> (Case, relay::Job, crate::codefriend::cycle_bridge::Capsule) {
    let (case, job) = prepared_job();
    let (job, capsule) = install_cycle(&case, job);
    (case, job, capsule)
}

fn install_cycle(
    case: &Case,
    job: relay::Job,
) -> (relay::Job, crate::codefriend::cycle_bridge::Capsule) {
    install_cycle_version(case, job, false)
}

fn install_cycle_version(
    case: &Case,
    mut job: relay::Job,
    historical_v2: bool,
) -> (relay::Job, crate::codefriend::cycle_bridge::Capsule) {
    use crate::codefriend::{
        activities::{self, Activity, CycleExecutionBinding, UpdateCyclePlan},
        evidence::Retention,
    };
    let root = case
        .temp
        .path()
        .join(format!("state/run-{}", job.binding.run_id));
    let mut report: RunReport =
        serde_json::from_slice(&fs::read(root.join("report.json")).unwrap()).unwrap();
    let mut command: Command =
        serde_json::from_slice(&fs::read(root.join("command.json")).unwrap()).unwrap();
    let local = report
        .result
        .as_ref()
        .unwrap()
        .review_record
        .admission
        .clone();
    private(&root.join("admission.json"), &local);
    let now = crate::codefriend::agent::clock();
    let gateway = Admission::new(local.packet.clone(), Retention { seconds: 50 }, now).unwrap();
    let plan = UpdateCyclePlan {
        schema: activities::PLAN_SCHEMA.into(),
        repository: gateway.packet.repository.clone(),
        activities: vec![Activity::Review],
        testing: None,
    };
    let gateway_id = hash(&(
        PROTOCOL,
        &report.agent_id,
        &report.run_id,
        "cycle",
        hash(&plan).unwrap(),
    ))
    .unwrap();
    let model = report.gateway_lanes[0].model_identity.clone();
    let route = runner::provider_route_identity_from_model(&model);
    let producer = case
        .temp
        .path()
        .join(format!("gateway-producer-{}", report.run_id));
    let mut run = runner::run_assessments_with_executor(
        ExecutionOptions {
            out: producer.clone(),
            run_id: gateway_id.clone(),
            cancel_file: None,
        },
        gateway.clone(),
        route.clone(),
        |_, _, _| {
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"assessments\":[]}".into()),
            })
        },
    )
    .unwrap();
    if historical_v2 {
        // Reconstruct the old producer's exact input contract and all persisted
        // coupled bytes, rather than merely relabelling a ReviewRecord.
        for lane in crate::codefriend::review::lanes::ReviewLane::ALL {
            let (manifest, prompt) = runner::assessment_lane_input_manifest_version(
                &run.run_id,
                lane,
                &gateway,
                "codefriend.review_lane.v2",
            )
            .unwrap();
            assert!(prompt.contains("zero-based half-open UTF8 byte offsets in ORIGINAL content"));
            assert!(prompt.contains("\"start_byte\":0"));
            assert!(!prompt.contains("Do not calculate or return byte offsets"));
            run.review_record
                .run
                .lane_versions
                .insert(lane.id().into(), "codefriend.review_lane.v2".into());
            let result = run
                .lane_results
                .iter_mut()
                .find(|r| r.lane == lane.id())
                .unwrap();
            result.lane_contract = "codefriend.review_lane.v2".into();
            result.input_digest = manifest.input_digest.clone();
            private(
                &producer.join(format!("lanes/{}/input.json", lane.id())),
                &manifest,
            );
            private(
                &producer.join(format!("lanes/{}/result.json", lane.id())),
                result,
            );
        }
        run.review_record.run.refresh_identity(&gateway).unwrap();
        run.successful_execution().unwrap();
        private(&producer.join("run.json"), &run);
        private(&producer.join("review-record.json"), &run.review_record);
        runner::validate_complete_run(&run, &run.run_id, &gateway, &route).unwrap();
        let mut historical_report: RunReport =
            serde_json::from_slice(&serde_json::to_vec(&report).unwrap()).unwrap();
        historical_report.run_id = run.run_id.clone();
        historical_report.expires_at = gateway.expires_at;
        let mut local_review = run.clone();
        local_review
            .rebind_provider_route(&historical_report.gateway_lanes[0].route().unwrap())
            .unwrap();
        historical_report.result = Some(local_review);
        historical_report.digest.clear();
        historical_report.digest = hash(&historical_report).unwrap();
        historical_report.validate(now).unwrap();
        let bytes = serde_json::to_vec(&historical_report).unwrap();
        let decoded: RunReport = serde_json::from_slice(&bytes).unwrap();
        decoded.validate(now).unwrap();
        assert_eq!(serde_json::to_vec(&decoded).unwrap(), bytes);
    }
    let mut cycle = activities::run_with_executor(
        plan.clone(),
        gateway,
        gateway_id,
        route,
        Some(run),
        |_, _, _| panic!("review-only cycle has no activity dispatch"),
    )
    .unwrap();
    cycle.execution = Some(CycleExecutionBinding {
        candidate_revision: "c".repeat(40),
        request_digest: "d".repeat(64),
        model_identity: model.clone(),
    });
    let capsule =
        crate::codefriend::cycle_bridge::Capsule::capture(&producer, &cycle, now).unwrap();
    report.result = None;
    report.expires_at = local.expires_at.min(cycle.admission.expires_at);
    report.gateway_lanes = vec![GatewayLaneIdentity {
        lane: "cycle".into(),
        candidate_revision: "c".repeat(40),
        request_digest: Some("d".repeat(64)),
        model_identity: model,
    }];
    report.cycle_result = Some(cycle);
    report.digest.clear();
    report.digest = hash(&report).unwrap();
    report.validate(now).unwrap();
    private(&root.join("report.json"), &report);
    command.cycle = Some(plan);
    private(&root.join("command.json"), &command);
    fs::remove_dir_all(root.join("work/review")).unwrap();
    job.binding.report_digest = report.digest.clone();
    job.binding.expires_at = report.expires_at;
    case.extra_receipts.lock().unwrap().insert(report.run_id.clone(), json!({"schema":"codefriend.agent_report_receipt.v1","subject":report.subject,"agent_id":report.agent_id,"run_id":report.run_id,"report_digest":report.digest,"received_digest":job.binding.received_digest,"consent_digest":report.consent_digest,"expires_at":report.expires_at}));
    case.extra_receipts.lock().unwrap().insert(
        format!("capsule:{}", capsule.operation_id),
        serde_json::to_value(&capsule).unwrap(),
    );
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    (job, capsule)
}

#[test]
fn cycle_journey_imports_exact_producer_bytes_without_replaying_review() {
    let (case, mut job, capsule) = cycle_job();
    let root = case.temp.path().join("state/run-run1");
    let original = fs::read(root.join("report.json")).unwrap();
    case.poll().unwrap();
    for (name, bytes) in capsule.files {
        assert_eq!(
            fs::read(root.join("imported-cycle/review").join(name)).unwrap(),
            bytes.as_bytes()
        );
    }
    assert!(!root.join("work/review").exists());
    job.binding.job_id = "cycle-graph".into();
    job.request = relay::Request::Graph;
    job.binding.request_digest = job.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&job).unwrap();
    case.poll().unwrap();
    assert_eq!(fs::read(root.join("report.json")).unwrap(), original);
    assert_eq!(case.posts(), 0);
    let report: RunReport = serde_json::from_slice(&original).unwrap();
    let result: relay::StageResult =
        serde_json::from_value(case.journey_results.lock().unwrap().last().unwrap().clone())
            .unwrap();
    let receipt =
        serde_json::from_value(case.extra_receipts.lock().unwrap()["run1"].clone()).unwrap();
    let context = relay::verification::VerificationContext {
        schema: "codefriend.agent_journey_verifier_context.v1".into(),
        job,
        report,
        receipt,
        previous: None,
        now: crate::codefriend::agent::clock(),
    };
    relay::verification::verify_stage(&result, &context, context.now).unwrap();
    case.extra_receipts
        .lock()
        .unwrap()
        .retain(|key, _| !key.starts_with("capsule:"));
    assert!(
        case.poll().is_err(),
        "remote deletion must stop continuation"
    );
    assert!(
        !root.join("imported-cycle").exists(),
        "failed remote authority scrubs imports"
    );
}

#[test]
fn cycle_capsule_rejects_swapped_lanes_and_post_transfer_revocation() {
    let (case, _, mut capsule) = cycle_job();
    let report: RunReport = serde_json::from_slice(
        &fs::read(case.temp.path().join("state/run-run1/report.json")).unwrap(),
    )
    .unwrap();
    capsule.files.insert(
        "lanes/correctness/result.json".into(),
        capsule.files["lanes/security/result.json"].clone(),
    );
    capsule.digest.clear();
    capsule.digest = hash(&capsule).unwrap();
    assert!(capsule
        .validate(
            report.cycle_result.as_ref().unwrap(),
            crate::codefriend::agent::clock()
        )
        .is_err());
    case.extra_receipts
        .lock()
        .unwrap()
        .insert("revoke_after_capsule".into(), json!(true));
    assert!(case.poll().is_err());
    assert!(!case
        .temp
        .path()
        .join("state/run-run1/imported-cycle")
        .exists());
    assert!(case.journey_results.lock().unwrap().is_empty());
    assert_eq!(case.posts(), 0);
}

#[test]
fn cycle_publication_preserves_report_and_requires_live_gateway() {
    let (case, journey, _) = cycle_job();
    let root = case.temp.path().join("state/run-run1");
    let original = fs::read(root.join("report.json")).unwrap();
    fs::remove_dir_all(root.join("work/publications/job1")).unwrap();
    *case.journey.lock().unwrap() = Value::Null;
    *case.state.lock().unwrap() = json!({"binding":{
        "schema":"codefriend.agent_publication.v1","job_id":"job1","subject":journey.binding.subject,
        "agent_id":journey.binding.agent_id,"run_id":"run1","report_digest":journey.binding.report_digest,
        "received_digest":journey.binding.received_digest,"consent_digest":journey.binding.consent_digest,
        "format":"markdown","expires_at":journey.binding.expires_at},
        "status":"awaiting_agent","decision":null,"prepared_digest":null,"terminal_digest":null,"effect_unresolved":false,
        "agent_candidate_revision":null,"prepared":null,"terminal":null});
    case.poll().unwrap();
    {
        let mut job = case.state.lock().unwrap();
        job["decision"] = json!({"challenge_digest":job["prepared"]["challenge_digest"],"binding_digest":job["prepared"]["binding_digest"],"expected_decision_digest":job["prepared"]["expected_decision_digest"],"decision":"approved"});
        job["status"] = json!("decision_pending");
    }
    case.poll().unwrap();
    assert_eq!(case.state.lock().unwrap()["status"], "complete");
    assert_eq!(fs::read(root.join("report.json")).unwrap(), original);
    assert!(case
        .calls
        .lock()
        .unwrap()
        .iter()
        .all(|line| !line.starts_with("POST /v1/operations")));
    case.extra_receipts
        .lock()
        .unwrap()
        .retain(|key, _| !key.starts_with("capsule:"));
    assert!(case.poll().is_err());
    assert!(!root.join("imported-cycle").exists());
}

#[test]
fn cycle_import_rejects_expiry_after_transfer_without_retaining_payload() {
    let (case, _, _) = cycle_job();
    case.extra_receipts
        .lock()
        .unwrap()
        .insert("expire_after_capsule".into(), json!(true));
    assert!(case.poll().is_err());
    assert!(!case
        .temp
        .path()
        .join("state/run-run1/imported-cycle")
        .exists());
    assert!(case.journey_results.lock().unwrap().is_empty());
    assert_eq!(case.posts(), 0);
}

#[test]
fn cycle_drift_rechecks_two_imported_owners_and_denies_revoked_baseline() {
    let (case, mut current, _) = cycle_job();
    case.poll().unwrap();
    let now = crate::codefriend::agent::clock();
    let source_root = case.temp.path().join("state/run-run1");
    let report: RunReport =
        serde_json::from_slice(&fs::read(source_root.join("report.json")).unwrap()).unwrap();
    let mut consent: Consent =
        serde_json::from_slice(&fs::read(case.temp.path().join("consent.json")).unwrap()).unwrap();
    consent.expires_at = now + 100;
    let consent_path = case.temp.path().join("baseline-consent.json");
    private(&consent_path, &consent);
    let pairing = case.journal.pairing(now).unwrap();
    let command = Command {
        schema: PROTOCOL.into(),
        agent_id: pairing.agent_id.clone(),
        subject: pairing.subject.clone(),
        run_id: "run2".into(),
        consent_digest: consent.digest().unwrap(),
        expires_at: now + 80,
        cycle: None,
    };
    let root = case
        .journal
        .reserve(&command, &pairing, &consent, now)
        .unwrap();
    let store =
        crate::codefriend::evidence::store::Store::open(&root.join("evidence"), move || now)
            .unwrap();
    // A separate original admission and native run, even for a repeated source revision.
    let original = &report.completed_review().unwrap().review_record;
    let admission = store
        .admit(
            original.admission.packet.clone(),
            original.admission.retention.clone(),
        )
        .unwrap();
    drop(store);
    let result = runner::run_with_executor(
        ExecutionOptions {
            out: root.join("work/review"),
            run_id: "run2".into(),
            cancel_file: None,
        },
        admission.clone(),
        report.gateway_lanes[0].route().unwrap(),
        |_, _, _| {
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"findings\":[]}".into()),
            })
        },
    )
    .unwrap();
    let mut second = RunReport {
        schema: PROTOCOL.into(),
        subject: pairing.subject,
        agent_id: pairing.agent_id,
        run_id: "run2".into(),
        consent_digest: command.consent_digest.clone(),
        execution_location: "local_agent".into(),
        gateway_lanes: ReviewLane::ALL
            .iter()
            .map(|lane| GatewayLaneIdentity {
                lane: lane.id().into(),
                candidate_revision: report.gateway_lanes[0].candidate_revision.clone(),
                request_digest: None,
                model_identity: report.gateway_lanes[0].model_identity.clone(),
            })
            .collect(),
        status: "complete".into(),
        expires_at: admission.expires_at,
        result: Some(result),
        cycle_result: None,
        digest: String::new(),
    };
    second.digest = hash(&second).unwrap();
    second.validate(now).unwrap();
    private(&root.join("expires.json"), &second.expires_at);
    private(&root.join("report.json"), &second);
    private(
        &root.join("local-consent.json"),
        &json!({"path":consent_path.canonicalize().unwrap(),"digest":second.consent_digest}),
    );
    case.extra_receipts.lock().unwrap().insert("run2".into(), json!({"schema":"codefriend.agent_report_receipt.v1",
        "subject":second.subject,"agent_id":second.agent_id,"run_id":"run2","report_digest":second.digest,
        "received_digest":"e".repeat(64),"consent_digest":second.consent_digest,"expires_at":second.expires_at}));
    let mut baseline = current.clone();
    baseline.binding.job_id = "prepare_run2".into();
    baseline.binding.run_id = "run2".into();
    baseline.binding.report_digest = second.digest.clone();
    baseline.binding.consent_digest = second.consent_digest.clone();
    baseline.binding.received_digest = "e".repeat(64);
    baseline.binding.expires_at = second.expires_at;
    let (baseline, baseline_capsule) = install_cycle(&case, baseline);
    *case.journey.lock().unwrap() = serde_json::to_value(&baseline).unwrap();
    case.transport
        .poll_journey(&case.journal, &consent_path)
        .unwrap();
    current.binding.job_id = "drift_original_owners".into();
    current.request = relay::Request::Drift {
        baseline_run: "run2".into(),
    };
    current.binding.request_digest = current.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&current).unwrap();
    case.poll().unwrap();
    let drift = case.journey_results.lock().unwrap().last().unwrap().clone();
    assert_eq!(drift["manifest"]["stages"]["drift"]["status"], "complete");
    current.binding.job_id = "status_with_saved_baseline".into();
    current.request = relay::Request::Status;
    current.binding.request_digest = current.request.digest().unwrap();
    *case.journey.lock().unwrap() = serde_json::to_value(&current).unwrap();
    case.poll().unwrap();
    let count = case.journey_results.lock().unwrap().len();
    case.extra_receipts
        .lock()
        .unwrap()
        .remove(&format!("capsule:{}", baseline_capsule.operation_id));
    assert!(case.poll().is_err());
    assert_eq!(case.journey_results.lock().unwrap().len(), count);
    assert!(!root.join("imported-cycle").exists());
    assert!(source_root.join("imported-cycle").exists());
    assert_eq!(case.posts(), 0);
}

#[test]
fn cycle_import_shortens_cleanup_deadline_before_expired_report_rejection() {
    let (case, _, capsule) = cycle_job();
    let root = case.temp.path().join("state/run-run1");
    let original = fs::read(root.join("report.json")).unwrap();
    fs::write(
        root.join("expires.json"),
        (capsule.expires_at + 3600).to_string(),
    )
    .unwrap();
    case.poll().unwrap();
    let deadline: u64 =
        serde_json::from_slice(&fs::read(root.join("cycle-import-expires.json")).unwrap()).unwrap();
    assert!(deadline <= capsule.expires_at);
    assert_eq!(fs::read(root.join("report.json")).unwrap(), original);
    case.journal.expire(capsule.expires_at).unwrap();
    assert!(!root.join("imported-cycle").exists());
    assert!(!root.join("report.json").exists());
    assert_eq!(case.posts(), 0);
}

// PVF #1140: historical assessment-v2 report/capsule and installed Journey.
#[test]
fn historical_v2_cycle_capsule_and_report_preserve_original_prompt_contract() {
    let (case, job) = prepared_job();
    let (_, capsule) = install_cycle_version(&case, job, true);
    let root = case.temp.path().join("state/run-run1");
    let original = fs::read(root.join("report.json")).unwrap();
    let report: RunReport = serde_json::from_slice(&original).unwrap();
    let now = crate::codefriend::agent::clock();
    report.validate(now).unwrap();
    capsule
        .validate(report.cycle_result.as_ref().unwrap(), now)
        .unwrap();
    case.poll().unwrap();
    assert!(!case.journey_results.lock().unwrap().is_empty());
    assert_eq!(fs::read(root.join("report.json")).unwrap(), original);
    assert_eq!(case.posts(), 0);
}
