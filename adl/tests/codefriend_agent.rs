//! PVF: fixtures/codefriend/agent/PVF.json. Component guards, not product acceptance.
use adl::codefriend::{
    agent::{Command, Consent, Journal, Pairing, Transport, PROTOCOL},
    ingestion::Scope,
};
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};
static NEXT: AtomicU64 = AtomicU64::new(0);
struct Fixture(PathBuf);
impl Fixture {
    fn new() -> Self {
        let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/agent-tests")
            .join(format!(
                "{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }
    fn consent(&self) -> Consent {
        Consent {
            schema: PROTOCOL.into(),
            repository_path: self.0.clone(),
            repository: "https://github.com/example/project".into(),
            revision: "a".repeat(40),
            scope: Scope {
                analysis: vec!["src/lib.rs".into()],
                context: vec![],
                max_files: 2,
                max_bytes: 1024,
                max_file_bytes: 1024,
            },
            expires_at: 500,
            retention_seconds: 60,
            allow_model_egress: true,
            allow_result_upload: true,
        }
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}
fn pairing() -> Pairing {
    Pairing {
        schema: PROTOCOL.into(),
        origin: "https://codefriend.example/".into(),
        agent_id: "agent-one".into(),
        subject: "github-123".into(),
        agent_token: "a".repeat(64),
        model_token: "b".repeat(64),
        expires_at: 500,
    }
}
fn command(consent: &Consent) -> Command {
    Command {
        schema: PROTOCOL.into(),
        agent_id: "agent-one".into(),
        subject: "github-123".into(),
        run_id: "run-one".into(),
        consent_digest: consent.digest().unwrap(),
        expires_at: 300,
        cycle: None,
    }
}
#[test]
fn commands_cannot_change_identity_scope_or_expiry() {
    let f = Fixture::new();
    let c = f.consent();
    let p = pairing();
    let cmd = command(&c);
    cmd.validate(&p, &c, 100).unwrap();
    let mut changed = cmd.clone();
    changed.subject = "github-other".into();
    assert!(changed.validate(&p, &c, 100).is_err());
    let mut changed = cmd.clone();
    changed.agent_id = "agent-two".into();
    assert!(changed.validate(&p, &c, 100).is_err());
    let mut changed = cmd.clone();
    changed.consent_digest = "a".repeat(64);
    assert!(changed.validate(&p, &c, 100).is_err());
    assert!(cmd.validate(&p, &c, 300).is_err());
    assert!(cmd.validate(&p, &c, 501).is_err());
    let mut value = serde_json::to_value(&cmd).unwrap();
    value["repository_path"] = "/private".into();
    assert!(serde_json::from_value::<Command>(value).is_err());
}
#[test]
fn consent_requires_exact_revision_bounded_scope_and_egress() {
    let f = Fixture::new();
    let c = f.consent();
    c.validate(100).unwrap();
    let mut v = c.clone();
    v.allow_model_egress = false;
    assert!(v.validate(100).is_err());
    let mut v = c.clone();
    v.allow_result_upload = false;
    assert!(v.validate(100).is_err());
    let mut v = c.clone();
    v.revision = "main".into();
    assert!(v.validate(100).is_err());
    let mut v = c.clone();
    v.scope.analysis = vec!["../secrets".into()];
    assert!(v.validate(100).is_err());
    let mut v = c.clone();
    v.repository_path = PathBuf::from("relative");
    assert!(v.validate(100).is_err());
    let mut v = c.clone();
    v.scope.max_bytes = 1024 * 1024 + 1;
    assert!(v.validate(100).is_err());
    assert!(c.validate(500).is_err());
}
#[test]
fn run_reservations_survive_restart_and_never_replay() {
    let f = Fixture::new();
    let path = f.0.join("state");
    let c = f.consent();
    let p = pairing();
    let cmd = command(&c);
    let journal = Journal::open(&path).unwrap();
    let run = journal.reserve(&cmd, &p, &c, 100).unwrap();
    assert!(run.join("command.json").is_file());
    assert!(Journal::open(&path).is_err());
    assert!(journal.reserve(&cmd, &p, &c, 100).is_err());
    drop(journal);
    let journal = Journal::open(&path).unwrap();
    assert!(journal.reserve(&cmd, &p, &c, 100).is_err());
    // A crash immediately after directory reservation must still prevent dispatch.
    fs::create_dir(path.join("run-crash")).unwrap();
    let mut crash = cmd;
    crash.run_id = "crash".into();
    assert!(journal.reserve(&crash, &p, &c, 100).is_err());
}
#[test]
fn pairing_storage_is_private_create_only_and_expiring() {
    let f = Fixture::new();
    let path = f.0.join("state");
    let journal = Journal::open(&path).unwrap();
    let p = pairing();
    journal.save_pairing(&p, 100).unwrap();
    assert_eq!(
        fs::metadata(path.join("pairing.json"))
            .unwrap()
            .permissions()
            .mode()
            & 0o777,
        0o600
    );
    assert_eq!(journal.pairing(100).unwrap().subject, p.subject);
    assert!(journal.save_pairing(&p, 100).is_err());
    assert!(journal.pairing(500).is_err());
    fs::set_permissions(path.join("pairing.json"), fs::Permissions::from_mode(0o644)).unwrap();
    assert!(journal.pairing(100).is_err());
}
#[test]
fn origin_and_fixture_transport_reject_credential_exfiltration() {
    for origin in [
        "http://example.com/",
        "https://user:pass@example.com/",
        "https://example.com/path",
        "https://example.com/?q=secret",
        "https://example.com/#fragment",
    ] {
        assert!(Transport::new(origin).is_err());
    }
    assert!(Transport::new("https://codefriend.example/").is_ok());
    assert!(Transport::loopback_fixture("http://example.com/").is_err());
    assert!(Transport::loopback_fixture("http://127.0.0.1:1/").is_ok());
}
#[test]
fn journal_rejects_world_readable_or_symlink_store() {
    let f = Fixture::new();
    let path = f.0.join("state");
    fs::create_dir(&path).unwrap();
    fs::set_permissions(&path, fs::Permissions::from_mode(0o755)).unwrap();
    assert!(Journal::open(&path).is_err());
    fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
    let link = f.0.join("alias");
    std::os::unix::fs::symlink(&path, &link).unwrap();
    assert!(Journal::open(&link).is_err());
}

#[derive(Clone, Copy)]
enum Scenario {
    Success,
    Cycle,
    CycleFailure,
    Unpair,
    AggregateLimit,
    LostResultObservation,
    MissingOriginalStore,
    TamperedOriginalStore,
    LostStatusObservation,
    LostInitialControl,
    LostFinalControl,
    CachedBeyondObservationDeadline,
    CandidateResultMismatch,
    CandidateAcrossLanes,
    ModelAcrossLanes,
    PartialScope,
    ShortDeadline,
    LostModelReply,
    Cancel,
    DeleteConsent,
    ExpireRetention,
    CancelRedelivery,
    DeleteDuringControl,
    ExpireDuringControl,
    DeleteBeforeUpload,
    TamperReport,
    Interrupted,
}
fn live_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn git(root: &std::path::Path, args: &[&str]) -> String {
    let out = std::process::Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    String::from_utf8(out.stdout).unwrap().trim().into()
}
struct WireServer {
    origin: String,
    stop: std::sync::Arc<std::sync::atomic::AtomicBool>,
    thread: Option<std::thread::JoinHandle<()>>,
    dispatches: std::sync::Arc<AtomicU64>,
    reports: std::sync::Arc<std::sync::Mutex<Vec<serde_json::Value>>>,
}
impl Drop for WireServer {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        if let Some(t) = self.thread.take() {
            let result = t.join();
            if !std::thread::panicking() {
                result.unwrap();
            }
        }
    }
}
impl WireServer {
    fn start(
        command: Command,
        scenario: Scenario,
        consent_path: PathBuf,
        clock: std::sync::Arc<AtomicU64>,
    ) -> Self {
        use serde_json::json;
        use std::io::{Read, Write};
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let origin = format!("http://{}/", listener.local_addr().unwrap());
        listener.set_nonblocking(true).unwrap();
        let stop = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
        let flag = stop.clone();
        let dispatches = std::sync::Arc::new(AtomicU64::new(0));
        let count = dispatches.clone();
        let reports = std::sync::Arc::new(std::sync::Mutex::new(Vec::new()));
        let captures = reports.clone();
        let thread = std::thread::spawn(move || {
            let mut requests =
                std::collections::BTreeMap::<String, adl::codefriend::server::Submit>::new();
            let mut controls = 0;
            let mut observation_dropped = false;
            while !flag.load(Ordering::SeqCst) {
                let (mut socket, _) = match listener.accept() {
                    Ok(v) => v,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(std::time::Duration::from_millis(5));
                        continue;
                    }
                    Err(e) => panic!("{e}"),
                };
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(std::time::Duration::from_secs(3)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut chunk = [0; 4096];
                let split;
                loop {
                    let n = socket.read(&mut chunk).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&chunk[..n]);
                    if let Some(p) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                        split = p + 4;
                        break;
                    }
                    assert!(bytes.len() < 16384);
                }
                let header = String::from_utf8_lossy(&bytes[..split]).into_owned();
                let mut first = header.lines().next().unwrap().split_whitespace();
                let method = first.next().unwrap();
                let path = first.next().unwrap();
                let size = header
                    .lines()
                    .find_map(|line| {
                        line.to_lowercase()
                            .strip_prefix("content-length:")
                            .and_then(|v| v.trim().parse::<usize>().ok())
                    })
                    .unwrap_or(0);
                assert!(size < 2 * 1024 * 1024);
                while bytes.len() < split + size {
                    let n = socket.read(&mut chunk).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&chunk[..n]);
                }
                let body: serde_json::Value = if size == 0 {
                    json!(null)
                } else {
                    serde_json::from_slice(&bytes[split..split + size]).unwrap()
                };
                let expected_token = if path.starts_with("/v1/operations") {
                    "b".repeat(64)
                } else {
                    "a".repeat(64)
                };
                assert!(header.contains(&format!("Bearer {expected_token}")));
                let mut reply = if method == "GET"
                    && matches!(path, "/v1/agent/publications" | "/v1/agent/journeys")
                {
                    assert_eq!(size, 0);
                    json!({"job": null})
                } else if path == "/v1/agent/poll" {
                    assert_eq!(body["consent_digest"], command.consent_digest);
                    json!({"schema":PROTOCOL,"command":command})
                } else if path == "/v1/agent/revoke" {
                    json!({"agent_id":command.agent_id,"revoked":true})
                } else if path.ends_with("/control") {
                    controls += 1;
                    if ((matches!(scenario, Scenario::LostInitialControl) && controls == 4)
                        || (matches!(scenario, Scenario::LostFinalControl) && controls == 10))
                        && !observation_dropped
                    {
                        observation_dropped = true;
                        continue;
                    }
                    if (matches!(scenario, Scenario::DeleteDuringControl) && controls == 2)
                        || (matches!(scenario, Scenario::DeleteBeforeUpload) && controls == 11)
                    {
                        fs::remove_file(&consent_path).unwrap();
                    }
                    if matches!(scenario, Scenario::ExpireDuringControl) && controls == 2 {
                        clock.fetch_add(61, Ordering::SeqCst);
                    }
                    json!({"schema":PROTOCOL,"agent_id":command.agent_id,"subject":command.subject,"run_id":command.run_id,"cancelled":(matches!(scenario,Scenario::Cancel)&&count.load(Ordering::SeqCst)>0) || (matches!(scenario,Scenario::CancelRedelivery)&& !captures.lock().unwrap().is_empty())})
                } else if method == "PUT" && path.ends_with("/result") {
                    captures.lock().unwrap().push(body.clone());
                    json!({"schema":PROTOCOL,"run_id":command.run_id,"digest":body["digest"]})
                } else if method == "POST" && path == "/v1/operations" {
                    let submit: adl::codefriend::server::Submit =
                        serde_json::from_value(body).unwrap();
                    assert!(matches!(
                        submit.mode,
                        adl::codefriend::server::Mode::LocalModel
                    ));
                    let id = submit.operation_id.clone();
                    count.fetch_add(1, Ordering::SeqCst);
                    if matches!(scenario, Scenario::CachedBeyondObservationDeadline)
                        && count.load(Ordering::SeqCst) == 2
                    {
                        clock.fetch_add(100, Ordering::SeqCst);
                    }
                    if matches!(scenario, Scenario::DeleteConsent) {
                        fs::remove_file(&consent_path).unwrap();
                    }
                    if matches!(scenario, Scenario::ExpireRetention) {
                        clock.fetch_add(61, Ordering::SeqCst);
                    }
                    requests.insert(id.clone(), submit);
                    if matches!(scenario, Scenario::LostModelReply) {
                        continue;
                    }
                    operation(
                        &requests[&id],
                        &command,
                        if matches!(scenario, Scenario::Cancel | Scenario::LostStatusObservation) {
                            "running"
                        } else {
                            "complete"
                        },
                    )
                } else if path.ends_with("/cancel") {
                    let id = path.split('/').nth(3).unwrap();
                    operation(&requests[id], &command, "cancelled")
                } else if path.ends_with("/result") {
                    let id = path.split('/').nth(3).unwrap();
                    if matches!(
                        scenario,
                        Scenario::LostResultObservation
                            | Scenario::MissingOriginalStore
                            | Scenario::TamperedOriginalStore
                            | Scenario::CachedBeyondObservationDeadline
                    ) && count.load(Ordering::SeqCst) == 2
                        && !observation_dropped
                    {
                        observation_dropped = true;
                        if matches!(scenario, Scenario::CachedBeyondObservationDeadline) {
                            clock.fetch_add(30, Ordering::SeqCst);
                        }
                        continue;
                    }
                    let r = &requests[id];
                    if let Some(plan) = &r.cycle {
                        let admission = adl::codefriend::evidence::Admission::new(
                            r.packet.clone(),
                            adl::codefriend::evidence::Retention { seconds: 60 },
                            clock.load(Ordering::SeqCst),
                        )
                        .unwrap();
                        let route = "agent-logic-fixture:hosted_api:fixture-model-v1";
                        let cycle = adl::codefriend::activities::run_with_executor(
                            plan.clone(),
                            admission.clone(),
                            id.into(),
                            route.into(),
                            None,
                            |activity, _, _| {
                                let (path, kind, content) = match activity {
                                    adl::codefriend::activities::Activity::Documentation =>
                                        ("docs/guide.md", "documentation", "# Guide"),
                                    adl::codefriend::activities::Activity::Diagrams =>
                                        ("docs/system.mmd", "mermaid_diagram", "flowchart LR\nA-->B"),
                                    adl::codefriend::activities::Activity::Tests =>
                                        ("tests/answer.rs", "test", "assert_eq!(answer(), 42);"),
                                    adl::codefriend::activities::Activity::Review => unreachable!(),
                                };
                                Ok(adl::codefriend::activities::ProviderOutput {
                                    final_status: adl::provider_communication::ProviderInvocationFinalStatusV1::Ok,
                                    output_text: Some(if matches!(scenario, Scenario::CycleFailure) {
                                        "{\"schema\":\"wrong\"}".into()
                                    } else { json!({
                                        "schema":"codefriend.activity_output.v1",
                                        "artifacts":[{"path":path,"kind":kind,"content":content,"evidence_paths":["src/lib.rs"],"limitations":["proposal only"]}],
                                        "gaps":[],"measured_coverage_percent":null
                                    }).to_string() }),
                                })
                            },
                        )
                        .unwrap();
                        json!({"schema":"codefriend.local_cycle_result.v1","execution_location":"local_agent","model_execution_location":"agent_logic_provider","candidate_revision":"c".repeat(40),"model_identity":{"provider_kind":"openai","provider":"agent-logic-fixture","model_ref":"fixture/exact","provider_model_id":"fixture-model-v1","runtime_surface":"hosted_api","identity_strength":"provider_asserted","observed_at":format!("unix:{}", clock.load(Ordering::SeqCst))},"cycle_result":cycle})
                    } else {
                        let lane = r.lane.unwrap().id();
                        let findings = if matches!(scenario, Scenario::AggregateLimit) {
                            let admission = adl::codefriend::evidence::Admission::new(
                                r.packet.clone(),
                                adl::codefriend::evidence::Retention { seconds: 60 },
                                live_now(),
                            )
                            .unwrap();
                            json!([{"rule":format!("{lane}.aggregate"),"semantic_anchor":"src/lib.rs","title":"x".repeat(700_000),"severity":"info","rationale":"y".repeat(700_000),"confidence":{"state":"known","percent":90},"evidence":[admission.evidence[0].id],"inference":"bounded fixture","limitations":[]}])
                        } else {
                            json!([])
                        };
                        json!({"schema":"codefriend.local_model_result.v1","execution_location":"local_agent","model_execution_location":"agent_logic_provider","candidate_revision":"c".repeat(40),"model_identity":{"provider_kind":"openai","provider":"agent-logic-fixture","model_ref":"fixture/exact","provider_model_id":"fixture-model-v1","runtime_surface":"hosted_api","identity_strength":"provider_asserted","observed_at":format!("unix:{}", clock.load(Ordering::SeqCst))},"input_manifest":{"schema":"codefriend.review_lane_input_manifest.v1","run_id":id,"packet_id":r.packet.packet_id,"admission_digest":"a".repeat(64),"lane":lane,"lane_contract":"codefriend.review_lane.v1","prompt_contract":"codefriend.four_perspective_review_prompt.v1","repository":r.packet.repository,"revision":r.packet.revision,"scope_digest":r.packet.scope_digest,"evidence":[],"peer_result_refs":[],"source_mutation_authority":"none","tool_authority":"none","publication_authority":"none","input_digest":"a".repeat(64)},"output":{"findings":findings}})
                    }
                } else if method == "GET" && path.starts_with("/v1/operations/") {
                    if matches!(scenario, Scenario::LostStatusObservation)
                        && count.load(Ordering::SeqCst) == 2
                        && !observation_dropped
                    {
                        observation_dropped = true;
                        continue;
                    }
                    let id = path.split('/').nth(3).unwrap();
                    operation(&requests[id], &command, "complete")
                } else {
                    panic!("unexpected route {method} {path}")
                };
                if matches!(scenario, Scenario::CandidateAcrossLanes)
                    && count.load(Ordering::SeqCst) >= 2
                    && reply.get("candidate_revision").is_some()
                {
                    reply["candidate_revision"] = "d".repeat(40).into();
                }
                if reply.get("input_manifest").is_some() {
                    if matches!(scenario, Scenario::CandidateResultMismatch) {
                        reply["candidate_revision"] = "d".repeat(40).into();
                    }
                    if matches!(scenario, Scenario::ModelAcrossLanes)
                        && count.load(Ordering::SeqCst) >= 2
                    {
                        reply["model_identity"]["provider_model_id"] = "different-model-v2".into();
                    }
                }
                let response = serde_json::to_vec(&reply).unwrap();
                write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",response.len()).unwrap();
                socket.write_all(&response).unwrap();
            }
        });
        Self {
            origin,
            stop,
            thread: Some(thread),
            dispatches,
            reports,
        }
    }
}
fn operation(
    r: &adl::codefriend::server::Submit,
    command: &Command,
    status: &str,
) -> serde_json::Value {
    serde_json::json!({"operation_id":r.operation_id,"subject":command.subject,"mode":"local_model","request_digest":adl::codefriend::evidence::hash(r).unwrap(),"packet_id":r.packet.packet_id,"source_revision":r.packet.revision,"candidate_revision":"c".repeat(40),"expires_at":live_now()+1000,"status":status,"model_identity":if status == "complete" { serde_json::json!({"provider_kind":"openai","provider":"agent-logic-fixture","model_ref":"fixture/exact","provider_model_id":"fixture-model-v1","runtime_surface":"hosted_api","identity_strength":"provider_asserted","observed_at":format!("unix:{}",live_now())}) } else { serde_json::Value::Null }})
}
fn journey(scenario: Scenario) -> (u64, Vec<serde_json::Value>) {
    let f = Fixture::new();
    let checkout = f.0.join("repo");
    fs::create_dir(&checkout).unwrap();
    fs::create_dir(checkout.join("src")).unwrap();
    fs::write(
        checkout.join("src/lib.rs"),
        "pub fn answer() -> u8 { 42 }\n",
    )
    .unwrap();
    git(&checkout, &["init"]);
    git(
        &checkout,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/example/project.git",
        ],
    );
    git(&checkout, &["add", "."]);
    git(
        &checkout,
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
    let mut c = f.consent();
    c.repository_path = checkout.clone();
    c.revision = git(&checkout, &["rev-parse", "HEAD"]);
    c.expires_at = live_now() + 1000;
    if matches!(scenario, Scenario::PartialScope) {
        c.scope.analysis.push("src/missing.rs".into());
        c.scope.max_files = 2;
    }
    if matches!(scenario, Scenario::CachedBeyondObservationDeadline) {
        c.retention_seconds = 600;
    }
    let mut cmd = command(&c);
    if matches!(scenario, Scenario::Cycle | Scenario::CycleFailure) {
        cmd.cycle = Some(adl::codefriend::activities::UpdateCyclePlan {
            schema: adl::codefriend::activities::PLAN_SCHEMA.into(),
            repository: c.repository.clone(),
            activities: vec![
                adl::codefriend::activities::Activity::Documentation,
                adl::codefriend::activities::Activity::Diagrams,
                adl::codefriend::activities::Activity::Tests,
            ],
            testing: Some(adl::codefriend::activities::TestingGoal {
                mode: adl::codefriend::activities::TestingMode::CloseGaps,
                target: None,
            }),
        });
    }
    cmd.expires_at = live_now()
        + if matches!(scenario, Scenario::ShortDeadline) {
            30
        } else {
            500
        };
    let consent_path = f.0.join("consent.json");
    fs::write(&consent_path, serde_json::to_vec(&c).unwrap()).unwrap();
    fs::set_permissions(&consent_path, fs::Permissions::from_mode(0o600)).unwrap();
    let clock = std::sync::Arc::new(AtomicU64::new(live_now()));
    let server = WireServer::start(cmd.clone(), scenario, consent_path.clone(), clock.clone());
    let mut p = pairing();
    p.origin = server.origin.clone();
    p.expires_at = live_now() + 1000;
    let journal = Journal::open(&f.0.join("state")).unwrap();
    journal.save_pairing(&p, live_now()).unwrap();
    let clock_fn = clock.clone();
    let transport = Transport::loopback_fixture_with_clock(
        &server.origin,
        std::sync::Arc::new(move || clock_fn.load(Ordering::SeqCst)),
    )
    .unwrap();
    if matches!(scenario, Scenario::Interrupted) {
        let dir = journal.reserve(&cmd, &p, &c, live_now()).unwrap();
        fs::write(
            dir.join("expires.json"),
            serde_json::to_vec(&(live_now() + 60)).unwrap(),
        )
        .unwrap();
    }
    let journal = if matches!(scenario, Scenario::Interrupted) {
        drop(journal);
        Journal::open(&f.0.join("state")).unwrap()
    } else {
        journal
    };
    let original_consent_path = fs::canonicalize(&consent_path).unwrap();
    let first = transport.poll_once(&journal, &consent_path);
    let run_dir = f.0.join("state/run-run-one");
    let saved_admission = fs::read(run_dir.join("admission.json")).ok();
    if let Some(bytes) = &saved_admission {
        let binding: serde_json::Value =
            serde_json::from_slice(&fs::read(run_dir.join("local-consent.json")).unwrap()).unwrap();
        assert_eq!(binding["path"], original_consent_path.to_str().unwrap());
        assert_eq!(binding["digest"], cmd.consent_digest);
        let admission: adl::codefriend::evidence::Admission =
            serde_json::from_slice(bytes).unwrap();
        let live_clock = clock.clone();
        let store =
            adl::codefriend::evidence::store::Store::open(&run_dir.join("evidence"), move || {
                live_clock.load(Ordering::SeqCst)
            })
            .unwrap();
        assert_eq!(store.get(&admission.packet.packet_id).unwrap(), admission);
    }
    if let Ok(bytes) = fs::read(run_dir.join("report.json")) {
        let report: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        if report["status"] == "complete" && !matches!(scenario, Scenario::Cycle) {
            let original: serde_json::Value =
                serde_json::from_slice(&fs::read(run_dir.join("work/review/run.json")).unwrap())
                    .unwrap();
            assert_eq!(original, report["result"]);
            assert!(!run_dir.join("work/run.json").exists());
        }
    }
    if matches!(
        scenario,
        Scenario::ExpireRetention | Scenario::ExpireDuringControl
    ) {
        assert!(
            !f.0.join("state/run-run-one/work").exists(),
            "poll_once must purge before returning"
        );
        assert!(
            !f.0.join("state/run-run-one/report.json").exists(),
            "poll_once must purge before returning"
        );
    }
    if matches!(scenario, Scenario::TamperReport) {
        let path = f.0.join("state/run-run-one/report.json");
        let mut v: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        v["status"] = "tampered".into();
        fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
    }
    let reconnecting = matches!(
        scenario,
        Scenario::LostResultObservation
            | Scenario::MissingOriginalStore
            | Scenario::TamperedOriginalStore
            | Scenario::LostStatusObservation
            | Scenario::LostInitialControl
            | Scenario::LostFinalControl
            | Scenario::CachedBeyondObservationDeadline
    );
    if reconnecting {
        assert!(first.is_err());
        assert!(!f.0.join("state/run-run-one/report.json").exists());
        assert_eq!(
            server.dispatches.load(Ordering::SeqCst),
            match scenario {
                Scenario::LostInitialControl => 1,
                Scenario::LostFinalControl => 4,
                _ => 2,
            }
        );
    }
    if matches!(scenario, Scenario::MissingOriginalStore) {
        fs::remove_dir_all(run_dir.join("evidence")).unwrap();
    } else if matches!(scenario, Scenario::TamperedOriginalStore) {
        let admission: adl::codefriend::evidence::Admission =
            serde_json::from_slice(saved_admission.as_ref().unwrap()).unwrap();
        fs::write(
            run_dir
                .join("evidence")
                .join(format!("{}.json", admission.packet.packet_id)),
            b"{}",
        )
        .unwrap();
    }
    // Release the process-owned lock and re-open persisted state before reconnect.
    drop(journal);
    let journal = Journal::open(&f.0.join("state")).unwrap();
    let second = transport.poll_once(&journal, &consent_path);
    if matches!(
        scenario,
        Scenario::Cancel
            | Scenario::DeleteConsent
            | Scenario::ExpireRetention
            | Scenario::DeleteDuringControl
            | Scenario::ExpireDuringControl
            | Scenario::DeleteBeforeUpload
    ) {
        assert!(first.is_err());
        assert!(second.is_err());
    } else if matches!(
        scenario,
        Scenario::CancelRedelivery | Scenario::TamperReport
    ) {
        first.unwrap();
        assert!(second.is_err());
    } else if reconnecting {
        second.unwrap();
        transport.poll_once(&journal, &consent_path).unwrap();
    } else {
        first.unwrap();
        second.unwrap();
    }
    let reports = server.reports.lock().unwrap().clone();
    if matches!(
        scenario,
        Scenario::Cancel
            | Scenario::DeleteConsent
            | Scenario::ExpireRetention
            | Scenario::DeleteDuringControl
            | Scenario::ExpireDuringControl
            | Scenario::DeleteBeforeUpload
    ) {
        assert!(reports.is_empty());
    } else if matches!(
        scenario,
        Scenario::CancelRedelivery | Scenario::TamperReport
    ) {
        assert_eq!(reports.len(), 1);
    } else {
        assert_eq!(reports.len(), 2);
        assert_eq!(reports[0], reports[1]);
    }
    if matches!(scenario, Scenario::MissingOriginalStore) {
        assert!(
            !run_dir.join("evidence").exists(),
            "missing owner must not be recreated"
        );
    }
    if let Some(bytes) = &saved_admission {
        assert_eq!(fs::read(run_dir.join("admission.json")).unwrap(), *bytes);
        if !matches!(
            scenario,
            Scenario::MissingOriginalStore | Scenario::TamperedOriginalStore
        ) {
            let admission: adl::codefriend::evidence::Admission =
                serde_json::from_slice(bytes).unwrap();
            let live_clock = clock.clone();
            let store = adl::codefriend::evidence::store::Store::open(
                &run_dir.join("evidence"),
                move || live_clock.load(Ordering::SeqCst),
            )
            .unwrap();
            assert_eq!(store.get(&admission.packet.packet_id).unwrap(), admission);
        }
    }
    assert_eq!(git(&checkout, &["status", "--porcelain"]), "");
    if matches!(scenario, Scenario::ShortDeadline) {
        for report in &reports {
            assert_eq!(report["expires_at"].as_u64(), Some(cmd.expires_at));
        }
    }
    let calls = server.dispatches.load(Ordering::SeqCst);
    if matches!(scenario, Scenario::Unpair) {
        let dir = f.0.join("state/run-run-one");
        assert!(dir.join("work").exists());
        assert!(dir.join("report.json").exists());
        transport.unpair(&journal).unwrap();
        assert!(!dir.join("work").exists());
        assert!(!dir.join("evidence").exists());
        assert!(!dir.join("local-consent.json").exists());
        assert!(!dir.join("report.json").exists());
        assert!(dir.join("command.json").exists());
        assert!(dir.join("expires.json").exists());
        assert!(journal.reserve(&cmd, &p, &c, live_now()).is_err());
        assert!(!f.0.join("state/pairing.json").exists());
    }
    clock.store(live_now() + 2000, Ordering::SeqCst);
    journal.expire(clock.load(Ordering::SeqCst)).unwrap();
    assert!(!f.0.join("state/run-run-one/work").exists());
    assert!(!f.0.join("state/run-run-one/evidence").exists());
    assert!(!f.0.join("state/run-run-one/local-consent.json").exists());
    assert!(!f.0.join("state/run-run-one/report.json").exists());
    assert!(transport.poll_once(&journal, &consent_path).is_err());
    assert_eq!(server.dispatches.load(Ordering::SeqCst), calls);
    (calls, reports)
}
#[test]
fn local_orchestration_uses_four_gateway_lanes_and_redelivery_never_dispatches() {
    let (calls, reports) = journey(Scenario::Success);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
    assert_eq!(reports[0]["execution_location"], "local_agent");
    assert_eq!(reports[0]["result"]["completion"], "complete");
    let identities = reports[0]["gateway_lanes"].as_array().unwrap();
    assert_eq!(identities.len(), 4);
    for identity in identities {
        assert_eq!(identity["candidate_revision"], "c".repeat(40));
        assert_eq!(
            identity["model_identity"]["provider_model_id"],
            "fixture-model-v1"
        );
    }
}
#[test]
fn local_update_cycle_uses_one_durable_gateway_operation_and_preserves_activity_results() {
    let (calls, reports) = journey(Scenario::Cycle);
    assert_eq!(calls, 1);
    assert_eq!(reports[0]["status"], "complete");
    assert!(reports[0]["result"].is_null());
    assert_eq!(reports[0]["gateway_lanes"][0]["lane"], "cycle");
    assert_eq!(
        reports[0]["cycle_result"]["activities"]
            .as_array()
            .unwrap()
            .len(),
        3
    );
}

