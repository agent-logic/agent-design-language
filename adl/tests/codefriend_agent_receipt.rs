//! PVF: fixtures/codefriend/agent-receipt/PVF.json; no provider or deployment proof.
use adl::codefriend::{
    agent::{
        Command, Consent, ForwardReceipt, GatewayLaneIdentity, Journal, Pairing, RunReport,
        Transport, PROTOCOL,
    },
    evidence::{contracts::ReviewRecord, hash, Admission},
    review::{
        lanes::ReviewLane,
        runner::{self, ExecutionOptions, LaneExecution},
    },
};
use adl::provider_communication::ProviderInvocationFinalStatusV1;
use serde_json::{json, Value};
use std::{
    fs,
    io::{Read, Write},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::Path,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};

fn private(path: &Path, value: &impl serde::Serialize) {
    fs::write(path, serde_json::to_vec(value).unwrap()).unwrap();
    fs::set_permissions(path, fs::Permissions::from_mode(0o600)).unwrap();
}
struct Case {
    temp: tempfile::TempDir,
    journal: Option<Journal>,
    transport: Transport,
    clock: Arc<AtomicU64>,
    receipt: Value,
    advance_on_receipt: Arc<AtomicU64>,
    responses: Arc<Mutex<Vec<(u16, Value)>>>,
    calls: Arc<Mutex<Vec<String>>>,
    stop: Arc<std::sync::atomic::AtomicBool>,
    server: Option<thread::JoinHandle<()>>,
}
impl Case {
    fn new() -> Self {
        let temp = tempfile::tempdir().unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let origin = format!("http://{}/", listener.local_addr().unwrap());
        let clock = Arc::new(AtomicU64::new(100));
        let time = clock.clone();
        let transport = Transport::loopback_fixture_with_clock(
            &origin,
            Arc::new(move || time.load(Ordering::SeqCst)),
        )
        .unwrap();
        let fixture: ReviewRecord = serde_json::from_slice(include_bytes!(
            "fixtures/codefriend/evidence/review-v1.json"
        ))
        .unwrap();
        let mut retention = fixture.admission.retention.clone();
        retention.seconds = 60;
        let admission = Admission::new(fixture.admission.packet, retention, 100).unwrap();
        let consent = Consent {
            schema: PROTOCOL.into(),
            repository_path: temp.path().into(),
            repository: admission.packet.repository.clone(),
            revision: admission.packet.revision.clone(),
            scope: admission.packet.scope.clone(),
            expires_at: 200,
            retention_seconds: 60,
            allow_model_egress: true,
            allow_result_upload: true,
        };
        private(&temp.path().join("consent.json"), &consent);
        let pairing = Pairing {
            schema: PROTOCOL.into(),
            origin,
            agent_id: "agent1".into(),
            subject: "user1".into(),
            agent_token: "a".repeat(64),
            model_token: "b".repeat(64),
            expires_at: 200,
        };
        let command = Command {
            schema: PROTOCOL.into(),
            agent_id: pairing.agent_id.clone(),
            subject: pairing.subject.clone(),
            run_id: "run1".into(),
            consent_digest: consent.digest().unwrap(),
            expires_at: 180,
            cycle: None,
        };
        let journal = Journal::open(&temp.path().join("state")).unwrap();
        journal.save_pairing(&pairing, 100).unwrap();
        let dir = journal.reserve(&command, &pairing, &consent, 100).unwrap();
        fs::create_dir(dir.join("work")).unwrap();
        private(&dir.join("expires.json"), &160u64);
        let model: adl::model_identity::ModelIdentityV1 = serde_json::from_value(json!({"provider_kind":"openai","provider":"fixture","model_ref":"fixture/exact","provider_model_id":"fixture-v1","runtime_surface":"hosted_api","identity_strength":"provider_asserted","observed_at":"unix:100"})).unwrap();
        let candidate = "c".repeat(40);
        let route = format!(
            "agent_logic_gateway:{}",
            hash(&(
                &candidate,
                &model.provider_kind,
                &model.provider,
                &model.runtime_surface,
                &model.model_ref,
                &model.provider_model_id,
                &model.identity_strength,
                &model.resolved_digest
            ))
            .unwrap()
        );
        let result = runner::run_with_executor(
            ExecutionOptions {
                out: dir.join("work/review"),
                run_id: "run1".into(),
                cancel_file: None,
            },
            admission,
            route,
            |_, _, _| {
                Ok(LaneExecution {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some("{\"findings\":[]}".into()),
                })
            },
        )
        .unwrap();
        let mut report = RunReport {
            schema: PROTOCOL.into(),
            agent_id: pairing.agent_id,
            subject: pairing.subject,
            run_id: "run1".into(),
            consent_digest: command.consent_digest,
            execution_location: "local_agent".into(),
            gateway_lanes: ReviewLane::ALL
                .iter()
                .map(|lane| GatewayLaneIdentity {
                    lane: lane.id().into(),
                    candidate_revision: candidate.clone(),
                    model_identity: model.clone(),
                })
                .collect(),
            status: "complete".into(),
            expires_at: 160,
            result: Some(result),
            cycle_result: None,
            digest: String::new(),
        };
        report.digest = hash(&report).unwrap();
        report.validate(100).unwrap();
        private(&dir.join("report.json"), &report);
        let receipt = json!({"schema":"codefriend.agent_report_receipt.v1","subject":"user1","agent_id":"agent1","run_id":"run1","report_digest":report.digest,"received_digest":"d".repeat(64),"consent_digest":report.consent_digest,"expires_at":160});
        let responses = Arc::new(Mutex::new(Vec::<(u16, Value)>::new()));
        let calls = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let (r, c, s) = (responses.clone(), calls.clone(), stop.clone());
        let advance_on_receipt = Arc::new(AtomicU64::new(0));
        let advance = advance_on_receipt.clone();
        let server_clock = clock.clone();
        let server = thread::spawn(move || {
            while !s.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        stream.set_nonblocking(false).unwrap();
                        stream
                            .set_read_timeout(Some(Duration::from_secs(2)))
                            .unwrap();
                        let mut bytes = Vec::new();
                        let mut b = [0u8; 1];
                        while !bytes.ends_with(b"\r\n\r\n") {
                            stream.read_exact(&mut b).unwrap();
                            bytes.push(b[0]);
                        }
                        let request = String::from_utf8(bytes).unwrap();
                        let length = request
                            .lines()
                            .find_map(|line| {
                                let (name, value) = line.split_once(':')?;
                                name.eq_ignore_ascii_case("content-length")
                                    .then(|| value.trim().parse::<usize>().unwrap())
                            })
                            .unwrap_or(0);
                        let mut body = vec![0; length];
                        stream.read_exact(&mut body).unwrap();

                        assert!(request
                            .to_lowercase()
                            .contains(&format!("authorization: bearer {}", "a".repeat(64))));
                        c.lock()
                            .unwrap()
                            .push(request.lines().next().unwrap().into());
                        if request.starts_with("GET /v1/agent/runs/run1/receipt ")
                            && advance.load(Ordering::SeqCst) != 0
                        {
                            server_clock.store(advance.load(Ordering::SeqCst), Ordering::SeqCst);
                        }
                        let (status, value) = r.lock().unwrap().remove(0);
                        let body = serde_json::to_vec(&value).unwrap();
                        write!(stream,"HTTP/1.1 {status} Reply\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();
                        stream.write_all(&body).unwrap();
                    }
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2))
                    }
                    Err(e) => panic!("{e}"),
                }
            }
        });
        Self {
            temp,
            journal: Some(journal),
            transport,
            clock,
            receipt,
            advance_on_receipt,
            responses,
            calls,
            stop,
            server: Some(server),
        }
    }
    fn queue(&self, receipt: Value) {
        self.responses.lock().unwrap().extend([(200,json!({"schema":PROTOCOL,"subject":"user1","agent_id":"agent1","run_id":"run1","cancelled":false})),(200,receipt)]);
    }
    fn get(&self) -> anyhow::Result<ForwardReceipt> {
        self.transport.publication_receipt(
            self.journal.as_ref().unwrap(),
            "run1",
            &self.temp.path().join("consent.json"),
        )
    }
    fn saved(&self) -> std::path::PathBuf {
        self.temp
            .path()
            .join("state/run-run1/work/forward-receipt.json")
    }
}
impl Drop for Case {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let result = self.server.take().unwrap().join();
        if !thread::panicking() {
            result.unwrap();
        }
    }
}

