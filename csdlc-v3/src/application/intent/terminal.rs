//! Intent adapters for authenticated terminal delivery and exact preview cleanup.
use super::{context::git, Context, IntentRequest};
use crate::lifecycle::semantic::{Facts, SemanticCommand};
use crate::storage::{
    semantic::{self, protocol::*},
    DurableTransactionStore,
};
use crate::{
    adapters::{
        CommandInvocation, EnvironmentCredentialResolver, ProcessAdapter, ProcessStatus,
        RealProcessAdapter,
    },
    commands::{
        local::operational_state_root,
        remote::{intent::publication_target, settled_coordination_completion_receipt},
        terminal::*,
    },
};
use serde_json::{json, Value};
use std::{fs, path::PathBuf};

const ABSENT_CLEANUP_DISPOSITION_SCHEMA: &str =
    "csdlc.v3.semantic_cleanup_absence_recovery_disposition.v1";

fn semantic_error(error: semantic::Error) -> String {
    format!("intent_terminal_semantic_{error:?}")
}
fn encode(value: &impl serde::Serialize) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|_| "intent_terminal_evidence_invalid".into())
}
fn completion(value: &Completion) -> Value {
    json!({"operation_id":value.operation_id().as_str(),"outcome":value.outcome_kind(),
        "effect_truth":value.truth(),"version":value.current_version(),"original_version":value.original_version()})
}
fn legacy_coordination_completion_verified(context: &Context) -> Result<(), String> {
    if settled_coordination_completion_receipt(
        &context.primary,
        &context.repository,
        context.issue,
        &context.head,
    )
    .map_err(|finding| finding.code)?
    {
        Ok(())
    } else {
        Err("intent_legacy_coordination_completion_receipt_required".into())
    }
}
fn semantic_for(
    context: &Context,
    command: SemanticCommand,
) -> Result<super::context::SemanticContext, String> {
    let (root, key) = context.semantic_root_key()?;
    if let semantic::Observation::Current(snapshot)
    | semantic::Observation::ProjectionRepairRequired(snapshot) =
        DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)?
    {
        if let Some(pending) = snapshot.pending() {
            if pending.command() != command {
                return Err("intent_terminal_other_operation_pending".into());
            }
            return context.semantic_recovery_context(pending.id());
        }
    }
    if command == SemanticCommand::RecordCleanup {
        context.semantic_cleanup_context()
    } else {
        context.semantic_context()
    }
}
fn effect_result(
    context: &Context,
    semantic: &super::context::SemanticContext,
    ticket: OperationTicket,
    request: &EffectRequest,
    result: Result<Value, String>,
    facts: Facts,
) -> Result<Value, String> {
    let mut result = result.unwrap_or_else(
        |code| json!({"status":"recovery_required","effects_unknown":true,"error":code}),
    );
    // Invocation writes and the retained operation's historical effect are distinct.
    // Verified receipt/archive reconciliation supplies history even when this call
    // only attaches the already-performed result.
    let truth = if let Some(value) = result.get("historical_effect_truth") {
        serde_json::from_value(value.clone()).map_err(|_| "intent_terminal_effect_truth_invalid")?
    } else {
        match result["performed_mutation"].as_bool() {
            Some(true) => EffectTruth::Performed,
            Some(false) => EffectTruth::NotPerformed,
            None => EffectTruth::Unknown,
        }
    };
    let attachment = semantic.fresh_for_effect(ticket.id());
    let fresh = attachment.is_ok();
    let kind = if !fresh || truth == EffectTruth::Unknown || result["status"] == "recovery_required"
    {
        OutcomeKind::Unresolved
    } else if matches!(
        result["status"].as_str(),
        Some("completed" | "expected_noop")
    ) {
        OutcomeKind::Success
    } else {
        OutcomeKind::Failure
    };
    let outcome = VerifiedOutcome::from_native_owner(
        kind,
        truth,
        encode(&result)?,
        facts,
        request.native_identity().clone(),
    )
    .map_err(semantic_error)?;
    let operation_id = ticket.id().as_str().to_owned();
    let attached = DurableTransactionStore::attach_outcome(
        &semantic.root,
        ticket,
        outcome,
        attachment.unwrap_or_else(|_| {
            AttachmentAdmission::from_native_owner(
                semantic.snapshot.inputs().authority().clone(),
                request.origin().clone(),
            )
        }),
    );
    let attached = match attached {
        Ok(attached) => attached,
        Err(error) => {
            result["status"] = json!("recovery_required");
            result["semantic"] = json!({"operation_id":operation_id,"attachment_error":semantic_error(error),"effect_truth":truth});
            return Ok(result);
        }
    };
    result["semantic"] = match attached {
        Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
            let snapshot =
                match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
                    .map_err(semantic_error)?
                {
                    semantic::Observation::Current(snapshot)
                    | semantic::Observation::ProjectionRepairRequired(snapshot) => snapshot,
                    _ => return Err("intent_terminal_projection_state_unavailable".into()),
                };
            let projected = match semantic.complete_projection(&snapshot) {
                Ok(projected) => projected,
                Err(error) => {
                    result["status"] = json!("recovery_required");
                    result["semantic"] = completion(&done);
                    result["semantic"]["projection_error"] = json!(error);
                    return Ok(result);
                }
            };
            let mut value = completion(&done);
            value["version"] = serde_json::to_value(projected.version())
                .map_err(|_| "intent_terminal_evidence_invalid")?;
            value
        }
        Attachment::RecoveryRequired(version) => {
            result["status"] = json!("recovery_required");
            json!({"recovery_required":true,"version":version})
        }
    };
    let _ = context; // The retained semantic context remains valid after checkout removal.
    Ok(result)
}

