//! Semantic ownership for the guarded #505 cutover and rollback effects.
//!
//! The terminal owner retains authority for validating and performing the
//! filesystem operation. This adapter only supplies durable intent-before-effect,
//! exact receipt reconciliation, and one semantic administrative outcome.
use super::{context::SemanticContext, Context, IntentRequest};
use crate::adapters::{EnvironmentCredentialResolver, RealProcessAdapter};
use crate::commands::terminal::{
    observe_cutover_effect, prepare_terminal_cutover_with_github_observation,
    prepare_terminal_route, CutoverOperation, TerminalRouteRequest, TerminalRouteStatus,
};
use crate::lifecycle::semantic::{Facts, SemanticCommand};
use crate::storage::{
    semantic::{self, protocol::*},
    DurableTransactionStore,
};
use serde_json::{json, Value};

fn semantic_error(error: semantic::Error) -> String {
    format!("intent_administrative_semantic_{error:?}")
}

fn encode(value: &impl serde::Serialize) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|_| "intent_administrative_encoding_failed".into())
}

fn completion(value: &Completion) -> Value {
    json!({
        "operation_id":value.operation_id().as_str(),
        "outcome":value.outcome_kind(),
        "effect_truth":value.truth(),
        "version":value.current_version(),
        "original_version":value.original_version()
    })
}

fn semantic_for(context: &Context, command: SemanticCommand) -> Result<SemanticContext, String> {
    let (root, key) = context.semantic_root_key()?;
    if let semantic::Observation::Current(snapshot)
    | semantic::Observation::ProjectionRepairRequired(snapshot) =
        DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)?
    {
        if let Some(pending) = snapshot.pending() {
            if pending.command() != command {
                return Err("intent_administrative_other_operation_pending".into());
            }
            return context.semantic_recovery_context(pending.id());
        }
    }
    context.semantic_context()
}

fn completed_replay(
    context: &Context,
    command: SemanticCommand,
    identity: &NativeIdentity,
    bytes: &[u8],
) -> Result<Option<Value>, String> {
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            semantic::Observation::Current(snapshot)
            | semantic::Observation::ProjectionRepairRequired(snapshot) => *snapshot,
            _ => return Ok(None),
        };
    let candidate_content: Value =
        serde_json::from_slice(bytes).map_err(|_| "intent_administrative_encoding_failed")?;
    for completed in snapshot.completed().iter().rev() {
        let inspection = DurableTransactionStore::inspect_effect(&root, &key, completed.id())
            .map_err(semantic_error)?;
        let retained_content: Value = serde_json::from_slice(
            &inspection
                .request()
                .canonical_content()
                .map_err(semantic_error)?,
        )
        .map_err(|_| "intent_administrative_retained_request_invalid")?;
        if inspection.request().command() == command
            && inspection.request().native_identity() == identity
            && retained_content == candidate_content
        {
            return Ok(Some(json!({"status":"expected_noop","read_only":true,
            "performed_mutation":false,"semantic":{
                "operation_id":completed.id().as_str(),
                "outcome":completed.outcome(),
                "effect_truth":completed.truth(),
                "version":snapshot.version()
            }})));
        }
    }
    Ok(None)
}

fn native_request(
    request: &IntentRequest,
    context: &Context,
) -> Result<TerminalRouteRequest, String> {
    if request.content.is_null() {
        return Err("intent_administrative_operation_required".into());
    }
    let native: TerminalRouteRequest = serde_json::from_value(request.content.clone())
        .map_err(|_| "intent_administrative_operation_invalid")?;
    let cutover = native
        .cutover
        .as_ref()
        .ok_or("intent_administrative_cutover_required")?;
    let expected = match request.command.as_str() {
        "cutover" => CutoverOperation::Apply,
        "rollback" => CutoverOperation::Rollback,
        _ => return Err("intent_administrative_command_invalid".into()),
    };
    if cutover.operation != expected
        || native.repository != context.repository
        || native.issue != context.issue
        || native.issue != 505
        || cutover.repository_root.as_deref() != Some(context.primary.as_path())
        || cutover.execute != request.execute
    {
        return Err("intent_administrative_identity_mismatch".into());
    }
    Ok(native)
}

fn preflight(native: &TerminalRouteRequest) -> Result<Value, String> {
    let mut request = native.clone();
    request.cutover.as_mut().expect("validated cutover").execute = false;
    let plan = match request
        .cutover
        .as_ref()
        .expect("validated cutover")
        .operation
    {
        CutoverOperation::Apply => prepare_terminal_cutover_with_github_observation(
            &request,
            &mut RealProcessAdapter::new(EnvironmentCredentialResolver),
        )
        .map_err(|finding| finding.code)?,
        CutoverOperation::Rollback => {
            prepare_terminal_route("cutover", &request).map_err(|finding| finding.code)?
        }
    };
    Ok(
        json!({"status":if plan.status == TerminalRouteStatus::Ready {"ready"} else {"blocked"},
        "read_only":true,"performed_mutation":false,"operational_authority":false,"result":plan}),
    )
}

