use super::{read_json, Context, IntentPlan, IntentRequest};
use crate::{
    adapters::{
        CommandInvocation, EnvironmentCredentialResolver, ProcessAdapter, ProcessStatus,
        RealProcessAdapter,
    },
    commands::local::{self, LocalPreparationRequest, PlanStatus},
};
use serde_json::{json, Value};

fn errors(findings: Vec<local::DoctorFinding>) -> String {
    serde_json::to_string(&findings).unwrap_or_else(|_| "intent_local_owner_failed".into())
}
fn owner(
    context: &Context,
    request: &LocalPreparationRequest,
) -> Result<local::OperationalLocalContext, String> {
    local::discover_operational_local_context(&context.root, request)
        .map_err(errors)?
        .ok_or_else(|| "intent_operational_authority_required".into())
}
pub fn run(context: &Context, intent: &IntentRequest) -> Result<Value, String> {
    if intent.command == "prepare" {
        return prepare(context, &intent.content);
    }
    let mut request = context.local_request()?;
    let registry = context.registry()?;
    if intent.command == "edit" {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Changes {
            schema: String,
            cards: std::collections::BTreeMap<String, Value>,
        }
        let changes: Changes =
            serde_json::from_value(intent.content.clone()).map_err(|_| "intent_changes_invalid")?;
        if changes.schema != "csdlc.v3.intent_changes.v1" {
            return Err("intent_changes_schema_unsupported".into());
        }
        request.card_updates = changes.cards;
    } else if intent.command != "status" && !intent.content.is_null() {
        return Err("intent_unexpected_content".into());
    }
    let native = owner(context, &request)?;
    if intent.command == "recover" {
        return local::intent::recover(
            &request,
            &native,
            intent.execute,
            intent.preview.as_deref(),
        )
        .map_err(errors);
    }
    let route = if intent.command == "status" {
        "doctor"
    } else {
        intent.command.as_str()
    };
    context.fresh()?;
    let result = local::execute_operational_local_route(route, &request, &registry, &native)
        .map_err(errors)?;
    let blocked = result
        .findings
        .iter()
        .any(|finding| finding.status != PlanStatus::Passed);
    let mut output = json!({"schema":"csdlc.v3.intent_local.v1","read_only":!result.mutated,"operational_authority":true,"writes_v3_state":result.mutated,"status":if blocked{"blocked"}else{"completed"},"result":result});
    if intent.command == "status" {
        #[derive(Default, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Decisions {
            design_ready: Option<bool>,
            dependencies_ready: Option<bool>,
            budget_available: Option<bool>,
            #[serde(default)]
            retryable_failure: bool,
            #[serde(default)]
            operator_decision_needed: bool,
        }
        let decisions: Decisions = if intent.content.is_null() {
            Decisions::default()
        } else {
            serde_json::from_value(intent.content.clone())
                .map_err(|_| "intent_status_decisions_invalid")?
        };
        let proof_current = semantic_proof_current(context)?;
        output["evidence"] = json!({"proof_current":proof_current,"validators_run":false});
        let unknown = decisions.design_ready.is_none()
            || decisions.dependencies_ready.is_none()
            || decisions.budget_available.is_none();
        request.schedule_readiness = Some(local::ScheduleReadinessInput {
            phase_ready: matches!(context.index["phase"].as_str(), Some("ready" | "bound")),
            cards_ready: !blocked,
            design_ready: decisions.design_ready.unwrap_or(false),
            dependencies_ready: decisions.dependencies_ready.unwrap_or(false),
            paths_clear: !blocked,
            budget_available: decisions.budget_available.unwrap_or(false),
        });
        request.shepherd_routing = Some(local::ShepherdRoutingInput {
            validation: proof_current.then(|| "current_declared_validator_proof".into()),
            dependency_wait: decisions.dependencies_ready == Some(false),
            retryable_failure: decisions.retryable_failure,
            repair_needed: blocked,
            operator_decision_needed: unknown || decisions.operator_decision_needed,
        });
        // Preserve all native recommendation owners; unknown human decisions are
        // explicit, never silently promoted into observed readiness facts.
        context.fresh()?;
        output["eligibility"] = json!(local::execute_operational_local_route(
            "eligibility",
            &request,
            &registry,
            &native
        )
        .map_err(errors)?);
        output["scheduling"] = json!(local::execute_operational_local_route(
            "schedule", &request, &registry, &native
        )
        .map_err(errors)?);
        output["shepherd"] = json!(local::execute_operational_local_route(
            "shepherd", &request, &registry, &native
        )
        .map_err(errors)?);
        output["operator_decisions"] = json!({"required":unknown||decisions.operator_decision_needed,"design_ready":decisions.design_ready,"dependencies_ready":decisions.dependencies_ready,"budget_available":decisions.budget_available});
        output["allowed_next"] = json!(if blocked {
            vec!["status", "recover"]
        } else if context.index["phase"] == "ready" {
            vec!["bind"]
        } else if !proof_current {
            vec!["edit", "validate", "proof"]
        } else {
            vec!["review"]
        });
        match crate::commands::remote::intent::pending_operations(
            &context.root,
            &context.repository,
            context.issue,
            &context.head,
        ) {
            Ok(pending) => {
                if !pending.is_empty() {
                    output["status"] = json!("recovery_required");
                    output["allowed_next"] = json!(["recover"]);
                }
                output["pending_remote"] = json!(pending);
            }
            Err(finding) => {
                output["status"] = json!("blocked");
                output["pending_remote"] = json!({"finding":finding});
                output["allowed_next"] = json!(["status"]);
            }
        }
    }
    Ok(output)
}

