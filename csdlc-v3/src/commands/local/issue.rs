//! Issue for the native local owner.

use std::{fs, path::Path};

use serde_json::Value;

use super::cards::{initial_card_values, merge_json_object, render_template};
use super::filesystem::{
    append_directory_rollback_finding, atomic_write, atomic_write_json, io_finding,
    next_local_temp_sequence,
};
use super::results::operational_result;
use super::storage::persist_index;
use super::{
    finding, DoctorFinding, LocalPreparationRequest, OperationalLocalResult, PlanStatus,
    PromptRegistry, REQUIRED_CARD_KINDS,
};

pub(super) fn initialize_operational_issue(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    initialize_operational_issue_with_plan(request, registry, issue_root, None)
}
pub(super) fn initialize_operational_issue_with_plan(
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
    issue_root: &Path,
    intent_plan: Option<&Value>,
) -> Result<OperationalLocalResult, Vec<DoctorFinding>> {
    if issue_root.exists() {
        return Err(vec![finding(
            PlanStatus::Blocked,
            "issue_already_initialized",
            "native issue initialization is create-only",
        )]);
    }
    let parent = issue_root.parent().ok_or_else(|| {
        vec![finding(
            PlanStatus::Failed,
            "issue_state_parent_missing",
            "issue state path must have a parent",
        )]
    })?;
    fs::create_dir_all(parent).map_err(io_finding("issue_state_parent_create_failed"))?;
    let stage = parent.join(format!(
        ".issue-{}.init-{}-{}",
        request.issue,
        std::process::id(),
        next_local_temp_sequence()
    ));
    fs::create_dir(&stage).map_err(io_finding("issue_stage_create_failed"))?;
    let staged_result = (|| {
        let cards_root = stage.join("cards");
        fs::create_dir(&cards_root).map_err(io_finding("issue_state_create_failed"))?;
        for kind in REQUIRED_CARD_KINDS {
            let mut values = initial_card_values(request, registry, kind);
            if intent_plan.is_some() {
                if let Some(update) = request.card_updates.get(kind) {
                    merge_json_object(&mut values, update);
                }
            }
            let template_path = registry
                .template_paths
                .get(kind)
                .expect("registry validated");
            let template =
                fs::read_to_string(template_path).map_err(io_finding("template_read_failed"))?;
            let rendered = render_template(&template, &values);
            atomic_write_json(&cards_root.join(format!("{kind}.values.json")), &values)?;
            atomic_write(&cards_root.join(format!("{kind}.md")), rendered.as_bytes())?;
        }
        if let Some(plan) = intent_plan {
            atomic_write_json(&stage.join("intent-plan.json"), plan)?;
        }
        persist_index(&stage, request, registry, "ready", 1)
    })();
    let (generation, digest) = match staged_result {
        Ok(result) => result,
        Err(mut findings) => {
            append_directory_rollback_finding(&stage, &mut findings);
            return Err(findings);
        }
    };
    if let Err(error) = fs::rename(&stage, issue_root) {
        let mut findings = io_finding("issue_state_commit_failed")(error);
        append_directory_rollback_finding(&stage, &mut findings);
        return Err(findings);
    }
    Ok(operational_result(
        "issue",
        request.issue,
        true,
        "ready",
        generation,
        digest,
        Some("bind"),
        vec![],
    ))
}