fn execute(native: &TerminalRouteRequest) -> Result<Value, String> {
    let plan = match native
        .cutover
        .as_ref()
        .expect("validated cutover")
        .operation
    {
        CutoverOperation::Apply => prepare_terminal_cutover_with_github_observation(
            native,
            &mut RealProcessAdapter::new(EnvironmentCredentialResolver),
        )
        .map_err(|finding| finding.code)?,
        CutoverOperation::Rollback => {
            prepare_terminal_route("cutover", native).map_err(|finding| finding.code)?
        }
    };
    if plan.status != TerminalRouteStatus::Ready {
        let codes = plan
            .findings
            .iter()
            .map(|finding| finding.code.as_str())
            .collect::<Vec<_>>()
            .join(",");
        return Err(format!("intent_administrative_native_blocked:{codes}"));
    }
    Ok(
        json!({"status":"completed","read_only":false,"performed_mutation":true,
        "operational_authority":false,"result":plan}),
    )
}

fn attach(
    semantic: &SemanticContext,
    ticket: OperationTicket,
    operation: &EffectRequest,
    observation: Value,
    _command: SemanticCommand,
    invocation_performed: bool,
) -> Result<Value, String> {
    let facts = Facts {
        administrative: true,
        administrative_outcome: true,
        ..Default::default()
    };
    let outcome = VerifiedOutcome::from_native_owner(
        OutcomeKind::Success,
        EffectTruth::Performed,
        encode(&observation)?,
        facts,
        operation.native_identity().clone(),
    )
    .map_err(semantic_error)?;
    // Administrative effects may rewrite or suspend selected authority bytes.
    // Exact terminal-owner readback above is the witness for that expected change;
    // retained pre-effect authority and origin still identify this transaction.
    let admission = AttachmentAdmission::from_native_owner(
        semantic.snapshot.inputs().authority().clone(),
        operation.origin().clone(),
    );
    let attached =
        DurableTransactionStore::attach_outcome(&semantic.root, ticket, outcome, admission)
            .map_err(semantic_error)?;
    match attached {
        Attachment::RecoveryRequired(version) => Ok(json!({"status":"recovery_required",
            "read_only":false,"performed_mutation":false,"semantic":{"version":version}})),
        Attachment::AlreadyCompleted(done) => Ok(json!({"status":"expected_noop",
            "read_only":true,"performed_mutation":false,"semantic":completion(&done)})),
        Attachment::Completed(done) => {
            let snapshot =
                match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
                    .map_err(semantic_error)?
                {
                    semantic::Observation::Current(value)
                    | semantic::Observation::ProjectionRepairRequired(value) => *value,
                    _ => return Err("intent_administrative_projection_state_unavailable".into()),
                };
            let projected = semantic.complete_projection(&snapshot)?;
            Ok(json!({"status":"completed","read_only":false,
                "performed_mutation":true,"native_invocation_performed":invocation_performed,
                "operational_authority":false,"observation":observation,
                "semantic":{"operation_id":done.operation_id().as_str(),"outcome":done.outcome_kind(),
                    "effect_truth":done.truth(),"version":projected.version()}}))
        }
    }
}

