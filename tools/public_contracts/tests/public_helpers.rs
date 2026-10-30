//! PVF local contract: deterministic fake-Cargo routing and output preservation.
use adl_public_contracts::{executable, public_docs, root};
use std::{fs, process::Command};
#[test]
fn documentation_negative_fixtures() {
    public_docs::self_test().unwrap();
}
#[test]
fn package_commands_and_failures() {
    for fail in ["", "test", "fmt", "clippy"] {
        let t = tempfile::tempdir().unwrap();
        let log = t.path().join("calls");
        executable(&t.path().join("cargo"),"#!/bin/sh\nprintf '%s\\n' \"$@\" >> \"$CALL_LOG\"\nprintf '%s\\n' END >> \"$CALL_LOG\"\nif [ \"$1\" = \"$FAIL_COMMAND\" ]; then exit 23; fi\n").unwrap();
        let output = Command::new("bash")
            .arg(root().join("tools/public_adl/run_package_checks.sh"))
            .current_dir(t.path())
            .env(
                "PATH",
                format!("{}:{}", t.path().display(), std::env::var("PATH").unwrap()),
            )
            .env("CALL_LOG", &log)
            .env("FAIL_COMMAND", fail)
            .output()
            .unwrap();
        let text = fs::read_to_string(log).unwrap();
        let calls: Vec<Vec<_>> = text
            .split("END\n")
            .filter(|s| !s.is_empty())
            .map(|s| s.lines().map(str::to_owned).collect())
            .collect();
        if fail.is_empty() {
            assert!(output.status.success());
            let mut expected = vec![];
            for (path, name) in [
                ("adl-schema", "adl-schema"),
                ("adl-uts", "adl-uts"),
                ("adl-v2/crates/adl-language", "adl-language"),
                ("adl-v2/crates/adl-compiler", "adl-compiler"),
                ("adl-legacy-contracts", "adl-legacy-contracts"),
            ] {
                let common = vec![
                    "--manifest-path".to_owned(),
                    format!("{path}/Cargo.toml"),
                    "--package".into(),
                    name.into(),
                ];
                for (cmd, locked, tail) in [
                    ("test", true, vec![]),
                    ("fmt", false, vec!["--", "--check"]),
                    (
                        "clippy",
                        true,
                        vec!["--all-targets", "--", "-D", "warnings"],
                    ),
                ] {
                    let mut args = vec![cmd.into()];
                    if locked {
                        args.push("--locked".into());
                    }
                    args.extend(common.clone());
                    args.extend(tail.into_iter().map(str::to_owned));
                    expected.push(args);
                }
            }
            assert_eq!(calls, expected);
        } else {
            assert_eq!(output.status.code(), Some(23));
            assert_eq!(
                calls.len(),
                match fail {
                    "test" => 1,
                    "fmt" => 2,
                    _ => 3,
                }
            );
            assert_eq!(calls.last().unwrap()[0], fail);
        }
    }
}
#[test]
fn demo_outputs_preserved() {
    let t = tempfile::tempdir().unwrap();
    let parent = t.path();
    let existing = parent.join("existing");
    fs::create_dir(&existing).unwrap();
    let sentinel = existing.join("sentinel");
    fs::write(&sentinel, b"must survive\0\xff").unwrap();
    let file = parent.join("file");
    fs::write(&file, "keep").unwrap();
    let dangling = parent.join("dangling");
    let linked = parent.join("linked");
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(parent.join("missing"), &dangling).unwrap();
        std::os::unix::fs::symlink(&existing, &linked).unwrap();
    }
    for demo in [
        "demo_v090_codebuddy_review_showcase.sh",
        "demo_v090_architecture_document_generation.sh",
        "demo_v0891_arxiv_manuscript_workflow.sh",
    ] {
        for target in [&existing, &file, &dangling, &linked, &parent.to_path_buf()] {
            let o = Command::new("bash")
                .arg(root().join("adl/tools").join(demo))
                .arg(target)
                .current_dir(root())
                .output()
                .unwrap();
            assert!(!o.status.success());
            assert!(
                String::from_utf8_lossy(&o.stderr).contains("demo output must be a new directory")
            );
            assert_eq!(fs::read(&sentinel).unwrap(), b"must survive\0\xff");
            assert_eq!(fs::read_to_string(&file).unwrap(), "keep");
            assert!(dangling.is_symlink() && !dangling.exists() && linked.is_symlink());
        }
    }
}
