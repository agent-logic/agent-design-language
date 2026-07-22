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
    assert!(output.stdout.is_empty());
    assert!(!output.stderr.is_empty());
}

#[test]
fn selector_select_inspect_and_rollback_are_transactional() {
    let root = tempfile::tempdir().expect("selector root");
    fs::write(root.path().join("v2"), b"binary-v2").expect("generation");
    let selected = cli()
        .args(["select", "v2", "--root", root.path().to_str().unwrap()])
        .output()
        .expect("select");
    assert!(selected.status.success());
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
    assert!(rolled_back.stdout.is_empty());
}
