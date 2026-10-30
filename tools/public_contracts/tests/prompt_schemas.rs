// PVF: local deterministic contract; bounded filesystem fixtures; public schema
// consistency proof only, not private owner qualification or a release gate.
use adl_public_contracts::{prompt_schemas::check, root};
use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};

fn write(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, value.to_string()).unwrap();
}
fn fixture() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    let registry: Value = serde_json::from_slice(
        &fs::read(root().join("docs/templates/prompts/current.json")).unwrap(),
    )
    .unwrap();
    write(
        &temp.path().join("docs/templates/prompts/current.json"),
        &registry,
    );
    for entry in registry["templates"].as_object().unwrap().values() {
        for field in ["path", "structure_schema_path"] {
            let relative = entry[field].as_str().unwrap();
            let destination = temp.path().join(relative);
            fs::create_dir_all(destination.parent().unwrap()).unwrap();
            fs::copy(root().join(relative), destination).unwrap();
        }
    }
    temp
}
#[test]
fn supported_versions_and_interpreter_free_cli() {
    for set in ["1.0.0", "1.0.1", "1.0.2", "1.0.3", "1.0.4", "1.0.5"] {
        assert_eq!(check(&root(), Some(set), false).unwrap(), set);
    }
    assert_eq!(check(&root(), None, true).unwrap(), "1.0.5");
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .args(["prompt-schemas", "--active-registry"])
        .env("PATH", "")
        .current_dir(std::env::temp_dir())
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout).contains("JSON artifacts validated"));
    for args in [
        vec!["--bad"],
        vec!["--template-set"],
        vec!["--template-set", "../1.0.5"],
        vec!["--template-set", "1.0.5", "extra"],
    ] {
        assert!(!Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
            .arg("prompt-schemas")
            .args(args)
            .output()
            .unwrap()
            .status
            .success());
    }
}
#[test]
fn refuses_schema_identity_shape_and_missing_artifacts() {
    let temp = fixture();
    let registry: Value = serde_json::from_slice(
        &fs::read(temp.path().join("docs/templates/prompts/current.json")).unwrap(),
    )
    .unwrap();
    let path = temp.path().join(
        registry["templates"]["sip"]["structure_schema_path"]
            .as_str()
            .unwrap(),
    );
    let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for field in [
        "schema",
        "template_set",
        "card_kind",
        "template_path",
        "parser",
        "editable_sections",
        "frontmatter_keys",
        "headings",
        "fenced_blocks",
        "locked_lines",
    ] {
        let mut changed = original.clone();
        changed[field] = Value::Null;
        write(&path, &changed);
        assert!(check(temp.path(), None, false).is_err(), "{field}");
    }
    let mut changed = original.clone();
    changed["headings"] = json!([]);
    write(&path, &changed);
    assert!(check(temp.path(), None, false).is_err());
    fs::write(&path, "{").unwrap();
    assert!(check(temp.path(), None, false).is_err());
    fs::remove_file(&path).unwrap();
    assert!(check(temp.path(), None, false).is_err());
}
#[test]
fn active_registry_checks_preserve_wrapper_contract() {
    let temp = fixture();
    let path = temp.path().join("docs/templates/prompts/current.json");
    let original: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
    for (field, value) in [
        ("status", json!("draft")),
        ("semver", json!("1.0.4")),
        ("templates", json!([])),
        ("csdlc_prompt_template_set", Value::Null),
    ] {
        let mut changed = original.clone();
        changed[field] = value;
        write(&path, &changed);
        assert!(check(temp.path(), None, true).is_err(), "{field}");
    }
    let mut changed = original.clone();
    changed["templates"]["extra"] = changed["templates"]["sip"].clone();
    write(&path, &changed);
    assert!(check(temp.path(), None, true).is_err());
    assert!(check(temp.path(), None, false).is_ok());
    write(&path, &original);
    assert!(check(temp.path(), Some("1.0.5"), true).is_err());
    fs::remove_file(
        temp.path()
            .join(original["templates"]["sip"]["path"].as_str().unwrap()),
    )
    .unwrap();
    assert!(check(temp.path(), None, true).is_err());
    assert!(check(temp.path(), None, false).is_ok());
}
