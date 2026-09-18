//! PVF runtime component tests: deterministic HTTP authorization, persistence and
//! cancellation with injected backend. No paid provider or deployment proof.
use adl::codefriend::{
    evidence::Admission,
    ingestion::{local, Packet, Scope},
    review::lanes::ReviewLane,
    server::*,
};
use anyhow::Result;
use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tower::ServiceExt;
const ALICE: &str = "alice-test-only-0123456789012345678901234567";
const BOB: &str = "bob-test-only-012345678901234567890123456789";
const AGENT: &str = "agent-test-only-0123456789012345678901234567";
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
struct Fixture {
    dir: PathBuf,
    config: Config,
    packet: Packet,
}
fn git(p: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(p)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().into()
}
impl Fixture {
    fn new() -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("target/codefriend-server-tests")
            .join(format!(
                "{}-{}-{}",
                std::process::id(),
                now(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&dir).unwrap();
        let repo = dir.join("repo");
        fs::create_dir(&repo).unwrap();
        git(&repo, &["init"]);
        git(
            &repo,
            &["remote", "add", "origin", "https://example.com/team/repo"],
        );
        fs::write(repo.join("lib.rs"), "pub fn answer() -> u32 { 42 }\n").unwrap();
        git(&repo, &["add", "lib.rs"]);
        git(
            &repo,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "-m",
                "fixture",
            ],
        );
        let revision = git(&repo, &["rev-parse", "HEAD"]);
        let packet = local::acquire(
            &repo,
            "https://example.com/team/repo",
            &revision,
            Scope {
                analysis: vec!["lib.rs".into()],
                context: vec![],
                max_files: 1,
                max_bytes: 4096,
                max_file_bytes: 4096,
            },
        )
        .unwrap();
        let credentials_file = dir.join("credentials.json");
        let creds = vec![
            credential(ALICE, "alice", Mode::Hosted),
            credential(BOB, "bob", Mode::Hosted),
            credential(AGENT, "alice", Mode::LocalModel),
        ];
        fs::write(&credentials_file, serde_json::to_vec(&creds).unwrap()).unwrap();
        let provider = provider_request();
        let config = Config {
            root: dir.join("state"),
            credentials_file,
            provider,
            candidate_revision: build_revision().into(),
            max_concurrent: 2,
            max_operations_per_subject: 4,
            retention_seconds: 60,
        };
        Self {
            dir,
            config,
            packet,
        }
    }
    fn request(&self, id: &str) -> Value {
        json!({"operation_id":id,"packet":self.packet,"mode":"hosted","lane":null})
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}
fn credential(token: &str, subject: &str, mode: Mode) -> Credential {
    Credential {
        token_hash: blake3::hash(token.as_bytes()).to_hex().to_string(),
        subject: subject.into(),
        mode,
        expires_at: now() + 3600,
    }
}
struct Fake {
    calls: AtomicUsize,
    hold: AtomicBool,
    fail: bool,
}
impl Fake {
    fn new(hold: bool, fail: bool) -> Arc<Self> {
        Arc::new(Self {
            calls: AtomicUsize::new(0),
            hold: AtomicBool::new(hold),
            fail,
        })
    }
}
impl Backend for Fake {
    fn execute(&self, _: &Config, _: &Submit, _: Admission, _: &Path) -> Result<Value> {
        self.calls.fetch_add(1, Ordering::SeqCst);
        while self.hold.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(5));
        }
        anyhow::ensure!(!self.fail, "fixture_failure");
        Ok(json!({"fixture":true}))
    }
}
async fn call(
    app: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut r = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(t) = token {
        r = r.header("authorization", format!("Bearer {t}"));
    }
    let response = app
        .clone()
        .oneshot(
            r.body(Body::from(serde_json::to_vec(&body).unwrap()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), MAX_BODY * 2).await.unwrap();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}
async fn settled(app: &Router, token: &str, id: &str) -> Value {
    for _ in 0..200 {
        let (_, v) = call(
            app,
            "GET",
            &format!("/v1/operations/{id}"),
            Some(token),
            Value::Null,
        )
        .await;
        if !matches!(v["status"].as_str(), Some("running" | "cancel_requested")) {
            return v;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    panic!("operation did not settle")
}

#[tokio::test]
async fn unauthorized_bodies_are_never_polled_or_parsed() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    for token in [None, Some("invalid-token")] {
        let polls = Arc::new(AtomicUsize::new(0));
        let observed = polls.clone();
        let body = Body::from_stream(futures_util::stream::poll_fn(move |_| {
            observed.fetch_add(1, Ordering::SeqCst);
            std::task::Poll::Ready(Some(Ok::<_, std::io::Error>(
                axum::body::Bytes::from_static(b"{invalid"),
            )))
        }));
        let mut request = Request::builder()
            .method("POST")
            .uri("/v1/operations")
            .header("content-type", "application/json")
            .header("content-length", MAX_BODY + 1);
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let response = app
            .clone()
            .oneshot(request.body(body).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(polls.load(Ordering::SeqCst), 0);
        assert_eq!(response.headers()["cache-control"], "no-store");
    }
    for token in [None, Some(ALICE)] {
        let mut request = Request::builder()
            .method("POST")
            .uri("/v1/operations")
            .header("content-type", "application/json");
        if let Some(token) = token {
            request = request.header("authorization", format!("Bearer {token}"));
        }
        let response = app
            .clone()
            .oneshot(request.body(Body::from("{invalid")).unwrap())
            .await
            .unwrap();
        assert_eq!(
            response.status(),
            if token.is_some() {
                StatusCode::BAD_REQUEST
            } else {
                StatusCode::UNAUTHORIZED
            }
        );
    }
}

#[tokio::test]
async fn authorization_is_rechecked_after_a_delayed_body() {
    for expire in [false, true] {
        let f = Fixture::new();
        let backend = Fake::new(false, false);
        let app = Service::open(f.config.clone(), backend.clone())
            .unwrap()
            .router();
        let mut body = Some(serde_json::to_vec(&f.request("delayed")).unwrap());
        let credentials = f.config.credentials_file.clone();
        // Polling starts only after early authentication. Revoke then yield the
        // delayed JSON, reproducing expiry/revocation while a body is in flight.
        let stream = futures_util::stream::poll_fn(move |_| {
            let Some(bytes) = body.take() else {
                return std::task::Poll::Ready(None);
            };
            let mut records: Vec<Credential> =
                serde_json::from_slice(&fs::read(&credentials).unwrap()).unwrap();
            if expire {
                records[0].expires_at = 0;
            } else {
                records.remove(0);
            }
            fs::write(&credentials, serde_json::to_vec(&records).unwrap()).unwrap();
            std::task::Poll::Ready(Some(Ok::<_, std::io::Error>(axum::body::Bytes::from(
                bytes,
            ))))
        });
        let response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/v1/operations")
                    .header("content-type", "application/json")
                    .header("authorization", format!("Bearer {ALICE}"))
                    .body(Body::from_stream(stream))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
        assert_eq!(backend.calls.load(Ordering::SeqCst), 0);
        assert!(!f.config.root.join("operations/alice/delayed").exists());
    }
}

#[test]
fn build_provenance_tracks_sources_without_lifecycle_residue() {
    let f = Fixture::new();
    let repository = f.dir.join("build-source");
    fs::create_dir_all(repository.join("adl/src")).unwrap();
    fs::write(repository.join("adl/Cargo.toml"), "fixture").unwrap();
    fs::write(repository.join("adl/src/lib.rs"), "// clean").unwrap();
    fs::create_dir_all(repository.join("adl-uts/schemas")).unwrap();
    fs::write(repository.join("adl-uts/schemas/resource.json"), "{}").unwrap();
    git(&repository, &["init"]);
    git(&repository, &["add", "adl", "adl-uts"]);
    git(
        &repository,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "source",
        ],
    );
    let probe = f.dir.join("build-provenance-probe");
    assert!(Command::new("rustc")
        .args(["--edition", "2021"])
        .arg(Path::new(env!("CARGO_MANIFEST_DIR")).join("build.rs"))
        .arg("-o")
        .arg(&probe)
        .status()
        .unwrap()
        .success());
    let run = || {
        let result = Command::new(&probe)
            .env("CARGO_MANIFEST_DIR", repository.join("adl"))
            .output()
            .unwrap();
        assert!(result.status.success());
        String::from_utf8(result.stdout).unwrap()
    };
    let revision = git(&repository, &["rev-parse", "HEAD"]);
    let clean = run();
    assert!(clean.contains(&format!("CODEFRIEND_BUILD_REVISION={revision}")));
    assert!(clean.contains("CODEFRIEND_BUILD_CLEAN=true"));
    fs::create_dir_all(repository.join(".csdlc/evidence")).unwrap();
    fs::write(repository.join(".csdlc/evidence/untracked.json"), "{}").unwrap();
    assert!(run().contains("CODEFRIEND_BUILD_CLEAN=true"));
    fs::write(
        repository.join("adl-uts/schemas/resource.json"),
        "{\"changed\":true}",
    )
    .unwrap();
    let dirty_resource = run();
    assert!(dirty_resource.contains("CODEFRIEND_BUILD_CLEAN=false"));
    assert!(dirty_resource.contains("adl-uts/schemas"));
    fs::write(repository.join("adl-uts/schemas/resource.json"), "{}").unwrap();
    fs::write(repository.join("adl/src/lib.rs"), "// dirty").unwrap();
    assert!(run().contains("CODEFRIEND_BUILD_CLEAN=false"));
    git(&repository, &["add", "adl/src/lib.rs"]);
    assert!(run().contains("CODEFRIEND_BUILD_CLEAN=false"));
    git(
        &repository,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "changed source",
        ],
    );
    let next = git(&repository, &["rev-parse", "HEAD"]);
    assert_ne!(revision, next);
    let clean = run();
    assert!(clean.contains(&format!("CODEFRIEND_BUILD_REVISION={next}")));
    assert!(clean.contains("CODEFRIEND_BUILD_CLEAN=true"));
}

#[test]
fn candidate_identity_must_match_the_compiled_build() {
    let mut f = Fixture::new();
    for forged in [
        "0".repeat(40),
        "f".repeat(40),
        git(&f.dir.join("repo"), &["rev-parse", "HEAD"]),
    ] {
        f.config.candidate_revision = forged;
        assert!(Service::open(f.config.clone(), Fake::new(false, false))
            .err()
            .unwrap()
            .to_string()
            .contains("candidate_revision_mismatch"));
    }
    f.config.candidate_revision = build_revision().into();
    assert!(Service::open(f.config.clone(), Fake::new(false, false)).is_ok());
}

#[tokio::test]
async fn authentication_scope_and_isolation_precede_provider_effects() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    assert_eq!(
        call(&app, "POST", "/v1/operations", None, f.request("one"))
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(AGENT),
            f.request("one")
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("one")
        )
        .await
        .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(settled(&app, ALICE, "one").await["status"], "complete");
    assert_eq!(
        call(
            &app,
            "GET",
            "/v1/operations/one/result",
            Some(BOB),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations/one/cancel",
            Some(BOB),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(
            &app,
            "GET",
            "/v1/operations/one/result",
            Some(AGENT),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(
            &app,
            "GET",
            "/v1/operations/one/result",
            Some(ALICE),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    fs::write(
        &f.config.credentials_file,
        serde_json::to_vec(&vec![credential(BOB, "bob", Mode::Hosted)]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        call(&app, "GET", "/v1/operations/one", Some(ALICE), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
}
#[tokio::test]
async fn durable_reservation_rejects_replay_and_restart_interruption() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    {
        let app = Service::open(f.config.clone(), backend.clone())
            .unwrap()
            .router();
        assert_eq!(
            call(
                &app,
                "POST",
                "/v1/operations",
                Some(ALICE),
                f.request("one")
            )
            .await
            .0,
            StatusCode::ACCEPTED
        );
        settled(&app, ALICE, "one").await;
        assert_eq!(
            call(
                &app,
                "POST",
                "/v1/operations",
                Some(ALICE),
                f.request("one")
            )
            .await
            .0,
            StatusCode::CONFLICT
        );
    }
    let p = f.config.root.join("operations/alice/one/operation.json");
    let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    v["status"] = json!("running");
    fs::write(&p, serde_json::to_vec(&v).unwrap()).unwrap();
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    assert_eq!(
        call(&app, "GET", "/v1/operations/one", Some(ALICE), Value::Null)
            .await
            .1["status"],
        "interrupted"
    );
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("one")
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 1);
}
#[tokio::test]
async fn cancel_does_not_free_capacity_or_publish_late_success() {
    let mut f = Fixture::new();
    f.config.max_concurrent = 1;
    let backend = Fake::new(true, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("one")
        )
        .await
        .0,
        StatusCode::ACCEPTED
    );
    for _ in 0..200 {
        if backend.calls.load(Ordering::SeqCst) > 0 {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert_eq!(backend.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations/one/cancel",
            Some(ALICE),
            Value::Null
        )
        .await
        .1["status"],
        "cancel_requested"
    );
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("two")
        )
        .await
        .0,
        StatusCode::TOO_MANY_REQUESTS
    );
    backend.hold.store(false, Ordering::SeqCst);
    assert_eq!(settled(&app, ALICE, "one").await["status"], "cancelled");
    assert_eq!(
        call(
            &app,
            "GET",
            "/v1/operations/one/result",
            Some(ALICE),
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
}
#[tokio::test]
async fn failures_quotas_and_expiry_do_not_create_success() {
    let mut f = Fixture::new();
    f.config.max_operations_per_subject = 1;
    let backend = Fake::new(false, true);
    let service = Service::open(f.config.clone(), backend.clone()).unwrap();
    let app = service.clone().router();
    let mut invalid = f.request("bad");
    invalid["packet"]["packet_id"] = json!("forged");
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(ALICE), invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("one")
        )
        .await
        .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(settled(&app, ALICE, "one").await["status"], "failed");
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("two")
        )
        .await
        .0,
        StatusCode::TOO_MANY_REQUESTS
    );
    let dir = f.config.root.join("operations/alice/one");
    let p = dir.join("operation.json");
    let mut v: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    v["expires_at"] = json!(0);
    fs::write(p, serde_json::to_vec(&v).unwrap()).unwrap();
    fs::write(dir.join("result.tmp"), b"private crash residue").unwrap();
    service.expire().unwrap();
    assert!(!dir.join("result.tmp").exists());
    assert!(!dir.join("work").exists());
    assert_eq!(
        call(&app, "GET", "/v1/operations/one", Some(ALICE), Value::Null)
            .await
            .1["status"],
        "expired"
    );
}
#[tokio::test]
async fn expired_credentials_and_oversized_requests_never_dispatch() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    let mut expired = credential(ALICE, "alice", Mode::Hosted);
    expired.expires_at = now() - 1;
    fs::write(
        &f.config.credentials_file,
        serde_json::to_vec(&vec![expired, credential(BOB, "bob", Mode::Hosted)]).unwrap(),
    )
    .unwrap();
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("expired")
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let huge = json!({"operation_id":"huge", "packet": "x".repeat(MAX_BODY + 1), "mode":"hosted", "lane":null});
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(BOB), huge)
            .await
            .0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn local_model_scope_and_lane_are_restricted() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    let mut request = f.request("lane");
    request["mode"] = json!("local_model");
    request["lane"] = json!(ReviewLane::Security);
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(ALICE), request.clone())
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(AGENT), request)
            .await
            .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(settled(&app, AGENT, "lane").await["status"], "complete");
    assert_eq!(
        call(
            &app,
            "GET",
            "/v1/operations/lane/result",
            Some(ALICE),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
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

/// Exercises the built server and real review/provider adapter over loopback.
/// Provider replies are controlled fixtures; this is not real-provider qualification.
#[test]
fn built_server_runs_hosted_pipeline_and_rejects_invalid_local_findings() {
    use std::io::{Read, Write};
    use std::net::TcpListener;
    use std::process::Stdio;
    let mut f = Fixture::new();
    f.config.max_operations_per_subject = 8;
    let provider = TcpListener::bind("127.0.0.1:0").unwrap();
    provider.set_nonblocking(true).unwrap();
    f.config.provider.route.endpoint_ref = Some(format!(
        "http://{}/v1/responses",
        provider.local_addr().unwrap()
    ));
    let stop = Arc::new(AtomicBool::new(false));
    let stopped = stop.clone();
    let count = Arc::new(AtomicUsize::new(0));
    let calls = count.clone();
    let mock = std::thread::spawn(move || {
        while !stopped.load(Ordering::SeqCst) {
            let (mut stream, _) = match provider.accept() {
                Ok(s) => s,
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                    std::thread::sleep(Duration::from_millis(5));
                    continue;
                }
                Err(e) => panic!("{e}"),
            };
            stream
                .set_read_timeout(Some(Duration::from_secs(3)))
                .unwrap();
            let mut bytes = Vec::new();
            let mut buf = [0; 4096];
            loop {
                let n = stream.read(&mut buf).unwrap();
                if n == 0 {
                    break;
                }
                bytes.extend_from_slice(&buf[..n]);
                if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                    let head = String::from_utf8_lossy(&bytes[..end]);
                    let length = head
                        .lines()
                        .find_map(|l| {
                            l.to_ascii_lowercase()
                                .strip_prefix("content-length:")
                                .and_then(|s| s.trim().parse::<usize>().ok())
                        })
                        .unwrap_or(0);
                    if bytes.len() >= end + 4 + length {
                        break;
                    }
                }
            }
            let index = calls.fetch_add(1, Ordering::SeqCst);
            if index >= 6 {
                if write!(
                    stream,
                    "HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\nConnection: close\r\n\r\n"
                )
                .is_ok()
                {
                    for _ in 0..=(adl::provider_adapter::MAX_PROVIDER_HTTP_RESPONSE_BYTES / 8192) {
                        if stream
                            .write_all(b"2000\r\n")
                            .and_then(|_| stream.write_all(&[b'x'; 8192]))
                            .and_then(|_| stream.write_all(b"\r\n"))
                            .is_err()
                        {
                            break;
                        }
                    }
                    let _ = stream.write_all(b"0\r\n\r\n");
                }
                continue;
            }
            let text = if index < 4 || index == 5 {
                json!({"findings":[]})
            } else {
                json!({"findings":[{"rule":"correctness.wrong_lane","semantic_anchor":"lib.rs","title":"fixture","severity":"info","rationale":"fixture","confidence":{"state":"known","percent":90},"evidence":["foreign"],"inference":"fixture","limitations":[]}]})
            };
            let body = json!({"output_text":text.to_string()}).to_string();
            write!(stream,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",body.len(),body).unwrap();
        }
    });
    let config = f.dir.join("config.json");
    fs::write(&config, serde_json::to_vec(&f.config).unwrap()).unwrap();
    let address = TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap();
    // Relocation preserves compiled provenance; adjacent JSON has no authority.
    let installed = f.dir.join("relocated-codefriend-server");
    fs::copy(env!("CARGO_BIN_EXE_codefriend-server"), &installed).unwrap();
    fs::write(
        f.dir.join("relocated-codefriend-server.json"),
        br#"{"candidate_revision":"0000000000000000000000000000000000000000"}"#,
    )
    .unwrap();
    let stderr = fs::File::create(f.dir.join("server.stderr")).unwrap();
    let mut child = Command::new(&installed)
        .args([
            "--config",
            config.to_str().unwrap(),
            "--listen",
            &address.to_string(),
        ])
        .env("ADL_CODEFRIEND_REVIEW_FIXTURE_KEY", "fixture-key")
        .env("ADL_OBSERVABILITY_OTEL", "0")
        .stdout(Stdio::null())
        .stderr(stderr)
        .spawn()
        .unwrap();
    let outcome = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(5))
            .build()
            .unwrap();
        let base = format!("http://{address}");
        let mut listening = false;
        for _ in 0..200 {
            if std::net::TcpStream::connect(address).is_ok() {
                listening = true;
                break;
            }
            assert!(child.try_wait().unwrap().is_none(), "server exited");
            std::thread::sleep(Duration::from_millis(10));
        }
        assert!(listening);
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(ALICE)
                .json(&f.request("hosted"))
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        let terminal = |token: &str, id: &str| -> Value {
            for _ in 0..300 {
                let v: Value = client
                    .get(format!("{base}/v1/operations/{id}"))
                    .bearer_auth(token)
                    .send()
                    .unwrap()
                    .json()
                    .unwrap();
                if v["status"] != "running" {
                    return v;
                }
                std::thread::sleep(Duration::from_millis(10));
            }
            panic!("subprocess operation timed out")
        };
        assert_eq!(terminal(ALICE, "hosted")["status"], "complete");
        let response = client
            .get(format!("{base}/v1/operations/hosted/result"))
            .bearer_auth(ALICE)
            .send()
            .unwrap();
        assert_eq!(response.headers()["cache-control"], "no-store");
        let result: Value = response.json().unwrap();
        assert_eq!(result["lane_results"].as_array().unwrap().len(), 4);
        assert_eq!(result["completion"], "complete");
        let mut local = f.request("local");
        local["mode"] = json!("local_model");
        local["lane"] = json!("security");
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(AGENT)
                .json(&local)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        assert_eq!(terminal(AGENT, "local")["status"], "failed");
        assert_eq!(
            client
                .get(format!("{base}/v1/operations/local/result"))
                .bearer_auth(AGENT)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            409
        );
        assert_eq!(count.load(Ordering::SeqCst), 5);
        let mut valid = f.request("local-valid");
        valid["mode"] = json!("local_model");
        valid["lane"] = json!("security");
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(AGENT)
                .json(&valid)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        let operation = terminal(AGENT, "local-valid");
        assert_eq!(operation["status"], "complete");
        let result: Value = client
            .get(format!("{base}/v1/operations/local-valid/result"))
            .bearer_auth(AGENT)
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(result["candidate_revision"], build_revision());
        assert_eq!(
            operation["candidate_revision"],
            result["candidate_revision"]
        );
        assert_eq!(operation["model_identity"], result["model_identity"]);
        assert_eq!(
            result["model_identity"]["provider"],
            f.config.provider.route.provider
        );
        assert_eq!(
            result["model_identity"]["provider_model_id"],
            f.config.provider.route.provider_model_id
        );
        for (id, token, mode) in [
            ("oversized-hosted", ALICE, "hosted"),
            ("oversized-local", AGENT, "local_model"),
        ] {
            let mut request = f.request(id);
            request["mode"] = json!(mode);
            if mode == "local_model" {
                request["lane"] = json!("security");
            }
            assert_eq!(
                client
                    .post(format!("{base}/v1/operations"))
                    .bearer_auth(token)
                    .json(&request)
                    .send()
                    .unwrap()
                    .status()
                    .as_u16(),
                202
            );
            assert_eq!(terminal(token, id)["status"], "failed");
            assert_eq!(
                client
                    .get(format!("{base}/v1/operations/{id}/result"))
                    .bearer_auth(token)
                    .send()
                    .unwrap()
                    .status()
                    .as_u16(),
                409
            );
            let operation_root = f.config.root.join("operations").join("alice").join(id);
            assert!(operation_root.join("operation.json").exists());
            assert!(!operation_root.join("result.json").exists());
        }
    }));
    // Exercise the operator shutdown path and let instrumented binaries flush
    // their coverage. SIGKILL remains only bounded failure cleanup.
    #[cfg(unix)]
    let shutdown = {
        // SAFETY: this is the live child process owned by this test.
        let _ = unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGINT) };
        let deadline = std::time::Instant::now() + Duration::from_secs(5);
        loop {
            if let Some(status) = child.try_wait().unwrap() {
                break Some(status);
            }
            if std::time::Instant::now() >= deadline {
                break None;
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    };
    #[cfg(not(unix))]
    let shutdown: Option<std::process::ExitStatus> = None;
    if shutdown.is_none() {
        let _ = child.kill();
        let _ = child.wait();
    }
    stop.store(true, Ordering::SeqCst);
    mock.join().unwrap();
    if let Err(p) = outcome {
        std::panic::resume_unwind(p);
    }
    #[cfg(unix)]
    assert!(shutdown.expect("graceful shutdown deadline").success());
}
