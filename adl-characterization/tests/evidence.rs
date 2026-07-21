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
        let value = fs::read_to_string(&path)
            .unwrap()
            .replace("0.91.7", "0.91.X");
        fs::write(path, value).unwrap();
    }
    let error = verify_corpus(&corpus, temp.path()).unwrap_err();
    assert!(error.to_string().contains("repeated-run divergence"));
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
