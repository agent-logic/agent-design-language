//! Executes the existing milestone planning validator; this is not product qualification.
#[test]
fn planning_contract_and_negative_fixtures() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
    let result = std::process::Command::new("python3")
        .current_dir(root)
        .args(["-c", include_str!("../check.py")])
        .output().expect("start existing planning validator");
    assert!(result.status.success(), "planning validation failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}

#[test]
fn workspace_coverage_deadline_contract() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../..");
    let result = std::process::Command::new("ruby")
        .current_dir(root)
        .arg("adl/tools/test_workspace_coverage_deadline.rb")
        .output().expect("start workspace coverage deadline contract");
    assert!(result.status.success(), "coverage deadline validation failed:\n{}\n{}",
        String::from_utf8_lossy(&result.stdout), String::from_utf8_lossy(&result.stderr));
}
