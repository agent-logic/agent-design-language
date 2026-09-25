//! PVF: deterministic local planning-contract proof, no network/providers/cloud.
//! Release gate for #1047 planning consistency only; not Runtime/product proof.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

// PVF: planning-contract lane, deterministic local document/graph validation;
// bounded Python subprocess and repository reads, no network or product claims.
#[test]
fn approved_release_split_preserves_scope_and_rejects_invalid_plans() {
    let output = std::process::Command::new("python3")
        .env("PYTHONDONTWRITEBYTECODE", "1")
        .arg(root().join("validate_split.py"))
        .output()
        .expect("Python 3 is required for the release-split planning validator");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout).expect("validator JSON report");
    assert_eq!(report["status"], "pass");
    assert_eq!(report["source_tasks"], 83);
    assert_eq!(report["successor_tasks"], serde_json::json!([43, 53]));
    assert_eq!(report["negative_fixtures"], 22);
    assert_eq!(report["execution_opened"], false);
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .join("docs/milestones/v0.93")
}
fn read(name: &str) -> Value {
    serde_json::from_str(&std::fs::read_to_string(root().join(name)).unwrap()).unwrap()
}
fn check(plan: &Value) -> Result<(), String> {
    let rows = plan["work_packages"].as_array().ok_or("missing rows")?;
    let mut by_id = BTreeMap::new();
    for row in rows {
        let id = row["id"].as_str().ok_or("missing id")?;
        if by_id.insert(id, row).is_some() {
            return Err(format!("duplicate {id}"));
        }
        for field in ["result", "repository", "source", "resources", "pvf"] {
            if row[field].as_str().unwrap_or("").trim().is_empty() {
                return Err(format!("{id}: missing {field}"));
            }
        }
        if row["negative_cases"]
            .as_array()
            .is_none_or(|v| v.is_empty())
        {
            return Err(format!("{id}: missing negatives"));
        }
        let source = row["source"].as_str().unwrap();
        if source.contains("..") || !root().join(source).is_file() {
            return Err(format!("{id}: missing source"));
        }
        if plan["repositories"][row["repository"].as_str().unwrap()]
            .as_str()
            .is_none()
        {
            return Err(format!("{id}: unknown owner"));
        }
    }
    fn visit<'a>(
        id: &'a str,
        rows: &BTreeMap<&'a str, &'a Value>,
        stack: &mut BTreeSet<&'a str>,
        all: &mut BTreeSet<&'a str>,
    ) -> Result<(), String> {
        if !stack.insert(id) {
            return Err(format!("cycle {id}"));
        }
        let row = rows
            .get(id)
            .ok_or_else(|| format!("missing dependency {id}"))?;
        for dep in row["depends_on"].as_array().ok_or("missing dependencies")? {
            let dep = dep.as_str().ok_or("invalid dependency")?;
            all.insert(dep);
            visit(dep, rows, stack, all)?;
        }
        stack.remove(id);
        Ok(())
    }
    for (id, row) in &by_id {
        let mut ancestors = BTreeSet::new();
        visit(id, &by_id, &mut BTreeSet::new(), &mut ancestors)?;
        if matches!(
            row["phase"].as_str(),
            Some("runtime_v4" | "codefriend_launch" | "governance" | "security")
        ) && !ancestors.contains("RD-11")
        {
            return Err(format!("{id}: bypasses split"));
        }
    }
    if by_id["RV-01"]["pvf"] != "planning_contract" {
        return Err("RV-01 requires a design-review proof gate".into());
    }
    for i in 2..=11 {
        if by_id[format!("RV-{i:02}").as_str()]["pvf"] != "installed_integration" {
            return Err(format!("RV-{i:02} requires installed implementation proof"));
        }
    }
    let mut release = BTreeSet::new();
    visit("TAIL-10", &by_id, &mut BTreeSet::new(), &mut release)?;
    for prefix_count in [("RV", 11), ("CF", 7), ("CT", 10), ("CM", 4)] {
        for i in 1..=prefix_count.1 {
            let id = format!("{}-{i:02}", prefix_count.0);
            if !release.contains(id.as_str()) {
                return Err(format!("release omits {id}"));
            }
        }
    }
    for i in 2..=10 {
        let id = format!("TAIL-{i:02}");
        let prev = format!("TAIL-{:02}", i - 1);
        if by_id[id.as_str()]["depends_on"] != serde_json::json!([prev]) {
            return Err(format!("tail sequence {id}"));
        }
    }
    let public: Vec<_> = plan["repositories"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(_, v)| v.as_str() == Some("public"))
        .map(|(k, _)| k.as_str())
        .collect();
    if public != ["agent-design-language"] {
        return Err("public boundary".into());
    }
    for repo in ["codefriend", "codefriend.ai"] {
        if plan["repositories"][repo] != "private" {
            return Err("CodeFriend separation".into());
        }
    }
    if !matches!(
        plan["status"].as_str(),
        Some("first_pass_not_open" | "reconciled_draft_pending_final_predecessor")
    ) {
        return Err("opening claim".into());
    }
    let schedule = &plan["sprint_plan"];
    let sprints = schedule["sprints"].as_array().ok_or("missing sprints")?;
    if schedule["sprint_count"].as_u64() != Some(sprints.len() as u64) {
        return Err("sprint count mismatch".into());
    }
    let mut assigned = BTreeMap::new();
    for (index, sprint) in sprints.iter().enumerate() {
        let number = (index + 1) as u64;
        if sprint["number"].as_u64() != Some(number) {
            return Err("nonconsecutive sprint numbering".into());
        }
        let members = sprint["work_packages"]
            .as_array()
            .ok_or("missing members")?;
        if members.is_empty() {
            return Err("empty sprint".into());
        }
        for member in members {
            let id = member.as_str().ok_or("invalid sprint member")?;
            if !by_id.contains_key(id) || assigned.insert(id, number).is_some() {
                return Err(format!("unknown or duplicate sprint member {id}"));
            }
        }
    }
    let split_end = *assigned.get("RD-11").ok_or("unscheduled split gate")?;
    for (id, row) in &by_id {
        let number = *assigned
            .get(id)
            .ok_or_else(|| format!("unscheduled {id}"))?;
        if row["sprint"].as_u64() != Some(number) {
            return Err(format!("sprint membership mismatch {id}"));
        }
        let split_task = *id == "WP-01" || id.starts_with("RD-");
        if (split_task && number > split_end) || (!split_task && number <= split_end) {
            return Err(format!("split must finish before features: {id}"));
        }
        for dep in row["depends_on"].as_array().unwrap() {
            if assigned
                .get(dep.as_str().unwrap())
                .is_none_or(|n| *n > number)
            {
                return Err(format!("sprint precedes dependency: {id}"));
            }
        }
    }
    Ok(())
}
#[test]
fn package_preserves_release_dependencies_and_source_owners() {
    check(&read("EXECUTION_PLAN_v0.93.json")).unwrap();
}
#[test]
fn issue_wave_and_specifications_match_canonical_results() {
    let p = read("EXECUTION_PLAN_v0.93.json");
    assert_eq!(
        p["work_packages"],
        read("WP_EXECUTION_SPECIFICATIONS_v0.93.yaml")["work_packages"]
    );
    let wave = read("WP_ISSUE_WAVE_v0.93.yaml");
    assert_eq!(p["sprint_plan"], wave["sprint_plan"]);
    assert_eq!(
        p["sprint_plan"],
        read("WP_EXECUTION_SPECIFICATIONS_v0.93.yaml")["sprint_plan"]
    );
    let rows = p["work_packages"].as_array().unwrap();
    let w = wave["work_packages"].as_array().unwrap();
    assert_eq!(rows.len(), w.len());
    for (r, w) in rows.iter().zip(w) {
        for (a, b) in [
            ("id", "wp"),
            ("title", "title"),
            ("repository", "repository"),
            ("depends_on", "depends_on"),
            ("result", "outcome"),
            ("sprint", "sprint"),
        ] {
            assert_eq!(r[a], w[b], "{} {a}", r["id"]);
        }
    }
}

