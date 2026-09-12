//! Resolve remote intent inputs once, then invoke the production remote owner.
use super::{Context, IntentRequest};
use crate::adapters::{EnvironmentCredentialResolver, RealProcessAdapter};
use crate::commands::remote::{intent as owner, *};
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RecoveryOperation {
    operation: Value,
    recovery: GithubMutationRecovery,
}

fn parse_operation(value: Value) -> Result<GithubMutation, String> {
    if value.get("action").and_then(Value::as_str) == Some("pull_request_merge") {
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct MergeIntent {
            action: String,
            base: String,
            method: MergeMethod,
            operator_approval: String,
        }
        let merge: MergeIntent =
            serde_json::from_value(value).map_err(|_| "intent_merge_internal_inputs_denied")?;
        if merge.action != "pull_request_merge" || merge.operator_approval.trim().is_empty() {
            return Err("intent_remote_operation_invalid".into());
        }
        return Ok(GithubMutation::PullRequestMerge {
            base: merge.base,
            method: merge.method,
            review_receipt_path: String::new(),
            review_receipt_digest: String::new(),
        });
    }
    serde_json::from_value(value).map_err(|_| "intent_remote_operation_invalid".into())
}

fn failure(finding: RemoteRouteFinding) -> String {
    finding.code
}
fn issue_digest(context: &Context) -> Result<&str, String> {
    context.index["digest"]
        .as_str()
        .ok_or_else(|| "intent_issue_digest_required".into())
}
fn route(context: &Context) -> Result<RemoteRouteRequest, String> {
    serde_json::from_value(
        json!({"repository":context.repository,"issue":context.issue,
        "expected_head_sha":context.head,"head_sha":context.head,
        "credential_names":["GITHUB_TOKEN"],"mode":"closing"}),
    )
    .map_err(|_| "intent_remote_request_invalid".into())
}
fn target(context: &Context) -> Result<Option<u64>, String> {
    owner::publication_target(
        &context.root,
        &context.repository,
        context.issue,
        &context.branch,
        &context.head,
    )
    .map_err(failure)
}
fn evidence(context: &Context) -> Result<owner::ExternalReview, String> {
    let evidence = owner::load_external_review(&context.root, context.issue, &context.head)
        .map_err(failure)?;
    owner::verify_external_review(
        &context.root,
        &context.repository,
        context.issue,
        &context.head,
        issue_digest(context)?,
        &evidence,
    )
    .map_err(failure)?;
    Ok(evidence)
}
fn reviewed_route(
    context: &Context,
    evidence: &owner::ExternalReview,
) -> Result<RemoteRouteRequest, String> {
    let mut route = route(context)?;
    route.implementer = Some(evidence.receipt.implementer.clone());
    route.reviewer = Some(evidence.receipt.reviewer.clone());
    route.review_revision = Some(evidence.receipt.reviewed_revision.clone());
    route.review_present = true;
    route.typed_review_receipt_path = Some(owner::review_path(context.issue, &context.head));
    route.typed_review_receipt_digest = Some(evidence.receipt_digest.clone());
    Ok(route)
}
fn observation(
    context: &Context,
    route: &RemoteRouteRequest,
    review: Option<&owner::ExternalReview>,
    command: &str,
) -> Result<Value, String> {
    context.fresh_integrity()?;
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let mut observed = observe_github_pr_readback(route, &mut process).map_err(failure)?;
    if let Some(review) = review {
        observed.receipts.typed_review = Some(review.receipt.clone());
    }
    let plan = prepare_remote_publication_route_with_receipts(
        command,
        &observed.request,
        &observed.receipts,
    )
    .map_err(failure)?;
    Ok(json!({"plan":plan,"request":observed.request,"readback":observed.receipts.github_readback}))
}
fn mutation_request(
    context: &Context,
    operation: GithubMutation,
    recovery: Option<GithubMutationRecovery>,
    pull_request: Option<u64>,
    operator_approval: Option<String>,
) -> GithubMutationRequest {
    GithubMutationRequest {
        repository: context.repository.clone(),
        issue: if matches!(operation, GithubMutation::IssueCreate { .. }) {
            0
        } else {
            context.issue
        },
        pull_request,
        cutover_issue: None,
        operator_approval,
        expected_head_sha: context.head.clone(),
        credential_names: vec!["GITHUB_TOKEN".into()],
        recovery,
        mutation: operation,
    }
}
fn mutation(
    context: &Context,
    operation: GithubMutation,
    recovery: Option<GithubMutationRecovery>,
    pull_request: Option<u64>,
    operator_approval: Option<String>,
) -> Result<Value, String> {
    context.fresh_integrity()?;
    let native = mutation_request(
        context,
        operation,
        recovery,
        pull_request,
        operator_approval,
    );
    owner::validate_intent_mutation(&context.root, &native).map_err(failure)?;
    let dispatch = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: context.authority_digest.clone(),
        exact_review_sha: context.head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(native),
    };
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    let publication_base = if matches!(&dispatch.operation,OperationalRemoteOperation::GithubMutation(request) if matches!(request.mutation,GithubMutation::PullRequestUpdate{..}|GithubMutation::PullRequestReady))
    {
        context.plan()?.publication.base
    } else {
        String::new()
    };
    // Read-only preflight reports a definite refusal without intent/receipt writes.
    if let OperationalRemoteOperation::GithubMutation(request) = &dispatch.operation {
        if matches!(
            request.mutation,
            GithubMutation::PullRequestUpdate { .. } | GithubMutation::PullRequestReady
        ) {
            owner::verify_publication_target(
                request,
                &publication_base,
                &context.branch,
                &mut process,
            )
            .map_err(failure)?;
        }
    }
    context.fresh_integrity()?;
    match owner::dispatch_intent_mutation(
        &context.root,
        &dispatch,
        &publication_base,
        &context.branch,
        &mut process,
    ) {
        Ok(result) => {
            let (performed, status) = match &result.outcome {
                OperationalRemoteOutcome::GithubMutation(outcome) => (
                    outcome.performed_mutation,
                    if outcome.performed_mutation == Some(false) {
                        "expected_noop"
                    } else {
                        "completed"
                    },
                ),
                _ => (None, "failed"),
            };
            Ok(
                json!({"status":status,"operational_authority":true,"read_only":performed == Some(false),"performed_mutation":performed,"result":result}),
            )
        }
        // A transport failure can follow intent persistence or remote dispatch.
        // Preserve uncertainty instead of converting the error to read-only failure.
        Err(finding) => Ok(
            json!({"status":"recovery_required","operational_authority":true,
            "read_only":false,"performed_mutation":null,"effects_unknown":true,"findings":[finding]}),
        ),
    }
}

