//! PVF local_contract: deterministic native routing and Git refusal fixtures; no providers.
use adl_ci_contracts::ci::*;
use std::{collections::BTreeMap, fs, process::Command};
fn selected(paths: &[&str]) -> BTreeMap<String, String> {
    select(
        "pull_request",
        &paths.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
    )
}
#[test]
fn docs_only() {
    let d = selected(&["docs/design.md"]);
    assert_eq!(d["public_adl_validation_required"], "false");
    assert_eq!(d["transpiler_distribution_required"], "false");
    assert_eq!(d["ci_contracts_required"], "true");
}
#[test]
fn conservative_paths() {
    for path in [
        "adl/src/main.rs",
        "adl/tools/install_owner_binaries.sh",
        "adl-runtime/src/lib.rs",
        "unknown/value.bin",
        "docs/api/contract.json",
    ] {
        assert_eq!(selected(&[path])["public_adl_validation_required"], "true");
    }
}
#[test]
fn manual_and_empty() {
    assert_eq!(
        select("workflow_dispatch", &["docs/x.md".into()])["public_adl_validation_required"],
        "true"
    );
    assert_eq!(selected(&[])["public_adl_validation_required"], "true");
}
#[test]
fn every_selected_result_pair() {
    for selected in [
        Some("true"),
        Some("false"),
        Some(""),
        Some("malformed"),
        None,
    ] {
        for result in [
            Some("success"),
            Some("skipped"),
            Some("failure"),
            Some("cancelled"),
            Some(""),
            Some("malformed"),
            None,
        ] {
            assert_eq!(
                validate_result(selected, result).is_ok(),
                matches!(
                    (selected, result),
                    (Some("true"), Some("success")) | (Some("false"), Some("skipped"))
                )
            );
        }
    }
}
#[test]
fn aggregation_requires_producers() {
    let good: BTreeMap<String, String> = [
        ("PATH_POLICY_RESULT", "success"),
        ("PUBLIC_REQUIRED", "true"),
        ("PUBLIC_RESULT", "success"),
        ("CONTRACT_RESULT", "success"),
        ("TRANSPILE_REQUIRED", "false"),
        ("TRANSPILE_RESULT", "skipped"),
    ]
    .into_iter()
    .map(|(k, v)| (k.into(), v.into()))
    .collect();
    assert!(aggregate("ci", &good).is_ok());
    for field in ["PATH_POLICY_RESULT", "CONTRACT_RESULT"] {
        for value in ["skipped", "failure", "cancelled", "", "malformed"] {
            let mut bad = good.clone();
            bad.insert(field.into(), value.into());
            assert!(aggregate("ci", &bad).is_err());
        }
    }
}
#[test]
fn real_git_deletions_spaces_and_bad_refs() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let git = |argv: &[&str]| {
        let o = Command::new("git")
            .args(argv)
            .current_dir(root)
            .output()
            .unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        String::from_utf8(o.stdout).unwrap().trim().to_owned()
    };
    git(&["init", "-q"]);
    git(&["config", "user.name", "fixture"]);
    git(&["config", "user.email", "fixture@example.invalid"]);
    fs::create_dir(root.join("adl-runtime")).unwrap();
    fs::write(root.join("adl-runtime/source.rs"), "fixture").unwrap();
    git(&["add", "."]);
    git(&["-c", "commit.gpgsign=false", "commit", "-qm", "base"]);
    let base = git(&["rev-parse", "HEAD"]);
    fs::remove_file(root.join("adl-runtime/source.rs")).unwrap();
    fs::write(root.join("unknown file.txt"), "fixture").unwrap();
    git(&["add", "-A"]);
    git(&["-c", "commit.gpgsign=false", "commit", "-qm", "change"]);
    assert_eq!(
        paths(root, &base, "HEAD").unwrap(),
        vec!["adl-runtime/source.rs", "unknown file.txt"]
    );
    assert!(paths(root, "missing-ref", "HEAD").is_err());
    assert!(paths(root, "--help", "HEAD").is_err());
    let output = root.join("output");
    fs::write(&output, "existing=true\n").unwrap();
    let o = Command::new(env!("CARGO_BIN_EXE_adl-ci-contracts"))
        .args([
            "ci",
            "--event-name",
            "pull_request",
            "--base",
            &base,
            "--github-output",
            output.to_str().unwrap(),
        ])
        .current_dir(root)
        .output()
        .unwrap();
    assert!(o.status.success());
    assert!(String::from_utf8_lossy(&o.stdout).contains("public_adl_validation_required=true\n"));
    assert!(fs::read_to_string(output)
        .unwrap()
        .starts_with("existing=true\n"));
}
