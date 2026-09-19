//! PVF runtime; deterministic local challenge binding, supporting Beta proof.
//! No authentication, provider, installed-binary or complete journey claim.
use adl::codefriend::{
    evidence::contracts::ReviewRecord, ingestion::digest, integration::PublicationChallenge,
    publication::ManifestInput,
};
use serde_json::json;
use std::fs;

#[test]
fn challenge_binds_identity_artifacts_destination_and_decision_head() {
    let tmp = local_temp();
    let artifacts = tmp.path().join("artifacts");
    let destination = tmp.path().join("destination");
    fs::create_dir(&artifacts).unwrap();
    fs::create_dir(&destination).unwrap();
    fs::write(artifacts.join("report.md"), "Bounded report\n").unwrap();
    let review: ReviewRecord = serde_json::from_slice(include_bytes!(
        "fixtures/codefriend/evidence/review-v1.json"
    ))
    .unwrap();
    let manifest: ManifestInput = serde_json::from_value(json!({
        "schema":"codefriend.publication_manifest_input.v1",
        "artifact_manifest":[{"path":"report.md","digest":digest(b"Bounded report\n")}],
        "renderer_versions":{"markdown":"v1"},"target":"review-output",
        "claims":["Bounded local review"],"nonclaims":["No remote publication"]
    }))
    .unwrap();
    let publication = manifest.publication(&review, &destination).unwrap();
    let challenge = PublicationChallenge::prepare(
        "user1",
        "run1",
        &"a".repeat(40),
        &review,
        &publication,
        &artifacts,
        None,
        100,
        200,
    )
    .unwrap();
    let binding = publication.binding_digest().unwrap();
    let verify = |subject, operation, id, binding: &str, head, candidate, time| {
        challenge.verify_response(
            subject,
            operation,
            &"a".repeat(40),
            id,
            binding,
            head,
            &review,
            candidate,
            &artifacts,
            time,
        )
    };
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        None,
        &publication,
        150
    )
    .is_ok());
    for (subject, operation) in [("user2", "run1"), ("user1", "run2")] {
        assert!(verify(
            subject,
            operation,
            challenge.digest(),
            &binding,
            None,
            &publication,
            150
        )
        .is_err());
    }
    for time in [99, 200, 201] {
        assert!(verify(
            "user1",
            "run1",
            challenge.digest(),
            &binding,
            None,
            &publication,
            time
        )
        .is_err());
    }
    assert!(verify("user1", "run1", "forged", &binding, None, &publication, 150).is_err());
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        "review-json-digest",
        None,
        &publication,
        150
    )
    .is_err());
    let head = "a".repeat(64);
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        Some(head.as_str()),
        &publication,
        150
    )
    .is_err());
    let mut changed = publication.clone();
    changed.claims.push("Additional claim".into());
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        None,
        &changed,
        150
    )
    .is_err());
    let mut changed = publication.clone();
    changed
        .renderer_versions
        .insert("markdown".into(), "v2".into());
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        None,
        &changed,
        150
    )
    .is_err());
    let mut changed = publication.clone();
    changed.target = "other-output".into();
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        None,
        &changed,
        150
    )
    .is_err());
    fs::write(artifacts.join("report.md"), "Replaced artifact\n").unwrap();
    assert!(verify(
        "user1",
        "run1",
        challenge.digest(),
        &binding,
        None,
        &publication,
        150
    )
    .is_err());
    assert!(!destination.join("review-output").exists());
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
}

