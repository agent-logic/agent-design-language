//! Offline data validation only. References do not authenticate native authority.
use super::*;
use anyhow::ensure;
use serde::de::{self, Deserialize, Deserializer, MapAccess, SeqAccess, Visitor};
use std::fmt;

struct Strict(Value);
impl<'de> Deserialize<'de> for Strict {
    fn deserialize<D: Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        struct V;
        impl<'de> Visitor<'de> for V {
            type Value = Strict;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("integer-only JSON with unique ASCII object keys")
            }
            fn visit_bool<E: de::Error>(self, v: bool) -> std::result::Result<Strict, E> {
                Ok(Strict(Value::Bool(v)))
            }
            fn visit_unit<E: de::Error>(self) -> std::result::Result<Strict, E> {
                Ok(Strict(Value::Null))
            }
            fn visit_i64<E: de::Error>(self, v: i64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_u64<E: de::Error>(self, v: u64) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_str<E: de::Error>(self, v: &str) -> std::result::Result<Strict, E> {
                Ok(Strict(v.into()))
            }
            fn visit_seq<A: SeqAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut v = vec![];
                while let Some(x) = a.next_element::<Strict>()? {
                    v.push(x.0)
                }
                Ok(Strict(Value::Array(v)))
            }
            fn visit_map<A: MapAccess<'de>>(
                self,
                mut a: A,
            ) -> std::result::Result<Strict, A::Error> {
                let mut v = serde_json::Map::new();
                while let Some(k) = a.next_key::<String>()? {
                    if !k.is_ascii() || v.contains_key(&k) {
                        return Err(de::Error::custom("duplicate or non-ASCII key"));
                    }
                    v.insert(k, a.next_value::<Strict>()?.0);
                }
                Ok(Strict(Value::Object(v)))
            }
        }
        d.deserialize_any(V)
    }
}
pub fn parse(raw: &[u8]) -> Result<Value> {
    ensure!(raw.len() <= 2 * 1024 * 1024, "packet too large");
    Ok(serde_json::from_slice::<Strict>(raw)?.0)
}
pub fn canonical(v: &Value) -> Result<Vec<u8>> {
    fn emit(v: &Value) -> Result<String> {
        Ok(match v {
            Value::Object(o) => {
                let sorted: BTreeMap<_, _> = o.iter().collect();
                format!(
                    "{{{}}}",
                    sorted
                        .into_iter()
                        .map(|(k, v)| Ok(format!("{}:{}", serde_json::to_string(k)?, emit(v)?)))
                        .collect::<Result<Vec<_>>>()?
                        .join(",")
                )
            }
            Value::Array(a) => format!(
                "[{}]",
                a.iter().map(emit).collect::<Result<Vec<_>>>()?.join(",")
            ),
            _ => serde_json::to_string(v)?,
        })
    }
    let bytes = emit(v)?.into_bytes();
    parse(&bytes)?;
    Ok(bytes)
}
pub fn digest(v: &Value) -> Result<String> {
    Ok(archive::sha(&canonical(v)?))
}
const CONTEXT: &[&str] = &[
    "resident_identity_ref",
    "body_ref",
    "runtime_incarnation",
    "objective_ref",
    "policy_ref",
    "memory_head_ref",
    "continuity_head_ref",
    "archive_head_ref",
];
fn references<'a>(v: &'a Value, out: &mut Vec<&'a Value>) {
    if let Some(o) = v.as_object() {
        if o.contains_key("artifact_id") && o.contains_key("sha256") {
            out.push(v);
            return;
        }
        for child in o.values() {
            references(child, out);
        }
    } else if let Some(a) = v.as_array() {
        for child in a {
            references(child, out);
        }
    }
}
fn reject_remote_schema_refs(v: &Value) -> Result<()> {
    if let Some(o) = v.as_object() {
        for (k, c) in o {
            if k == "$ref" {
                ensure!(
                    c.as_str().is_some_and(|s| s.starts_with('#')),
                    "external schema resolution refused"
                );
            }
            reject_remote_schema_refs(c)?;
        }
    }
    if let Some(a) = v.as_array() {
        for c in a {
            reject_remote_schema_refs(c)?;
        }
    }
    Ok(())
}
pub fn validate(schema: &Value, packet: &Value, expected: &Value) -> Result<Value> {
    canonical(packet)?;
    canonical(expected)?;
    reject_remote_schema_refs(schema)?;
    let compiled = jsonschema::JSONSchema::options()
        .with_draft(jsonschema::Draft::Draft7)
        .compile(schema)
        .map_err(|e| anyhow::anyhow!("invalid contract schema: {e}"))?;
    if let Err(errors) = compiled.validate(packet) {
        bail!(
            "schema refusal: {}",
            errors
                .take(3)
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join("; ")
        );
    }
    let c = &packet["cycle"];
    ensure!(
        expected
            .as_object()
            .is_some_and(|o| o.len() == CONTEXT.len()),
        "exact trusted current context required"
    );
    for field in CONTEXT {
        ensure!(
            expected.get(*field) == c.get(*field) && expected.get(*field).is_some(),
            "stale context: {field}"
        );
    }
    let mut known = BTreeMap::new();
    for a in packet["artifacts"]
        .as_array()
        .context("artifacts required")?
    {
        let id = a["artifact_id"].as_str().context("artifact identity")?;
        ensure!(known.insert(id,json!({"artifact_id":id,"owner":a["owner"],"schema_version":a["schema_version"],"sha256":digest(a)?})).is_none(),"duplicate artifact identity");
    }
    for r in packet["external_refs"]
        .as_array()
        .context("external_refs required")?
    {
        let id = r["artifact_id"].as_str().context("external identity")?;
        ensure!(
            known.insert(id, r.clone()).is_none(),
            "duplicate/aliased external identity"
        );
    }
    let mut refs = vec![];
    references(packet, &mut refs);
    for r in refs {
        ensure!(
            known.get(r["artifact_id"].as_str().context("reference identity")?) == Some(r),
            "missing or stale artifact reference"
        );
    }
    let b = &c["budget"];
    for (k, limit) in b["ceilings"].as_object().context("budget ceilings")? {
        ensure!(
            b["consumed"][k].as_u64().context("budget consumed")?
                <= limit.as_u64().context("budget ceiling")?,
            "budget exceeded: {k}"
        );
    }
    for a in packet["artifacts"].as_array().context("artifacts")? {
        let p = &a["payload"];
        if a["artifact_type"] == "belief_update" && p["mode"] == "quantitative" {
            let posterior = &p["quantitative"]["posterior"];
            ensure!(
                posterior["numerator"]
                    .as_u64()
                    .context("posterior numerator")?
                    <= posterior["denominator"]
                        .as_u64()
                        .context("posterior denominator")?,
                "posterior outside probability range"
            );
        }
        if a["artifact_type"] == "disposition" {
            ensure!(
                !(c["loop_kind"] == "task" && p["decision"] == "recommend_activation")
                    && !(c["loop_kind"] == "improvement"
                        && p["decision"] == "recommend_task_action"),
                "disposition loop mismatch"
            );
        }
    }
    let mut previous: Option<&Value> = None;
    let mut last_remaining = b["ceilings"].clone();
    let mut recovery_blocked = false;
    let mut operations = std::collections::BTreeSet::new();
    for t in c["transitions"].as_array().context("transitions")? {
        if let Some(prev) = previous {
            ensure!(prev == &t["source_state"], "transition chain discontinuity");
        } else {
            ensure!(
                t["source_state"] == "initial",
                "partial history lacks admission provenance"
            );
        }
        ensure!(
            t["correlation_ids"]["cycle_id"] == c["cycle_id"],
            "transition cycle mismatch"
        );
        ensure!(
            operations.insert(t["operation_id"].as_str().context("operation identity")?),
            "duplicate operation identity"
        );
        previous = Some(&t["target_state"]);
        let name = t["transition"].as_str().context("transition identity")?;
        ensure!(
            !recovery_blocked || name == "retain_lineage",
            "unknown effects block further operations"
        );
        for (k, remaining) in t["remaining_budget"]
            .as_object()
            .context("remaining budget")?
        {
            ensure!(
                remaining.as_u64().context("remaining amount")?
                    <= last_remaining[k].as_u64().context("prior remaining")?,
                "budget increased: {k}"
            );
        }
        last_remaining = t["remaining_budget"].clone();
        {
            let (source, target) = match name {
                "observe" => ("initial", "observed"),
                "hypothesize" => ("observed", "hypothesized"),
                "propose_candidate" => ("hypothesized", "candidate_proposed"),
                "admit_experiment" => ("candidate_proposed", "experiment_admitted"),
                "execute_experiment" => ("experiment_admitted", "experiment_executed"),
                "measure" => ("experiment_executed", "measured"),
                "update_belief" => ("measured", "belief_updated"),
                "disposition" => ("belief_updated", "disposed"),
                "admit_task_action" => ("disposed", "task_admitted"),
                "execute_task_action" => ("task_admitted", "task_executed"),
                "authorize_activation" => ("disposed", "activation_authorized"),
                "reconcile_activation" => ("activation_authorized", "activation_reconciled"),
                "retain_lineage" => (
                    t["source_state"].as_str().context("lineage source")?,
                    "lineage_retained",
                ),
                _ => bail!("unsupported transition"),
            };
            ensure!(
                t["source_state"] == source,
                "transition source state mismatch"
            );
            if t["outcome"] == "completed" {
                ensure!(
                    t["target_state"] == target,
                    "transition target state mismatch"
                );
            } else if t["outcome"] == "interrupted" || t["outcome"] == "recovery_required" {
                ensure!(
                    t["target_state"] == "recovery_required",
                    "unknown effects must stop for recovery"
                );
            } else {
                ensure!(
                    t["target_state"] == "stopped",
                    "non-completed transition cannot advance admission"
                );
            }
            if name == "retain_lineage" {
                ensure!(
                    matches!(
                        source,
                        "disposed"
                            | "task_executed"
                            | "activation_reconciled"
                            | "stopped"
                            | "recovery_required"
                    ),
                    "lineage before disposition"
                );
            }
        }
        if !matches!(
            name,
            "execute_experiment" | "execute_task_action" | "reconcile_activation"
        ) {
            ensure!(
                t["effects"]
                    .as_array()
                    .context("effects")?
                    .iter()
                    .all(|e| e == "none" || e == "artifact_retention"),
                "transition effects outside owner boundary"
            );
        }
        if c["loop_kind"] == "task" {
            ensure!(
                !matches!(name, "authorize_activation" | "reconcile_activation"),
                "task cannot activate body"
            );
        }
        if c["loop_kind"] == "improvement" {
            ensure!(
                !matches!(name, "admit_task_action" | "execute_task_action"),
                "improvement cannot execute task action"
            );
        }
        if c["execution_mode"] == "replay" {
            ensure!(
                t["effects"]
                    .as_array()
                    .context("effects")?
                    .iter()
                    .all(|e| e == "none" || e == "artifact_retention"),
                "replay effects refused"
            );
        }
        if matches!(
            name,
            "execute_experiment"
                | "execute_task_action"
                | "authorize_activation"
                | "reconcile_activation"
        ) && t["outcome"] == "completed"
        {
            ensure!(
                !t["authority_ref"].is_null(),
                "authority reference required (authenticity still external)"
            );
        }
        if t["outcome"] == "recovery_required" || t["outcome"] == "interrupted" {
            recovery_blocked = true;
            ensure!(
                !t["recovery_owner_ref"].is_null(),
                "unknown effects require original recovery owner"
            );
        }
    }
    if !c["transitions"].as_array().unwrap().is_empty() {
        for (k, remaining) in last_remaining.as_object().unwrap() {
            ensure!(
                remaining.as_u64().unwrap()
                    == b["ceilings"][k].as_u64().unwrap() - b["consumed"][k].as_u64().unwrap(),
                "final remaining/consumed mismatch"
            );
        }
    }
    Ok(
        json!({"schema":"adl.ghb.validation.v1","packet_sha256":digest(packet)?,"cycle_id":c["cycle_id"],"contract_version":1,"validated":true,"authority_authenticated":false,"execution_performed":false}),
    )
}
