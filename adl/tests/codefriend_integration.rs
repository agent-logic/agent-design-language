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
    // The shared journey routes keep the same pre-body authentication boundary.
    for (method, route) in [
        ("POST", "/v1/operations/run1/journey"),
        ("GET", "/v1/operations/run1/journey"),
        ("POST", "/v1/operations/run1/journey/step"),
        ("GET", "/v1/operations/run1/journey/graph"),
    ] {
        assert_eq!(
            http_call(&app, method, route, None, json!(null)).await.0,
            401
        );
    }
    for route in [
        "/v1/operations/run1/journey",
        "/v1/operations/run1/journey/graph",
    ] {
        assert_eq!(
            http_call(&app, "GET", route, Some(bob), json!(null))
                .await
                .0,
            404
        );
    }
    assert!(!operation.join("work/journey-reservation.json").exists());
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
    // Actual native renderers consume the authenticated website decision. Synthetic
    // review input remains component proof, not live provider or human acceptance.
    use base64::Engine;
    let render_route = "/v1/operations/run1/publication/render";
    let render_request = json!({"format":"markdown", "binding_digest":challenge["binding_digest"],
        "decision_digest":approved["decision"]["digest"]});
    assert_eq!(
        http_call(&app, "POST", render_route, None, render_request.clone())
            .await
            .0,
        401
    );
    assert_eq!(
        http_call(
            &app,
            "POST",
            render_route,
            Some(bob),
            render_request.clone()
        )
        .await
        .0,
        404
    );
    let mut forged = render_request.clone();
    forged["decision_digest"] = json!("forged");
    assert_eq!(
        http_call(&app, "POST", render_route, Some(alice), forged)
            .await
            .0,
        409
    );
    assert!(!operation
        .join("work/publications/markdown/render-reservation.json")
        .exists());
    let (status, rendered) = http_call(
        &app,
        "POST",
        render_route,
        Some(alice),
        render_request.clone(),
    )
    .await;
    assert_eq!(status, 200, "{rendered}");
    assert_eq!(rendered["export_status"], "complete");
    let report = base64::engine::general_purpose::STANDARD
        .decode(rendered["exports"][0]["bytes_base64"].as_str().unwrap())
        .unwrap();
    assert!(!report.is_empty());
    assert_eq!(
        rendered["exports"][0]["digest"],
        blake3::hash(&report).to_hex().to_string()
    );
    let marker = operation.join("work/publications/markdown/render-reservation.json");
    let reserved_bytes = fs::read(&marker).unwrap();
    let (_, observed) = http_call(
        &app,
        "GET",
        "/v1/operations/run1/publication/result",
        Some(alice),
        json!(null),
    )
    .await;
    assert_eq!(observed, rendered);
    let (_, repeated_render) = http_call(
        &app,
        "POST",
        render_route,
        Some(alice),
        render_request.clone(),
    )
    .await;
    assert_eq!(repeated_render, rendered);
    assert_eq!(fs::read(&marker).unwrap(), reserved_bytes);
    let report_path = operation
        .join("work/exports")
        .join(&publication.target)
        .join("report.md");
    fs::write(&report_path, b"tampered").unwrap();
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
        500
    );
    assert_eq!(
        http_call(
            &app,
            "POST",
            render_route,
            Some(alice),
            render_request.clone()
        )
        .await
        .0,
        500
    );
    assert_eq!(fs::read(&report_path).unwrap(), b"tampered"); // no rerender repairs tampered output
    fs::write(&report_path, &report).unwrap();
    // An existing reservation never permits a second render when the committed
    // output is absent, regardless of how the interruption occurred.
    let target = report_path.parent().unwrap();
    let saved_target = operation.join("work/export-test-held");
    fs::rename(target, &saved_target).unwrap();
    let (status, unresolved) = http_call(
        &app,
        "POST",
        render_route,
        Some(alice),
        render_request.clone(),
    )
    .await;
    assert_eq!(status, 200, "{unresolved}");
    assert_eq!(unresolved["export_status"], "effect_unresolved");
    assert_eq!(unresolved["exports"], json!([]));
    assert!(!target.exists());
    fs::rename(&saved_target, target).unwrap();
    // Bound bytes before serialization; committed oversized payloads cannot be
    // claimed as deliverable or silently replaced by another renderer effect.
    fs::write(&report_path, vec![b'x'; 2 * 1024 * 1024 + 1]).unwrap();
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
        500
    );
    assert_eq!(
        http_call(
            &app,
            "POST",
            render_route,
            Some(alice),
            render_request.clone()
        )
        .await
        .0,
        500
    );
    assert_eq!(
        fs::metadata(&report_path).unwrap().len(),
        2 * 1024 * 1024 + 1
    );
    fs::write(&report_path, &report).unwrap();
    let font = [
        "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
        "/System/Library/Fonts/Supplemental/Arial.ttf",
    ]
    .into_iter()
    .map(std::path::PathBuf::from)
    .find(|p| p.is_file())
    .expect("PDF fixture font required");
    fs::copy(font, root.join("publication-font.ttf")).unwrap();
    for format in ["html", "pdf"] {
        let (status, challenge) =
            http_call(&app, "POST", prepare, Some(alice), json!({"format":format})).await;
        assert_eq!(status, 200, "{challenge}");
        let (status, approved) = http_call(&app, "POST", decide, Some(alice), json!({"format":format,
            "challenge_digest":challenge["challenge_digest"], "binding_digest":challenge["binding_digest"],
            "expected_decision_digest":challenge["expected_decision_digest"], "decision":"approved"})).await;
        assert_eq!(status, 200, "{approved}");
        let (status, output) = http_call(&app, "POST", render_route, Some(alice), json!({"format":format,
            "binding_digest":challenge["binding_digest"], "decision_digest":approved["decision"]["digest"]})).await;
        assert_eq!(status, 200, "{output}");
        assert_eq!(output["export_status"], "complete", "{output}");
        assert_eq!(output["exports"].as_array().unwrap().len(), 1);
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(output["exports"][0]["bytes_base64"].as_str().unwrap())
            .unwrap();
        if format == "pdf" {
            assert!(bytes.starts_with(b"%PDF-"));
        }
        let manifest = base64::engine::general_purpose::STANDARD
            .decode(output["manifest_bytes_base64"].as_str().unwrap())
            .unwrap();
        assert_eq!(
            output["render_result"]["manifest_digest"],
            blake3::hash(&manifest).to_hex().to_string()
        );
    }
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
    assert_eq!(
        http_call(&app, "POST", render_route, Some(alice), render_request)
            .await
            .0,
        409
    );
    let (_, revoked_output) = http_call(
        &app,
        "GET",
        "/v1/operations/run1/publication/result",
        Some(alice),
        json!(null),
    )
    .await;
    assert_eq!(revoked_output["exports"], json!([]));
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
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).unwrap_or(serde_json::Value::Null);
    // Explicit optional evidence export for cross-language interoperability. This
    // records the actual native response; it never changes assertions or behavior.
    if status == 200 {
        if let Some(output) = std::env::var_os("CODEFRIEND_TEST_RESPONSE_OUTPUT") {
            let output = std::path::PathBuf::from(output);
            assert!(
                output.is_absolute(),
                "fixture evidence directory must be explicit and absolute"
            );
            fs::create_dir_all(&output).unwrap();
            let name = format!("{}.json", blake3::hash(&bytes).to_hex());
            let file = output.join(name);
            if file.exists() {
                assert_eq!(
                    serde_json::from_slice::<serde_json::Value>(&fs::read(file).unwrap()).unwrap(),
                    value
                );
            } else {
                adl::codefriend::publication::write_json_create_only(&file, &value).unwrap();
            }
        }
    }
    (status, value)
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