fn prepare(context: &Context, value: &Value) -> Result<Value, String> {
    if !context.index.is_null() {
        return Err("issue_already_initialized".into());
    }
    let plan: IntentPlan =
        serde_json::from_value(value.clone()).map_err(|_| "intent_plan_invalid")?;
    if plan.schema != "csdlc.v3.intent_plan.v1"
        || plan.slug.is_empty()
        || plan.slug.len() > 100
        || !plan
            .slug
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return Err("intent_plan_schema_or_slug_invalid".into());
    }
    if plan.cards.len() != 6
        || ["sip", "stp", "spp", "vpp", "srp", "sor"]
            .iter()
            .any(|kind| !plan.cards.get(*kind).is_some_and(Value::is_object))
    {
        return Err("intent_plan_six_cards_required".into());
    }
    let invocation = CommandInvocation::new(
        "github-api-read-only",
        [
            "issue".to_string(),
            context.repository.clone(),
            context.issue.to_string(),
        ],
    )
    .map_err(|_| "intent_issue_readback_invocation_invalid")?
    .with_child_credential("GH_TOKEN")
    .map_err(|_| "intent_credential_invalid")?;
    let output = RealProcessAdapter::new(EnvironmentCredentialResolver).run(invocation);
    if output.status != ProcessStatus::Exit(0) || output.truncated {
        return Err("intent_issue_readback_failed".into());
    }
    let issue: Value =
        serde_json::from_str(&output.stdout).map_err(|_| "intent_issue_readback_invalid")?;
    if issue["number"] != context.issue {
        return Err("intent_issue_readback_identity_mismatch".into());
    }
    let title = issue["title"]
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or("intent_issue_title_missing")?;
    let registry = context.registry()?;
    let policy = read_json(&context.root.join(".adl/worktree-policy.json"))?;
    let parent = std::path::PathBuf::from(
        policy["required_parent"]
            .as_str()
            .ok_or("intent_worktree_parent_missing")?,
    )
    .canonicalize()
    .map_err(|_| "intent_worktree_parent_unavailable")?;
    let mut request = LocalPreparationRequest {
        issue: context.issue,
        title: title.into(),
        repository: context.repository.clone(),
        branch: format!("codex/{}-{}", context.issue, plan.slug),
        worktree: parent
            .join(format!("adl-issue-{}-{}", context.issue, plan.slug))
            .to_string_lossy()
            .into_owned(),
        registry_version: registry.version.clone(),
        expected_lifecycle_digest: None,
        commands: local::required_local_commands().to_vec(),
        card_updates: plan.cards.clone(),
        schedule_readiness: None,
        shepherd_routing: None,
    };
    // Identity fields are derived once and cannot be overridden by changed prose.
    for card in request.card_updates.values_mut() {
        for key in [
            "issue",
            "issue_padded",
            "issue_url",
            "repository",
            "branch",
            "worktree",
            "title",
        ] {
            card.as_object_mut().unwrap().remove(key);
        }
    }
    let native = owner(context, &request)?;
    context.fresh()?;
    let (root, key) = context.semantic_root_key()?;
    let snapshot = local::intent::prepare_semantic(
        &request,
        &registry,
        &native,
        value,
        &root,
        key,
        context.semantic_authority()?,
    )
    .map_err(errors)?;
    match context.complete_semantic_projection(&snapshot) {
        Ok(current) => Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
            "operational_authority":true,"writes_v3_state":true,"status":"completed",
            "issue":context.issue,"semantic_version":current.version(),"inputs":current.inputs_version(),"phase":current.phase()}),
        ),
        Err(error) => Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
            "operational_authority":true,"writes_v3_state":true,"status":"recovery_required",
            "issue":context.issue,"semantic_version":snapshot.version(),"projection_repair":{"required":true,"finding":error}}),
        ),
    }
}

