//! PVF local_contract: deterministic offline cache filesystem proof; tiny temporary
//! artifacts, Unix hardlinks, no network/interpreter. Required cache replacement gate.
#![cfg(unix)]
use adl_public_contracts::warm_cache::{warm, wrapper, Options};
use std::{
    collections::BTreeMap,
    fs,
    os::unix::fs::{symlink, MetadataExt},
    path::Path,
};
fn write(path: &Path, bytes: &str) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, bytes).unwrap();
}
fn fixture() -> (tempfile::TempDir, Options) {
    let t = tempfile::tempdir().unwrap();
    let root = t.path();
    write(
        &root.join("project/Cargo.toml"),
        "[package]\nname='adl'\nversion='0.1.0'\n[workspace]\nmembers=[\n'crates/helper',\n]\n",
    );
    write(
        &root.join("project/crates/helper/Cargo.toml"),
        "[package]\nname='helper'\nversion='0.1.0'\n",
    );
    write(&root.join("project/Cargo.lock"),"version=4\n[[package]]\nname='adl'\nversion='0.1.0'\n[[package]]\nname='helper'\nversion='0.1.0'\n[[package]]\nname='serde'\nversion='1.0.0'\n[[package]]\nname='proc-macro2'\nversion='1.0.0'\n");
    for name in [
        "libserde-abc.rlib",
        "serde-abc.d",
        "libproc_macro2-def.rmeta",
        "libadl-aaa.rlib",
        "libhelper-bbb.rlib",
        "unmatched-file",
    ] {
        write(&root.join("source/debug/deps").join(name), name);
    }
    for name in [
        "serde-abc/lib-serde",
        "serde-abc/dep-lib-serde",
        "serde-abc/serde-abc.d",
        "helper-bbb/lib-helper",
        "unmatched-ccc/lib-unmatched",
    ] {
        write(&root.join("source/debug/.fingerprint").join(name), name);
    }
    write(
        &root.join("source/debug/build/libserde-build-output"),
        "do not link",
    );
    let options = Options {
        source: root.join("source"),
        destination: root.join("dest"),
        manifest: root.join("project/Cargo.toml"),
        profile: "debug".into(),
        dry_run: false,
        replace: false,
    };
    (t, options)
}
fn linked(a: &Path, b: &Path) {
    let a = fs::metadata(a).unwrap();
    let b = fs::metadata(b).unwrap();
    assert_eq!((a.dev(), a.ino()), (b.dev(), b.ino()));
}
#[test]
fn only_external_artifacts_and_fingerprints_are_linked() {
    let (_t, o) = fixture();
    let r = warm(&o).unwrap();
    assert_eq!(r["linked_files"], 4);
    assert_eq!(r["linked_fingerprint_files"], 2);
    assert_eq!(r["skipped_dep_info"], 2);
    assert_eq!(r["validation_proof"], false);
    for p in [
        "deps/libserde-abc.rlib",
        "deps/libproc_macro2-def.rmeta",
        ".fingerprint/serde-abc/lib-serde",
        ".fingerprint/serde-abc/dep-lib-serde",
    ] {
        linked(
            &o.source.join("debug").join(p),
            &o.destination.join("debug").join(p),
        );
    }
    for p in [
        "deps/libadl-aaa.rlib",
        "deps/libhelper-bbb.rlib",
        "deps/serde-abc.d",
        "deps/unmatched-file",
        "build",
        ".fingerprint/helper-bbb",
        ".fingerprint/unmatched-ccc",
        ".fingerprint/serde-abc/serde-abc.d",
    ] {
        assert!(!o.destination.join("debug").join(p).exists(), "{p}");
    }
}
#[test]
fn replacement_dry_run_and_same_inode_are_explicit() {
    let (_t, mut o) = fixture();
    o.dry_run = true;
    assert_eq!(warm(&o).unwrap()["linked_files"], 4);
    assert!(!o.destination.exists());
    o.dry_run = false;
    let dest = o.destination.join("debug/deps/libserde-abc.rlib");
    write(&dest, "original");
    assert_eq!(warm(&o).unwrap()["skipped_existing"], 1);
    assert_eq!(fs::read_to_string(&dest).unwrap(), "original");
    o.replace = true;
    assert_eq!(warm(&o).unwrap()["linked_files"], 1);
    linked(&o.source.join("debug/deps/libserde-abc.rlib"), &dest);
    assert_eq!(warm(&o).unwrap()["skipped_existing"], 4);
}
#[test]
fn later_failure_rolls_back_new_links_and_restores_replaced_files() {
    for replace in [false, true] {
        let (_t, mut o) = fixture();
        o.replace = replace;
        let dest = o.destination.join("debug/deps/libserde-abc.rlib");
        if replace {
            write(&dest, "original");
        }
        write(&o.destination.join("debug/.fingerprint"), "not directory");
        let r = warm(&o).unwrap();
        assert_eq!(r["status"], "completed_with_errors");
        assert!(r["rolled_back_files"].as_u64().unwrap() > 0);
        if replace {
            assert_eq!(fs::read_to_string(&dest).unwrap(), "original");
        } else {
            assert!(!dest.exists());
        }
        assert!(!o
            .destination
            .join("debug/deps/libproc_macro2-def.rmeta")
            .exists());
    }
}
#[test]
fn source_and_destination_symlinks_cannot_escape() {
    let (t, o) = fixture();
    let foreign = t.path().join("foreign");
    write(&foreign.join("original"), "untouched");
    fs::create_dir_all(o.destination.join("debug")).unwrap();
    symlink(&foreign, o.destination.join("debug/deps")).unwrap();
    let r = warm(&o).unwrap();
    assert_eq!(r["status"], "completed_with_errors");
    assert_eq!(fs::read_dir(&foreign).unwrap().count(), 1);
    symlink(
        foreign.join("original"),
        o.source.join("debug/deps/libserde-evil.rlib"),
    )
    .unwrap();
    assert!(warm(&o).is_err());
}
#[test]
fn invalid_profile_and_missing_lock_fail_without_copying() {
    let (_t, mut o) = fixture();
    for profile in ["/", "..", "../other", ""] {
        o.profile = profile.into();
        assert!(warm(&o).is_err());
    }
    o.profile = "debug".into();
    fs::remove_file(o.manifest.parent().unwrap().join("Cargo.lock")).unwrap();
    assert!(warm(&o).is_err());
    assert!(!o.destination.exists());
}
#[test]
fn wrapper_skip_and_json_escape_preserve_environment_contract() {
    let (t, o) = fixture();
    let mut env = BTreeMap::from([
        ("ADL_RUST_WARM_CACHE".into(), "0".into()),
        (
            "ADL_RUST_WARM_CACHE_SOURCE_TARGET".into(),
            "quote\"newline\n".into(),
        ),
    ]);
    let r = wrapper(t.path(), &env).unwrap();
    assert_eq!(r["reason"], "disabled");
    let text = r.to_string();
    assert!(serde_json::from_str::<serde_json::Value>(&text).is_ok());
    env.insert("ADL_RUST_WARM_CACHE".into(), "1".into());
    assert_eq!(
        wrapper(t.path(), &env).unwrap()["reason"],
        "manifest missing"
    );
    env.insert(
        "ADL_RUST_WARM_CACHE_MANIFEST_PATH".into(),
        o.manifest.display().to_string(),
    );
    assert_eq!(
        wrapper(t.path(), &env).unwrap()["reason"],
        "source target cache missing"
    );
    env.insert(
        "ADL_RUST_WARM_CACHE_SOURCE_TARGET".into(),
        o.source.display().to_string(),
    );
    env.insert(
        "ADL_RUST_WARM_CACHE_DEST_TARGET".into(),
        o.source.display().to_string(),
    );
    assert_eq!(
        wrapper(t.path(), &env).unwrap()["reason"],
        "source target is destination target"
    );
    env.insert(
        "ADL_RUST_WARM_CACHE_DEST_TARGET".into(),
        o.destination.display().to_string(),
    );
    assert_eq!(wrapper(t.path(), &env).unwrap()["status"], "ok");
}