#[test]
fn missing_ack_recovers_by_get_and_restart_reobserves_exact_receipt() {
    let mut c = Case::new();
    c.queue(c.receipt.clone());
    let first = c.get().unwrap();
    assert!(c.saved().is_file());
    assert_eq!(
        fs::metadata(c.saved()).unwrap().permissions().mode() & 0o777,
        0o600
    );
    drop(c.journal.take());
    c.journal = Some(Journal::open(&c.temp.path().join("state")).unwrap());
    c.queue(c.receipt.clone());
    assert_eq!(c.get().unwrap(), first);
    assert_eq!(
        *c.calls.lock().unwrap(),
        vec![
            "GET /v1/agent/runs/run1/control HTTP/1.1",
            "GET /v1/agent/runs/run1/receipt HTTP/1.1",
            "GET /v1/agent/runs/run1/control HTTP/1.1",
            "GET /v1/agent/runs/run1/receipt HTTP/1.1"
        ]
    );
}
#[test]
fn forged_bindings_and_schema_are_rejected_without_retention() {
    for field in [
        "subject",
        "agent_id",
        "run_id",
        "report_digest",
        "consent_digest",
        "received_digest",
        "schema",
        "expires_at",
        "extra",
    ] {
        let c = Case::new();
        let mut value = c.receipt.clone();
        value[field] = if field == "expires_at" {
            json!(161)
        } else {
            json!("forged")
        };
        c.queue(value);
        assert!(c.get().is_err(), "{field}");
        assert!(!c.saved().exists());
        assert!(c
            .calls
            .lock()
            .unwrap()
            .iter()
            .all(|r| r.starts_with("GET ")));
    }
}
#[test]
fn changed_authenticated_receipt_never_overwrites_retained_binding() {
    let c = Case::new();
    c.queue(c.receipt.clone());
    c.get().unwrap();
    let original = fs::read(c.saved()).unwrap();
    let mut changed = c.receipt.clone();
    changed["received_digest"] = json!("e".repeat(64));
    c.queue(changed);
    assert!(c
        .get()
        .unwrap_err()
        .to_string()
        .contains("agent_receipt_changed"));
    assert_eq!(fs::read(c.saved()).unwrap(), original);
}
#[test]
fn expiry_scrubs_receipt_but_preserves_no_replay_reservation() {
    let c = Case::new();
    c.queue(c.receipt.clone());
    c.get().unwrap();
    c.clock.store(160, Ordering::SeqCst);
    assert!(c.get().is_err());
    assert!(!c.saved().exists());
    assert!(c.temp.path().join("state/run-run1/command.json").exists());
    assert_eq!(c.calls.lock().unwrap().len(), 2);
}
#[test]
fn remote_revocation_and_changed_consent_fail_closed() {
    let c = Case::new();
    c.responses
        .lock()
        .unwrap()
        .push((401, json!({"error":"revoked"})));
    assert!(c.get().is_err());
    assert!(!c.saved().exists());
    let path = c.temp.path().join("consent.json");
    let mut consent: Consent = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    consent.allow_result_upload = false;
    private(&path, &consent);
    assert!(c.get().is_err());
    assert_eq!(c.calls.lock().unwrap().len(), 1);
}
#[test]
fn unpair_scrubs_receipt_and_retains_tombstone() {
    let c = Case::new();
    c.queue(c.receipt.clone());
    c.get().unwrap();
    c.responses
        .lock()
        .unwrap()
        .push((200, json!({"revoked":true,"agent_id":"agent1"})));
    c.transport.unpair(c.journal.as_ref().unwrap()).unwrap();
    assert!(!c.saved().exists());
    assert!(c.temp.path().join("state/run-run1/command.json").exists());
    assert!(c.get().is_err());
}