pub fn proof(context: &Context, intent: &IntentRequest) -> Result<Value, String> {
    if !intent.content.is_null() {
        return Err("intent_unexpected_content".into());
    }
    #[cfg(not(unix))]
    {
        let _ = context;
        Err("intent_validator_platform_not_supported".into())
    }
    #[cfg(unix)]
    {
        semantic_proof(context)
    }
}

#[cfg(unix)]
fn semantic_proof(context: &Context) -> Result<Value, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, DurableTransactionStore};
    let admitted_context = context.semantic_context()?;
    let validators = admitted_context
        .snapshot
        .inputs()
        .validation()
        .iter()
        .map(|value| super::Validator {
            id: value.id.clone(),
            program: value.program.clone(),
            args: value.args.clone(),
            success_marker: value.success_marker.clone(),
            timeout_seconds: value.timeout_seconds,
        })
        .collect::<Vec<_>>();
    let admitted = crate::commands::proof::intent::admit_validators(&context.root, &validators)?;
    let producer: Value = serde_json::from_slice(&admitted.request_bytes()?)
        .map_err(|_| "intent_validator_request_invalid")?;
    let request_bytes = serde_json::to_vec(&json!({"schema":"csdlc.v3.semantic_proof_request.v1",
        "inputs":admitted_context.snapshot.inputs_version(),"producer":producer}))
    .map_err(|_| "intent_validator_request_serialization_failed")?;
    let native = NativeIdentity::new(
        "intent-proof".into(),
        blake3::hash(&request_bytes).to_hex().to_string(),
    )
    .map_err(semantic_error)?;
    let request = EffectRequest::new(
        SemanticCommand::RecordProof,
        native.clone(),
        admitted_context.origin.clone(),
        &request_bytes,
    )
    .map_err(semantic_error)?;
    let reservation = DurableTransactionStore::reserve_effect(
        &admitted_context.root,
        EffectAdmission::from_native_owner(
            admitted_context.admission.clone(),
            admitted_context.origin.clone(),
            Facts::default(),
        ),
        request,
    )
    .map_err(semantic_error)?;
    let ticket = match reservation {
        Reservation::AlreadyCompleted(done) => return Ok(completed_proof(&done, true)),
        Reservation::AlreadyPending(ticket) => {
            return Ok(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"operation_id":ticket.id().as_str(),"reason":"explicit_original_proof_recovery_required"}))
        }
        Reservation::Reserved(ticket) => ticket,
    };
    let execution = crate::commands::proof::intent::execute_admitted(&admitted, || {
        admitted_context.admit_before_effect(ticket.id())
    });
    let final_admission = admitted_context.fresh_for_effect(ticket.id());
    let kind = if final_admission.is_err()
        || execution.effect_truth == EffectTruth::Unknown
        || !crate::commands::proof::intent::cleanup_complete(&execution.validators)
    {
        OutcomeKind::Unresolved
    } else if execution.passed {
        OutcomeKind::Success
    } else {
        OutcomeKind::Failure
    };
    let (attachment_admission, final_admission_error) = match final_admission {
        Ok(observed) => (observed, None),
        Err(error) => (
            AttachmentAdmission::from_native_owner(
                admitted_context.snapshot.inputs().authority().clone(),
                admitted_context.origin.clone(),
            ),
            Some(error),
        ),
    };
    let evidence = json!({"schema":"csdlc.v3.semantic_proof_evidence.v1","issue":context.issue,
        "repository":context.repository,"head":context.head,"inputs":admitted_context.snapshot.inputs_version(),
        "execution":execution.evidence(),"final_admission_error":final_admission_error});
    let bytes =
        serde_json::to_vec(&evidence).map_err(|_| "intent_proof_evidence_serialization_failed")?;
    let outcome = VerifiedOutcome::from_native_owner(
        kind,
        execution.effect_truth,
        bytes,
        Facts {
            current_proof: execution.passed && kind == OutcomeKind::Success,
            ..Facts::default()
        },
        native,
    )
    .map_err(semantic_error)?;
    let attached = DurableTransactionStore::attach_outcome(
        &admitted_context.root,
        ticket.clone(),
        outcome,
        attachment_admission,
    );
    match attached {
        Ok(Attachment::Completed(done)) => finish_proof_projection(&admitted_context, &done),
        Ok(Attachment::AlreadyCompleted(done)) => Ok(completed_proof(&done, true)),
        Ok(Attachment::RecoveryRequired(version)) => Ok(json!({"status":"recovery_required","read_only":false,
            "performed_mutation":true,"operation_id":ticket.id().as_str(),"semantic_version":version,
            "native_effect_truth":execution.effect_truth,"proof":evidence})),
        Err(error) => Err(json!({"status":"recovery_required","read_only":false,"performed_mutation":true,
            "operation_id":ticket.id().as_str(),"native_effect_truth":execution.effect_truth,"proof":evidence,
            "finding":semantic_error(error),"evidence_persistence":"not_confirmed"}).to_string()),
    }
}