#[test]
fn retained_review_builds_verified_publication_bundle_without_approval() {
    use adl::codefriend::integration::prepare_publication_bundle;
    use adl::codefriend::publication::verify_artifacts;
    let temp = local_temp();
    let review = temp.path().join("review.json");
    fs::write(&review, serde_json::to_vec(&complete_review()).unwrap()).unwrap();
    let destination = temp.path().join("destination");
    fs::create_dir(&destination).unwrap();
    let output = temp.path().join("bundle");
    let publication = prepare_publication_bundle(&review, &output, &destination).unwrap();
    assert_eq!(publication.artifact_manifest.len(), 13);
    assert!(publication.approval.is_none());
    verify_artifacts(&output.join("artifacts"), &publication.artifact_manifest).unwrap();
    assert!(prepare_publication_bundle(&review, &output, &destination).is_err());
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
}

fn local_temp() -> tempfile::TempDir {
    let root = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"));
    fs::create_dir_all(root).unwrap();
    tempfile::tempdir_in(root.canonicalize().unwrap()).unwrap()
}

fn complete_review() -> ReviewRecord {
    use adl::codefriend::{
        evidence::contracts::{Completion, Run},
        review::lanes::LANE_CONTRACT_VERSION,
    };
    let mut review: ReviewRecord = serde_json::from_slice(include_bytes!(
        "fixtures/codefriend/evidence/review-v1.json"
    ))
    .unwrap();
    review.run = Run::new(
        &review.admission,
        ["adversarial", "constitutional", "correctness", "security"]
            .into_iter()
            .map(|lane| (lane.into(), LANE_CONTRACT_VERSION.into()))
            .collect(),
        "fixture:mock:reviewer".into(),
        Completion::Complete,
        vec![],
    )
    .unwrap();
    review.validate().unwrap();
    review
}

