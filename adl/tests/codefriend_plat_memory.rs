//! PVF runtime, deterministic CPU/filesystem fixtures and production CLI. Required acceptance gate.
use adl::codefriend::{
    evidence::{
        contracts::{
            Completion, Confidence, Delta, Finding, ReviewRecord, Run, Severity, CONTRACT,
        },
        store::Store,
        Retention,
    },
    ingestion::{local, Scope},
    memory::{
        baseline::{AdmittedBaselines, BaselineRef},
        palace::{self, IndexRequest, RetrieveRequest},
        palace_authority,
    },
};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
static TEST_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap().trim().into()
}
struct Fixture {
    temp: tempfile::TempDir,
    source: PathBuf,
    store: Option<Store>,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new() -> Self {
        let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = temp.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        fs::write(source.join("LICENSE"), "MIT fixture\n").unwrap();
        fs::write(source.join("helper.rs"), "pub fn helper() {}\n").unwrap();
        let clock = Arc::new(AtomicU64::new(
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        ));
        let c = clock.clone();
        let store =
            Store::open(&temp.path().join("store"), move || c.load(Ordering::SeqCst)).unwrap();
        Self {
            temp,
            source,
            store: Some(store),
            clock,
        }
    }
    fn store(&self) -> &Store {
        self.store.as_ref().unwrap()
    }
    fn baselines(&self) -> PathBuf {
        self.temp.path().join("baselines")
    }
    fn record(
        &self,
        source: &str,
        names: &[(&str, &str)],
        version: &str,
        completion: Completion,
        narrow: bool,
    ) -> ReviewRecord {
        fs::write(self.source.join("lib.rs"), source).unwrap();
        git(&self.source, &["add", "."]);
        git(
            &self.source,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "--allow-empty",
                "-m",
                source,
            ],
        );
        let revision = git(&self.source, &["rev-parse", "HEAD"]);
        let scope = Scope {
            analysis: if narrow {
                vec!["lib.rs".into()]
            } else {
                vec!["helper.rs".into(), "lib.rs".into()]
            },
            context: vec!["LICENSE".into()],
            max_files: 3,
            max_bytes: 65536,
            max_file_bytes: 32768,
        };
        let packet = local::acquire(
            &self.source,
            "https://example.com/owner/repo",
            &revision,
            scope,
        )
        .unwrap();
        let admission = self
            .store()
            .admit(packet, Retention { seconds: 1000 })
            .unwrap();
        let run = Run::new(
            &admission,
            BTreeMap::from([("fixture".into(), version.into())]),
            "local".into(),
            completion,
            vec![],
        )
        .unwrap();
        let findings = names
            .iter()
            .map(|(name, title)| {
                let mut f = Finding {
                    schema: CONTRACT.into(),
                    id: String::new(),
                    repository: run.repository.clone(),
                    perspective: "fixture".into(),
                    rule: "boundary".into(),
                    semantic_anchor: (*name).into(),
                    title: (*title).into(),
                    severity: Severity::Medium,
                    rationale: "Review the referenced dependency".into(),
                    confidence: Confidence::Unknown,
                    evidence: vec![admission
                        .evidence
                        .iter()
                        .find(|e| e.path == "lib.rs")
                        .unwrap()
                        .id
                        .clone()],
                    inference: "Fixture assessment".into(),
                    scope_digest: run.scope_digest.clone(),
                    limitations: vec![],
                };
                f.id = f.identity().unwrap();
                f
            })
            .collect();
        let record = ReviewRecord {
            admission,
            run,
            findings,
        };
        record.validate().unwrap();
        record
    }
}

