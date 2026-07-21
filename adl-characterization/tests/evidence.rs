use std::fs;
use std::path::{Path, PathBuf};

use adl_characterization::{load_corpus, verify_corpus, RawObservation};
use sha2::{Digest, Sha256};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).to_path_buf()
}

#[test]
fn retained_three_run_evidence_verifies() {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let report = verify_corpus(&corpus_path, &corpus, &root().join("observations/v1")).unwrap();
    assert_eq!(report.status, "pass");
    assert_eq!(report.observation_count, 75);
}

#[test]
fn tampered_normalized_evidence_is_rejected() {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let target = temp.path().join("cli-version/01.normalized.json");
    let value = fs::read_to_string(&target)
        .unwrap()
        .replace("0.91.7", "0.91.8");
    fs::write(target, value).unwrap();
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains("stale"));
}

#[test]
fn repeated_run_divergence_is_rejected_even_when_retained_derivation_matches() {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let raw = temp.path().join("cli-version/02.raw.json");
    let normalized = temp.path().join("cli-version/02.normalized.json");
    for path in [&raw, &normalized] {
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        value["commands"][0]["stdout"] = serde_json::json!("0.91.7\nextra\n");
        fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }
    rehash_raw(&raw, true);
    sync_command_digests(&raw, &normalized);
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains("repeated-run divergence"));
}

#[test]
fn committed_evidence_contains_no_absolute_host_paths() {
    for path in walk(&root().join("observations/v1")) {
        if path.is_file() {
            let text = fs::read_to_string(&path).unwrap();
            for prefix in ["/Users/", "/private/var/", "/Volumes/", "/tmp/", "/home/"] {
                assert!(
                    !text.contains(prefix),
                    "host path {prefix} in {}",
                    path.display()
                );
            }
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
fn offline_verify_rejects_expanded_argument_tamper() {
    assert_contract_tamper(
        "cli-version/01.raw.json",
        |value| {
            value["commands"][0]["expanded_args"][0] = serde_json::json!("--help");
        },
        "expanded args",
    );
}

#[test]
fn offline_verify_rejects_portable_stream_hash_tamper() {
    assert_raw_tamper(
        "cli-version/01.raw.json",
        |value| {
            value.commands[0].portable_stdout_sha256 = "0".repeat(64);
        },
        false,
        "portable stream digest mismatch",
    );
}

#[test]
fn offline_verify_rejects_captured_stream_hash_tamper() {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let path = temp.path().join("cli-version/01.raw.json");
    let mut raw: RawObservation = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    raw.commands[0].captured_stdout_sha256 = "0".repeat(64);
    fs::write(path, serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("evidence envelope digest mismatch"),
        "{error:#}"
    );
}

#[test]
fn offline_verify_rejects_joint_raw_and_normalized_stream_mutation() {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let raw_path = temp.path().join("cli-version/01.raw.json");
    let normalized_path = temp.path().join("cli-version/01.normalized.json");
    for path in [&raw_path, &normalized_path] {
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        value["commands"][0]["stdout"] = serde_json::json!("0.91.7\nforged\n");
        fs::write(path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    }
    rehash_raw(&raw_path, false);
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(
        error
            .to_string()
            .contains("portable stream digest mismatch"),
        "{error:#}"
    );
}

#[test]
fn offline_verify_rejects_corpus_bundle_drift() {
    let temp = tempfile::tempdir().unwrap();
    let copied_corpus = temp.path().join("corpus");
    copy_tree(&root().join("corpus/v1"), &copied_corpus);
    let corpus_path = copied_corpus.join("corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let coverage = copied_corpus.join("COVERAGE.md");
    let mut text = fs::read_to_string(&coverage).unwrap();
    text.push_str("\n<!-- drift -->\n");
    fs::write(coverage, text).unwrap();
    let error = verify_corpus(&corpus_path, &corpus, &root().join("observations/v1")).unwrap_err();
    assert!(error.to_string().contains("observation identity mismatch"));
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
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let path = temp.path().join(relative);
    let mut value: serde_json::Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut value);
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    rehash_raw(&path, true);
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains(expected), "{error:#}");
}

fn assert_raw_tamper(
    relative: &str,
    mutate: impl FnOnce(&mut RawObservation),
    refresh_portable_hashes: bool,
    expected: &str,
) {
    let corpus_path = root().join("corpus/v1/corpus.yaml");
    let corpus = load_corpus(&corpus_path).unwrap();
    let temp = tempfile::tempdir().unwrap();
    copy_tree(&root().join("observations/v1"), temp.path());
    let path = temp.path().join(relative);
    let mut raw: RawObservation = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    mutate(&mut raw);
    if refresh_portable_hashes {
        refresh_stream_hashes(&mut raw);
    }
    raw.evidence_envelope_sha256 = raw.compute_evidence_envelope_sha256().unwrap();
    fs::write(path, serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
    let error = verify_corpus(&corpus_path, &corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains(expected), "{error:#}");
}

fn rehash_raw(path: &Path, refresh_portable_hashes: bool) {
    let mut raw: RawObservation = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    if refresh_portable_hashes {
        refresh_stream_hashes(&mut raw);
    }
    raw.evidence_envelope_sha256 = raw.compute_evidence_envelope_sha256().unwrap();
    fs::write(path, serde_json::to_vec_pretty(&raw).unwrap()).unwrap();
}

fn refresh_stream_hashes(raw: &mut RawObservation) {
    for command in &mut raw.commands {
        command.portable_stdout_sha256 = format!("{:x}", Sha256::digest(command.stdout.as_bytes()));
        command.portable_stderr_sha256 = format!("{:x}", Sha256::digest(command.stderr.as_bytes()));
    }
}

fn sync_command_digests(raw_path: &Path, normalized_path: &Path) {
    let raw: serde_json::Value = serde_json::from_slice(&fs::read(raw_path).unwrap()).unwrap();
    let mut normalized: serde_json::Value =
        serde_json::from_slice(&fs::read(normalized_path).unwrap()).unwrap();
    for field in [
        "captured_stdout_sha256",
        "captured_stderr_sha256",
        "portable_stdout_sha256",
        "portable_stderr_sha256",
    ] {
        normalized["commands"][0][field] = raw["commands"][0][field].clone();
    }
    fs::write(
        normalized_path,
        serde_json::to_vec_pretty(&normalized).unwrap(),
    )
    .unwrap();
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
