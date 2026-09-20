//! Pure verifier accepts actual paired native snapshots and rejects rewritten identities.
use crate as adl;
include!("codefriend_local_publication_case.rs");
use crate::codefriend::agent::journey::verification as verifier;
use crate::codefriend::{
    agent::journey as relay, architecture::structure, governance::local as fitness,
};

#[test]
fn actual_paired_snapshot_verification_binds_owner_candidate_and_native_stages() {
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
    for variant in 0..9 {
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
            _ => unreachable!(),
        }
        changed.digest.clear();
        changed.digest = hash(&changed).unwrap();
        assert!(
            verifier::verify_stage(&changed, &context, now).is_err(),
            "variant {variant}"
        );
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
        serde_json::from_value(case.journey_results.lock().unwrap()[1].clone()).unwrap();
    verifier::verify_stage(&graph, &context, now).unwrap();
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
    let mut later = graph.clone();
    later.checkpoint_sequence += 1;
    later.digest.clear();
    later.digest = hash(&later).unwrap();
    context.previous = Some(later);
    assert!(verifier::verify_stage(&graph, &context, now).is_err());
    assert_eq!(case.posts(), 0);
}
