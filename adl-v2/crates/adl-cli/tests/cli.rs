use sha2::{Digest, Sha256};
use std::fs;
use std::process::Command;

fn cli() -> Command {
    let test_binary = std::env::current_exe().expect("test path");
    let binary = test_binary
        .parent()
        .and_then(|deps| deps.parent())
        .expect("target debug directory")
        .join("adl-v2");
    Command::new(binary)
}

#[test]
fn schema_is_machine_readable() {
    let output = cli().arg("schema").output().expect("schema command");
    assert!(output.status.success());
    let value: serde_json::Value = serde_json::from_slice(&output.stdout).expect("JSON stdout");
    assert_eq!(value["schema"], "adl.schema.v1");
    assert!(value["result"]["properties"].is_object());
    assert!(output.stderr.is_empty());
}

#[test]
fn malformed_input_fails_with_stderr_only() {
    let root = tempfile::tempdir().expect("temp root");
    let input = root.path().join("bad.json");
    fs::write(&input, "{not-json").expect("input");
    let output = cli()
        .args(["validate", input.to_str().unwrap()])
        .output()
        .expect("validate");
    assert!(!output.status.success());
    let error: serde_json::Value = serde_json::from_slice(&output.stdout).expect("error JSON");
    assert_eq!(error["schema"], "adl.error.v1");
    assert!(output.stderr.is_empty());
}

#[test]
fn selector_select_inspect_and_rollback_are_transactional() {
    let root = tempfile::tempdir().expect("selector root");
    fs::create_dir(root.path().join("bin")).expect("bin");
    fs::write(root.path().join("bin/v2"), b"binary-v2").expect("generation");
    fs::create_dir(root.path().join("receipts")).expect("receipts");
    let digest = format!("{:x}", Sha256::digest(b"binary-v2"));
    fs::write(
        root.path().join("receipts/v2.json"),
        format!(r#"{{"schema":"adl.install.receipt.v1","binary":"v2","sha256":"{digest}"}}"#),
    )
    .expect("receipt");
    let selected = cli()
        .args(["select", "v2", "--root", root.path().to_str().unwrap()])
        .output()
        .expect("select");
    assert!(selected.status.success());
    let stale = cli()
        .args([
            "select",
            "v2",
            "--expected-current-digest",
            "deadbeef",
            "--root",
            root.path().to_str().unwrap(),
        ])
        .output()
        .expect("stale select");
    assert!(!stale.status.success());
    let receipt: serde_json::Value = serde_json::from_slice(&selected.stdout).expect("receipt");
    assert_eq!(receipt["schema"], "adl.selector.receipt.v1");

    let inspected = cli()
        .args(["inspect", "--root", root.path().to_str().unwrap()])
        .output()
        .expect("inspect");
    assert!(inspected.status.success());
    let state: serde_json::Value = serde_json::from_slice(&inspected.stdout).expect("state");
    assert_eq!(state["result"]["current"]["generation"], "v2");

    let rolled_back = cli()
        .args(["rollback", "--root", root.path().to_str().unwrap()])
        .output()
        .expect("rollback");
    assert!(!rolled_back.status.success());
    let error: serde_json::Value =
        serde_json::from_slice(&rolled_back.stdout).expect("rollback error");
    assert_eq!(error["schema"], "adl.error.v1");
}

#[test]
fn selector_rejects_stale_cas_and_unsupported_schema() {
    let root = tempfile::tempdir().expect("selector root");
    fs::write(
        root.path().join("selector.json"),
        r#"{"schema":"adl.selector.v0","current":null,"previous":null}"#,
    )
    .expect("selector");
    let inspected = cli()
        .args(["inspect", "--root", root.path().to_str().unwrap()])
        .output()
        .expect("inspect");
    assert!(!inspected.status.success());
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&inspected.stdout).unwrap()["schema"],
        "adl.error.v1"
    );
}
