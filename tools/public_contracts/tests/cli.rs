//! PVF local contract: compatibility refusal exit and channel stability.

#[test]
fn cli_refusal_preserves_exit_and_channels() {
    let output = std::process::Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .args([
            "compatibility",
            "--lockset",
            "missing.json",
            "--graph",
            "missing.json",
            "--rollback",
            "missing.json",
        ])
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(2));
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr)
        .starts_with("adl_event component=compatibility_lockset outcome=refused reason="));
}
