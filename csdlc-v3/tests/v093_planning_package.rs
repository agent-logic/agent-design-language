//! PVF: deterministic local planning-contract proof, no network/providers/cloud.
//! Release gate for #1047 planning consistency only; not Runtime/product proof.
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;

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
    let mut release = BTreeSet::new();
    visit("TAIL-10", &by_id, &mut BTreeSet::new(), &mut release)?;
    for prefix_count in [("RV", 8), ("CF", 7)] {
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
    if plan["status"] != "first_pass_not_open" {
        return Err("opening claim".into());
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
        ] {
            assert_eq!(r[a], w[b], "{} {a}", r["id"]);
        }
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