#[test]
fn failed_cycle_cannot_be_uploaded_as_a_complete_report() {
    let (calls, reports) = journey(Scenario::CycleFailure);
    assert_eq!(calls, 1);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
    assert!(reports[0]["cycle_result"].is_null());
}

#[test]
fn native_report_verifier_rejects_rehashed_malformed_cycle_content() {
    let (_, reports) = journey(Scenario::Cycle);
    let mut report: adl::codefriend::agent::RunReport =
        serde_json::from_value(reports[0].clone()).unwrap();
    report.validate(live_now()).unwrap();
    let activity = &mut report.cycle_result.as_mut().unwrap().activities[0];
    let output = activity.output.as_mut().unwrap();
    output.artifacts[0].evidence_paths.clear();
    activity.output_digest = Some(adl::codefriend::evidence::hash(output).unwrap());
    report.digest.clear();
    report.digest = adl::codefriend::evidence::hash(&report).unwrap();
    assert!(report.validate(live_now()).is_err());
}
#[test]
fn lost_model_reply_is_terminal_and_never_replayed() {
    let (calls, reports) = journey(Scenario::LostModelReply);
    assert_eq!(calls, 1);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
    assert!(reports[0]["result"].is_null());
}
#[test]
fn website_cancellation_prevents_success_and_remaining_model_lanes() {
    let (calls, reports) = journey(Scenario::Cancel);
    assert_eq!(calls, 1);
    assert!(reports.is_empty());
}

