// PVF: deterministic local schema contract; no lane execution, network, or
// release qualification. Tiny in-memory mutations plus isolated CLI fixture.
use adl_public_contracts::pvf_manifest as p;
use serde_json::{json, Value};
use std::{fs, process::Command};
fn valid() -> Value {
    json!({"manifest_version":"v0.91.4","lane_classes":["docs"],"lanes":{"docs_1":{"lane_class":"docs","owner_surface":"docs","command":"never execute this","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"optional","default_trigger":"manual","changed_path_hints":[],"evidence_outputs":[],"timeout_minutes":1,"requires_credentials":false,"requires_worktree":false,"notes":"retained contract"}}})
}
#[test]
fn strict_schema_and_lane_classes() {
    let original = valid();
    p::validate(&original).unwrap();
    for key in ["manifest_version", "lane_classes", "lanes"] {
        let mut d = original.clone();
        d.as_object_mut().unwrap().remove(key);
        assert!(p::validate(&d).is_err());
    }
    for key in p::REQUIRED {
        let mut d = original.clone();
        d["lanes"]["docs_1"].as_object_mut().unwrap().remove(*key);
        assert!(p::validate(&d).is_err(), "{key}");
    }
    for key in p::REQUIRED {
        let mut d = original.clone();
        d["lanes"]["docs_1"][key] = Value::Null;
        assert!(p::validate(&d).is_err(), "{key}");
    }
    for classes in [
        json!([]),
        json!(["docs", "docs"]),
        json!(["docs", "fast_unit"]),
        json!(["unknown"]),
        json!(["fast_unit"]),
    ] {
        let mut d = original.clone();
        d["lane_classes"] = classes;
        assert!(p::validate(&d).is_err());
    }
    for id in ["", "Docs", "a-b", "é"] {
        let mut d = original.clone();
        let v = d["lanes"]
            .as_object_mut()
            .unwrap()
            .remove("docs_1")
            .unwrap();
        d["lanes"][id] = v;
        assert!(p::validate(&d).is_err());
    }
    let mut d = original.clone();
    d["extra"] = json!(1);
    assert!(p::validate(&d).is_err());
    let mut d = original.clone();
    d["lanes"]["docs_1"]["extra"] = json!(1);
    assert!(p::validate(&d).is_err());
    for key in ["requires_credentials", "requires_worktree"] {
        let mut d = original.clone();
        d["lanes"]["docs_1"][key] = json!(1);
        assert!(p::validate(&d).is_err());
    }
    for v in [json!(0), json!(-1), json!(1.5), json!(true)] {
        let mut d = original.clone();
        d["lanes"]["docs_1"]["timeout_minutes"] = v;
        assert!(p::validate(&d).is_err());
    }
}
#[test]
fn cli_multiple_files_and_failures_do_not_execute_commands() {
    let temp = tempfile::tempdir().unwrap();
    let a = temp.path().join("a.json");
    let b = temp.path().join("b.json");
    fs::write(&a, valid().to_string()).unwrap();
    fs::write(&b, valid().to_string()).unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg("pvf-manifest")
        .args([&a, &b])
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8_lossy(&out.stdout)
            .matches("PASS: pvf manifest valid")
            .count(),
        2
    );
    fs::write(&b, "{").unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg("pvf-manifest")
        .args([&a, &b])
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(1));
    let out = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg("pvf-manifest")
        .output()
        .unwrap();
    assert_eq!(out.status.code(), Some(2));
}
