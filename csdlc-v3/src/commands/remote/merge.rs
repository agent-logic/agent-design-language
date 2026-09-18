//! Issue844: single-PR merge, exact review and authenticated fail-closed policy.
use std::{fs, path::Path};

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};
use fs2::FileExt;
use serde_json::{json, Value};

use super::model::*;
use super::publication::{
    load_optional_receipt, same_principal, typed_review_receipt_payload_digest,
};
use super::storage::*;
use super::support::*;
use super::transport::*;

// Closing only the parent's descriptor can leave the shared open-file lock held
// by a concurrently forked child until exec. Release ownership explicitly.
struct MergeLock(fs::File);
impl Drop for MergeLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}

fn reject(message: &str) -> RemoteRouteFinding {
    remote_finding("github_merge_ineligible", message)
}
fn ensure(ok: bool, message: &str) -> Result<(), RemoteRouteFinding> {
    if ok {
        Ok(())
    } else {
        Err(reject(message))
    }
}
fn text<'a>(value: &'a Value, key: &str) -> Result<&'a str, RemoteRouteFinding> {
    value[key]
        .as_str()
        .ok_or_else(|| reject("required observation field missing"))
}
fn complete_nodes(value: &Value) -> Result<&Vec<Value>, RemoteRouteFinding> {
    ensure(
        value["pageInfo"]["hasNextPage"] == false,
        "incomplete observation pagination",
    )?;
    value["nodes"]
        .as_array()
        .ok_or_else(|| reject("observation nodes missing"))
}
fn observe(
    request: &GithubMutationRequest,
    operation: &str,
    target: String,
    process: &mut impl ProcessAdapter,
) -> Result<(Value, CommandInvocation), RemoteRouteFinding> {
    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [operation.into(), request.repository.clone(), target],
    )
    .and_then(|i| i.with_child_credential(mutation_credential_name(request).unwrap_or_default()))
    .map_err(|_| reject("invalid authenticated observation"))?;
    let value = read_mutation_reconciliation_page(invocation.clone(), process)?;
    ensure(value.get("errors").is_none(), "GraphQL partial errors")?;
    Ok((value, invocation))
}
fn identity<'a>(
    value: &'a Value,
    request: &GithubMutationRequest,
    base: &str,
) -> Result<&'a Value, RemoteRouteFinding> {
    let repo = &value["data"]["repository"];
    let pr = &repo["pullRequest"];
    ensure(
        repo["nameWithOwner"] == request.repository
            && pr["number"].as_u64() == request.pull_request
            && pr["url"]
                == format!(
                    "https://github.com/{}/pull/{}",
                    request.repository,
                    request.pull_request.unwrap_or_default()
                )
            && pr["headRefOid"] == request.expected_head_sha
            && pr["baseRefName"] == base,
        "repository, PR, reviewed head or base mismatch",
    )?;
    Ok(pr)
}
fn eligibility(
    value: &Value,
    rules: &Value,
    request: &GithubMutationRequest,
    base: &str,
    linkage: &PublicationLinkage,
) -> Result<String, RemoteRouteFinding> {
    let pr = identity(value, request, base)?;
    linkage.validate(value, request, false)?;
    ensure(
        value["data"]["repository"]["mergeCommitAllowed"] == true,
        "merge commits disabled",
    )?;
    ensure(
        pr["state"] == "OPEN"
            && pr["merged"] == false
            && pr["isDraft"] == false
            && pr["mergeable"] == "MERGEABLE"
            && pr["mergeStateStatus"] == "CLEAN",
        "PR must be open, non-draft, clean and mergeable",
    )?;
    ensure(
        complete_nodes(&pr["reviewThreads"])?
            .iter()
            .all(|t| t["isResolved"] == true),
        "unresolved review thread",
    )?;
    ensure(
        complete_nodes(&pr["latestReviews"])?.iter().all(|r| {
            matches!(
                r["state"].as_str(),
                Some("APPROVED" | "COMMENTED" | "DISMISSED")
            )
        }),
        "unresolved or unknown review state",
    )?;
    let mut required: Vec<(String, Option<u64>)> = vec![];
    let mut review_required = false;
    let protection = pr["baseRef"]
        .get("branchProtectionRule")
        .ok_or_else(|| reject("branch policy missing"))?;
    if !protection.is_null() {
        ensure(
            protection["requiresLinearHistory"] == false,
            "linear history not supported",
        )?;
        review_required = protection["requiresApprovingReviews"]
            .as_bool()
            .ok_or_else(|| reject("review policy missing"))?;
        let status_required = protection["requiresStatusChecks"]
            .as_bool()
            .ok_or_else(|| reject("status policy missing"))?;
        let checks = protection["requiredStatusChecks"]
            .as_array()
            .ok_or_else(|| reject("required status policy missing"))?;
        ensure(
            !status_required || !checks.is_empty(),
            "required check denominator missing",
        )?;
        for check in checks {
            required.push((
                text(check, "context")?.into(),
                if check["app"].is_null() {
                    None
                } else {
                    Some(
                        check["app"]["databaseId"]
                            .as_u64()
                            .ok_or_else(|| reject("required app identity missing"))?,
                    )
                },
            ));
        }
    }
    let rules = rules
        .as_array()
        .ok_or_else(|| reject("active rules unavailable"))?;
    ensure(rules.len() < 100, "active rules pagination incomplete")?;
    for rule in rules {
        let params = &rule["parameters"];
        match text(rule, "type")? {
            "deletion" | "non_fast_forward" => (),
            "required_status_checks" => {
                let checks = params["required_status_checks"]
                    .as_array()
                    .ok_or_else(|| reject("ruleset check denominator missing"))?;
                ensure(!checks.is_empty(), "empty required check denominator")?;
                for check in checks {
                    let app = match check.get("integration_id") {
                        Some(Value::Null) => None,
                        Some(v) => Some(v.as_u64().ok_or_else(|| reject("invalid required app"))?),
                        None => return Err(reject("missing required app")),
                    };
                    required.push((text(check, "context")?.into(), app));
                }
            }
            "pull_request" => {
                let count = params["required_approving_review_count"]
                    .as_u64()
                    .ok_or_else(|| reject("review count missing"))?;
                let code_owner = params["require_code_owner_review"]
                    .as_bool()
                    .ok_or_else(|| reject("code owner policy missing or malformed"))?;
                let last_push = params["require_last_push_approval"]
                    .as_bool()
                    .ok_or_else(|| reject("last push policy missing or malformed"))?;
                review_required |= count > 0 || code_owner || last_push;
                if let Some(methods) = params.get("allowed_merge_methods") {
                    ensure(
                        methods
                            .as_array()
                            .is_some_and(|v| v.iter().any(|m| m == "merge")),
                        "merge method prohibited",
                    )?;
                }
            }
            _ => return Err(reject("unsupported active rule; no bypass permitted")),
        }
    }
    let decision = pr
        .get("reviewDecision")
        .ok_or_else(|| reject("review decision missing"))?;
    ensure(
        decision == "APPROVED" || (!review_required && decision.is_null()),
        "required review is not approved",
    )?;
    let commits = pr["commits"]["nodes"]
        .as_array()
        .ok_or_else(|| reject("head commit observation missing"))?;
    ensure(
        commits.len() == 1 && commits[0]["commit"]["oid"] == request.expected_head_sha,
        "checks not bound to head",
    )?;
    let rollup = &commits[0]["commit"]["statusCheckRollup"];
    ensure(rollup["state"] == "SUCCESS", "checks not successful")?;
    let checks = complete_nodes(&rollup["contexts"])?;
    ensure(!checks.is_empty(), "empty check observation")?;
    for check in checks {
        let required_by_github = check["isRequired"]
            .as_bool()
            .ok_or_else(|| reject("required check classification missing"))?;
        if required_by_github {
            ensure(check_passes(check), "required check unsuccessful")?;
        }
    }
    for (context, app) in required {
        ensure(
            checks.iter().any(|c| {
                check_passes(c)
                    && (c["name"] == context || c["context"] == context)
                    && app.is_none_or(|app| {
                        c["checkSuite"]["app"]["databaseId"].as_u64() == Some(app)
                    })
            }),
            "required context/app missing or unsuccessful",
        )?;
    }
    let base_sha = text(pr, "baseRefOid")?;
    ensure(is_full_git_sha(base_sha), "base commit missing")?;
    Ok(base_sha.into())
}
fn check_passes(check: &Value) -> bool {
    match check["__typename"].as_str() {
        Some("CheckRun") => check["status"] == "COMPLETED" && check["conclusion"] == "SUCCESS",
        Some("StatusContext") => check["state"] == "SUCCESS",
        _ => false,
    }
}
fn merged(
    value: &Value,
    request: &GithubMutationRequest,
    base: &str,
    expected_base: Option<&str>,
    linkage: &PublicationLinkage,
) -> Result<MergedIdentity, RemoteRouteFinding> {
    let pr = identity(value, request, base)?;
    ensure(
        pr["state"] == "MERGED" && pr["merged"] == true,
        "merge not yet authenticated; reconciliation only",
    )?;
    let issue_state = linkage.validate(value, request, true)?;
    let commit = text(&pr["mergeCommit"], "oid")?;
    ensure(is_full_git_sha(commit), "merged commit missing")?;
    let parents = complete_nodes(&pr["mergeCommit"]["parents"])?;
    ensure(
        parents.len() == 2 && parents[1]["oid"] == request.expected_head_sha,
        "merge method/head parent mismatch",
    )?;
    let base_sha = text(&parents[0], "oid")?;
    ensure(
        is_full_git_sha(base_sha) && expected_base.is_none_or(|b| b == base_sha),
        "merge base parent drift",
    )?;
    Ok(MergedIdentity {
        repository: request.repository.clone(),
        pull_request: request.pull_request.unwrap_or_default(),
        head_sha: request.expected_head_sha.clone(),
        base: base.into(),
        base_sha: base_sha.into(),
        method: MergeMethod::Merge,
        merge_commit: commit.into(),
        publication_linkage: linkage.clone(),
        issue_state,
    })
}

