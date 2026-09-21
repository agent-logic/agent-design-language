//! Component proof: real local publication transport/native owners, synthetic model output.
use crate as adl;
use crate::codefriend::agent::publication::{verify_stage, VerificationContext};
include!("codefriend_local_publication_case.rs");
use crate::codefriend::integration::journey::{self, OwnedAdmissionJourneyOptions};

fn completed(format: PublicationFormat, decision: &str) -> (Case, VerificationContext) {
    assert_eq!(
        env!("CODEFRIEND_BUILD_CLEAN"),
        "true",
        "native authority requires a clean committed candidate"
    );
    let timestamp = super::now();
    let case = Case::with_format_at("none", 0, false, format, timestamp);
    let run = case.temp.path().join("state/run-run1");
    fs::remove_dir_all(run.join("work/publications/job1")).unwrap();
    case.poll().unwrap();
    {
        let mut job = case.state.lock().unwrap();
        job["decision"] = json!({"challenge_digest":job["prepared"]["challenge_digest"],"binding_digest":job["prepared"]["binding_digest"],"expected_decision_digest":job["prepared"]["expected_decision_digest"],"decision":decision});
        job["status"] = json!("decision_pending");
    }
    case.poll().unwrap();
    let terminal: Stage = serde_json::from_slice(
        &fs::read(run.join("work/publications/job1/terminal-stage.json")).unwrap(),
    )
    .unwrap();
    let context = VerificationContext {
        schema: "codefriend.agent_publication_verifier_context.v1".into(),
        binding: terminal.binding.clone(),
        report: serde_json::from_slice(&fs::read(run.join("report.json")).unwrap()).unwrap(),
        decision: Some(
            serde_json::from_value(case.state.lock().unwrap()["decision"].clone()).unwrap(),
        ),
        prepared: Some(
            serde_json::from_slice(
                &fs::read(run.join("work/publications/job1/prepared-stage.json")).unwrap(),
            )
            .unwrap(),
        ),
        now: timestamp,
    };
    verify_stage(&terminal, &context, super::now()).unwrap();
    (case, context)
}
fn owned(case: &Case, context: &VerificationContext) -> journey::Journey {
    use crate::codefriend::{architecture::structure, governance::local as fitness};
    let root = case.temp.path().join("state/run-run1");
    let run = context.report.result.as_ref().unwrap();
    journey::prepare_owned_admission(OwnedAdmissionJourneyOptions {
        store: root.join("evidence"),
        output: root.join("journey"),
        owner_root: root.clone(),
        review_root: root.join("work/review"),
        packet_id: run.review_record.admission.packet.packet_id.clone(),
        admission_digest: run.review_record.admission.digest.clone(),
        operation_id: run.run_id.clone(),
        candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
        expires_at: context.binding.expires_at,
        completed_run: run.clone(),
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
    })
    .unwrap()
}
#[test]
fn local_native_exports_attach_atomically_and_resume_without_dispatch() {
    for format in [PublicationFormat::Markdown, PublicationFormat::Html] {
        let (case, context) = completed(format, "approved");
        let mut journey = owned(&case, &context);
        let out = journey.output().to_owned();
        let posts = case.posts();
        let before = journey.checkpoint_sequence();
        journey.attach_local_publication(&context).unwrap();
        assert_eq!(journey.checkpoint_sequence(), before + 1);
        for name in [
            format!("publication_{}", format.key()),
            format!("approval_{}", format.key()),
            format.key().into(),
        ] {
            assert_eq!(
                journey.manifest().stages[&name].status,
                journey::StageStatus::Complete
            );
        }
        journey.attach_local_publication(&context).unwrap();
        assert_eq!(journey.checkpoint_sequence(), before + 1);
        drop(journey);
        let resumed = journey::resume(&out).unwrap();
        assert_eq!(resumed.checkpoint_sequence(), before + 1);
        drop(resumed);
        assert_eq!(
            case.posts(),
            posts,
            "attachment and restart may not dispatch publication effects"
        );
        // Real Markdown/HTML retained bytes, never altered PDF payloads.
        let name = if format == PublicationFormat::Markdown {
            "report.md"
        } else {
            "report.html"
        };
        let path = case
            .temp
            .path()
            .join("state/run-run1/work/publications/job1/exports")
            .join(format.target())
            .join(name);
        let original = fs::read(&path).unwrap();
        fs::write(&path, b"changed").unwrap();
        assert!(journey::resume(&out).is_err());
        fs::write(&path, original).unwrap();
        assert!(journey::resume(&out).is_ok());
    }
}
#[test]
fn local_attachment_rejects_withheld_and_changed_job_identity() {
    let (case, context) = completed(PublicationFormat::Markdown, "withheld");
    let mut journey = owned(&case, &context);
    let sequence = journey.checkpoint_sequence();
    assert!(journey.attach_local_publication(&context).is_err());
    assert_eq!(journey.checkpoint_sequence(), sequence);
    let (case, mut context) = completed(PublicationFormat::Markdown, "approved");
    let mut journey = owned(&case, &context);
    let sequence = journey.checkpoint_sequence();
    context.binding.agent_id = "another-agent".into();
    assert!(journey.attach_local_publication(&context).is_err());
    assert_eq!(journey.checkpoint_sequence(), sequence);
    context.binding.agent_id = "agent1".into();
    context.binding.job_id = "../job1".into();
    assert!(journey.attach_local_publication(&context).is_err());
    assert_eq!(journey.checkpoint_sequence(), sequence);
}