#[tokio::test]
async fn hosted_journey_reuses_original_admission_and_completed_review() {
    use adl::codefriend::{
        evidence::Admission,
        ingestion::{local, Scope},
        review::runner::{run_with_executor, ExecutionOptions, LaneExecution},
        server::*,
    };
    use std::{
        path::Path,
        process::Command,
        sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        },
        time::{SystemTime, UNIX_EPOCH},
    };
    struct FixtureBackend(Arc<AtomicUsize>);
    impl Backend for FixtureBackend {
        fn execute(
            &self,
            _: &Config,
            request: &Submit,
            admission: Admission,
            dir: &Path,
        ) -> anyhow::Result<serde_json::Value> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Ok(serde_json::to_value(run_with_executor(
                ExecutionOptions {
                    out: dir.join("work/review"),
                    run_id: request.operation_id.clone(),
                    cancel_file: None,
                },
                admission,
                "fixture:no-provider".into(),
                |_, _, _| {
                    Ok(LaneExecution {
                        final_status:
                            adl::provider_communication::ProviderInvocationFinalStatusV1::Ok,
                        output_text: Some(r#"{"findings":[]}"#.into()),
                    })
                },
            )?)?)
        }
    }
    let temp = local_temp();
    let source = temp.path().join("source");
    fs::create_dir(&source).unwrap();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&source)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().to_string()
    };
    git(&["init", "-b", "main"]);
    git(&["remote", "add", "origin", "https://example.com/owner/repo"]);
    fs::write(source.join("lib.rs"), "pub fn answer() -> u8 { 42 }\n").unwrap();
    fs::write(
        source.join("compose.json"),
        include_bytes!("fixtures/codefriend/rationale/compose.json"),
    )
    .unwrap();
    fs::write(
        source.join("adr.md"),
        include_bytes!("fixtures/codefriend/rationale/accepted.md"),
    )
    .unwrap();
    git(&["add", "lib.rs", "compose.json", "adr.md"]);
    git(&[
        "-c",
        "user.name=fixture",
        "-c",
        "user.email=fixture@example.com",
        "-c",
        "commit.gpgsign=false",
        "commit",
        "-m",
        "fixture",
    ]);
    let revision = git(&["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &source,
        "https://example.com/owner/repo",
        &revision,
        Scope {
            analysis: vec!["lib.rs".into()],
            context: vec!["adr.md".into(), "compose.json".into()],
            max_files: 3,
            max_bytes: 8192,
            max_file_bytes: 4096,
        },
    )
    .unwrap();
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let token = "hosted-journey-fixture-token-0123456789012345";
    let credentials = temp.path().join("credentials.json");
    fs::write(
        &credentials,
        serde_json::to_vec(&vec![Credential {
            token_hash: blake3::hash(token.as_bytes()).to_hex().to_string(),
            subject: "alice".into(),
            mode: Mode::Hosted,
            expires_at: now + 3600,
        }])
        .unwrap(),
    )
    .unwrap();
    let root = temp.path().join("service");
    let calls = Arc::new(AtomicUsize::new(0));
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
        Arc::new(FixtureBackend(calls.clone())),
    )
    .unwrap();
    let app = service.clone().router();
    let submit = json!({"operation_id":"journey1", "packet":packet, "mode":"hosted", "lane":null});
    assert_eq!(
        http_call(&app, "POST", "/v1/operations", Some(token), submit)
            .await
            .0,
        202
    );
    let result_path = root.join("operations/alice/journey1/result.json");
    for _ in 0..200 {
        let (_, observed) = http_call(
            &app,
            "GET",
            "/v1/operations/journey1",
            Some(token),
            json!(null),
        )
        .await;
        if observed["status"] == "complete" {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
    assert!(result_path.exists());
    let completed: serde_json::Value =
        serde_json::from_slice(&fs::read(result_path).unwrap()).unwrap();
    let original_digest = completed["review_record"]["admission"]["digest"].clone();
    let route = "/v1/operations/journey1/journey";
    let policies = json!({
        "boundary_policy":{"schema":"codefriend.structure.v1","crate_root":"lib.rs","manifest_path":null,"layers":{"lib.rs":"core"},"allowed":[],"coupling_threshold":2},
        "fitness_policy":{"schema":"codefriend.fitness.v1","rules":[{"id":"no_network","kind":"forbidden_declared_use","source_path":"lib.rs","forbidden_prefix":"reqwest"}]}
    });
    let mut invalid = policies.clone();
    invalid["fitness_policy"]["rules"][0]["forbidden_prefix"] = json!("self::bad");
    assert_eq!(
        http_call(&app, "POST", route, Some(token), invalid).await.0,
        400
    );
    let mut invalid = policies.clone();
    invalid["boundary_policy"]["crate_root"] = json!("foreign.rs");
    assert_eq!(
        http_call(&app, "POST", route, Some(token), invalid).await.0,
        400
    );
    assert!(!root
        .join("operations/alice/journey1/work/journey-reservation.json")
        .exists());
    let (status, manifest) = http_call(&app, "POST", route, Some(token), policies.clone()).await;
    assert_eq!(status, 200, "{manifest}");
    assert_eq!(manifest["admission_digest"], original_digest);
    assert_eq!(manifest["stages"]["review"]["status"], "complete");
    assert_eq!(manifest["status"], "pending");
    assert_eq!(
        http_call(&app, "POST", route, Some(token), policies.clone()).await,
        (200, manifest.clone())
    );
    assert_eq!(
        http_call(&app, "GET", route, Some(token), json!(null)).await,
        (200, manifest)
    );
    let (status, graph) = http_call(
        &app,
        "GET",
        "/v1/operations/journey1/journey/graph",
        Some(token),
        json!(null),
    )
    .await;
    assert_eq!(status, 200, "{graph}");
    assert_eq!(graph["record"]["admission"]["digest"], original_digest);
    let artifact_route = "/v1/operations/journey1/journey/artifacts";
    assert_eq!(
        http_call(
            &app,
            "GET",
            &format!("{artifact_route}/impact"),
            Some(token),
            json!(null)
        )
        .await
        .0,
        409
    );
    assert_eq!(
        http_call(
            &app,
            "GET",
            &format!("{artifact_route}/unknown"),
            Some(token),
            json!(null)
        )
        .await
        .0,
        404
    );
    let step_route = "/v1/operations/journey1/journey/step";
    let changes = json!({"stage":"impact","changes":{"schema":"codefriend.impact.v1","repository":graph["record"]["run"]["repository"],"revision":revision,"graph_digest":graph["digest"],"targets":[{"kind":"module","name":graph["nodes"][0]["module"]}]}});
    let rationale = json!({"stage":"rationale","selection":{"schema":"codefriend.rationale.v1","graph_digest":graph["digest"],"revision":revision,"boundaries":[{"boundary":"core","deployment_path":"compose.json","service":"api","rationale_paths":["adr.md"]}]}});
    for body in [changes, rationale] {
        let stage = body["stage"].as_str().unwrap().to_string();
        let (status, value) = http_call(&app, "POST", step_route, Some(token), body.clone()).await;
        assert_eq!(status, 200, "{value}");
        assert_eq!(value["stages"][&stage]["status"], "complete", "{value}");
        assert_eq!(
            http_call(&app, "POST", step_route, Some(token), body)
                .await
                .0,
            409
        );
    }
    for stage in ["structure", "fitness", "impact", "rationale"] {
        let (status, value) = http_call(
            &app,
            "GET",
            &format!("{artifact_route}/{stage}"),
            Some(token),
            json!(null),
        )
        .await;
        assert_eq!(status, 200, "{value}");
        assert_eq!(value["record"]["admission"]["digest"], original_digest);
    }
    service.begin_drain().unwrap();
    assert_eq!(http_call(&app,"POST",step_route,Some(token),json!({"stage":"impact","changes":{"schema":"codefriend.impact.v1","repository":graph["record"]["run"]["repository"],"revision":revision,"graph_digest":graph["digest"],"targets":[]}})).await.0,503);
    // Existing preparation is observation-only and remains readable during drain.
    assert_eq!(
        http_call(&app, "POST", route, Some(token), policies.clone())
            .await
            .0,
        200
    );
    let mut changed = policies;
    changed["boundary_policy"]["coupling_threshold"] = json!(3);
    assert_eq!(
        http_call(&app, "POST", route, Some(token), changed).await.0,
        409
    );
    fs::write(&credentials, b"[]").unwrap();
    assert_eq!(
        http_call(
            &app,
            "GET",
            &format!("{artifact_route}/structure"),
            Some(token),
            json!(null)
        )
        .await
        .0,
        401
    );
    assert_eq!(
        calls.load(Ordering::SeqCst),
        1,
        "journey must not redispatch the review"
    );
}