/// Construct and durably retain the exact authenticated merge intent without
/// dispatching the merge. The semantic owner reserves this identity before it
/// calls `execute_staged`.
pub(super) fn stage(
    root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
    selector_digest: &str,
) -> Result<StagedMerge, RemoteRouteFinding> {
    let GithubMutation::PullRequestMerge {
        base,
        method: _,
        review_receipt_path,
        review_receipt_digest,
    } = &request.mutation
    else {
        unreachable!()
    };
    ensure(
        request.issue > 0
            && request.pull_request.is_some_and(|number| number > 0)
            && crate::adapters::supported_pr_branch(base),
        "invalid merge target/base",
    )?;
    ensure(
        request
            .operator_approval
            .as_ref()
            .is_some_and(|approval| !approval.trim().is_empty()),
        "explicit operator merge authorization reference required",
    )?;
    ensure(
        request.recovery.is_none(),
        "merge retries are reconciliation only",
    )?;
    let review: TypedReviewReceipt =
        load_optional_receipt(root, Some(review_receipt_path), "merge_review_missing")?
            .ok_or_else(|| reject("review receipt missing"))?;
    ensure(
        review.schema == "csdlc.v3.typed_review_receipt.v1"
            && review.repository == request.repository
            && review.issue == request.issue
            && review.reviewed_revision == request.expected_head_sha
            && review.expected_head_sha == request.expected_head_sha
            && !review.evidence_digest.trim().is_empty()
            && !review.implementer.trim().is_empty()
            && !review.reviewer.trim().is_empty()
            && !same_principal(Some(&review.implementer), Some(&review.reviewer))
            && typed_review_receipt_payload_digest(&review) == *review_receipt_digest,
        "stale or invalid exact-head review",
    )?;
    let linkage = review
        .publication_linkage
        .as_ref()
        .filter(|linkage| linkage.valid_for(request))
        .ok_or_else(|| reject("merge requires reviewed qualified publication linkage"))?;
    preflight_github_credential(&mutation_credential_name(request)?, process)?;

    let operation_digest = github_mutation_operation_digest(request);
    let control = git_control_dir(root).ok_or_else(|| reject("Git receipt directory missing"))?;
    let dir = control.join("csdlc-v3/remote/merges");
    fs::create_dir_all(&dir).map_err(|_| reject("merge receipt directory unavailable"))?;
    sync_directory_ancestry(&dir, &control, |path| fs::File::open(path)?.sync_all())
        .map_err(|_| reject("merge directory ancestry not durable"))?;
    let lock_path = dir.join(format!(
        "{}.lock",
        stable_digest(&[
            &request.repository,
            &request.pull_request.unwrap_or_default().to_string()
        ])
    ));
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| reject("merge lock unavailable"))?;
    lock.try_lock_exclusive()
        .map_err(|_| reject("another merge invocation owns this PR"))?;
    let _lock = MergeLock(lock);

    let intent_path = dir.join(format!("{operation_digest}.intent.json"));
    let target_path = dir.join(format!(
        "{}.target.json",
        stable_digest(&[
            &request.repository,
            &request.pull_request.unwrap_or_default().to_string()
        ])
    ));
    let target = json!({"schema":"csdlc.v3.merge_target.v1","repository":request.repository,"pull_request":request.pull_request,"operation_digest":operation_digest});
    let preexisting = intent_path.exists();
    if target_path.exists() {
        let existing: Value = serde_json::from_slice(
            &fs::read(&target_path).map_err(|_| reject("merge target guard unavailable"))?,
        )
        .map_err(|_| reject("merge target guard invalid"))?;
        ensure(
            existing == target,
            "PR already has a durable merge attempt; replay the original request",
        )?;
    } else {
        ensure(!preexisting, "retained intent is missing its target guard")?;
    }
    let (observation, _) = observe(
        request,
        "pull-request-merge-linkage",
        linkage.observation_target(request),
        process,
    )?;
    let pr = identity(&observation, request, base)?;
    let intent = if preexisting {
        let saved: MergeIntent = serde_json::from_slice(
            &fs::read(&intent_path).map_err(|_| reject("intent unavailable"))?,
        )
        .map_err(|_| reject("intent invalid"))?;
        ensure(
            saved.schema == "csdlc.v3.merge_intent.v1"
                && saved.request == *request
                && saved.selector_digest == selector_digest
                && saved.publication_linkage == *linkage,
            "merge intent identity mismatch",
        )?;
        saved
    } else {
        let (rules, base_sha) = if pr["merged"] == true {
            (
                Value::Null,
                merged(&observation, request, base, None, linkage)?.base_sha,
            )
        } else {
            let (rules, _) = observe(request, "branch-merge-rules", base.clone(), process)?;
            let base_sha = eligibility(&observation, &rules, request, base, linkage)?;
            (rules, base_sha)
        };
        let saved = MergeIntent {
            schema: "csdlc.v3.merge_intent.v1".into(),
            request: request.clone(),
            selector_digest: selector_digest.into(),
            publication_linkage: linkage.clone(),
            pre_state: observation,
            rules,
            base_sha,
        };
        if !target_path.exists() {
            persist_json_create_new(&target_path, &target)?;
        }
        persist_json_create_new(&intent_path, &saved)?;
        saved
    };
    let intent_digest = stable_digest(&[
        &serde_json::to_string(&intent).map_err(|_| reject("intent encoding failed"))?
    ]);
    Ok(StagedMerge {
        request: request.clone(),
        intent,
        intent_digest,
        operation_digest,
        preexisting,
    })
}

