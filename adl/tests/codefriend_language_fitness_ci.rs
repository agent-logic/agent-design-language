//! PVF component: fixtures/codefriend/language-fitness-ci/PVF.json. Actual native CI route, not release qualification.
//! Local temporary Git/Store only; no provider or inspected-code execution.
use adl::codefriend::{
    evidence::{store::Store, Retention},
    governance::language::{self, Policy, Rule, StaticSelector, Status},
    ingestion::{local, Scope},
    language::{AnalysisPolicy, Language, Limits, ProjectRoot},
};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
static SERIAL: Mutex<()> = Mutex::new(());
fn git(root: &Path, args: &[&str]) -> String {
    let o = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(o.status.success());
    String::from_utf8(o.stdout).unwrap().trim().into()
}
struct Fixture {
    _temp: tempfile::TempDir,
    store: Store,
    packet: String,
    analysis: AnalysisPolicy,
    clock: Arc<AtomicU64>,
}
impl Fixture {
    fn new(files: &[(&str, &str, Language)]) -> Self {
        Self::new_at(files, 100)
    }
    fn new_at(files: &[(&str, &str, Language)], now: u64) -> Self {
        let t = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
        let source = t.path().join("source");
        fs::create_dir(&source).unwrap();
        git(&source, &["init", "-b", "main"]);
        git(
            &source,
            &["remote", "add", "origin", "https://example.com/owner/repo"],
        );
        for (p, s, _) in files {
            fs::write(source.join(p), s).unwrap();
        }
        fs::write(source.join("LICENSE"), "MIT fixture\n").unwrap();
        git(&source, &["add", "."]);
        git(
            &source,
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
        let rev = git(&source, &["rev-parse", "HEAD"]);
        let paths: BTreeMap<_, _> = files.iter().map(|(p, _, l)| (p.to_string(), *l)).collect();
        let packet = local::acquire(
            &source,
            "https://example.com/owner/repo",
            &rev,
            Scope {
                analysis: paths.keys().cloned().collect(),
                context: vec!["LICENSE".into()],
                max_files: files.len() + 1,
                max_bytes: 1024 * 1024,
                max_file_bytes: 512 * 1024,
            },
        )
        .unwrap();
        let clock = Arc::new(AtomicU64::new(now));
        let c = clock.clone();
        let store = Store::open(&t.path().join("store"), move || c.load(Ordering::SeqCst)).unwrap();
        let a = store.admit(packet, Retention { seconds: 100 }).unwrap();
        let roots = paths
            .values()
            .copied()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .map(|language| ProjectRoot {
                language,
                root: ".".into(),
                manifest: None,
            })
            .collect();
        let analysis = AnalysisPolicy {
            schema: adl::codefriend::language::VERSION.into(),
            files: paths,
            roots,
            layers: BTreeMap::new(),
            allowed: BTreeSet::new(),
            limits: Limits {
                max_nodes: 10000,
                max_depth: 128,
                max_facts: 1000,
                max_output_bytes: 1024 * 1024,
            },
        };
        Self {
            _temp: t,
            store,
            packet: a.packet.packet_id,
            analysis,
            clock,
        }
    }
    fn policy(&self, rule: Rule) -> Policy {
        Policy {
            schema: language::VERSION.into(),
            analysis: self.analysis.clone(),
            rules: vec![rule],
        }
    }
    fn run(&self, p: &Policy) -> language::Report {
        language::evaluate(&self.store, &self.packet, p, 101).unwrap()
    }
}
#[test]
fn native_v2_receipt_unknown_is_valid_but_never_success_and_pins_live_source() {
    use adl::codefriend::governance::{ci::Expected, ci_v2};
    let _guard = SERIAL.lock().unwrap();
    let f = Fixture::new(&[("app.py", "from .missing import x\n", Language::Python)]);
    let policy = f.policy(Rule::ForbiddenStaticImport {
        id: "boundary".into(),
        source_path: "app.py".into(),
        selector: StaticSelector::PythonFrom {
            module_prefix: vec!["blocked".into()],
            member: None,
        },
    });
    let report = f.run(&policy);
    assert_eq!(report.status, Status::Unknown);
    let pins = Expected {
        candidate: report.record.run.revision.clone(),
        packet_id: f.packet.clone(),
        policy_digest: report.policy_digest.clone(),
    };
    let receipt = ci_v2::verify(&f.store, &report, &pins, 2, 101).unwrap();
    assert!(receipt.artifact_valid);
    assert_eq!(receipt.exit_code, 2);
    assert_eq!(receipt.assessment, Some(Status::Unknown));
    assert!(ci_v2::verify(&f.store, &report, &pins, 0, 101).is_err());
    for unsupported in [-1, 3] {
        assert!(ci_v2::verify(&f.store, &report, &pins, unsupported, 101).is_err());
    }
    for changed in [
        Expected {
            candidate: "a".repeat(40),
            ..pins.clone()
        },
        Expected {
            packet_id: "b".repeat(64),
            ..pins.clone()
        },
        Expected {
            policy_digest: "c".repeat(64),
            ..pins.clone()
        },
    ] {
        assert!(ci_v2::verify(&f.store, &report, &changed, 2, 101).is_err());
    }
    f.clock.store(201, Ordering::SeqCst);
    assert!(ci_v2::verify(&f.store, &report, &pins, 2, 101).is_err());
}

#[test]
fn actual_ci_run_and_wrapper_preserve_pass_fail_unknown_for_four_languages() {
    let _guard = SERIAL.lock().unwrap();
    let cases = [
        (
            "app.rs",
            "use crate::blocked;\n",
            Language::Rust,
            StaticSelector::RustUse {
                prefix: vec!["crate".into(), "blocked".into()],
            },
            1,
            "fail",
        ),
        (
            "App.java",
            "import java.util.List; class App {}\n",
            Language::Java,
            StaticSelector::JavaTypeImport {
                prefix: vec!["blocked".into()],
            },
            0,
            "pass",
        ),
        (
            "app.py",
            "from .missing import x\n",
            Language::Python,
            StaticSelector::PythonFrom {
                module_prefix: vec!["blocked".into()],
                member: None,
            },
            2,
            "unknown",
        ),
        (
            "app.js",
            "import x from 'blocked';\n",
            Language::JavaScript,
            StaticSelector::JavaScriptModule {
                specifier: "blocked".into(),
            },
            1,
            "fail",
        ),
    ];
    for (path, source, lang, selector, exit, status) in cases {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let f = Fixture::new_at(&[(path, source, lang)], now);
        let policy = f.policy(Rule::ForbiddenStaticImport {
            id: "boundary".into(),
            source_path: path.into(),
            selector,
        });
        let policy_path = f._temp.path().join("policy.json");
        fs::write(&policy_path, serde_json::to_vec(&policy).unwrap()).unwrap();
        let digest = adl::codefriend::evidence::hash(&policy).unwrap();
        let revision = f.store.get(&f.packet).unwrap().packet.revision;
        let store = f._temp.path().join("store");
        let out = f._temp.path().join("ci");
        let packet = f.packet.clone();
        drop(f.store);
        let result = Command::new("bash")
            .arg(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tools/codefriend_fitness_ci.sh"
            ))
            .args([
                "--binary",
                env!("CARGO_BIN_EXE_adl"),
                "--store",
                store.to_str().unwrap(),
                "--packet-id",
                &packet,
                "--policy",
                policy_path.to_str().unwrap(),
                "--candidate",
                &revision,
                "--policy-digest",
                &digest,
                "--out",
                out.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert_eq!(
            result.status.code(),
            Some(exit),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(out.join("receipt.json")).unwrap()).unwrap();
        assert_eq!(receipt["schema"], "codefriend.fitness.ci.v2");
        assert_eq!(receipt["assessment"], status);
        assert_eq!(receipt["artifact_valid"], true);
        assert_eq!(receipt["original_exit"], exit);
        let verify = f._temp.path().join("verified.json");
        let result = Command::new(env!("CARGO_BIN_EXE_adl"))
            .args([
                "codefriend",
                "fitness",
                "ci-verify",
                "--store",
                store.to_str().unwrap(),
                "--input",
                out.join("report.json").to_str().unwrap(),
                "--candidate",
                &revision,
                "--packet-id",
                &packet,
                "--policy-digest",
                &digest,
                "--runner-exit",
                &exit.to_string(),
                "--out",
                verify.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert_eq!(result.status.code(), Some(exit));
        let saved: serde_json::Value = serde_json::from_slice(&fs::read(&verify).unwrap()).unwrap();
        assert_eq!(saved, receipt);
        // A valid analysis cannot certify a different source candidate. The rejection
        // must retain the runner exit for audit, without carrying successful identities.
        let wrong_revision = format!(
            "{}{}",
            if revision.starts_with('a') { 'b' } else { 'a' },
            &revision[1..]
        );
        let rejected_path = f._temp.path().join("rejected.json");
        let report_bytes = fs::read(out.join("report.json")).unwrap();
        let rejected = Command::new(env!("CARGO_BIN_EXE_adl"))
            .args([
                "codefriend",
                "fitness",
                "ci-verify",
                "--store",
                store.to_str().unwrap(),
                "--input",
                out.join("report.json").to_str().unwrap(),
                "--candidate",
                &wrong_revision,
                "--packet-id",
                &packet,
                "--policy-digest",
                &digest,
                "--runner-exit",
                &exit.to_string(),
                "--out",
                rejected_path.to_str().unwrap(),
            ])
            .output()
            .unwrap();
        assert_eq!(rejected.status.code(), Some(2));
        let rejection: serde_json::Value =
            serde_json::from_slice(&fs::read(&rejected_path).unwrap()).unwrap();
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&rejected.stdout).unwrap(),
            rejection
        );
        assert_eq!(rejection["schema"], "codefriend.fitness.ci.v2");
        assert_eq!(rejection["artifact_valid"], false);
        assert_eq!(rejection["exit_code"], 2);
        assert_eq!(rejection["original_exit"], exit);
        assert_eq!(rejection["error"], "fitness_ci_contract_rejected");
        for key in [
            "assessment",
            "candidate",
            "packet_id",
            "policy_digest",
            "report_digest",
        ] {
            assert!(rejection[key].is_null(), "rejected receipt retained {key}");
        }
        assert_eq!(fs::read(out.join("report.json")).unwrap(), report_bytes);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&fs::read(&verify).unwrap()).unwrap(),
            receipt
        );
    }
}