pub(crate) fn run(context: &Context, request: &IntentRequest) -> Result<Value, String> {
    let native = native_request(request, context)?;
    let preview = preflight(&native)?;
    if !request.execute {
        return Ok(preview);
    }
    context.fresh_integrity()?;
    let command = match native
        .cutover
        .as_ref()
        .expect("validated cutover")
        .operation
    {
        CutoverOperation::Apply => SemanticCommand::RecordCutover,
        CutoverOperation::Rollback => SemanticCommand::RecordRollback,
    };
    let bytes = encode(&native)?;
    let identity = NativeIdentity::new(
        match command {
            SemanticCommand::RecordCutover => "terminal-cutover",
            SemanticCommand::RecordRollback => "terminal-rollback",
            _ => unreachable!(),
        }
        .into(),
        blake3::hash(&bytes).to_hex().to_string(),
    )
    .map_err(semantic_error)?;
    if let Some(completed) = completed_replay(context, command, &identity, &bytes)? {
        return Ok(completed);
    }
    let semantic = semantic_for(context, command)?;
    let operation = EffectRequest::new(command, identity, semantic.origin.clone(), &bytes)
        .map_err(semantic_error)?;
    let facts = Facts {
        administrative: true,
        ..Default::default()
    };
    let reservation = DurableTransactionStore::reserve_effect(
        &semantic.root,
        EffectAdmission::from_native_owner(
            semantic.admission.clone(),
            semantic.origin.clone(),
            facts,
        ),
        operation.clone(),
    )
    .map_err(semantic_error)?;
    let ticket = match reservation {
        Reservation::AlreadyCompleted(done) => {
            return Ok(json!({"status":"expected_noop","read_only":true,
                "performed_mutation":false,"semantic":completion(&done)}))
        }
        Reservation::AlreadyPending(ticket) => {
            // A normal replay is reconciliation-only. Explicit recovery may later
            // resume the retained native journal, but never from caller bytes.
            return match observe_cutover_effect(&native) {
                Ok(observation) => {
                    attach(&semantic, ticket, &operation, observation, command, false)
                }
                Err(_) => Ok(json!({"status":"recovery_required","read_only":true,
                    "performed_mutation":false,"effects_unknown":true,
                    "semantic":{"operation_id":ticket.id().as_str()}})),
            };
        }
        Reservation::Reserved(ticket) => ticket,
    };
    semantic.admit_before_effect(ticket.id())?;
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_administrative_after_reservation")
    {
        std::process::exit(91);
    }
    if command == SemanticCommand::RecordRollback
        && !crate::authority::canonical_v2_rollback(&context.root)?
    {
        return Ok(json!({"status":"recovery_required","read_only":false,
            "performed_mutation":false,"effects_unknown":false,
            "reason":"tracked_selector_rollback_required",
            "semantic":{"operation_id":ticket.id().as_str(),"effect_truth":"not_performed"}}));
    }
    let native_result = execute(&native);
    let observation = observe_cutover_effect(&native);
    #[cfg(debug_assertions)]
    if observation.is_ok()
        && std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
            == Ok("semantic_administrative_after_effect")
    {
        std::process::exit(91);
    }
    match observation {
        Ok(observation) => attach(&semantic, ticket, &operation, observation, command, true),
        Err(observer) => {
            let native_error = native_result.as_ref().err().cloned();
            let observation_error = observer.code;
            let evidence = json!({"status":"recovery_required","native_error":native_error.clone(),
                "observation_error":observation_error.clone(),"effect_truth":"unknown"});
            let outcome = VerifiedOutcome::from_native_owner(
                OutcomeKind::Unresolved,
                EffectTruth::Unknown,
                encode(&evidence)?,
                Facts {
                    administrative: true,
                    administrative_outcome: false,
                    ..Default::default()
                },
                operation.native_identity().clone(),
            )
            .map_err(semantic_error)?;
            let attached = DurableTransactionStore::attach_outcome(
                &semantic.root,
                ticket,
                outcome,
                AttachmentAdmission::from_native_owner(
                    semantic.snapshot.inputs().authority().clone(),
                    operation.origin().clone(),
                ),
            )
            .map_err(semantic_error)?;
            Ok(json!({"status":"recovery_required","read_only":false,
                "performed_mutation":false,"effects_unknown":true,
                "native_error":native_error,"observation_error":observation_error,
                "semantic":{"attachment":format!("{attached:?}")}}))
        }
    }
}

/// Reconcile or explicitly resume only the exact request retained by a pending
/// administrative operation. Caller-supplied cutover bytes are never accepted.
pub(crate) fn recover(context: &Context, request: &IntentRequest) -> Result<Option<Value>, String> {
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            semantic::Observation::Current(value)
            | semantic::Observation::ProjectionRepairRequired(value) => *value,
            _ => return Ok(None),
        };
    let Some(pending) = snapshot.pending() else {
        return Ok(None);
    };
    if !matches!(
        pending.command(),
        SemanticCommand::RecordCutover | SemanticCommand::RecordRollback
    ) {
        return Ok(None);
    }
    let semantic = context.semantic_recovery_context(pending.id())?;
    let inspection = DurableTransactionStore::inspect_effect(&root, &key, pending.id())
        .map_err(semantic_error)?;
    let operation = inspection.request().clone();
    let native: TerminalRouteRequest =
        serde_json::from_slice(&operation.canonical_content().map_err(semantic_error)?)
            .map_err(|_| "intent_administrative_retained_request_invalid")?;
    let ticket = inspection
        .ticket()
        .cloned()
        .ok_or("intent_administrative_pending_ticket_missing")?;
    let settled = observe_cutover_effect(&native).ok();
    if !request.execute {
        return Ok(Some(
            json!({"status":if settled.is_some(){"ready"}else{"recovery_required"},
            "read_only":true,"performed_mutation":false,"effects_unknown":settled.is_none(),
            "semantic":{"operation_id":pending.id().as_str(),"resume_available":true}}),
        ));
    }
    if let Some(observation) = settled {
        return attach(
            &semantic,
            ticket,
            &operation,
            observation,
            pending.command(),
            false,
        )
        .map(Some);
    }
    semantic.admit_before_effect(pending.id())?;
    let result = execute(&native);
    let observation = observe_cutover_effect(&native);
    match observation {
        Ok(observation) => attach(
            &semantic,
            ticket,
            &operation,
            observation,
            pending.command(),
            result.is_ok(),
        )
        .map(Some),
        Err(_) => Ok(Some(json!({"status":"recovery_required","read_only":false,
            "performed_mutation":false,"effects_unknown":true,"native_error":result.err(),
            "semantic":{"operation_id":pending.id().as_str()}}))),
    }
}