#[test]
fn deleting_consent_during_lane_stops_further_dispatch_and_upload() {
    let (calls, reports) = journey(Scenario::DeleteConsent);
    assert_eq!(calls, 1);
    assert!(reports.is_empty());
}
#[test]
fn retention_deadline_stops_dispatch_and_cleans_even_single_poll() {
    let (calls, reports) = journey(Scenario::ExpireRetention);
    assert_eq!(calls, 1);
    assert!(reports.is_empty());
}
#[test]
fn cancelled_redelivery_does_not_reupload_complete_artifact() {
    let (calls, reports) = journey(Scenario::CancelRedelivery);
    assert_eq!(calls, 4);
    assert_eq!(reports.len(), 1);
}
#[test]
fn expired_pairing_can_be_forgotten_and_replaced_without_remote_claim() {
    let f = Fixture::new();
    let journal = Journal::open(&f.0.join("state")).unwrap();
    let p = pairing();
    journal.save_pairing(&p, 100).unwrap();
    assert!(journal.pairing(600).is_err());
    journal.forget_pairing().unwrap();
    let mut new = p;
    new.expires_at = 1000;
    journal.save_pairing(&new, 600).unwrap();
    assert!(journal.pairing(600).is_ok());
}

#[test]
fn consent_deleted_during_control_prevents_model_post() {
    let (calls, reports) = journey(Scenario::DeleteDuringControl);
    assert_eq!(calls, 0);
    assert!(reports.is_empty());
}
#[test]
fn retention_expiring_during_control_prevents_model_post() {
    let (calls, reports) = journey(Scenario::ExpireDuringControl);
    assert_eq!(calls, 0);
    assert!(reports.is_empty());
}
#[test]
fn consent_deleted_during_final_control_prevents_result_put() {
    let (calls, reports) = journey(Scenario::DeleteBeforeUpload);
    assert_eq!(calls, 4);
    assert!(reports.is_empty());
}
#[test]
fn retained_report_corruption_is_not_forwarded() {
    let (calls, reports) = journey(Scenario::TamperReport);
    assert_eq!(calls, 4);
    assert_eq!(reports.len(), 1);
}
#[test]
fn cli_restart_purges_expired_payload_before_rejecting_expired_pairing() {
    let f = Fixture::new();
    let root = f.0.join("state");
    let journal = Journal::open(&root).unwrap();
    let c = f.consent();
    let p = pairing();
    journal.save_pairing(&p, 100).unwrap();
    let dir = journal.reserve(&command(&c), &p, &c, 100).unwrap();
    fs::write(dir.join("expires.json"), "200").unwrap();
    fs::create_dir(dir.join("work")).unwrap();
    fs::write(dir.join("work/source"), "private fixture").unwrap();
    fs::write(dir.join("report.json"), "private fixture").unwrap();
    drop(journal);
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_codefriend-agent"))
        .args(["once", "--store"])
        .arg(&root)
        .args(["--consent", "missing"])
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(!dir.join("work").exists());
    assert!(!dir.join("report.json").exists());
    assert!(!String::from_utf8_lossy(&out.stderr).contains(&p.agent_token));
}

