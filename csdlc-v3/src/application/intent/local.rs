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
        let proof = read_json(&context.root.join(format!(
            ".csdlc/evidence/{}/intent-proof.json",
            context.issue
        )))
        .ok();
        let proof_current = proof.as_ref().is_some_and(|value| {
            crate::commands::proof::intent::verify_current_inputs(&context.root, value).is_ok()
        });
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
    let result = local::intent::prepare(&request, &registry, &native, value).map_err(errors)?;
    Ok(
        json!({"schema":"csdlc.v3.intent_local.v1","read_only":false,"operational_authority":true,"writes_v3_state":true,"status":"completed","result":result}),
    )
}

pub fn proof(context: &Context, intent: &IntentRequest) -> Result<Value, String> {
    if !intent.content.is_null() {
        return Err("intent_unexpected_content".into());
    }
    let request = context.local_request()?;
    let native = owner(context, &request)?;
    let validation =
        local::execute_operational_local_route("validate", &request, &context.registry()?, &native)
            .map_err(errors)?;
    if validation
        .findings
        .iter()
        .any(|finding| finding.status != PlanStatus::Passed)
    {
        return Err(errors(validation.findings));
    }
    context.fresh()?;
    crate::commands::proof::intent::execute(context, &context.plan()?.validators)
}
