//! Cards for the native local owner.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use serde_json::Value;

use super::filesystem::{
    append_directory_rollback_finding, atomic_write, atomic_write_json, io_finding,
};
use super::lifecycle::inspect_lifecycle_issue_root;
use super::results::operational_result;
use super::storage::{lifecycle_digest, persist_index, read_index_value};
use super::transactions::{
    begin_local_transaction, commit_pending_local_transaction, local_request_digest,
    prepare_local_transaction_stage,
};
use super::{
    finding, DoctorFinding, LocalMutationJournal, LocalPreparationRequest, OperationalLocalContext,
    OperationalLocalResult, PlanStatus, PromptRegistry, REQUIRED_CARD_KINDS,
};

pub(super) fn initial_card_values(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    kind: &str,
) -> Value {
    serde_json::json!({
        "schema": "csdlc.v3.card_values.v1",
        "issue": request.issue,
        "issue_padded": format!("{:04}", request.issue),
        "issue_url": format!("https://github.com/{}/issues/{}", request.repository, request.issue),
        "title": request.title,
        "branch": request.branch,
        "version": registry.version,
        "card": kind,
        "card_status": "ready",
        "repository": request.repository,
        "worktree": request.worktree
    })
}
pub(super) fn edit_operational_cards(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    context: &OperationalLocalContext,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if request.card_updates.is_empty() {
        return Err(vec![finding(
            PlanStatus::Failed,
            "card_updates_missing",
            "edit requires at least one typed card update",
        )]);
    }
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    let phase = observed.phase.as_deref().unwrap_or("ready");
    let request_digest = local_request_digest(request)?;
    let stage = prepare_local_transaction_stage(context, request.issue, "edit", &request_digest)?;
    let staged = (|| {
        for (kind, update) in &request.card_updates {
            if !REQUIRED_CARD_KINDS.contains(&kind.as_str()) || !update.is_object() {
                return Err(vec![finding(
                    PlanStatus::Failed,
                    "card_update_invalid",
                    "card updates must be objects keyed by canonical card kind",
                )]);
            }
            let values_path = stage.join(format!("cards/{kind}.values.json"));
            let mut values: Value = serde_json::from_slice(
                &fs::read(&values_path).map_err(io_finding("card_values_read_failed"))?,
            )
            .map_err(|error| {
                vec![finding(
                    PlanStatus::Failed,
                    "card_values_invalid",
                    &error.to_string(),
                )]
            })?;
            merge_json_object(&mut values, update);
            let template = fs::read_to_string(
                registry
                    .template_paths
                    .get(kind)
                    .expect("registry validated"),
            )
            .map_err(io_finding("template_read_failed"))?;
            atomic_write_json(&values_path, &values)?;
            atomic_write(
                &stage.join(format!("cards/{kind}.md")),
                render_template(&template, &values).as_bytes(),
            )?;
        }
        persist_index(
            &stage,
            request,
            registry,
            phase,
            observed.generation.unwrap_or(1) + 1,
        )
    })();
    let (generation, digest) = match staged {
        Ok(value) => value,
        Err(mut findings) => {
            append_directory_rollback_finding(&stage, &mut findings);
            return Err(findings);
        }
    };
    let result = operational_result(
        "edit",
        request.issue,
        true,
        phase,
        generation,
        digest,
        Some("validate"),
        vec![],
    );
    if let Err(mut findings) = begin_local_transaction(
        context,
        LocalMutationJournal {
            schema: "csdlc.v3.local_mutation_journal.v1".into(),
            issue: request.issue,
            route: "edit".into(),
            request_digest,
            result: result.clone(),
            bind_branch: None,
            bind_worktree: None,
        },
    ) {
        append_directory_rollback_finding(&stage, &mut findings);
        return Err(findings);
    }
    commit_pending_local_transaction(context, request.issue)?;
    Ok(result)
}
pub(super) fn validate_operational_issue(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    let findings = validation_findings(request, registry, issue_root);
    if findings
        .iter()
        .any(|item| item.status != PlanStatus::Passed)
    {
        return Err(findings);
    }
    let observed = inspect_lifecycle_issue_root(issue_root, request.issue, "v3");
    Ok(operational_result(
        "validate",
        request.issue,
        false,
        observed.phase.as_deref().unwrap_or("unknown"),
        observed.generation.unwrap_or(0),
        observed.digest.unwrap_or_default(),
        Some(if observed.phase.as_deref() == Some("bound") {
            "shepherd"
        } else {
            "bind"
        }),
        findings,
    ))
}
pub(super) fn validation_findings(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Vec<DoctorFinding> {
    let mut findings = Vec::new();
    let index = match read_index_value(issue_root) {
        Ok(index) => index,
        Err(mut errors) => {
            return {
                findings.append(&mut errors);
                findings
            }
        }
    };
    let recorded_digest = index.get("digest").and_then(Value::as_str).unwrap_or("");
    match lifecycle_digest(issue_root, &index) {
        Ok(actual) if actual == recorded_digest => findings.push(finding(
            PlanStatus::Passed,
            "lifecycle_digest_valid",
            "lifecycle digest matches canonical state",
        )),
        Ok(_) => findings.push(finding(
            PlanStatus::Blocked,
            "lifecycle_digest_mismatch",
            "lifecycle digest does not match canonical state",
        )),
        Err(mut errors) => findings.append(&mut errors),
    }
    for kind in REQUIRED_CARD_KINDS {
        let values_path = issue_root.join(format!("cards/{kind}.values.json"));
        let rendered_path = issue_root.join(format!("cards/{kind}.md"));
        let values: Value = match fs::read(&values_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        {
            Some(value) if value.is_object() => value,
            _ => {
                findings.push(finding(
                    PlanStatus::Blocked,
                    "card_values_invalid",
                    &format!("{kind} values are missing or invalid"),
                ));
                continue;
            }
        };
        let template = match registry
            .template_paths
            .get(kind)
            .and_then(|path| fs::read_to_string(path).ok())
        {
            Some(template) => template,
            None => {
                findings.push(finding(
                    PlanStatus::Blocked,
                    "template_read_failed",
                    &format!("{kind} template is unavailable"),
                ));
                continue;
            }
        };
        let expected = render_template(&template, &values);
        let actual = fs::read_to_string(&rendered_path).unwrap_or_default();
        if expected != actual {
            findings.push(finding(
                PlanStatus::Blocked,
                "rendered_card_drift",
                &format!("{kind} rendered card differs from typed values"),
            ));
            continue;
        }
        if !structure_valid(registry, kind, &actual) {
            findings.push(finding(
                PlanStatus::Blocked,
                "card_structure_invalid",
                &format!("{kind} rendered card violates its structure schema"),
            ));
        }
    }
    if findings.len() == 1 && findings[0].status == PlanStatus::Passed {
        findings.push(finding(
            PlanStatus::Passed,
            "six_card_validation_passed",
            &format!(
                "issue #{} has valid values, renders, structures, and digest",
                request.issue
            ),
        ));
    }
    findings
}
pub(super) fn structure_valid(registry: &PromptRegistry, kind: &str, markdown: &str) -> bool {
    let Some(template_path) = registry.template_paths.get(kind) else {
        return false;
    };
    let template_path = PathBuf::from(template_path);
    let schema_path = template_path
        .parent()
        .map(|root| root.join(format!("schemas/{kind}.structure.json")));
    let Some(schema_path) = schema_path else {
        return false;
    };
    let Ok(template) = fs::read_to_string(&template_path) else {
        return false;
    };
    let Ok(bytes) = fs::read(schema_path) else {
        return false;
    };
    let Ok(schema) = serde_json::from_slice::<Value>(&bytes) else {
        return false;
    };
    let Some(schema_template_path) = schema.get("template_path").and_then(Value::as_str) else {
        return false;
    };
    if schema.get("schema").and_then(Value::as_str) != Some("adl.csdlc.prompt_card_structure.v1")
        || schema.get("template_set").and_then(Value::as_str) != Some(registry.version.as_str())
        || schema.get("card_kind").and_then(Value::as_str) != Some(kind)
        || !(template_path == Path::new(schema_template_path)
            || template_path.ends_with(schema_template_path))
    {
        return false;
    }
    let template_lines = template.lines().map(str::trim).collect::<BTreeSet<_>>();
    let rendered_lines = markdown.lines().map(str::trim).collect::<BTreeSet<_>>();
    let scaffold_valid = schema
        .get("scaffold_lines")
        .and_then(Value::as_array)
        .is_some_and(|lines| {
            lines
                .iter()
                .filter_map(Value::as_str)
                .filter(|line| template_lines.contains(line.trim()))
                .all(|line| rendered_lines.contains(line.trim()))
        });
    let headings_valid = schema
        .get("headings")
        .and_then(Value::as_array)
        .is_some_and(|headings| {
            headings.iter().all(|heading| {
                let Some(level) = heading.get("level").and_then(Value::as_u64) else {
                    return false;
                };
                let text = heading.get("text").and_then(Value::as_str);
                let prefix = "#".repeat(level as usize);
                if text.is_none_or(str::is_empty) {
                    markdown.lines().any(|line| {
                        line.starts_with(&format!("{prefix} "))
                            && !line.starts_with(&format!("{prefix}#"))
                    })
                } else {
                    rendered_lines.contains(format!("{prefix} {}", text.unwrap()).as_str())
                }
            })
        });
    let locked_lines_valid = schema
        .get("locked_lines")
        .and_then(Value::as_array)
        .is_some_and(|lines| {
            lines.iter().all(|line| {
                line.get("text")
                    .and_then(Value::as_str)
                    .is_some_and(|text| rendered_lines.contains(text.trim()))
            })
        });
    scaffold_valid && headings_valid && locked_lines_valid
}
pub fn render_semantic_card_projection(
    registry: &PromptRegistry,
    kind: &str,
    values: &Value,
) -> Result<String, String> {
    if !REQUIRED_CARD_KINDS.contains(&kind) || !values.is_object() {
        return Err("semantic projection card input is invalid".into());
    }
    let template_path = registry
        .template_paths
        .get(kind)
        .ok_or_else(|| "semantic projection template is missing".to_string())?;
    let template = fs::read_to_string(template_path)
        .map_err(|_| "semantic projection template is unreadable".to_string())?;
    let rendered = render_template(&template, values);
    if !structure_valid(registry, kind, &rendered) {
        return Err("semantic projection violates the active structure schema".into());
    }
    Ok(rendered)
}
pub(super) fn render_template(template: &str, values: &Value) -> String {
    let mut output = template.to_owned();
    if let Some(object) = values.as_object() {
        for (key, value) in object {
            let rendered = value
                .as_str()
                .map(str::to_owned)
                .unwrap_or_else(|| value.to_string());
            output = output.replace(&format!("<{key}>"), &rendered);
        }
    }
    output
}
pub(super) fn merge_json_object(target: &mut Value, update: &Value) {
    if let (Some(target), Some(update)) = (target.as_object_mut(), update.as_object()) {
        for (key, value) in update {
            target.insert(key.clone(), value.clone());
        }
    }
}
