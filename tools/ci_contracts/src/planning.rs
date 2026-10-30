//! Historical split proof, retaining the original constraints and 27 mutation probes.
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
    process::Command,
};
type Result<T> = std::result::Result<T, String>;
macro_rules! require {
    ($e:expr) => {
        if !$e {
            return Err(format!("planning constraint: {}", stringify!($e)));
        }
    };
}
fn a(v: &Value) -> Result<&Vec<Value>> {
    v.as_array().ok_or_else(|| "expected planning array".into())
}
fn s(v: &Value) -> Result<&str> {
    v.as_str().ok_or_else(|| "expected planning string".into())
}
fn o(v: &Value) -> Result<&serde_json::Map<String, Value>> {
    v.as_object()
        .ok_or_else(|| "expected planning object".into())
}
fn set(v: &Value) -> Result<BTreeSet<String>> {
    a(v)?.iter().map(|x| s(x).map(str::to_owned)).collect()
}
fn read(root: &Path, v: &str, name: &str) -> Result<Value> {
    let mut base = root.join("docs/milestones").join(v);
    if v == "v0.93.2"
        && [
            "EXECUTION_PLAN_v0.93.2.json",
            "WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml",
            "WP_ISSUE_WAVE_v0.93.2.yaml",
        ]
        .contains(&name)
    {
        base = base.join("baseline-before-reorganization");
    }
    serde_json::from_slice(&fs::read(base.join(name)).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn rows(plan: &Value) -> Result<BTreeMap<String, Value>> {
    a(&plan["work_packages"])?
        .iter()
        .map(|r| Ok((s(&r["id"])?.to_owned(), r.clone())))
        .collect()
}
fn ancestors(
    key: &str,
    plans: &Value,
    all: &BTreeMap<String, Value>,
    active: &mut BTreeSet<String>,
) -> Result<BTreeSet<String>> {
    require!(active.insert(key.into()));
    let row = all.get(key).ok_or("missing dependency")?;
    let version = key.split('/').next().ok_or("invalid key")?;
    let mut found = BTreeSet::new();
    for dep in a(&row["depends_on"])? {
        if o(&plans[version]["external_dependencies"])?.contains_key(s(dep)?) {
            continue;
        }
        let dep = s(dep)?;
        let target = if dep.contains('/') {
            dep.into()
        } else {
            format!("{version}/{dep}")
        };
        found.insert(target.clone());
        found.extend(ancestors(&target, plans, all, active)?);
    }
    active.remove(key);
    Ok(found)
}
pub fn validate(root: &Path, plans: &Value, mapping: &Value, inspect: bool) -> Result<()> {
    let mut all = BTreeMap::new();
    for (version, plan) in o(plans)? {
        require!(
            plan["status"]
                == if version == "v0.93.1" {
                    "sprint_1_preparation_open_execution_not_authorized"
                } else {
                    "wp01_open_downstream_not_activated"
                }
        );
        let rows = rows(plan)?;
        require!(
            rows.len() == a(&plan["work_packages"])?.len()
                && Some(rows.len() as u64) == plan["counts"]["core_work_packages"].as_u64()
        );
        require!(
            plan["repositories"]["codefriend"] == "private"
                && plan["repositories"]["codefriend.ai"] == "private"
        );
        require!(
            o(&plan["repositories"])?
                .iter()
                .filter(|(_, v)| *v == "public")
                .map(|(k, _)| k.as_str())
                .collect::<Vec<_>>()
                == vec!["agent-design-language"]
        );
        let mut assignment = BTreeMap::new();
        for (n, sprint) in a(&plan["sprint_plan"]["sprints"])?.iter().enumerate() {
            require!(sprint["number"] == n + 1);
            for item in a(&sprint["work_packages"])? {
                let item = s(item)?;
                require!(
                    rows.contains_key(item) && assignment.insert(item.to_owned(), n + 1).is_none()
                );
            }
        }
        require!(assignment.keys().eq(rows.keys()));
        for (key, row) in &rows {
            require!(row["sprint"] == assignment[key]);
            require!(
                !s(&row["result"])?.is_empty()
                    && !a(&row["acceptance"])?.is_empty()
                    && !a(&row["negative_cases"])?.is_empty()
            );
            require!(!s(&row["task_boundary"]["single_result"])?.is_empty());
            if inspect {
                require!(root
                    .join("docs/milestones")
                    .join(version)
                    .join(s(&row["source"])?)
                    .is_file());
            }
            for dep in a(&row["depends_on"])? {
                let ds = s(dep)?;
                if o(&plan["external_dependencies"])?.contains_key(s(dep)?) {
                    if ds.contains('/') {
                        require!(ds.starts_with("v0.93.1/") && version == "v0.93.2");
                    }
                } else {
                    require!(rows.contains_key(ds) && assignment[ds] <= assignment[key]);
                }
            }
            all.insert(format!("{version}/{key}"), row.clone());
        }
        for n in 2..=10 {
            require!(
                rows[&format!("TAIL-{n:02}")]["depends_on"]
                    == json!([format!("TAIL-{:02}", n - 1)])
            );
        }
        if inspect {
            let spec = read(
                root,
                version,
                &format!("WP_EXECUTION_SPECIFICATIONS_{version}.yaml"),
            )?;
            let wave = read(root, version, &format!("WP_ISSUE_WAVE_{version}.yaml"))?;
            require!(
                spec["work_packages"] == plan["work_packages"]
                    && spec["sprint_plan"] == plan["sprint_plan"]
                    && wave["sprint_plan"] == plan["sprint_plan"]
            );
            require!(a(&wave["work_packages"])?.len() == rows.len());
            for (row, item) in a(&plan["work_packages"])?
                .iter()
                .zip(a(&wave["work_packages"])?)
            {
                for (a, b) in [
                    ("id", "wp"),
                    ("result", "outcome"),
                    ("depends_on", "depends_on"),
                    ("acceptance", "acceptance"),
                    ("sprint", "sprint"),
                    ("pvf", "pvf"),
                    ("negative_cases", "negative_cases"),
                    ("existing_issue", "existing_issue"),
                    ("issue_repository", "issue_repository"),
                    ("dependency_gates", "dependency_gates"),
                ] {
                    require!(row[a] == item[b]);
                }
            }
        }
    }
    let anc = |key: &str| ancestors(key, plans, &all, &mut BTreeSet::new());
    for key in all.keys() {
        anc(key)?;
    }
    let first = &plans["v0.93.1"];
    let fr = rows(first)?;
    require!(!fr.keys().any(|k| k.starts_with("RV-")));
    require!(fr["CF-05"]["existing_issue"] == 1150);
    require!(!a(&fr["CF-05"]["acceptance"])?.iter().any(|x| x
        .as_str()
        .unwrap_or("")
        .contains("closure of the #915 successors #1148, #1149 and #1150")));
    require!(!s(&fr["CF-01"]["result"])?
        .contains("Missing Beta 1 behavior remains a predecessor blocker."));
    require!(
        a(&first["existing_issue_routing"])?
            .iter()
            .map(|r| r["issue"].as_u64())
            .collect::<BTreeSet<_>>()
            == [875, 1145, 1148, 1149, 1150]
                .into_iter()
                .map(Some)
                .collect()
    );
    require!(first["counts"]["distinct_planned_issue_identities"] == 47);
    let prep = &first["preparation"];
    let expected = json!({"WP-01":1178,"RD-01":1181,"RD-02":1182,"RD-12":1183,"RD-13":1184,"RD-03":1185,"RD-04":1186,"RD-05":1187,"RD-06":1188,"RD-09":1189,"RD-10":1190,"RD-08":1191,"RD-07":1192,"RD-11":1193});
    require!(
        prep["issue_map"] == expected && prep["umbrella_issue"] == 1180 && prep["sprint"] == 1
    );
    require!(
        prep["execution_authorized"] == false
            && prep["closeout_issue_creation_authorized"] == false
            && prep["wp01_terminal_closure_required"] == false
    );
    require!(
        prep["later_sprint_creation"] == "after_split_acceptance"
            && prep["checkpoint_status"] == "pending_independent_acceptance"
    );
    require!(
        set(&first["sprint_plan"]["sprints"][0]["work_packages"])?
            == o(&expected)?.keys().cloned().collect()
    );
    for (key, issue) in o(&expected)? {
        require!(
            fr[key]["existing_issue"] == *issue
                && fr[key]["issue_repository"] == "agent-logic/agent-design-language"
        );
    }
    require!(
        fr["RD-01"]["dependency_gates"]
            == json!({"WP-01":"accepted_sprint_1_preparation_checkpoint_not_terminal_closure"})
    );
    for key in ["WP-01", "RD-01", "RD-02"] {
        require!(fr[key]["pvf"] == "planning_contract");
        require!(!a(&fr[key]["acceptance"])?.iter().any(|x| x
            .as_str()
            .unwrap_or("")
            .contains("Installed consumer evidence")));
    }
    require!(a(&fr["WP-01"]["acceptance"])?
        .iter()
        .any(|x| x.as_str().unwrap_or("").contains("remains OPEN")));
    require!(
        s(&fr["RD-13"]["result"])?.contains("transfer")
            && s(&fr["RD-13"]["result"])?.contains("existing")
    );
    require!(
        fr["RD-03"]["depends_on"] == json!(["RD-13"])
            && fr["RD-04"]["depends_on"] == json!(["RD-13"])
    );
    require!(
        first["counts"]["new_core_issues_to_seed_at_most"] == 28
            && first["counts"]["existing_additional_issues"] == 4
    );
    let successor = &plans["v0.93.2"];
    let opening = &successor["opening"];
    require!(
        opening["issue"] == 1296
            && opening["execution_authorized"] == json!(["WP-01"])
            && opening["owner"] == "Planning #12"
            && opening["authorized_on"] == "2026-10-19"
    );
    require!(
        opening["downstream_execution_authorized"] == false
            && opening["deployment_authorized"] == false
            && opening["predecessor_issue"] == 1230
    );
    require!(
        opening["predecessor_closure"] == "closed_with_native_coordination_completed"
            && opening["acceptance_merge"] == "69fae86cde65861cdace95183c2c373dcb09ca01"
    );
    let sprints: Vec<_> = a(&successor["sprint_plan"]["sprints"])?
        .iter()
        .map(|x| {
            Ok((
                x["number"].as_u64().ok_or("sprint number")?,
                a(&x["work_packages"])?.len(),
            ))
        })
        .collect::<Result<_>>()?;
    require!(sprints == vec![(1, 12), (2, 22), (3, 6), (4, 14), (5, 13)]);
    let routing: BTreeSet<_> = a(&successor["existing_issue_routing"])?
        .iter()
        .map(|r| {
            Ok((
                s(&r["wp"])?.to_owned(),
                s(&r["repository"])?.to_owned(),
                r["issue"].as_u64().ok_or("issue")?,
            ))
        })
        .collect::<Result<_>>()?;
    require!(
        routing
            == [
                ("WP-01", "agent-logic/agent-design-language", 1296),
                ("GHB-WP07", "agent-logic/agent-logic-runtime", 112),
                ("CF-92", "agent-logic/codefriend", 92)
            ]
            .into_iter()
            .map(|(a, b, c)| (a.into(), b.into(), c))
            .collect()
    );
    require!(!set(&successor["required_release_tracks"])?.contains("PY-01"));
    if inspect {
        require!(root
            .join("docs/milestones/v0.93.2")
            .join(s(&opening["record"])?)
            .is_file());
        for name in [
            "WP_EXECUTION_SPECIFICATIONS_v0.93.2.yaml",
            "WP_ISSUE_WAVE_v0.93.2.yaml",
        ] {
            let mirror = read(root, "v0.93.2", name)?;
            for field in ["opening", "status", "existing_issue_routing"] {
                require!(mirror[field] == successor[field]);
            }
        }
    }
    require!(successor["counts"]["distinct_planned_issue_identities"] == 67);
    require!(
        mapping["existing_issue_routing"]
            == json!({"v0.93.1":[1148,1149,1150,875,1145],"v0.93.2":[]})
    );
    if inspect {
        let text = fs::read_to_string(
            root.join("docs/milestones/v0.93")
                .join(s(&mapping["operator_amendment"])?),
        )
        .map_err(|e| e.to_string())?;
        require!(text.contains("#923"));
    }
    for key in fr.keys() {
        let ancestors = anc(&format!("v0.93.1/{key}"))?;
        if key != "WP-01" && !key.starts_with("RD-") {
            require!(ancestors.contains("v0.93.1/RD-11"));
        }
        if key.starts_with("RD-") {
            require!(!ancestors
                .iter()
                .any(|a| ["/CF-", "/CT-", "/RV-"].iter().any(|p| a.contains(p))));
        }
    }
    require!(!anc("v0.93.1/CF-07")?
        .iter()
        .any(|a| a.starts_with("v0.93.2/")));
    for (v, prefixes) in [
        ("v0.93.1", vec![("CF", 7), ("CT", 10)]),
        (
            "v0.93.2",
            vec![("RV", 11), ("GOV", 16), ("WP-S", 6), ("CM", 4)],
        ),
    ] {
        let release = anc(&format!("{v}/TAIL-10"))?;
        for (prefix, count) in prefixes {
            for n in 1..=count {
                let key = if prefix == "WP-S" {
                    format!("{prefix}{n}")
                } else {
                    format!("{prefix}-{n:02}")
                };
                require!(release.contains(&format!("{v}/{key}")));
            }
        }
    }
    let original = rows(&read(root, "v0.93", "EXECUTION_PLAN_v0.93.json")?)?;
    let mapped = a(&mapping["source_disposition"])?;
    let ids: BTreeSet<_> = mapped
        .iter()
        .map(|r| s(&r["source_id"]).map(str::to_owned))
        .collect::<Result<_>>()?;
    require!(mapped.len() == 83 && ids.len() == 83 && ids == original.keys().cloned().collect());
    for row in mapped {
        if row["source_id"] == "PY-01" {
            require!(
                row["destination"] == "later_milestone_unassigned"
                    && row["destination_id"] == "PY-01"
                    && row["disposition"] == "deferred_by_1242"
                    && row["definition"]
                        == "v0.93.2/tail07-1242/SUCCESSOR_PLAN.json#deferred/PY-01"
            );
            require!(!all.contains_key("v0.93.2/PY-01"));
            let deferred = read(root, "v0.93.2", "tail07-1242/SUCCESSOR_PLAN.json")?;
            let preserved = &a(&deferred["deferred"])?
                .iter()
                .find(|r| r["id"] == "PY-01")
                .ok_or("missing PY deferral")?["original"];
            require!(
                preserved["id"] == "PY-01"
                    && !a(&preserved["acceptance"])?.is_empty()
                    && !a(&preserved["negative_cases"])?.is_empty()
            );
        } else {
            require!(all.contains_key(&format!(
                "{}/{}",
                s(&row["destination"])?,
                s(&row["destination_id"])?
            )));
        }
    }
    let mut extras: BTreeSet<String> = a(&mapping["additional_version_specific_gates"])?
        .iter()
        .map(|r| Ok(format!("{}/{}", s(&r["milestone"])?, s(&r["id"])?)))
        .collect::<Result<_>>()?;
    require!(extras.len() == 13);
    extras.extend(["v0.93.2/CF-92".into(), "v0.93.2/GHB-SPRINT".into()]);
    for n in 0..13 {
        extras.insert(format!("v0.93.2/GHB-WP{n:02}"));
    }
    let destinations: BTreeSet<_> = mapped
        .iter()
        .map(|r| {
            Ok(format!(
                "{}/{}",
                s(&r["destination"])?,
                s(&r["destination_id"])?
            ))
        })
        .collect::<Result<_>>()?;
    require!(
        all.keys()
            .filter(|k| !destinations.contains(*k))
            .cloned()
            .collect::<BTreeSet<_>>()
            == extras
    );
    Ok(())
}
pub fn inputs(root: &Path) -> Result<(Value, Value)> {
    Ok((
        json!({"v0.93.1":read(root,"v0.93.1","EXECUTION_PLAN_v0.93.1.json")?,"v0.93.2":read(root,"v0.93.2","EXECUTION_PLAN_v0.93.2.json")?}),
        read(root, "v0.93", "MILESTONE_SPLIT_v0.93.json")?,
    ))
}
pub fn run(root: &Path) -> Result<Value> {
    let status = Command::new("node")
        .args(["--test", "adl/tools/validate_v0932_planning.test.mjs"])
        .current_dir(root)
        .stdout(std::process::Stdio::inherit())
        .status()
        .map_err(|e| e.to_string())?;
    require!(status.success());
    let (plans, mapping) = inputs(root)?;
    validate(root, &plans, &mapping, true)?;
    let negatives = negative_probes(root, &plans, &mapping)?;
    Ok(
        json!({"status":"pass","source_tasks":83,"successor_tasks":[43,67],"version_specific_gates":13,"execution_opened":true,"negative_fixtures":negatives}),
    )
}
fn row_mut<'a>(plans: &'a mut Value, version: &str, id: &str) -> Result<&'a mut Value> {
    plans[version]["work_packages"]
        .as_array_mut()
        .ok_or("work packages")?
        .iter_mut()
        .find(|r| r["id"] == id)
        .ok_or_else(|| "missing mutation target".into())
}
pub fn negative_probes(root: &Path, plans: &Value, mapping: &Value) -> Result<usize> {
    for index in 0..27 {
        let (mut p, mut m) = (plans.clone(), mapping.clone());
        match index {
            0 => {
                m["source_disposition"].as_array_mut().unwrap().pop();
            }
            1 => m["source_disposition"][0] = m["source_disposition"][1].clone(),
            2 => p["v0.93.1"]["work_packages"][1]["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("MISSING")),
            3 => p["v0.93.1"]["work_packages"][0]["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("RD-11")),
            4 => row_mut(&mut p, "v0.93.1", "CF-02")?["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("v0.93.2/RV-08")),
            5 => row_mut(&mut p, "v0.93.1", "RD-10")?["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("CF-05")),
            6 => row_mut(&mut p, "v0.93.1", "CF-05")?["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("TAIL-10")),
            7 => {
                row_mut(&mut p, "v0.93.1", "CF-05")?
                    .as_object_mut()
                    .unwrap()
                    .remove("existing_issue");
            }
            8 => row_mut(&mut p, "v0.93.1", "CT-03")?["depends_on"]
                .as_array_mut()
                .unwrap()
                .retain(|x| x != "CT-09"),
            9 => p["v0.93.1"]["repositories"]["codefriend"] = json!("public"),
            10 => row_mut(&mut p, "v0.93.2", "TAIL-10")?["depends_on"] = json!([]),
            11 => p["v0.93.2"]["opening"]["downstream_execution_authorized"] = json!(true),
            12 => p["v0.93.2"]["opening"]["deployment_authorized"] = json!(true),
            13 => p["v0.93.2"]["opening"]["predecessor_closure"] = json!("pending"),
            14 => p["v0.93.2"]["status"] = json!("open"),
            15 => p["v0.93.1"]["preparation"]["issue_map"]["RD-01"] = json!(1178),
            16 => {
                row_mut(&mut p, "v0.93.1", "RD-01")?["dependency_gates"]["WP-01"] =
                    json!("terminal_closure")
            }
            17 => p["v0.93.1"]["preparation"]["execution_authorized"] = json!(true),
            18 => p["v0.93.1"]["preparation"]["later_sprint_creation"] = json!("now"),
            19 => p["v0.93.1"]["preparation"]["closeout_issue_creation_authorized"] = json!(true),
            20 => row_mut(&mut p, "v0.93.1", "RD-01")?["acceptance"]
                .as_array_mut()
                .unwrap()
                .push(json!("Installed consumer evidence")),
            21 => row_mut(&mut p, "v0.93.1", "RD-13")?["result"] = json!("Create new issues"),
            22 => row_mut(&mut p, "v0.93.1", "RD-04")?["depends_on"]
                .as_array_mut()
                .unwrap()
                .push(json!("RD-03")),
            23 => p["v0.93.2"]["existing_issue_routing"]
                .as_array_mut()
                .unwrap()
                .push(json!({"issue":875})),
            24 => p["v0.93.1"]["counts"]["core_work_packages"] = json!(45),
            25 | 26 => {
                let row = m["source_disposition"]
                    .as_array_mut()
                    .unwrap()
                    .iter_mut()
                    .find(|r| r["source_id"] == "PY-01")
                    .unwrap();
                if index == 25 {
                    row.as_object_mut().unwrap().remove("disposition");
                } else {
                    row["destination"] = json!("v0.93.2");
                }
            }
            _ => unreachable!(),
        }
        if validate(root, &p, &m, false).is_ok() {
            return Err(format!("planning negative accepted: {index}"));
        }
    }
    Ok(27)
}