#[test]
fn cancelled_missing_receipt_and_expired_pairing_do_not_create_receipt() {
    let c = Case::new();
    c.responses.lock().unwrap().push((200, json!({"schema":PROTOCOL,"subject":"user1","agent_id":"agent1","run_id":"run1","cancelled":true})));
    assert!(c.get().is_err());
    assert_eq!(c.calls.lock().unwrap().len(), 1);
    c.responses.lock().unwrap().extend([(200,json!({"schema":PROTOCOL,"subject":"user1","agent_id":"agent1","run_id":"run1","cancelled":false})), (404,json!({"error":"missing"}))]);
    assert!(c.get().is_err());
    assert!(!c.saved().exists());
    let path = c.temp.path().join("state/pairing.json");
    let mut pairing: Pairing = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    pairing.expires_at = 100;
    private(&path, &pairing);
    assert!(c.get().is_err());
    assert_eq!(c.calls.lock().unwrap().len(), 3);
}

#[test]
fn malformed_complete_report_is_rejected_even_with_recomputed_digest() {
    let c = Case::new();
    let path = c.temp.path().join("state/run-run1/report.json");
    let mut report: RunReport = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    report.gateway_lanes.clear();
    report.digest.clear();
    report.digest = hash(&report).unwrap();
    private(&path, &report);
    assert!(c.get().is_err());
    assert!(c.calls.lock().unwrap().is_empty());
    assert!(!c.saved().exists());
}

#[test]
fn expiry_during_receipt_observation_prevents_retention() {
    let c = Case::new();
    c.queue(c.receipt.clone());
    c.advance_on_receipt.store(160, Ordering::SeqCst);
    assert!(c.get().is_err());
    assert!(!c.saved().exists());
    assert_eq!(c.calls.lock().unwrap().len(), 2);
}