#[tokio::test]
async fn website_approval_authenticates_rejects_stale_binding_and_expires_payloads() {
    use adl::codefriend::{
        evidence::{hash, Admission},
        publication::{append_decision, read_decision_head, DecisionKind},
        review::runner::FourPerspectiveReviewRun,
        server::*,
    };
    use std::{
        path::Path,
        sync::Arc,
        time::{SystemTime, UNIX_EPOCH},
    };
    struct NoProvider;
    impl Backend for NoProvider {
        fn execute(
            &self,
            _: &Config,
            _: &Submit,
            _: Admission,
            _: &Path,
        ) -> anyhow::Result<serde_json::Value> {
            panic!("approval test must never dispatch a provider")
        }
    }
    let temp = local_temp();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let alice = "alice-approval-fixture-token-01234567890123";
    let bob = "bob-approval-fixture-token-0123456789012345";
    let credential = |token: &str, subject: &str| Credential {
        token_hash: blake3::hash(token.as_bytes()).to_hex().to_string(),
        subject: subject.into(),
        mode: Mode::Hosted,
        expires_at: now + 3600,
    };
    let credentials = temp.path().join("credentials.json");
    fs::write(
        &credentials,
        serde_json::to_vec(&vec![credential(alice, "alice"), credential(bob, "bob")]).unwrap(),
    )
    .unwrap();
    let root = temp.path().join("state");
    let service = Service::open(
        Config {
            root: root.clone(),
            credentials_file: credentials.clone(),
            provider: provider_request(),
            candidate_revision: build_revision().into(),
            max_concurrent: 1,
            max_operations_per_subject: 4,
            retention_seconds: 3600,
        },
        Arc::new(NoProvider),
    )
    .unwrap();
    let app = service.clone().router();
    let review = complete_review();
    let operation = root.join("operations/alice/run1");
    fs::create_dir_all(operation.join("work/review")).unwrap();
    fs::write(
        operation.join("work/review/review-record.json"),
        serde_json::to_vec(&review).unwrap(),
    )
    .unwrap();
    let mut op = Operation {
        operation_id: "run1".into(),
        subject: "alice".into(),
        mode: Mode::Hosted,
        request_digest: hash(&review).unwrap(),
        packet_id: review.run.packet_id.clone(),
        source_revision: review.run.revision.clone(),
        candidate_revision: build_revision().into(),
        model_identity: None,
        expires_at: now + 3600,
        status: Status::Complete,
    };
    fs::write(
        operation.join("operation.json"),
        serde_json::to_vec(&op).unwrap(),
    )
    .unwrap();
    fs::write(
        operation.join("result.json"),
        serde_json::to_vec(&FourPerspectiveReviewRun {
            schema: adl::codefriend::review::runner::REVIEW_RUN_SCHEMA.into(),
            run_id: "run1".into(),
            completion: review.run.completion.clone(),
            review_record: review.clone(),
            lane_results: vec![],
            failures: vec![],
        })
        .unwrap(),
    )
    .unwrap();
    let prepare = "/v1/operations/run1/publication/challenge";
    let decide = "/v1/operations/run1/publication/decision";
    assert_eq!(
        http_call(&app, "POST", prepare, None, json!(null)).await.0,
        401
    );
    assert_eq!(
        http_call(&app, "POST", prepare, Some(bob), json!({}))
            .await
            .0,
        404
    );
    assert!(!operation
        .join("work/publications/markdown/approval")
        .exists());
    let (status, challenge) = http_call(&app, "POST", prepare, Some(alice), json!({})).await;
    assert_eq!(status, 200, "{challenge}");
    let request = json!({"challenge_digest":challenge["challenge_digest"],
        "binding_digest":challenge["binding_digest"],"expected_decision_digest":challenge["expected_decision_digest"],"decision":"approved"});
    let mut wrong = request.clone();
    wrong["binding_digest"] = json!("forged");
    assert_eq!(
        http_call(&app, "POST", decide, Some(alice), wrong).await.0,
        409
    );
    let publication = challenge["challenge"]["publication"].clone();
    let publication: adl::codefriend::evidence::contracts::Publication =
        serde_json::from_value(publication).unwrap();
    let store = operation.join("work/publications/markdown/approval");
    assert!(read_decision_head(&store, &review, &publication)
        .unwrap()
        .is_none());
    let mut forged_actor = request.clone();
    forged_actor["actor"] = json!("bob");
    assert_eq!(
        http_call(&app, "POST", decide, Some(alice), forged_actor)
            .await
            .0,
        422
    );
    let (status, approved) = http_call(&app, "POST", decide, Some(alice), request.clone()).await;
    assert_eq!(status, 200, "{approved}");
    assert_eq!(approved["decision"]["channel"], "authenticated_website");
    assert_eq!(approved["decision"]["actor"], "alice");
    assert_eq!(
        approved["decision"]["schema"],
        "codefriend.publication_decision.v2"
    );
    let (status, repeated) = http_call(&app, "POST", decide, Some(alice), request.clone()).await;
    assert_eq!(status, 200, "{repeated}");
    assert_eq!(
        repeated["decision"]["digest"],
        approved["decision"]["digest"]
    );
    append_decision(
        &store,
        &review,
        &publication,
        DecisionKind::Invalidated,
        "operator",
        "Revoked test decision",
        now + 1,
    )
    .unwrap();
    assert_eq!(
        http_call(&app, "POST", decide, Some(alice), request)
            .await
            .0,
        409
    );
    assert_eq!(
        read_decision_head(&store, &review, &publication)
            .unwrap()
            .unwrap()
            .decision,
        DecisionKind::Invalidated
    );
    fs::write(
        operation.join("work/exports/retained-output.md"),
        "source-derived output fixture",
    )
    .unwrap();
    fs::write(
        &credentials,
        serde_json::to_vec(&vec![credential(bob, "bob")]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        http_call(
            &app,
            "GET",
            "/v1/operations/run1/publication/result",
            Some(alice),
            json!(null)
        )
        .await
        .0,
        401
    );
    op.expires_at = 0;
    fs::write(
        operation.join("operation.json"),
        serde_json::to_vec(&op).unwrap(),
    )
    .unwrap();
    service.expire().unwrap();
    assert!(!operation.join("work").exists());
    assert!(!operation.join("result.json").exists());
    let retained: Vec<_> = fs::read_dir(&operation)
        .unwrap()
        .map(|entry| entry.unwrap().file_name())
        .collect();
    assert_eq!(retained, vec![std::ffi::OsString::from("operation.json")]);
}

async fn http_call(
    app: &axum::Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: serde_json::Value,
) -> (u16, serde_json::Value) {
    use axum::{
        body::{to_bytes, Body},
        http::Request,
    };
    use tower::ServiceExt;
    let mut request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(
            request
                .body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status().as_u16();
    let bytes = to_bytes(response.into_body(), 16 * 1024 * 1024)
        .await
        .unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null),
    )
}

