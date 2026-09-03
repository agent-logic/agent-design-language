use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

#[test]
fn generation_installer_rejects_mixed_set_and_preserves_current_reference() {
    let root = tempfile::tempdir().expect("temp root");
    let sources = root.path().join("sources");
    let install = root.path().join("install");
    fs::create_dir_all(&sources).expect("sources");
    for binary in ["csm", "adl-runtime-guardian", "adl-runtime-kernel"] {
        let path = sources.join(binary);
        fs::write(&path, "#!/bin/sh\nexit 0\n").expect("binary");
        let mut permissions = fs::metadata(&path).expect("metadata").permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).expect("executable");
    }
    let script = format!(
        "{}/tools/install_runtime_v3_generation.sh",
        env!("CARGO_MANIFEST_DIR")
    );
    let installed = Command::new(&script)
        .args([
            "install",
            "--root",
            install.to_str().unwrap(),
            "--generation",
            "one",
            "--csm",
            sources.join("csm").to_str().unwrap(),
            "--guardian",
            sources.join("adl-runtime-guardian").to_str().unwrap(),
            "--kernel",
            sources.join("adl-runtime-kernel").to_str().unwrap(),
            "--source-revision",
            "test-revision",
            "--build-profile",
            "debug",
        ])
        .status()
        .expect("run installer");
    assert!(installed.success());
    let current_before = fs::read_link(install.join("current")).expect("current");
    fs::write(
        install.join("generations/one/bin/adl-runtime-kernel"),
        "mixed",
    )
    .expect("tamper");
    let verified = Command::new(&script)
        .args(["verify", "--root", install.to_str().unwrap()])
        .status()
        .expect("run verifier");
    assert!(!verified.success());
    assert_eq!(
        fs::read_link(install.join("current")).expect("current"),
        current_before
    );
}