pub(super) fn execute_staged(
    root: &Path,
    staged: &StagedMerge,
    reconciliation_only: bool,
    process: &mut impl ProcessAdapter,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    execute_inner(
        root,
        &staged.request,
        process,
        &staged.intent.selector_digest,
        !reconciliation_only && !staged.preexisting,
        Some(&staged.intent_digest),
    )
}

pub(super) fn execute(
    root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
    selector_digest: &str,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    execute_inner(root, request, process, selector_digest, false, None)
}

fn execute_inner(
    root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
    selector_digest: &str,
    dispatch_staged: bool,
    expected_intent_digest: Option<&str>,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    let GithubMutation::PullRequestMerge {
        base,
        method: _,
        review_receipt_path,
        review_receipt_digest,
    } = &request.mutation
    else {
        unreachable!()
    };
    ensure(
        request.issue > 0
            && request.pull_request.is_some_and(|n| n > 0)
            && crate::adapters::supported_pr_branch(base),
        "invalid merge target/base",
    )?;
    ensure(
        request
            .operator_approval
            .as_ref()
            .is_some_and(|s| !s.trim().is_empty()),
        "explicit operator merge authorization reference required",
    )?;
    ensure(
        request.recovery.is_none(),
        "merge retries are reconciliation only",
    )?;
    let review: TypedReviewReceipt =
        load_optional_receipt(root, Some(review_receipt_path), "merge_review_missing")?
            .ok_or_else(|| reject("review receipt missing"))?;
    ensure(
        review.schema == "csdlc.v3.typed_review_receipt.v1"
            && review.repository == request.repository
            && review.issue == request.issue
            && review.reviewed_revision == request.expected_head_sha
            && review.expected_head_sha == request.expected_head_sha
            && !review.evidence_digest.trim().is_empty()
            && !review.implementer.trim().is_empty()
            && !review.reviewer.trim().is_empty()
            && !same_principal(Some(&review.implementer), Some(&review.reviewer))
            && typed_review_receipt_payload_digest(&review) == *review_receipt_digest,
        "stale or invalid exact-head review",
    )?;
    let linkage = review
        .publication_linkage
        .as_ref()
        .filter(|l| l.valid_for(request))
        .ok_or_else(|| reject("merge requires reviewed qualified publication linkage"))?;
    preflight_github_credential(&mutation_credential_name(request)?, process)?;
    let digest = github_mutation_operation_digest(request);
    let control = git_control_dir(root).ok_or_else(|| reject("Git receipt directory missing"))?;
    let dir = control.join("csdlc-v3/remote/merges");
    let mut performed_mutation = false;
    fs::create_dir_all(&dir).map_err(|_| reject("merge receipt directory unavailable"))?;
    sync_directory_ancestry(&dir, &control, |path| fs::File::open(path)?.sync_all())
        .map_err(|_| reject("merge directory ancestry not durable"))?;
    let lock_path = dir.join(format!(
        "{}.lock",
        stable_digest(&[
            &request.repository,
            &request.pull_request.unwrap_or_default().to_string()
        ])
    ));
    let lock = fs::OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(&lock_path)
        .map_err(|_| reject("merge lock unavailable"))?;
    lock.try_lock_exclusive()
        .map_err(|_| reject("another merge invocation owns this PR"))?;
    let _lock = MergeLock(lock);
    let intent_path = dir.join(format!("{digest}.intent.json"));
    let reconciliation_path = dir.join(format!("{digest}.reconciliation.json"));
    let receipt_path = github_mutation_receipt_path(root, &digest)?;
    let replay = intent_path.exists();
    let target_path = dir.join(format!(
        "{}.target.json",
        stable_digest(&[
            &request.repository,
            &request.pull_request.unwrap_or_default().to_string()
        ])
    ));
    let target = json!({"schema":"csdlc.v3.merge_target.v1","repository":request.repository,"pull_request":request.pull_request,"operation_digest":digest});
    if target_path.exists() {
        let existing: Value = serde_json::from_slice(
            &fs::read(&target_path).map_err(|_| reject("merge target guard unavailable"))?,
        )
        .map_err(|_| reject("merge target guard invalid"))?;
        ensure(
            existing == target,
            "PR already has a durable merge attempt; replay the original request",
        )?;
    } else {
        ensure(!replay, "retained intent is missing its target guard")?;
    }

    let (observation, mut invocation) = observe(
        request,
        "pull-request-merge-linkage",
        linkage.observation_target(request),
        process,
    )?;
    let pr = identity(&observation, request, base)?;
    let intent = if replay {
        let saved: MergeIntent = serde_json::from_slice(
            &fs::read(&intent_path).map_err(|_| reject("intent unavailable"))?,
        )
        .map_err(|_| reject("intent invalid"))?;
        ensure(
            saved.schema == "csdlc.v3.merge_intent.v1"
                && saved.request == *request
                && saved.selector_digest == selector_digest
                && saved.publication_linkage == *linkage,
            "merge intent identity mismatch",
        )?;
        saved
    } else {
        let (rules, base_sha) = if pr["merged"] == true {
            (
                Value::Null,
                merged(&observation, request, base, None, linkage)?.base_sha,
            )
        } else {
            let (rules, _) = observe(request, "branch-merge-rules", base.clone(), process)?;
            let base_sha = eligibility(&observation, &rules, request, base, linkage)?;
            (rules, base_sha)
        };
        let saved = MergeIntent {
            schema: "csdlc.v3.merge_intent.v1".into(),
            request: request.clone(),
            selector_digest: selector_digest.into(),
            publication_linkage: linkage.clone(),
            pre_state: observation.clone(),
            rules,
            base_sha,
        };
        if !target_path.exists() {
            persist_json_create_new(&target_path, &target)?;
        }
        persist_json_create_new(&intent_path, &saved)?;
        saved
    };
    let intent_digest = stable_digest(&[
        &serde_json::to_string(&intent).map_err(|_| reject("intent encoding failed"))?
    ]);
    ensure(
        expected_intent_digest.is_none_or(|expected| expected == intent_digest),
        "staged merge intent identity mismatch",
    )?;
    let response_path = dir.join(format!("{digest}.response.json"));
    let has_retained_success_response = response_path.exists();
    let mut response_digest = None;
    let mut response_sha = None;
    let identity = if (replay && !dispatch_staged) || pr["merged"] == true {
        let expected_base = if has_retained_success_response {
            None
        } else {
            Some(intent.base_sha.as_str())
        };
        merged(&observation, request, base, expected_base, linkage)?
    } else {
        // Repeat authenticated policy and PR checks immediately before dispatch.
        // REST SHA provides head CAS, not body/base/policy CAS; poststate proves base parent.
        let (fresh_rules, _) = observe(request, "branch-merge-rules", base.clone(), process)?;
        let (fresh, _) = observe(
            request,
            "pull-request-merge-linkage",
            linkage.observation_target(request),
            process,
        )?;
        ensure(
            fresh_rules == intent.rules
                && fresh["data"]["repository"]["pullRequest"]["baseRef"]["branchProtectionRule"]
                    == intent.pre_state["data"]["repository"]["pullRequest"]["baseRef"]
                        ["branchProtectionRule"]
                && eligibility(&fresh, &fresh_rules, request, base, linkage)? == intent.base_sha,
            "base or policy changed before dispatch",
        )?;
        persist_json_create_new(
            &dir.join(format!("{digest}.dispatch-prestate.json")),
            &json!({"observation":fresh,"rules":fresh_rules}),
        )?;
        let input = dir.join(format!("{digest}.input.json"));
        write_private_create_new(
            &input,
            &serde_json::to_vec(&json!({"sha": request.expected_head_sha, "merge_method":"merge"}))
                .map_err(|_| reject("input encoding failed"))?,
        )?;
        invocation = CommandInvocation::new(
            GITHUB_OPERATIONAL_ADAPTER,
            [
                "PUT".into(),
                format!(
                    "repos/{}/pulls/{}/merge",
                    request.repository,
                    request.pull_request.unwrap_or_default()
                ),
                input.to_string_lossy().into_owned(),
            ],
        )
        .and_then(|i| {
            i.with_child_credential(mutation_credential_name(request).unwrap_or_default())
        })
        .map_err(|_| reject("invalid merge dispatch"))?;
        let output = process.run(invocation.clone());
        performed_mutation = true;
        fs::remove_file(&input).map_err(|_| reject("private merge input cleanup failed"))?;
        if output.status == ProcessStatus::Exit(0) && !output.truncated {
            if let Ok(response) = serde_json::from_str::<Value>(&output.stdout) {
                if response["merged"] == true {
                    if let Some(sha) = response["sha"].as_str().filter(|sha| is_full_git_sha(sha)) {
                        persist_json_create_new(
                            &dir.join(format!("{digest}.response.json")),
                            &json!({"sha":sha,"digest":stable_digest(&[&output.stdout])}),
                        )?;
                    }
                }
            }
        }
        // Nonzero, timeout, cancellation and truncated responses never authorize retry.
        let (after, _) = observe(
            request,
            "pull-request-merge-linkage",
            linkage.observation_target(request),
            process,
        )?;
        let expected_base = if response_path.exists() {
            None
        } else {
            Some(intent.base_sha.as_str())
        };
        merged(&after, request, base, expected_base, linkage)?
    };
    if response_path.exists() {
        let response: Value = serde_json::from_slice(
            &fs::read(response_path).map_err(|_| reject("retained response unavailable"))?,
        )
        .map_err(|_| reject("retained response invalid"))?;
        response_sha = Some(text(&response, "sha")?.to_owned());
        response_digest = Some(text(&response, "digest")?.to_owned());
    }
    ensure(
        response_sha
            .as_ref()
            .is_none_or(|sha| sha == &identity.merge_commit),
        "response and authenticated merge commit disagree",
    )?;
    let reconciliation = GithubMutationReconciliationReceipt {
        schema: "csdlc.v3.github_mutation_reconciliation.v1".into(),
        operation_digest: digest.clone(),
        operation_marker: github_mutation_operation_marker(&digest),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request,
        remote_object_id: request.pull_request,
        expected_head_sha: request.expected_head_sha.clone(),
        readback_digest: stable_digest(&[&serde_json::to_string(&identity)
            .map_err(|_| reject("merge identity encoding failed"))?]),
        observed_by: GITHUB_READ_ONLY_ADAPTER.into(),
        authenticated: true,
        merge: Some(identity),
    };
    if reconciliation_path.exists() {
        let existing: GithubMutationReconciliationReceipt = serde_json::from_slice(
            &fs::read(&reconciliation_path).map_err(|_| reject("reconciliation unavailable"))?,
        )
        .map_err(|_| reject("reconciliation invalid"))?;
        ensure(
            existing == reconciliation,
            "authenticated merge result changed",
        )?;
    } else {
        persist_json_create_new(&reconciliation_path, &reconciliation)?;
    }
    let receipt = if receipt_path.exists() {
        let mut receipt = load_mutation_receipt(&receipt_path, &digest)?;
        ensure(
            receipt.intent_digest == intent_digest
                && receipt.reconciliation_digest
                    == github_mutation_reconciliation_digest(&reconciliation),
            "merge receipt identity drift",
        )?;
        receipt.idempotent_replay = true;
        receipt
    } else {
        let receipt = finalize_mutation_receipt(
            request,
            &digest,
            &intent_digest,
            response_digest,
            &reconciliation,
            (replay && !dispatch_staged) || pr["merged"] == true,
        );
        persist_json_create_new(&receipt_path, &receipt)?;
        receipt
    };
    Ok(GithubMutationResult {
        performed_mutation: Some(performed_mutation),
        receipt,
        reconciliation,
        invocation,
    })
}

