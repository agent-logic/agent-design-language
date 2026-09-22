//! PVF runtime component tests: deterministic HTTP authorization, persistence and
//! cancellation with injected backend. No paid provider or deployment proof.
use adl::codefriend::{
    activities::{Activity, TestingGoal, TestingMode, UpdateCyclePlan, PLAN_SCHEMA},
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
        Self::with_source("pub fn answer() -> u32 { 42 }\n")
    }
    fn with_source(source: &str) -> Self {
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
        fs::write(repo.join("lib.rs"), source).unwrap();
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
                max_bytes: 1024 * 1024,
                max_file_bytes: 1024 * 1024,
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
async fn empty_registry_starts_denies_then_provisions_revokes_and_restarts() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let replace = |records: &[Credential]| {
        let temp = f.config.credentials_file.with_extension("tmp");
        fs::write(&temp, serde_json::to_vec(records).unwrap()).unwrap();
        fs::rename(temp, &f.config.credentials_file).unwrap();
    };
    replace(&[]);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    for token in [None, Some(ALICE)] {
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
    }
    replace(&[credential(ALICE, "alice", Mode::Hosted)]);
    let path = "/v1/operations/not-created";
    assert_eq!(
        call(&app, "GET", path, Some(ALICE), Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        call(&app, "GET", path, Some(BOB), Value::Null).await.0,
        StatusCode::UNAUTHORIZED
    );
    replace(&[]);
    assert_eq!(
        call(&app, "GET", path, Some(ALICE), Value::Null).await.0,
        StatusCode::UNAUTHORIZED
    );
    drop(app);
    let restarted = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    assert_eq!(
        call(&restarted, "GET", path, Some(ALICE), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        fs::read_dir(f.config.root.join("operations"))
            .unwrap()
            .count(),
        0
    );
}

#[test]
fn empty_registry_support_does_not_admit_invalid_registries() {
    let duplicate = vec![
        credential(ALICE, "alice", Mode::Hosted),
        credential(ALICE, "alice", Mode::Hosted),
    ];
    let too_many: Vec<_> = (0..513)
        .map(|i| credential(&format!("fixture-token-{i}"), "alice", Mode::Hosted))
        .collect();
    for bytes in [
        b"{".to_vec(),
        b"{}".to_vec(),
        b"[{}]".to_vec(),
        serde_json::to_vec(&duplicate).unwrap(),
        serde_json::to_vec(&too_many).unwrap(),
    ] {
        let f = Fixture::new();
        fs::write(&f.config.credentials_file, bytes).unwrap();
        assert!(Service::open(f.config.clone(), Fake::new(false, false)).is_err());
    }
    let f = Fixture::new();
    fs::remove_file(&f.config.credentials_file).unwrap();
    assert!(Service::open(f.config.clone(), Fake::new(false, false)).is_err());
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
    // An absent optional Git file must not make every Cargo build dirty.
    assert!(!repository.join(".git/packed-refs").exists());
    assert!(!clean.contains(".git/packed-refs"));
    git(&repository, &["pack-refs", "--all"]);
    assert!(run().contains(".git/packed-refs"));
    assert!(run().contains(".git/refs/heads"));
    assert!(run().contains(&format!("CODEFRIEND_BUILD_REVISION={revision}")));
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
fn cargo_provenance_stays_fresh_and_detects_packed_ref_recreation() {
    let f = Fixture::new();
    let repository = f.dir.join("cargo-provenance");
    fs::create_dir_all(repository.join("adl/src")).unwrap();
    fs::write(repository.join(".gitignore"), "target/\n").unwrap();
    fs::write(
        repository.join("adl/Cargo.toml"),
        "[package]\nname=\"provenance-probe\"\nversion=\"0.1.0\"\nedition=\"2021\"\n",
    )
    .unwrap();
    fs::write(repository.join("adl/src/main.rs"), "fn main() { println!(\"{} {}\", env!(\"CODEFRIEND_BUILD_REVISION\"), env!(\"CODEFRIEND_BUILD_CLEAN\")); }\n").unwrap();
    fs::copy(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("build.rs"),
        repository.join("adl/build.rs"),
    )
    .unwrap();
    git(&repository, &["init"]);
    // Materialize all declared production-resource watches in this tiny fixture.
    let probe = f.dir.join("cargo-build-script-probe");
    assert!(Command::new("rustc")
        .arg(repository.join("adl/build.rs"))
        .arg("-o")
        .arg(&probe)
        .status()
        .unwrap()
        .success());
    let output = Command::new(&probe)
        .env("CARGO_MANIFEST_DIR", repository.join("adl"))
        .output()
        .unwrap();
    for line in String::from_utf8(output.stdout).unwrap().lines() {
        if let Some(path) = line.strip_prefix("cargo:rerun-if-changed=") {
            let path = Path::new(path);
            if !path.exists() && !path.starts_with(repository.join(".git")) {
                fs::create_dir_all(path.parent().unwrap()).unwrap();
                fs::write(path, "{}").unwrap();
            }
        }
    }
    let commit = || {
        git(&repository, &["add", "."]);
        git(
            &repository,
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
        git(&repository, &["rev-parse", "HEAD"])
    };
    assert!(Command::new("cargo")
        .args(["generate-lockfile", "--offline", "--manifest-path"])
        .arg(repository.join("adl/Cargo.toml"))
        .status()
        .unwrap()
        .success());
    let first = commit();
    let build = || {
        let out = Command::new("cargo")
            .args([
                "build",
                "--offline",
                "--verbose",
                "--color",
                "never",
                "--manifest-path",
            ])
            .arg(repository.join("adl/Cargo.toml"))
            .env("CARGO_TARGET_DIR", repository.join("target"))
            .env("CARGO_ENCODED_RUSTFLAGS", "")
            .env("RUSTFLAGS", "")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stderr).unwrap()
    };
    let embedded = || {
        let out = Command::new(repository.join("target/debug/provenance-probe"))
            .output()
            .unwrap();
        assert!(out.status.success());
        String::from_utf8(out.stdout).unwrap().trim().to_string()
    };
    build();
    assert_eq!(embedded(), format!("{first} true"));
    assert!(build().contains("Fresh provenance-probe"));
    git(&repository, &["pack-refs", "--all"]);
    build();
    assert!(build().contains("Fresh provenance-probe"));
    fs::write(repository.join("README.md"), "docs-only commit\n").unwrap();
    let second = commit();
    assert_ne!(first, second);
    build();
    assert_eq!(embedded(), format!("{second} true"));
    assert!(build().contains("Fresh provenance-probe"));
    git(&repository, &["checkout", "--detach", &first]);
    build();
    assert_eq!(embedded(), format!("{first} true"));
    assert!(build().contains("Fresh provenance-probe"));
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
async fn update_cycle_plan_is_validated_before_reservation_for_hosted_and_local_modes() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let app = Service::open(f.config.clone(), backend.clone())
        .unwrap()
        .router();
    let plan = UpdateCyclePlan {
        schema: PLAN_SCHEMA.into(),
        repository: f.packet.repository.clone(),
        activities: vec![Activity::Documentation, Activity::Tests],
        testing: Some(TestingGoal {
            mode: TestingMode::Target,
            target: Some(80),
        }),
    };
    let mut invalid = f.request("cycle-hosted");
    invalid["cycle"] = serde_json::to_value(&plan).unwrap();
    invalid["cycle"]["repository"] = "https://example.com/wrong/repo".into();
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(ALICE), invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 0);

    let mut hosted = f.request("cycle-hosted");
    hosted["cycle"] = serde_json::to_value(&plan).unwrap();
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(ALICE), hosted)
            .await
            .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(
        settled(&app, ALICE, "cycle-hosted").await["status"],
        "complete"
    );

    let local = json!({
        "operation_id":"cycle-local",
        "packet":f.packet,
        "mode":"local_model",
        "lane":null,
        "cycle":plan
    });
    assert_eq!(
        call(&app, "POST", "/v1/operations", Some(AGENT), local)
            .await
            .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(
        settled(&app, AGENT, "cycle-local").await["status"],
        "complete"
    );
    assert_eq!(backend.calls.load(Ordering::SeqCst), 2);
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
    f.config.max_operations_per_subject = 16;
    // Kimi observes the returned model ID, exercising requested-alias resolution.
    f.config.provider.route.provider = "kimi".into();
    f.config.provider.model_identity.provider = "kimi".into();
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
            // Accepted sockets can inherit the listener's nonblocking mode on
            // macOS. Request reads use the explicit bounded blocking timeout.
            stream.set_nonblocking(false).unwrap();
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
            if index >= 25 {
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
            let text = if index < 4 || (12..25).contains(&index) {
                json!({"assessments":[]})
            } else if index == 5 {
                json!({"findings":[]})
            } else if index == 6 {
                json!({
                    "schema":"codefriend.activity_output.v1",
                    "artifacts":[{"path":"docs/guide.md","kind":"documentation","disposition":"create","content":"# Guide","evidence_paths":["lib.rs"],"unsupported_claims":[],"limitations":["proposal only"],"render_manifest":null}],
                    "gaps":[],
                    "measured_coverage_percent":null
                })
            } else if index == 7 {
                json!({"schema":"wrong"})
            } else {
                json!({"findings":[{"rule":"correctness.wrong_lane","semantic_anchor":"lib.rs","title":"fixture","severity":"info","rationale":"fixture","confidence":{"state":"known","percent":90},"evidence":["foreign"],"inference":"fixture","limitations":[]}]})
            };
            // #1133: a provider may resolve a requested alias to an observed model.
            // All four lanes of each cycle must finalize the same original bytes.
            let mut response = json!({"output_text":text.to_string(), "choices":[{"message":{"content":text.to_string()}}]});
            if (12..20).contains(&index) {
                response["model"] = json!("fixture-observed-cycle-model");
            }
            let body = response.to_string();
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
        // Test supervision only: instrumented CI can take longer than three
        // seconds for four lanes. This never cancels or reposts a customer job.
        let mut terminal = |token: &str, id: &str| -> Value {
            let started = std::time::Instant::now();
            let budget = Duration::from_secs(60);
            loop {
                if let Some(status) = child.try_wait().expect("observe owned server child") {
                    panic!("server exited while observing operation {id}: {status}");
                }
                let response = client
                    .get(format!("{base}/v1/operations/{id}"))
                    .bearer_auth(token)
                    .send()
                    .unwrap_or_else(|error| panic!("operation {id} observation failed: {error}"));
                let http_status = response.status();
                let v: Value = response.json().unwrap_or_else(|error| {
                    panic!("operation {id} HTTP {http_status} invalid status response: {error}")
                });
                assert!(
                    http_status.is_success(),
                    "operation {id}: HTTP {http_status}"
                );
                if v["status"] != "running" {
                    return v;
                }
                assert!(
                    started.elapsed() < budget,
                    "fixture observation budget exhausted: operation={id} status={} elapsed={:?}",
                    v["status"],
                    started.elapsed()
                );
                std::thread::sleep(Duration::from_millis(20));
            }
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
        assert_eq!(
            result["schema"],
            "codefriend.four_perspective_review_run.v3"
        );
        assert_eq!(
            result["review_record"]["run"]["schema"],
            "codefriend.contracts.v3"
        );
        assert_eq!(
            result["review_record"]["run"]["assessment_set"]["schema"],
            "codefriend.assessment_set.v1"
        );
        assert_eq!(
            result["review_record"]["run"]["assessment_set"]["assessments"],
            json!([])
        );
        assert_eq!(result["review_record"]["findings"], json!([]));
        for lane in result["lane_results"].as_array().unwrap() {
            assert_eq!(lane["schema"], "codefriend.review_lane_result.v2");
            assert_eq!(lane["assessment_ids"], json!([]));
        }
        let typed: adl::codefriend::review::runner::FourPerspectiveReviewRun =
            serde_json::from_value(result).unwrap();
        typed.successful_execution().unwrap();
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
        assert_eq!(result["schema"], "codefriend.local_model_result.v1");
        assert_eq!(result["output"], json!({"findings":[]}));
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
        let mut complete_cycle = f.request("cycle-complete");
        complete_cycle["cycle"] = serde_json::to_value(UpdateCyclePlan {
            schema: PLAN_SCHEMA.into(),
            repository: f.packet.repository.clone(),
            activities: vec![Activity::Documentation],
            testing: None,
        })
        .unwrap();
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(ALICE)
                .json(&complete_cycle)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        let complete_operation = terminal(ALICE, "cycle-complete");
        assert_eq!(complete_operation["status"], "complete");
        let complete_result: Value = client
            .get(format!("{base}/v1/operations/cycle-complete/result"))
            .bearer_auth(ALICE)
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(complete_result["completion"], "complete");
        assert_eq!(
            complete_result["execution"]["candidate_revision"],
            complete_operation["candidate_revision"]
        );
        assert_eq!(
            complete_result["execution"]["request_digest"],
            complete_operation["request_digest"]
        );
        assert_eq!(
            complete_result["execution"]["model_identity"],
            complete_operation["model_identity"]
        );
        let mut failed_cycle = f.request("cycle-failed");
        failed_cycle["cycle"] = serde_json::to_value(UpdateCyclePlan {
            schema: PLAN_SCHEMA.into(),
            repository: f.packet.repository.clone(),
            activities: vec![Activity::Documentation],
            testing: None,
        })
        .unwrap();
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(ALICE)
                .json(&failed_cycle)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        let failed_operation = terminal(ALICE, "cycle-failed");
        assert_eq!(failed_operation["status"], "failed");
        let failed_result: Value = client
            .get(format!("{base}/v1/operations/cycle-failed/result"))
            .bearer_auth(ALICE)
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(failed_result["completion"], "failed");
        assert_eq!(failed_result["activities"][0]["status"], "failed");
        assert_eq!(failed_result["failures"][0], "documentation_failed");
        assert_eq!(
            failed_result["execution"]["candidate_revision"],
            failed_operation["candidate_revision"]
        );
        assert_eq!(
            failed_result["execution"]["request_digest"],
            failed_operation["request_digest"]
        );
        assert_eq!(
            failed_result["execution"]["model_identity"],
            failed_operation["model_identity"]
        );
        let mut failed_review_cycle = f.request("cycle-review-failed");
        failed_review_cycle["cycle"] = serde_json::to_value(UpdateCyclePlan {
            schema: PLAN_SCHEMA.into(),
            repository: f.packet.repository.clone(),
            activities: vec![Activity::Review],
            testing: None,
        })
        .unwrap();
        assert_eq!(
            client
                .post(format!("{base}/v1/operations"))
                .bearer_auth(ALICE)
                .json(&failed_review_cycle)
                .send()
                .unwrap()
                .status()
                .as_u16(),
            202
        );
        assert_eq!(terminal(ALICE, "cycle-review-failed")["status"], "failed");
        let failed_review_result: Value = client
            .get(format!("{base}/v1/operations/cycle-review-failed/result"))
            .bearer_auth(ALICE)
            .send()
            .unwrap()
            .json()
            .unwrap();
        assert_eq!(failed_review_result["completion"], "failed");
        assert_eq!(failed_review_result["activities"][0]["activity"], "review");
        assert_eq!(failed_review_result["activities"][0]["status"], "failed");
        assert_eq!(failed_review_result["failures"][0], "review_failed");
        assert_eq!(count.load(Ordering::SeqCst), 12);
        for (id, token, mode) in [
            ("assessment-cycle-hosted", ALICE, "hosted"),
            ("assessment-cycle-local", AGENT, "local_model"),
        ] {
            let mut request = f.request(id);
            request["mode"] = json!(mode);
            request["review_generation"] = json!("assessments");
            request["cycle"] = serde_json::to_value(UpdateCyclePlan {
                schema: PLAN_SCHEMA.into(),
                repository: f.packet.repository.clone(),
                activities: vec![Activity::Review],
                testing: None,
            })
            .unwrap();
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
            assert_eq!(terminal(token, id)["status"], "complete");
            let result: Value = client
                .get(format!("{base}/v1/operations/{id}/result"))
                .bearer_auth(token)
                .send()
                .unwrap()
                .json()
                .unwrap();
            let cycle = if mode == "hosted" {
                &result
            } else {
                &result["cycle_result"]
            };
            assert_eq!(cycle["completion"], "complete");
            assert_eq!(
                cycle["execution"]["model_identity"]["provider_model_id"],
                "fixture-observed-cycle-model"
            );
            // #1133: the native capsule is a GET of retained producer evidence;
            // Journey/publication reuse this cycle without another model call.
            let before = count.load(Ordering::SeqCst);
            let evidence = client
                .get(format!("{base}/v1/operations/{id}/review-evidence"))
                .bearer_auth(token)
                .send()
                .unwrap();
            assert!(
                evidence.status().is_success(),
                "capsule: {}",
                evidence.text().unwrap()
            );
            let capsule: Value = client
                .get(format!("{base}/v1/operations/{id}/review-evidence"))
                .bearer_auth(token)
                .send()
                .unwrap()
                .json()
                .unwrap();
            assert_eq!(capsule["schema"], "codefriend.cycle_review_evidence.v1");
            assert_eq!(capsule["files"].as_object().unwrap().len(), 10);
            assert_eq!(
                serde_json::from_str::<Value>(capsule["files"]["run.json"].as_str().unwrap())
                    .unwrap(),
                cycle["review"]
            );
            assert!(capsule["files"]
                .as_object()
                .unwrap()
                .keys()
                .all(|name| !name.contains("provider")));
            assert!(!client
                .get(format!("{base}/v1/operations/{id}/review-evidence"))
                .bearer_auth(BOB)
                .send()
                .unwrap()
                .status()
                .is_success());
            if mode == "hosted" {
                let policies = json!({"boundary_policy":{"schema":"codefriend.structure.v1","crate_root":"lib.rs","manifest_path":null,"layers":{"lib.rs":"core"},"allowed":[],"coupling_threshold":2},
                    "fitness_policy":{"schema":"codefriend.fitness.v1","rules":[{"id":"no_network","kind":"forbidden_declared_use","source_path":"lib.rs","forbidden_prefix":"reqwest"}]}});
                let prepared = client
                    .post(format!("{base}/v1/operations/{id}/journey"))
                    .bearer_auth(token)
                    .json(&policies)
                    .send()
                    .unwrap();
                let status = prepared.status();
                let body = prepared.text().unwrap();
                assert!(status.is_success(), "cycle Journey {status}: {body}");
                let publication = client
                    .post(format!("{base}/v1/operations/{id}/publication/challenge"))
                    .bearer_auth(token)
                    .json(&json!({"format":"markdown"}))
                    .send()
                    .unwrap();
                let status = publication.status();
                let body = publication.text().unwrap();
                assert!(status.is_success(), "cycle publication {status}: {body}");
            }
            assert_eq!(
                count.load(Ordering::SeqCst),
                before,
                "continuation cannot replay model work"
            );
            assert_eq!(
                cycle["review"]["schema"],
                "codefriend.four_perspective_review_run.v3"
            );
            assert_eq!(
                cycle["review"]["review_record"]["run"]["assessment_set"]["assessments"],
                json!([])
            );
        }
        assert_eq!(count.load(Ordering::SeqCst), 20);
        // Real source acquisition and production HTTP/provider paths: the same
        // >128 KiB prompt is accepted only by assessment generation.
        let large = Fixture::with_source(&format!("// {}\n", "a".repeat(200 * 1024)));
        let expanded = Fixture::with_source(&"\n".repeat(400 * 1024));
        for (id, token, mode, generation, packet, expected) in [
            (
                "large-legacy",
                AGENT,
                "local_model",
                None,
                &large.packet,
                "failed",
            ),
            (
                "large-hosted",
                ALICE,
                "hosted",
                None,
                &large.packet,
                "complete",
            ),
            (
                "large-local",
                AGENT,
                "local_model",
                Some("assessments"),
                &large.packet,
                "complete",
            ),
            (
                "expanded-hosted",
                ALICE,
                "hosted",
                None,
                &expanded.packet,
                "failed",
            ),
            (
                "expanded-local",
                AGENT,
                "local_model",
                Some("assessments"),
                &expanded.packet,
                "failed",
            ),
        ] {
            let before = count.load(Ordering::SeqCst);
            let mut request = f.request(id);
            request["packet"] = serde_json::to_value(packet).unwrap();
            request["mode"] = json!(mode);
            if mode == "local_model" {
                request["lane"] = json!("security");
            }
            if let Some(generation) = generation {
                request["review_generation"] = json!(generation);
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
            let operation = terminal(token, id);
            assert_eq!(operation["status"], expected, "{id}: {operation}");
            let after = count.load(Ordering::SeqCst);
            assert_eq!(
                after - before,
                if expected == "failed" {
                    0
                } else if mode == "hosted" {
                    4
                } else {
                    1
                }
            );
        }
        assert_eq!(count.load(Ordering::SeqCst), 25);

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

#[tokio::test]
async fn drain_blocks_new_reservations_without_consuming_identity_and_can_resume() {
    let f = Fixture::new();
    let backend = Fake::new(false, false);
    let service = Service::open(f.config.clone(), backend.clone()).unwrap();
    let app = service.clone().router();
    assert!(!service.drained_without_payloads().unwrap());
    assert!(service.quiescent_without_payloads().unwrap());
    service.begin_drain().unwrap();
    assert!(service.drained_without_payloads().unwrap());
    let request = json!({"operation_id":"after_drain","packet":f.packet,"mode":"hosted"});
    let (status, _) = call(&app, "POST", "/v1/operations", Some(ALICE), request.clone()).await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert!(!f.config.root.join("operations/alice/after_drain").exists());
    service.resume_admissions().unwrap();
    let (status, _) = call(&app, "POST", "/v1/operations", Some(ALICE), request).await;
    assert_eq!(status, StatusCode::ACCEPTED);
    settled(&app, ALICE, "after_drain").await;
}

#[tokio::test]
async fn drain_waits_for_worker_and_retained_payload_then_keeps_no_replay_identity() {
    let f = Fixture::new();
    let backend = Fake::new(true, false);
    let service = Service::open(f.config.clone(), backend.clone()).unwrap();
    let app = service.clone().router();
    let (status, _) = call(
        &app,
        "POST",
        "/v1/operations",
        Some(ALICE),
        json!({"operation_id":"active","packet":f.packet,"mode":"hosted"}),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED);
    service.begin_drain().unwrap();
    assert!(!service.quiescent_without_payloads().unwrap());
    let before = service.drained_without_payloads();
    backend.hold.store(false, Ordering::SeqCst);
    assert!(!before.unwrap());
    settled(&app, ALICE, "active").await;
    assert!(!service.drained_without_payloads().unwrap());
    assert!(!service.quiescent_without_payloads().unwrap());
    let dir = f.config.root.join("operations/alice/active");
    let file = dir.join("operation.json");
    let mut operation: Operation = serde_json::from_slice(&fs::read(&file).unwrap()).unwrap();
    operation.expires_at = 0;
    fs::write(&file, serde_json::to_vec(&operation).unwrap()).unwrap();
    for _ in 0..200 {
        if service.drained_without_payloads().unwrap() {
            break;
        }
        tokio::time::sleep(Duration::from_millis(5)).await;
    }
    assert!(service.drained_without_payloads().unwrap());
    assert!(file.exists());
    service.resume_admissions().unwrap();
    assert!(service.quiescent_without_payloads().unwrap());
    assert!(!service.drained_without_payloads().unwrap());
    assert!(!dir.join("work").exists());
    assert!(!dir.join("result.json").exists());
}

#[test]
fn drain_rejects_uncertain_or_corrupt_reservations() {
    let f = Fixture::new();
    let service = Service::open(f.config.clone(), Fake::new(false, false)).unwrap();
    let dir = f.config.root.join("operations/alice/unknown");
    fs::create_dir_all(&dir).unwrap();
    service.begin_drain().unwrap();
    assert!(!service.drained_without_payloads().unwrap());
    fs::write(dir.join("operation.json"), b"invalid").unwrap();
    assert!(service.drained_without_payloads().is_err());
}

#[test]
fn drain_retains_interrupted_uncertainty_through_expiry_and_restart() {
    let f = Fixture::new();
    let service = Service::open(f.config.clone(), Fake::new(false, false)).unwrap();
    let dir = f.config.root.join("operations/alice/interrupted");
    fs::create_dir_all(dir.join("work")).unwrap();
    fs::write(dir.join("work/source"), b"retained fixture source").unwrap();
    fs::write(dir.join("result.json"), b"{}").unwrap();
    let operation = Operation {
        operation_id: "interrupted".into(),
        subject: "alice".into(),
        mode: Mode::Hosted,
        request_digest: "fixture".into(),
        packet_id: f.packet.packet_id.clone(),
        source_revision: f.packet.revision.clone(),
        candidate_revision: build_revision().into(),
        model_identity: None,
        expires_at: 0,
        status: Status::Interrupted,
    };
    fs::write(
        dir.join("operation.json"),
        serde_json::to_vec(&operation).unwrap(),
    )
    .unwrap();
    drop(service);
    // Startup invokes cleanup too: it must not turn uncertainty into permission.
    let restarted = Service::open(f.config.clone(), Fake::new(false, false)).unwrap();
    restarted.begin_drain().unwrap();
    assert!(!restarted.drained_without_payloads().unwrap());
    assert!(!dir.join("work").exists());
    assert!(!dir.join("result.json").exists());
    let retained: Operation =
        serde_json::from_slice(&fs::read(dir.join("operation.json")).unwrap()).unwrap();
    assert_eq!(retained.status, Status::Interrupted);
}

// PVF: local platform integration; exercises the real CLI control lifecycle and
// graceful shutdown with private sockets and no provider requests. Required
// component/coverage regression; not installed systemd or deployment acceptance.
#[cfg(unix)]
#[test]
fn server_cli_control_socket_drains_resumes_and_shuts_down_gracefully() {
    use std::io::{Read, Write};
    use std::os::unix::{fs::PermissionsExt, net::UnixStream};
    use std::process::{Child, Stdio};

    struct ChildGuard(Child);
    impl Drop for ChildGuard {
        fn drop(&mut self) {
            if self.0.try_wait().ok().flatten().is_none() {
                let _ = self.0.kill();
                let _ = self.0.wait();
            }
        }
    }
    let f = Fixture::new();
    fs::write(
        f.dir.join("config.json"),
        serde_json::to_vec(&f.config).unwrap(),
    )
    .unwrap();
    let private = f.dir.join("control");
    fs::create_dir(&private).unwrap();
    fs::set_permissions(&private, fs::Permissions::from_mode(0o700)).unwrap();
    // Resolve the short socket name relative to the child directory on macOS.
    let socket = private.join("gateway.sock");
    let stderr = fs::File::create(f.dir.join("control-server.stderr")).unwrap();
    let mut child = ChildGuard(
        Command::new(env!("CARGO_BIN_EXE_codefriend-server"))
            .current_dir(&f.dir)
            .args([
                "--config",
                "config.json",
                "--listen",
                "127.0.0.1:0",
                "--control-socket",
                "control/gateway.sock",
            ])
            .env("ADL_OBSERVABILITY_OTEL", "0")
            .stdout(Stdio::null())
            .stderr(stderr)
            .spawn()
            .unwrap(),
    );
    // The client also needs a short path: Unix socket paths have a small limit.
    let client_socket = socket
        .strip_prefix(std::env::current_dir().unwrap())
        .unwrap();
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while !socket.exists() {
        assert!(
            child.0.try_wait().unwrap().is_none(),
            "server exited before control bind"
        );
        assert!(
            std::time::Instant::now() < deadline,
            "control bind deadline"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    let request = |value: Value| -> Value {
        let mut stream = UnixStream::connect(client_socket).unwrap();
        stream
            .set_read_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        stream
            .set_write_timeout(Some(Duration::from_secs(3)))
            .unwrap();
        let mut bytes = serde_json::to_vec(&value).unwrap();
        bytes.push(b'\n');
        stream.write_all(&bytes).unwrap();
        let mut reply = Vec::new();
        stream.take(4096).read_to_end(&mut reply).unwrap();
        serde_json::from_slice(&reply).unwrap()
    };
    let status = request(json!({"schema":"codefriend.host_control.v1","action":"status"}));
    assert_eq!(
        fs::metadata(&socket).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert_eq!(status["ok"], true);
    assert_eq!(status["candidate_revision"], build_revision());
    assert_eq!(status["pid"], child.0.id());
    assert_eq!(status["quiescent_without_payloads"], true);
    let instance = status["instance"].as_str().unwrap();
    assert!(!instance.is_empty());
    let attempt = "0123456789abcdef0123456789abcdef";
    let drained = request(
        json!({"schema":"codefriend.host_control.v1","action":"drain","instance":instance,"attempt":attempt}),
    );
    assert_eq!(drained["ok"], true);
    assert_eq!(drained["drained_without_payloads"], true);
    assert_eq!(drained["attempt"], attempt);
    let resumed = request(
        json!({"schema":"codefriend.host_control.v1","action":"resume","instance":instance,"attempt":attempt}),
    );
    assert_eq!(resumed["ok"], true);
    assert_eq!(resumed["draining"], false);
    // A control task can answer before Axum polls its shutdown future. Prove
    // the HTTP server is serving before sending the shutdown signal.
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    let address = loop {
        let log = fs::read_to_string(f.dir.join("control-server.stderr")).unwrap();
        if let Some(value) = log.split_inclusive('\n').find_map(|line| {
            line.strip_suffix('\n')?
                .strip_prefix("adl_event component=codefriend_server event=listening address=")
        }) {
            break value.parse::<std::net::SocketAddr>().unwrap();
        }
        assert!(
            child.0.try_wait().unwrap().is_none(),
            "server exited before HTTP readiness"
        );
        assert!(
            std::time::Instant::now() < deadline,
            "HTTP readiness deadline"
        );
        std::thread::sleep(Duration::from_millis(10));
    };
    assert!(address.ip().is_loopback());
    assert_ne!(address.port(), 0);
    let response = reqwest::blocking::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
        .unwrap()
        .get(format!("http://{address}/no-such-route"))
        .send()
        .unwrap();
    assert_eq!(response.status().as_u16(), 404);
    // SIGINT exercises control task cancellation and permits LLVM profile flush.
    // SAFETY: the PID belongs to the live child owned by this test.
    assert_eq!(
        unsafe { libc::kill(child.0.id() as libc::pid_t, libc::SIGINT) },
        0
    );
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = child.0.try_wait().unwrap() {
            assert!(
                status.success(),
                "graceful server shutdown failed: {status}"
            );
            break;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "graceful shutdown deadline"
        );
        std::thread::sleep(Duration::from_millis(10));
    }
    assert!(UnixStream::connect(client_socket).is_err());
}

#[cfg(unix)]
async fn operator_control(socket: &Path, value: Value) -> Value {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let mut stream = tokio::net::UnixStream::connect(socket).await.unwrap();
    let mut bytes = serde_json::to_vec(&value).unwrap();
    bytes.push(b'\n');
    stream.write_all(&bytes).await.unwrap();
    let mut reply = Vec::new();
    tokio::time::timeout(
        Duration::from_secs(3),
        stream.take(4096).read_to_end(&mut reply),
    )
    .await
    .unwrap()
    .unwrap();
    serde_json::from_slice(&reply).unwrap()
}

#[cfg(unix)]
#[tokio::test]
async fn private_control_binds_drain_to_instance_and_attempt() {
    use std::os::unix::fs::PermissionsExt;
    let f = Fixture::new();
    let service = Service::open(f.config.clone(), Fake::new(false, false)).unwrap();
    let parent = f.dir.join("control");
    fs::create_dir(&parent).unwrap();
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o700)).unwrap();
    // Relative socket names avoid the small sockaddr_un limit on macOS.
    let cwd = std::env::current_dir().unwrap();
    let socket = parent.strip_prefix(&cwd).unwrap().join("gateway.sock");
    let control = control::ControlServer::bind(service.clone(), &socket).unwrap();
    assert_eq!(
        fs::metadata(&socket).unwrap().permissions().mode() & 0o777,
        0o600
    );
    assert!(control::ControlServer::bind(service.clone(), &socket).is_err());
    let task = tokio::spawn(control.serve());
    let status = operator_control(
        &socket,
        json!({"schema":"codefriend.host_control.v1","action":"status"}),
    )
    .await;
    assert_eq!(status["ok"], true);
    assert_eq!(status["drained_without_payloads"], false);
    assert_eq!(status["quiescent_without_payloads"], true);
    let instance = status["instance"].as_str().unwrap();
    let attempt = "0123456789abcdef0123456789abcdef";
    let stale = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":"drain","instance":"old","attempt":attempt})).await;
    assert_eq!(stale["ok"], false);
    let drained = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":"drain","instance":instance,"attempt":attempt})).await;
    assert_eq!(drained["drained_without_payloads"], true);
    assert_eq!(drained["attempt"], attempt);
    for action in ["drain", "resume"] {
        let conflicting = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":action,"instance":instance,"attempt":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"})).await;
        assert_eq!(conflicting["ok"], false);
        assert!(service.drained_without_payloads().unwrap());
    }
    let resumed = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":"resume","instance":instance,"attempt":attempt})).await;
    assert_eq!(resumed["draining"], false);
    assert!(!service.drained_without_payloads().unwrap());
    let replay = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":"drain","instance":instance,"attempt":attempt})).await;
    assert_eq!(replay["ok"], false);
    assert!(!service.drained_without_payloads().unwrap());
    let fresh = operator_control(&socket, json!({"schema":"codefriend.host_control.v1","action":"drain","instance":instance,"attempt":"bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"})).await;
    assert_eq!(fresh["drained_without_payloads"], true);

    let malformed = operator_control(
        &socket,
        json!({"schema":"codefriend.host_control.v1","action":"status","command":"shutdown"}),
    )
    .await;
    assert_eq!(malformed["ok"], false);
    task.abort();
    let _ = task.await;
}

#[cfg(unix)]
#[tokio::test]
async fn private_control_rejects_shared_directory_and_oversized_requests() {
    use std::os::unix::fs::PermissionsExt;
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    let f = Fixture::new();
    let service = Service::open(f.config.clone(), Fake::new(false, false)).unwrap();
    let parent = f.dir.join("control");
    fs::create_dir(&parent).unwrap();
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o755)).unwrap();
    let cwd = std::env::current_dir().unwrap();
    let socket = parent.strip_prefix(&cwd).unwrap().join("gateway.sock");
    assert!(control::ControlServer::bind(service.clone(), &socket).is_err());
    assert!(!socket.exists());
    fs::set_permissions(&parent, fs::Permissions::from_mode(0o700)).unwrap();
    let task = tokio::spawn(
        control::ControlServer::bind(service.clone(), &socket)
            .unwrap()
            .serve(),
    );
    let mut stream = tokio::net::UnixStream::connect(&socket).await.unwrap();
    stream.write_all(&vec![b'x'; 4097]).await.unwrap();
    let mut reply = Vec::new();
    let _ = tokio::time::timeout(Duration::from_secs(3), stream.read_to_end(&mut reply))
        .await
        .unwrap();
    assert!(reply.is_empty());
    assert!(!service.drained_without_payloads().unwrap());
    let status = operator_control(
        &socket,
        json!({"schema":"codefriend.host_control.v1","action":"status"}),
    )
    .await;
    assert_eq!(status["ok"], true);
    task.abort();
    let _ = task.await;
}

#[tokio::test]
async fn uncertain_provider_effect_is_retained_after_cancel_and_cannot_redispatch() {
    struct Uncertain(AtomicUsize);
    impl Backend for Uncertain {
        fn execute(&self, _: &Config, _: &Submit, _: Admission, dir: &Path) -> Result<Value> {
            self.0.fetch_add(1, Ordering::SeqCst);
            fs::write(dir.join("cancel"), b"operator cancellation")?;
            Err(adl::provider_adapter::CodeFriendProviderInterrupted.into())
        }
    }
    let f = Fixture::new();
    let backend = Arc::new(Uncertain(AtomicUsize::new(0)));
    let service = Service::open(f.config.clone(), backend.clone()).unwrap();
    let app = service.clone().router();
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("uncertain")
        )
        .await
        .0,
        StatusCode::ACCEPTED
    );
    assert_eq!(
        settled(&app, ALICE, "uncertain").await["status"],
        "interrupted"
    );
    assert_eq!(
        call(
            &app,
            "POST",
            "/v1/operations",
            Some(ALICE),
            f.request("uncertain")
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(backend.0.load(Ordering::SeqCst), 1);
    service.begin_drain().unwrap();
    assert!(!service.drained_without_payloads().unwrap());
}

// PVF runtime deterministic serialization: legacy request identity is unchanged;
// explicit assessment generation is authenticated by the existing request digest.
#[test]
fn model_generation_is_explicit_and_legacy_submit_bytes_are_preserved() {
    let f = Fixture::new();
    let old = json!({"operation_id":"legacy-model", "packet":f.packet,"mode":"local_model","lane":"code"});
    let mut old = old;
    old["lane"] = serde_json::to_value(ReviewLane::ALL[0]).unwrap();
    let legacy: Submit = serde_json::from_value(old.clone()).unwrap();
    assert!(legacy.review_generation.is_none());
    assert_eq!(serde_json::to_value(&legacy).unwrap(), old);
    let mut modern = legacy.clone();
    modern.review_generation = Some(ReviewGeneration::Assessments);
    assert_ne!(
        adl::codefriend::evidence::hash(&modern).unwrap(),
        adl::codefriend::evidence::hash(&legacy).unwrap()
    );
    let mut unknown = serde_json::to_value(modern).unwrap();
    unknown["review_generation"] = "unknown".into();
    assert!(serde_json::from_value::<Submit>(unknown).is_err());
}

/// PVF owner_binary, deterministic CPU/files only: actual admission and canonical
/// annotated source prompts, including a later lane that fails before dispatch.
#[test]
fn assessment_prompt_exact_boundary_and_all_lane_preflight() {
    use adl::codefriend::{
        evidence::{store::Store, Retention},
        review::runner,
    };
    let admit = |f: &Fixture| {
        Store::open(&f.dir.join("prompt-evidence"), now)
            .unwrap()
            .admit(f.packet.clone(), Retention { seconds: 3600 })
            .unwrap()
    };
    let base_source = format!("{}a", "\n".repeat(250_000));
    let base = Fixture::with_source(&base_source);
    let admission = admit(&base);
    let sizes: Vec<_> = ReviewLane::ALL
        .into_iter()
        .map(|lane| {
            runner::assessment_lane_input_manifest("boundary", lane, &admission)
                .unwrap()
                .1
                .len()
        })
        .collect();
    let largest = *sizes.iter().max().unwrap();
    assert!(largest < runner::MAX_ASSESSMENT_PROMPT_BYTES);
    assert!(
        sizes[0] < largest,
        "fixture must isolate a later oversized lane"
    );
    let padding = runner::MAX_ASSESSMENT_PROMPT_BYTES - largest;
    for extra in [0usize, 1] {
        let f = Fixture::with_source(&format!("{base_source}{}", "a".repeat(padding + extra)));
        let admission = admit(&f);
        let results: Vec<_> = ReviewLane::ALL
            .into_iter()
            .map(|lane| runner::assessment_lane_input_manifest("boundary", lane, &admission))
            .collect();
        assert!(results[0].is_ok());
        if extra == 0 {
            assert!(results.iter().all(Result::is_ok));
            assert_eq!(
                results
                    .iter()
                    .map(|r| r.as_ref().unwrap().1.len())
                    .max()
                    .unwrap(),
                runner::MAX_ASSESSMENT_PROMPT_BYTES
            );
        } else {
            assert!(results.iter().any(|r| r
                .as_ref()
                .err()
                .is_some_and(|e| e.to_string() == "assessment_prompt_byte_limit")));
            let calls = AtomicUsize::new(0);
            let out = f.dir.join("no-dispatch");
            let result = runner::run_assessments_with_executor(
                runner::ExecutionOptions {
                    out: out.clone(),
                    run_id: "boundary".into(),
                    cancel_file: None,
                },
                admission,
                "synthetic:no-provider".into(),
                |_, _, _| {
                    calls.fetch_add(1, Ordering::SeqCst);
                    anyhow::bail!("must not dispatch")
                },
            );
            assert_eq!(
                result.unwrap_err().to_string(),
                "assessment_prompt_byte_limit"
            );
            assert_eq!(calls.load(Ordering::SeqCst), 0);
            assert!(!out.exists());
        }
    }
}
