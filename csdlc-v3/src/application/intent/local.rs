use super::{read_json, Context, IntentPlan, IntentRequest, Publication, Validator};
use crate::{
    adapters::{
        CommandInvocation, EnvironmentCredentialResolver, ProcessAdapter, ProcessStatus,
        RealProcessAdapter,
    },
    commands::local::{self, LocalPreparationRequest, PlanStatus},
};
use serde_json::{json, Value};

#[derive(Clone, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
struct AmendmentDeclaration {
    class: crate::lifecycle::semantic::AmendmentClass,
    transition_approved: bool,
    #[serde(default)]
    implementation_revision: Option<String>,
    #[serde(default)]
    new_commit: bool,
}

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
    if intent.command == "status" {
        // Converted records already have authoritative semantic state. Reading
        // it must not require recreating a legacy local index first.
        if context.index.is_null() {
            let semantic = context.semantic_context()?;
            let snapshot = &semantic.snapshot;
            let pending = snapshot.pending().is_some();
            let allowed_next = if pending {
                vec!["recover"]
            } else if snapshot.phase() == crate::lifecycle::LifecycleState::ClosedOut {
                vec!["validate"]
            } else if snapshot.inputs().binding().is_none() {
                vec!["bind"]
            } else {
                vec!["edit", "validate", "proof"]
            };
            return Ok(json!({
                "schema":"csdlc.v3.intent_local.v1", "status":if pending {"recovery_required"} else {"completed"},
                "read_only":true,"performed_mutation":false,"writes_v3_state":false,
                "operational_authority":true,"issue":context.issue,
                "phase":snapshot.phase(),"semantic_version":snapshot.version(),
                "allowed_next":allowed_next,"evidence":{"validators_run":false},
                "preparation":{"structural_state":"semantic_record_present","execution_ready":false}
            }));
        }
        let (semantic_root, semantic_key) = context.semantic_root_key()?;
        if let crate::storage::semantic::Observation::Current(snapshot)
        | crate::storage::semantic::Observation::ProjectionRepairRequired(snapshot) =
            crate::storage::DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                .map_err(semantic_error)?
        {
            use crate::lifecycle::semantic::SemanticCommand;
            if snapshot.pending().is_some_and(|pending| {
                matches!(
                    pending.command(),
                    SemanticCommand::RecordInstall
                        | SemanticCommand::RecordCutover
                        | SemanticCommand::RecordRollback
                )
            }) {
                return Ok(json!({
                    "schema":"csdlc.v3.intent_local.v1", "status":"recovery_required",
                    "read_only":true, "performed_mutation":false, "writes_v3_state":false,
                    "operational_authority":false, "allowed_next":["recover"],
                    "evidence":{"proof_current":false,"validators_run":false}
                }));
            }
        }
    }
    if intent.command == "bind" && context.index.is_null() {
        if !intent.content.is_null() {
            return Err("intent_unexpected_content".into());
        }
        let registry = context.registry()?;
        let (snapshot, _, _) = semantic_card_projection_observation(context, &registry, false)?
            .ok_or("intent_semantic_state_missing")?;
        if snapshot.inputs().binding().is_some() {
            let changed = context.refresh_semantic_binding()?;
            let mut result = semantic_rebuild(context, &registry)?;
            if changed {
                result["read_only"] = json!(false);
                result["writes_v3_state"] = json!(true);
                result["performed_mutation"] = json!(true);
            }
            result["binding_refreshed"] = json!(changed);
            return Ok(result);
        }
        let semantic = context.semantic_context()?;
        if semantic.snapshot.phase() != crate::lifecycle::LifecycleState::Ready
            || semantic.snapshot.pending().is_some()
            || semantic.snapshot.inputs().binding().is_some()
        {
            return Err("intent_bind_phase_invalid".into());
        }
        let inputs = semantic.snapshot.inputs();
        let registry = context.registry()?;
        let policy = read_json(&context.root.join(".adl/worktree-policy.json"))?;
        let parent = std::path::PathBuf::from(
            policy["required_parent"]
                .as_str()
                .ok_or("intent_worktree_parent_missing")?,
        );
        let request = LocalPreparationRequest {
            issue: context.issue,
            title: inputs.intent().to_owned(),
            repository: context.repository.clone(),
            branch: format!("codex/{}-{}", context.issue, inputs.slug()),
            worktree: parent
                .join(format!("adl-issue-{}-{}", context.issue, inputs.slug()))
                .to_string_lossy()
                .into_owned(),
            registry_version: registry.version.clone(),
            expected_lifecycle_digest: None,
            commands: local::required_local_commands().to_vec(),
            card_updates: inputs.cards().clone(),
            schedule_readiness: None,
            shepherd_routing: None,
        };
        let native = owner(context, &request)?;
        let plan =
            serde_json::to_value(inputs.accepted_plan()).map_err(|_| "intent_plan_invalid")?;
        context.fresh_integrity()?;
        // The existing native binder still consumes a compatibility projection.
        // Materialize it through its create-only owner from canonical inputs;
        // do not recreate or replace the converted semantic record.
        local::intent::prepare(&request, &registry, &native, &plan).map_err(errors)?;
        let current = Context::load(&context.root, context.issue)?;
        if current.semantic_context()?.snapshot.version() != semantic.snapshot.version() {
            return Err("intent_snapshot_stale".into());
        }
        return run(&current, intent);
    }
    if intent.command == "validate" && context.index.is_null() {
        if !intent.content.is_null() {
            return Err("intent_unexpected_content".into());
        }
        let semantic = context.semantic_context()?;
        let bundle = crate::application::derive_semantic_card_projection(
            &semantic.snapshot,
            &context.registry()?,
        )
        .map_err(|error| format!("intent_semantic_projection_derivation_failed:{error}"))?;
        let observation = crate::storage::DurableTransactionStore::observe_card_projection(
            &semantic.root,
            &semantic.snapshot,
            &bundle,
        )
        .map_err(semantic_error)?;
        return Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","status":"completed","read_only":true,
            "performed_mutation":false,"writes_v3_state":false,"operational_authority":true,
            "phase":semantic.snapshot.phase(),"semantic_version":semantic.snapshot.version(),
            "projection":{"observation":observation},"evidence":{"validators_run":false}}),
        );
    }
    let mut request = context.local_request()?;
    let registry = context.registry()?;
    if intent.command == "edit" {
        #[derive(serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Changes {
            schema: String,
            #[serde(default)]
            cards: std::collections::BTreeMap<String, Value>,
            #[serde(default)]
            validators: Option<Vec<Validator>>,
            #[serde(default)]
            publication: Option<Publication>,
            #[serde(default)]
            amendment: Option<AmendmentDeclaration>,
        }
        let changes: Changes =
            serde_json::from_value(intent.content.clone()).map_err(|_| "intent_changes_invalid")?;
        if changes.schema != "csdlc.v3.intent_changes.v1" {
            return Err("intent_changes_schema_unsupported".into());
        }
        let selected = usize::from(!changes.cards.is_empty())
            + usize::from(changes.validators.is_some())
            + usize::from(changes.publication.is_some());
        if selected == 0 {
            return Err("intent_changes_missing".into());
        }
        if selected != 1 {
            return Err("intent_changes_single_surface_required".into());
        }
        if let Some(publication) = changes.publication {
            if changes.amendment.is_some() {
                return Err("intent_publication_amendment_unexpected".into());
            }
            return semantic_publication_edit(context, publication, &intent.snapshot);
        }
        request.card_updates = changes.cards;
        if let Some(validators) = changes.validators {
            return semantic_validation_edit(context, validators);
        }
        let amendment = changes
            .amendment
            .as_ref()
            .ok_or("intent_amendment_missing")?;
        let native = owner(context, &request)?;
        return semantic_edit(
            context,
            &request,
            &registry,
            &native,
            amendment,
            &intent.snapshot,
        );
    } else if intent.command != "status" && !intent.content.is_null() {
        return Err("intent_unexpected_content".into());
    }
    let native = owner(context, &request)?;
    if intent.command == "rebuild" {
        return semantic_rebuild(context, &registry);
    }
    if intent.command == "recover" {
        return local::intent::recover(
            &request,
            &native,
            intent.execute,
            intent.preview.as_deref(),
        )
        .map_err(errors);
    }
    if intent.command == "bind" {
        return semantic_bind(context, &request, &registry, &native);
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
    if intent.command == "validate" {
        let projection = semantic_card_projection_observation(context, &registry, false)?;
        if let Some((_, bundle, observation)) = projection {
            output["projection"] = json!({
                "observation": observation,
                "semantic_digest": bundle.semantic_digest().as_str(),
                "projection_digest": bundle.projection_digest().as_str(),
                "registry_version": bundle.registry_version(),
            });
            if observation != crate::storage::semantic::CardProjectionObservation::Healthy {
                output["projection"]["repair_command"] = json!("rebuild");
            }
        }
    }
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
        let projection = semantic_card_projection_observation(context, &registry, false)?;
        let semantic_preparation_required = projection.is_none();
        let proof_current = if semantic_preparation_required {
            false
        } else {
            match semantic_proof_current(context) {
                Ok(current) => current,
                // Status is an observation route. A commit after binding makes proof
                // stale, but must not turn that read-only observation into a failure.
                Err(error)
                    if matches!(
                        error.as_str(),
                        "intent_semantic_binding_stale"
                            | "intent_semantic_projection_repair_required"
                    ) =>
                {
                    false
                }
                Err(error) => return Err(error),
            }
        };
        output["evidence"] = json!({"proof_current":proof_current,"validators_run":false});
        if semantic_preparation_required {
            output["preparation"] = json!({
                "structural_state":"native_record_present_semantic_preparation_required",
                "dependencies_ready":decisions.dependencies_ready,
                "execution_ready":false,
                "allowed_next":["prepare"]
            });
        }
        if let Some((_, bundle, observation)) = &projection {
            output["projection"] = json!({
                "observation": observation,
                "semantic_digest": bundle.semantic_digest().as_str(),
                "projection_digest": bundle.projection_digest().as_str(),
                "registry_version": bundle.registry_version(),
            });
        }
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
        output["allowed_next"] = json!(if semantic_preparation_required {
            vec!["prepare"]
        } else if blocked {
            vec!["status", "recover"]
        } else if context.index["phase"] == "ready" {
            vec!["bind"]
        } else if !proof_current {
            vec!["edit", "validate", "proof"]
        } else {
            vec!["review"]
        });
        if let Some((_, _, observation)) = projection {
            if observation != crate::storage::semantic::CardProjectionObservation::Healthy {
                output["projection"]["repair_command"] = json!("rebuild");
            }
        }
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

pub(super) fn semantic_card_projection_observation(
    context: &Context,
    registry: &local::PromptRegistry,
    require_current_binding: bool,
) -> Result<
    Option<(
        crate::storage::semantic::Snapshot,
        crate::storage::semantic::CardProjectionBundle,
        crate::storage::semantic::CardProjectionObservation,
    )>,
    String,
> {
    use crate::storage::{semantic::Observation, DurableTransactionStore};
    let (root, key) = context.semantic_root_key()?;
    let snapshot = match crate::storage::DurableTransactionStore::observe_issue(&root, &key)
        .map_err(semantic_error)?
    {
        Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
            *snapshot
        }
        Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
        Observation::LegacyMigrationRequired | Observation::Absent => return Ok(None),
    };
    if snapshot.inputs().authority() != &context.semantic_authority()? {
        return Err("intent_semantic_authority_changed".into());
    }
    if let Some(binding) = snapshot.inputs().binding() {
        if require_current_binding
            && (binding.branch != context.branch
                || binding.head != context.head
                || binding.worktree != context.root)
        {
            return Err("intent_semantic_binding_stale".into());
        }
    } else if require_current_binding && context.root != context.primary {
        return Err("intent_semantic_binding_stale".into());
    }
    let bundle = crate::application::derive_semantic_card_projection(&snapshot, registry)
        .map_err(|error| format!("intent_semantic_projection_derivation_failed:{error}"))?;
    let observation = DurableTransactionStore::observe_card_projection(&root, &snapshot, &bundle)
        .map_err(semantic_error)?;
    Ok(Some((snapshot, bundle, observation)))
}

