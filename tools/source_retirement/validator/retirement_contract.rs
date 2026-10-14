use std::path::Path;
use std::process::Command;

fn run(script: &str) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output = Command::new("python3")
        .args(["-B", script])
        .current_dir(root)
        .output()
        .expect("execute retirement validator");
    assert!(
        output.status.success(),
        "retirement validation failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn committed_candidate_retirement_guard() {
    run("tools/source_retirement/check.py");
}

#[test]
fn retirement_negative_cases() {
    run("tools/source_retirement/test_check.py");
}
