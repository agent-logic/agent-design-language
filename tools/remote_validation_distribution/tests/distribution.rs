use std::process::Command;

fn check(case: &str) {
    let output = Command::new("python3")
        .args([
            "-B",
            "-m",
            "unittest",
            &format!("test_distribution.DistributionTests.{case}"),
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("run the package boundary regression");
    assert!(
        output.status.success(),
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stderr).contains("Ran 1 test"));
}

#[test]
fn frozen_git_export_reproduces_archive() {
    check("test_frozen_git_export");
}
#[test]
fn verified_archive_preserves_exact_payload() {
    check("test_verified_extraction");
}
#[test]
fn altered_archive_and_version_are_rejected() {
    check("test_tampered_payload_and_identity");
}
#[test]
fn unsafe_entries_are_rejected_before_extraction() {
    check("test_unsafe_archive_entries");
}
#[test]
fn existing_destination_is_preserved() {
    check("test_existing_destination_preserved");
}
