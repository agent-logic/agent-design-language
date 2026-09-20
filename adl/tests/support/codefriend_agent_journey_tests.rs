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
        },
        fitness_policy: fitness::Policy {
            schema: fitness::VERSION.into(),
            rules: vec![fitness::Rule {
                id: "no_network".into(),
                kind: "forbidden_declared_use".into(),
                source_path: "lib.rs".into(),
                forbidden_prefix: "reqwest".into(),
            }],
        },
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
        },
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
