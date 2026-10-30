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
fn prose_readmes_skip_compilation_but_keep_contracts() {
    // PVF local_contract: deterministic routing; tiny path-only inputs, no IO or providers.
    // Release gate: ci / adl-tooling-contracts, not installed-product qualification.
    for path in [
        "README.md",
        "adl/README.md",
        "adl/examples/README.md",
        "adl/tools/README.md",
        "adl-schema/README.md",
        "adl-uts/README.md",
        "adl-legacy-contracts/README.md",
        "adl-v2/README.md",
        "adl-v2/crates/adl-language/README.md",
        "adl-v2/crates/adl-compiler/README.md",
        "adl-spec/examples/README.md",
        "demos/rust-transpiler/README.md",
        "docs/tooling/README.md",
        "tools/transpiler_distribution/README.md",
    ] {
        let d = selected(&[path]);
        assert_eq!(d["public_adl_validation_required"], "false", "{path}");
        assert_eq!(d["transpiler_distribution_required"], "false", "{path}");
        assert_eq!(d["ci_contracts_required"], "true");
    }
}
#[test]
fn markdown_data_and_policy_keep_full_gates() {
    for path in [
        "adl-legacy-contracts/fixtures/README.md",
        "adl-legacy-contracts/fixtures/gate.md",
        "adl-uts/resources/README.md",
        "adl-spec/schemas/README.md",
        "docs/templates/planning/fixtures/minimal/readme.md",
        "docs/templates/README.md",
        "docs/repository-setup/1314/scaffold/README.md",
        "unknown/README.md",
        "adl/src/README.md",
        "tools/godel_contract_distribution/README.md",
        "tools/source_retirement/README.md",
        ".github/README.md",
    ] {
        assert_eq!(
            selected(&[path])["public_adl_validation_required"],
            "true",
            "{path}"
        );
    }
    for paths in [
        vec!["adl/README.md", "adl/src/main.rs"],
        vec!["adl/src/main.rs", "adl/README.md"],
        vec!["demos/rust-transpiler/README.md", "Cargo.lock"],
        vec!["README.md", ".github/workflows/ci.yaml"],
    ] {
        assert_eq!(selected(&paths)["public_adl_validation_required"], "true");
    }
    assert_eq!(
        selected(&["demos/rust-transpiler/Cargo.toml"])["transpiler_distribution_required"],
        "true"
    );
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

#[test]
fn naming_guard_checks_unrelated_roots_and_rejects_unknown_base() {
    // PVF local_contract: deterministic local Git fixtures, no remote or provider.
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let temp = tempfile::tempdir().unwrap();
    let git = |args: &[&str]| {
        let result = Command::new("git")
            .args(args)
            .current_dir(temp.path())
            .env("GIT_AUTHOR_NAME", "Fixture")
            .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
            .env("GIT_COMMITTER_NAME", "Fixture")
            .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        String::from_utf8(result.stdout).unwrap().trim().to_owned()
    };
    git(&["init", "-q"]);
    fs::write(temp.path().join("fixture.txt"), "inherited text\n").unwrap();
    git(&["add", "fixture.txt"]);
    git(&["commit", "-qm", "old"]);
    let baseline = git(&["rev-parse", "HEAD"]);
    git(&["switch", "--orphan", "new-root"]);
    fs::write(
        temp.path().join("fixture.txt"),
        "inherited text\nnew safe text\n",
    )
    .unwrap();
    git(&["add", "fixture.txt"]);
    git(&["commit", "-qm", "new root"]);
    let guard = |base: &str| {
        Command::new("bash")
            .arg(root.join("adl/tools/check_no_new_legacy_swarm_refs.sh"))
            .arg(base)
            .current_dir(temp.path())
            .output()
            .unwrap()
    };
    assert!(guard(&baseline).status.success());
    fs::write(temp.path().join("new.txt"), "swarm\n").unwrap();
    git(&["add", "new.txt"]);
    git(&["commit", "-qm", "unintended legacy reference"]);
    assert!(!guard(&baseline).status.success());
    assert!(!guard("missing-ref").status.success());
}
