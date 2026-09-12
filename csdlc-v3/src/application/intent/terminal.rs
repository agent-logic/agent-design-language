//! Intent adapters for authenticated terminal delivery and exact preview cleanup.
use super::{context::git, Context, IntentRequest};
use crate::{
    adapters::{
        CommandInvocation, EnvironmentCredentialResolver, ProcessAdapter, ProcessStatus,
        RealProcessAdapter,
    },
    commands::{local::operational_state_root, remote::intent::publication_target, terminal::*},
};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

fn native_request(context: &Context) -> Result<TerminalRouteRequest, String> {
    serde_json::from_value(json!({"repository":context.repository,"issue":context.issue,
        "pull_request":publication_target(&context.root,&context.repository,context.issue,&context.branch,&context.head).map_err(|finding|finding.code)?,
        "expected_head_sha":context.head,"mode":"closing","credential_names":["GITHUB_TOKEN"]}))
        .map_err(|_|"intent_terminal_request_invalid".into())
}
fn state_root(context: &Context) -> Result<PathBuf, String> {
    operational_state_root(&context.primary)
        .map_err(|_| "intent_terminal_state_root_invalid".into())
}
fn file_digest(path: &std::path::Path) -> Result<Option<String>, String> {
    match fs::read(path) {
        Ok(bytes) => Ok(Some(blake3::hash(&bytes).to_hex().to_string())),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(_) => Err("intent_terminal_state_unreadable".into()),
    }
}

