use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn help_exposes_capture_and_verify_without_running_incumbent() {
    Command::cargo_bin("adl-characterize")
        .unwrap()
        .arg("--help")
        .assert()
        .success()
        .stdout(predicate::str::contains("capture"))
        .stdout(predicate::str::contains("verify"));
}

#[test]
fn cli_verifies_retained_evidence_and_writes_report() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let temp = tempfile::tempdir().unwrap();
    Command::cargo_bin("adl-characterize")
        .unwrap()
        .args([
            "verify",
            "--corpus",
            root.join("corpus/v1/corpus.yaml").to_str().unwrap(),
            "--observations",
            root.join("observations/v1").to_str().unwrap(),
            "--report",
            temp.path().join("report.json").to_str().unwrap(),
        ])
        .assert()
        .success()
        .stdout(predicate::str::contains("\"status\": \"pass\""));
    assert!(temp.path().join("report.json").exists());
}
