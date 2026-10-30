use std::{path::Path, process::Command};
// PVF installed consumer qualification: native proof enforces the macOS sandbox.
#[test]
fn public_adl_install_validator() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let output = Command::new("cargo")
        .args([
            "run",
            "--locked",
            "--quiet",
            "--manifest-path",
            "tools/public_contracts/Cargo.toml",
            "--bin",
            "adl-public-contracts",
            "--",
            "install-validator",
            "--issue",
            "1192",
        ])
        .current_dir(root)
        .output()
        .expect("execute native install validator");
    assert!(
        output.status.success(),
        "native validator failed; evidence result: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("\"status\":\"passed\""),
        "missing pass marker"
    );
}