fn native_request(context: &Context) -> Result<TerminalRouteRequest, String> {
    serde_json::from_value(json!({"repository":context.repository,"issue":context.issue,
        "pull_request":publication_target(&context.root,&context.repository,context.issue,&context.branch,&context.head).map_err(|finding|finding.code)?,
        "expected_head_sha":context.head,"mode":"closing","credential_names":["GITHUB_TOKEN"]}))
        .map_err(|_|"intent_terminal_request_invalid".into())
}
fn attach_terminal_observation(
    context: &Context,
    command: SemanticCommand,
    identity: &str,
    evidence: &Value,
    facts: Facts,
) -> Result<(), String> {
    let semantic = semantic_for(context, command)?;
    let bytes = encode(evidence)?;
    let operation = EffectRequest::new(
        command,
        NativeIdentity::new("terminal-finish".into(), identity.to_owned())
            .map_err(semantic_error)?,
        semantic.origin.clone(),
        &bytes,
    )
    .map_err(semantic_error)?;
    context.repair_before_effect(&semantic.snapshot, &operation)?;
    let reservation = DurableTransactionStore::reserve_effect(
        &semantic.root,
        EffectAdmission::from_native_owner(
            semantic.admission.clone(),
            operation.origin().clone(),
            facts.clone(),
        ),
        operation.clone(),
    )
    .map_err(semantic_error)?;
    let ticket = match reservation {
        Reservation::AlreadyCompleted(_) => return Ok(()),
        Reservation::AlreadyPending(ticket) | Reservation::Reserved(ticket) => ticket,
    };
    let outcome = VerifiedOutcome::from_native_owner(
        OutcomeKind::Success,
        EffectTruth::NotPerformed,
        bytes,
        facts,
        operation.native_identity().clone(),
    )
    .map_err(semantic_error)?;
    let observed = semantic.fresh_for_effect(ticket.id()).unwrap_or_else(|_| {
        AttachmentAdmission::from_native_owner(
            semantic.snapshot.inputs().authority().clone(),
            operation.origin().clone(),
        )
    });
    match DurableTransactionStore::attach_outcome(&semantic.root, ticket, outcome, observed)
        .map_err(semantic_error)?
    {
        Attachment::AlreadyCompleted(_) | Attachment::Completed(_) => {
            let snapshot =
                match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
                    .map_err(semantic_error)?
                {
                    semantic::Observation::Current(value)
                    | semantic::Observation::ProjectionRepairRequired(value) => *value,
                    _ => return Err("intent_terminal_semantic_state_unavailable".into()),
                };
            let _ = semantic.complete_projection(&snapshot)?;
            Ok(())
        }
        Attachment::RecoveryRequired(_) => Err("intent_terminal_merge_recovery_required".into()),
    }
}
fn catch_up_terminal_merge_state(
    context: &Context,
    staged: &TerminalRoutePlan,
) -> Result<(), String> {
    if staged.status != TerminalRouteStatus::Ready {
        return Ok(());
    }
    let Some(finish) = staged.finish.as_ref() else {
        return Ok(());
    };
    let evidence = json!({
        "schema":"csdlc.v3.semantic_terminal_merge_observation.v1",
        "repository":context.repository,
        "issue":context.issue,
        "head":context.head,
        "finish":finish
    });
    let semantic = semantic_for(context, SemanticCommand::MarkMergeReady)
        .or_else(|_| semantic_for(context, SemanticCommand::RecordMerge))
        .or_else(|_| semantic_for(context, SemanticCommand::Finish))?;
    match semantic.snapshot.phase() {
        crate::lifecycle::LifecycleState::Published => {
            attach_terminal_observation(
                context,
                SemanticCommand::MarkMergeReady,
                &format!("merge-ready:{}", context.head),
                &evidence,
                Facts {
                    merge_ready: true,
                    ..Default::default()
                },
            )?;
            attach_terminal_observation(
                context,
                SemanticCommand::RecordMerge,
                &format!("merged:{}", context.head),
                &evidence,
                Facts {
                    merge_ready: true,
                    merged: true,
                    ..Default::default()
                },
            )?;
        }
        crate::lifecycle::LifecycleState::MergeReady => {
            attach_terminal_observation(
                context,
                SemanticCommand::RecordMerge,
                &format!("merged:{}", context.head),
                &evidence,
                Facts {
                    merge_ready: true,
                    merged: true,
                    ..Default::default()
                },
            )?;
        }
        _ => {}
    }
    Ok(())
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

pub fn recover_absent_cleanup(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    if request.content.get("schema").and_then(Value::as_str)
        != Some(ABSENT_CLEANUP_DISPOSITION_SCHEMA)
    {
        return Ok(None);
    }
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Disposition {
        schema: String,
        disposition: String,
        repository: String,
        issue: u64,
        worktree: PathBuf,
        branch: String,
        head: String,
        terminal_receipt_digest: String,
        operator: String,
        rationale: String,
        evidence_refs: Vec<String>,
    }
    let approved: Disposition = serde_json::from_value(request.content.clone())
        .map_err(|_| "intent_cleanup_absence_disposition_invalid")?;
    if approved.schema != ABSENT_CLEANUP_DISPOSITION_SCHEMA
        || approved.disposition != "reconcile_already_absent_cleanup"
        || approved.repository != context.repository
        || approved.issue != context.issue
        || approved.operator.trim().is_empty()
        || approved.rationale.trim().is_empty()
        || approved.evidence_refs.is_empty()
        || approved
            .evidence_refs
            .iter()
            .any(|reference| reference.trim().is_empty())
    {
        return Err("intent_cleanup_absence_disposition_invalid".into());
    }
    let output_root = state_root(context)?;
    let receipt_path =
        output_root.join(format!("evidence/{}/terminal-receipt.json", context.issue));
    let receipt_bytes = fs::read(&receipt_path).map_err(|_| "intent_terminal_receipt_required")?;
    let receipt: DurableTerminalReceipt =
        serde_json::from_slice(&receipt_bytes).map_err(|_| "intent_terminal_receipt_invalid")?;
    let receipt_digest = blake3::hash(&receipt_bytes).to_hex().to_string();
    let state_path = output_root.join(format!("v3/issues/{}/terminal.json", context.issue));
    if receipt.schema != "csdlc.v3.terminal_receipt.v1"
        || receipt.repository != context.repository
        || receipt.issue != context.issue
        || receipt.disposition != "closed_out"
        || receipt.pull_request.is_none()
        || receipt.head_sha != approved.head
        || approved.terminal_receipt_digest != receipt_digest
        || receipt.state_digest.is_none()
        || receipt.state_digest != file_digest(&state_path)?
    {
        return Err("intent_terminal_receipt_mismatch".into());
    }
    let binding_path = output_root.join(format!("bindings/{}.json", context.issue));
    let binding = super::read_json(&binding_path).map_err(|_| "intent_cleanup_binding_required")?;
    if binding["schema"] != "csdlc.v3.binding.v1"
        || binding["issue"] != context.issue
        || binding["worktree"].as_str() != approved.worktree.to_str()
        || binding["branch"] != approved.branch
    {
        return Err("intent_cleanup_binding_invalid".into());
    }
    let topology = git(&context.primary, &["worktree", "list", "--porcelain"])?;
    if approved.worktree.exists()
        || topology
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
            .any(|path| path == approved.worktree.to_str().unwrap_or_default())
    {
        return Err("intent_cleanup_absence_target_present".into());
    }
    let semantic = context.semantic_cleanup_absence_context()?;
    let semantic_binding = semantic
        .snapshot
        .inputs()
        .binding()
        .ok_or("intent_cleanup_semantic_binding_required")?;
    if semantic_binding.worktree != approved.worktree
        || semantic_binding.branch != approved.branch
        || semantic_binding.head != approved.head
    {
        return Err("intent_cleanup_semantic_binding_mismatch".into());
    }
    let expected_registration = blake3::hash(
        serde_json::to_string(&json!({
            "branch":semantic_binding.branch,
            "worktree":semantic_binding.worktree
        }))
        .map_err(|_| "intent_cleanup_binding_invalid")?
        .as_bytes(),
    )
    .to_hex()
    .to_string();
    if semantic_binding.registration != expected_registration
        || binding
            .get("head")
            .is_some_and(|head| head != &json!(semantic_binding.head))
        || binding
            .get("registration")
            .is_some_and(|registration| registration != &json!(semantic_binding.registration))
    {
        return Err("intent_cleanup_binding_invalid".into());
    }
    if semantic
        .snapshot
        .pending()
        .is_some_and(|pending| pending.command() != SemanticCommand::RecordCleanup)
        || semantic.snapshot.completed().iter().any(|done| {
            DurableTransactionStore::inspect_effect(&semantic.root, &semantic.key, done.id())
                .is_ok_and(|inspection| {
                    inspection.request().command() == SemanticCommand::RecordCleanup
                })
        })
    {
        return Err("intent_cleanup_absence_operation_already_exists".into());
    }

    let native: TerminalRouteRequest = serde_json::from_value(json!({
        "repository":context.repository,
        "issue":context.issue,
        "expected_head_sha":receipt.head_sha,
        "pull_request":receipt.pull_request,
        "mode":"closing",
        "credential_names":["GITHUB_TOKEN"]
    }))
    .map_err(|_| "intent_terminal_request_invalid")?;
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let staged = prepare_terminal_finish_with_github_observation(&native, &mut process)
        .map_err(|finding| finding.code)?;
    if staged.status != TerminalRouteStatus::Ready {
        return Err("intent_cleanup_absence_remote_terminal_required".into());
    }

    let packet = json!({
        "schema":"csdlc.v3.semantic_cleanup_absence_reconciliation.v1",
        "repository":context.repository,
        "issue":context.issue,
        "binding":semantic_binding,
        "terminal_receipt_digest":receipt_digest,
        "topology":topology,
        "disposition":request.content,
        "remote_terminal":staged
    });
    let token = blake3::hash(&encode(&packet)?).to_hex().to_string();
    if !request.execute {
        return Ok(Some(json!({
            "status":"ready",
            "read_only":true,
            "operational_authority":false,
            "performed_mutation":false,
            "preview_token":token,
            "reconciliation":packet
        })));
    }
    if request.preview.as_deref() != Some(token.as_str()) {
        return Err("intent_cleanup_preview_stale".into());
    }
    context.fresh()?;
    if approved.worktree.exists()
        || git(&context.primary, &["worktree", "list", "--porcelain"])? != packet["topology"]
        || file_digest(&receipt_path)?.as_deref() != Some(approved.terminal_receipt_digest.as_str())
    {
        return Err("intent_cleanup_preview_stale".into());
    }
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let refreshed_terminal = prepare_terminal_finish_with_github_observation(&native, &mut process)
        .map_err(|finding| finding.code)?;
    if refreshed_terminal.status != TerminalRouteStatus::Ready
        || serde_json::to_value(&refreshed_terminal)
            .map_err(|_| "intent_terminal_observation_invalid")?
            != packet["remote_terminal"]
    {
        return Err("intent_cleanup_preview_stale".into());
    }
    let disposition_bytes = encode(&request.content)?;
    let origin = EffectOrigin::cleanup(
        semantic_binding.clone(),
        CleanupIdentity::from_absence_reconciliation(
            receipt_bytes,
            encode(&packet)?,
            disposition_bytes.clone(),
        )
        .map_err(semantic_error)?,
    );
    let proposed = EffectRequest::new(
        SemanticCommand::RecordCleanup,
        NativeIdentity::new("terminal-cleanup-absence-reconciliation".into(), token)
            .map_err(semantic_error)?,
        origin,
        &encode(&json!({
            "schema":ABSENT_CLEANUP_DISPOSITION_SCHEMA,
            "disposition":request.content,
            "reconciliation":packet
        }))?,
    )
    .map_err(semantic_error)?;
    let operation = if let Some(pending) = semantic.snapshot.pending() {
        let retained =
            DurableTransactionStore::inspect_effect(&semantic.root, &semantic.key, pending.id())
                .map_err(semantic_error)?;
        if retained.request() != &proposed {
            return Err("intent_cleanup_absence_operation_already_exists".into());
        }
        retained.request().clone()
    } else {
        proposed
    };
    let facts = Facts {
        terminal_receipt: true,
        cleanup: true,
        ..Default::default()
    };
    context.repair_before_effect(&semantic.snapshot, &operation)?;
    let ticket = match DurableTransactionStore::reserve_effect(
        &semantic.root,
        EffectAdmission::from_native_owner(
            semantic.admission.clone(),
            operation.origin().clone(),
            facts.clone(),
        ),
        operation.clone(),
    )
    .map_err(semantic_error)?
    {
        Reservation::Reserved(ticket) | Reservation::AlreadyPending(ticket) => ticket,
        Reservation::AlreadyCompleted(_) => {
            return Err("intent_cleanup_absence_operation_already_exists".into())
        }
    };
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("cleanup_absence_after_reservation")
    {
        std::process::exit(91);
    }
    let reserved_topology = git(&context.primary, &["worktree", "list", "--porcelain"])?;
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let reserved_terminal = prepare_terminal_finish_with_github_observation(&native, &mut process)
        .map_err(|finding| finding.code)?;
    if approved.worktree.exists()
        || reserved_topology != packet["topology"]
        || file_digest(&receipt_path)?.as_deref() != Some(approved.terminal_receipt_digest.as_str())
        || reserved_terminal.status != TerminalRouteStatus::Ready
        || serde_json::to_value(&reserved_terminal)
            .map_err(|_| "intent_terminal_observation_invalid")?
            != packet["remote_terminal"]
    {
        return Err("intent_cleanup_absence_changed_after_reservation".into());
    }
    semantic.fresh_for_recovery_effect(ticket.id())?;
    Ok(Some(effect_result(
        context,
        &semantic,
        ticket,
        &operation,
        Ok(json!({
            "status":"expected_noop",
            "read_only":false,
            "operational_authority":false,
            "performed_mutation":false,
            "historical_effect_truth":EffectTruth::NotPerformed,
            "reconciled_absence":packet
        })),
        facts,
    )?))
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
        let binding_path = output_root.join(format!("bindings/{}.json", context.issue));
        let binding =
            super::read_json(&binding_path).map_err(|_| "intent_cleanup_binding_required")?;
        if binding["schema"] != "csdlc.v3.binding.v1" || binding["issue"] != context.issue {
            return Err("intent_cleanup_binding_invalid".into());
        }
        let bound_path = binding["worktree"]
            .as_str()
            .ok_or("intent_cleanup_binding_invalid")?;
        let candidate = PathBuf::from(bound_path);
        let topology = git(&context.primary, &["worktree", "list", "--porcelain"])?;
        let registered = topology
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
            .any(|path| path == bound_path);
        let (root, key) = context.semantic_root_key()?;
        let observation =
            DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)?;
        if receipt
            .no_pr_closeout
            .as_ref()
            .is_some_and(|closeout| closeout.disposition == NoPrDisposition::CoordinationCompleted)
            && matches!(observation, semantic::Observation::LegacyMigrationRequired)
            && !registered
            && !candidate.exists()
        {
            legacy_coordination_completion_verified(context)?;
            retained_cleanup_index(&context.primary, &candidate, context.issue)
                .map_err(|finding| finding.code)?
                .ok_or("cleanup_archive_recovery_required")?;
            context.fresh()?;
            return Ok(json!({
                "status":"expected_noop",
                "read_only":true,
                "operational_authority":false,
                "performed_mutation":false,
                "issue":context.issue,
                "terminal_head":receipt.head_sha,
                "compatibility":"legacy_coordination_only"
            }));
        }
        let snapshot = match observation {
            semantic::Observation::Current(snapshot)
            | semantic::Observation::ProjectionRepairRequired(snapshot) => snapshot,
            _ => return Err("intent_cleanup_semantic_state_required".into()),
        };
        if registered || candidate.exists() {
            // A missing source index does not mean Git removal completed. Resume
            // only the exact registered checkout backed by verified archive bytes
            // and the retained semantic cleanup reservation.
            if !registered || !candidate.exists() {
                return Err("cleanup_archive_recovery_required".into());
            }
            let Some(pending) = snapshot
                .pending()
                .filter(|pending| pending.command() == SemanticCommand::RecordCleanup)
            else {
                return Err("intent_cleanup_pending_identity_required".into());
            };
            if snapshot
                .inputs()
                .binding()
                .is_none_or(|binding| binding.worktree != candidate)
            {
                return Err("intent_cleanup_pending_identity_required".into());
            }
            let inspection = DurableTransactionStore::inspect_effect(&root, &key, pending.id())
                .map_err(semantic_error)?;
            let content: Value = serde_json::from_slice(
                &inspection
                    .request()
                    .canonical_content()
                    .map_err(semantic_error)?,
            )
            .map_err(|_| "intent_cleanup_retained_request_invalid")?;
            let digest = content["archive_identity"]["inventory_digest"]
                .as_str()
                .ok_or("intent_cleanup_archive_identity_invalid")?;
            let retained = matching_retained_cleanup_index(
                &context.primary,
                &candidate,
                context.issue,
                digest,
            )
            .map_err(|finding| finding.code)?
            .ok_or("cleanup_archive_recovery_required")?;
            if snapshot
                .inputs()
                .binding()
                .is_none_or(|binding| binding.worktree != candidate)
                || snapshot
                    .pending()
                    .is_none_or(|current| current.id() != pending.id())
            {
                return Err("intent_cleanup_pending_identity_required".into());
            }
            let recovered = Context::load(&candidate, context.issue)?;
            if !recovered.cleanup_pending
                || recovered.root != candidate
                || recovered.index != retained
            {
                return Err("cleanup_archive_recovery_required".into());
            }
            return run(&recovered, request);
        }
        context.fresh()?;
        let mut selected = snapshot.pending().map(|pending| pending.id().clone());
        if selected.is_none() {
            for done in snapshot.completed().iter().rev() {
                let inspection = DurableTransactionStore::inspect_effect(&root, &key, done.id())
                    .map_err(semantic_error)?;
                if inspection.request().command() == SemanticCommand::RecordCleanup {
                    selected = Some(done.id().clone());
                    break;
                }
            }
        }
        let selected = selected.ok_or("intent_cleanup_semantic_completion_required")?;
        let semantic = context.semantic_recovery_context(&selected)?;
        let inspection = DurableTransactionStore::inspect_effect(&root, &key, &selected)
            .map_err(semantic_error)?;
        if inspection.request().command() != SemanticCommand::RecordCleanup {
            return Err("intent_cleanup_semantic_completion_required".into());
        }
        let retained: Value = serde_json::from_slice(
            &inspection
                .request()
                .canonical_content()
                .map_err(semantic_error)?,
        )
        .map_err(|_| "intent_cleanup_retained_request_invalid")?;
        if retained["schema"] == ABSENT_CLEANUP_DISPOSITION_SCHEMA {
            let completed = semantic
                .snapshot
                .completed()
                .iter()
                .find(|done| done.id() == &selected)
                .ok_or("intent_cleanup_semantic_completion_required")?;
            if completed.outcome() != OutcomeKind::Success
                || completed.truth() != EffectTruth::NotPerformed
            {
                return Err("intent_cleanup_semantic_success_required".into());
            }
            return Ok(json!({
                "status":"expected_noop",
                "read_only":true,
                "operational_authority":false,
                "performed_mutation":false,
                "historical_effect_truth":"not_performed",
                "issue":context.issue,
                "terminal_head":receipt.head_sha,
                "semantic":{
                    "operation_id":completed.id().as_str(),
                    "outcome":completed.outcome(),
                    "effect_truth":completed.truth(),
                    "version":semantic.snapshot.version()
                }
            }));
        }
        let expected = encode(&retained["archive_identity"])?;
        let archive = semantic_matching_retained_cleanup_archive_identity(
            &context.primary,
            &candidate,
            context.issue,
            &expected,
        )
        .map_err(|finding| finding.code)?
        .ok_or("cleanup_archive_recovery_required")?;
        if let Some(pending) = semantic.snapshot.pending() {
            if pending.command() != SemanticCommand::RecordCleanup {
                return Err("intent_cleanup_other_operation_pending".into());
            }
            let inspection = DurableTransactionStore::inspect_effect(
                &semantic.root,
                &semantic.key,
                pending.id(),
            )
            .map_err(semantic_error)?;
            let operation = inspection.request();
            let identity: Value = serde_json::from_slice(&archive)
                .map_err(|_| "intent_cleanup_archive_identity_invalid")?;
            if retained["archive_identity"] != identity {
                return Err("intent_cleanup_retained_archive_mismatch".into());
            }
            let packet = json!({"schema":"csdlc.v3.semantic_cleanup_recovery.v1","operation":pending.id().as_str(),
                "version":semantic.snapshot.version(),"archive":identity,"receipt":receipt,"topology":topology});
            let token = blake3::hash(&encode(&packet)?).to_hex().to_string();
            if !request.execute {
                return Ok(
                    json!({"status":"ready","read_only":true,"performed_mutation":false,"preview_token":token,"recovery":packet}),
                );
            }
            if request.preview.as_deref() != Some(token.as_str()) {
                return Err("intent_cleanup_preview_stale".into());
            }
            let ticket = inspection
                .ticket()
                .ok_or("intent_cleanup_pending_ticket_missing")?
                .clone();
            // No native dispatch occurs here; effect_result authenticates attachment.
            return effect_result(
                context,
                &semantic,
                ticket,
                operation,
                Ok(
                    json!({"status":"expected_noop","read_only":false,"performed_mutation":false,"historical_effect_truth":"performed","reconciled_removal":packet}),
                ),
                Facts {
                    terminal_receipt: true,
                    cleanup: true,
                    ..Default::default()
                },
            );
        }
        let completed = semantic
            .snapshot
            .completed()
            .iter()
            .find(|done| done.id() == &selected)
            .ok_or("intent_cleanup_semantic_completion_required")?;
        if completed.outcome() != OutcomeKind::Success
            || completed.truth() != EffectTruth::Performed
        {
            return Err("intent_cleanup_semantic_success_required".into());
        }
        return Ok(
            json!({"status":"expected_noop","read_only":true,"operational_authority":false,"performed_mutation":false,
            "issue":context.issue,"terminal_head":receipt.head_sha,"semantic":{"operation_id":completed.id().as_str(),
                "outcome":completed.outcome(),"effect_truth":completed.truth(),"version":semantic.snapshot.version()}}),
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
    let legacy_coordination_compatibility = native
        .no_pr_closeout
        .as_ref()
        .is_some_and(|closeout| closeout.disposition == NoPrDisposition::CoordinationCompleted)
        && context.semantic_migration_required()?;
    if legacy_coordination_compatibility {
        legacy_coordination_completion_verified(context)?;
    }
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
            context.fresh()?;
            let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
            // Authentication and terminal admission are observational before reservation.
            let staged = prepare_terminal_finish_with_github_observation(&native, &mut process)
                .map_err(|finding| finding.code)?;
            if request.preview.is_some() || staged.status != TerminalRouteStatus::Ready {
                return Ok(
                    json!({"status":if staged.status == TerminalRouteStatus::Ready {"ready"}else{"blocked"},
                    "read_only":true,"operational_authority":false,"performed_mutation":false,"result":staged}),
                );
            }
            let command = if native.no_pr_closeout.is_some() {
                SemanticCommand::FinishWithoutPr
            } else {
                SemanticCommand::Finish
            };
            if command == SemanticCommand::Finish {
                catch_up_terminal_merge_state(context, &staged)?;
            }
            if legacy_coordination_compatibility {
                if receipt_path.exists() {
                    let receipt: DurableTerminalReceipt = serde_json::from_slice(
                        &fs::read(&receipt_path).map_err(|_| "intent_terminal_receipt_required")?,
                    )
                    .map_err(|_| "intent_terminal_receipt_invalid")?;
                    if receipt.repository != context.repository
                        || receipt.issue != context.issue
                        || receipt.head_sha != context.head
                        || receipt.disposition != "closed_out"
                        || receipt.no_pr_closeout != native.no_pr_closeout
                        || receipt.state_digest.is_none()
                        || receipt.state_digest != file_digest(&state_path)?
                    {
                        return Err("intent_terminal_receipt_mismatch".into());
                    }
                    return Ok(json!({
                        "status":"expected_noop",
                        "read_only":true,
                        "operational_authority":true,
                        "performed_mutation":false,
                        "effects_unknown":false,
                        "result":staged,
                        "compatibility":"legacy_coordination_only"
                    }));
                }
                native.terminal_state = Some(TerminalStateWriteRequest {
                    repository_root: context.primary.clone(),
                    state_path,
                    receipt_path: receipt_path.clone(),
                    expected_state_digest: file_digest(
                        &output_root.join(format!("v3/issues/{}/terminal.json", context.issue)),
                    )?,
                });
                let result = prepare_terminal_finish_with_github_observation(&native, &mut process)
                    .map_err(|finding| finding.code)?;
                let completed = result.status == TerminalRouteStatus::Ready;
                return Ok(json!({
                    "status":if completed {"completed"} else {"blocked"},
                    "read_only":false,
                    "operational_authority":result.operational_authority,
                    "performed_mutation":if completed {Some(true)} else {None},
                    "effects_unknown":!completed,
                    "result":result,
                    "compatibility":"legacy_coordination_only"
                }));
            }
            let semantic = semantic_for(context, command)?;
            let bytes = encode(&native)?;
            let operation = EffectRequest::new(
                command,
                NativeIdentity::new(
                    "terminal-finish".into(),
                    blake3::hash(&bytes).to_hex().to_string(),
                )
                .map_err(semantic_error)?,
                semantic.origin.clone(),
                &bytes,
            )
            .map_err(semantic_error)?;
            let facts = Facts {
                terminal: true,
                merged: native.no_pr_closeout.is_none()
                    && staged.status == TerminalRouteStatus::Ready,
                no_pr_disposition: native.no_pr_closeout.is_some(),
                ..Default::default()
            };
            context.repair_before_effect(&semantic.snapshot, &operation)?;
            let reservation = DurableTransactionStore::reserve_effect(
                &semantic.root,
                EffectAdmission::from_native_owner(
                    semantic.admission.clone(),
                    operation.origin().clone(),
                    facts.clone(),
                ),
                operation.clone(),
            )
            .map_err(semantic_error)?;
            let ticket = match reservation {
                Reservation::AlreadyCompleted(done) => {
                    return Ok(
                        json!({"status":"expected_noop","read_only":true,"performed_mutation":false,"semantic":completion(&done)}),
                    )
                }
                Reservation::AlreadyPending(ticket) => {
                    // Reconcile retained terminal bytes; never repeat the writer blindly.
                    let bytes = fs::read(&receipt_path)
                        .map_err(|_| "intent_terminal_pending_readback_required")?;
                    let receipt: DurableTerminalReceipt = serde_json::from_slice(&bytes)
                        .map_err(|_| "intent_terminal_receipt_invalid")?;
                    if receipt.repository != context.repository
                        || receipt.issue != context.issue
                        || receipt.head_sha != context.head
                        || receipt.disposition != "closed_out"
                        || receipt.no_pr_closeout != native.no_pr_closeout
                        || receipt.state_digest.is_none()
                        || receipt.state_digest != file_digest(&state_path)?
                    {
                        return Err("intent_terminal_pending_readback_mismatch".into());
                    }
                    return effect_result(
                        context,
                        &semantic,
                        ticket,
                        &operation,
                        Ok(
                            json!({"status":"expected_noop","read_only":false,"performed_mutation":false,"historical_effect_truth":"performed","retained_receipt":receipt}),
                        ),
                        facts,
                    );
                }
                Reservation::Reserved(ticket) => ticket,
            };
            semantic.admit_before_effect(ticket.id())?;
            native.terminal_state = Some(TerminalStateWriteRequest {
                repository_root: context.primary.clone(),
                state_path,
                receipt_path: receipt_path.clone(),
                expected_state_digest: file_digest(
                    &output_root.join(format!("v3/issues/{}/terminal.json", context.issue)),
                )?,
            });
            let result = prepare_terminal_finish_with_github_observation(&native,&mut process)
                .map(|plan| {
                    let success = plan.status == TerminalRouteStatus::Ready;
                    json!({"status":if success {"completed"}else{"blocked"},"read_only":false,
                        "operational_authority":plan.operational_authority,"performed_mutation":if success {Some(true)}else{None},
                        "effects_unknown":!success,"result":plan})
                }).map_err(|finding|finding.code);
            effect_result(context, &semantic, ticket, &operation, result, facts)
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
            if legacy_coordination_compatibility {
                let result = prepare_intent_cleanup(&native).map_err(|finding| finding.code)?;
                let removed = matches!(result.cleanup, Some(CleanupDecision::Removed { .. }));
                let noop = matches!(
                    result.cleanup,
                    Some(CleanupDecision::Absent { .. } | CleanupDecision::AlreadyRemoved { .. })
                );
                return Ok(json!({
                    "status":if removed {"completed"} else if noop {"expected_noop"} else {"blocked"},
                    "read_only":false,
                    "operational_authority":removed,
                    "performed_mutation":if removed {Some(true)} else if noop {Some(false)} else {None},
                    "effects_unknown":!removed&&!noop,
                    "result":result,
                    "compatibility":"legacy_coordination_only"
                }));
            }
            let semantic = semantic_for(context, SemanticCommand::RecordCleanup)?;
            let archive = if semantic
                .snapshot
                .pending()
                .is_some_and(|pending| pending.command() == SemanticCommand::RecordCleanup)
            {
                let pending = semantic
                    .snapshot
                    .pending()
                    .expect("checked pending cleanup");
                let retained = DurableTransactionStore::inspect_effect(
                    &semantic.root,
                    &semantic.key,
                    pending.id(),
                )
                .map_err(semantic_error)?;
                let content: Value = serde_json::from_slice(
                    &retained
                        .request()
                        .canonical_content()
                        .map_err(semantic_error)?,
                )
                .map_err(|_| "intent_cleanup_retained_request_invalid")?;
                let expected = encode(&content["archive_identity"])?;
                match semantic_matching_retained_cleanup_archive_identity(
                    &context.primary,
                    &context.root,
                    context.issue,
                    &expected,
                )
                .map_err(|finding| finding.code)?
                {
                    Some(retained) => retained,
                    None => semantic_cleanup_archive_identity(
                        &context.primary,
                        &context.root,
                        context.issue,
                    )
                    .map_err(|finding| finding.code)?,
                }
            } else {
                semantic_cleanup_archive_identity(&context.primary, &context.root, context.issue)
                    .map_err(|finding| finding.code)?
            };
            let archive_identity: Value = serde_json::from_slice(&archive)
                .map_err(|_| "intent_cleanup_archive_identity_invalid")?;
            let facts = Facts {
                terminal_receipt: true,
                cleanup: true,
                ..Default::default()
            };
            let operation = if let Some(pending) = semantic.snapshot.pending() {
                if pending.command() != SemanticCommand::RecordCleanup {
                    return Err("intent_cleanup_other_operation_pending".into());
                }
                let retained = DurableTransactionStore::inspect_effect(
                    &semantic.root,
                    &semantic.key,
                    pending.id(),
                )
                .map_err(semantic_error)?;
                let content: Value = serde_json::from_slice(
                    &retained
                        .request()
                        .canonical_content()
                        .map_err(semantic_error)?,
                )
                .map_err(|_| "intent_cleanup_retained_request_invalid")?;
                if content["archive_identity"] != archive_identity {
                    return Err("intent_cleanup_retained_archive_mismatch".into());
                }
                retained.request().clone()
            } else {
                let origin = EffectOrigin::cleanup(
                    semantic
                        .snapshot
                        .inputs()
                        .binding()
                        .ok_or("intent_cleanup_semantic_binding_required")?
                        .clone(),
                    CleanupIdentity::from_native_owner(
                        fs::read(&receipt_path).map_err(|_| "intent_terminal_receipt_required")?,
                        encode(&packet)?,
                        archive,
                    )
                    .map_err(semantic_error)?,
                );
                let content =
                    json!({"native":native,"archive_identity":archive_identity,"preview":packet});
                let bytes = encode(&content)?;
                EffectRequest::new(
                    SemanticCommand::RecordCleanup,
                    NativeIdentity::new(
                        "terminal-cleanup".into(),
                        blake3::hash(&bytes).to_hex().to_string(),
                    )
                    .map_err(semantic_error)?,
                    origin,
                    &bytes,
                )
                .map_err(semantic_error)?
            };
            context.repair_before_effect(&semantic.snapshot, &operation)?;
            let ticket = match DurableTransactionStore::reserve_effect(
                &semantic.root,
                EffectAdmission::from_native_owner(
                    semantic.admission.clone(),
                    operation.origin().clone(),
                    facts.clone(),
                ),
                operation.clone(),
            )
            .map_err(semantic_error)?
            {
                Reservation::AlreadyCompleted(done) => {
                    return Ok(
                        json!({"status":"expected_noop","read_only":true,"performed_mutation":false,"semantic":completion(&done)}),
                    )
                }
                Reservation::Reserved(ticket) | Reservation::AlreadyPending(ticket) => ticket,
            };
            // The exact user-approved native preview is re-admitted on recovery;
            // retained semantic request and archive identity are never regenerated.
            semantic.admit_before_effect(ticket.id())?;
            let result = match prepare_intent_cleanup(&native) {
                Ok(result) => result,
                Err(finding) => {
                    let archived =
                        retained_cleanup_index(&context.primary, &context.root, context.issue)
                            .is_ok_and(|index| index.is_some());
                    return effect_result(
                        context,
                        &semantic,
                        ticket,
                        &operation,
                        Ok(json!({"status":"recovery_required","error":finding.code,
                            "performed_mutation":if archived{Some(true)}else{None},
                            "effects_unknown":!archived})),
                        facts,
                    );
                }
            };
            let removed = matches!(result.cleanup, Some(CleanupDecision::Removed { .. }));
            let noop = matches!(
                result.cleanup,
                Some(CleanupDecision::Absent { .. } | CleanupDecision::AlreadyRemoved { .. })
            );
            effect_result(
                context,
                &semantic,
                ticket,
                &operation,
                Ok(
                    json!({"status":if removed {"completed"}else if noop {"expected_noop"}else{"blocked"},"read_only":false,"operational_authority":removed,
                "performed_mutation":if removed{Some(true)}else if noop{Some(false)}else{None},
                "historical_effect_truth":if removed||context.cleanup_pending {EffectTruth::Performed}else if noop {EffectTruth::NotPerformed}else{EffectTruth::Unknown},
                "effects_unknown":!removed&&!noop,"result":result}),
                ),
                facts,
            )
        }
        _ => Err("intent_terminal_command_unknown".into()),
    }
}