#[test]
fn interrupted_reservation_reports_failure_without_new_dispatch() {
    let (calls, reports) = journey(Scenario::Interrupted);
    assert_eq!(calls, 0);
    assert_eq!(reports.len(), 2);
    assert_eq!(reports[0]["status"], "interrupted");
    assert!(reports[0]["result"].is_null());
}

#[test]
fn serialized_reports_honor_shorter_website_command_deadlines() {
    let (calls, reports) = journey(Scenario::ShortDeadline);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}

#[test]
fn website_verifier_checks_native_report_digest_and_complete_contract() {
    let (_, reports) = journey(Scenario::Success);
    let report: adl::codefriend::agent::RunReport =
        serde_json::from_value(reports[0].clone()).unwrap();
    report.validate(live_now()).unwrap();
    let mut altered = reports[0].clone();
    altered["digest"] = "b".repeat(64).into();
    assert!(
        serde_json::from_value::<adl::codefriend::agent::RunReport>(altered)
            .unwrap()
            .validate(live_now())
            .is_err()
    );
    let mut extended: adl::codefriend::agent::RunReport =
        serde_json::from_value(reports[0].clone()).unwrap();
    extended.expires_at += 30;
    extended.digest.clear();
    extended.digest = adl::codefriend::evidence::hash(&extended).unwrap();
    assert!(extended.validate(live_now()).is_err());
    let mut missing = reports[0].clone();
    missing["result"] = serde_json::json!({"schema":"codefriend.four_perspective_review_run.v1","run_id":"run-one","completion":"complete"});
    assert!(serde_json::from_value::<adl::codefriend::agent::RunReport>(missing).is_err());
    let mut incomplete: adl::codefriend::agent::RunReport =
        serde_json::from_value(reports[0].clone()).unwrap();
    incomplete.result.as_mut().unwrap().lane_results.pop();
    incomplete.digest.clear();
    incomplete.digest = adl::codefriend::evidence::hash(&incomplete).unwrap();
    assert!(incomplete.validate(live_now()).is_err());
}

