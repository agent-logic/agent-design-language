// PVF: deterministic local compatibility regression, small CPU/disk, no network.
// Proves retired-source refusal only; no CSM binary or provider qualification.
use serde_json::Value;
use std::{fs, os::unix::fs::PermissionsExt, process::Command};
#[test]
fn retired_resolver_cannot_be_reactivated_and_keeps_diagnostics() {
    let root = adl_public_contracts::root();
    let d = tempfile::tempdir().unwrap();
    let bin = d.path().join("bin");
    fs::create_dir(&bin).unwrap();
    let marker = d.path().join("unexpected-execution");
    for name in ["cargo", "csm", "python", "python3", "ruby"] {
        let p = bin.join(name);
        fs::write(
            &p,
            "#!/bin/sh\nprintf unexpected >>\"$CSM_TEST_MARKER\"\nexit 91\n",
        )
        .unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o755)).unwrap();
    }
    let source = d.path().join("source");
    fs::create_dir_all(source.join("adl/src/bin")).unwrap();
    fs::write(source.join("adl/src/bin/csm.rs"), "fn main() {}\n").unwrap();
    fs::write(
        source.join("adl/Cargo.toml"),
        "[package]\nname='fixture'\nversion='0.0.0'\n",
    )
    .unwrap();
    let run = |args: &[&str]| {
        Command::new("bash")
            .arg(root.join("adl/tools/ensure_csm_binary.sh"))
            .args(args)
            .env(
                "PATH",
                format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
            )
            .env("ADL_TOOLING_MANIFEST_ROOT", &source)
            .env("ADL_CSM_SOURCE_ROOT", &source)
            .env("ADL_CSM_BIN", bin.join("csm"))
            .env("CSM_TEST_MARKER", &marker)
            .output()
            .unwrap()
    };
    let failed = |args: &[&str]| {
        let result = run(args);
        assert_eq!(result.status.code(), Some(1));
        assert!(result.stderr.is_empty());
        let p: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert_eq!(p["schema"], "adl.csm.binary_availability.v1");
        assert_eq!(p["status"], "failed");
        assert_eq!(p["runtime_owner"], "csm");
        assert_eq!(p["action"], "retired_source_route");
        assert!(p["binary"].is_null());
        assert_eq!(p["source_presence"], false);
        assert_eq!(p["validation_proof"], false);
        assert!(!String::from_utf8_lossy(&result.stdout).contains(d.path().to_str().unwrap()));
        p
    };
    let expected = failed(&["--json"]);
    assert_eq!(
        failed(&[
            "--json",
            "--check-only",
            "--csm-bin",
            bin.join("csm").to_str().unwrap()
        ]),
        expected
    );
    let output = d.path().join("diagnostic.json");
    let out = output.to_str().unwrap();
    assert_eq!(failed(&["--json", "--out", out]), expected);
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&output).unwrap()).unwrap(),
        expected
    );
    let human = run(&["--out", out]);
    assert_eq!(human.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&human.stdout).contains("status=failed"));
    assert_eq!(
        serde_json::from_slice::<Value>(&fs::read(&output).unwrap()).unwrap(),
        expected
    );
    let help = run(&["--help"]);
    assert!(help.status.success());
    assert!(String::from_utf8_lossy(&help.stdout).contains("never builds, discovers, or executes"));
    for args in [
        vec!["--out"],
        vec!["--csm-bin"],
        vec!["--out", "--json"],
        vec!["--unknown"],
    ] {
        let r = run(&args);
        assert_eq!(r.status.code(), Some(2));
        assert!(!r.stderr.is_empty());
        assert!(r.stdout.is_empty());
    }
    assert!(
        !marker.exists(),
        "retired route invoked an interpreter or executable candidate"
    );
}