// Sync the new directory and each linking ancestor, through existing Git control.
fn sync_directory_ancestry(
    path: &Path,
    boundary: &Path,
    mut sync: impl FnMut(&Path) -> std::io::Result<()>,
) -> std::io::Result<()> {
    if !path.starts_with(boundary) {
        return Err(std::io::Error::other("directory outside Git control"));
    }
    for parent in path.ancestors() {
        sync(parent)?;
        if parent == boundary {
            return Ok(());
        }
    }
    Err(std::io::Error::other("Git control boundary missing"))
}

#[cfg(test)]
mod directory_tests {
    use std::path::Path;

    use super::sync_directory_ancestry;
    #[test]
    fn directory_sync_orders_every_link_and_propagates_faults() {
        let boundary = Path::new("control");
        let leaf = boundary.join("csdlc-v3/remote/merges");
        let mut visited = vec![];
        sync_directory_ancestry(&leaf, boundary, |p| {
            visited.push(p.to_path_buf());
            Ok(())
        })
        .unwrap();
        assert_eq!(
            visited,
            vec![
                leaf.clone(),
                boundary.join("csdlc-v3/remote"),
                boundary.join("csdlc-v3"),
                boundary.to_path_buf()
            ]
        );
        for fail_at in 0..4 {
            let mut count = 0;
            assert!(sync_directory_ancestry(&leaf, boundary, |_| {
                let current = count;
                count += 1;
                if current == fail_at {
                    Err(std::io::Error::other("sync fault"))
                } else {
                    Ok(())
                }
            })
            .is_err());
            assert_eq!(count, fail_at + 1);
        }
    }
}

#[cfg(test)]
mod lock_tests {
    use super::MergeLock;
    use fs2::FileExt;
    use std::fs;

    #[test]
    fn merge_lock_releases_even_when_a_duplicate_handle_survives() {
        let root = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/merge-lock-release")
            .join(std::process::id().to_string());
        fs::create_dir_all(&root).unwrap();
        let path = root.join("merge.lock");
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        file.try_lock_exclusive().unwrap();
        let owner = MergeLock(file);
        // Models the open-file description inherited by a concurrently spawned
        // child until exec closes its CLOEXEC descriptor.
        let inherited = owner.0.try_clone().unwrap();
        let next = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .open(&path)
            .unwrap();
        assert!(
            next.try_lock_exclusive().is_err(),
            "live ownership must exclude another invocation"
        );
        drop(owner);
        next.try_lock_exclusive()
            .expect("ended owner must release even while a duplicate survives");
        FileExt::unlock(&next).unwrap();
        drop(inherited);
        drop(next);
        fs::remove_dir_all(root).unwrap();
    }
}
