use std::fs;
use std::path::{Path, PathBuf};

use adl_characterization::{load_corpus, verify_corpus};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn retained_three_run_evidence_verifies() {
    let corpus = load_corpus(&root().join("corpus/v1/corpus.yaml")).unwrap();
    let report = verify_corpus(&corpus, &root().join("observations/v1")).unwrap();
    assert_eq!(report.status, "pass");
    assert_eq!(report.observation_count, 75);
}

#[test]
fn tampered_normalized_evidence_is_rejected() {
    let corpus = load_corpus(&root().join("corpus/v1/corpus.yaml")).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let target = temp.path().join("cli-version/01.normalized.json");
    let value = fs::read_to_string(&target)
        .unwrap()
        .replace("0.91.7", "0.91.8");
    fs::write(target, value).unwrap();
    let error = verify_corpus(&corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains("stale"));
}

#[test]
fn repeated_run_divergence_is_rejected_even_when_retained_derivation_matches() {
    let corpus = load_corpus(&root().join("corpus/v1/corpus.yaml")).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let raw = temp.path().join("cli-version/02.raw.json");
    let normalized = temp.path().join("cli-version/02.normalized.json");
    for path in [raw, normalized] {
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        value["commands"][0]["stdout"] = serde_json::json!("0.91.7\nextra\n");
        fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }
    let error = verify_corpus(&corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains("repeated-run divergence"));
}

#[test]
fn committed_evidence_contains_no_absolute_host_paths() {
    for path in walk(&root().join("observations/v1")) {
        if path.is_file() {
            let text = fs::read_to_string(&path).unwrap();
            assert!(!text.contains("/Users/"), "host path in {}", path.display());
            assert!(
                !text.contains("/private/var/"),
                "temp path in {}",
                path.display()
            );
        }
    }
}

#[test]
fn offline_verify_rejects_command_count_tamper() {
    assert_contract_tamper(
        "cli-version/01.raw.json",
        |value| {
            value["commands"].as_array_mut().unwrap().clear();
        },
        "command count",
    );
}

#[test]
fn offline_verify_rejects_step_order_tamper() {
    assert_contract_tamper(
        "ed25519-sign-verify-tamper/01.raw.json",
        |value| {
            value["commands"].as_array_mut().unwrap().swap(0, 1);
        },
        "command order",
    );
}

#[test]
fn offline_verify_rejects_declared_argument_tamper() {
    assert_contract_tamper(
        "cli-version/01.raw.json",
        |value| {
            value["commands"][0]["declared_args"][0] = serde_json::json!("--help");
        },
        "declared args",
    );
}

#[test]
fn offline_verify_rejects_exit_tamper() {
    assert_contract_tamper(
        "cli-version/01.raw.json",
        |value| {
            value["commands"][0]["exit_code"] = serde_json::json!(9);
        },
        "exit does not match",
    );
}

#[test]
fn offline_verify_rejects_stdout_fragment_tamper() {
    assert_contract_tamper(
        "cli-version/01.raw.json",
        |value| {
            value["commands"][0]["stdout"] = serde_json::json!("changed\n");
        },
        "stdout misses",
    );
}

#[test]
fn offline_verify_rejects_stderr_fragment_tamper() {
    assert_contract_tamper(
        "invalid-argument/01.raw.json",
        |value| {
            value["commands"][0]["stderr"] = serde_json::json!("changed\n");
        },
        "stderr misses",
    );
}

fn assert_contract_tamper(
    relative: &str,
    mutate: impl FnOnce(&mut serde_json::Value),
    expected: &str,
) {
    let corpus = load_corpus(&root().join("corpus/v1/corpus.yaml")).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let path = temp.path().join(relative);
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut value);
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    let error = verify_corpus(&corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains(expected), "{error:#}");
}

fn copy_tree(source: &Path, destination: &Path) {
    for entry in walk(source) {
        let relative = entry.strip_prefix(source).unwrap();
        let target = destination.join(relative);
        if entry.is_dir() {
            fs::create_dir_all(target).unwrap();
        } else {
            fs::copy(entry, target).unwrap();
        }
    }
}

fn walk(root: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(path) = pending.pop() {
        paths.push(path.clone());
        if path.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                pending.push(entry.unwrap().path());
            }
        }
    }
    paths
}
