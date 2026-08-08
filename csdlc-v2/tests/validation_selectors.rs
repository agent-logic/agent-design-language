use csdlc_v2::cards::{classify_rust_test_selector, RustTestSelectorPosture};
use std::process::Command;

fn argv(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).into()).collect()
}

#[test]
fn exact_broad_and_invalid_rust_test_selectors_are_distinct() {
    let exact = classify_rust_test_selector(&argv(&[
        "cargo",
        "test",
        "--manifest-path",
        "csdlc-v2/Cargo.toml",
        "--lib",
        "schema::tests",
    ]))
    .expect("cargo test classification");
    assert_eq!(exact.posture, RustTestSelectorPosture::ExactTarget);

    let integration = classify_rust_test_selector(&argv(&["cargo", "test", "--test", "gate2"]))
        .expect("cargo test classification");
    assert_eq!(integration.posture, RustTestSelectorPosture::ExactTarget);

    let doc = classify_rust_test_selector(&argv(&["cargo", "test", "--doc", "schema"]))
        .expect("cargo test classification");
    assert_eq!(doc.posture, RustTestSelectorPosture::ExactTarget);

    let toolchain =
        classify_rust_test_selector(&argv(&["cargo", "+stable", "test", "--test", "gate2"]))
            .expect("cargo toolchain test classification");
    assert_eq!(toolchain.posture, RustTestSelectorPosture::ExactTarget);

    let global_options = classify_rust_test_selector(&argv(&[
        "cargo", "--color", "never", "--locked", "test", "--test", "gate2",
    ]))
    .expect("cargo global options test classification");
    assert_eq!(global_options.posture, RustTestSelectorPosture::ExactTarget);

    let alias = classify_rust_test_selector(&argv(&["cargo", "-vv", "t", "--test", "gate2"]))
        .expect("cargo test alias classification");
    assert_eq!(alias.posture, RustTestSelectorPosture::ExactTarget);

    let broad = classify_rust_test_selector(&argv(&[
        "cargo",
        "test",
        "--manifest-path",
        "csdlc-v2/Cargo.toml",
    ]))
    .expect("cargo test classification");
    assert_eq!(broad.posture, RustTestSelectorPosture::IntentionalBroad);

    for invalid in [
        argv(&["cargo", "test", "schema"]),
        argv(&["cargo", "test", "--", "schema", "--list"]),
        argv(&["cargo", "+stable", "test", "schema"]),
        argv(&["cargo", "--locked", "test", "schema"]),
        argv(&["cargo", "t", "schema"]),
        argv(&["cargo", "+stable", "t", "schema"]),
        argv(&["cargo", "test", "--test"]),
        argv(&["cargo", "test", "--lib", "--test", "gate2"]),
    ] {
        let classification = classify_rust_test_selector(&invalid).expect("cargo test");
        assert_eq!(classification.posture, RustTestSelectorPosture::Invalid);
        assert!(classification.diagnostic.is_some());
    }
}

#[test]
fn exact_schema_lane_selects_nonzero_library_tests_only() {
    let manifest = format!("{}/Cargo.toml", env!("CARGO_MANIFEST_DIR"));
    let output = Command::new("cargo")
        .args([
            "test",
            "--locked",
            "--manifest-path",
            &manifest,
            "--lib",
            "schema::tests",
            "--",
            "--list",
        ])
        .output()
        .expect("list exact schema tests");
    assert!(
        output.status.success(),
        "schema list failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("utf8 test list");
    let stderr = String::from_utf8(output.stderr).expect("utf8 cargo diagnostics");
    assert!(stdout.lines().any(|line| line.contains("schema::tests::")));
    assert!(!stdout.contains("estimation_contracts"));
    assert!(!stderr.contains("estimation_contracts"));
}
