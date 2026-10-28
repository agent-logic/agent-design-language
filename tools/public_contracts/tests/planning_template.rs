//! PVF local_contract: offline deterministic planning renderer/structure checks;
//! small temporary files, no interpreters/network. Not review or release approval.
use adl_public_contracts::planning_template::{failures, fill};
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};
fn write(p: &Path, s: &str) {
    fs::create_dir_all(p.parent().unwrap()).unwrap();
    fs::write(p, s).unwrap();
}
struct Fixture {
    _tmp: tempfile::TempDir,
    root: PathBuf,
    registry: PathBuf,
    values: PathBuf,
    output: PathBuf,
    document: Value,
}
impl Fixture {
    fn new() -> Self {
        let tmp = tempfile::tempdir().unwrap();
        let root = tmp.path().to_path_buf();
        let registry = root.join("docs/templates/planning/current.json");
        let values = root.join("values.json");
        let output = root.join("output.md");
        write(
            &root.join("docs/templates/planning/1.0.0/test.md"),
            "# Example\n## Metadata\n<name>\n## Status\n<status>\n",
        );
        write(&values, r#"{"name":"Example","status":"draft"}"#);
        let document = json!({"schema":"adl.planning_template_registry.v1","planning_template_set":"1.0.0","template_root":"docs/templates/planning/1.0.0","templates":{"test":{"status":"active","path":"docs/templates/planning/1.0.0/test.md","version":"1.0.0","required_sections":["Metadata","Status"]}}});
        let f = Self {
            _tmp: tmp,
            root,
            registry,
            values,
            output,
            document,
        };
        f.save();
        f
    }
    fn save(&self) {
        write(&self.registry, &self.document.to_string());
    }
    fn call(&self, validate: bool, extra: &[&str]) -> Output {
        let mut c = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"));
        c.env("PATH", "")
            .current_dir(&self.root)
            .arg(if validate {
                "planning-validate"
            } else {
                "planning-fill"
            })
            .arg("--registry")
            .arg(&self.registry)
            .args(["--template", "test"]);
        if validate {
            c.arg("--input").arg(&self.output);
        } else {
            c.arg("--values")
                .arg(&self.values)
                .arg("--output")
                .arg(&self.output);
        }
        c.args(extra).output().unwrap()
    }
}
#[test]
fn explicit_values_preserve_rendering_and_missing_keys() {
    let values = json!({"text":"hello","list":["first",true,null,3],"object":{"z":true,"a":"é"},"nil":null,"bool":false});
    let (text, missing) = fill(
        "<text>\n<list>\n<object>\n<nil> <bool> <absent> <absent>",
        values.as_object().unwrap(),
    );
    assert_eq!(text,"hello\nfirst\nTrue\nNone\n3\n{\"a\": \"\\u00e9\", \"z\": true}\nNone False <absent> <absent>");
    assert_eq!(missing, ["absent"]);
}
#[test]
fn portable_fill_and_validation_preserve_draft_boundary() {
    let f = Fixture::new();
    let result = f.call(false, &[]);
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "PASS");
    assert_eq!(
        report["claim_boundary"],
        "generated draft only; not review or approval"
    );
    let text = fs::read_to_string(&f.output).unwrap();
    assert!(text.contains("generation_status: generated_draft"));
    assert!(!text.contains(f.root.to_str().unwrap()));
    assert!(text.ends_with("# Example\n## Metadata\nExample\n## Status\ndraft\n"));
    let result = f.call(true, &[]);
    assert!(result.status.success());
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(
        report["claim_boundary"],
        "structural validation only; not review or approval"
    );
}
#[test]
fn unresolved_values_never_overwrite_without_explicit_allow_missing() {
    let f = Fixture::new();
    write(&f.values, "{}");
    write(&f.output, "previous output");
    let result = f.call(false, &[]);
    assert_eq!(result.status.code(), Some(1));
    assert_eq!(fs::read_to_string(&f.output).unwrap(), "previous output");
    let result = f.call(false, &["--allow-missing"]);
    assert!(result.status.success());
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["status"], "PARTIAL");
    assert_eq!(report["missing_values"], json!(["name", "status"]));
    assert_eq!(f.call(true, &[]).status.code(), Some(1));
    assert!(!failures(
        "## Metadata\n## Status\n{{missing}}",
        &f.document["templates"]["test"]
    )
    .is_empty());
    assert!(!failures("Metadata\nStatus\n", &f.document["templates"]["test"]).is_empty());
}
#[test]
fn invalid_registry_template_and_input_are_refused() {
    for (pointer, value) in [
        ("/schema", json!("wrong")),
        ("/templates/test/status", json!("retired")),
        ("/templates/test/path", json!("/absolute/template.md")),
        (
            "/templates/test/path",
            json!("docs/templates/planning/1.0.0/missing.md"),
        ),
        ("/templates/test/path", json!("outside.md")),
        ("/templates/test/version", json!("2.0.0")),
    ] {
        let mut f = Fixture::new();
        write(&f.root.join("outside.md"), "outside");
        assert!(f.call(false, &[]).status.success());
        *f.document.pointer_mut(pointer).unwrap() = value;
        f.save();
        let result = f.call(true, &[]);
        assert_eq!(
            result.status.code(),
            Some(if pointer == "/schema" { 1 } else { 2 }),
            "{pointer}"
        );
    }
    let mut f = Fixture::new();
    assert!(f.call(false, &[]).status.success());
    f.document["templates"]["test"]["required_sections"] = json!("wrong type");
    f.save();
    assert_eq!(f.call(true, &[]).status.code(), Some(1));
    fs::remove_file(&f.output).unwrap();
    assert_eq!(f.call(true, &[]).status.code(), Some(2));
}
#[test]
fn all_active_templates_can_be_filled_and_structurally_checked() {
    let root = adl_public_contracts::root();
    let registry: Value = serde_json::from_slice(
        &fs::read(root.join("docs/templates/planning/current.json")).unwrap(),
    )
    .unwrap();
    let re = regex::Regex::new(r"<([A-Za-z][A-Za-z0-9_]*)>").unwrap();
    for (key, template) in registry["templates"].as_object().unwrap() {
        if template["status"] != "active" {
            continue;
        }
        let text = fs::read_to_string(root.join(template["path"].as_str().unwrap())).unwrap();
        let values = re
            .captures_iter(&text)
            .map(|c| (c[1].to_owned(), json!("filled")))
            .collect();
        let (filled, missing) = fill(&text, &values);
        assert!(missing.is_empty(), "{key}");
        assert_eq!(failures(&filled, template), Vec::<String>::new(), "{key}");
    }
}
#[cfg(unix)]
#[test]
fn registered_symlink_cannot_escape_active_template_root() {
    let f = Fixture::new();
    let target = f.root.join("docs/templates/planning/1.0.0/test.md");
    fs::remove_file(&target).unwrap();
    write(&f.root.join("outside.md"), "outside");
    std::os::unix::fs::symlink(f.root.join("outside.md"), target).unwrap();
    assert_eq!(f.call(false, &[]).status.code(), Some(1));
    assert!(!f.output.exists());
}