pub(super) fn semantic_rebuild(
    context: &Context,
    registry: &local::PromptRegistry,
) -> Result<Value, String> {
    use crate::storage::{
        semantic::{CardProjectionObservation, ProjectionWriteProof},
        DurableTransactionStore,
    };
    context.refresh_semantic_binding()?;
    let Some((snapshot, bundle, before)) =
        semantic_card_projection_observation(context, registry, true)?
    else {
        return Err("intent_semantic_state_missing".into());
    };
    if snapshot.pending().is_some() {
        return Err("intent_semantic_recovery_required".into());
    }
    let (root, _) = context.semantic_root_key()?;
    if before == CardProjectionObservation::Healthy
        && ProjectionWriteProof::verify(&root, &snapshot).is_ok()
    {
        return Ok(json!({
            "schema":"csdlc.v3.intent_local.v1", "status":"expected_noop",
            "read_only":true, "writes_v3_state":false, "operational_authority":true,
            "projection":{"before":before,"after":CardProjectionObservation::Healthy,
                "semantic_digest":bundle.semantic_digest().as_str(),
                "projection_digest":bundle.projection_digest().as_str(),
                "registry_version":bundle.registry_version()},
            "semantic_version":snapshot.version(),
        }));
    }
    let semantic_digest = bundle.semantic_digest().as_str().to_owned();
    let projection_digest = bundle.projection_digest().as_str().to_owned();
    let registry_version = bundle.registry_version().to_owned();
    DurableTransactionStore::write_card_projection(&root, &snapshot, bundle)
        .map_err(semantic_error)?;
    let committed = snapshot;
    let regenerated = crate::application::derive_semantic_card_projection(&committed, registry)
        .map_err(|error| format!("intent_semantic_projection_derivation_failed:{error}"))?;
    let after = DurableTransactionStore::observe_card_projection(&root, &committed, &regenerated)
        .map_err(semantic_error)?;
    if after != CardProjectionObservation::Healthy
        || regenerated.semantic_digest().as_str() != semantic_digest
        || regenerated.projection_digest().as_str() != projection_digest
    {
        return Err("intent_semantic_projection_readback_mismatch".into());
    }
    Ok(json!({
        "schema":"csdlc.v3.intent_local.v1", "status":"completed",
        "read_only":false, "writes_v3_state":true, "operational_authority":true,
        "projection":{"before":before,"after":after,"semantic_digest":semantic_digest,
            "projection_digest":projection_digest,"registry_version":registry_version},
        "semantic_version":committed.version(),
    }))
}

fn semantic_rebuild_current(
    context: &Context,
    registry: &local::PromptRegistry,
) -> Result<Value, String> {
    let current = Context::load(&context.root, context.issue)?;
    semantic_rebuild(&current, registry)
}

fn semantic_edit(
    context: &Context,
    request: &LocalPreparationRequest,
    registry: &local::PromptRegistry,
    native: &local::OperationalLocalContext,
    amendment: &AmendmentDeclaration,
    expected: &super::Snapshot,
) -> Result<Value, String> {
    use crate::storage::semantic::{LocalChange, VerifiedCardAmendment};
    fn merge(base: &mut Value, update: &Value) {
        if let (Some(base), Some(update)) = (base.as_object_mut(), update.as_object()) {
            for (key, value) in update {
                match base.get_mut(key) {
                    Some(existing) if existing.is_object() && value.is_object() => {
                        merge(existing, value)
                    }
                    _ => {
                        base.insert(key.clone(), value.clone());
                    }
                }
            }
        }
    }
    let Some((preflight, _, _)) = semantic_card_projection_observation(context, registry, false)?
    else {
        return Err("intent_semantic_state_missing".into());
    };
    if preflight.pending().is_some() {
        return Err("intent_semantic_recovery_required".into());
    }
    require_semantic_version(expected, &preflight)?;
    let Some(binding) = preflight.inputs().binding() else {
        return Err("intent_semantic_binding_stale".into());
    };
    let registration = blake3::hash(
        serde_json::to_string(&json!({"branch":context.branch,"worktree":context.root}))
            .map_err(|_| "intent_bind_identity_invalid")?
            .as_bytes(),
    )
    .to_hex()
    .to_string();
    if binding.branch != context.branch
        || binding.worktree != context.root
        || (binding.registration != registration && binding.registration != "git-worktree-list")
    {
        return Err("intent_semantic_binding_stale".into());
    }
    let binding_advances = binding.head != context.head || binding.registration != registration;
    let mut cards = preflight.inputs().cards().clone();
    for (kind, update) in &request.card_updates {
        let card = cards
            .get_mut(kind)
            .ok_or("intent_semantic_card_kind_invalid")?;
        merge(card, update);
    }
    let inferred_classes = request
        .card_updates
        .keys()
        .map(|kind| match kind.as_str() {
            "sip" | "stp" => Ok(crate::lifecycle::semantic::AmendmentClass::ScopeAcceptance),
            "spp" => Ok(crate::lifecycle::semantic::AmendmentClass::Plan),
            "vpp" => Ok(crate::lifecycle::semantic::AmendmentClass::ProofValidator),
            "srp" => Ok(crate::lifecycle::semantic::AmendmentClass::Review),
            "sor" => Ok(crate::lifecycle::semantic::AmendmentClass::Implementation),
            _ => Err("intent_semantic_card_kind_invalid"),
        })
        .collect::<Result<std::collections::BTreeSet<_>, _>>()?;
    if inferred_classes.len() != 1 || !inferred_classes.contains(&amendment.class) {
        return Err("intent_amendment_class_mismatch".into());
    }
    if amendment.class == crate::lifecycle::semantic::AmendmentClass::Implementation
        && amendment.implementation_revision.as_deref() != Some(native.expected_head_sha.as_str())
    {
        return Err("intent_amendment_revision_mismatch".into());
    }
    let phase = if binding_advances {
        match crate::lifecycle::semantic::decide_amendment(
            preflight.phase(),
            crate::lifecycle::semantic::AmendmentClass::Binding,
            &crate::lifecycle::semantic::AmendmentFacts {
                source_version_current: true,
                issue_checkout_match: true,
                evidence_integrity: true,
                transition_approved: true,
                topology: true,
                new_commit: true,
                ..Default::default()
            },
        ) {
            crate::lifecycle::semantic::AmendmentOutcome::Admitted { phase, .. } => phase,
            _ => return Err("intent_semantic_binding_stale".into()),
        }
    } else {
        preflight.phase()
    };
    let amendment_facts = crate::lifecycle::semantic::AmendmentFacts {
        source_version_current: true,
        issue_checkout_match: true,
        evidence_integrity: true,
        transition_approved: amendment.transition_approved,
        topology: preflight.inputs().binding().is_some(),
        implementation_revision: amendment
            .implementation_revision
            .as_deref()
            .is_some_and(|revision| revision == native.expected_head_sha),
        current_proof: matches!(
            phase,
            crate::lifecycle::LifecycleState::Implemented
                | crate::lifecycle::LifecycleState::Reviewed
                | crate::lifecycle::LifecycleState::Published
                | crate::lifecycle::LifecycleState::MergeReady
        ),
        independent_review: matches!(
            phase,
            crate::lifecycle::LifecycleState::Reviewed
                | crate::lifecycle::LifecycleState::Published
                | crate::lifecycle::LifecycleState::MergeReady
        ),
        projection_change: cards != *preflight.inputs().cards(),
        new_commit: amendment.new_commit,
    };
    if !matches!(
        crate::lifecycle::semantic::decide_amendment(phase, amendment.class, &amendment_facts,),
        crate::lifecycle::semantic::AmendmentOutcome::Admitted { .. }
    ) {
        return Err("intent_amendment_policy_rejected".into());
    }
    let admitted_cards = cards;
    if context.refresh_semantic_binding()? {
        super::rebuild_semantic_card_projection(context)?;
    }
    let refreshed = Context::load(&context.root, context.issue)?;
    let semantic = refreshed.semantic_context()?;
    let mut cards = semantic.snapshot.inputs().cards().clone();
    for (kind, update) in &request.card_updates {
        let card = cards
            .get_mut(kind)
            .ok_or("intent_semantic_card_kind_invalid")?;
        merge(card, update);
    }
    if semantic.snapshot.phase() != phase
        || cards != admitted_cards
        || !matches!(
            crate::lifecycle::semantic::decide_amendment(
                semantic.snapshot.phase(),
                amendment.class,
                &amendment_facts,
            ),
            crate::lifecycle::semantic::AmendmentOutcome::Admitted { .. }
        )
    {
        return Err("intent_amendment_admission_changed_after_binding_refresh".into());
    }
    // Render/validate every prospective value before committing authority. A
    // stale generated view is not an input to this amendment.
    for (kind, values) in &cards {
        local::render_semantic_card_projection(registry, kind, values)
            .map_err(|error| format!("intent_card_values_invalid:{error}"))?;
    }
    commit_local_amendment(
        &refreshed,
        semantic,
        LocalChange::AmendVerifiedCards(VerifiedCardAmendment::from_native_owner(
            cards,
            amendment.class,
            amendment_facts,
        )),
    )
}