fn provider_request() -> adl::provider_communication::ProviderInvocationRequestV1 {
    use adl::{model_identity::ModelIdentityStrengthV1, provider_communication::*};
    let route = ProviderRouteV1 {
        provider_kind: ProviderKindV1::Hosted,
        provider: "openai".to_string(),
        runtime_surface: RuntimeSurfaceV1::HostedApi,
        provider_model_id: "codefriend-fixture-model".to_string(),
        endpoint_ref: Some("http://127.0.0.1:1".to_string()),
        credential_ref: Some("env:ADL_CODEFRIEND_REVIEW_FIXTURE_KEY".to_string()),
        source_registry: Some("codefriend-review-fixture".to_string()),
    };
    let mut model_identity = hosted_model_identity(
        "openai",
        "codefriend-fixture-model",
        "codefriend-fixture-model",
        Some("codefriend-review-fixture".to_string()),
    );
    model_identity.identity_strength = ModelIdentityStrengthV1::ProviderAsserted;
    ProviderInvocationRequestV1 {
        route,
        model_identity,
        prompt_contract_ref: "template.replaced.by.runner".to_string(),
        lane_ref: "template".to_string(),
        run_id: None,
        request_id: None,
        attempt_policy: ProviderAttemptPolicyV1 {
            max_attempts: 1,
            timeout_ms: 5_000,
            retry_backoff_ms: Some(1),
        },
        input_text: None,
        max_output_tokens: Some(512),
        context_window_tokens: None,
        reasoning_effort: None,
        clear_thinking: Some(true),
        temperature: Some(0.0),
        top_p: None,
        local_keep_alive: None,
        inference_parameter_fingerprint: Some("temperature=0,max_output_tokens=512".into()),
        tool_surface: Some("none".into()),
        governance_surface: Some("read_only_findings_only".into()),
        evaluator_ref: None,
        benchmark_ref: None,
    }
}

#[test]
fn interrupted_preparation_does_not_block_distinct_format_bundles() {
    use adl::codefriend::integration::{prepare_publication_bundle_for_format, PublicationFormat};
    let temp = local_temp();
    let review = temp.path().join("review.json");
    let destination = temp.path().join("exports");
    fs::create_dir(&destination).unwrap();
    let output = temp.path().join("markdown");
    fs::write(&review, b"invalid review").unwrap();
    assert!(prepare_publication_bundle_for_format(
        &review,
        &output,
        &destination,
        PublicationFormat::Markdown
    )
    .is_err());
    assert!(!output.exists());
    fs::write(&review, serde_json::to_vec(&complete_review()).unwrap()).unwrap();
    let mut bindings = std::collections::BTreeSet::new();
    let mut targets = std::collections::BTreeSet::new();
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        let publication = prepare_publication_bundle_for_format(
            &review,
            &temp.path().join(format.key()),
            &destination,
            format,
        )
        .unwrap();
        assert_eq!(
            publication.renderer_versions.get(format.key()).unwrap(),
            format.renderer_version()
        );
        assert_eq!(publication.renderer_versions.len(), 1);
        assert!(bindings.insert(publication.binding_digest().unwrap()));
        assert!(targets.insert(publication.target));
    }
    assert_eq!(fs::read_dir(&destination).unwrap().count(), 0);
}
