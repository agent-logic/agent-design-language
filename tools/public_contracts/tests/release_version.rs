// PVF: deterministic local contract, small CPU/disk, no network/provider calls.
// Release gate helper regression; does not qualify a deployed release.
use adl_public_contracts::release_version::{metadata, source};
use std::{fs, process::Command};
const MANIFEST: &str = "[package]\nname='adl'\nversion='0.93.2'\n";
const LOCK: &str =
    "[[package]]\nname='other'\nversion='1.0.0'\n[[package]]\nname='adl'\nversion='0.93.2'\n";
const META: &str =
    r#"{"packages":[{"name":"other","version":"1.0.0"},{"name":"adl","version":"0.93.2"}]}"#;
#[test]
fn preserves_version_equality_and_missing_input_refusals() {
    assert_eq!(source(MANIFEST, LOCK).unwrap(), "0.93.2");
    assert_eq!(metadata("0.93.2", META).unwrap(), "0.93.2");
    for lock in [
        LOCK.replace("0.93.2", "0.93.1"),
        String::from("package=[]"),
        String::from("!invalid"),
    ] {
        assert!(source(MANIFEST, &lock).is_err());
    }
    for manifest in [
        "[package]",
        "[package]\nversion=42",
        "[package]\nversion=''",
        "!",
    ] {
        assert!(source(manifest, LOCK).is_err());
    }
    for m in [
        META.replace("0.93.2", "0.93.1"),
        String::from("{}"),
        String::from(r#"{"packages":[{"name":"adl","version":42}]}"#),
        String::from("!"),
    ] {
        assert!(metadata("0.93.2", &m).is_err());
    }
}
#[test]
fn cli_needs_no_interpreter_and_refuses_drift() {
    let d = tempfile::tempdir().unwrap();
    let manifest = d.path().join("manifest");
    let lock = d.path().join("lock");
    let meta = d.path().join("meta");
    fs::write(&manifest, MANIFEST).unwrap();
    fs::write(&lock, LOCK).unwrap();
    fs::write(&meta, META).unwrap();
    let binary = env!("CARGO_BIN_EXE_adl-public-contracts");
    let out = Command::new(binary)
        .arg("release-version-source")
        .arg(&manifest)
        .arg(&lock)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b"0.93.2\n");
    let out = Command::new(binary)
        .args(["release-version-metadata", "0.93.2"])
        .arg(&meta)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b"0.93.2\n");
    let out = Command::new(binary)
        .args(["release-version-metadata", "0.93.1"])
        .arg(&meta)
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(!out.status.success());
    assert!(String::from_utf8_lossy(&out.stderr)
        .contains("cargo metadata resolved 0.93.2 but manifest says 0.93.1"));
}
