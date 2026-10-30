//! PVF local contract: deterministic custody/storage; macOS-only OS sandbox probes.
use adl_public_contracts::{install, install_validator, root};
use std::{fs, path::Path, process::Command};
#[test]
fn canonical_resources_reconstruct() {
    let report = install::resources(&root().join("adl-uts")).unwrap();
    assert_eq!(report["members_verified"], 33);
    assert_eq!(report["archive_reconstructed"], true);
}
#[test]
fn failures_keep_default_and_override_evidence() {
    for override_root in [false, true] {
        let tmp = tempfile::tempdir().unwrap();
        let base = fs::canonicalize(tmp.path()).unwrap();
        let checkout = base.join("checkout");
        let explicit = base.join("explicit proof");
        let expected = if override_root {
            explicit.clone()
        } else {
            checkout.join("target/public-adl-proof")
        };
        let result = install_validator::run_with(
            &checkout,
            override_root.then_some(explicit.as_path()),
            1223,
            |_| {
                Ok(Command::new("sh")
                    .args([
                        "-c",
                        "printf 'fixture stdout'; printf 'fixture failure' >&2; exit 2",
                    ])
                    .output()?)
            },
        )
        .unwrap();
        let evidence = Path::new(result["evidence_directory"].as_str().unwrap());
        assert!(evidence.starts_with(expected));
        assert_eq!(
            fs::read_to_string(evidence.join("stderr.txt")).unwrap(),
            "fixture failure"
        );
        assert_eq!(result["status"], "failed");
        assert!(evidence.join("validator-result.json").is_file());
    }
}
#[test]
#[cfg(target_os = "macos")]
fn os_sandbox_enforces_read_and_network_denials() {
    let t = tempfile::tempdir().unwrap();
    let base = fs::canonicalize(t.path()).unwrap();
    let home = base.join("denied-home");
    let work = home.join("scratch");
    fs::create_dir_all(&work).unwrap();
    let secret = work.join("private-credential");
    fs::write(&secret, "synthetic secret").unwrap();
    let producer = home.join("producer");
    fs::create_dir(&producer).unwrap();
    fs::write(producer.join("source"), "synthetic producer source").unwrap();
    let fixture = work.join("fixture");
    fs::write(&fixture, "public fixture").unwrap();
    let profile = work.join("consumer.sb");
    fs::write(
        &profile,
        install::sandbox_profile(
            &work,
            &home,
            &base.join("toolchain"),
            &base.join("cache"),
            &secret,
            &producer,
        ),
    )
    .unwrap();
    let run = |args: &[&str]| {
        Command::new("/usr/bin/sandbox-exec")
            .arg("-f")
            .arg(&profile)
            .args(args)
            .output()
            .unwrap()
    };
    let denied = |args: &[&str]| {
        let o = run(args);
        let error = String::from_utf8_lossy(&o.stderr).to_lowercase();
        assert!(
            !error.contains("sandbox_apply:"),
            "sandbox setup failed: {error}"
        );
        assert!(!o.status.success());
        assert!(
            error.contains("operation not permitted") || error.contains("permission denied"),
            "{error}"
        );
    };
    let o = run(&["/bin/cat", fixture.to_str().unwrap()]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    assert_eq!(o.stdout, b"public fixture");
    denied(&["/bin/cat", secret.to_str().unwrap()]);
    denied(&["/bin/cat", producer.join("source").to_str().unwrap()]);
    let probe = work.join("network-probe");
    fs::copy(env!("CARGO_BIN_EXE_adl-public-contracts"), &probe).unwrap();
    denied(&[probe.to_str().unwrap(), "network-probe"]);
    let sibling = base.join("unrelated-source");
    fs::write(&sibling, "must remain unreadable").unwrap();
    let original = fs::read_to_string(&profile).unwrap();
    let extra = format!(
        "(allow default)\n(deny file-read* (subpath {}))",
        serde_json::to_string(&base).unwrap()
    );
    fs::write(&profile, original.replace("(allow default)", &extra)).unwrap();
    let o = run(&["/usr/bin/stat", "-f", "%i", base.to_str().unwrap()]);
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    denied(&["/bin/cat", sibling.to_str().unwrap()]);
}
