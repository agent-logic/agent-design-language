//! Exact retained journal activation. This never replays the business effect.
use super::{Context, IntentRequest};
use crate::storage::{
    semantic::journal_recovery::JournalRecoveryApproval, DurableTransactionStore,
};
use serde_json::{json, Value};

pub(super) fn recover(context: &Context, request: &IntentRequest) -> Result<Option<Value>, String> {
    let (root, key) = context.semantic_root_key()?;
    let error = |e| format!("intent_journal_recovery_{e:?}");
    let Some(preview) =
        DurableTransactionStore::describe_journal_recovery(&root, &key).map_err(error)?
    else {
        return Ok(None);
    };
    let target =
        DurableTransactionStore::journal_recovery_target(&root, &preview).map_err(error)?;
    if target.inputs().authority() != &context.semantic_authority()? {
        return Err("intent_semantic_authority_changed".into());
    }
    context.semantic_origin(&target)?;
    context.fresh()?;
    if !request.execute {
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"action":preview.action(),
            "preview_digest":preview.digest().as_str(),"target":preview.target()})));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_journal_recovery_preview_stale".into());
    }
    let approval = JournalRecoveryApproval::from_native_owner(&preview);
    DurableTransactionStore::execute_journal_recovery(&root, preview, approval).map_err(error)?;
    Ok(Some(
        json!({"status":"completed","read_only":false,"performed_mutation":true,
        "action":"activated_retained_issue_commit","business_effect_replayed":false}),
    ))
}

/// Repository-scoped creation journals require explicit operation selection.
pub(super) fn recover_creation(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    const SCHEMA: &str = "csdlc.v3.creation_journal_recovery_disposition.v1";
    if request.content["schema"] != SCHEMA {
        return Ok(None);
    }
    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    struct Selection {
        schema: String,
        operation_id: crate::storage::semantic::protocol::OperationId,
    }
    let selected: Selection = serde_json::from_value(request.content.clone())
        .map_err(|_| "intent_creation_journal_selection_invalid")?;
    if selected.schema != SCHEMA {
        return Err("intent_creation_journal_selection_invalid".into());
    }
    let (root, _) = context.semantic_root_key()?;
    let error = |e| format!("intent_creation_journal_recovery_{e:?}");
    let preview =
        DurableTransactionStore::describe_creation_journal_recovery(&root, &selected.operation_id)
            .map_err(error)?
            .ok_or("intent_creation_journal_not_pending")?;
    DurableTransactionStore::validate_creation_journal_admission(
        &root,
        &preview,
        &context.semantic_authority()?,
        &context.head,
    )
    .map_err(error)?;
    context.fresh()?;
    if !request.execute {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,
        "action":preview.action(),"operation_id":preview.operation_id().as_str(),"preview_digest":preview.digest().as_str()}),
        ));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_creation_journal_preview_stale".into());
    }
    let approval =
        crate::storage::semantic::protocol::creation::CreationJournalApproval::from_native_owner(
            &preview,
        );
    DurableTransactionStore::execute_creation_journal_recovery(&root, preview, approval)
        .map_err(error)?;
    Ok(Some(
        json!({"status":"completed","read_only":false,"performed_mutation":true,
        "action":"activated_retained_creation_commit","business_effect_replayed":false}),
    ))
}