pub fn run(context: &Context, request: &IntentRequest) -> Result<Value, String> {
    context.fresh_integrity()?;
    if let Some(preview) = &request.preview {
        if preview != "plan" || request.execute {
            return Err("intent_remote_preview_invalid".into());
        }
    }
    match request.command.as_str() {
        "review" => {
            if request.execute {
                return Err("intent_review_arguments_invalid".into());
            }
            let evidence: owner::ExternalReview = serde_json::from_value(request.content.clone())
                .map_err(|_| "intent_external_review_invalid")?;
            owner::verify_external_review(
                &context.root,
                &context.repository,
                context.issue,
                &context.head,
                issue_digest(context)?,
                &evidence,
            )
            .map_err(failure)?;
            if request.preview.is_some() {
                return Ok(
                    json!({"status":"ready","read_only":true,"operational_authority":false,"review_revision":context.head}),
                );
            }
            context.fresh_integrity()?;
            match owner::record_external_review(
                &context.root,
                &context.repository,
                context.issue,
                &context.head,
                issue_digest(context)?,
                &context.authority_digest,
                &evidence,
            ) {
                Ok(wrote) => Ok(
                    json!({"status":if wrote {"completed"} else {"expected_noop"},"read_only":!wrote,"operational_authority":true,"performed_mutation":wrote,"review_receipt_path":owner::review_path(context.issue,&context.head),"review_receipt_digest":evidence.receipt_digest}),
                ),
                Err(finding) => Ok(
                    json!({"status":"recovery_required","read_only":false,"operational_authority":true,"performed_mutation":null,"effects_unknown":true,"findings":[finding]}),
                ),
            }
        }
        "pr-state" => {
            if !request.content.is_null() || request.execute {
                return Err("intent_remote_unexpected_content".into());
            }
            let mut route = route(context)?;
            route.pull_request = Some(target(context)?.ok_or("intent_publication_target_missing")?);
            let result = observation(context, &route, None, "pr-state")?;
            Ok(
                json!({"status":result["plan"]["status"],"read_only":true,"operational_authority":true,"result":result}),
            )
        }
        "publish" => {
            if !request.content.is_null() || request.execute {
                return Err("intent_remote_unexpected_content".into());
            }
            let plan = context.plan()?;
            let review = evidence(context)?;
            let mut route = reviewed_route(context, &review)?;
            route.title = Some(plan.publication.title.clone());
            route.body = Some(plan.publication.body.clone());
            owner::publication_create_admission(&route, &review).map_err(failure)?;
            let pull_request = target(context)?;
            if request.preview.is_some() {
                return Ok(
                    json!({"status":"ready","read_only":true,"operational_authority":false,"pull_request":pull_request,"publication":plan.publication}),
                );
            }
            let operation = match pull_request {
                None => GithubMutation::PullRequestCreate {
                    base: plan.publication.base,
                    head: context.branch.clone(),
                    title: plan.publication.title,
                    body: plan.publication.body,
                    draft: plan.publication.draft,
                },
                Some(_) => GithubMutation::PullRequestUpdate {
                    title: Some(plan.publication.title),
                    body: Some(plan.publication.body),
                },
            };
            let mut outcome = mutation(context, operation, None, pull_request, None)?;
            if outcome["status"] == "recovery_required" {
                return Ok(outcome);
            }
            route.pull_request = outcome["result"]["outcome"]["result"]["receipt"]["pull_request"]
                .as_u64()
                .or(pull_request);
            match observation(context, &route, Some(&review), "publish") {
                Ok(observed) => {
                    if observed["plan"]["status"] != "ready" {
                        outcome["status"] = "blocked".into();
                    }
                    outcome["publication_observation"] = observed;
                }
                Err(code) => {
                    outcome["status"] = "recovery_required".into();
                    outcome["findings"] = json!([{"code":code,"message":"publication mutation completed but authenticated admission readback failed"}]);
                }
            }
            Ok(outcome)
        }
        "github-issue" | "github-pr" => {
            let operator_content = request.content.get("operation").unwrap_or(&request.content);
            let operator_approval = operator_content
                .get("operator_approval")
                .and_then(Value::as_str)
                .map(str::to_owned);
            let (mut operation, recovery) = if request.content.get("operation").is_some() {
                let content: RecoveryOperation = serde_json::from_value(request.content.clone())
                    .map_err(|_| "intent_remote_recovery_content_invalid")?;
                (parse_operation(content.operation)?, Some(content.recovery))
            } else {
                (parse_operation(request.content.clone())?, None)
            };
            let issue_operation = matches!(
                operation,
                GithubMutation::IssueCreate { .. }
                    | GithubMutation::IssueComment { .. }
                    | GithubMutation::IssueEdit { .. }
                    | GithubMutation::IssueClose { .. }
            );
            if (request.command == "github-issue") != issue_operation {
                return Err("intent_remote_operation_family_mismatch".into());
            }
            let mut pull_request = None;
            if !issue_operation {
                let review = evidence(context)?;
                let plan = context.plan()?;
                let mut route = reviewed_route(context, &review)?;
                route.title = Some(plan.publication.title.clone());
                route.body = Some(plan.publication.body.clone());
                owner::publication_create_admission(&route, &review).map_err(failure)?;
                match &mut operation {
                    GithubMutation::PullRequestCreate {
                        base,
                        head,
                        title,
                        body,
                        draft,
                    } => {
                        if base != &plan.publication.base
                            || head != &context.branch
                            || title != &plan.publication.title
                            || body != &plan.publication.body
                            || *draft != plan.publication.draft
                        {
                            return Err("intent_publication_plan_mismatch".into());
                        }
                        if target(context)?.is_some() {
                            return Err("intent_publication_already_exists".into());
                        }
                    }
                    GithubMutation::PullRequestUpdate { title, body } => {
                        if title.as_ref().is_some_and(|value| value.trim().is_empty()) {
                            return Err("intent_publication_title_empty".into());
                        }
                        if let Some(body) = body {
                            route.body = Some(body.clone());
                            owner::publication_create_admission(&route, &review)
                                .map_err(failure)?;
                        }
                        pull_request =
                            Some(target(context)?.ok_or("intent_publication_target_missing")?);
                    }
                    GithubMutation::PullRequestMerge {
                        base,
                        review_receipt_path,
                        review_receipt_digest,
                        ..
                    } => {
                        if base != &plan.publication.base {
                            return Err("intent_publication_plan_mismatch".into());
                        }
                        let canonical = owner::review_path(context.issue, &context.head);
                        // Internal identity may be omitted as empty strings in the existing typed enum;
                        // nonempty caller identities must agree, never silently get refreshed.
                        if (!review_receipt_path.is_empty() && review_receipt_path != &canonical)
                            || (!review_receipt_digest.is_empty()
                                && review_receipt_digest != &review.receipt_digest)
                        {
                            return Err("intent_merge_review_identity_mismatch".into());
                        }
                        *review_receipt_path = canonical;
                        *review_receipt_digest = review.receipt_digest;
                        pull_request =
                            Some(target(context)?.ok_or("intent_publication_target_missing")?);
                    }
                    GithubMutation::PullRequestReady => {
                        pull_request =
                            Some(target(context)?.ok_or("intent_publication_target_missing")?)
                    }
                    _ => unreachable!(),
                }
            }
            let admission = mutation_request(
                context,
                operation.clone(),
                recovery.clone(),
                pull_request,
                operator_approval.clone(),
            );
            owner::validate_intent_mutation(&context.root, &admission).map_err(failure)?;
            if !request.execute || request.preview.is_some() {
                return Ok(
                    json!({"status":"ready","read_only":true,"operational_authority":false,"pull_request":pull_request,"operation":operation,"recovery":recovery}),
                );
            }
            mutation(
                context,
                operation,
                recovery,
                pull_request,
                operator_approval,
            )
        }
        _ => Err("intent_remote_command_unknown".into()),
    }
}

