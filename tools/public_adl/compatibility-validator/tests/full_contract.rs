use std::{path::Path, process::Command};

#[test]
fn complete_python_contract_suite_runs_29_tests() {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repository_root = manifest_dir
        .ancestors()
        .nth(3)
        .expect("compatibility validator manifest must remain under tools/public_adl");
    let output = Command::new("python3")
        .args(["-B", "tools/public_adl/test_compatibility_lockset.py"])
        .current_dir(repository_root)
        .output()
        .expect("python3 must execute the tracked compatibility contract suite");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let transcript = format!("{stdout}\n{stderr}");
    assert!(
        output.status.success(),
        "inner compatibility suite failed\n{transcript}"
    );
    assert!(
        transcript.contains("Ran 29 tests"),
        "inner compatibility suite denominator changed\n{transcript}"
    );
    assert!(
        transcript.lines().any(|line| line.trim() == "OK"),
        "inner compatibility suite did not report terminal OK\n{transcript}"
    );
    eprintln!("inner_python_unittest_count=29");
}
