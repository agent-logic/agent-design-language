//! Actual original review/Store and native Journey over the paired polling path.
use crate as adl;
include!("codefriend_local_publication_case.rs");
use crate::codefriend::{
    agent::journey as relay, architecture::structure, governance::local as fitness,
};

#[test]
fn paired_journey_prepare_replay_and_graph_reuse_original_review_without_posts() {
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
    let mut job = relay::Job {
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
