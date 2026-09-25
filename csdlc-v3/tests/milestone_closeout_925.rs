//! PVF: deterministic local document identity checks; no external effects.
//! Does not prove product qualification, review success, or milestone closure.
#[test]
fn closeout_identity_and_truth_boundaries() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let out = std::process::Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(root.join("docs/milestones/v0.92.2/evidence/issue-925/validate_closeout.py"))
        .output()
        .unwrap();
    assert!(
        out.status.success(),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    let value: serde_json::Value = serde_json::from_slice(&out.stdout).unwrap();
    assert_eq!(value["status"], "pass");
    assert_eq!(value["negative_cases"], 6);
    assert_eq!(value["historical_negative_cases"], 5);
    assert_eq!(value["remote_effects"], false);
}