fn semantic_error(error: crate::storage::semantic::Error) -> String {
    format!("semantic_owner_error: {error:?}")
}

fn proof_status(
    kind: crate::storage::semantic::protocol::OutcomeKind,
    replay: bool,
) -> &'static str {
    use crate::storage::semantic::protocol::OutcomeKind;
    match (kind, replay) {
        (OutcomeKind::Success, true) => "expected_noop",
        (OutcomeKind::Success, false) => "completed",
        (OutcomeKind::Failure, _) => "failed",
        (OutcomeKind::Unresolved, _) => "recovery_required",
    }
}

fn semantic_proof_current(context: &Context) -> Result<bool, String> {
    use crate::lifecycle::semantic::SemanticCommand;
    use crate::storage::{semantic::protocol::OutcomeKind, DurableTransactionStore};
    let admitted = context.semantic_context()?;
    if admitted
        .snapshot
        .pending()
        .is_some_and(|p| p.command() == SemanticCommand::RecordProof)
    {
        return Ok(false);
    }
    for completed in admitted.snapshot.completed().iter().rev() {
        let inspection =
            DurableTransactionStore::inspect_effect(&admitted.root, &admitted.key, completed.id())
                .map_err(semantic_error)?;
        if inspection.request().command() != SemanticCommand::RecordProof {
            continue;
        }
        if completed.outcome() != OutcomeKind::Success {
            return Ok(false);
        }
        let Some(bytes) = inspection.evidence() else {
            return Ok(false);
        };
        let evidence: Value =
            serde_json::from_slice(bytes).map_err(|_| "intent_retained_proof_invalid")?;
        if evidence["schema"] != "csdlc.v3.semantic_proof_evidence.v1"
            || evidence["repository"] != context.repository
            || evidence["issue"] != context.issue
            || evidence["head"] != context.head
            || evidence["inputs"]
                != serde_json::to_value(admitted.snapshot.inputs_version())
                    .map_err(|_| "intent_input_version_invalid")?
        {
            return Ok(false);
        }
        let validators = admitted
            .snapshot
            .inputs()
            .validation()
            .iter()
            .map(|v| super::Validator {
                id: v.id.clone(),
                program: v.program.clone(),
                args: v.args.clone(),
                success_marker: v.success_marker.clone(),
                timeout_seconds: v.timeout_seconds,
            })
            .collect::<Vec<_>>();
        return Ok(crate::commands::proof::intent::verify_execution_inputs(
            &context.root,
            &validators,
            &evidence["execution"],
        )
        .is_ok());
    }
    Ok(false)
}