fn semantic_bind(
    context: &Context,
    request: &LocalPreparationRequest,
    registry: &local::PromptRegistry,
    native: &local::OperationalLocalContext,
) -> Result<Value, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, semantic::Binding, DurableTransactionStore};
    let diagnostic = local::execute_operational_local_route("doctor", request, registry, native)
        .map_err(errors)?;
    if diagnostic
        .findings
        .iter()
        .any(|finding| finding.status != PlanStatus::Passed)
    {
        return Err(errors(diagnostic.findings));
    }
    // Explicit bind is the recovery route even when retained validators are no
    // longer admitted. Refresh only the exact registered checkout, never run proof.
    let refreshed = context.refresh_semantic_binding()?;
    if refreshed {
        semantic_rebuild(context, registry)?;
    }
    let semantic = context.semantic_context()?;
    if semantic.snapshot.inputs().binding().is_some()
        && semantic.snapshot.phase() != crate::lifecycle::LifecycleState::Ready
    {
        if !matches!(
            semantic.snapshot.phase(),
            crate::lifecycle::LifecycleState::Bound
                | crate::lifecycle::LifecycleState::Implemented
                | crate::lifecycle::LifecycleState::Reviewed
                | crate::lifecycle::LifecycleState::Published
                | crate::lifecycle::LifecycleState::MergeReady
        ) {
            return Err("intent_bind_phase_invalid".into());
        }
        return Ok(json!({"schema":"csdlc.v3.intent_local.v1",
            "status":if refreshed {"completed"} else {"expected_noop"},
            "read_only":!refreshed,"writes_v3_state":refreshed,"operational_authority":true,
            "semantic_version":semantic.snapshot.version(),"binding_refreshed":refreshed,
            "validators_run":false}));
    }
    let target = Binding {
        branch: request.branch.clone(),
        head: context.head.clone(),
        worktree: std::path::PathBuf::from(&request.worktree),
        registration: blake3::hash(
            serde_json::to_string(&json!({"branch":request.branch,"worktree":request.worktree}))
                .map_err(|_| "intent_bind_identity_invalid")?
                .as_bytes(),
        )
        .to_hex()
        .to_string(),
    };
    let origin = semantic
        .origin
        .with_bind_target(target.clone())
        .map_err(semantic_error)?;
    let bytes = serde_json::to_vec(&json!({
        "schema":"csdlc.v3.semantic_bind_request.v1",
        "repository":context.repository,"issue":context.issue,
        "semantic_version":semantic.snapshot.version(),"target":target,
        "native_request":request
    }))
    .map_err(|_| "intent_bind_identity_invalid")?;
    let identity = NativeIdentity::new(
        "csdlc-v3-local-bind".into(),
        blake3::hash(&bytes).to_hex().to_string(),
    )
    .map_err(semantic_error)?;
    let effect = EffectRequest::new(
        SemanticCommand::Bind,
        identity.clone(),
        origin.clone(),
        &bytes,
    )
    .map_err(semantic_error)?;
    let attachment_origin = origin.clone();
    let admission = EffectAdmission::from_native_owner(
        semantic.admission.clone(),
        origin,
        Facts {
            bind_target: true,
            topology: true,
            ..Default::default()
        },
    );
    context.repair_before_effect(&semantic.snapshot, &effect)?;
    let ticket = match DurableTransactionStore::reserve_effect(&semantic.root, admission, effect)
        .map_err(semantic_error)?
    {
        Reservation::AlreadyCompleted(done) => {
            return Ok(
                json!({"schema":"csdlc.v3.intent_local.v1","status":"expected_noop",
                "read_only":true,"writes_v3_state":false,"operational_authority":true,
                "semantic_version":done.current_version(),"operation_id":done.operation_id().as_str()}),
            )
        }
        Reservation::AlreadyPending(ticket) => {
            return Ok(
                json!({"schema":"csdlc.v3.intent_local.v1","status":"recovery_required",
                "read_only":true,"writes_v3_state":false,"operational_authority":true,
                "operation_id":ticket.id().as_str()}),
            )
        }
        Reservation::Reserved(ticket) => ticket,
    };
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_bind_after_reservation")
    {
        std::process::exit(91);
    }
    semantic.admit_before_effect(ticket.id())?;
    // Rebinding a retained checkout revalidates its native ownership/cards;
    // it must not invoke the create/move binder again on native Bound state.
    let native_route = if semantic.snapshot.inputs().binding().is_some() {
        "doctor"
    } else {
        "bind"
    };
    let native_result =
        local::execute_operational_local_route(native_route, request, registry, native);
    let (mut kind, truth, evidence, facts) = match native_result {
        Ok(result)
            if result
                .findings
                .iter()
                .any(|finding| finding.status != PlanStatus::Passed) =>
        {
            (
                OutcomeKind::Failure,
                EffectTruth::NotPerformed,
                serde_json::to_vec(&result).map_err(|_| "intent_bind_result_invalid")?,
                Facts::default(),
            )
        }
        Ok(result) => (
            OutcomeKind::Success,
            if result.mutated {
                EffectTruth::Performed
            } else {
                EffectTruth::NotPerformed
            },
            serde_json::to_vec(&result).map_err(|_| "intent_bind_result_invalid")?,
            Facts {
                bind_target: true,
                topology: true,
                ..Default::default()
            },
        ),
        Err(findings) => (
            OutcomeKind::Unresolved,
            EffectTruth::Unknown,
            serde_json::to_vec(&findings).map_err(|_| "intent_bind_result_invalid")?,
            Facts::default(),
        ),
    };
    let attachment = match semantic.fresh_for_effect(ticket.id()) {
        Ok(admission) => admission,
        Err(_) => {
            kind = OutcomeKind::Unresolved;
            AttachmentAdmission::from_native_owner(
                semantic.snapshot.inputs().authority().clone(),
                attachment_origin,
            )
        }
    };
    let outcome = VerifiedOutcome::from_native_owner(kind, truth, evidence, facts, identity)
        .map_err(semantic_error)?;
    match DurableTransactionStore::attach_outcome(&semantic.root, ticket, outcome, attachment)
        .map_err(semantic_error)?
    {
        Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
            let snapshot =
                match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
                    .map_err(semantic_error)?
                {
                    crate::storage::semantic::Observation::Current(value)
                    | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => {
                        *value
                    }
                    _ => return Err("intent_bind_semantic_state_unavailable".into()),
                };
            let projected = semantic.complete_projection(&snapshot)?;
            let bound = Context::load(&context.primary, context.issue)?;
            semantic_rebuild(&bound, registry)?;
            Ok(
                json!({"schema":"csdlc.v3.intent_local.v1","status":if kind == OutcomeKind::Success {"completed"} else {"failed"},"read_only":false,
                "writes_v3_state":true,"operational_authority":true,"semantic_version":projected.version(),
                "operation_id":done.operation_id().as_str(),"native_effect_truth":done.truth()}),
            )
        }
        Attachment::RecoveryRequired(version) => Ok(json!({"schema":"csdlc.v3.intent_local.v1",
            "status":"recovery_required","read_only":false,"writes_v3_state":true,
            "operational_authority":true,"semantic_version":version})),
    }
}