#[test]
fn cli_emits_valid_json_and_failure_codes_without_interpreters() {
    use std::process::Command;
    let (_t, o) = fixture();
    let binary = env!("CARGO_BIN_EXE_adl-public-contracts");
    let invoke = |options: &Options| {
        Command::new(binary)
            .env("PATH", "")
            .args(["warm-cache", "--source-target"])
            .arg(&options.source)
            .arg("--dest-target")
            .arg(&options.destination)
            .arg("--manifest-path")
            .arg(&options.manifest)
            .arg("--json")
            .output()
            .unwrap()
    };
    let success = invoke(&o);
    assert!(success.status.success());
    let report: serde_json::Value = serde_json::from_slice(&success.stdout).unwrap();
    assert_eq!(report["linked_files"], 4);
    assert!(success.stderr.is_empty());
    let (_t, bad) = fixture();
    write(&bad.destination.join("debug/.fingerprint"), "block");
    let failure = invoke(&bad);
    assert_eq!(failure.status.code(), Some(2));
    let report: serde_json::Value = serde_json::from_slice(&failure.stdout).unwrap();
    assert_eq!(report["status"], "completed_with_errors");
    assert!(!failure.stderr.is_empty());
    let mut missing = bad;
    missing.manifest = missing.manifest.with_file_name("missing.toml");
    let failure = invoke(&missing);
    assert_eq!(failure.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&failure.stdout).unwrap();
    assert_eq!(report["status"], "error");
}