#[test]
fn confirmed_unpair_scrubs_payloads_and_preserves_no_replay_tombstone() {
    let (calls, _) = journey(Scenario::Unpair);
    assert_eq!(calls, 4);
}
#[test]
fn aggregate_four_lane_budget_never_persists_unforwardable_completion() {
    let (calls, reports) = journey(Scenario::AggregateLimit);
    assert_eq!(calls, 4);
    for report in reports {
        assert_eq!(report["status"], "failed_or_interrupted");
        assert!(report["result"].is_null());
        assert!(serde_json::to_vec(&report).unwrap().len() <= 4 * 1024 * 1024);
    }
}

#[test]
fn restart_resumes_known_result_observation_without_reposting_any_lane() {
    let (calls, reports) = journey(Scenario::LostResultObservation);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}
#[test]
fn restart_resumes_known_status_observation_without_reposting_any_lane() {
    let (calls, reports) = journey(Scenario::LostStatusObservation);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}

#[test]
fn reconnect_preserves_known_work_after_initial_lane_control_disconnect() {
    let (calls, reports) = journey(Scenario::LostInitialControl);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}
#[test]
fn reconnect_preserves_completed_work_after_final_control_disconnect() {
    let (calls, reports) = journey(Scenario::LostFinalControl);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}

#[test]
fn completed_lane_cache_survives_its_observation_timeout_during_later_lane_reconnect() {
    let (calls, reports) = journey(Scenario::CachedBeyondObservationDeadline);
    assert_eq!(calls, 4);
    assert_eq!(reports[0]["status"], "complete");
}