fn prepare(context: &Context, value: &Value) -> Result<Value, String> {
    let legacy_native = if !context.index.is_null() {
        let (root, key) = context.semantic_root_key()?;
        match crate::storage::DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            crate::storage::semantic::Observation::LegacyMigrationRequired => true,
            crate::storage::semantic::Observation::RecoveryRequired => {
                return Err("intent_semantic_recovery_required".into())
            }
            crate::storage::semantic::Observation::Absent => {
                return Err("intent_native_state_without_semantic_classification".into())
            }
            crate::storage::semantic::Observation::Current(snapshot)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(snapshot)
                if snapshot.phase() == crate::lifecycle::LifecycleState::Bound
                    && snapshot.inputs().binding().is_some() =>
            {
                true
            }
            crate::storage::semantic::Observation::Current(_)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(_) => {
                return Err("issue_already_initialized".into())
            }
        }
    } else {
        false
    };
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
    crate::commands::remote::intent::validate_publication_metadata(
        context.issue,
        &format!("codex/{}-{}", context.issue, plan.slug),
        &plan.publication.base,
        &plan.publication.title,
        &plan.publication.body,
    )
    .map_err(|finding| finding.code)?;
    crate::commands::proof::intent::admit_validator_declarations(&context.root, &plan.validators)?;
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
    let expected_lifecycle_digest = legacy_native
        .then(|| {
            context.index["digest"]
                .as_str()
                .filter(|value| !value.is_empty())
                .map(str::to_owned)
                .ok_or("intent_legacy_lifecycle_digest_missing")
        })
        .transpose()?;
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
        expected_lifecycle_digest,
        commands: local::required_local_commands().to_vec(),
        card_updates: plan.cards.clone(),
        schedule_readiness: None,
        shepherd_routing: None,
    };
    // A bound legacy issue already has an authenticated checkout identity.
    // Preparation may refresh its plan slug, but it must not reinterpret that
    // slug as a request to move the retained branch or worktree.
    if legacy_native && context.index["phase"] == "bound" {
        request.branch.clone_from(&context.branch);
        request.worktree = context.root.to_string_lossy().into_owned();
    }
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
    let legacy_binding = if legacy_native && context.index["phase"] == "bound" {
        let primary_binding = read_json(
            &context
                .git_common
                .join(format!("csdlc-v3/local/bindings/{}.json", context.issue)),
        )?;
        let linked_binding = read_json(&context.issue_root.join("binding.json"))?;
        let exact = |binding: &Value| {
            binding["schema"] == "csdlc.v3.binding.v1"
                && binding["issue"] == context.issue
                && binding["branch"] == context.branch
                && binding["worktree"].as_str() == context.root.to_str()
        };
        if !exact(&primary_binding) || !exact(&linked_binding) || primary_binding != linked_binding
        {
            return Err("intent_bound_legacy_binding_mismatch".into());
        }
        Some(crate::storage::semantic::Binding {
            branch: context.branch.clone(),
            head: context.head.clone(),
            worktree: context.root.clone(),
            registration: blake3::hash(
                serde_json::to_string(&json!({"branch":context.branch,"worktree":context.root}))
                    .map_err(|_| "intent_bind_identity_invalid")?
                    .as_bytes(),
            )
            .to_hex()
            .to_string(),
        })
    } else {
        None
    };
    let prepared = local::intent::prepare_semantic(
        &request,
        &registry,
        &native,
        value,
        local::intent::SemanticPrepareTarget {
            root: &root,
            key,
            authority: context.semantic_authority()?,
            legacy_binding,
        },
    )
    .map_err(errors)?;
    let snapshot = prepared.snapshot;
    let native_result = (!legacy_native)
        .then(|| local::execute_operational_local_route("issue", &request, &registry, &native));
    if let Some(Err(findings)) = native_result {
        return Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
            "operational_authority":true,"writes_v3_state":true,"status":"recovery_required",
            "issue":context.issue,"semantic_version":snapshot.version(),"native_prepare_findings":findings}),
        );
    }
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_prepare_after_activation")
    {
        std::process::exit(91);
    }
    match context.complete_semantic_projection(&snapshot) {
        Ok(current) => {
            if let Some(fence) = prepared.adoption_fence {
                if let Err(error) = fence.release() {
                    return Ok(
                        json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
                        "operational_authority":true,"writes_v3_state":true,"status":"recovery_required",
                        "issue":context.issue,"semantic_version":current.version(),
                        "adoption_fence":{"release_required":true,"finding":error}}),
                    );
                }
            }
            Ok(
                json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
                "operational_authority":true,"writes_v3_state":true,"status":"completed",
                "issue":context.issue,"semantic_version":current.version(),"inputs":current.inputs_version(),"phase":current.phase()}),
            )
        }
        Err(error) => Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,
            "operational_authority":true,"writes_v3_state":true,"status":"recovery_required",
            "issue":context.issue,"semantic_version":snapshot.version(),"projection_repair":{"required":true,"finding":error}}),
        ),
    }
}

fn storage_validators(
    validators: Vec<Validator>,
) -> Result<Vec<crate::storage::semantic::Validator>, String> {
    validators
        .into_iter()
        .map(|validator| {
            serde_json::from_value(
                serde_json::to_value(validator).map_err(|_| "intent_validator_invalid")?,
            )
            .map_err(|_| "intent_validator_invalid".into())
        })
        .collect()
}

fn semantic_validation_edit(
    context: &Context,
    validators: Vec<Validator>,
) -> Result<Value, String> {
    crate::commands::proof::intent::admit_semantic_validator_declarations(context, &validators)?;
    let storage_validators = storage_validators(validators)?;
    let refreshed = context.refresh_semantic_binding()?;
    if refreshed {
        semantic_rebuild(context, &context.registry()?)?;
    }
    let semantic = context.semantic_context()?;
    if semantic.snapshot.inputs().validation() == storage_validators.as_slice() {
        return Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","status":"expected_noop",
            "read_only":!refreshed,"writes_v3_state":refreshed,"performed_mutation":refreshed,"operational_authority":true,
            "semantic_version":semantic.snapshot.version(),
            "inputs":semantic.snapshot.inputs_version()}),
        );
    }
    commit_local_amendment(
        context,
        semantic,
        crate::storage::semantic::LocalChange::AmendValidation(storage_validators),
    )
}

fn validate_publication_edit(context: &Context, publication: &Publication) -> Result<(), String> {
    let semantic = context.semantic_context()?;
    let inputs = semantic.snapshot.inputs();
    let branch = inputs
        .binding()
        .map(|binding| binding.branch.clone())
        .unwrap_or_else(|| format!("codex/{}-{}", context.issue, inputs.slug()));
    crate::commands::remote::intent::validate_publication_metadata(
        context.issue,
        &branch,
        &publication.base,
        &publication.title,
        &publication.body,
    )
    .map_err(|finding| finding.code)?;
    let original = inputs.publication();
    // A prior remote publication still exists after proof/review invalidation.
    // Derive this boundary from retained completions, not only the current phase.
    if publication.base != original.base || publication.draft != original.draft {
        for done in semantic.snapshot.completed() {
            let inspected = crate::storage::DurableTransactionStore::inspect_effect(
                &semantic.root,
                &semantic.key,
                done.id(),
            )
            .map_err(semantic_error)?;
            if inspected.request().command() == crate::lifecycle::semantic::SemanticCommand::Publish
                && !(done.outcome() == crate::storage::semantic::protocol::OutcomeKind::Failure
                    && done.truth()
                        == crate::storage::semantic::protocol::EffectTruth::NotPerformed)
            {
                return Err("intent_publication_topology_change_denied".into());
            }
        }
    }
    Ok(())
}

fn semantic_publication_edit(
    context: &Context,
    publication: Publication,
    expected: &super::Snapshot,
) -> Result<Value, String> {
    let semantic = context.semantic_context()?;
    // Preserve the caller's semantic version, including emitted requests. The
    // same admitted snapshot is carried into reservation, whose CAS check also
    // rejects changes racing this preflight.
    let admitted_version = super::IssueVersion {
        generation: Some(semantic.snapshot.version().generation()),
        digest: Some(semantic.snapshot.version().digest().as_str().to_owned()),
    };
    if expected.semantic_version.as_ref() != Some(&admitted_version) {
        return Err("intent_publication_stale_semantic_version".into());
    }
    if semantic.snapshot.pending().is_some() {
        return Err(semantic_error(
            crate::storage::semantic::Error::PendingOperation,
        ));
    }
    validate_publication_edit(context, &publication)?;
    let publication: crate::storage::semantic::Publication = serde_json::from_value(
        serde_json::to_value(publication).map_err(|_| "intent_publication_invalid")?,
    )
    .map_err(|_| "intent_publication_invalid")?;
    if semantic.snapshot.inputs().publication() == &publication {
        return Ok(
            json!({"schema":"csdlc.v3.intent_local.v1","status":"expected_noop","publication_amended":false,
            "read_only":true,"writes_v3_state":false,"operational_authority":true,
            "semantic_version":semantic.snapshot.version(),
            "inputs":semantic.snapshot.inputs_version()}),
        );
    }
    let mut result = commit_local_amendment(
        context,
        semantic,
        crate::storage::semantic::LocalChange::AmendPublication(publication),
    )?;
    result["publication_amended"] = json!(true);
    Ok(result)
}

fn require_semantic_version(
    expected: &super::Snapshot,
    snapshot: &crate::storage::semantic::Snapshot,
) -> Result<(), String> {
    let version = super::IssueVersion {
        generation: Some(snapshot.version().generation()),
        digest: Some(snapshot.version().digest().as_str().to_owned()),
    };
    if expected.semantic_version.as_ref() != Some(&version) {
        return Err("intent_stale_semantic_version".into());
    }
    Ok(())
}

fn commit_local_amendment(
    context: &Context,
    semantic: super::context::SemanticContext,
    change: crate::storage::semantic::LocalChange,
) -> Result<Value, String> {
    use crate::storage::{semantic::CommitOutcome, DurableTransactionStore};
    context.fresh_integrity()?;
    context.repair_interrupted_projection(&semantic.snapshot)?;
    let outcome = DurableTransactionStore::commit_issue_local(
        &semantic.root,
        semantic.admission.clone(),
        change,
    )
    .map_err(semantic_error)?;
    let (snapshot, changed) = match outcome {
        CommitOutcome::Committed(snapshot) => (*snapshot, true),
        CommitOutcome::Unchanged(snapshot) => (*snapshot, false),
    };
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_local_amendment_after_commit")
    {
        std::process::exit(91);
    }
    semantic.complete_projection(&snapshot)?;
    let projection = semantic_rebuild_current(context, &context.registry()?)?;
    Ok(json!({
        "schema":"csdlc.v3.intent_local.v1",
        "status":if changed {"completed"} else {"expected_noop"},
        "read_only":!changed && projection["read_only"] == true,
        "writes_v3_state":changed || projection["writes_v3_state"] == true,
        "operational_authority":true,
        "semantic_version":projection["semantic_version"],
        "inputs":snapshot.inputs_version(),
        "projection":projection["projection"],
        "local_transaction":true
    }))
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
        let validators = context
            .plan()?
            .validators
            .into_iter()
            .map(|value| super::Validator {
                id: value.id,
                program: value.program,
                args: value.args,
                success_marker: value.success_marker,
                timeout_seconds: value.timeout_seconds,
            })
            .collect::<Vec<_>>();
        // Complete every no-effect validator/input admission check before a
        // changed checkout HEAD amends the semantic binding or reserves proof.
        crate::commands::proof::intent::admit_semantic_validators(context, &validators)?;
        if context.refresh_semantic_binding()? {
            super::rebuild_semantic_card_projection(context)?;
        }
        let refreshed = Context::load(&context.primary, context.issue)?;
        semantic_proof(&refreshed)
    }
}