fn completed_proof(done: &crate::storage::semantic::protocol::Completion, replay: bool) -> Value {
    use crate::storage::semantic::protocol::OutcomeKind;
    json!({"status":proof_status(done.outcome_kind(), replay),
        "read_only":replay,"performed_mutation":!replay,"operational_authority":true,
        "operation_id":done.operation_id().as_str(),"native_effect_truth":done.truth(),
        "semantic_outcome":done.outcome_kind(),"committed_at":done.original_version(),"current_version":done.current_version()})
}

/// Explicit reconciliation of an already retained proof attempt. A missing outcome
/// cannot establish whether a crashed validator launched, so this never reruns it.
pub(crate) fn recover_semantic_proof(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, DurableTransactionStore};
    let admitted = context.semantic_context()?;
    let Some(pending) = admitted.snapshot.pending() else {
        return Ok(None);
    };
    if pending.command() != SemanticCommand::RecordProof {
        return Ok(None);
    }
    let id = pending.id().clone();
    let preview = DurableTransactionStore::describe_effect_recovery(&admitted.root, &admitted.key)
        .map_err(semantic_error)?
        .ok_or("intent_proof_pending_missing")?;
    if !request.execute {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,
            "operation_id":id.as_str(),"preview_digest":preview.digest().as_str(),"action":"attach_retained_proof_only"}),
        ));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_proof_recovery_preview_stale".into());
    }
    let inspection = DurableTransactionStore::inspect_effect(&admitted.root, &admitted.key, &id)
        .map_err(semantic_error)?;
    let Some(bytes) = inspection.evidence() else {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,
            "operation_id":id.as_str(),"reason":"proof_outcome_unavailable_no_automatic_rerun"}),
        ));
    };
    let evidence: Value =
        serde_json::from_slice(bytes).map_err(|_| "intent_retained_proof_invalid")?;
    if evidence["schema"] != "csdlc.v3.semantic_proof_evidence.v1"
        || evidence["issue"] != context.issue
        || evidence["repository"] != context.repository
        || evidence["head"] != context.head
        || evidence["inputs"]
            != serde_json::to_value(admitted.snapshot.inputs_version())
                .map_err(|_| "intent_input_version_invalid")?
    {
        return Err("intent_retained_proof_identity_mismatch".into());
    }
    let truth = inspection
        .effect_truth()
        .ok_or("intent_retained_proof_truth_missing")?;
    if truth == EffectTruth::Unknown {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,
            "operation_id":id.as_str(),"reason":"proof_effect_unknown_requires_native_reconciliation"}),
        ));
    }
    let records = evidence["execution"]["validators"]
        .as_array()
        .ok_or("intent_retained_proof_validators_missing")?;
    if !crate::commands::proof::intent::cleanup_complete(records) {
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,
            "operation_id":id.as_str(),"native_effect_truth":truth,
            "reason":"proof_cleanup_unresolved_requires_native_reconciliation"}),
        ));
    }
    let passed = evidence["execution"]["passed"] == true;
    if passed {
        let validators = admitted
            .snapshot
            .inputs()
            .validation()
            .iter()
            .map(|value| super::Validator {
                id: value.id.clone(),
                program: value.program.clone(),
                args: value.args.clone(),
                success_marker: value.success_marker.clone(),
                timeout_seconds: value.timeout_seconds,
            })
            .collect::<Vec<_>>();
        crate::commands::proof::intent::verify_execution_inputs(
            &context.root,
            &validators,
            &evidence["execution"],
        )?;
    }
    let outcome = VerifiedOutcome::from_native_owner(
        if passed {
            OutcomeKind::Success
        } else {
            OutcomeKind::Failure
        },
        truth,
        bytes.to_vec(),
        Facts {
            current_proof: passed,
            ..Facts::default()
        },
        inspection.request().native_identity().clone(),
    )
    .map_err(semantic_error)?;
    let observed = admitted.fresh_for_effect(&id)?;
    let result = DurableTransactionStore::execute_effect_recovery(
        &admitted.root,
        preview,
        outcome,
        observed,
    )
    .map_err(semantic_error)?;
    Ok(Some(match result {
        Attachment::Completed(done) => finish_proof_projection(&admitted, &done)?,
        Attachment::AlreadyCompleted(done) => completed_proof(&done, true),
        Attachment::RecoveryRequired(version) => {
            json!({"status":"recovery_required","read_only":false,
            "performed_mutation":true,"operation_id":id.as_str(),"semantic_version":version})
        }
    }))
}

