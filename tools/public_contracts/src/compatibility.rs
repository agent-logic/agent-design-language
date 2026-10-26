//! Read-only evidence correspondence. This module cannot grant acceptance.
use super::*;
use anyhow::ensure;
use std::collections::BTreeSet;
pub const ROLES: [&str; 7] = [
    "public_adl",
    "csdlc",
    "runtime",
    "infrastructure",
    "enterprise_adapter",
    "codefriend",
    "website",
];
const REPOS: [&str; 7] = [
    "agent-design-language",
    "cognitive-sdlc",
    "agent-logic-runtime",
    "agent-logic-infrastructure",
    "agent-logic-enterprise-security",
    "codefriend",
    "codefriend.ai",
];
pub const REQUIRED: [(&str, &str, &str, &str); 6] = [
    ("runtime", "public_adl", "distribution", "adl-engine"),
    ("runtime", "public_adl", "distribution", "adl-records"),
    (
        "infrastructure",
        "public_adl",
        "build",
        "adl-remote-validation",
    ),
    (
        "infrastructure",
        "runtime",
        "distribution",
        "runtime-candidate.tar",
    ),
    ("codefriend", "runtime", "build", "runtime-source"),
    (
        "enterprise_adapter",
        "runtime",
        "build",
        "adl-runtime-policy",
    ),
];
pub const NEGATIVES: [&str; 10] = [
    "missing_product_proof",
    "corrupt_artifact",
    "incompatible_version",
    "unproven_artifact",
    "hidden_sibling_source",
    "undeclared_build_cycle",
    "standalone_csdlc_runtime_dependency",
    "optional_adapter_absent_or_invalid",
    "mixed_active_owners",
    "unqualified_shared_tool_activation",
];
fn keys(v: &Value, names: &[&str], label: &str) -> Result<()> {
    let o = v.as_object().with_context(|| format!("{label} fields"))?;
    ensure!(
        o.len() == names.len() && names.iter().all(|k| o.contains_key(*k)),
        "{label} fields"
    );
    Ok(())
}
fn s<'a>(v: &'a Value, label: &str) -> Result<&'a str> {
    let s = v.as_str().with_context(|| format!("{label} missing"))?;
    ensure!(!s.trim().is_empty(), "{label} missing");
    Ok(s)
}
fn array<'a>(v: &'a Value, label: &str) -> Result<&'a Vec<Value>> {
    v.as_array().with_context(|| format!("{label} list"))
}
fn digest(v: &Value, len: usize, label: &str) -> Result<()> {
    let x = s(v, label)?;
    ensure!(
        x.len() == len
            && x.bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "{label} digest"
    );
    Ok(())
}
fn portable(v: &Value) -> Result<()> {
    match v {
        Value::String(x) => ensure!(
            ![
                "/Users/",
                "/Volumes/",
                "/private/tmp/",
                "file://",
                "BEGIN PRIVATE KEY"
            ]
            .iter()
            .any(|p| x.contains(p)),
            "private host content forbidden"
        ),
        Value::Array(a) => {
            for x in a {
                portable(x)?
            }
        }
        Value::Object(o) => {
            for x in o.values() {
                portable(x)?
            }
        }
        _ => {}
    }
    Ok(())
}
fn versions(v: &Value, label: &str, unique: bool) -> Result<()> {
    let a = array(v, label)?;
    ensure!(!a.is_empty(), "{label} explicit consumer versions");
    let mut seen = BTreeSet::new();
    for x in a {
        let x = s(x, label)?;
        ensure!(
            x != "*" && (!unique || seen.insert(x)),
            "{label} explicit consumer versions"
        );
    }
    Ok(())
}
fn reference(v: &Value, root: Option<&Path>, required: bool, label: &str) -> Result<()> {
    if v.is_null() {
        ensure!(!required, "{label} missing");
        return Ok(());
    }
    keys(v, &["path", "sha256"], label)?;
    digest(&v["sha256"], 64, label)?;
    let name = s(&v["path"], label)?;
    ensure!(
        archive::safe_name(name) && !name.split('/').any(|x| x.is_empty() || x == "."),
        "{label} relative path"
    );
    if required {
        let root = fs::canonicalize(root.context("evidence root required")?)?;
        let path = root.join(name);
        ensure!(
            fs::canonicalize(&path)?.starts_with(&root),
            "{label} escaped evidence root"
        );
        for part in path.ancestors().take_while(|p| p.starts_with(&root)) {
            ensure!(!part.is_symlink(), "{label} escaped evidence root");
        }
        ensure!(
            path.is_file() && archive::digest(&path)? == v["sha256"],
            "{label} bytes mismatch"
        );
    }
    Ok(())
}
fn identity(v: &Value, root: Option<&Path>, required: bool, label: &str) -> Result<()> {
    if v.is_null() {
        ensure!(!required, "{label} selection missing");
        return Ok(());
    }
    keys(
        v,
        &[
            "source_commit",
            "asset_identity",
            "sha256",
            "package_version",
            "dependency_lock",
            "build_provenance",
            "supported_consumer_versions",
        ],
        label,
    )?;
    digest(&v["source_commit"], 40, label)?;
    digest(&v["sha256"], 64, label)?;
    s(&v["package_version"], label)?;
    let a = &v["asset_identity"];
    if a.get("member").is_some() {
        keys(a, &["name", "path", "member"], "asset identity")?
    } else {
        keys(a, &["name", "path"], "asset identity")?
    }
    let name = s(&a["name"], "asset name")?;
    ensure!(
        !regex::Regex::new(r"(?i)(?:^|[/@:])(latest|main|master)(?:$|[/@:])")?.is_match(name),
        "mutable artifact identity"
    );
    reference(
        &json!({"path":a["path"],"sha256":v["sha256"]}),
        root,
        required,
        &format!("{label} actual artifact"),
    )?;
    if let Some(member) = a.get("member") {
        reference(member, root, false, "bundle member")?;
        if required {
            let data = fs::read(
                root.context("evidence root")?
                    .join(s(&a["path"], "asset path")?),
            )?;
            let files = archive::unpack(&data, data.starts_with(&[0x1f, 0x8b]))
                .context("invalid bundle archive")?;
            let payload = files
                .get(s(&member["path"], "member path")?)
                .context("bundle member absent/duplicate/nonregular")?;
            ensure!(
                archive::sha(&payload.0) == member["sha256"],
                "bundle member bytes mismatch"
            );
        }
    }
    versions(&v["supported_consumer_versions"], label, true)?;
    for k in ["dependency_lock", "build_provenance"] {
        reference(&v[k], root, required, &format!("{label} {k}"))?;
    }
    Ok(())
}
fn generation(v: &Value, root: Option<&Path>, required: bool) -> Result<()> {
    keys(
        v,
        &[
            "source_commit",
            "generation",
            "platform",
            "artifacts",
            "receipt",
            "supported_consumer_versions",
            "replay_authority",
        ],
        "installed generation",
    )?;
    digest(&v["source_commit"], 40, "generation")?;
    s(&v["generation"], "generation")?;
    s(&v["platform"], "platform")?;
    let artifacts = array(&v["artifacts"], "artifacts")?;
    ensure!(
        !artifacts.is_empty(),
        "installed generation artifacts missing"
    );
    let mut names = BTreeMap::new();
    for a in artifacts {
        keys(
            a,
            &["name", "path", "sha256"],
            "installed generation artifact",
        )?;
        let name = s(&a["name"], "artifact name")?;
        ensure!(
            names.insert(name, &a["sha256"]).is_none(),
            "duplicate installed generation artifact"
        );
        reference(
            &json!({"path":a["path"],"sha256":a["sha256"]}),
            root,
            required,
            "installed generation artifact",
        )?;
    }
    ensure!(
        names.keys().copied().collect::<BTreeSet<_>>()
            == BTreeSet::from(["adl-runtime-kernel", "adl-runtime-guardian", "csm"]),
        "exact installed generation artifacts required"
    );
    reference(
        &v["receipt"],
        root,
        required,
        "installed generation receipt",
    )?;
    versions(
        &v["supported_consumer_versions"],
        "installed generation",
        false,
    )?;
    ensure!(
        v["replay_authority"] == false,
        "installed generation replay authority forbidden"
    );
    if required {
        let receipt = load(
            &root
                .context("evidence root")?
                .join(s(&v["receipt"]["path"], "receipt path")?),
        )?;
        ensure!(
            receipt["source_revision"] == v["source_commit"]
                && receipt["generation"] == v["generation"]
                && receipt["platform"] == v["platform"],
            "installed generation receipt identity mismatch"
        );
        keys(
            &receipt["artifacts"],
            &["kernel", "guardian", "csm"],
            "installed generation receipt artifacts",
        )?;
        for (name, key) in [
            ("adl-runtime-kernel", "kernel"),
            ("adl-runtime-guardian", "guardian"),
            ("csm", "csm"),
        ] {
            ensure!(
                receipt["artifacts"][key]["sha256"] == *names[name],
                "installed generation receipt artifact mismatch"
            );
        }
    }
    Ok(())
}
fn argv(v: &Value, label: &str) -> Result<()> {
    let a = array(v, label)?;
    ensure!(
        !a.is_empty() && a.iter().all(|x| x.as_str().is_some_and(|x| !x.is_empty())),
        "exact {label} argv missing"
    );
    Ok(())
}
fn qualify(row: &Value, selected: &Value, root: Option<&Path>, role: &str) -> Result<()> {
    let kind = if row.get("operations").is_some() {
        "operations"
    } else if row.get("adapter_invocation").is_some() {
        "adapter"
    } else {
        "retained"
    };
    let mut fields = vec![
        "target",
        "source_commit",
        "artifact_sha256",
        "independent_clean",
        "distributable_only",
        "evidence",
    ];
    fields.extend(match kind {
        "operations" => vec!["operations"],
        "adapter" => vec!["adapter_invocation", "phase_outcomes"],
        _ => vec!["retained_acceptance"],
    });
    keys(row, &fields, "platform")?;
    s(&row["target"], "platform")?;
    ensure!(
        row["source_commit"] == selected["source_commit"]
            && row["artifact_sha256"] == selected["sha256"],
        "platform artifact/source mismatch"
    );
    ensure!(
        row["independent_clean"] == true && row["distributable_only"] == true,
        "independent clean distributable proof missing"
    );
    match kind {
        "operations" => {
            let ops = array(&row["operations"], "operations")?;
            ensure!(
                ops.len() == 3
                    && ops
                        .iter()
                        .filter_map(|x| x["operation"].as_str())
                        .collect::<BTreeSet<_>>()
                        == BTreeSet::from(["build", "test", "install"]),
                "three original operations required"
            );
            for op in ops {
                keys(
                    op,
                    &["operation", "argv", "exit_code", "executed", "skipped"],
                    "operation",
                )?;
                argv(&op["argv"], "operation")?;
                ensure!(
                    op["exit_code"].as_i64() == Some(0)
                        && op["executed"].as_u64().is_some_and(|n| n > 0)
                        && op["skipped"].as_u64() == Some(0),
                    "failed/zero/skipped operation cannot qualify"
                );
            }
        }
        "adapter" => {
            let i = &row["adapter_invocation"];
            keys(
                i,
                &["argv", "exit_code", "executed", "skipped"],
                "adapter invocation",
            )?;
            argv(&i["argv"], "adapter")?;
            ensure!(
                i["exit_code"].as_i64() == Some(0)
                    && i["executed"].as_u64() == Some(1)
                    && i["skipped"].as_u64() == Some(0),
                "adapter must record exactly one successful invocation"
            );
            let phases = array(&row["phase_outcomes"], "adapter phases")?;
            ensure!(
                phases.len() == 3
                    && phases
                        .iter()
                        .filter_map(|x| x["phase"].as_str())
                        .collect::<BTreeSet<_>>()
                        == BTreeSet::from(["build", "test", "install"]),
                "adapter build/test/install phase outcomes required"
            );
            for p in phases {
                keys(
                    p,
                    &["phase", "executed", "failed", "skipped", "claim"],
                    "adapter phase",
                )?;
                s(&p["claim"], "adapter phase claim")?;
                ensure!(
                    p["executed"].as_u64().is_some_and(|n| n > 0)
                        && p["failed"].as_u64() == Some(0)
                        && p["skipped"].as_u64() == Some(0),
                    "failed/zero/skipped adapter phase cannot qualify"
                );
            }
        }
        _ => {
            ensure!(role == "runtime", "retained acceptance is Runtime-only");
            let r = &row["retained_acceptance"];
            keys(
                r,
                &[
                    "build_provenance",
                    "installation_identity",
                    "scenario_acceptance",
                    "independent_review",
                    "scope_decision",
                    "scenarios_passed",
                    "exact_install_invocation_retained",
                    "current_qualification_status",
                    "limitations",
                ],
                "retained acceptance",
            )?;
            ensure!(
                r["scenarios_passed"].as_u64() == Some(7),
                "retained Runtime scenario denominator"
            );
            ensure!(
                r["exact_install_invocation_retained"] == false,
                "retained Runtime install argv must remain explicitly unavailable"
            );
            ensure!(
                r["current_qualification_status"] == "operator_deferred_outside_sprint1",
                "retained Runtime qualification disposition"
            );
            let limitations = array(&r["limitations"], "retained Runtime limitations")?;
            ensure!(
                !limitations.is_empty()
                    && limitations
                        .iter()
                        .all(|v| v.as_str().is_some_and(|s| !s.trim().is_empty())),
                "retained Runtime limitations"
            );
            for k in [
                "build_provenance",
                "installation_identity",
                "scenario_acceptance",
                "independent_review",
                "scope_decision",
            ] {
                reference(&r[k], root, true, &format!("retained Runtime {k}"))?;
            }
        }
    }
    reference(&row["evidence"], root, true, "platform qualification")
}
type Edge = (String, String, String, String);
type AssetKey = (String, String, String, Option<(String, String)>);
fn edge(v: &Value) -> Result<Edge> {
    Ok((
        s(&v["consumer"], "consumer")?.into(),
        s(&v["producer"], "producer")?.into(),
        s(&v["kind"], "kind")?.into(),
        v["interface"].as_str().unwrap_or("").into(),
    ))
}
fn required() -> BTreeSet<Edge> {
    REQUIRED
        .iter()
        .map(|(a, b, c, d)| (a.to_string(), b.to_string(), c.to_string(), d.to_string()))
        .collect()
}
pub fn verify(
    lock: &Value,
    graph: &Value,
    rollback: &Value,
    root: Option<&Path>,
    require_qualified: bool,
) -> Result<Value> {
    portable(lock)?;
    portable(graph)?;
    portable(rollback)?;
    keys(
        lock,
        &[
            "schema",
            "state",
            "roles",
            "rd07_acceptance",
            "negative_evidence",
            "independent_review",
        ],
        "lockset",
    )?;
    ensure!(
        lock["schema"] == "adl.compatibility_lockset.v1"
            && ["candidate", "accepted"].contains(&lock["state"].as_str().unwrap_or("")),
        "lockset schema/state"
    );
    let qualified = require_qualified || lock["state"] == "accepted";
    let rows = array(&lock["roles"], "roles")?;
    ensure!(
        rows.len() == 7
            && rows
                .iter()
                .filter_map(|r| r["role"].as_str())
                .collect::<BTreeSet<_>>()
                == BTreeSet::from(ROLES),
        "exact seven unique roles required"
    );
    let mut by_role = BTreeMap::new();
    let mut missing = vec![];
    let mut assets: Vec<AssetKey> = vec![];
    for row in rows {
        keys(
            row,
            &[
                "role",
                "repository",
                "available",
                "selected",
                "active_source_owners",
                "platforms",
                "website_disposition",
                "authentication",
                "activation",
            ],
            "role",
        )?;
        let role = s(&row["role"], "role")?;
        by_role.insert(role, row);
        let index = ROLES
            .iter()
            .position(|r| *r == role)
            .context("unknown role")?;
        ensure!(
            row["repository"] == format!("agent-logic/{}", REPOS[index]),
            "role/repository mismatch"
        );
        for a in array(&row["available"], "available manifest observations")? {
            keys(a, &["description", "evidence"], "available input")?;
            s(&a["description"], "available description")?;
            reference(&a["evidence"], root, false, "available evidence")?;
        }
        let selected = &row["selected"];
        identity(selected, root, qualified, role)?;
        if selected.is_null() {
            missing.push(format!("{role}: selected distribution"));
        } else {
            let a = &selected["asset_identity"];
            let member = a.get("member").filter(|m| !m.is_null()).map(|m| {
                (
                    m["path"].as_str().unwrap_or("").to_owned(),
                    m["sha256"].as_str().unwrap_or("").to_owned(),
                )
            });
            let key = (
                s(&selected["source_commit"], "source")?.to_owned(),
                s(&a["name"], "asset")?.to_owned(),
                s(&selected["sha256"], "sha")?.to_owned(),
                member,
            );
            ensure!(
                !assets.contains(&key),
                "duplicate selected asset across roles"
            );
            ensure!(
                !assets.iter().any(|p| p.0 == key.0
                    && p.1 == key.1
                    && p.2 == key.2
                    && (p.3.is_none() || key.3.is_none())),
                "whole bundle overlaps selected member"
            );
            assets.push(key);
        }
        let owners = array(&row["active_source_owners"], "owners")?;
        ensure!(
            owners.len() <= 1
                && owners
                    .iter()
                    .all(|v| v.as_str().is_some_and(|s| !s.is_empty())),
            "mixed active source owners"
        );
        ensure!(
            !qualified || owners.len() == 1,
            "qualified role owner missing"
        );
        ensure!(
            row["activation"] == "none",
            "shared activation is not authorized by lockset"
        );
        reference(
            &row["authentication"],
            root,
            qualified,
            &format!("{role} independent producer authentication"),
        )?;
        let platforms = array(&row["platforms"], "platform")?;
        let targets: Vec<_> = platforms
            .iter()
            .map(|p| s(&p["target"], "target"))
            .collect::<Result<_>>()?;
        ensure!(
            targets.iter().collect::<BTreeSet<_>>().len() == targets.len(),
            "duplicate platform row"
        );
        if role == "website" {
            ensure!(
                row["website_disposition"].is_null()
                    || ["bundle_api_compatible", "preserved_no_deployed_baseline"]
                        .contains(&row["website_disposition"].as_str().unwrap_or("")),
                "website disposition"
            );
            ensure!(
                !qualified || !row["website_disposition"].is_null(),
                "website compatibility/absence disposition missing"
            );
            ensure!(
                platforms.is_empty(),
                "website does not inherit software platform proofs"
            );
        } else {
            ensure!(
                row["website_disposition"].is_null(),
                "software website disposition"
            );
            ensure!(
                !qualified || !platforms.is_empty(),
                "supported software platform proof missing"
            );
            if selected.is_null() {
                ensure!(
                    platforms.is_empty(),
                    "unselected artifact cannot have platform proof"
                );
            } else {
                for p in platforms {
                    qualify(p, selected, root, role)?;
                }
            }
        }
    }
    let dispositions = match graph["schema"].as_str() {
        Some("adl.compatibility_graph.v1") => {
            keys(graph, &["schema", "edges"], "graph")?;
            vec![]
        }
        Some("adl.compatibility_graph.v2") => {
            keys(
                graph,
                &["schema", "edges", "dependency_dispositions"],
                "graph",
            )?;
            array(&graph["dependency_dispositions"], "dependency dispositions")?
                .iter()
                .collect()
        }
        _ => bail!("graph schema"),
    };
    let mut edges = BTreeSet::new();
    let mut build: BTreeMap<&str, BTreeSet<&str>> =
        ROLES.iter().map(|r| (*r, BTreeSet::new())).collect();
    for e in array(&graph["edges"], "edges")? {
        keys(
            e,
            &[
                "consumer",
                "producer",
                "kind",
                "interface",
                "version",
                "optional",
                "evidence",
            ],
            "edge",
        )?;
        let (a, b, k, i) = edge(e)?;
        ensure!(
            ROLES.contains(&a.as_str()) && ROLES.contains(&b.as_str()) && a != b,
            "unknown/self dependency"
        );
        ensure!(
            ["build", "distribution", "runtime", "api"].contains(&k.as_str())
                && e["optional"].is_boolean(),
            "dependency kind/optionality"
        );
        ensure!(
            edges.insert((a.clone(), b.clone(), k.clone(), i)),
            "duplicate dependency edge"
        );
        ensure!(
            a != "csdlc" || b != "runtime",
            "standalone CSDLC Runtime dependency"
        );
        ensure!(
            a != "runtime"
                || b != "enterprise_adapter"
                || (e["optional"] == true && k == "runtime"),
            "mandatory enterprise dependency"
        );
        if qualified {
            s(&e["interface"], "versioned interface")?;
            s(&e["version"], "interface version")?;
        } else if !e["interface"].is_null() {
            s(&e["interface"], "interface")?;
        }
        reference(
            &e["evidence"],
            root,
            qualified,
            "declared interface evidence",
        )?;
        if ["build", "distribution"].contains(&k.as_str()) {
            build
                .get_mut(e["consumer"].as_str().unwrap())
                .unwrap()
                .insert(e["producer"].as_str().unwrap());
        }
    }
    let mut disposed = BTreeSet::new();
    for d in dispositions {
        keys(
            d,
            &[
                "schema",
                "consumer",
                "producer",
                "kind",
                "interface",
                "disposition",
                "selected_source_commit",
                "selected_artifact_sha256",
                "dependency_lock",
                "build_provenance",
                "authentication",
                "evidence",
            ],
            "dependency disposition",
        )?;
        ensure!(
            d["schema"] == "adl.compatibility_dependency_disposition.v1",
            "dependency disposition schema"
        );
        let key = edge(d)?;
        ensure!(
            key == (
                "codefriend".into(),
                "runtime".into(),
                "build".into(),
                "runtime-source".into()
            ) && disposed.insert(key.clone()),
            "unsupported/duplicate dependency disposition"
        );
        ensure!(
            !edges.contains(&key),
            "dependency edge and disposition conflict"
        );
        ensure!(
            d["disposition"] == "independently_built_no_dependency",
            "dependency disposition"
        );
        let owner = by_role["codefriend"];
        let selected = &owner["selected"];
        ensure!(
            !selected.is_null(),
            "dependency disposition selected distribution missing"
        );
        ensure!(
            d["selected_source_commit"] == selected["source_commit"]
                && d["selected_artifact_sha256"] == selected["sha256"],
            "dependency disposition selected identity mismatch"
        );
        ensure!(
            d["dependency_lock"] == selected["dependency_lock"]
                && d["build_provenance"] == selected["build_provenance"],
            "dependency disposition source declaration mismatch"
        );
        ensure!(
            d["authentication"] == owner["authentication"],
            "dependency disposition authentication mismatch"
        );
        for k in [
            "dependency_lock",
            "build_provenance",
            "authentication",
            "evidence",
        ] {
            reference(
                &d[k],
                root,
                qualified,
                &format!("dependency disposition {k}"),
            )?;
        }
    }
    let required = required();
    ensure!(
        required
            .iter()
            .all(|e| edges.contains(e) || disposed.contains(e)),
        "original concrete interface/kind obligation missing"
    );
    for e in array(&graph["edges"], "edges")? {
        if required.contains(&edge(e)?) {
            ensure!(
                e["optional"] == false,
                "required interface cannot be optional"
            );
        }
    }
    if qualified && by_role["website"]["website_disposition"] == "bundle_api_compatible" {
        ensure!(
            edges
                .iter()
                .any(|(a, b, k, _)| a == "website" && b == "codefriend" && k == "api"),
            "website API compatibility edge missing"
        );
    }
    fn walk<'a>(
        node: &'a str,
        build: &BTreeMap<&'a str, BTreeSet<&'a str>>,
        active: &mut BTreeSet<&'a str>,
        done: &mut BTreeSet<&'a str>,
    ) -> Result<()> {
        ensure!(!active.contains(node), "undeclared build-order cycle");
        if done.contains(node) {
            return Ok(());
        }
        active.insert(node);
        for dep in &build[node] {
            walk(dep, build, active, done)?;
        }
        active.remove(node);
        done.insert(node);
        Ok(())
    }
    let mut done = BTreeSet::new();
    for role in ROLES {
        walk(role, &build, &mut BTreeSet::new(), &mut done)?;
    }
    keys(rollback, &["schema", "roles"], "rollback")?;
    ensure!(
        rollback["schema"] == "adl.compatibility_rollback.v1",
        "rollback schema"
    );
    let rollback_rows = array(&rollback["roles"], "rollback roles")?;
    ensure!(
        rollback_rows.len() == 7
            && rollback_rows
                .iter()
                .filter_map(|r| r["role"].as_str())
                .collect::<BTreeSet<_>>()
                == BTreeSet::from(ROLES),
        "exact seven rollback rows"
    );
    for r in rollback_rows {
        keys(
            r,
            &[
                "role",
                "disposition",
                "prior",
                "prior_observations",
                "restore_argv",
                "recoverability",
            ],
            "rollback row",
        )?;
        for p in array(&r["prior_observations"], "prior observations")? {
            keys(p, &["description", "evidence"], "prior observation")?;
            s(&p["description"], "prior description")?;
            reference(&p["evidence"], root, false, "prior observation")?;
        }
        let disposition = s(&r["disposition"], "rollback disposition")?;
        ensure!(
            [
                "pending",
                "retained_prior",
                "retained_installed_generation",
                "no_prior_accepted_baseline"
            ]
            .contains(&disposition),
            "rollback disposition"
        );
        ensure!(
            !qualified || disposition != "pending",
            "rollback unresolved"
        );
        match disposition {
            "retained_prior" => {
                identity(&r["prior"], root, qualified, "prior distribution")?;
                argv(&r["restore_argv"], "existing restore command")?;
            }
            "retained_installed_generation" => {
                generation(&r["prior"], root, qualified)?;
                ensure!(
                    r["restore_argv"] == json!([]),
                    "historical installed generation cannot imply replay argv"
                );
            }
            _ => ensure!(
                r["prior"].is_null() && r["restore_argv"] == json!([]),
                "absence/pending cannot imply prior restore"
            ),
        };
        reference(
            &r["recoverability"],
            root,
            qualified,
            "rollback recovery or baseline absence evidence",
        )?;
    }
    reference(&lock["rd07_acceptance"], root, qualified, "accepted RD07")?;
    reference(
        &lock["independent_review"],
        root,
        qualified,
        "independent exact lockset review",
    )?;
    let mut negatives = BTreeSet::new();
    for n in array(&lock["negative_evidence"], "negative evidence")? {
        keys(
            n,
            &["case", "observed_refusal", "executed", "evidence"],
            "negative",
        )?;
        let name = s(&n["case"], "negative case")?;
        ensure!(
            NEGATIVES.contains(&name) && negatives.insert(name),
            "unknown/duplicate negative"
        );
        s(&n["observed_refusal"], "actual refusal")?;
        ensure!(
            n["executed"].as_u64().is_some_and(|n| n > 0),
            "zero negative proof"
        );
        reference(&n["evidence"], root, qualified, "negative evidence")?;
    }
    ensure!(
        !qualified || negatives == BTreeSet::from(NEGATIVES),
        "original negative proof missing"
    );
    Ok(
        json!({"status":if qualified{"qualified_evidence_correspondence"}else{"candidate_structure_valid"},"roles":7,"pending_selections":missing,"acceptance_authority":false,"activation_performed":false,"limits":"Hashes/normalized claims do not authenticate producers or independently review proofs; execution owner supplies authentic evidence and native acceptance."}),
    )
}
// serde_json's default Value parser accepts duplicate keys; evidence must not.
struct Unique(Value);
impl<'de> serde::Deserialize<'de> for Unique {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct Visitor;
        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Unique;
            fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                f.write_str("JSON with unique keys")
            }
            fn visit_map<M: serde::de::MapAccess<'de>>(
                self,
                mut m: M,
            ) -> std::result::Result<Unique, M::Error> {
                let mut o = serde_json::Map::new();
                while let Some((k, v)) = m.next_entry::<String, Unique>()? {
                    if o.insert(k, v.0).is_some() {
                        return Err(serde::de::Error::custom("duplicate JSON key"));
                    }
                }
                Ok(Unique(Value::Object(o)))
            }
            fn visit_seq<A: serde::de::SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Unique, A::Error> {
                let mut v = vec![];
                while let Some(x) = a.next_element::<Unique>()? {
                    v.push(x.0);
                }
                Ok(Unique(Value::Array(v)))
            }
            fn visit_bool<E: serde::de::Error>(self, v: bool) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_i64<E: serde::de::Error>(self, v: i64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_u64<E: serde::de::Error>(self, v: u64) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_f64<E: serde::de::Error>(self, v: f64) -> std::result::Result<Unique, E> {
                Ok(Unique(
                    serde_json::Number::from_f64(v)
                        .ok_or_else(|| E::custom("invalid number"))?
                        .into(),
                ))
            }
            fn visit_str<E: serde::de::Error>(self, v: &str) -> std::result::Result<Unique, E> {
                Ok(Unique(v.into()))
            }
            fn visit_none<E: serde::de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
            fn visit_unit<E: serde::de::Error>(self) -> std::result::Result<Unique, E> {
                Ok(Unique(Value::Null))
            }
        }
        d.deserialize_any(Visitor)
    }
}
pub fn load(path: &Path) -> Result<Value> {
    ensure!(
        path.is_file() && !path.is_symlink(),
        "regular JSON input required"
    );
    Ok(serde_json::from_slice::<Unique>(&fs::read(path)?)?.0)
}
