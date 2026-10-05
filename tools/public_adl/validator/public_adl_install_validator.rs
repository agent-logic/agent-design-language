use std::path::Path;
use std::process::Command;

#[test]
fn public_adl_install_validator() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output = Command::new("python3")
        .args([
            "tools/public_adl/verify_install_validator.py",
            "--issue",
            "1192",
        ])
        .current_dir(root)
        .output()
        .expect("execute public ADL install validator");
    assert!(
        output.status.success(),
        "public ADL validator failed; evidence result: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let stdout = String::from_utf8(output.stdout).expect("validator stdout is UTF-8 JSON");
    assert!(
        stdout.contains("\"status\":\"passed\""),
        "missing pass marker: {stdout}"
    );
}