pub(crate) fn recover_semantic_projection(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    use crate::storage::{
        semantic::{CardProjectionObservation, Observation, ProjectionWriteProof},
        DurableTransactionStore,
    };
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            Observation::ProjectionRepairRequired(snapshot) | Observation::Current(snapshot) => {
                *snapshot
            }
            _ => return Ok(None),
        };
    // A pending business operation owns recovery. Stale generated cards must
    // not intercept bind/proof/remote reconciliation.
    if snapshot.pending().is_some() {
        return Ok(None);
    }
    let proof = retained_current_proof_receipt(context, &root, &key, &snapshot)?;
    let proof_projection_required = proof.as_ref().is_some_and(|receipt| {
        let path = context
            .root
            .join(crate::commands::remote::intent::semantic_proof_path(
                context.issue,
            ));
        match read_json(&path) {
            Ok(current) => current != *receipt,
            Err(_) => true,
        }
    });
    let registry = context.registry()?;
    let bundle = crate::application::derive_semantic_card_projection(&snapshot, &registry)
        .map_err(|error| format!("intent_semantic_projection_derivation_failed:{error}"))?;
    let card_observation =
        DurableTransactionStore::observe_card_projection(&root, &snapshot, &bundle)
            .map_err(semantic_error)?;
    let card_projection_required = card_observation != CardProjectionObservation::Healthy;
    if ProjectionWriteProof::verify(&root, &snapshot).is_ok()
        && !proof_projection_required
        && !card_projection_required
    {
        return Ok(None);
    }
    let token = if card_projection_required {
        let identity = serde_json::to_vec(&json!({
            "audit":snapshot.audit_identity(),
            "projection":bundle.projection_digest(),
            "observation":card_observation,
        }))
        .map_err(|_| "intent_projection_recovery_identity_invalid")?;
        blake3::hash(&identity).to_hex().to_string()
    } else {
        snapshot.audit_identity().as_str().to_owned()
    };
    if !request.execute {
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"action":"repair_semantic_projection",
            "preview_digest":token,"semantic_version":snapshot.version(),
            "card_projection":card_observation})));
    }
    if request.preview.as_deref() != Some(token.as_str()) {
        return Err("intent_projection_recovery_preview_stale".into());
    }
    if let Some(receipt) = proof {
        crate::commands::proof::intent::write_semantic_proof_projection(context, &receipt)?;
    }
    if card_projection_required {
        DurableTransactionStore::write_card_projection(&root, &snapshot, bundle)
            .map_err(semantic_error)?;
    }
    let repaired = context.complete_semantic_projection(&snapshot)?;
    Ok(Some(
        json!({"status":"completed","read_only":false,"performed_mutation":true,
        "action":"repaired_semantic_projection","semantic_version":repaired.version()}),
    ))
}

fn retained_current_proof_receipt(
    context: &Context,
    root: &crate::storage::semantic::SemanticRoot,
    key: &crate::storage::semantic::IssueKey,
    snapshot: &crate::storage::semantic::Snapshot,
) -> Result<Option<Value>, String> {
    use crate::lifecycle::semantic::SemanticCommand;
    use crate::storage::DurableTransactionStore;
    for completed in snapshot.completed().iter().rev() {
        let inspection = DurableTransactionStore::inspect_effect(root, key, completed.id())
            .map_err(semantic_error)?;
        if inspection.request().command() != SemanticCommand::RecordProof {
            continue;
        }
        let Some(bytes) = inspection.evidence() else {
            return Ok(None);
        };
        let evidence: Value =
            serde_json::from_slice(bytes).map_err(|_| "intent_retained_proof_invalid")?;
        if evidence["schema"] != "csdlc.v3.semantic_proof_evidence.v1"
            || evidence["repository"] != context.repository
            || evidence["issue"] != context.issue
            || evidence["head"] != context.head
            || evidence["issue_digest"] != context.index["digest"]
        {
            return Ok(None);
        }
        return crate::commands::proof::intent::receipt_from_semantic_execution(
            context,
            &evidence["execution"],
        )
        .map(Some);
    }
    Ok(None)
}

pub(crate) fn recover_semantic_bind(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, DurableTransactionStore};
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            crate::storage::semantic::Observation::Current(value)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => *value,
            _ => return Ok(None),
        };
    let Some(pending) = snapshot.pending() else {
        return Ok(None);
    };
    if pending.command() != SemanticCommand::Bind {
        return Ok(None);
    }
    let preview = DurableTransactionStore::describe_effect_recovery(&root, &key)
        .map_err(semantic_error)?
        .ok_or("intent_bind_pending_missing")?;
    if !request.execute {
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"action":"reconcile_native_bind",
            "operation_id":pending.id().as_str(),"preview_digest":preview.digest().as_str()})));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_bind_recovery_preview_stale".into());
    }
    let inspection = DurableTransactionStore::inspect_effect(&root, &key, pending.id())
        .map_err(semantic_error)?;
    let retained: Value = serde_json::from_slice(
        &inspection
            .request()
            .canonical_content()
            .map_err(semantic_error)?,
    )
    .map_err(|_| "intent_bind_retained_request_invalid")?;
    let target: crate::storage::semantic::Binding =
        serde_json::from_value(retained["target"].clone())
            .map_err(|_| "intent_bind_retained_target_invalid")?;
    let native_request: LocalPreparationRequest =
        serde_json::from_value(retained["native_request"].clone())
            .map_err(|_| "intent_bind_retained_native_request_invalid")?;
    if native_request.issue != context.issue
        || native_request.repository != context.repository
        || native_request.branch != target.branch
        || native_request.worktree != target.worktree.to_string_lossy()
    {
        return Err("intent_bind_retained_native_identity_mismatch".into());
    }
    let recovery_source_exists =
        local::intent::recovery_source(&context.git_common.join("csdlc-v3/local"), context.issue)
            .map_err(errors)?
            .is_some();
    let registration = super::context::git(&context.primary, &["worktree", "list", "--porcelain"])?;
    let canonical_target = target.worktree.canonicalize().ok();
    let branch_ref = format!("refs/heads/{}", target.branch);
    let registered = registration.split("\n\n").any(|record| {
        record.lines().any(|line| {
            line.strip_prefix("worktree ")
                .and_then(|path| std::path::Path::new(path).canonicalize().ok())
                .is_some_and(|path| Some(path) == canonical_target)
        }) && record
            .lines()
            .any(|line| line == format!("branch {branch_ref}"))
    });
    let binding_state_root = if recovery_source_exists {
        context.git_common.join("csdlc-v3/local")
    } else if target.worktree.exists() {
        local::operational_state_root(&target.worktree).map_err(errors)?
    } else {
        target.worktree.join(".git/csdlc-v3/local")
    };
    let mut binding_path = binding_state_root.join(format!("bindings/{}.json", context.issue));
    let mut definitely_absent = !binding_path.exists() && !target.worktree.exists() && !registered;
    let mut observed_context = if recovery_source_exists || definitely_absent {
        Context::load(&context.primary, context.issue)?
    } else {
        Context::load(&target.worktree, context.issue)?
    };
    let mut native_bound = !definitely_absent
        && binding_path.exists()
        && read_json(&binding_path).is_ok_and(|binding| {
            binding["issue"] == context.issue
                && binding["branch"] == target.branch
                && binding["worktree"].as_str() == target.worktree.to_str()
        })
        && observed_context.root == target.worktree
        && observed_context.branch == target.branch
        && observed_context.head == target.head;
    if !native_bound && !definitely_absent && recovery_source_exists {
        let native = local::discover_operational_local_context(&context.primary, &native_request)
            .map_err(errors)?
            .ok_or("intent_operational_authority_required")?;
        let native_preview =
            local::intent::recover(&native_request, &native, false, None).map_err(errors)?;
        let native_digest = native_preview["preview_digest"]
            .as_str()
            .ok_or("intent_native_recovery_preview_missing")?;
        local::intent::recover(&native_request, &native, true, Some(native_digest))
            .map_err(errors)?;
        observed_context = Context::load(&context.primary, context.issue)?;
        binding_path = binding_state_root.join(format!("bindings/{}.json", context.issue));
        native_bound = binding_path.exists()
            && read_json(&binding_path).is_ok_and(|binding| {
                binding["issue"] == context.issue
                    && binding["branch"] == target.branch
                    && binding["worktree"].as_str() == target.worktree.to_str()
            })
            && observed_context.root == target.worktree
            && observed_context.branch == target.branch
            && observed_context.head == target.head;
        let registration =
            super::context::git(&context.primary, &["worktree", "list", "--porcelain"])?;
        let canonical_target = target.worktree.canonicalize().ok();
        let registered = registration.split("\n\n").any(|record| {
            record.lines().any(|line| {
                line.strip_prefix("worktree ")
                    .and_then(|path| std::path::Path::new(path).canonicalize().ok())
                    .is_some_and(|path| Some(path) == canonical_target)
            }) && record
                .lines()
                .any(|line| line == format!("branch {branch_ref}"))
        });
        definitely_absent = !binding_path.exists() && !target.worktree.exists() && !registered;
    }
    if !native_bound && !definitely_absent && !binding_path.exists() && registered {
        let native = local::discover_operational_local_context(&target.worktree, &native_request)
            .map_err(errors)?
            .ok_or("intent_operational_authority_required")?;
        let registry = context.registry()?;
        let native_result =
            local::execute_operational_local_route("bind", &native_request, &registry, &native)
                .map_err(errors)?;
        if !native_result
            .findings
            .iter()
            .any(|finding| finding.status != PlanStatus::Passed)
        {
            observed_context = Context::load(&target.worktree, context.issue)?;
            binding_path = binding_state_root.join(format!("bindings/{}.json", context.issue));
            native_bound = binding_path.exists()
                && read_json(&binding_path).is_ok_and(|binding| {
                    binding["issue"] == context.issue
                        && binding["branch"] == target.branch
                        && binding["worktree"].as_str() == target.worktree.to_str()
                })
                && observed_context.root == target.worktree
                && observed_context.branch == target.branch
                && observed_context.head == target.head;
        }
    }
    let (kind, truth, facts, evidence) = if native_bound {
        (
            OutcomeKind::Success,
            EffectTruth::Performed,
            Facts {
                bind_target: true,
                topology: true,
                ..Default::default()
            },
            json!({"schema":"csdlc.v3.semantic_bind_reconciliation.v1","status":"bound","target":target}),
        )
    } else if definitely_absent {
        (
            OutcomeKind::Failure,
            EffectTruth::NotPerformed,
            Facts {
                bind_target: true,
                topology: true,
                ..Default::default()
            },
            json!({"schema":"csdlc.v3.semantic_bind_reconciliation.v1","status":"absent","target":target}),
        )
    } else {
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":null,"operation_id":pending.id().as_str(),
            "reason":"native_bind_partial_or_ambiguous"})));
    };
    let semantic = observed_context.semantic_recovery_context(pending.id())?;
    let admission = if definitely_absent {
        AttachmentAdmission::from_native_owner(
            semantic.snapshot.inputs().authority().clone(),
            inspection.request().origin().clone(),
        )
    } else {
        match semantic.fresh_for_effect(pending.id()) {
            Ok(value) => value,
            Err(error) => {
                return Ok(Some(json!({"status":"recovery_required","read_only":true,
                    "performed_mutation":null,"operation_id":pending.id().as_str(),
                    "reason":"bind_target_changed_before_attachment","finding":error})))
            }
        }
    };
    let outcome = VerifiedOutcome::from_native_owner(
        kind,
        truth,
        serde_json::to_vec(&evidence).map_err(|_| "intent_bind_recovery_evidence_invalid")?,
        facts,
        inspection.request().native_identity().clone(),
    )
    .map_err(semantic_error)?;
    let attached = DurableTransactionStore::attach_outcome(
        &root,
        inspection
            .ticket()
            .ok_or("intent_bind_pending_ticket_missing")?
            .clone(),
        outcome,
        admission,
    )
    .map_err(semantic_error)?;
    let completed = match attached {
        Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => done,
        Attachment::RecoveryRequired(_) => {
            return Ok(Some(json!({"status":"recovery_required","read_only":false,
                "performed_mutation":false,"operation_id":pending.id().as_str()})))
        }
    };
    let current =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            crate::storage::semantic::Observation::Current(value)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => *value,
            _ => return Err("intent_bind_semantic_state_unavailable".into()),
        };
    let projected = semantic.complete_projection(&current)?;
    semantic_rebuild(context, &context.registry()?)?;
    Ok(Some(json!({"status":"completed",
        "read_only":false,"performed_mutation":false,"operation_id":completed.operation_id().as_str(),
        "semantic_outcome":kind,
        "native_effect_truth":truth,"semantic_version":projected.version()})))
}

