use adl::codefriend::{
    agent::{
        publication::{Binding, Prepared, PreparedNative, Stage},
        GatewayLaneIdentity, RunReport, PROTOCOL,
    },
    evidence::{contracts::ReviewRecord, hash, Admission},
    integration::{prepare_publication_bundle_for_format, PublicationChallenge, PublicationFormat},
    review::{
        lanes::ReviewLane,
        runner::{ExecutionOptions, LaneExecution},
    },
};
use adl::provider_communication::ProviderInvocationFinalStatusV1;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
fn seal(stage: &mut Stage) {
    fn canonical(value: serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let ordered = map
                    .into_iter()
                    .collect::<std::collections::BTreeMap<_, _>>();
                serde_json::Value::Object(
                    ordered
                        .into_iter()
                        .map(|(k, v)| (k, canonical(v)))
                        .collect(),
                )
            }
            serde_json::Value::Array(xs) => {
                serde_json::Value::Array(xs.into_iter().map(canonical).collect())
            }
            value => value,
        }
    }
    let mut value = serde_json::to_value(&stage).unwrap();
    value["digest"] = json!("");
    stage.digest = format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&canonical(value)).unwrap())
    );
}

use adl::codefriend::agent::{Command, Consent, Journal, Pairing, Transport};
use adl::codefriend::review::runner;
use serde_json::Value;
use std::{
    io::{Read, Write},
    net::TcpListener,
    os::unix::fs::PermissionsExt,
    path::Path,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
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
    journal: Journal,
    transport: Transport,
    state: Arc<Mutex<Value>>,
    #[allow(dead_code)] // Shared with the Journey transport regression target.
    journey: Arc<Mutex<Value>>,
    #[allow(dead_code)]
    journey_results: Arc<Mutex<Vec<Value>>>,
    #[allow(dead_code)]
    drop_journey_ack: Arc<AtomicBool>,
    #[allow(dead_code)]
    extra_receipts: Arc<Mutex<std::collections::BTreeMap<String, Value>>>,
    calls: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    server: Option<thread::JoinHandle<()>>,
}
impl Case {
    #[allow(dead_code)] // Shared fixture entrypoint used by the transport integration target.
    fn new(mutation: &str, control_number: u64, acknowledged: bool) -> Self {
        Self::with_format(
            mutation,
            control_number,
            acknowledged,
            PublicationFormat::Markdown,
        )
    }
    #[allow(dead_code)] // Shared fixture entrypoint used by the transport integration target.
    fn with_format(
        mutation: &str,
        control_number: u64,
        acknowledged: bool,
        format: PublicationFormat,
    ) -> Self {
        Self::with_format_at(mutation, control_number, acknowledged, format, 100)
    }
    fn with_format_at(
        mutation: &str,
        control_number: u64,
        acknowledged: bool,
        format: PublicationFormat,
        now: u64,
    ) -> Self {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let origin = format!("http://{}/", listener.local_addr().unwrap());
        let clock = Arc::new(AtomicU64::new(now));
        let time = clock.clone();
        let transport = Transport::loopback_fixture_with_clock(
            &origin,
            Arc::new(move || time.load(Ordering::SeqCst)),
        )
        .unwrap();
        let fixture: ReviewRecord = serde_json::from_slice(include_bytes!(
            "../fixtures/codefriend/evidence/review-v1.json"
        ))
        .unwrap();
        let mut retention = fixture.admission.retention.clone();
        retention.seconds = 60;
        let admission = Admission::new(fixture.admission.packet, retention, now).unwrap();
        let consent = Consent {
            schema: PROTOCOL.into(),
            repository_path: temp.path().into(),
            repository: admission.packet.repository.clone(),
            revision: admission.packet.revision.clone(),
            scope: admission.packet.scope.clone(),
            expires_at: now + 100,
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
            expires_at: now + 100,
        };
        let command = Command {
            schema: PROTOCOL.into(),
            agent_id: pairing.agent_id.clone(),
            subject: pairing.subject.clone(),
            run_id: "run1".into(),
            consent_digest: consent.digest().unwrap(),
            expires_at: now + 80,
            cycle: None,
        };
        let journal = Journal::open(&temp.path().join("state")).unwrap();
        journal.save_pairing(&pairing, now).unwrap();
        let dir = journal.reserve(&command, &pairing, &consent, now).unwrap();
        let store =
            adl::codefriend::evidence::store::Store::open(&dir.join("evidence"), move || now)
                .unwrap();
        let stored = store
            .admit(admission.packet.clone(), admission.retention.clone())
            .unwrap();
        assert_eq!(stored, admission);
        drop(store);
        fs::create_dir(dir.join("work")).unwrap();
        private(&dir.join("expires.json"), &(now + 60));
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
                    request_digest: "e".repeat(64),
                    model_identity: model.clone(),
                })
                .collect(),
            status: "complete".into(),
            expires_at: now + 60,
            cycle_result: None,
            result: Some(result),
            digest: String::new(),
        };
        report.digest = hash(&report).unwrap();
        report.validate(now).unwrap();
        private(&dir.join("report.json"), &report);
        let receipt = json!({"schema":"codefriend.agent_report_receipt.v1","subject":"user1","agent_id":"agent1","run_id":"run1","report_digest":report.digest,"received_digest":"d".repeat(64),"consent_digest":report.consent_digest,"expires_at":now + 60});

        let binding = Binding {
            schema: "codefriend.agent_publication.v1".into(),
            job_id: "job1".into(),
            subject: "user1".into(),
            agent_id: "agent1".into(),
            run_id: "run1".into(),
            report_digest: report.digest.clone(),
            received_digest: "d".repeat(64),
            consent_digest: report.consent_digest.clone(),
            format,
            expires_at: now + 60,
        };
        let publication_root = dir.join("work/publications/job1");
        fs::create_dir_all(&publication_root).unwrap();
        let review = &report.result.as_ref().unwrap().review_record;
        private(&publication_root.join("review-record.json"), review);
        let destination = publication_root.join("exports");
        fs::create_dir(&destination).unwrap();
        let publication = prepare_publication_bundle_for_format(
            &publication_root.join("review-record.json"),
            &publication_root.join("bundle"),
            &destination,
            format,
        )
        .unwrap();
        let candidate = env!("CODEFRIEND_BUILD_REVISION");
        let challenge = PublicationChallenge::prepare(
            "user1",
            "job1",
            candidate,
            review,
            &publication,
            &publication_root.join("bundle/artifacts"),
            None,
            now,
            now + 60,
        )
        .unwrap();
        let payload = Prepared {
            challenge_digest: challenge.digest().into(),
            binding_digest: publication.binding_digest().unwrap(),
            expected_decision_digest: None,
            issued_at: now,
            expires_at: now + 60,
            native: PreparedNative {
                schema: "codefriend.agent_publication_prepared.v1".into(),
                challenge,
            },
        };
        let mut stage = Stage {
            schema: "codefriend.agent_publication_stage.v1".into(),
            stage: "prepared".into(),
            binding: binding.clone(),
            agent_candidate_revision: candidate.into(),
            payload: serde_json::to_value(payload).unwrap(),
            exports: vec![],
            digest: String::new(),
        };
        seal(&mut stage);
        private(&publication_root.join("prepared-stage.json"), &stage);
        let job = json!({"binding":binding,"status":"awaiting_agent","decision":null,"prepared_digest":null,"terminal_digest":null,"effect_unresolved":true,"agent_candidate_revision":null,"prepared":null,"terminal":null});
        let mut controlled = job.clone();
        if acknowledged {
            controlled["prepared_digest"] = json!(stage.digest);
            controlled["status"] = json!("prepared");
        }
        let state = Arc::new(Mutex::new(controlled));
        let web = state.clone();
        let journey = Arc::new(Mutex::new(Value::Null));
        let journey_web = journey.clone();
        let journey_results = Arc::new(Mutex::new(Vec::new()));
        let received_journeys = journey_results.clone();
        let drop_journey_ack = Arc::new(AtomicBool::new(false));
        let drop_ack = drop_journey_ack.clone();
        let calls = Arc::new(Mutex::new(Vec::new()));
        let seen = calls.clone();
        let stop = Arc::new(AtomicBool::new(false));
        let stopping = stop.clone();
        let consent_path = temp.path().join("consent.json");
        let pairing_path = temp.path().join("state/pairing.json");
        let mutation = mutation.to_owned();
        let time = clock.clone();
        let extra_receipts = Arc::new(Mutex::new(
            std::collections::BTreeMap::<String, Value>::new(),
        ));
        let run_receipts = extra_receipts.clone();
        let server = thread::spawn(move || {
            let mut controls = 0;
            while !stopping.load(Ordering::SeqCst) {
                let (mut socket, _) = match listener.accept() {
                    Ok(v) => v,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                        continue;
                    }
                    Err(e) => panic!("{e}"),
                };
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(2)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut buf = [0; 4096];
                let split = loop {
                    let n = socket.read(&mut buf).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buf[..n]);
                    if let Some(p) = bytes.windows(4).position(|x| x == b"\r\n\r\n") {
                        break p + 4;
                    }
                };
                let header = String::from_utf8(bytes[..split].to_vec()).unwrap();
                let first = header.lines().next().unwrap().to_owned();
                assert!(header
                    .to_ascii_lowercase()
                    .contains(&format!("authorization: bearer {}", "a".repeat(64))));
                let count = header
                    .lines()
                    .find_map(|l| {
                        l.to_ascii_lowercase()
                            .strip_prefix("content-length:")
                            .map(|v| v.trim().parse::<usize>().unwrap())
                    })
                    .unwrap_or(0);
                while bytes.len() < split + count {
                    let n = socket.read(&mut buf).unwrap();
                    assert!(n > 0);
                    bytes.extend_from_slice(&buf[..n]);
                }
                seen.lock().unwrap().push(first.clone());
                let response = if first.starts_with("GET /v1/agent/publications HTTP") {
                    {
                        let current = web.lock().unwrap().clone();
                        json!({"job":if acknowledged && controls == 0 {job.clone()} else {current}})
                    }
                } else if first.starts_with("GET /v1/agent/journeys HTTP") {
                    json!({"job":journey_web.lock().unwrap().clone()})
                } else if first.starts_with("GET /v1/agent/journeys/") {
                    journey_web.lock().unwrap().clone()
                } else if first.starts_with("PUT /v1/agent/journeys/") {
                    let posted: Value =
                        serde_json::from_slice(&bytes[split..split + count]).unwrap();
                    received_journeys.lock().unwrap().push(posted.clone());
                    if drop_ack.swap(false, Ordering::SeqCst) {
                        continue;
                    }
                    json!({"binding":posted["binding"],"agent_candidate_revision":posted["agent_candidate_revision"],
                        "checkpoint_sequence":posted["checkpoint_sequence"],"digest":posted["digest"],"superseded":false})
                } else if first.starts_with("GET /v1/agent/runs/run1/control ") {
                    assert_eq!(count, 0);
                    json!({"schema":PROTOCOL,"agent_id":"agent1","subject":"user1","run_id":"run1","cancelled":false})
                } else if first.starts_with("GET /v1/agent/runs/run1/receipt ") {
                    receipt.clone()
                } else if first.starts_with("GET /v1/agent/runs/") {
                    let path = first.split_whitespace().nth(1).unwrap();
                    let suffix = path.strip_prefix("/v1/agent/runs/").unwrap();
                    let (run_id, action) = suffix.split_once('/').unwrap();
                    let receipts = run_receipts.lock().unwrap();
                    let receipt = receipts.get(run_id).expect("unregistered fixture run");
                    match action {
                        "receipt" => receipt.clone(),
                        "control" => {
                            json!({"schema":PROTOCOL,"agent_id":"agent1","subject":"user1","run_id":run_id,"cancelled":false})
                        }
                        _ => panic!("unexpected run action"),
                    }
                } else if first.starts_with("GET /v1/agent/publications/job1 ") {
                    controls += 1;
                    if controls == control_number {
                        match mutation.as_str() {
                            "consent" => {
                                fs::remove_file(&consent_path).unwrap();
                            }
                            "pairing" => {
                                fs::remove_file(&pairing_path).unwrap();
                            }
                            "expiry" => {
                                time.store(now + 60, Ordering::SeqCst);
                            }
                            _ => {}
                        }
                    }
                    web.lock().unwrap().clone()
                } else if first.starts_with("POST /v1/agent/publications/job1/prepared ") {
                    let posted: Value =
                        serde_json::from_slice(&bytes[split..split + count]).unwrap();
                    let mut w = web.lock().unwrap();
                    w["prepared_digest"] = posted["digest"].clone();
                    w["prepared"] = posted["payload"].clone();
                    w["agent_candidate_revision"] = posted["agent_candidate_revision"].clone();
                    w["status"] = json!("prepared");
                    w.clone()
                } else if first.starts_with("POST /v1/agent/publications/job1/terminal ") {
                    let posted: Value =
                        serde_json::from_slice(&bytes[split..split + count]).unwrap();
                    let mut w = web.lock().unwrap();
                    w["terminal_digest"] = posted["digest"].clone();
                    w["status"] = posted["payload"]["status"].clone();
                    w["terminal"] = json!({"status":posted["payload"]["status"],"decision_digest":posted["payload"]["decision_digest"],"reason":posted["payload"]["reason"],
                        "exports":posted["exports"].as_array().unwrap().iter().enumerate().map(|(index,x)|json!({"index":index,"name":x["name"],"media_type":x["media_type"],"digest":x["digest"]})).collect::<Vec<_>>()});
                    w.clone()
                } else {
                    panic!("unexpected route {first}")
                };
                let body = serde_json::to_vec(&response).unwrap();
                write!(socket,"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",body.len()).unwrap();
                socket.write_all(&body).unwrap();
            }
        });
        Self {
            temp,
            journal,
            transport,
            state,
            journey,
            journey_results,
            drop_journey_ack,
            calls,
            extra_receipts,
            stop,
            server: Some(server),
        }
    }
    fn poll(&self) -> anyhow::Result<Option<String>> {
        self.transport
            .poll_once(&self.journal, &self.temp.path().join("consent.json"))
    }
    fn posts(&self) -> usize {
        self.calls
            .lock()
            .unwrap()
            .iter()
            .filter(|s| s.starts_with("POST "))
            .count()
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
