//! Offline #1275 report consistency only; never lifecycle/product acceptance.
use anyhow::{anyhow, ensure, Context, Result};
use serde_json::Value;
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

pub type Documents = BTreeMap<String, Value>;
fn field<'a>(v: &'a Value, key: &str) -> Result<&'a Value> {
    v.get(key)
        .filter(|v| !v.is_null())
        .with_context(|| format!("missing {key}"))
}
fn string<'a>(v: &'a Value, key: &str) -> Result<&'a str> {
    field(v, key)?
        .as_str()
        .filter(|s| !s.is_empty())
        .with_context(|| format!("invalid {key}"))
}
fn yes(v: &Value, key: &str) -> Result<()> {
    ensure!(field(v, key)? == true, "{key} must be true");
    Ok(())
}
fn equal(a: &Value, ak: &str, b: &Value, bk: &str) -> Result<()> {
    ensure!(field(a, ak)? == field(b, bk)?, "{ak}/{bk} mismatch");
    Ok(())
}
fn rows(v: &Value) -> Result<&Vec<Value>> {
    field(v, "repositories")?
        .as_array()
        .context("repositories must be an array")
}
fn index(v: &Value) -> Result<BTreeMap<&str, &Value>> {
    let mut result = BTreeMap::new();
    for row in rows(v)? {
        let repo = string(row, "repository")?;
        ensure!(
            result.insert(repo, row).is_none(),
            "duplicate repository: {repo}"
        );
    }
    Ok(result)
}
fn keys<'a>(m: &BTreeMap<&'a str, &Value>) -> BTreeSet<&'a str> {
    m.keys().copied().collect()
}
fn findings(v: &Value) -> Result<()> {
    for f in field(v, "doctor_findings")?
        .as_array()
        .context("doctor_findings must be array")?
    {
        ensure!(
            string(f, "status")? == "passed",
            "doctor finding not passed"
        );
    }
    Ok(())
}
fn identity(
    receipt: &Value,
    repo: &str,
    issue: &Value,
    pr: &Value,
    head: &Value,
    head_key: &str,
) -> Result<()> {
    ensure!(
        string(receipt, "repository")? == repo,
        "repository mismatch"
    );
    ensure!(field(receipt, "issue")? == issue, "issue mismatch");
    ensure!(
        field(receipt, "pull_request")? == pr,
        "pull request mismatch"
    );
    ensure!(field(receipt, head_key)? == head, "head mismatch");
    Ok(())
}
pub fn load(directory: &Path) -> Result<Documents> {
    let mut docs = Documents::new();
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.extension().is_some_and(|s| s == "json") {
            let name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .context("invalid document name")?;
            docs.insert(
                name.into(),
                serde_json::from_slice(&fs::read(&path)?)
                    .with_context(|| format!("parse {name}"))?,
            );
        }
    }
    Ok(docs)
}
pub fn verify(docs: &Documents) -> Result<()> {
    let doc = |key: &str| {
        docs.get(key)
            .ok_or_else(|| anyhow!("missing document {key}"))
    };
    let baseline = index(doc("BASELINE")?)?;
    let status = index(doc("STATUS")?)?;
    let contracts = index(doc("CONTRACTS")?)?;
    ensure!(baseline.len() == 13, "expected 13 baseline repositories");
    ensure!(
        keys(&baseline) == keys(&status) && keys(&baseline) == keys(&contracts),
        "baseline/status/contracts mismatch"
    );
    let admission = index(doc("ADMISSION")?)?;
    let delivery = index(doc("DELIVERY")?)?;
    ensure!(
        keys(&admission) == keys(&delivery),
        "admission/delivery mismatch"
    );
    ensure!(
        keys(&admission).is_subset(&keys(&baseline)),
        "unknown admitted repository"
    );
    for (repo, observation) in &admission {
        let publication = delivery[repo];
        let receipt = field(publication, "publication_receipt")?;
        equal(observation, "head", publication, "head")?;
        equal(observation, "issue", publication, "issue")?;
        identity(
            receipt,
            repo,
            field(publication, "issue")?,
            field(publication, "pull_request")?,
            field(publication, "head")?,
            "expected_head_sha",
        )?;
        yes(receipt, "authenticated")?;
        yes(observation, "read_only")?;
        yes(observation, "operational_authority")?;
        findings(observation)?;
        equal(contracts[repo], "source_revision", observation, "head")?;
    }
    let package = doc("PACKAGE")?;
    ensure!(
        field(package, "lifecycle")?
            == &serde_json::json!(["SIP", "STP", "SPP", "VPP", "SRP", "SOR"]),
        "lifecycle order mismatch"
    );
    for r in admission.values() {
        equal(r, "package_artifact", package, "artifact_id")?;
        equal(r, "registry_version", package, "registry_version")?;
    }
    if let Some(retained_doc) = docs.get("RETAINED_ACCEPTANCE") {
        let retained = index(retained_doc)?;
        ensure!(
            keys(&retained).is_disjoint(&keys(&admission)),
            "duplicate retained/admitted repository"
        );
        ensure!(
            keys(&retained)
                .union(&keys(&admission))
                .copied()
                .collect::<BTreeSet<_>>()
                == keys(&baseline),
            "incomplete retained/admitted coverage"
        );
        let mut bindings = BTreeMap::new();
        for r in rows(doc("PACKAGE_BINDINGS")?)? {
            let key = (
                string(r, "repository")?,
                field(r, "issue")?
                    .as_u64()
                    .context("invalid binding issue")?,
            );
            ensure!(
                bindings.insert(key, r).is_none(),
                "duplicate package binding"
            );
        }
        for (repo, row) in retained {
            let issue = field(row, "representative_issue")?;
            let head = field(row, "head")?;
            let pr = field(row, "pull_request")?;
            let provenance = bindings
                .get(&(
                    repo,
                    issue.as_u64().context("invalid representative issue")?,
                ))
                .context("missing package binding")?;
            ensure!(
                provenance.get("pending") == Some(&Value::Null),
                "pending binding or missing pending state"
            );
            let artifacts = field(provenance, "packages")?
                .as_array()
                .context("invalid packages")?
                .iter()
                .map(|r| string(r, "artifact_id"))
                .collect::<Result<BTreeSet<_>>>()?;
            let preparation = field(row, "preparation")?;
            ensure!(
                string(preparation, "phase")? == "ready",
                "preparation not ready"
            );
            ensure!(
                artifacts.contains(string(preparation, "package_artifact")?),
                "unbound preparation package"
            );
            for stage in ["binding", "proof", "review", "publication"] {
                let evidence = field(row, stage)?;
                ensure!(
                    string(evidence, "outcome")? == "success",
                    "stage not successful: {stage}"
                );
                yes(evidence, "object_digest_verified")?;
                ensure!(
                    artifacts.contains(string(evidence, "package_artifact")?),
                    "unbound stage package"
                );
                if stage != "binding" {
                    ensure!(field(evidence, "head")? == head, "stage head mismatch");
                }
            }
            let receipt = field(field(row, "publication")?, "receipt")?;
            yes(receipt, "authenticated")?;
            identity(receipt, repo, issue, pr, head, "expected_head_sha")?;
            let terminal = field(field(row, "terminal")?, "delivery")?;
            ensure!(
                string(terminal, "status")? == "delivered",
                "terminal not delivered"
            );
            identity(terminal, repo, issue, pr, head, "head_sha")?;
            let current = field(row, "current_admission")?;
            ensure!(
                string(current, "status")? == "completed",
                "current admission incomplete"
            );
            yes(current, "read_only")?;
            yes(current, "operational_authority")?;
            ensure!(
                field(current, "performed_mutation")? == false,
                "current admission mutated"
            );
            findings(current)?;
            equal(current, "package_artifact", package, "artifact_id")?;
            equal(current, "registry_version", package, "registry_version")?;
            ensure!(
                matches!(
                    string(field(row, "cleanup")?, "status")?,
                    "retained_removed" | "deferred_by_operator"
                ),
                "invalid cleanup status"
            );
        }
    }
    let complete = field(doc("STATUS")?, "all_requirements_satisfied")?
        .as_bool()
        .context("invalid aggregate acceptance")?;
    if complete {
        ensure!(
            docs.contains_key("RETAINED_ACCEPTANCE"),
            "missing retained acceptance"
        );
        for r in status.values() {
            ensure!(
                string(r, "acceptance")? == "complete",
                "incomplete repository hidden by aggregate success"
            );
        }
        for (repo, row) in &delivery {
            let terminal = row
                .get("terminal_delivery")
                .or_else(|| row.get("terminal_readback").and_then(|r| r.get("delivery")))
                .context("missing terminal delivery")?;
            ensure!(string(terminal, "status")? == "delivered", "not delivered");
            ensure!(
                string(terminal, "repository")? == *repo,
                "terminal repository mismatch"
            );
            equal(terminal, "pull_request", row, "pull_request")?;
            equal(terminal, "head_sha", row, "head")?;
            if terminal.get("issue").is_some() {
                equal(terminal, "issue", row, "issue")?;
            }
            ensure!(
                string(terminal, "issue_closure")? == "closed",
                "issue not closed"
            );
        }
    }
    // Validate field types even for incomplete reports; no truthy string acceptance.
    for r in status.values() {
        string(r, "acceptance")?;
    }
    Ok(())
}
