//! RecordInstall coordinates the existing native installer, never a second installer.
use super::{Context, IntentRequest};
use crate::{
    commands::proof::{self, InstallPlanInput, ProofRouteRequest, ProofWorktreeBinding},
    lifecycle::semantic::{Facts, SemanticCommand},
    storage::{
        semantic::{self, protocol::*},
        DurableTransactionStore,
    },
};
use serde_json::{json, Value};

fn error(e: semantic::Error) -> String {
    format!("semantic_install_error: {e:?}")
}
fn native_error(e: proof::ProofRouteFinding) -> String {
    e.code.into()
}
fn encode(value: &Value) -> Result<Vec<u8>, String> {
    serde_json::to_vec(value).map_err(|_| "install_encoding_failed".into())
}
fn facts(success: bool) -> Facts {
    Facts {
        administrative: true,
        administrative_outcome: success,
        ..Facts::default()
    }
}
fn native(context: &Context, plan: InstallPlanInput) -> Result<ProofRouteRequest, String> {
    Ok(ProofRouteRequest {
        issue: context.issue,
        repository: context.repository.clone(),
        binding: Some(ProofWorktreeBinding {
            worktree: context.root.clone(),
            branch: context.branch.clone(),
            exact_head: context.head.clone(),
            git_common_dir: context.git_common.clone(),
            generation: context.index["generation"]
                .as_u64()
                .ok_or("install_generation_missing")?,
            lifecycle_digest: context.index["digest"]
                .as_str()
                .ok_or("install_digest_missing")?
                .into(),
        }),
        cutover_issue: Some(505),
        operator_approval: None,
        evidence_root: Some(context.root.to_string_lossy().into_owned()),
        proof: None,
        shadow: None,
        soak: None,
        install: Some(plan),
    })
}
fn pending(id: &OperationId, detail: &str) -> Value {
    json!({"status":"recovery_required","read_only":false,"performed_mutation":null,"effects_unknown":true,"operation_id":id.as_str(),"finding":detail,"allowed_next":["recover"]})
}
fn complete(
    context: &Context,
    semantic: &super::context::SemanticContext,
    done: &Completion,
    witness: Value,
    replay: bool,
) -> Result<Value, String> {
    let snapshot = match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
        .map_err(error)?
    {
        semantic::Observation::Current(s) | semantic::Observation::ProjectionRepairRequired(s) => s,
        _ => return Err("install_projection_state_unavailable".into()),
    };
    let current = if replay {
        *snapshot
    } else {
        semantic.complete_projection(&snapshot)?
    };
    #[cfg(debug_assertions)]
    if !replay
        && std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
            == Ok("semantic_install_after_projection")
    {
        std::process::exit(91);
    }
    let registry = context.registry()?;
    let projection = super::local::semantic_rebuild(context, &registry)?;
    let projection_repaired = projection["status"] == "completed";
    Ok(
        json!({"status":if done.outcome_kind()!=OutcomeKind::Success {"failed"} else if replay && !projection_repaired {"expected_noop"} else {"completed"},
        "read_only":replay && !projection_repaired,"performed_mutation":!replay || projection_repaired,
        "native_effect_truth":done.truth(),"operational_authority":true,
        "operation_id":done.operation_id().as_str(),"semantic_version":projection["semantic_version"],
        "phase":current.phase(),"install":witness,"projection":projection["projection"]}),
    )
}
fn perform(
    context: &Context,
    session: &super::context::SemanticContext,
    request: EffectRequest,
    ticket: OperationTicket,
    recovering: bool,
) -> Result<Value, String> {
    let content: Value = serde_json::from_slice(&request.canonical_content().map_err(error)?)
        .map_err(|_| "install_retained_request_invalid")?;
    let native: ProofRouteRequest = serde_json::from_value(content["native"].clone())
        .map_err(|_| "install_retained_request_invalid")?;
    session.admit_before_effect(ticket.id())?;
    let observed = proof::semantic_install_admit(&native).map_err(native_error)?;
    if observed["inputs"] != content["preimage"]["inputs"] {
        return Err("install_reserved_inputs_changed".into());
    }
    let witness = if recovering {
        match proof::semantic_install_witness(&native) {
            Ok(_) => proof::semantic_install_sync(&native),
            Err(_)
                if observed["receipt"].is_null()
                    && observed["destination"]
                        == content["native"]["install"]["selected_binary_digest"] =>
            {
                proof::semantic_install_resume_receipt(&native)
            }
            Err(_) if observed == content["preimage"] => proof::semantic_install_execute(&native)
                .and_then(|()| proof::semantic_install_sync(&native)),
            Err(_) => {
                return Ok(pending(
                    ticket.id(),
                    "install_partial_or_foreign_destination_requires_reconciliation",
                ))
            }
        }
    } else {
        if observed != content["preimage"] {
            return Err("install_destination_preimage_changed".into());
        }
        proof::semantic_install_execute(&native)
            .and_then(|()| proof::semantic_install_sync(&native))
    };
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref() == Ok("semantic_install_after_effect")
    {
        std::process::exit(91);
    }
    let admission = session.fresh_for_effect(ticket.id());
    let inputs_current = proof::semantic_install_admit(&native)
        .is_ok_and(|current| current["inputs"] == content["preimage"]["inputs"]);
    let success = witness.is_ok() && admission.is_ok() && inputs_current;
    let result = match witness {
        Ok(w) => w,
        Err(e) => json!({"finding":e.code}),
    };
    let outcome = VerifiedOutcome::from_native_owner(
        if success {
            OutcomeKind::Success
        } else {
            OutcomeKind::Unresolved
        },
        if result.get("receipt").is_some() {
            EffectTruth::Performed
        } else {
            EffectTruth::Unknown
        },
        encode(&result)?,
        facts(success),
        request.native_identity().clone(),
    )
    .map_err(error)?;
    let attached = DurableTransactionStore::attach_outcome(
        &session.root,
        ticket.clone(),
        outcome,
        admission.unwrap_or_else(|_| {
            AttachmentAdmission::from_native_owner(
                session.snapshot.inputs().authority().clone(),
                request.origin().clone(),
            )
        }),
    )
    .map_err(error)?;
    match attached {
        Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
            complete(context, session, &done, result, false)
        }
        Attachment::RecoveryRequired(_) => Ok(pending(
            ticket.id(),
            "install_outcome_requires_reconciliation",
        )),
    }
}
pub(super) fn run(context: &Context, intent: &IntentRequest) -> Result<Value, String> {
    let mut plan: InstallPlanInput =
        serde_json::from_value(intent.content.clone()).map_err(|_| "install_plan_invalid")?;
    plan.executes_install = true;
    let request = native(context, plan)?;
    // Admission is observational and precedes any semantic reservation.
    let preimage = proof::semantic_install_admit(&request).map_err(native_error)?;
    let session = context.semantic_context()?;
    let plan_value = serde_json::to_value(request.install.as_ref().unwrap())
        .map_err(|_| "install_plan_invalid")?;
    for done in session.snapshot.completed().iter().rev() {
        let inspected =
            DurableTransactionStore::inspect_effect(&session.root, &session.key, done.id())
                .map_err(error)?;
        if inspected.request().command() != SemanticCommand::RecordInstall {
            continue;
        }
        let saved: Value =
            serde_json::from_slice(&inspected.request().canonical_content().map_err(error)?)
                .map_err(|_| "install_retained_request_invalid")?;
        if saved["native"]["install"] == plan_value
            && saved["inputs"]
                == serde_json::to_value(session.snapshot.inputs_version())
                    .map_err(|_| "install_inputs_invalid")?
        {
            let witness = proof::semantic_install_witness(&request).map_err(native_error)?;
            if !intent.execute {
                return Ok(
                    json!({"status":"ready","read_only":true,"performed_mutation":false,"install":witness}),
                );
            }
            // Repeat reserve only reads the exact completed operation and returns its immutable completion.
            let reserved = DurableTransactionStore::reserve_effect(
                &session.root,
                EffectAdmission::from_native_owner(
                    session.admission.clone(),
                    inspected.request().origin().clone(),
                    facts(false),
                ),
                inspected.request().clone(),
            )
            .map_err(error)?;
            let Reservation::AlreadyCompleted(done) = reserved else {
                return Err("install_completed_replay_changed".into());
            };
            return complete(context, &session, &done, witness, true);
        }
    }
    if !preimage["receipt"].is_null() {
        return Err("install_receipt_preexists".into());
    }
    if !intent.execute {
        return Ok(
            json!({"status":"ready","read_only":true,"performed_mutation":false,"preimage":preimage}),
        );
    }
    let content = json!({"schema":"csdlc.v3.semantic_install_request.v1","native":request,"inputs":session.snapshot.inputs_version(),"preimage":preimage});
    let bytes = encode(&content)?;
    let operation = EffectRequest::new(
        SemanticCommand::RecordInstall,
        NativeIdentity::new(
            "native-install".into(),
            blake3::hash(&bytes).to_hex().to_string(),
        )
        .map_err(error)?,
        session.origin.clone(),
        &bytes,
    )
    .map_err(error)?;
    context.repair_before_effect(&session.snapshot, &operation)?;
    let ticket = match DurableTransactionStore::reserve_effect(
        &session.root,
        EffectAdmission::from_native_owner(
            session.admission.clone(),
            session.origin.clone(),
            facts(false),
        ),
        operation.clone(),
    )
    .map_err(error)?
    {
        Reservation::Reserved(t) => t,
        Reservation::AlreadyPending(t) => {
            return Ok(pending(t.id(), "explicit_install_recovery_required"))
        }
        Reservation::AlreadyCompleted(done) => {
            return complete(
                context,
                &session,
                &done,
                proof::semantic_install_witness(&request).map_err(native_error)?,
                true,
            )
        }
    };
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_install_after_reservation")
    {
        std::process::exit(91);
    }
    perform(context, &session, operation, ticket, false)
}
pub(super) fn recover(context: &Context, intent: &IntentRequest) -> Result<Option<Value>, String> {
    let (root, key) = context.semantic_root_key()?;
    let observation = DurableTransactionStore::observe_issue(&root, &key).map_err(error)?;
    if !matches!(observation, semantic::Observation::Current(ref s) | semantic::Observation::ProjectionRepairRequired(ref s)
        if s.pending().is_some_and(|p| p.command() == SemanticCommand::RecordInstall))
    {
        return Ok(None);
    }
    let session = context.semantic_context()?;
    let Some(pending) = session
        .snapshot
        .pending()
        .filter(|p| p.command() == SemanticCommand::RecordInstall)
    else {
        return Ok(None);
    };
    let preview = DurableTransactionStore::describe_effect_recovery(&session.root, &session.key)
        .map_err(error)?
        .ok_or("install_recovery_missing")?;
    if !intent.execute {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,"preview_digest":preview.digest().as_str(),"action":"reconcile_reserved_install"}),
        ));
    }
    if intent.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("install_recovery_preview_stale".into());
    }
    let inspected =
        DurableTransactionStore::inspect_effect(&session.root, &session.key, pending.id())
            .map_err(error)?;
    let ticket = inspected
        .ticket()
        .cloned()
        .ok_or("install_pending_ticket_missing")?;
    perform(context, &session, inspected.request().clone(), ticket, true).map(Some)
}
