//! PVF agent-publication/PVF.json: deterministic native prepared-stage verification.
//! Synthetic transport/model identities; real generated native review/bundle outputs.
use adl::codefriend::{
    agent::{
        publication::{
            verify_stage, Binding, Prepared, PreparedNative, Stage, VerificationContext,
        },
        GatewayLaneIdentity, RunReport, PROTOCOL,
    },
    evidence::{contracts::ReviewRecord, hash, Admission},
    integration::{prepare_publication_bundle_for_format, PublicationChallenge, PublicationFormat},
    review::{
        lanes::ReviewLane,
        runner::{run_with_executor, ExecutionOptions, LaneExecution},
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

struct Fixture {
    _temp: tempfile::TempDir,
    stage: Stage,
    context: VerificationContext,
}
impl Fixture {
    fn new(format: PublicationFormat) -> Self {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let previous: ReviewRecord = serde_json::from_slice(include_bytes!(
            "fixtures/codefriend/evidence/review-v1.json"
        ))
        .unwrap();
        let mut retention = previous.admission.retention.clone();
        retention.seconds = 550;
        let admission = Admission::new(previous.admission.packet, retention, 100).unwrap();
        let model:adl::model_identity::ModelIdentityV1=serde_json::from_value(json!({"provider_kind":"openai","provider":"fixture","model_ref":"fixture/exact","provider_model_id":"fixture-v1","runtime_surface":"hosted_api","identity_strength":"provider_asserted","observed_at":"unix:100"})).unwrap();
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
        let result = run_with_executor(
            ExecutionOptions {
                out: temp.path().join("review"),
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
            agent_id: "agent1".into(),
            subject: "user1".into(),
            run_id: "run1".into(),
            consent_digest: "d".repeat(64),
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
            expires_at: 650,
            result: Some(result),
            digest: String::new(),
        };
        report.digest = hash(&report).unwrap();
        report.validate(101).unwrap();
        let binding = Binding {
            schema: "codefriend.agent_publication.v1".into(),
            job_id: "job1".into(),
            subject: report.subject.clone(),
            agent_id: report.agent_id.clone(),
            run_id: report.run_id.clone(),
            report_digest: report.digest.clone(),
            received_digest: "e".repeat(64),
            consent_digest: report.consent_digest.clone(),
            format,
            expires_at: report.expires_at,
        };
        let review = &report.result.as_ref().unwrap().review_record;
        // Agent protocol intentionally saves compact typed review snapshots.
        let path = temp.path().join("review-record.json");
        fs::write(&path, serde_json::to_vec(review).unwrap()).unwrap();
        let dest = temp.path().join("exports");
        fs::create_dir(&dest).unwrap();
        let bundle = temp.path().join("bundle");
        let publication =
            prepare_publication_bundle_for_format(&path, &bundle, &dest, format).unwrap();
        let challenge = PublicationChallenge::prepare(
            "user1",
            "job1",
            &"a".repeat(40),
            review,
            &publication,
            &bundle.join("artifacts"),
            None,
            101,
            401,
        )
        .unwrap();
        let p = Prepared {
            challenge_digest: challenge.digest().into(),
            binding_digest: publication.binding_digest().unwrap(),
            expected_decision_digest: None,
            issued_at: 101,
            expires_at: 401,
            native: PreparedNative {
                schema: "codefriend.agent_publication_prepared.v1".into(),
                challenge,
            },
        };
        let mut stage = Stage {
            schema: "codefriend.agent_publication_stage.v1".into(),
            stage: "prepared".into(),
            binding: binding.clone(),
            agent_candidate_revision: "a".repeat(40),
            payload: serde_json::to_value(p).unwrap(),
            exports: vec![],
            digest: String::new(),
        };
        seal(&mut stage);
        let context = VerificationContext {
            schema: "codefriend.agent_publication_verifier_context.v1".into(),
            binding,
            report,
            decision: None,
            prepared: None,
            now: 101,
        };
        Self {
            _temp: temp,
            stage,
            context,
        }
    }
}
#[test]
fn native_generated_preparation_validates_three_formats_without_granting_approval() {
    for format in [
        PublicationFormat::Markdown,
        PublicationFormat::Html,
        PublicationFormat::Pdf,
    ] {
        let f = Fixture::new(format);
        verify_stage(&f.stage, &f.context, 101).unwrap();
        assert!(f.stage.exports.is_empty());
        assert!(f.context.decision.is_none());
        assert!(verify_stage(&f.stage, &f.context, 401).is_err());
        assert!(verify_stage(&f.stage, &f.context, 650).is_err());
    }
}
#[test]
fn recomputed_transport_hash_does_not_authorize_changed_native_challenge_or_owner() {
    let f = Fixture::new(PublicationFormat::Markdown);
    for path in ["challenge_digest", "binding_digest"] {
        let mut changed = f.stage.clone();
        changed.payload[path] = json!("b".repeat(64));
        seal(&mut changed);
        assert!(verify_stage(&changed, &f.context, 101).is_err(), "{path}");
    }
    let mut changed = f.stage.clone();
    changed.agent_candidate_revision = "f".repeat(40);
    seal(&mut changed);
    assert!(verify_stage(&changed, &f.context, 101).is_err());
    let mut changed = f.stage.clone();
    changed.binding.subject = "other".into();
    seal(&mut changed);
    assert!(verify_stage(&changed, &f.context, 101).is_err());
    let mut changed = f.stage.clone();
    changed.payload["native"]["challenge"]["publication"]["claims"] = json!(["Forged claim"]);
    seal(&mut changed);
    assert!(verify_stage(&changed, &f.context, 101).is_err());
}
#[test]
fn prepared_receipt_cannot_be_relabelled_complete_or_bypass_owned_report_validation() {
    let mut f = Fixture::new(PublicationFormat::Html);
    f.context
        .report
        .result
        .as_mut()
        .unwrap()
        .review_record
        .run
        .provider_route = "forged".into();
    assert!(verify_stage(&f.stage, &f.context, 101).is_err());
    let f = Fixture::new(PublicationFormat::Html);
    let mut changed = f.stage.clone();
    changed.stage = "terminal".into();
    seal(&mut changed);
    assert!(verify_stage(&changed, &f.context, 101).is_err());
    let mut changed = f.stage.clone();
    changed.payload["native"]["additional_authority"] = json!(true);
    seal(&mut changed);
    assert!(verify_stage(&changed, &f.context, 101).is_err());
}

#[test]
fn shared_builder_preserves_all_thirteen_original_owner_bytes_for_snapshot_encodings() {
    use adl::codefriend::{
        actions::{remediation, test_plan},
        review::synthesis,
    };
    for pretty in [false, true] {
        let f = Fixture::new(PublicationFormat::Markdown);
        let review = &f.context.report.result.as_ref().unwrap().review_record;
        let input = f._temp.path().join("comparison-review.json");
        let mut bytes = if pretty {
            serde_json::to_vec_pretty(review).unwrap()
        } else {
            serde_json::to_vec(review).unwrap()
        };
        if pretty {
            bytes.push(b'\n');
        }
        fs::write(&input, &bytes).unwrap();
        let original = f._temp.path().join("original-owners");
        fs::create_dir(&original).unwrap();
        synthesis::synthesize_from_file(synthesis::SynthesisOptions {
            input: input.clone(),
            out: original.join("synthesis"),
        })
        .unwrap();
        remediation::plan_from_file(remediation::RemediationOptions {
            input: original.join("synthesis/synthesis.json"),
            out: original.join("remediation"),
        })
        .unwrap();
        test_plan::plan_from_file(test_plan::TestPlanOptions {
            input: original.join("synthesis/synthesis.json"),
            out: original.join("tests"),
        })
        .unwrap();
        let destination = f._temp.path().join("comparison-exports");
        fs::create_dir(&destination).unwrap();
        let bundle = f._temp.path().join("comparison-bundle");
        let publication = prepare_publication_bundle_for_format(
            &input,
            &bundle,
            &destination,
            PublicationFormat::Markdown,
        )
        .unwrap();
        assert_eq!(publication.artifact_manifest.len(), 13);
        for artifact in &publication.artifact_manifest {
            assert_eq!(
                fs::read(original.join(&artifact.path)).unwrap(),
                fs::read(bundle.join("artifacts").join(&artifact.path)).unwrap(),
                "{}",
                artifact.path
            );
        }
        assert_eq!(
            fs::read(bundle.join("artifacts/synthesis/review-record.json")).unwrap(),
            bytes
        );
    }
}

mod transport_tests {
    use super::*;
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
        calls: Arc<Mutex<Vec<String>>>,
        stop: Arc<AtomicBool>,
        server: Option<thread::JoinHandle<()>>,
    }
    impl Case {
        fn new(mutation: &str, control_number: u64, acknowledged: bool) -> Self {
            Self::with_format(
                mutation,
                control_number,
                acknowledged,
                PublicationFormat::Markdown,
            )
        }
        fn with_format(
            mutation: &str,
            control_number: u64,
            acknowledged: bool,
            format: PublicationFormat,
        ) -> Self {
            let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
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
                digest: String::new(),
            };
            report.digest = hash(&report).unwrap();
            report.validate(100).unwrap();
            private(&dir.join("report.json"), &report);
            let receipt = json!({"schema":"codefriend.agent_report_receipt.v1","subject":"user1","agent_id":"agent1","run_id":"run1","report_digest":report.digest,"received_digest":"d".repeat(64),"consent_digest":report.consent_digest,"expires_at":160});

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
                expires_at: 160,
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
                100,
                160,
            )
            .unwrap();
            let payload = Prepared {
                challenge_digest: challenge.digest().into(),
                binding_digest: publication.binding_digest().unwrap(),
                expected_decision_digest: None,
                issued_at: 100,
                expires_at: 160,
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
            let calls = Arc::new(Mutex::new(Vec::new()));
            let seen = calls.clone();
            let stop = Arc::new(AtomicBool::new(false));
            let stopping = stop.clone();
            let consent_path = temp.path().join("consent.json");
            let pairing_path = temp.path().join("state/pairing.json");
            let mutation = mutation.to_owned();
            let time = clock.clone();
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
                    } else if first.starts_with("GET /v1/agent/runs/run1/control ") {
                        assert_eq!(count, 0);
                        json!({"schema":PROTOCOL,"agent_id":"agent1","subject":"user1","run_id":"run1","cancelled":false})
                    } else if first.starts_with("GET /v1/agent/runs/run1/receipt ") {
                        receipt.clone()
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
                                    time.store(160, Ordering::SeqCst);
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
                calls,
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
    #[test]
    fn publication_job_observation_rechecks_local_authority_before_upload_and_ack() {
        assert_eq!(env!("CODEFRIEND_BUILD_CLEAN"),"true","This public-path regression requires a committed clean candidate; no dirty guard bypass");
        for mutation in ["consent", "pairing", "expiry"] {
            for acknowledged in [false, true] {
                let case = Case::new(mutation, 1, acknowledged);
                assert!(
                    case.poll().is_err(),
                    "{mutation} acknowledged={acknowledged}"
                );
                assert_eq!(
                    case.posts(),
                    0,
                    "authority changed during job GET must prevent stage upload"
                );
            }
            let case = Case::new(mutation, 2, false);
            assert!(
                case.poll().is_err(),
                "post-upload readback must not claim current authority after {mutation}"
            );
            assert_eq!(
                case.posts(),
                1,
                "earlier authorized upload is not replayed or retroactively undone"
            );
        }
        let case = Case::new("none", 0, false);
        assert_eq!(case.poll().unwrap().as_deref(), Some("run1"));
        assert_eq!(case.posts(), 1);
        assert!(case.state.lock().unwrap()["prepared_digest"]
            .as_str()
            .is_some());
    }
    #[test]
    fn public_local_relay_renders_three_real_formats_and_preserves_withheld_decision() {
        assert_eq!(
            env!("CODEFRIEND_BUILD_CLEAN"),
            "true",
            "public relay proof requires committed candidate"
        );
        for (format, decision) in [
            (PublicationFormat::Markdown, "approved"),
            (PublicationFormat::Html, "approved"),
            (PublicationFormat::Pdf, "approved"),
            (PublicationFormat::Markdown, "withheld"),
        ] {
            let case = Case::with_format("none", 0, false, format);
            // Exercise fresh native preparation rather than replaying the fixture's
            // ready stage used by the separate forwarding-race regression.
            fs::remove_dir_all(
                case.temp
                    .path()
                    .join("state/run-run1/work/publications/job1"),
            )
            .unwrap();
            assert_eq!(case.poll().unwrap().as_deref(), Some("run1"));
            {
                let mut job = case.state.lock().unwrap();
                job["decision"] = json!({"challenge_digest":job["prepared"]["challenge_digest"],"binding_digest":job["prepared"]["binding_digest"],"expected_decision_digest":job["prepared"]["expected_decision_digest"],"decision":decision});
                job["status"] = json!("decision_pending");
            }
            assert_eq!(case.poll().unwrap().as_deref(), Some("run1"));
            let run = case.temp.path().join("state/run-run1");
            let stage: Stage = serde_json::from_slice(
                &fs::read(run.join("work/publications/job1/terminal-stage.json")).unwrap(),
            )
            .unwrap();
            let report: RunReport =
                serde_json::from_slice(&fs::read(run.join("report.json")).unwrap()).unwrap();
            let prepared: Stage = serde_json::from_slice(
                &fs::read(run.join("work/publications/job1/prepared-stage.json")).unwrap(),
            )
            .unwrap();
            let requested =
                serde_json::from_value(case.state.lock().unwrap()["decision"].clone()).unwrap();
            let context = VerificationContext {
                schema: "codefriend.agent_publication_verifier_context.v1".into(),
                binding: stage.binding.clone(),
                report,
                decision: Some(requested),
                prepared: Some(prepared),
                now: 100,
            };
            verify_stage(&stage, &context, 100).unwrap();
            assert_eq!(
                stage.payload["native"]["decision"]["schema"],
                "codefriend.publication_decision.v3"
            );
            assert_eq!(
                stage.payload["native"]["decision"]["channel"],
                "authenticated_local_agent"
            );
            assert_eq!(
                stage.payload["status"],
                if decision == "approved" {
                    "complete"
                } else {
                    "withheld"
                }
            );
            assert_eq!(
                stage.exports.len(),
                if decision == "approved" { 1 } else { 0 }
            );
            assert_eq!(
                case.posts(),
                2,
                "one prepared and one terminal upload, no model post"
            );
            if let Some(output) = std::env::var_os("CODEFRIEND_LOCAL_STAGE_OUTPUT") {
                let output = std::path::PathBuf::from(output);
                assert!(output.is_absolute());
                fs::create_dir_all(&output).unwrap();
                private(
                    &output.join(format!("{}-{decision}-stage.json", format.key())),
                    &stage,
                );
                private(
                    &output.join(format!("{}-{decision}-context.json", format.key())),
                    &context,
                );
            }
        }
    }
}