#[test]
fn gateway_result_must_match_acknowledged_candidate() {
    let (calls, reports) = journey(Scenario::CandidateResultMismatch);
    assert_eq!(calls, 1);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
}
#[test]
fn gateway_candidate_must_remain_consistent_across_lanes() {
    let (calls, reports) = journey(Scenario::CandidateAcrossLanes);
    assert_eq!(calls, 2);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
}
#[test]
fn gateway_actual_model_must_remain_consistent_across_lanes() {
    let (calls, reports) = journey(Scenario::ModelAcrossLanes);
    assert_eq!(calls, 2);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
}

#[test]
fn partial_scope_is_rejected_before_any_model_dispatch() {
    let (calls, reports) = journey(Scenario::PartialScope);
    assert_eq!(calls, 0);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
    assert!(reports[0]["result"].is_null());
}

#[test]
fn missing_original_store_never_readmits_or_dispatches_on_reconnect() {
    let (calls, reports) = journey(Scenario::MissingOriginalStore);
    assert_eq!(calls, 2);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
    assert!(reports[0]["result"].is_null());
}

#[test]
fn tampered_original_store_never_dispatches_on_reconnect() {
    let (calls, reports) = journey(Scenario::TamperedOriginalStore);
    assert_eq!(calls, 2);
    assert_eq!(reports[0]["status"], "failed_or_interrupted");
    assert!(reports[0]["result"].is_null());
}
