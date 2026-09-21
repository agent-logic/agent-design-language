//! PVF owner_binary: genuine repeated native runner execution with synthetic lane
//! output; deterministic local CPU/files, no provider. Required regression gate.
use adl::codefriend::{
    evidence::{
        contracts::{Completion, Confidence, Severity},
        store::Store,
        Retention,
    },
    ingestion::{local, Scope},
    memory::{
        baseline::{AdmittedBaselines, BaselineAccess},
        comparison,
    },
    review::runner::{
        run_with_executor, ExecutionOptions, LaneExecution, ParsedLaneFinding, ProviderLaneOutput,
    },
};
use adl::provider_communication::ProviderInvocationFinalStatusV1;
use std::{
    fs,
    path::Path,
    process::Command,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
};
fn git(root: &Path, args: &[&str]) -> String {
    let out = Command::new("git")
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
#[test]
fn genuine_repeated_review_results_retain_compare_and_keep_original_expiry() {
    let temp = tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap();
    let source = temp.path().join("source");
    fs::create_dir(&source).unwrap();
    git(&source, &["init", "-b", "main"]);
    git(
        &source,
        &["remote", "add", "origin", "https://example.com/owner/repo"],
    );
    fs::write(source.join("LICENSE"), "MIT fixture\n").unwrap();
    fs::write(source.join("lib.rs"), "pub fn answer() -> u8 { 42 }\n").unwrap();
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
    let revision = git(&source, &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &source,
        "https://example.com/owner/repo",
        &revision,
        Scope {
            analysis: vec!["lib.rs".into()],
            context: vec!["LICENSE".into()],
            max_files: 2,
            max_bytes: 65536,
            max_file_bytes: 32768,
        },
    )
    .unwrap();
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let clock = Arc::new(AtomicU64::new(now));
    let live = clock.clone();
    let store = Store::open(&temp.path().join("store"), move || {
        live.load(Ordering::SeqCst)
    })
    .unwrap();
    let admission = store.admit(packet, Retention { seconds: 1000 }).unwrap();
    let evidence = admission
        .evidence
        .iter()
        .find(|e| e.path == "lib.rs")
        .unwrap()
        .id
        .clone();
    let mut calls = 0;
    let mut execute = |name: &str, title: &str| {
        run_with_executor(
            ExecutionOptions {
                out: temp.path().join(name),
                run_id: name.into(),
                cancel_file: None,
            },
            admission.clone(),
            "fixture:local:no-provider".into(),
            |lane, _, _| {
                calls += 1;
                let output = ProviderLaneOutput {
                    findings: vec![ParsedLaneFinding {
                        rule: format!("{}.repeat", lane.id()),
                        semantic_anchor: "answer".into(),
                        title: title.into(),
                        severity: Severity::Medium,
                        rationale: "Synthetic fixture assessment".into(),
                        confidence: Confidence::Unknown,
                        evidence: vec![evidence.clone()],
                        inference: "Fixture inference".into(),
                        limitations: vec![],
                    }],
                };
                Ok(LaneExecution {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some(serde_json::to_string(&output).unwrap()),
                })
            },
        )
        .unwrap()
    };
    let first = execute("first", "First assessment");
    let second = execute("second", "Changed assessment");
    assert_eq!(calls, 8);
    assert_eq!(first.completion, Completion::Complete);
    assert_eq!(second.completion, Completion::Complete);
    assert_ne!(first.run_id, second.run_id);
    assert_eq!(first.review_record.run.id, second.review_record.run.id);
    assert_eq!(first.review_record.admission, admission);
    assert_eq!(second.review_record.admission, admission);
    let backend = AdmittedBaselines::open(&store, &temp.path().join("baselines"), true).unwrap();
    let a = backend.retain(&first.review_record).unwrap();
    let b = backend.retain(&second.review_record).unwrap();
    assert_ne!(a.record_digest, b.record_digest);
    assert_eq!(backend.retain(&second.review_record).unwrap(), b);
    assert_eq!(backend.load(&a).unwrap(), first.review_record);
    assert_eq!(backend.load(&b).unwrap(), second.review_record);
    let delta = comparison::compare(&backend, &a, &b).unwrap();
    assert!(delta.comparable);
    assert_eq!(store.get(&a.packet_id).unwrap(), admission);
    clock.store(now + 1001, Ordering::SeqCst);
    assert!(backend.load(&a).is_err());
    assert!(backend.load(&b).is_err());
    assert!(backend.retain(&second.review_record).is_err());
}
