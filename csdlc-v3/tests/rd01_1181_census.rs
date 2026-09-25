//! PVF: deterministic docs/planning-contract proof for #1181 only.
//! Local Git objects, Python and bounded disk/CPU; no network or providers.
//! Required issue gate; does not qualify extraction, installation or release.
#[test]
fn complete_frozen_census_and_both_deltas_reject_missing_evidence() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let result = std::process::Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(root.join("docs/milestones/v0.93.1/repository-decomposition/rd01-1181/census.py"))
        .output()
        .expect("Python and retained Git objects are required");
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "pass");
    assert_eq!(report["paths"], 36899);
    assert_eq!(report["negative_fixtures"], 6);
}