fn finish_proof_projection(
    context: &super::context::SemanticContext,
    done: &crate::storage::semantic::protocol::Completion,
) -> Result<Value, String> {
    use crate::storage::{semantic::Observation, DurableTransactionStore};
    let result = DurableTransactionStore::observe_issue(&context.root, &context.key)
        .map_err(semantic_error)
        .and_then(|observation| match observation {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                context.complete_projection(&snapshot)
            }
            _ => Err("intent_proof_projection_state_unavailable".into()),
        });
    let mut value = completed_proof(done, false);
    match result {
        Ok(snapshot) => {
            value["current_version"] = serde_json::to_value(snapshot.version())
                .map_err(|_| "semantic_version_serialization_failed")?;
        }
        Err(error) => {
            value["status"] = json!("recovery_required");
            value["projection_repair"] = json!({"required":true,"finding":error});
        }
    }
    Ok(value)
}

#[cfg(test)]
mod semantic_proof_regressions {
    // PVF: tooling/unit; deterministic in-memory outcome mapping; required local guard proof.
    use super::*;
    use crate::storage::semantic::protocol::OutcomeKind;
    #[test]
    fn failed_proof_replay_preserves_failure() {
        assert_eq!(proof_status(OutcomeKind::Failure, true), "failed");
        assert_eq!(proof_status(OutcomeKind::Success, true), "expected_noop");
        assert_eq!(
            proof_status(OutcomeKind::Unresolved, true),
            "recovery_required"
        );
    }
    #[test]
    fn every_validator_requires_explicit_cleanup_witness() {
        let complete = json!({"cleanup_complete":true});
        assert!(crate::commands::proof::intent::cleanup_complete(&[
            complete.clone()
        ]));
        assert!(!crate::commands::proof::intent::cleanup_complete(&[
            complete.clone(),
            json!({"cleanup_complete":false})
        ]));
        assert!(!crate::commands::proof::intent::cleanup_complete(&[
            complete,
            json!({})
        ]));
    }
}
