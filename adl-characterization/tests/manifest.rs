use std::fs;

use adl_characterization::load_corpus;

fn corpus_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/v1/corpus.yaml")
}

#[test]
fn canonical_corpus_is_schema_and_semantically_complete() {
    let corpus = load_corpus(&corpus_path()).expect("canonical corpus");
    assert_eq!(corpus.cases.len(), 25);
    assert_eq!(corpus.required_behaviors.len(), 23);
    assert_eq!(corpus.coverage.len(), corpus.required_behaviors.len());
}

#[test]
fn fewer_than_three_repetitions_is_rejected() {
    let temp = tempfile::tempdir().unwrap();
    let original = fs::read_to_string(corpus_path()).unwrap();
    fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/v1/schema.json"),
        temp.path().join("schema.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("corpus.yaml"),
        original.replacen("repetitions: 3", "repetitions: 2", 1),
    )
    .unwrap();
    let error = load_corpus(&temp.path().join("corpus.yaml")).unwrap_err();
    assert!(error.to_string().contains("schema validation failed"));
}

#[test]
fn schema_path_traversal_is_rejected_before_reading() {
    let temp = tempfile::tempdir().unwrap();
    let original = fs::read_to_string(corpus_path()).unwrap();
    fs::write(
        temp.path().join("corpus.yaml"),
        original.replacen("schema_path: schema.json", "schema_path: ../schema.json", 1),
    )
    .unwrap();
    let error = load_corpus(&temp.path().join("corpus.yaml")).unwrap_err();
    assert!(error.to_string().contains("clean relative path"));
}

#[test]
fn coverage_must_exactly_match_required_behaviors() {
    let temp = tempfile::tempdir().unwrap();
    let original = fs::read_to_string(corpus_path()).unwrap();
    fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/v1/schema.json"),
        temp.path().join("schema.json"),
    )
    .unwrap();
    fs::write(
        temp.path().join("corpus.yaml"),
        original.replace("  - {behavior: cli-version, cases: [cli-version]}\n", ""),
    )
    .unwrap();
    let error = load_corpus(&temp.path().join("corpus.yaml")).unwrap_err();
    assert!(error
        .to_string()
        .contains("coverage map does not exactly cover required behaviors"));
}