#[allow(dead_code)]
#[path = "../examples/codefriend_palace_fixture.rs"]
mod authority_fixture;
fn setup_authority(f: &Fixture) -> (adl_runtime_kernel::VerifiedMemoryPalaceAuthority, PathBuf) {
    let root = f.temp.path().join("authority");
    authority_fixture::generate(&root).unwrap();
    let auth = palace_authority::provision(
        &root.join("trust.json"),
        &root.join("authority-evidence.json"),
    )
    .unwrap();
    (auth, root)
}
fn index_request(refs: Vec<BaselineRef>) -> IndexRequest {
    IndexRequest {
        schema: palace::VERSION.into(),
        references: refs,
        observed_epoch_ms: 2000,
        stale_after_ms: 10000,
        max_working_set_items: 8,
    }
}
fn retrieve_request(
    b: BaselineRef,
    c: BaselineRef,
    a: &adl_runtime_kernel::VerifiedMemoryPalaceAuthority,
) -> RetrieveRequest {
    RetrieveRequest {
        schema: palace::VERSION.into(),
        baseline: b,
        current: c,
        expected_identity_root: a.identity().identity_root.clone(),
        expected_continuity_head: a.continuity().record().continuity_head.clone(),
        packet_observed_epoch_ms: 2000,
        observed_epoch_ms: 2100,
        stale_after_ms: 10000,
        max_working_set_items: 8,
    }
}
#[test]
fn production_second_review_is_deterministic_and_retains_only_shared_digest_references() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record(
        "pub fn before() {}",
        &[("same", "Same"), ("resolved", "Gone")],
        "1",
        Completion::Complete,
        false,
    );
    let new = f.record(
        "pub fn after() {}",
        &[("same", "Same"), ("added", "New")],
        "1",
        Completion::Complete,
        false,
    );
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let c = backend.retain(&new).unwrap();
    let root = f.temp.path().join("palace");
    let receipt = palace::index(&backend, &root, &a, &index_request(vec![b.clone()])).unwrap();
    assert_eq!(receipt.backend, "RuntimeMemoryPalaceService");
    let request = retrieve_request(b.clone(), c, &a);
    let report = palace::retrieve(&backend, &root, &request).unwrap();
    assert!(report.delta.comparable);
    assert_eq!(report.selected_references, vec![b]);
    assert_eq!(report, palace::retrieve(&backend, &root, &request).unwrap());
    assert!(report
        .delta
        .changes
        .iter()
        .any(|c| c.comparison.outcome == Delta::Added));
    assert!(report
        .delta
        .changes
        .iter()
        .any(|c| c.comparison.outcome == Delta::Resolved));
    let commit = adl_runtime::memory_palace::RuntimeMemoryPalaceService::new(&root)
        .load_latest_strict()
        .unwrap()
        .unwrap();
    for item in commit.packet.working_set {
        let r: BaselineRef = serde_json::from_str(&item.payload).unwrap();
        r.validate().unwrap();
        assert!(!item.payload.contains("pub fn"));
    }
}
#[test]
fn missing_corrupt_pointer_and_tampered_kernel_packet_are_denied_without_repair() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record("pub fn old() {}", &[], "1", Completion::Complete, false);
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let root = f.temp.path().join("palace");
    palace::index(&backend, &root, &a, &index_request(vec![b.clone()])).unwrap();
    let req = retrieve_request(b.clone(), b, &a);
    let latest = root.join("latest.json");
    let bytes = fs::read(&latest).unwrap();
    fs::remove_file(&latest).unwrap();
    assert!(palace::retrieve(&backend, &root, &req).is_err());
    assert!(!latest.exists());
    fs::write(&latest, b"bad").unwrap();
    assert!(palace::retrieve(&backend, &root, &req).is_err());
    assert_eq!(fs::read(&latest).unwrap(), b"bad");
    let canary = "private-canary-never-log";
    let mut malicious: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    malicious["generation"] = serde_json::json!(canary);
    fs::write(&latest, serde_json::to_vec(&malicious).unwrap()).unwrap();
    let error = palace::retrieve(&backend, &root, &req)
        .unwrap_err()
        .to_string();
    assert!(error.contains("strict validation denied"));
    assert!(!error.contains(canary));
    fs::write(&latest, bytes).unwrap();
    let commit = adl_runtime::memory_palace::RuntimeMemoryPalaceService::new(&root)
        .load_latest_strict()
        .unwrap()
        .unwrap();
    let trace_path = root.join(&commit.checkpoint.trace_reference.path);
    let trace_bytes = fs::read(&trace_path).unwrap();
    fs::write(&trace_path, b"tampered").unwrap();
    assert!(palace::retrieve(&backend, &root, &req)
        .unwrap_err()
        .to_string()
        .contains("trace_digest"));
    fs::write(&trace_path, trace_bytes).unwrap();
    for artifact in [
        root.join("journal/00000000000000000001.json"),
        root.join("generations/00000000000000000001/packet.json"),
    ] {
        let saved = fs::read(&artifact).unwrap();
        let mut value: serde_json::Value = serde_json::from_slice(&saved).unwrap();
        let field = if artifact.file_name().unwrap() == "packet.json" {
            "observed_epoch_ms"
        } else {
            "generation"
        };
        value[field] = serde_json::json!(canary);
        fs::write(&artifact, serde_json::to_vec(&value).unwrap()).unwrap();
        let error = palace::retrieve(&backend, &root, &req)
            .unwrap_err()
            .to_string();
        assert!(error.contains("strict validation denied"));
        assert!(!error.contains(canary));
        fs::write(&artifact, saved).unwrap();
    }
    // Locate the actual kernel packet; no reconstructed fixture packet is used.
    fn alter(path: &Path) -> bool {
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                if alter(&entry.unwrap().path()) {
                    return true;
                }
            }
        } else if let Ok(bytes) = fs::read(path) {
            if let Ok(mut v) = serde_json::from_slice::<serde_json::Value>(&bytes) {
                if v.get("schema").and_then(|v| v.as_str())
                    == Some(adl_runtime_kernel::MEMORY_PALACE_PACKET_SCHEMA)
                {
                    v["working_set"][0]["citations"][0]["sha256"] =
                        serde_json::json!("0".repeat(64));
                    fs::write(path, serde_json::to_vec(&v).unwrap()).unwrap();
                    return true;
                }
            }
        }
        false
    }
    assert!(alter(&root.join("generations")));
    assert!(palace::retrieve(&backend, &root, &req).is_err());
}
#[test]
fn wrong_identity_continuity_run_and_stale_observation_are_denied() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record("pub fn old() {}", &[], "1", Completion::Complete, false);
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let root = f.temp.path().join("palace");
    palace::index(&backend, &root, &a, &index_request(vec![b.clone()])).unwrap();
    let req = retrieve_request(b.clone(), b, &a);
    let mut bad = req.clone();
    bad.expected_identity_root = "wrong".into();
    assert!(palace::retrieve(&backend, &root, &bad)
        .unwrap_err()
        .to_string()
        .contains("identity"));
    let mut bad = req.clone();
    bad.expected_continuity_head = "0".repeat(64);
    assert!(palace::retrieve(&backend, &root, &bad).is_err());
    let mut bad = req.clone();
    bad.baseline.run_id = "0".repeat(64);
    assert!(palace::retrieve(&backend, &root, &bad)
        .unwrap_err()
        .to_string()
        .contains("selected_baseline_missing"));
    let mut bad = req.clone();
    bad.observed_epoch_ms = 12001;
    assert!(palace::retrieve(&backend, &root, &bad)
        .unwrap_err()
        .to_string()
        .contains("stale"));
    let mut bad = req;
    bad.observed_epoch_ms = 1999;
    assert!(palace::retrieve(&backend, &root, &bad).is_err());
}
#[test]
fn partial_narrower_and_version_changed_runs_remain_not_comparable() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record(
        "pub fn old() {}",
        &[("risk", "Risk")],
        "1",
        Completion::Complete,
        false,
    );
    let currents = [
        f.record("pub fn new() {}", &[], "1", Completion::Incomplete, false),
        f.record("pub fn new() {}", &[], "1", Completion::Complete, true),
        f.record("pub fn new() {}", &[], "2", Completion::Complete, false),
    ];
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let root = f.temp.path().join("palace");
    palace::index(&backend, &root, &a, &index_request(vec![b.clone()])).unwrap();
    for record in currents {
        let c = backend.retain(&record).unwrap();
        let result =
            palace::retrieve(&backend, &root, &retrieve_request(b.clone(), c, &a)).unwrap();
        assert!(!result.delta.comparable);
        assert_eq!(result.delta.changes.len(), 1);
        assert_eq!(
            result.delta.changes[0].comparison.outcome,
            Delta::NotComparable
        );
    }
}
#[test]
fn deleted_baseline_and_expired_evidence_cannot_be_retrieved_or_reindexed() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record("pub fn old() {}", &[], "1", Completion::Complete, false);
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let root = f.temp.path().join("palace");
    let index = index_request(vec![b.clone()]);
    palace::index(&backend, &root, &a, &index).unwrap();
    let req = retrieve_request(b.clone(), b.clone(), &a);
    f.clock.fetch_add(7200, Ordering::SeqCst);
    assert!(palace::retrieve(&backend, &root, &req).is_err());
    assert!(palace::index(&backend, &root, &a, &index).is_err());
    f.clock.fetch_sub(7200, Ordering::SeqCst);
    backend.delete(&b).unwrap();
    assert!(palace::retrieve(&backend, &root, &req).is_err());
    assert!(palace::index(&backend, &root, &a, &index).is_err());
}
#[test]
fn working_set_is_bounded_and_duplicates_and_unselected_baselines_are_denied() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (a, _) = setup_authority(&f);
    let old = f.record("pub fn old() {}", &[], "1", Completion::Complete, false);
    let new = f.record("pub fn new() {}", &[], "1", Completion::Complete, false);
    let backend = AdmittedBaselines::open(f.store(), &f.baselines(), true).unwrap();
    let b = backend.retain(&old).unwrap();
    let c = backend.retain(&new).unwrap();
    let root = f.temp.path().join("palace");
    assert!(palace::index(
        &backend,
        &root,
        &a,
        &index_request(vec![b.clone(), b.clone()])
    )
    .unwrap_err()
    .to_string()
    .contains("duplicate"));
    let mut index = index_request(vec![c.clone(), b.clone()]);
    index.max_working_set_items = 1;
    palace::index(&backend, &root, &a, &index).unwrap();
    let mut requests = vec![
        retrieve_request(b.clone(), c.clone(), &a),
        retrieve_request(c, b, &a),
    ];
    let mut successes = 0;
    for req in &mut requests {
        req.max_working_set_items = 1;
        if let Ok(result) = palace::retrieve(&backend, &root, req) {
            assert_eq!(result.selected_references.len(), 1);
            successes += 1;
        }
    }
    assert_eq!(successes, 1);
}
#[test]
fn untrusted_keys_do_not_create_a_palace() {
    let _guard = TEST_LOCK.lock().unwrap();
    let f = Fixture::new();
    let (_, root) = setup_authority(&f);
    let trust = root.join("trust.json");
    let evidence = root.join("authority-evidence.json");
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&trust).unwrap()).unwrap();
    value["continuity_public_key"] = value["identity_public_key"].clone();
    fs::write(&trust, serde_json::to_vec(&value).unwrap()).unwrap();
    assert!(palace_authority::provision(&trust, &evidence).is_err());
}
