use std::process::Command;

fn assert_tooling_rejected(binary: &str, expected: &str) {
    let output = Command::new(binary)
        .arg("tooling")
        .output()
        .expect("run compatibility binary");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains(expected), "stderr={stderr}");
}

#[test]
fn issue_327_removed_tooling_routes_fail_closed() {
    assert_tooling_rejected(
        env!("CARGO_BIN_EXE_adl"),
        "the v1 tooling multiplexer was removed; use the independent C-SDLC v2 binaries",
    );
    assert_tooling_rejected(
        env!("CARGO_BIN_EXE_adl-review"),
        "adl-review owns review tooling only",
    );
}
