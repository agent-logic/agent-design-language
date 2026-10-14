use std::path::Path;
use std::process::Command;

#[test]
fn installed_public_packages_for_issue_1223() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output = Command::new("python3")
        .args([
            "-B",
            "tools/public_adl/verify_install_validator.py",
            "--issue",
            "1223",
        ])
        .current_dir(root)
        .output()
        .expect("execute exact-candidate installed public-package proof");
    assert!(
        output.status.success(),
        "installed public-package proof failed: {}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("\"status\":\"passed\""));
}