pub(crate) fn recover_semantic_edit(
    context: &Context,
    request: &IntentRequest,
) -> Result<Option<Value>, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, DurableTransactionStore};
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            crate::storage::semantic::Observation::Current(value)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => *value,
            _ => return Ok(None),
        };
    let Some(pending) = snapshot.pending() else {
        return Ok(None);
    };
    let command = pending.command();
    if !matches!(
        command,
        SemanticCommand::AmendCards
            | SemanticCommand::AmendValidation
            | SemanticCommand::AmendPublication
    ) {
        return Ok(None);
    }
    let pending_id = pending.id().clone();
    let semantic = context.semantic_recovery_context(&pending_id)?;
    let pending = semantic
        .snapshot
        .pending()
        .ok_or("intent_edit_pending_missing")?;
    let preview = DurableTransactionStore::describe_effect_recovery(&semantic.root, &semantic.key)
        .map_err(semantic_error)?
        .ok_or("intent_edit_pending_missing")?;
    if !request.execute {
        let action = if command == SemanticCommand::AmendCards {
            "reconcile_native_edit"
        } else if command == SemanticCommand::AmendPublication {
            "reconcile_publication_edit"
        } else {
            "reconcile_validation_edit"
        };
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"action":action,
            "operation_id":pending.id().as_str(),"preview_digest":preview.digest().as_str()})));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_edit_recovery_preview_stale".into());
    }
    let inspection =
        DurableTransactionStore::inspect_effect(&semantic.root, &semantic.key, pending.id())
            .map_err(semantic_error)?;
    let retained: Value = serde_json::from_slice(
        &inspection
            .request()
            .canonical_content()
            .map_err(semantic_error)?,
    )
    .map_err(|_| "intent_edit_retained_request_invalid")?;
    if command == SemanticCommand::AmendValidation {
        if retained["schema"] != "csdlc.v3.semantic_validation_edit_request.v1"
            || retained["repository"] != context.repository
            || retained["issue"] != context.issue
        {
            return Err("intent_validation_edit_retained_identity_mismatch".into());
        }
        let validators: Vec<Validator> = serde_json::from_value(retained["validators"].clone())
            .map_err(|_| "intent_validation_edit_retained_validators_invalid")?;
        crate::commands::proof::intent::admit_validators(&context.root, &validators)?;
        semantic.admit_before_effect(pending.id())?;
        let outcome = VerifiedOutcome::from_native_owner(
            OutcomeKind::Success,
            EffectTruth::NotPerformed,
            serde_json::to_vec(&json!({
                "schema":"csdlc.v3.semantic_validation_edit_outcome.v1",
                "validators_amended":true,
                "recovered":true
            }))
            .map_err(|_| "intent_validation_edit_result_invalid")?,
            Facts::default(),
            inspection.request().native_identity().clone(),
        )
        .map_err(semantic_error)?;
        let admission = semantic
            .fresh_for_effect(pending.id())
            .map_err(|_| "intent_validation_edit_semantic_state_changed".to_string())?;
        let attached = DurableTransactionStore::execute_effect_recovery(
            &semantic.root,
            preview,
            outcome,
            admission,
        )
        .map_err(semantic_error)?;
        return Ok(Some(match attached {
            Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
                let snapshot = match DurableTransactionStore::observe_issue(
                    &semantic.root,
                    &semantic.key,
                )
                .map_err(semantic_error)?
                {
                    crate::storage::semantic::Observation::Current(value)
                    | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => {
                        *value
                    }
                    _ => return Err("intent_validation_edit_semantic_state_unavailable".into()),
                };
                let projected = semantic.complete_projection(&snapshot)?;
                semantic_rebuild_current(context, &context.registry()?)?;
                json!({"status":"completed","read_only":false,"performed_mutation":true,
                    "operation_id":done.operation_id().as_str(),
                    "native_effect_truth":done.truth(),
                    "semantic_outcome":done.outcome_kind(),
                    "semantic_version":projected.version(),"inputs":projected.inputs_version()})
            }
            Attachment::RecoveryRequired(version) => {
                json!({"status":"recovery_required","read_only":false,
                "performed_mutation":false,"operation_id":pending.id().as_str(),
                "semantic_version":version})
            }
        }));
    }
    if command == SemanticCommand::AmendPublication {
        if retained["schema"] != "csdlc.v3.semantic_publication_edit_request.v1"
            || retained["repository"] != context.repository
            || retained["issue"] != context.issue
        {
            return Err("intent_publication_edit_retained_identity_mismatch".into());
        }
        let publication: Publication = serde_json::from_value(retained["publication"].clone())
            .map_err(|_| "intent_publication_edit_retained_invalid")?;
        crate::commands::remote::intent::validate_publication_metadata(
            context.issue,
            &semantic
                .snapshot
                .inputs()
                .binding()
                .map(|binding| binding.branch.clone())
                .unwrap_or_else(|| {
                    format!(
                        "codex/{}-{}",
                        context.issue,
                        semantic.snapshot.inputs().slug()
                    )
                }),
            &publication.base,
            &publication.title,
            &publication.body,
        )
        .map_err(|finding| finding.code)?;
        semantic.admit_before_effect(pending.id())?;
        let outcome = VerifiedOutcome::from_native_owner(
            OutcomeKind::Success,
            EffectTruth::NotPerformed,
            serde_json::to_vec(&json!({
                "schema":"csdlc.v3.semantic_publication_edit_outcome.v1",
                "publication_amended":true,
                "recovered":true
            }))
            .map_err(|_| "intent_publication_edit_result_invalid")?,
            Facts::default(),
            inspection.request().native_identity().clone(),
        )
        .map_err(semantic_error)?;
        let admission = semantic
            .fresh_for_effect(pending.id())
            .map_err(|_| "intent_publication_edit_semantic_state_changed".to_string())?;
        let attached = DurableTransactionStore::execute_effect_recovery(
            &semantic.root,
            preview,
            outcome,
            admission,
        )
        .map_err(semantic_error)?;
        return Ok(Some(match attached {
            Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
                let snapshot = match DurableTransactionStore::observe_issue(
                    &semantic.root,
                    &semantic.key,
                )
                .map_err(semantic_error)?
                {
                    crate::storage::semantic::Observation::Current(value)
                    | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => {
                        *value
                    }
                    _ => return Err("intent_publication_edit_semantic_state_unavailable".into()),
                };
                let projected = semantic.complete_projection(&snapshot)?;
                semantic_rebuild_current(context, &context.registry()?)?;
                json!({"status":"completed","read_only":false,"performed_mutation":true,
                    "operation_id":done.operation_id().as_str(),
                    "native_effect_truth":done.truth(),
                    "semantic_outcome":done.outcome_kind(),
                    "semantic_version":projected.version(),"inputs":projected.inputs_version()})
            }
            Attachment::RecoveryRequired(version) => {
                json!({"status":"recovery_required","read_only":false,
                "performed_mutation":false,"operation_id":pending.id().as_str(),
                "semantic_version":version})
            }
        }));
    }
    if retained["schema"] != "csdlc.v3.semantic_edit_request.v1"
        || retained["repository"] != context.repository
        || retained["issue"] != context.issue
    {
        return Err("intent_edit_retained_identity_mismatch".into());
    }
    let native_request: LocalPreparationRequest =
        serde_json::from_value(retained["native_request"].clone())
            .map_err(|_| "intent_edit_retained_native_request_invalid")?;
    let registry = context.registry()?;
    let native = owner(context, &native_request)?;
    semantic.admit_before_effect(pending.id())?;
    let native_result =
        local::execute_operational_local_route("edit", &native_request, &registry, &native);
    let (mut kind, truth, evidence) = match &native_result {
        Ok(result) => (
            OutcomeKind::Success,
            if result.mutated {
                EffectTruth::Performed
            } else {
                EffectTruth::NotPerformed
            },
            serde_json::to_vec(result).map_err(|_| "intent_edit_result_invalid")?,
        ),
        Err(findings) => (
            OutcomeKind::Unresolved,
            EffectTruth::Unknown,
            serde_json::to_vec(findings).map_err(|_| "intent_edit_result_invalid")?,
        ),
    };
    let admission = match semantic.fresh_for_effect(pending.id()) {
        Ok(value) => value,
        Err(_) => {
            kind = OutcomeKind::Unresolved;
            AttachmentAdmission::from_native_owner(
                semantic.snapshot.inputs().authority().clone(),
                inspection.request().origin().clone(),
            )
        }
    };
    let outcome = VerifiedOutcome::from_native_owner(
        kind,
        truth,
        evidence,
        Facts::default(),
        inspection.request().native_identity().clone(),
    )
    .map_err(semantic_error)?;
    let attached = DurableTransactionStore::execute_effect_recovery(
        &semantic.root,
        preview,
        outcome,
        admission,
    )
    .map_err(semantic_error)?;
    Ok(Some(match attached {
        Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
            let snapshot =
                match DurableTransactionStore::observe_issue(&semantic.root, &semantic.key)
                    .map_err(semantic_error)?
                {
                    crate::storage::semantic::Observation::Current(value)
                    | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => {
                        *value
                    }
                    _ => return Err("intent_edit_semantic_state_unavailable".into()),
                };
            let projected = semantic.complete_projection(&snapshot)?;
            semantic_rebuild_current(context, &context.registry()?)?;
            json!({"status":"completed","read_only":false,"performed_mutation":true,
                "action":"reconciled_native_edit","operation_id":done.operation_id().as_str(),
                "native_effect_truth":truth,"semantic_version":projected.version(),
                "result":native_result.map_err(errors)?})
        }
        Attachment::RecoveryRequired(version) => json!({"status":"recovery_required",
            "read_only":false,"performed_mutation":true,"operation_id":pending.id().as_str(),
            "native_effect_truth":truth,"semantic_version":version}),
    }))
}