#[test]
fn ci_run_missing_store_does_not_create_store_or_output() {
    let _guard = SERIAL.lock().unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let f = Fixture::new_at(
        &[("app.js", "import x from 'allowed';\n", Language::JavaScript)],
        now,
    );
    let policy = f.policy(Rule::ForbiddenStaticImport {
        id: "boundary".into(),
        source_path: "app.js".into(),
        selector: StaticSelector::JavaScriptModule {
            specifier: "blocked".into(),
        },
    });
    let policy_path = f._temp.path().join("policy.json");
    fs::write(&policy_path, serde_json::to_vec(&policy).unwrap()).unwrap();
    let digest = adl::codefriend::evidence::hash(&policy).unwrap();
    let revision = f.store.get(&f.packet).unwrap().packet.revision;
    let missing = f._temp.path().join("missing-store");
    let out = f._temp.path().join("missing-output");
    let packet = f.packet.clone();
    drop(f.store);
    let result = Command::new(env!("CARGO_BIN_EXE_adl"))
        .args([
            "codefriend",
            "fitness",
            "ci-run",
            "--store",
            missing.to_str().unwrap(),
            "--packet-id",
            &packet,
            "--policy",
            policy_path.to_str().unwrap(),
            "--candidate",
            &revision,
            "--policy-digest",
            &digest,
            "--out",
            out.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert_eq!(result.status.code(), Some(2));
    let rejected: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(rejected["artifact_valid"], false);
    assert!(!missing.exists());
    assert!(!out.exists());
}
