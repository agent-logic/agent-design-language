use std::fs;

use adl_characterization::load_corpus;

fn corpus_path() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/v1/corpus.yaml")
}

fn prepare_corpus(original: &str) -> (tempfile::TempDir, std::path::PathBuf) {
    let temp = tempfile::tempdir().unwrap();
    let canonical = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("corpus/v1");
    fs::copy(
        canonical.join("schema.json"),
        temp.path().join("schema.json"),
    )
    .unwrap();
    fs::create_dir_all(temp.path().join("fixtures")).unwrap();
    fs::copy(
        canonical.join("fixtures/mock-run.adl.yaml"),
        temp.path().join("fixtures/mock-run.adl.yaml"),
    )
    .unwrap();
    let path = temp.path().join("corpus.yaml");
    fs::write(&path, original).unwrap();
    (temp, path)
}

#[test]
fn canonical_corpus_is_schema_and_semantically_complete() {
    let corpus = load_corpus(&corpus_path()).expect("canonical corpus");
    assert_eq!(corpus.cases.len(), 25);
    assert_eq!(corpus.required_behaviors.len(), 23);
    assert_eq!(corpus.coverage.len(), corpus.required_behaviors.len());
}

// PVF: manifest/local_contract; deterministic, tiny filesystem fixtures;
// required regression proof. No external services or provider execution.
#[test]
fn remote_schema_references_are_refused_without_http_resolution() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    for scheme in ["http", "https"] {
        let (temp, path) = prepare_corpus(&original);
        let schema = serde_json::json!({
            "$ref": format!("{scheme}://127.0.0.1:9/schema.json")
        });
        fs::write(temp.path().join("schema.json"), schema.to_string()).unwrap();
        let error = load_corpus(&path).unwrap_err().to_string();
        assert!(error.contains("schema validation failed"), "{error}");
        assert!(
            error.contains("`resolve-http` feature or a custom resolver is required"),
            "{error}"
        );
    }
}

#[test]
fn local_file_schema_references_preserve_validation() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let (temp, path) = prepare_corpus(&original);
    let referenced = temp.path().join("referenced schema #1.json");
    let local_schema = serde_json::json!({
        "type": "object",
        "properties": {"repetitions": {"type": "integer", "minimum": 3}}
    });
    fs::write(&referenced, local_schema.to_string()).unwrap();
    let uri = url::Url::from_file_path(referenced.canonicalize().unwrap()).unwrap();
    let schema = serde_json::json!({"$ref": uri.as_str()});
    fs::write(temp.path().join("schema.json"), schema.to_string()).unwrap();
    assert_eq!(load_corpus(&path).unwrap().cases.len(), 25);
    fs::write(
        &path,
        original.replacen("repetitions: 3", "repetitions: 2", 1),
    )
    .unwrap();
    let error = load_corpus(&path).unwrap_err().to_string();
    assert!(error.contains("schema validation failed"), "{error}");
    assert!(error.contains("less than the minimum"), "{error}");
}

#[test]
fn fewer_than_three_repetitions_is_rejected() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let (_temp, path) = prepare_corpus(&original.replacen("repetitions: 3", "repetitions: 2", 1));
    let error = load_corpus(&path).unwrap_err();
    assert!(error.to_string().contains("schema validation failed"));
}

#[test]
fn schema_path_traversal_is_rejected_before_reading() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let (_temp, path) = prepare_corpus(&original.replacen(
        "schema_path: schema.json",
        "schema_path: ../schema.json",
        1,
    ));
    let error = load_corpus(&path).unwrap_err();
    assert!(error.to_string().contains("clean relative path"));
}

#[test]
fn coverage_must_exactly_match_required_behaviors() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let (_temp, path) = prepare_corpus(
        &original.replace("  - {behavior: cli-version, cases: [cli-version]}\n", ""),
    );
    let error = load_corpus(&path).unwrap_err();
    assert!(error
        .to_string()
        .contains("coverage map does not exactly cover required behaviors"));
}

#[test]
fn network_capable_execution_is_rejected_by_corpus_policy() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let changed = original.replacen(
        "args: [\"{ROOT}/fixtures/six-primitives.adl.yaml\", --print-plan]",
        "args: [\"{ROOT}/fixtures/six-primitives.adl.yaml\", --run]",
        1,
    );
    let (_temp, path) = prepare_corpus(&changed);
    let error = load_corpus(&path).unwrap_err();
    assert!(error
        .to_string()
        .contains("outside the local-only command policy"));
}

#[test]
fn default_document_execution_is_rejected_by_corpus_policy() {
    let original = fs::read_to_string(corpus_path()).unwrap();
    let changed = original.replacen(
        "args: [\"{ROOT}/fixtures/six-primitives.adl.yaml\", --print-plan]",
        "args: [\"{ROOT}/fixtures/six-primitives.adl.yaml\"]",
        1,
    );
    let (_temp, path) = prepare_corpus(&changed);
    let error = load_corpus(&path).unwrap_err();
    assert!(error
        .to_string()
        .contains("outside the local-only command policy"));
}