#[test]
fn rejects_missing_duplicate_and_premature_sprint_assignments() {
    let mut p = read("EXECUTION_PLAN_v0.93.json");
    p["sprint_plan"]["sprints"][0]["work_packages"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(check(&p).is_err());
    let mut p = read("EXECUTION_PLAN_v0.93.json");
    p["sprint_plan"]["sprints"][0]["work_packages"]
        .as_array_mut()
        .unwrap()
        .push(serde_json::json!("WP-01"));
    assert!(check(&p).is_err());
    for (id, target) in [("CF-01", 1_u64), ("CT-03", 2)] {
        let mut p = read("EXECUTION_PLAN_v0.93.json");
        for sprint in p["sprint_plan"]["sprints"].as_array_mut().unwrap() {
            sprint["work_packages"]
                .as_array_mut()
                .unwrap()
                .retain(|v| v != id);
            if sprint["number"] == target {
                sprint["work_packages"]
                    .as_array_mut()
                    .unwrap()
                    .push(serde_json::json!(id));
            }
        }
        p["work_packages"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["id"] == id)
            .unwrap()["sprint"] = serde_json::json!(target);
        assert!(check(&p).is_err(), "allowed premature {id}");
    }
}
#[test]
fn rejects_missing_dependency_cycle_and_split_bypass() {
    for deps in [
        serde_json::json!(["absent"]),
        serde_json::json!(["RV-01"]),
        serde_json::json!([]),
    ] {
        let mut p = read("EXECUTION_PLAN_v0.93.json");
        p["work_packages"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["id"] == "RV-01")
            .unwrap()["depends_on"] = deps;
        assert!(check(&p).is_err());
    }
}
#[test]
fn rejects_omitted_launch_and_private_boundary_changes() {
    let mut p = read("EXECUTION_PLAN_v0.93.json");
    p["work_packages"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|r| r["id"] == "QUALIFY")
        .unwrap()["depends_on"] = serde_json::json!(["INTEGRATE"]);
    assert!(check(&p).unwrap_err().contains("release omits CF"));
    let mut p = read("EXECUTION_PLAN_v0.93.json");
    p["repositories"]["agent-logic-runtime"] = serde_json::json!("public");
    assert!(check(&p).is_err());
}

#[test]
fn rejects_design_and_implementation_proof_lane_inversion() {
    for (id, lane) in [
        ("RV-01", "installed_integration"),
        ("RV-02", "planning_contract"),
    ] {
        let mut p = read("EXECUTION_PLAN_v0.93.json");
        p["work_packages"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["id"] == id)
            .unwrap()["pvf"] = serde_json::json!(lane);
        assert!(check(&p).is_err());
    }
}

#[test]
fn rejects_omitted_template_and_citizen_release_requirements() {
    for (id, deps) in [
        ("CF-05", serde_json::json!(["CF-04"])),
        ("INTEGRATE", serde_json::json!(["DEMO-GOV", "DEMO-SEC"])),
    ] {
        let mut p = read("EXECUTION_PLAN_v0.93.json");
        p["work_packages"]
            .as_array_mut()
            .unwrap()
            .iter_mut()
            .find(|r| r["id"] == id)
            .unwrap()["depends_on"] = deps;
        assert!(check(&p).unwrap_err().contains("release omits"));
    }
}