#[cfg(unix)]
fn semantic_proof(context: &Context) -> Result<Value, String> {
    use crate::lifecycle::semantic::{Facts, SemanticCommand};
    use crate::storage::{semantic::protocol::*, DurableTransactionStore};
    let admitted_context = context.semantic_context()?;
    crate::commands::proof::intent::admit_semantic_proof_projection(context)?;
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
    let admitted = crate::commands::proof::intent::admit_semantic_validators(context, &validators)?;
    let producer: Value = serde_json::from_slice(&admitted.request_bytes()?)
        .map_err(|_| "intent_validator_request_invalid")?;
    let administrative_epoch = latest_administrative_epoch(&admitted_context)?;
    let attempt_predecessor = latest_abandoned_proof_attempt(&admitted_context)?;
    let mut proof_request = json!({"schema":"csdlc.v3.semantic_proof_request.v1",
        "inputs":admitted_context.snapshot.inputs_version(),"administrative_epoch":administrative_epoch,
        "producer":producer});
    if let Some(predecessor) = attempt_predecessor {
        proof_request["attempt_predecessor"] = predecessor.into();
    }
    let request_bytes = serde_json::to_vec(&proof_request)
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
    context.repair_before_effect(&admitted_context.snapshot, &request)?;
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
        Reservation::AlreadyCompleted(done) => {
            return completed_proof_with_receipt(context, &admitted_context, &done, true)
        }
        Reservation::AlreadyPending(ticket) => {
            return Ok(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"operation_id":ticket.id().as_str(),"reason":"explicit_original_proof_recovery_required"}))
        }
        Reservation::Reserved(ticket) => ticket,
    };
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref()
        == Ok("semantic_proof_after_reservation")
    {
        std::process::exit(91);
    }
    let execution = crate::commands::proof::intent::execute_admitted(&admitted, || {
        admitted_context.admit_before_effect(ticket.id())?;
        crate::commands::proof::intent::admit_semantic_proof_projection(context)
    });
    #[cfg(debug_assertions)]
    if std::env::var("CSDLC_V3_TEST_CRASH_POINT").as_deref() == Ok("semantic_proof_after_execution")
    {
        std::process::exit(91);
    }
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
        "repository":context.repository,"head":context.head,"issue_digest":context.index["digest"],
        "inputs":admitted_context.snapshot.inputs_version(),
        "validation_inputs":admitted_context.snapshot.inputs().validation_digest().map_err(semantic_error)?,
        "administrative_epoch":administrative_epoch,
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
        Ok(Attachment::Completed(done)) => {
            finish_proof_projection(context, &admitted_context, &done, &evidence["execution"])
        }
        Ok(Attachment::AlreadyCompleted(done)) => Ok(completed_proof(&done, true)),
        Ok(Attachment::RecoveryRequired(version)) => Ok(json!({"status":"recovery_required","read_only":false,
            "performed_mutation":true,"operation_id":ticket.id().as_str(),"semantic_version":version,
            "native_effect_truth":execution.effect_truth,"proof":evidence})),
        Err(error) => Err(json!({"status":"recovery_required","read_only":false,"performed_mutation":true,
            "operation_id":ticket.id().as_str(),"native_effect_truth":execution.effect_truth,"proof":evidence,
            "finding":semantic_error(error),"evidence_persistence":"not_confirmed"}).to_string()),
    }
}

fn latest_abandoned_proof_attempt(
    context: &super::context::SemanticContext,
) -> Result<Option<String>, String> {
    use crate::lifecycle::semantic::SemanticCommand;
    use crate::storage::{semantic::protocol::OutcomeKind, DurableTransactionStore};
    for completed in context.snapshot.completed().iter().rev() {
        let inspection =
            DurableTransactionStore::inspect_effect(&context.root, &context.key, completed.id())
                .map_err(semantic_error)?;
        if inspection.request().command() != SemanticCommand::RecordProof {
            continue;
        }
        let Some(bytes) = inspection.evidence() else {
            return Ok(None);
        };
        let evidence: Value =
            serde_json::from_slice(bytes).map_err(|_| "intent_retained_proof_invalid")?;
        if completed.outcome() == OutcomeKind::Failure
            && completed.truth() == crate::storage::semantic::protocol::EffectTruth::Unknown
            && evidence["schema"] == "csdlc.v3.semantic_proof_abandonment.v1"
        {
            return Ok(Some(completed.id().as_str().to_owned()));
        }
        let content: Value = serde_json::from_slice(
            &inspection
                .request()
                .canonical_content()
                .map_err(semantic_error)?,
        )
        .map_err(|_| "intent_retained_proof_request_invalid")?;
        return Ok(content["attempt_predecessor"].as_str().map(str::to_owned));
    }
    Ok(None)
}