pub fn recover(context: &Context, request: &IntentRequest) -> Result<Option<Value>, String> {
    if !request.content.is_null() {
        return Err("intent_recover_unexpected_content".into());
    }
    let pending = owner::pending_operations(
        &context.root,
        &context.repository,
        context.issue,
        &context.head,
    )
    .map_err(failure)?;
    if pending.is_empty() {
        return Ok(None);
    }
    let local = context.local_request()?;
    let native = crate::commands::local::discover_operational_local_context(&context.root, &local)
        .map_err(|_| "intent_recovery_local_context_invalid")?
        .ok_or("intent_recovery_local_context_required")?;
    if crate::commands::local::intent::recovery_source(&native.state_root, context.issue)
        .map_err(|_| "intent_recovery_local_journal_invalid")?
        .is_some()
    {
        return Err("intent_recovery_multiple_owner_transactions: resolve the native local journal before remote recovery".into());
    }
    let packet = json!({"schema":"csdlc.v3.intent_remote_recovery_preview.v1","snapshot":request.snapshot,"pending":pending,"policy":"reconcile exact retained operation; permit one retry only after authenticated absence"});
    let digest =
        blake3::hash(&serde_json::to_vec(&packet).map_err(|_| "intent_recovery_preview_invalid")?)
            .to_hex()
            .to_string();
    if !request.execute {
        if request.preview.is_some() {
            return Err("intent_recovery_preview_argument_invalid".into());
        }
        return Ok(Some(
            json!({"status":"recovery_required","read_only":true,"performed_mutation":false,"preview_digest":digest,"pending":packet,"allowed_next":["recover"]}),
        ));
    }
    if request.preview.as_deref() != Some(digest.as_str()) {
        return Err("intent_recovery_preview_stale_or_missing".into());
    }
    if pending.len() != 1 {
        return Err("intent_recovery_ambiguous_remote_operations: explicit native operation selection required".into());
    }
    let mut retained: GithubMutationRequest = serde_json::from_value(pending[0]["request"].clone())
        .map_err(|_| "intent_recovery_request_invalid")?;
    retained.recovery = Some(GithubMutationRecovery::RetryAfterAuthenticatedAbsence);
    context.fresh_integrity()?;
    // The stored request is executed directly through the original transaction
    // owner. Its operation digest, resolved targets and retry budget stay intact.
    owner::validate_intent_mutation(&context.root, &retained).map_err(failure)?;
    let base = if matches!(
        retained.mutation,
        GithubMutation::PullRequestUpdate { .. } | GithubMutation::PullRequestReady
    ) {
        context.plan()?.publication.base
    } else {
        String::new()
    };
    let dispatch = OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: context.authority_digest.clone(),
        exact_review_sha: context.head.clone(),
        operation: OperationalRemoteOperation::GithubMutation(retained),
    };
    let mut process = RealProcessAdapter::new(EnvironmentCredentialResolver);
    match owner::dispatch_intent_mutation(
        &context.root,
        &dispatch,
        &base,
        &context.branch,
        &mut process,
    ) {
        Ok(result) => {
            let performed = match &result.outcome {
                OperationalRemoteOutcome::GithubMutation(value) => value.performed_mutation,
                _ => None,
            };
            Ok(Some(
                json!({"status":if performed==Some(false){"expected_noop"}else{"completed"},"read_only":performed==Some(false),"performed_mutation":performed,"operational_authority":true,"result":result}),
            ))
        }
        Err(finding) => Ok(Some(
            json!({"status":"recovery_required","read_only":false,"performed_mutation":null,"effects_unknown":true,"findings":[finding]}),
        )),
    }
}
