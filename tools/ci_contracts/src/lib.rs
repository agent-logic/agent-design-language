pub mod actions;
pub mod ci;
pub mod coverage;
pub mod planning;
use regex::Regex;
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};

pub const RUST: &str = "1.92.0";
fn read(p: &Path) -> Result<String, String> {
    fs::read_to_string(p).map_err(|e| format!("{}: {e}", p.display()))
}
fn yaml(text: &str) -> Result<serde_yaml::Value, String> {
    let mut v: serde_yaml::Value = serde_yaml::from_str(text).map_err(|e| e.to_string())?;
    v.apply_merge().map_err(|e| e.to_string())?;
    Ok(v)
}
fn events(text: &str) -> Vec<String> {
    let mut found = false;
    let mut out = vec![];
    let re = Regex::new(r"^  ([a-zA-Z0-9_-]+):").unwrap();
    for line in text.lines() {
        if !found {
            found = line == "on:";
            continue;
        }
        if line.trim().is_empty() {
            continue;
        }
        if !line.starts_with("  ") {
            break;
        }
        if let Some(c) = re.captures(line) {
            out.push(c[1].to_owned());
        }
    }
    out
}
fn jobs(text: &str) -> BTreeMap<String, String> {
    let re = Regex::new(r"^  ([A-Za-z0-9_-]+):$").unwrap();
    let mut out = BTreeMap::new();
    let mut current = None;
    for line in text.lines() {
        if let Some(c) = re.captures(line) {
            current = Some(c[1].to_owned());
            out.insert(c[1].to_owned(), String::new());
        } else if let Some(name) = &current {
            out.get_mut(name).unwrap().push_str(&format!("{line}\n"));
        }
    }
    out
}
pub fn workflow_policy(root: &Path) -> Result<Value, String> {
    let mut errors = Vec::new();
    let mut inventory = Vec::new();
    let channel = Regex::new(r#"(?m)^channel\s*=\s*"1\.92\.0"\s*$"#).unwrap();
    match read(&root.join("rust-toolchain.toml")) {
        Ok(t) if channel.is_match(&t) => {}
        Ok(_) => errors.push(format!("rust-toolchain.toml: channel must pin Rust {RUST}")),
        Err(_) => {
            errors.push("rust-toolchain.toml: missing required Rust toolchain declaration".into())
        }
    }
    let dir = root.join(".github/workflows");
    let mut paths: Vec<_> = fs::read_dir(&dir)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()))
        .collect::<Result<_, _>>()
        .map_err(|e| e.to_string())?;
    paths.retain(|p| p.extension().is_some_and(|x| x == "yaml" || x == "yml"));
    paths.sort();
    for path in paths {
        let text = read(&path)?;
        let ev = events(&text);
        let relative = path
            .strip_prefix(root)
            .map_err(|e| e.to_string())?
            .to_string_lossy()
            .replace('\\', "/");
        let primary = relative == ".github/workflows/ci.yaml";
        let allowed = if primary {
            vec!["pull_request", "workflow_dispatch"]
        } else {
            vec!["workflow_dispatch", "workflow_call"]
        };
        let bad: Vec<_> = ev
            .iter()
            .filter(|e| !allowed.contains(&e.as_str()))
            .cloned()
            .collect();
        if !bad.is_empty() {
            errors.push(format!(
                "{relative}: event {} is not allowed",
                bad.join(",")
            ));
        }
        if !primary && !ev.iter().any(|e| e == "workflow_dispatch") {
            errors.push(format!(
                "{relative}: standalone workflow must remain explicitly dispatchable"
            ));
        }
        inventory.push(json!({"path":relative,"events":ev,"automatic_pr":ev.iter().any(|e|e=="pull_request"),"dispatch_policy":if primary{"automatic_classifier"}else{"explicit_only"}}));
    }
    let ci = read(&dir.join("ci.yaml"))?;
    match yaml(&ci) {
        Ok(v) => {
            let coverage = &v["jobs"]["public_adl_coverage"];
            match (coverage.as_mapping(), coverage["steps"].as_sequence()) {
                (Some(m), Some(steps)) => {
                    if m.contains_key(serde_yaml::Value::String("timeout-minutes".into()))
                        || steps.iter().any(|s| {
                            s.as_mapping().is_some_and(|m| {
                                m.contains_key(serde_yaml::Value::String("timeout-minutes".into()))
                            })
                        })
                    {
                        errors.push("ci.yaml public_adl_coverage: custom job/step deadline can cancel successful coverage production".into());
                    }
                }
                _ => errors.push(
                    "ci.yaml: workspace coverage deadline policy cannot inspect workflow".into(),
                ),
            }
            if let Some(js) = v["jobs"].as_mapping() {
                for job in js.values() {
                    if let Some(steps) = job["steps"].as_sequence() {
                        for step in steps {
                            if step["uses"]
                                .as_str()
                                .is_some_and(|s| s.starts_with("dtolnay/rust-toolchain@"))
                                && step["with"]["toolchain"].as_str() != Some(RUST)
                            {
                                errors.push(format!(
                                    ".github/workflows/ci.yaml {}: Rust toolchain must pin {RUST}",
                                    step["name"].as_str().unwrap_or("unnamed step")
                                ));
                            }
                        }
                    }
                }
            }
        }
        Err(_) => errors.push(
            "ci.yaml: workspace coverage deadline policy cannot inspect workflow (invalid YAML)"
                .into(),
        ),
    }
    let ev = events(&ci);
    if !ev.iter().any(|e| e == "pull_request") {
        errors.push(".github/workflows/ci.yaml: pull_request entrypoint is missing".into());
    }
    if !ev.iter().any(|e| e == "workflow_dispatch") {
        errors.push(".github/workflows/ci.yaml: explicit full validation is missing".into());
    }
    let concurrency="group: ${{ github.repository }}:${{ github.workflow }}:${{ github.event.pull_request.base.ref || github.ref_name }}:${{ github.event.pull_request.head.repo.id || github.repository_id }}:${{ github.event.pull_request.head.ref || github.ref }}";
    if !ci.contains(concurrency) {
        errors.push(".github/workflows/ci.yaml: concurrency key must unambiguously identify workflow, target, source repository, and source branch".into());
    }
    if !ci.contains("cancel-in-progress: true") {
        errors
            .push(".github/workflows/ci.yaml: superseded revisions must cancel in progress".into());
    }
    for (name, body) in jobs(&ci) {
        let Some(line) = body.lines().find(|l| l.starts_with("    runs-on:")) else {
            continue;
        };
        let light = [
            "adl_path_policy",
            "adl_tooling_contracts",
            "adl_coverage_hosted",
            "adl-ci",
            "adl-coverage",
        ]
        .contains(&name.as_str());
        let required = if name == "public_adl_validation" {
            "runs-on: macos-latest"
        } else {
            "runs-on: ubuntu-latest"
        };
        if line.trim() != required {
            errors.push(format!(
                "ci.yaml {name}: {} must use {required}",
                if light {
                    "light classifier/aggregator"
                } else {
                    "selected required job"
                }
            ));
        }
        if !light {
            let header = body.split("runs-on:").next().unwrap();
            if !header.contains("adl_path_policy") {
                errors.push(format!(
                    "ci.yaml {name}: required standard-runner job must depend on adl_path_policy"
                ));
            }
            if !header.lines().any(|l| l.starts_with("    if:")) {
                errors.push(format!(
                    "ci.yaml {name}: required standard-runner job must have a job-level selector"
                ));
            }
        }
    }
    let outputs = ci::select("workflow_dispatch", &[]);
    for (key, value) in [
        ("automatic_pr_entrypoint", "ci"),
        ("optional_workflows_status", "deferred"),
        ("optional_workflows_reason", "explicit_dispatch_required"),
        ("soak_workflows_status", "deferred"),
        ("duplicate_head_status", "canceled"),
        (
            "duplicate_head_reason",
            "source_branch_concurrency_cancel_in_progress",
        ),
    ] {
        if outputs.get(key).map(String::as_str) != Some(value) {
            errors.push(format!("public CI selector: missing {key}={value}"));
        }
    }
    Ok(
        json!({"schema":"adl.ci.workflow-policy.v1","status":if errors.is_empty(){"pass"}else{"fail"},"automatic_pr_entrypoint":".github/workflows/ci.yaml","required_runner":"ubuntu-latest","optional_policy":"explicit_dispatch_only","scheduled_heavy_validation":"disabled","workflow_count":inventory.len(),"workflows":inventory,"errors":errors}),
    )
}
pub fn aggregation_contract(root: &Path) -> Result<(), String> {
    let v = yaml(&read(&root.join(".github/workflows/ci.yaml"))?)?;
    let jobs = &v["jobs"];
    let agg = &jobs["adl-ci"];
    if !agg["needs"].as_sequence().is_some_and(|s| {
        s.iter()
            .any(|x| x.as_str() == Some("public_adl_validation"))
    }) {
        return Err("public producer missing".into());
    }
    let step = agg["steps"]
        .as_sequence()
        .and_then(|s| {
            s.iter()
                .find(|x| x["name"].as_str() == Some("Require every selected public lane"))
        })
        .ok_or("aggregate step missing")?;
    let run = step["run"].as_str().ok_or("aggregate command missing")?;
    if run != "cargo run --locked --quiet --manifest-path tools/ci_contracts/Cargo.toml -- ci --aggregate ci" {
        return Err("wrong aggregate command".into());
    }
    for (k, val) in [
        (
            "PUBLIC_REQUIRED",
            "${{ needs.adl_path_policy.outputs.public_adl_validation_required }}",
        ),
        ("PUBLIC_RESULT", "${{ needs.public_adl_validation.result }}"),
    ] {
        if step["env"][k].as_str() != Some(val) {
            return Err(format!("wrong binding {k}"));
        }
    }
    if !read(&root.join("tools/public_adl/run_ci.sh"))?
        .contains("bash tools/public_adl/run_package_checks.sh")
    {
        return Err("shared public package checks disconnected".into());
    }
    if !read(&root.join("tools/public_adl/run_package_checks.sh"))?.contains("  adl-uts\n") {
        return Err("UTS package dropped".into());
    }
    for s in jobs["adl_path_policy"]["steps"]
        .as_sequence()
        .ok_or("classifier steps missing")?
    {
        if s["run"].as_str().unwrap_or("").contains("$PUBLIC_RESULT") {
            return Err("premature result gate".into());
        }
    }
    for (required, result, expected) in [
        ("true", "success", true),
        ("true", "failure", false),
        ("true", "skipped", false),
        ("true", "", false),
        ("false", "skipped", true),
        ("false", "success", false),
    ] {
        let observed = ci::aggregate(
            "ci",
            &[
                ("PATH_POLICY_RESULT", "success"),
                ("CONTRACT_RESULT", "success"),
                ("TRANSPILE_REQUIRED", "false"),
                ("TRANSPILE_RESULT", "skipped"),
                ("PUBLIC_REQUIRED", required),
                ("PUBLIC_RESULT", result),
            ]
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v.to_owned()))
            .collect(),
        );
        if observed.is_ok() != expected {
            return Err(format!("gate mismatch {required}/{result}"));
        }
    }
    Ok(())
}