pub fn run(context: &Context, request: &IntentRequest) -> Result<Value, String> {
    if request.command != "finish" && !request.content.is_null() {
        return Err("intent_terminal_unexpected_content".into());
    }
    context.fresh()?;
    let output_root = state_root(context)?;
    let receipt_path =
        output_root.join(format!("evidence/{}/terminal-receipt.json", context.issue));
    if request.command == "clean" && context.index.is_null() {
        let bytes = fs::read(&receipt_path).map_err(|_| "intent_terminal_receipt_required")?;
        let receipt: DurableTerminalReceipt =
            serde_json::from_slice(&bytes).map_err(|_| "intent_terminal_receipt_invalid")?;
        let state_path = output_root.join(format!("v3/issues/{}/terminal.json", context.issue));
        if receipt.schema != "csdlc.v3.terminal_receipt.v1"
            || receipt.repository != context.repository
            || receipt.issue != context.issue
            || receipt.disposition != "closed_out"
            || receipt.head_sha.len() != 40
            || !receipt
                .head_sha
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
            || receipt.state_digest.is_none()
            || receipt.state_digest != file_digest(&state_path)?
        {
            return Err("intent_terminal_receipt_mismatch".into());
        }
        let topology = git(&context.primary, &["worktree", "list", "--porcelain"])?;
        for worktree in topology
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
        {
            if PathBuf::from(worktree)
                .join(format!(".csdlc/issues/{}/index.json", context.issue))
                .exists()
            {
                return Err("intent_cleanup_registration_ambiguous".into());
            }
        }
        let binding_path = output_root.join(format!("bindings/{}.json", context.issue));
        let binding =
            super::read_json(&binding_path).map_err(|_| "intent_cleanup_binding_required")?;
        if binding["schema"] != "csdlc.v3.binding.v1" || binding["issue"] != context.issue {
            return Err("intent_cleanup_binding_invalid".into());
        }
        let bound_path = binding["worktree"]
            .as_str()
            .ok_or("intent_cleanup_binding_invalid")?;
        if topology
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
            .any(|path| path == bound_path)
            || PathBuf::from(bound_path).exists()
        {
            return Err("cleanup_archive_recovery_required".into());
        }
        context.fresh()?;
        return Ok(
            json!({"status":"expected_noop","read_only":true,"operational_authority":false,"performed_mutation":false,
            "issue":context.issue,"terminal_head":receipt.head_sha,"reason":"native_terminal_receipt_retained_and_no_registered_issue_binding"}),
        );
    }
    let mut native = if request.command == "finish" && !request.content.is_null() {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Disposition {
            disposition: NoPrDisposition,
            operator: String,
            rationale: String,
            evidence_refs: Vec<String>,
        }
        let approved: Disposition = serde_json::from_value(request.content.clone())
            .map_err(|_| "intent_finish_disposition_invalid")?;
        let invocation = CommandInvocation::new(
            "github-api-read-only",
            [
                "issue".to_string(),
                context.repository.clone(),
                context.issue.to_string(),
            ],
        )
        .map_err(|_| "intent_terminal_observation_invalid")?
        .with_child_credential("GITHUB_TOKEN")
        .map_err(|_| "intent_terminal_credential_invalid")?;
        let observed = RealProcessAdapter::new(EnvironmentCredentialResolver).run(invocation);
        if observed.status != ProcessStatus::Exit(0) || observed.truncated {
            return Err("intent_terminal_observation_failed".into());
        }
        let issue: Value = serde_json::from_str(&observed.stdout)
            .map_err(|_| "intent_terminal_observation_invalid")?;
        if issue["number"] != context.issue
            || issue["state"] != "closed"
            || !issue["pull_request"].is_null()
        {
            return Err("intent_terminal_closed_issue_required".into());
        }
        serde_json::from_value(json!({"repository":context.repository,"issue":context.issue,"expected_head_sha":context.head,"credential_names":["GITHUB_TOKEN"],"no_pr_closeout":{
            "disposition":approved.disposition,"operator":approved.operator,"rationale":approved.rationale,"evidence_refs":approved.evidence_refs,
            "expected_issue_updated_at":issue["updated_at"],"expected_issue_closed_at":issue["closed_at"]
        }})).map_err(|_|"intent_terminal_disposition_observation_invalid")?
    } else if request.command == "clean" {
        let receipt: DurableTerminalReceipt = serde_json::from_slice(
            &fs::read(&receipt_path).map_err(|_| "intent_terminal_receipt_required")?,
        )
        .map_err(|_| "intent_terminal_receipt_invalid")?;
        if receipt.repository != context.repository
            || receipt.issue != context.issue
            || receipt.head_sha != context.head
        {
            return Err("intent_terminal_receipt_mismatch".into());
        }
        serde_json::from_value(json!({"repository":context.repository,"issue":context.issue,"expected_head_sha":context.head,"pull_request":receipt.pull_request,
            "mode":if receipt.no_pr_closeout.is_some(){Value::Null}else{json!("closing")},"no_pr_closeout":receipt.no_pr_closeout,"credential_names":["GITHUB_TOKEN"]})).map_err(|_|"intent_terminal_request_invalid")?
    } else {
        native_request(context)?
    };
    match request.command.as_str() {
        "finish" => {
            if request.execute
                || request
                    .preview
                    .as_ref()
                    .is_some_and(|preview| preview != "plan")
            {
                return Err("intent_finish_arguments_invalid".into());
            }
            let state_path = output_root.join(format!("v3/issues/{}/terminal.json", context.issue));
            if request.preview.is_none() {
                native.terminal_state = Some(TerminalStateWriteRequest {
                    repository_root: context.primary.clone(),
                    state_path: state_path.clone(),
                    receipt_path: receipt_path.clone(),
                    expected_state_digest: file_digest(&state_path)?,
                });
            }
            context.fresh()?;
            let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
            let plan = prepare_terminal_finish_with_github_observation(&native, &mut process)
                .map_err(|finding| finding.code)?;
            let success = plan.status == TerminalRouteStatus::Ready;
            let preview = request.preview.is_some();
            Ok(
                json!({"status":if success {if preview {"ready"} else {"completed"}} else {"blocked"},
                "read_only":preview,"operational_authority":plan.operational_authority,
                "performed_mutation":if preview {Some(false)} else if success {Some(true)} else {None},
                "effects_unknown":!preview&&!success,"result":plan}),
            )
        }
        "clean" => {
            if context.root == context.primary || context.index["phase"] != "bound" {
                return Err("intent_cleanup_bound_target_required".into());
            }
            if !request.execute
                && request
                    .preview
                    .as_ref()
                    .is_some_and(|preview| preview != "plan")
            {
                return Err("intent_cleanup_preview_invalid".into());
            }
            let policy: Value = serde_json::from_slice(
                &fs::read(context.primary.join(".adl/worktree-policy.json"))
                    .map_err(|_| "intent_worktree_policy_unreadable")?,
            )
            .map_err(|_| "intent_worktree_policy_invalid")?;
            let parent = policy["required_parent"]
                .as_str()
                .ok_or("intent_worktree_policy_invalid")?;
            let receipt_digest =
                file_digest(&receipt_path)?.ok_or("intent_terminal_receipt_required")?;
            native.cleanup = Some(CleanupRouteRequest {
                approved_parent: PathBuf::from(parent),
                repository_root: context.primary.clone(),
                candidate_path: context.root.clone(),
                remove: false,
                terminal_receipt: true,
                terminal_receipt_path: Some(receipt_path.to_string_lossy().into_owned()),
                terminal_receipt_digest: Some(receipt_digest.clone()),
                preview_receipt_digest: None,
            });
            let preview = prepare_intent_cleanup(&native).map_err(|finding| finding.code)?;
            let native_digest = match &preview.cleanup {
                Some(CleanupDecision::Removable { receipt_digest, .. }) => {
                    Some(receipt_digest.clone())
                }
                _ => None,
            };
            let topology = git(&context.primary, &["worktree", "list", "--porcelain"])?;
            let packet = json!({"schema":"csdlc.v3.intent_cleanup_preview.v1","snapshot":request.snapshot,
                "terminal_receipt_digest":receipt_digest,"native_preview_digest":native_digest,
                "repository_root":context.primary,"candidate_path":context.root,"topology":topology,
                "policy_digest":blake3::hash(&serde_json::to_vec(&policy).map_err(|_|"intent_cleanup_token_invalid")?).to_hex().to_string()});
            let token = blake3::hash(
                &serde_json::to_vec(&packet).map_err(|_| "intent_cleanup_token_invalid")?,
            )
            .to_hex()
            .to_string();
            if !request.execute {
                return Ok(
                    json!({"status":if native_digest.is_some(){"ready"}else{"blocked"},"read_only":true,"operational_authority":false,"performed_mutation":false,"preview_token":token,"result":preview}),
                );
            }
            if request.preview.as_deref() != Some(token.as_str()) {
                return Err("intent_cleanup_preview_stale".into());
            }
            let native_digest = native_digest.ok_or("intent_cleanup_not_removable")?;
            context.fresh()?;
            // Re-observe topology after token comparison; the native owner then
            // repeats registration, liveness, dirtiness, receipt and HEAD admission.
            if git(&context.primary, &["worktree", "list", "--porcelain"])?
                != packet["topology"].as_str().unwrap_or_default()
                || file_digest(&receipt_path)?.as_deref() != Some(receipt_digest.as_str())
            {
                return Err("intent_cleanup_preview_stale".into());
            }
            let cleanup = native.cleanup.as_mut().expect("cleanup constructed above");
            cleanup.remove = true;
            cleanup.preview_receipt_digest = Some(native_digest);
            let result = prepare_intent_cleanup(&native).map_err(|finding| finding.code)?;
            let removed = matches!(result.cleanup, Some(CleanupDecision::Removed { .. }));
            let noop = matches!(
                result.cleanup,
                Some(CleanupDecision::Absent { .. } | CleanupDecision::AlreadyRemoved { .. })
            );
            Ok(
                json!({"status":if removed {"completed"}else if noop {"expected_noop"}else{"blocked"},"read_only":noop,"operational_authority":removed,
                "performed_mutation":if removed{Some(true)}else if noop{Some(false)}else{None},"effects_unknown":!removed&&!noop,"result":result}),
            )
        }
        _ => Err("intent_terminal_command_unknown".into()),
    }
}