#[test]
fn wrapper_output_file_matches_stdout_and_has_no_python_dependency() {
    use std::process::Command;
    let t = tempfile::tempdir().unwrap();
    let output_file = t.path().join("status.json");
    let result = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .env("PATH", "")
        .env("ADL_RUST_WARM_CACHE", "0")
        .env("ADL_RUST_WARM_CACHE_OUTPUT", &output_file)
        .arg("warm-cache-wrapper")
        .output()
        .unwrap();
    assert!(result.status.success());
    assert!(result.stderr.is_empty());
    assert_eq!(fs::read(&output_file).unwrap(), result.stdout);
    let report: serde_json::Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["reason"], "disabled");
    assert_eq!(report["validation_proof"], false);
}
#[test]
fn wrapper_discovers_primary_target_for_legacy_worktree_layout() {
    let (t, o) = fixture();
    let primary = t.path().join("primary");
    let wt = primary.join(".worktrees/adl-wp-fixture");
    fs::create_dir_all(primary.join("adl/target/debug/deps")).unwrap();
    fs::create_dir_all(&wt).unwrap();
    let env = BTreeMap::from([
        (
            "ADL_RUST_WARM_CACHE_MANIFEST_PATH".into(),
            o.manifest.display().to_string(),
        ),
        (
            "ADL_RUST_WARM_CACHE_DEST_TARGET".into(),
            primary.join("adl/target").display().to_string(),
        ),
    ]);
    let r = wrapper(&wt, &env).unwrap();
    assert_eq!(r["reason"], "source target is destination target");
}
#[test]
fn wrapper_skips_cross_device_when_second_filesystem_is_available() {
    let (_t, o) = fixture();
    let second = std::env::var_os("ADL_WARM_CACHE_CROSS_DEVICE_DEST_PARENT")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| "/Volumes/FastWork".into());
    let Ok(second_metadata) = fs::metadata(&second) else {
        eprintln!("cross-device case skipped: second filesystem unavailable");
        return;
    };
    if second_metadata.dev() == fs::metadata(&o.source).unwrap().dev() {
        eprintln!("cross-device case skipped: same filesystem");
        return;
    }
    let dest = tempfile::Builder::new()
        .prefix("adl-warm-native-cross-device-")
        .tempdir_in(second)
        .unwrap();
    let env = BTreeMap::from([
        (
            "ADL_RUST_WARM_CACHE_SOURCE_TARGET".into(),
            o.source.display().to_string(),
        ),
        (
            "ADL_RUST_WARM_CACHE_MANIFEST_PATH".into(),
            o.manifest.display().to_string(),
        ),
        (
            "ADL_RUST_WARM_CACHE_DEST_TARGET".into(),
            dest.path().display().to_string(),
        ),
    ]);
    let r = wrapper(Path::new("."), &env).unwrap();
    assert_eq!(
        r["reason"],
        "source and destination target are on different filesystems; hardlink warm cache skipped"
    );
    assert_eq!(fs::read_dir(dest.path()).unwrap().count(), 0);
}