fn latest_administrative_epoch(context: &super::context::SemanticContext) -> Result<Value, String> {
    use crate::lifecycle::semantic::SemanticCommand;
    use crate::storage::{semantic::protocol::OutcomeKind, DurableTransactionStore};
    for completed in context.snapshot.completed().iter().rev() {
        if completed.outcome() != OutcomeKind::Success {
            continue;
        }
        let inspection =
            DurableTransactionStore::inspect_effect(&context.root, &context.key, completed.id())
                .map_err(semantic_error)?;
        if matches!(
            inspection.request().command(),
            SemanticCommand::RecordInstall
                | SemanticCommand::RecordCutover
                | SemanticCommand::RecordRollback
        ) {
            return Ok(Value::String(completed.id().as_str().into()));
        }
    }
    Ok(Value::Null)
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

pub(crate) fn semantic_proof_current(context: &Context) -> Result<bool, String> {
    use crate::lifecycle::semantic::SemanticCommand;
    use crate::storage::{semantic::protocol::OutcomeKind, DurableTransactionStore};
    let admitted = context.semantic_context()?;
    let administrative_epoch = latest_administrative_epoch(&admitted)?;
    if admitted.snapshot.pending().is_some_and(|p| {
        matches!(
            p.command(),
            SemanticCommand::RecordProof
                | SemanticCommand::AmendPublication
                | SemanticCommand::RecordInstall
                | SemanticCommand::RecordCutover
                | SemanticCommand::RecordRollback
        )
    }) {
        return Ok(false);
    }
    for completed in admitted.snapshot.completed().iter().rev() {
        let inspection =
            DurableTransactionStore::inspect_effect(&admitted.root, &admitted.key, completed.id())
                .map_err(semantic_error)?;
        if completed.outcome() == OutcomeKind::Success
            && matches!(
                inspection.request().command(),
                SemanticCommand::RecordInstall
                    | SemanticCommand::RecordCutover
                    | SemanticCommand::RecordRollback
            )
        {
            // Administrative changes invalidate prior proof without changing the
            // issue-input digest. Walking newest-first makes the invalidation
            // causal: a later proof is current, while an older proof is not.
            return Ok(false);
        }
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
        let expected_inputs = serde_json::to_value(admitted.snapshot.inputs_version())
            .map_err(|_| "intent_input_version_invalid")?;
        if evidence["schema"] != "csdlc.v3.semantic_proof_evidence.v1"
            || evidence["repository"] != context.repository
            || evidence["issue"] != context.issue
            || evidence["head"] != context.head
            || (evidence["inputs"] != expected_inputs
                && evidence["validation_inputs"]
                    != serde_json::to_value(
                        admitted
                            .snapshot
                            .inputs()
                            .validation_digest()
                            .map_err(semantic_error)?,
                    )
                    .map_err(|_| "intent_input_version_invalid")?)
            || evidence["administrative_epoch"] != administrative_epoch
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
        let current = Context::load(&context.root, context.issue)?;
        let proof_current = crate::commands::proof::intent::verify_semantic_execution_inputs(
            &current,
            &validators,
            &evidence["execution"],
        );
        return Ok(proof_current.is_ok());
    }
    Ok(false)
}

fn completed_proof(done: &crate::storage::semantic::protocol::Completion, replay: bool) -> Value {
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
    let (root, key) = context.semantic_root_key()?;
    let snapshot =
        match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
            crate::storage::semantic::Observation::Current(value)
            | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => *value,
            _ => return Ok(None),
        };
    let Some(pending) = snapshot.pending() else {
        if request.execute
            && !request.content.is_null()
            && request.content["schema"] == "csdlc.v3.semantic_proof_recovery_disposition.v1"
            && request.content["action"] == "abandon_indeterminate_proof"
        {
            let operation_id = request.content["operation_id"]
                .as_str()
                .ok_or("intent_proof_recovery_disposition_invalid")?;
            if let Some(done) = snapshot
                .completed()
                .iter()
                .find(|done| done.id().as_str() == operation_id)
            {
                let inspection = DurableTransactionStore::inspect_effect(&root, &key, done.id())
                    .map_err(semantic_error)?;
                let evidence: Value = serde_json::from_slice(
                    inspection
                        .evidence()
                        .ok_or("intent_proof_abandonment_evidence_missing")?,
                )
                .map_err(|_| "intent_proof_abandonment_evidence_invalid")?;
                if done.outcome() == OutcomeKind::Failure
                    && done.truth() == EffectTruth::Unknown
                    && inspection.request().command() == SemanticCommand::RecordProof
                    && evidence["schema"] == "csdlc.v3.semantic_proof_abandonment.v1"
                    && evidence["operation_id"] == operation_id
                    && evidence["preview_digest"].as_str() == request.preview.as_deref()
                    && evidence["disposition"] == "abandoned_indeterminate"
                {
                    return Ok(Some(json!({"status":"expected_noop","read_only":true,
                        "performed_mutation":false,"action":"abandoned_indeterminate_proof",
                        "operation_id":operation_id,"native_effect_truth":done.truth(),
                        "semantic_outcome":done.outcome()})));
                }
                return Err("intent_proof_recovery_disposition_mismatch".into());
            }
        }
        return Ok(None);
    };
    if pending.command() != SemanticCommand::RecordProof {
        return Ok(None);
    }
    let id = pending.id().clone();
    let admitted = context.semantic_recovery_context(&id)?;
    let preview = DurableTransactionStore::describe_effect_recovery(&admitted.root, &admitted.key)
        .map_err(semantic_error)?
        .ok_or("intent_proof_pending_missing")?;
    let inspection = DurableTransactionStore::inspect_effect(&admitted.root, &admitted.key, &id)
        .map_err(semantic_error)?;
    if !request.content.is_null() && inspection.evidence().is_some() {
        return Err("intent_proof_recovery_disposition_not_applicable".into());
    }
    if !request.execute {
        let action = if inspection.evidence().is_some() {
            "attach_retained_proof_only"
        } else {
            "abandon_indeterminate_proof"
        };
        return Ok(Some(json!({"status":"recovery_required","read_only":true,
            "performed_mutation":false,"operation_id":id.as_str(),
            "preview_digest":preview.digest().as_str(),"action":action,
            "required_disposition":if inspection.evidence().is_none(){
                json!({"flag":"--disposition","schema":"csdlc.v3.semantic_proof_recovery_disposition.v1",
                    "fields":["schema","action","operation_id","rationale"]})
            }else{Value::Null}})));
    }
    if request.preview.as_deref() != Some(preview.digest().as_str()) {
        return Err("intent_proof_recovery_preview_stale".into());
    }
    let Some(bytes) = inspection.evidence() else {
        if request.content.is_null() {
            return Ok(Some(json!({"status":"recovery_required","read_only":true,
                "performed_mutation":false,"operation_id":id.as_str(),
                "reason":"proof_outcome_unavailable_requires_explicit_abandonment",
                "required_disposition_schema":"csdlc.v3.semantic_proof_recovery_disposition.v1"})));
        }
        if request.content["schema"] != "csdlc.v3.semantic_proof_recovery_disposition.v1"
            || request.content["action"] != "abandon_indeterminate_proof"
            || request.content["operation_id"] != id.as_str()
            || request.content["rationale"]
                .as_str()
                .is_none_or(|value| value.trim().is_empty())
        {
            return Err("intent_proof_recovery_disposition_invalid".into());
        }
        let evidence = json!({
            "schema":"csdlc.v3.semantic_proof_abandonment.v1",
            "repository":context.repository,"issue":context.issue,"head":context.head,
            "issue_digest":context.index["digest"],
            "inputs":admitted.snapshot.inputs_version(),
            "operation_id":id.as_str(),
            "native_identity":inspection.request().native_identity(),
            "preview_digest":preview.digest().as_str(),
            "disposition":"abandoned_indeterminate",
            "effect_truth":"unknown",
            "rationale":request.content["rationale"]
        });
        let outcome = VerifiedOutcome::from_native_owner(
            OutcomeKind::Failure,
            EffectTruth::Unknown,
            serde_json::to_vec(&evidence)
                .map_err(|_| "intent_proof_abandonment_serialization_failed")?,
            Facts::default(),
            inspection.request().native_identity().clone(),
        )
        .map_err(semantic_error)?;
        let observed = admitted.fresh_for_effect(&id)?;
        let resolution = VerifiedRecoveryResolution::abandon_indeterminate_proof(&preview);
        let result = DurableTransactionStore::execute_effect_recovery_with_resolution(
            &admitted.root,
            preview,
            outcome,
            observed,
            Some(resolution),
        )
        .map_err(semantic_error)?;
        return Ok(Some(match result {
            Attachment::Completed(done) | Attachment::AlreadyCompleted(done) => {
                let snapshot = match DurableTransactionStore::observe_issue(
                    &admitted.root,
                    &admitted.key,
                )
                .map_err(semantic_error)?
                {
                    crate::storage::semantic::Observation::Current(value)
                    | crate::storage::semantic::Observation::ProjectionRepairRequired(value) => {
                        *value
                    }
                    _ => return Err("intent_proof_semantic_state_unavailable".into()),
                };
                let projected = admitted.complete_projection(&snapshot)?;
                semantic_rebuild_current(context, &context.registry()?)?;
                json!({"status":"completed","read_only":false,"performed_mutation":true,
                    "action":"abandoned_indeterminate_proof","operation_id":done.operation_id().as_str(),
                    "native_effect_truth":done.truth(),"semantic_outcome":done.outcome_kind(),
                    "semantic_version":projected.version()})
            }
            Attachment::RecoveryRequired(version) => json!({"status":"recovery_required",
                "read_only":false,"performed_mutation":true,"operation_id":id.as_str(),
                "semantic_version":version}),
        }));
    };
    let evidence: Value =
        serde_json::from_slice(bytes).map_err(|_| "intent_retained_proof_invalid")?;
    if evidence["schema"] != "csdlc.v3.semantic_proof_evidence.v1"
        || evidence["issue"] != context.issue
        || evidence["repository"] != context.repository
        || evidence["head"] != context.head
        || evidence["issue_digest"] != context.index["digest"]
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
        crate::commands::proof::intent::verify_semantic_execution_inputs(
            context,
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
        Attachment::Completed(done) => {
            finish_proof_projection(context, &admitted, &done, &evidence["execution"])?
        }
        Attachment::AlreadyCompleted(done) => {
            completed_proof_with_receipt(context, &admitted, &done, true)?
        }
        Attachment::RecoveryRequired(version) => {
            json!({"status":"recovery_required","read_only":false,
            "performed_mutation":true,"operation_id":id.as_str(),"semantic_version":version})
        }
    }))
}

fn finish_proof_projection(
    native: &Context,
    context: &super::context::SemanticContext,
    done: &crate::storage::semantic::protocol::Completion,
    proof: &Value,
) -> Result<Value, String> {
    use crate::storage::{semantic::Observation, DurableTransactionStore};
    let receipt = crate::commands::proof::intent::receipt_from_semantic_execution(native, proof)?;
    let evidence_ref = crate::commands::remote::intent::semantic_proof_path(native.issue);
    let projection =
        crate::commands::proof::intent::write_semantic_proof_projection(native, &receipt);
    let mut value = completed_proof(done, false);
    value["proof"] = receipt;
    value["evidence_ref"] = evidence_ref.into();
    if let Err(error) = projection {
        value["status"] = json!("recovery_required");
        value["evidence_persisted"] = json!(false);
        value["projection_repair"] = json!({"required":true,"finding":error});
        return Ok(value);
    }
    value["evidence_persisted"] = json!(true);
    let result = DurableTransactionStore::observe_issue(&context.root, &context.key)
        .map_err(semantic_error)
        .and_then(|observation| match observation {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                context.complete_projection(&snapshot)
            }
            _ => Err("intent_proof_projection_state_unavailable".into()),
        });
    match result {
        Ok(snapshot) => {
            value["current_version"] = serde_json::to_value(snapshot.version())
                .map_err(|_| "semantic_version_serialization_failed")?;
            semantic_rebuild(native, &native.registry()?)?;
        }
        Err(error) => {
            value["status"] = json!("recovery_required");
            value["projection_repair"] = json!({"required":true,"finding":error});
        }
    }
    Ok(value)
}

fn completed_proof_with_receipt(
    native: &Context,
    context: &super::context::SemanticContext,
    done: &crate::storage::semantic::protocol::Completion,
    replay: bool,
) -> Result<Value, String> {
    let receipt =
        retained_current_proof_receipt(native, &context.root, &context.key, &context.snapshot)?
            .ok_or("intent_retained_proof_missing")?;
    let reference = crate::commands::remote::intent::semantic_proof_path(native.issue);
    let persisted = read_json(&native.root.join(&reference)).is_ok_and(|value| value == receipt);
    let mut value = completed_proof(done, replay);
    value["proof"] = receipt;
    value["evidence_ref"] = reference.into();
    value["evidence_persisted"] = persisted.into();
    if !persisted {
        value["status"] = json!("recovery_required");
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
        assert!(crate::commands::proof::intent::cleanup_complete(
            std::slice::from_ref(&complete)
        ));
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
