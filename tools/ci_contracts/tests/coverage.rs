//! PVF local_contract: synthetic LLVM evidence and reporting; no compiler/provider execution.
use adl_ci_contracts::coverage::*;
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
fn fixture() -> (Value, BTreeMap<String, Value>) {
    (json!({"packages":[{"name":"a","sources":["a/src/lib.rs","a/src/main.rs"],"declaration_only":["a/src/lib.rs"]}]}),[("a".into(),json!({"type":"llvm.coverage.json.export","data":[{"files":[{"filename":"/synthetic/repo/a/src/main.rs","summary":{"lines":{"count":100,"covered":90}}}]}]}))].into_iter().collect())
}
fn check(
    i: &Value,
    r: &BTreeMap<String, Value>,
) -> Result<(BTreeMap<String, Row>, Vec<String>), String> {
    check_rows(r, i, Path::new("/synthetic/repo"))
}
#[test]
fn declaration_and_integer_thresholds() {
    let (i, r) = fixture();
    let (mut rows, missing) = check(&i, &r).unwrap();
    assert_eq!(missing, vec!["a/src/lib.rs"]);
    assert!(enforce(&rows, "full", &BTreeSet::new()).unwrap().is_empty());
    rows.get_mut("a/src/main.rs").unwrap().covered = 89;
    assert_eq!(
        enforce(&rows, "full", &BTreeSet::new()).unwrap(),
        vec!["public aggregate below 90% line floor"]
    );
    assert!(enforce(
        &rows,
        "pull_request",
        &["a/src/main.rs".into()].into_iter().collect()
    )
    .unwrap()
    .is_empty());
}
#[test]
fn changed_and_full_floors() {
    let (i, r) = fixture();
    let (mut rows, _) = check(&i, &r).unwrap();
    rows.get_mut("a/src/main.rs").unwrap().covered = 79;
    assert!(!enforce(
        &rows,
        "pull_request",
        &["a/src/main.rs".into()].into_iter().collect()
    )
    .unwrap()
    .is_empty());
    assert!(enforce(&rows, "pull_request", &BTreeSet::new())
        .unwrap()
        .is_empty());
    assert_eq!(enforce(&rows, "full", &BTreeSet::new()).unwrap().len(), 2);
}
#[test]
fn missing_measurements() {
    let (i, mut r) = fixture();
    r.get_mut("a").unwrap()["data"][0]["files"] = json!([]);
    assert!(check(&i, &r).unwrap_err().contains("missing production"));
}
#[test]
fn test_like_sources_need_explicit_exclusions() {
    for path in [
        "a/src/tests.rs",
        "a/src/tests/production.rs",
        "a/tests/integration.rs",
    ] {
        let (mut i, mut r) = fixture();
        let mut extra = r["a"]["data"][0]["files"][0].clone();
        extra["filename"] = json!(format!("/synthetic/repo/{path}"));
        extra["summary"]["lines"]["covered"] = json!(0);
        r.get_mut("a").unwrap()["data"][0]["files"]
            .as_array_mut()
            .unwrap()
            .push(extra);
        assert!(check(&i, &r).unwrap_err().contains("unexpected repository"));
        i["packages"][0]["test_sources"] = json!([path]);
        assert!(!check(&i, &r).unwrap().0.contains_key(path));
    }
}
#[test]
fn test_exclusion_cannot_hide_production() {
    let (mut i, r) = fixture();
    i["packages"][0]["test_sources"] = json!(["a/src/main.rs"]);
    assert!(check(&i, &r).unwrap_err().contains("overlaps production"));
}
#[test]
fn missing_packages_and_malformed_reports() {
    let (i, _) = fixture();
    for r in [
        BTreeMap::new(),
        [("a".into(), json!({}))].into_iter().collect(),
        [(
            "a".into(),
            json!({"type":"llvm.coverage.json.export","data":[]}),
        )]
        .into_iter()
        .collect(),
    ] {
        assert!(check(&i, &r).is_err());
    }
}
#[test]
fn duplicate_owners_and_measurements() {
    let (mut i, mut r) = fixture();
    let p = i["packages"][0].clone();
    i["packages"].as_array_mut().unwrap().push(p);
    assert!(check(&i, &r).unwrap_err().contains("duplicate source"));
    i["packages"].as_array_mut().unwrap().pop();
    let f = r["a"]["data"][0]["files"][0].clone();
    r.get_mut("a").unwrap()["data"][0]["files"]
        .as_array_mut()
        .unwrap()
        .push(f);
    assert!(check(&i, &r).unwrap_err().contains("duplicate measured"));
}
#[test]
fn private_sources_refused_and_dependencies_count_once() {
    for path in [
        "private/src/lib.rs",
        "adl/src/codefriend/review/runner.rs",
        "adl-runtime/src/runtime_api_auth.rs",
    ] {
        let (i, mut r) = fixture();
        let mut extra = r["a"]["data"][0]["files"][0].clone();
        extra["filename"] = json!(format!("/synthetic/repo/{path}"));
        r.get_mut("a").unwrap()["data"][0]["files"]
            .as_array_mut()
            .unwrap()
            .push(extra);
        assert!(check(&i, &r).unwrap_err().contains("unexpected repository"));
    }
    let (mut i, mut r) = fixture();
    i["packages"]
        .as_array_mut()
        .unwrap()
        .push(json!({"name":"b","sources":["b/src/lib.rs"]}));
    r.insert("b".into(), r["a"].clone());
    let mut extra = r["a"]["data"][0]["files"][0].clone();
    extra["filename"] = json!("/synthetic/repo/b/src/lib.rs");
    for p in ["a", "b"] {
        r.get_mut(p).unwrap()["data"][0]["files"]
            .as_array_mut()
            .unwrap()
            .push(extra.clone());
    }
    assert_eq!(
        check(&i, &r)
            .unwrap()
            .0
            .values()
            .map(|r| r.count)
            .sum::<u64>(),
        200
    );
}
#[test]
fn zero_source_requires_declaration() {
    let (mut i, mut r) = fixture();
    let mut extra = r["a"]["data"][0]["files"][0].clone();
    extra["filename"] = json!("/synthetic/repo/a/src/new.rs");
    extra["summary"]["lines"] = json!({"count":0,"covered":0});
    r.get_mut("a").unwrap()["data"][0]["files"]
        .as_array_mut()
        .unwrap()
        .push(extra);
    i["packages"][0]["sources"]
        .as_array_mut()
        .unwrap()
        .push(json!("a/src/new.rs"));
    assert!(check(&i, &r).unwrap_err().contains("zero production"));
    i["packages"][0]["declaration_only"]
        .as_array_mut()
        .unwrap()
        .push(json!("a/src/new.rs"));
    assert_eq!(check(&i, &r).unwrap().0["a/src/main.rs"].count, 100);
}
#[test]
fn invalid_counts_and_empty_denominator() {
    for lines in [
        json!({"count":1,"covered":2}),
        json!({"count":1,"covered":-1}),
        json!({"count":true,"covered":1}),
        json!({"count":100,"covered":90.0}),
    ] {
        let (i, mut r) = fixture();
        r.get_mut("a").unwrap()["data"][0]["files"][0]["summary"]["lines"] = lines;
        assert!(check(&i, &r).unwrap_err().contains("invalid line counts"));
    }
    assert!(enforce(&BTreeMap::new(), "pull_request", &BTreeSet::new())
        .unwrap_err()
        .contains("empty coverage"));
}
#[test]
fn seven_nightly_report_cases() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path();
    let path = root.join("result.json");
    let summary = root.join("summary.md");
    let data = json!({"schema":"adl.public_coverage.v1","mode":"full","source_head":"fixture","line_count":100,"covered_lines":95,"status":"passed","failures":[],"thresholds":{"full_file":80,"full_aggregate":90},"files":{"adl/src/lib.rs":{"covered":95,"count":100}}});
    for case in [
        "passed",
        "failed",
        "missing",
        "malformed_json",
        "empty_denominator",
        "missing_field",
        "malformed_files",
    ] {
        let mut d = data.clone();
        match case {
            "failed" => {
                d["status"] = json!("failed");
                d["failures"] = json!(["public aggregate below 90% line floor"]);
            }
            "empty_denominator" => d["line_count"] = json!(0),
            "missing_field" => d = json!({"schema":"adl.public_coverage.v1"}),
            "malformed_files" => d["files"] = json!([]),
            _ => {}
        }
        if case == "missing" {
            fs::remove_file(&path).unwrap();
        } else {
            fs::write(
                &path,
                if case == "malformed_json" {
                    "{".into()
                } else {
                    d.to_string()
                },
            )
            .unwrap();
        }
        assert_eq!(
            report(root, root, &summary, "success").unwrap(),
            case == "passed"
        );
        let text = fs::read_to_string(root.join("nightly-public-coverage-report.md")).unwrap();
        let expected = match case {
            "passed" => "95.00%",
            "failed" => "below 90% line floor",
            "missing" => "Coverage result was not produced",
            _ => "Coverage result is malformed or unreadable",
        };
        assert!(text.contains(expected), "{case}: {text}");
        assert!(fs::read_to_string(&summary).unwrap().contains(&text));
    }
}
