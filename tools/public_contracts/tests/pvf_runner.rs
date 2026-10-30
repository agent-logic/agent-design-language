// PVF: deterministic local runner contract; bounded one-second synthetic lanes.
// No providers, credentials, private owners or release qualification.
use adl_public_contracts::{pvf_runner as p, root};
use serde_json::{json, Value};
use std::{fs, process::Command};
const FIXTURES: &[&str] = &[
    // manifest
    r#"{"manifest_version":"v0.91.4","lane_classes":["fast_unit","docs","cli_workflow","integration_worktree","release_gate"],"lanes":{"pass_lane":{"lane_class":"fast_unit","owner_surface":"pass lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"changed_paths","changed_path_hints":["docs/*.md"],"evidence_outputs":["stdout:pass"]},"reuse_lane":{"lane_class":"docs","owner_surface":"reuse lane","command":"reuse_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"artifact_reuse","release_gate_class":"required_on_pr","default_trigger":"changed_paths","changed_path_hints":["docs/"],"evidence_outputs":["stdout:reuse"]},"fail_lane":{"lane_class":"cli_workflow","owner_surface":"fail lane","command":"fail_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"changed_paths","changed_path_hints":["docs/"],"evidence_outputs":["stdout:fail"]},"blocked_lane":{"lane_class":"integration_worktree","owner_surface":"blocked lane","command":"pass_lane.sh","resource_profile":"medium","determinism":"fixture_bound","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"always","changed_path_hints":[],"evidence_outputs":["stdout:blocked"],"requires_worktree":true},"skipped_lane":{"lane_class":"docs","owner_surface":"skipped lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"changed_paths","changed_path_hints":["adl/src/"],"evidence_outputs":["stdout:skipped"]},"deferred_lane":{"lane_class":"docs","owner_surface":"deferred lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"optional","default_trigger":"manual","changed_path_hints":[],"evidence_outputs":["stdout:deferred"]},"release_gate_lane":{"lane_class":"release_gate","owner_surface":"release-gate lane","command":"pass_lane.sh","resource_profile":"high","determinism":"fixture_bound","cache_strategy":"artifact_reuse","release_gate_class":"manual_release_gate","default_trigger":"release_only","changed_path_hints":["docs/"],"evidence_outputs":["stdout:release"]}}}"#,
    // cred_manifest
    r#"{"manifest_version":"v0.91.4","lane_classes":["provider_live"],"lanes":{"credential_lane":{"lane_class":"provider_live","owner_surface":"credential lane","command":"pass_lane.sh","resource_profile":"high","determinism":"live","cache_strategy":"artifact_reuse","release_gate_class":"required_on_pr","default_trigger":"always","changed_path_hints":[],"evidence_outputs":["stdout:credential"],"requires_credentials":true}}}"#,
    // release_gate_manifest
    r#"{"manifest_version":"v0.91.4","lane_classes":["docs","release_gate"],"lanes":{"manual_docs_lane":{"lane_class":"docs","owner_surface":"manual docs lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"optional","default_trigger":"manual","changed_path_hints":[],"evidence_outputs":["stdout:manual"]},"release_gate_lane":{"lane_class":"release_gate","owner_surface":"release gate lane","command":"pass_lane.sh","resource_profile":"high","determinism":"fixture_bound","cache_strategy":"artifact_reuse","release_gate_class":"manual_release_gate","default_trigger":"release_only","changed_path_hints":["docs/*.md"],"evidence_outputs":["stdout:release"]}}}"#,
    // nonmatching_release_gate_manifest
    r#"{"manifest_version":"v0.91.4","lane_classes":["docs","release_gate"],"lanes":{"pass_lane":{"lane_class":"docs","owner_surface":"pass lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"changed_paths","changed_path_hints":["docs/*.md"],"evidence_outputs":["stdout:pass"]},"release_gate_lane":{"lane_class":"release_gate","owner_surface":"release gate lane","command":"pass_lane.sh","resource_profile":"high","determinism":"fixture_bound","cache_strategy":"artifact_reuse","release_gate_class":"manual_release_gate","default_trigger":"release_only","changed_path_hints":["adl/src/*"],"evidence_outputs":["stdout:release"]}}}"#,
    // invalid_manifest
    r#"{"manifest_version":"v0.91.4","lane_classes":["docs"],"lanes":{"pass_lane":{"lane_class":"fast_unit","owner_surface":"pass lane","command":"pass_lane.sh","resource_profile":"low","determinism":"strict","cache_strategy":"none","release_gate_class":"required_on_pr","default_trigger":"always","changed_path_hints":[],"evidence_outputs":["stdout:pass"]}}}"#,
];
#[test]
fn native_helpers_preserve_order_glob_and_json_data() {
    let original = FIXTURES[0];
    let plan = p::plan(original).unwrap();
    assert!(plan.starts_with("pass_lane\t"));
    assert!(plan.find("reuse_lane\t").unwrap() < plan.find("fail_lane\t").unwrap());
    for (path, pattern, expected) in [
        ("a/b.md", "*.md", true),
        ("é", "?", true),
        ("b", "[a-c]", true),
        ("d", "[!a-c]", true),
        ("a", "[!a-c]", false),
        ("]", "[]]", true),
        ("[", "[[]", true),
        ("z", "[z-a]", false),
        ("x", "[!z-a]", true),
        ("[abc", "[abc", true),
        ("a\nb", "a*b", true),
        ("a", "A", false),
    ] {
        assert_eq!(p::glob(path, pattern), expected, "{path} / {pattern}");
    }
    assert_eq!(p::matches("", &json!([])).unwrap(), "unknown");
    assert_eq!(
        p::matches("docs/test.md\n", &json!(["docs/"])).unwrap(),
        "match"
    );
    assert_eq!(
        p::matches("src/a.rs", &json!(["docs/*"])).unwrap(),
        "no_match"
    );
    let temp = tempfile::tempdir().unwrap();
    for (suffix, value) in [
        ("lane_class", "docs"),
        ("status", "passed"),
        ("reason", "command_succeeded"),
        ("out", "quote \" tab\t newline\n é"),
    ] {
        fs::write(temp.path().join(format!("one.{suffix}")), value).unwrap();
    }
    let report = p::report("one\tfields\n", temp.path(), "quoted\"path", "pr", "passed").unwrap();
    assert_eq!(
        report["lanes"]["one"]["output"],
        "quote \" tab\t newline\n é"
    );
    assert_eq!(report["manifest_path"], "quoted\"path");
    assert!(p::report("../escape\tfields\n", temp.path(), "x", "pr", "passed").is_err());
    let mut manifest: Value = serde_json::from_str(original).unwrap();
    manifest["lanes"]["pass_lane"]["command"] = json!("line\nbreak");
    assert!(p::plan(&manifest.to_string()).is_err());
}
#[test]
fn actual_runner_keeps_concurrency_gates_reports_and_plan_only() {
    let temp = tempfile::tempdir().unwrap();
    let dir = temp.path();
    let bin = dir.join("bin");
    fs::create_dir(&bin).unwrap();
    let trap = dir.join("interpreter-invoked");
    for name in ["python", "python3", "ruby"] {
        adl_public_contracts::executable(
            &bin.join(name),
            &format!(
                "#!/bin/sh\nprintf invoked > '{}'\nexit 97\n",
                trap.display()
            ),
        )
        .unwrap();
    }
    for (name, tail) in [
        ("pass", "echo lane-pass"),
        ("reuse", "echo PVF_STATUS=reused"),
        ("fail", "echo lane-fail >&2; exit 1"),
    ] {
        adl_public_contracts::executable(
            &bin.join(format!("{name}_lane.sh")),
            &format!(
                "#!/bin/sh\n: > '{}'
sleep 1\n{tail}\n",
                dir.join(format!("{name}.start")).display()
            ),
        )
        .unwrap();
    }
    let changed = dir.join("changed.txt");
    fs::write(&changed, "docs/readme.md\n").unwrap();
    let runner = root().join("adl/tools/run_pvf_validation_lane.sh");
    let invoke = |case: usize, extra: &[&str]| {
        let mut fixture = FIXTURES[case].to_owned();
        for name in ["pass", "reuse", "fail"] {
            fixture = fixture.replace(
                &format!("{name}_lane.sh"),
                &bin.join(format!("{name}_lane.sh")).to_string_lossy(),
            );
        }
        let manifest = dir.join("manifest.json");
        let report = dir.join("report.json");
        fs::write(&manifest, fixture).unwrap();
        if report.exists() {
            fs::remove_file(&report).unwrap();
        }
        let out = Command::new("bash")
            .arg(&runner)
            .arg("--manifest")
            .arg(&manifest)
            .arg("--changed-files")
            .arg(&changed)
            .arg("--report-out")
            .arg(&report)
            .args(extra)
            .env(
                "PATH",
                format!("{}:{}", bin.display(), std::env::var("PATH").unwrap()),
            )
            .output()
            .unwrap();
        let report = if report.exists() {
            serde_json::from_slice(&fs::read(report).unwrap()).unwrap()
        } else {
            Value::Null
        };
        (out, report)
    };
    let (out, report) = invoke(0, &[]);
    assert_eq!(
        out.status.code(),
        Some(1),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert_eq!(report["schema_version"], "adl.pvf_run.v1");
    assert_eq!(report["mode"], "pr");
    assert_eq!(report["aggregate_status"], "failed");
    for (lane, status) in [
        ("pass_lane", "passed"),
        ("reuse_lane", "reused"),
        ("fail_lane", "failed"),
        ("blocked_lane", "blocked"),
        ("skipped_lane", "skipped"),
        ("deferred_lane", "deferred"),
        ("release_gate_lane", "release_gate_required"),
    ] {
        assert_eq!(report["lanes"][lane]["status"], status, "{lane}");
    }
    let times: Vec<_> = ["pass", "reuse", "fail"]
        .iter()
        .map(|n| {
            fs::metadata(dir.join(format!("{n}.start")))
                .unwrap()
                .modified()
                .unwrap()
        })
        .collect();
    for time in &times[1..] {
        let delta = time
            .duration_since(times[0])
            .unwrap_or_else(|_| times[0].duration_since(*time).unwrap());
        assert!(delta.as_secs_f64() < 0.75, "lanes must launch concurrently");
    }
    let summary = String::from_utf8_lossy(&out.stdout);
    for text in [
        "aggregate_status=failed",
        "status=release_gate_required",
        "status=deferred",
    ] {
        assert!(summary.contains(text));
    }
    for name in ["pass", "reuse", "fail"] {
        fs::remove_file(dir.join(format!("{name}.start"))).unwrap();
    }
    let (out, report) = invoke(0, &["--print-plan"]);
    assert!(out.status.success());
    assert!(report.is_null());
    assert!(String::from_utf8_lossy(&out.stdout).contains("PVF lane plan"));
    for name in ["pass", "reuse", "fail"] {
        assert!(!dir.join(format!("{name}.start")).exists());
    }
    let (out, r) = invoke(1, &[]);
    assert!(!out.status.success());
    assert_eq!(r["aggregate_status"], "blocked");
    assert_eq!(
        r["lanes"]["credential_lane"]["reason"],
        "credential_lane_requires_explicit_opt_in"
    );
    let (out, r) = invoke(1, &["--allow-credential-lanes"]);
    assert!(out.status.success());
    assert_eq!(r["aggregate_status"], "passed");
    let (out, r) = invoke(2, &[]);
    assert!(!out.status.success());
    assert_eq!(r["aggregate_status"], "release_gate_required");
    assert_eq!(r["lanes"]["manual_docs_lane"]["status"], "deferred");
    assert_eq!(
        r["lanes"]["release_gate_lane"]["status"],
        "release_gate_required"
    );
    let (out, r) = invoke(3, &[]);
    assert!(out.status.success());
    assert_eq!(r["aggregate_status"], "passed");
    assert_eq!(r["lanes"]["release_gate_lane"]["status"], "skipped");
    let (out, r) = invoke(4, &[]);
    assert!(!out.status.success());
    assert!(r.is_null());
    assert!(String::from_utf8_lossy(&out.stderr).contains("manifest contract validation failed"));
    assert!(!trap.exists(), "runner invoked an interpreter");
}
