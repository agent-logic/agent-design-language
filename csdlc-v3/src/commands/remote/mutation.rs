//! Semantic staging, admission and orchestration for typed GitHub mutations.

use std::{fs, path::Path};

use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessStatus};

use super::authority::verify_canonical_v3_authority;
use super::coordination::{validate as validate_coordination, verify as verify_coordination};
use super::model::*;
use super::storage::*;
use super::support::{
    exact_issue_names, github_mutation_intent_digest, github_mutation_operation_digest,
    github_mutation_operation_marker, github_mutation_reconciliation_digest, remote_finding,
    stable_digest, validate_repository_name, GITHUB_OPERATIONAL_ADAPTER, GITHUB_READ_ONLY_ADAPTER,
};
use super::transport::*;

impl StagedGithubMutation {
    pub fn native_identity(&self) -> crate::storage::semantic::protocol::NativeIdentity {
        self.native_identity.clone()
    }
    pub fn request_bytes(&self) -> Result<Vec<u8>, RemoteRouteFinding> {
        Ok(self.request_bytes.clone())
    }
    pub fn request(&self) -> &GithubMutationRequest {
        &self.request
    }
    pub fn reservation_facts(&self) -> crate::lifecycle::semantic::Facts {
        use crate::lifecycle::semantic::Facts;
        match self.request.mutation {
            GithubMutation::PullRequestCreate { .. } | GithubMutation::PullRequestUpdate { .. } => {
                Facts {
                    current_proof: true,
                    independent_review: true,
                    publication: true,
                    ..Default::default()
                }
            }
            GithubMutation::PullRequestReady => Facts {
                merge_ready: true,
                ..Default::default()
            },
            GithubMutation::PullRequestMerge { .. } => Facts {
                merge_ready: true,
                merged: true,
                ..Default::default()
            },
            _ => Facts::default(),
        }
    }
    fn validate_result_identity(
        &self,
        result: &GithubMutationResult,
    ) -> Result<(), RemoteRouteFinding> {
        let reconciliation = &result.reconciliation;
        let expected_issue = if matches!(self.request.mutation, GithubMutation::IssueCreate { .. })
        {
            reconciliation.issue > 0
        } else {
            reconciliation.issue == self.request.issue
        };
        let expected_pull_request = match self.request.mutation {
            GithubMutation::PullRequestCreate { .. } => {
                reconciliation.pull_request.is_some_and(|number| number > 0)
            }
            GithubMutation::PullRequestUpdate { .. }
            | GithubMutation::PullRequestReady
            | GithubMutation::PullRequestMerge { .. } => {
                reconciliation.pull_request == self.request.pull_request
                    && reconciliation.pull_request.is_some()
            }
            _ => reconciliation.pull_request.is_none(),
        };
        if result.receipt.operation_digest != self.operation_digest
            || result.receipt.intent_digest != self.intent_digest
            || result.receipt.repository != self.request.repository
            || result.receipt.expected_head_sha != self.request.expected_head_sha
            || !result.receipt.authenticated
            || result.receipt.schema != "csdlc.v3.github_mutation_receipt.v2"
            || result.receipt.adapter != GITHUB_OPERATIONAL_ADAPTER
            || reconciliation.schema != "csdlc.v3.github_mutation_reconciliation.v1"
            || reconciliation.operation_digest != self.operation_digest
            || reconciliation.operation_marker != self.operation_marker
            || reconciliation.repository != self.request.repository
            || reconciliation.expected_head_sha != self.request.expected_head_sha
            || reconciliation.observed_by != GITHUB_READ_ONLY_ADAPTER
            || !reconciliation.authenticated
            || !expected_issue
            || !expected_pull_request
            || result.receipt.issue != reconciliation.issue
            || result.receipt.pull_request
                != reconciliation.pull_request.or(self.request.pull_request)
            || result.receipt.readback_digest.as_deref()
                != Some(reconciliation.readback_digest.as_str())
            || result.receipt.reconciliation_digest
                != github_mutation_reconciliation_digest(reconciliation)
        {
            return Err(remote_finding(
                "semantic_outcome_identity_mismatch",
                "authenticated remote receipt does not match the reserved native identity",
            ));
        }
        Ok(())
    }
    pub fn verified_outcome(
        &self,
        result: &GithubMutationResult,
    ) -> Result<crate::storage::semantic::protocol::VerifiedOutcome, RemoteRouteFinding> {
        use crate::storage::semantic::protocol::{EffectTruth, OutcomeKind, VerifiedOutcome};
        self.validate_result_identity(result)?;
        let truth = match result.performed_mutation {
            Some(true) => EffectTruth::Performed,
            Some(false) => EffectTruth::NotPerformed,
            None => EffectTruth::Unknown,
        };
        let evidence = serde_json::to_vec(&serde_json::json!({
            "schema":"csdlc.v3.semantic_remote_outcome.v1",
            "receipt":result.receipt,
            "reconciliation":result.reconciliation
        }))
        .map_err(|_| {
            remote_finding(
                "semantic_outcome_encoding_failed",
                "cannot encode remote outcome",
            )
        })?;
        VerifiedOutcome::from_native_owner(
            OutcomeKind::Success,
            truth,
            evidence,
            self.reservation_facts(),
            self.native_identity(),
        )
        .map_err(|_| {
            remote_finding(
                "semantic_outcome_invalid",
                "verified remote outcome is invalid",
            )
        })
    }
    pub fn verified_creation_outcome(
        &self,
        result: &GithubMutationResult,
    ) -> Result<
        crate::storage::semantic::protocol::creation::VerifiedCreationOutcome,
        RemoteRouteFinding,
    > {
        use crate::storage::semantic::protocol::{
            creation::VerifiedCreationOutcome, EffectTruth, OutcomeKind,
        };
        self.validate_result_identity(result)?;
        let number = result.reconciliation.issue;
        let issue =
            crate::storage::semantic::IssueKey::new(self.request.repository.clone(), number)
                .map_err(|_| {
                    remote_finding(
                        "semantic_creation_identity_invalid",
                        "created issue identity is invalid",
                    )
                })?;
        let truth = match result.performed_mutation {
            Some(true) => EffectTruth::Performed,
            Some(false) => EffectTruth::NotPerformed,
            None => EffectTruth::Unknown,
        };
        let evidence = serde_json::to_vec(&serde_json::json!({
            "schema":"csdlc.v3.semantic_issue_creation_outcome.v1",
            "receipt":result.receipt,
            "reconciliation":result.reconciliation
        }))
        .map_err(|_| {
            remote_finding(
                "semantic_outcome_encoding_failed",
                "cannot encode issue creation outcome",
            )
        })?;
        VerifiedCreationOutcome::from_native_owner(
            OutcomeKind::Success,
            truth,
            Some(issue),
            evidence,
            self.native_identity(),
        )
        .map_err(|_| {
            remote_finding(
                "semantic_outcome_invalid",
                "verified issue creation outcome is invalid",
            )
        })
    }
}

pub fn stage_github_mutation(
    repo_root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<StagedGithubMutation, RemoteRouteFinding> {
    validate_repository_name(&request.repository)?;
    validate_mutation(request)?;
    validate_coordination(request)?;
    let credential_name = mutation_credential_name(request)?;
    let authority = verify_canonical_v3_authority(repo_root, None, &request.expected_head_sha)?;
    let operation_digest = github_mutation_operation_digest(request);
    let operation_marker = github_mutation_operation_marker(&operation_digest);
    let is_merge = matches!(request.mutation, GithubMutation::PullRequestMerge { .. });
    if is_merge && request.recovery.is_some() {
        return Err(remote_finding(
            "github_merge_ineligible",
            "merge retries are reconciliation only",
        ));
    }
    let recovery = request.recovery.clone();
    let mut effective_request = request.clone();
    effective_request.recovery = None;
    let mut resolved_ready_target = None;
    let intent_digest;
    let mut preexisting = false;

    let staged_merge = if is_merge {
        let staged = super::merge::stage(
            repo_root,
            &effective_request,
            process,
            &authority.selector_digest,
        )?;
        intent_digest = staged.intent_digest().to_owned();
        Some(staged)
    } else {
        let intent_path = github_mutation_intent_path(repo_root, &operation_digest)?;
        let mut intent = GithubMutationIntent {
            schema: "csdlc.v3.github_mutation_intent.v2".into(),
            operation_digest: operation_digest.clone(),
            operation_marker: operation_marker.clone(),
            authority_selector_digest: authority.selector_digest.clone(),
            request: effective_request.clone(),
            adapter: GITHUB_OPERATIONAL_ADAPTER.into(),
            resolved_edit: None,
            resolved_ready_target: None,
        };
        if intent_path.exists() {
            preexisting = true;
            let retained = load_mutation_intent(&intent_path, &operation_digest)?;
            intent.schema = retained.schema.clone();
            intent.resolved_edit = retained.resolved_edit.clone();
            intent.resolved_ready_target = retained.resolved_ready_target.clone();
            if retained != intent {
                return Err(remote_finding(
                    "github_mutation_intent_mismatch",
                    "retained intent differs from this operation",
                ));
            }
        } else if matches!(request.mutation, GithubMutation::IssueEdit { .. }) {
            intent.resolved_edit = Some(resolve_issue_edit(request, process)?);
        } else if matches!(request.mutation, GithubMutation::PullRequestReady) {
            intent.resolved_ready_target = Some(resolve_ready_target(request, process)?);
        }
        if let Some(edit) = &intent.resolved_edit {
            effective_request.mutation = edit.clone();
        }
        resolved_ready_target = intent.resolved_ready_target.clone();
        intent_digest = github_mutation_intent_digest(&intent);
        preflight_github_credential(&credential_name, process)?;
        if !preexisting {
            verify_coordination(repo_root, request, process)?;
            persist_json_create_new(&intent_path, &intent)?;
        }
        None
    };
    let request_bytes = serde_json::to_vec(&serde_json::json!({
        "schema":"csdlc.v3.staged_github_mutation.v1",
        "operation_digest":operation_digest,
        "intent_digest":intent_digest,
        "request":effective_request
    }))
    .map_err(|_| {
        remote_finding(
            "github_mutation_intent_invalid",
            "staged mutation cannot serialize",
        )
    })?;
    let native_identity = crate::storage::semantic::protocol::NativeIdentity::new(
        "csdlc-v3-github".into(),
        intent_digest.clone(),
    )
    .map_err(|_| {
        remote_finding(
            "github_mutation_intent_invalid",
            "native mutation identity is invalid",
        )
    })?;
    Ok(StagedGithubMutation {
        request: effective_request,
        native_identity,
        request_bytes,
        operation_digest,
        operation_marker,
        intent_digest,
        credential_name,
        resolved_ready_target,
        merge: staged_merge,
        preexisting,
        recovery,
    })
}

pub fn execute_staged_github_mutation(
    repo_root: &Path,
    staged: &StagedGithubMutation,
    reconciliation_only: bool,
    process: &mut impl ProcessAdapter,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    let request = &staged.request;
    if let Some(merge) = &staged.merge {
        if merge.operation_digest() != staged.operation_digest
            || merge.request() != request
            || merge.intent_digest() != staged.intent_digest
        {
            return Err(remote_finding(
                "github_mutation_intent_mismatch",
                "staged merge identity changed before execution",
            ));
        }
        return super::merge::execute_staged(repo_root, merge, reconciliation_only, process);
    }
    let intent_path = github_mutation_intent_path(repo_root, &staged.operation_digest)?;
    let retained = load_mutation_intent(&intent_path, &staged.operation_digest)?;
    if github_mutation_intent_digest(&retained) != staged.intent_digest {
        return Err(remote_finding(
            "github_mutation_intent_mismatch",
            "staged native intent changed before execution",
        ));
    }
    let receipt_path = github_mutation_receipt_path(repo_root, &staged.operation_digest)?;
    if receipt_path.exists() || reconciliation_only || staged.preexisting {
        let reconciled = reconcile_github_mutation(
            request,
            &staged.operation_digest,
            &staged.operation_marker,
            process,
        );
        let (reconciliation, invocation, response_digest, idempotent_replay) = match reconciled {
            Ok((reconciliation, invocation)) => (reconciliation, invocation, None, true),
            Err(finding)
                if finding.code == "github_mutation_not_reconciled"
                    && !receipt_path.exists()
                    && !matches!(request.mutation, GithubMutation::IssueEdit { .. })
                    && staged.recovery
                        == Some(GithubMutationRecovery::RetryAfterAuthenticatedAbsence) =>
            {
                ensure_recovery_available(repo_root, &staged.operation_digest)?;
                if staged
                    .resolved_ready_target
                    .as_ref()
                    .is_some_and(|target| !target.draft)
                {
                    let (reconciliation, invocation) = reconcile_github_mutation(
                        request,
                        &staged.operation_digest,
                        &staged.operation_marker,
                        process,
                    )?;
                    (reconciliation, invocation, None, true)
                } else {
                    let (response_digest, invocation) = dispatch_github_mutation_after_intent(
                        repo_root,
                        request,
                        GithubMutationDispatchContext {
                            operation_digest: &staged.operation_digest,
                            operation_marker: &staged.operation_marker,
                            credential_name: &staged.credential_name,
                            ready_target: staged.resolved_ready_target.as_ref(),
                            recovery_intent_digest: Some(&staged.intent_digest),
                        },
                        process,
                    )?;
                    let (reconciliation, _) = reconcile_github_mutation(
                        request,
                        &staged.operation_digest,
                        &staged.operation_marker,
                        process,
                    )
                    .map_err(|finding| {
                        remote_finding(
                            "github_mutation_reconciliation_pending",
                            &format!("mutation outcome is uncertain after explicit recovery retry; durable intent forbids another replay until authenticated reconciliation succeeds: {}", finding.code),
                        )
                    })?;
                    (reconciliation, invocation, response_digest, false)
                }
            }
            Err(finding) => return Err(finding),
        };
        let mut receipt = if receipt_path.exists() {
            load_mutation_receipt(&receipt_path, &staged.operation_digest)?
        } else {
            let receipt = finalize_mutation_receipt(
                request,
                &staged.operation_digest,
                &staged.intent_digest,
                response_digest,
                &reconciliation,
                idempotent_replay,
            );
            persist_json_create_new(&receipt_path, &receipt)?;
            receipt
        };
        if receipt.intent_digest != staged.intent_digest
            || receipt.reconciliation_digest
                != github_mutation_reconciliation_digest(&reconciliation)
        {
            return Err(remote_finding(
                "github_mutation_receipt_mismatch",
                "existing mutation receipt does not match authenticated reconciliation",
            ));
        }
        receipt.idempotent_replay = idempotent_replay;
        return Ok(GithubMutationResult {
            performed_mutation: Some(!idempotent_replay),
            receipt,
            reconciliation,
            invocation,
        });
    }
    if staged
        .resolved_ready_target
        .as_ref()
        .is_some_and(|target| !target.draft)
    {
        let (reconciliation, invocation) = reconcile_github_mutation(
            request,
            &staged.operation_digest,
            &staged.operation_marker,
            process,
        )?;
        let receipt = finalize_mutation_receipt(
            request,
            &staged.operation_digest,
            &staged.intent_digest,
            None,
            &reconciliation,
            true,
        );
        persist_json_create_new(&receipt_path, &receipt)?;
        return Ok(GithubMutationResult {
            performed_mutation: Some(false),
            receipt,
            reconciliation,
            invocation,
        });
    }
    let (response_digest, invocation) = dispatch_github_mutation_after_intent(
        repo_root,
        request,
        GithubMutationDispatchContext {
            operation_digest: &staged.operation_digest,
            operation_marker: &staged.operation_marker,
            credential_name: &staged.credential_name,
            ready_target: staged.resolved_ready_target.as_ref(),
            recovery_intent_digest: None,
        },
        process,
    )?;
    let (reconciliation, _) = reconcile_github_mutation(
        request, &staged.operation_digest, &staged.operation_marker, process,
    ).map_err(|finding| remote_finding(
        "github_mutation_reconciliation_pending",
        &format!("mutation outcome is uncertain; durable intent forbids replay until authenticated reconciliation succeeds: {}", finding.code),
    ))?;
    let receipt = finalize_mutation_receipt(
        request,
        &staged.operation_digest,
        &staged.intent_digest,
        response_digest,
        &reconciliation,
        false,
    );
    persist_json_create_new(&receipt_path, &receipt)?;
    Ok(GithubMutationResult {
        performed_mutation: Some(true),
        receipt,
        reconciliation,
        invocation,
    })
}

pub fn execute_github_mutation(
    repo_root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<GithubMutationResult, RemoteRouteFinding> {
    #[cfg(not(unix))]
    return Err(remote_finding(
        "operational_remote_unsupported_platform",
        "native v3 operational remote mutation is supported only on Unix platforms",
    ));

    validate_repository_name(&request.repository)?;
    let credential_name = mutation_credential_name(request)?;
    validate_mutation(request)?;
    validate_coordination(request)?;
    let authority = verify_canonical_v3_authority(repo_root, None, &request.expected_head_sha)?;

    if matches!(request.mutation, GithubMutation::PullRequestMerge { .. }) {
        return super::merge::execute(repo_root, request, process, &authority.selector_digest);
    }

    let operation_digest = github_mutation_operation_digest(request);
    let operation_marker = github_mutation_operation_marker(&operation_digest);
    let mut intent_request = request.clone();
    intent_request.recovery = None;
    let mut intent = GithubMutationIntent {
        schema: "csdlc.v3.github_mutation_intent.v1".into(),
        operation_digest: operation_digest.clone(),
        operation_marker: operation_marker.clone(),
        authority_selector_digest: authority.selector_digest,
        request: intent_request,
        adapter: GITHUB_OPERATIONAL_ADAPTER.into(),
        resolved_edit: None,
        resolved_ready_target: None,
    };
    let intent_path = github_mutation_intent_path(repo_root, &operation_digest)?;
    let receipt_path = github_mutation_receipt_path(repo_root, &operation_digest)?;

    // Resolve once, before intent persistence. Retry uses the retained target rather
    // than recomputing label deltas or taking a newer issue body as the baseline.
    if intent_path.exists() {
        let existing = load_mutation_intent(&intent_path, &operation_digest)?;
        intent.resolved_edit = existing.resolved_edit.clone();
        intent.resolved_ready_target = existing.resolved_ready_target.clone();
        if existing != intent {
            return Err(remote_finding(
                "github_mutation_intent_mismatch",
                "retained intent differs from this operation",
            ));
        }
    } else if matches!(request.mutation, GithubMutation::IssueEdit { .. }) {
        intent.resolved_edit = Some(resolve_issue_edit(request, process)?);
    } else if matches!(request.mutation, GithubMutation::PullRequestReady) {
        intent.resolved_ready_target = Some(resolve_ready_target(request, process)?);
    } else {
        verify_coordination(repo_root, request, process)?;
    }
    let intent_digest = github_mutation_intent_digest(&intent);
    let mut effective_request = request.clone();
    if let Some(edit) = &intent.resolved_edit {
        effective_request.mutation = edit.clone();
    }
    let request = &effective_request;

    if receipt_path.exists() {
        let mut receipt = load_mutation_receipt(&receipt_path, &operation_digest)?;
        let (reconciliation, invocation) =
            reconcile_github_mutation(request, &operation_digest, &operation_marker, process)?;
        if receipt.intent_digest != intent_digest
            || receipt.reconciliation_digest
                != github_mutation_reconciliation_digest(&reconciliation)
        {
            return Err(remote_finding(
                "github_mutation_receipt_mismatch",
                "existing mutation receipt does not match current authenticated reconciliation",
            ));
        }
        receipt.idempotent_replay = true;
        return Ok(GithubMutationResult {
            performed_mutation: Some(false),
            receipt,
            reconciliation,
            invocation,
        });
    }

    if intent_path.exists() {
        let existing = load_mutation_intent(&intent_path, &operation_digest)?;
        if existing != intent {
            return Err(remote_finding(
                "github_mutation_intent_mismatch",
                "existing durable intent does not match this exact operation",
            ));
        }
        let reconciled =
            reconcile_github_mutation(request, &operation_digest, &operation_marker, process);
        let (reconciliation, invocation, response_digest, idempotent_replay) = match reconciled {
            Ok((reconciliation, invocation)) => (reconciliation, invocation, None, true),
            Err(finding)
                if finding.code == "github_mutation_not_reconciled"
                    && !matches!(request.mutation, GithubMutation::IssueEdit { .. })
                    && request.recovery
                        == Some(GithubMutationRecovery::RetryAfterAuthenticatedAbsence) =>
            {
                // Legacy intents are immutable. Resolve their missing target only
                // for an explicitly authorized retry after authenticated absence.
                ensure_recovery_available(repo_root, &operation_digest)?;
                let ready_target = match &intent.resolved_ready_target {
                    None if matches!(request.mutation, GithubMutation::PullRequestReady) => {
                        Some(resolve_ready_target(request, process)?)
                    }
                    target => target.clone(),
                };
                if ready_target.as_ref().is_some_and(|target| !target.draft) {
                    let (reconciliation, invocation) = reconcile_github_mutation(
                        request,
                        &operation_digest,
                        &operation_marker,
                        process,
                    )?;
                    (reconciliation, invocation, None, true)
                } else {
                    let (response_digest, invocation) = dispatch_github_mutation_after_intent(
                        repo_root,
                        request,
                        GithubMutationDispatchContext {
                            operation_digest: &operation_digest,
                            operation_marker: &operation_marker,
                            credential_name: &credential_name,
                            ready_target: ready_target.as_ref(),
                            recovery_intent_digest: Some(&intent_digest),
                        },
                        process,
                    )?;
                    let (reconciliation, _) =
                    reconcile_github_mutation(request, &operation_digest, &operation_marker, process)
                        .map_err(|finding| {
                            remote_finding(
                                "github_mutation_reconciliation_pending",
                                &format!(
                                    "mutation outcome is uncertain after explicit recovery retry; durable intent forbids another replay until authenticated reconciliation succeeds: {}",
                                    finding.code
                                ),
                            )
                        })?;
                    (reconciliation, invocation, response_digest, false)
                }
            }
            Err(finding) => return Err(finding),
        };
        let receipt = finalize_mutation_receipt(
            request,
            &operation_digest,
            &intent_digest,
            response_digest,
            &reconciliation,
            idempotent_replay,
        );
        persist_json_create_new(&receipt_path, &receipt)?;
        return Ok(GithubMutationResult {
            performed_mutation: Some(true),
            receipt,
            reconciliation,
            invocation,
        });
    }

    preflight_github_credential(&credential_name, process)?;
    persist_json_create_new(&intent_path, &intent)?;
    if intent
        .resolved_ready_target
        .as_ref()
        .is_some_and(|target| !target.draft)
    {
        let (reconciliation, invocation) =
            reconcile_github_mutation(request, &operation_digest, &operation_marker, process)?;
        let receipt = finalize_mutation_receipt(
            request,
            &operation_digest,
            &intent_digest,
            None,
            &reconciliation,
            true,
        );
        persist_json_create_new(&receipt_path, &receipt)?;
        return Ok(GithubMutationResult {
            performed_mutation: Some(true),
            receipt,
            reconciliation,
            invocation,
        });
    }
    let (response_digest, invocation) = dispatch_github_mutation_after_intent(
        repo_root,
        request,
        GithubMutationDispatchContext {
            operation_digest: &operation_digest,
            operation_marker: &operation_marker,
            credential_name: &credential_name,
            ready_target: intent.resolved_ready_target.as_ref(),
            recovery_intent_digest: None,
        },
        process,
    )?;
    let (reconciliation, _) = reconcile_github_mutation(
        request,
        &operation_digest,
        &operation_marker,
        process,
    )
    .map_err(|finding| {
        remote_finding(
            "github_mutation_reconciliation_pending",
            &format!(
                "mutation outcome is uncertain; durable intent forbids replay until authenticated reconciliation succeeds: {}",
                finding.code
            ),
        )
    })?;
    let receipt = finalize_mutation_receipt(
        request,
        &operation_digest,
        &intent_digest,
        response_digest,
        &reconciliation,
        false,
    );
    persist_json_create_new(&receipt_path, &receipt)?;
    Ok(GithubMutationResult {
        performed_mutation: Some(true),
        receipt,
        reconciliation,
        invocation,
    })
}

pub(super) fn resolve_issue_edit(
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<GithubMutation, RemoteRouteFinding> {
    let GithubMutation::IssueEdit {
        title,
        body,
        labels,
        assignees,
        milestone,
    } = &request.mutation
    else {
        unreachable!()
    };
    let invocation = github_mutation_reconciliation_invocation(request, "")?
        .with_child_credential(mutation_credential_name(request)?)
        .map_err(|_| {
            remote_finding("github_credential_scope_invalid", "invalid credential name")
        })?;
    let output = process.run(invocation);
    if output.truncated || output.status != ProcessStatus::Exit(0) {
        return Err(remote_finding(
            "github_issue_edit_baseline_unavailable",
            "authenticated issue baseline must succeed before retaining intent",
        ));
    }
    let value: serde_json::Value = serde_json::from_str(&output.stdout).map_err(|_| {
        remote_finding(
            "github_issue_edit_baseline_invalid",
            "issue baseline is not JSON",
        )
    })?;
    if value["number"].as_u64() != Some(request.issue) || value.get("pull_request").is_some() {
        return Err(remote_finding(
            "github_issue_edit_baseline_invalid",
            "issue baseline identity mismatch",
        ));
    }
    let preserved_body = match value.get("body") {
        Some(serde_json::Value::Null) => String::new(),
        Some(serde_json::Value::String(body)) => body.clone(),
        _ => {
            return Err(remote_finding(
                "github_issue_edit_baseline_invalid",
                "issue body missing",
            ))
        }
    };
    let baseline_labels = exact_issue_names(&value["labels"], "name").ok_or_else(|| {
        remote_finding(
            "github_issue_edit_baseline_invalid",
            "issue labels missing or invalid",
        )
    })?;
    let labels = match labels {
        Some(IssueLabelsUpdate::Add { names } | IssueLabelsUpdate::Remove { names }) => {
            let mut current = baseline_labels;
            if matches!(labels, Some(IssueLabelsUpdate::Add { .. })) {
                for name in names {
                    if !current.iter().any(|n| n.eq_ignore_ascii_case(name)) {
                        current.push(name.clone());
                    }
                }
            } else {
                current.retain(|n| !names.iter().any(|name| n.eq_ignore_ascii_case(name)));
            }
            Some(IssueLabelsUpdate::Replace { names: current })
        }
        other => other.clone(),
    };
    Ok(GithubMutation::IssueEdit {
        title: title.clone(),
        body: Some(body.clone().unwrap_or(preserved_body)),
        labels,
        assignees: assignees.clone(),
        milestone: milestone.clone(),
    })
}

pub(super) fn resolve_ready_target(
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<GithubReadyTarget, RemoteRouteFinding> {
    let invocation = github_mutation_reconciliation_invocation(request, "")?
        .with_child_credential(mutation_credential_name(request)?)
        .map_err(|_| {
            remote_finding("github_credential_scope_invalid", "invalid credential name")
        })?;
    let value = read_mutation_reconciliation_page(invocation, process)?;
    let number = value["number"].as_u64();
    let node_id = value["node_id"].as_str().map(str::to_owned);
    let head_sha = value["head"]["sha"].as_str().map(str::to_owned);
    let draft = value["draft"].as_bool();
    if number != request.pull_request
        || head_sha.as_deref() != Some(request.expected_head_sha.as_str())
        || node_id.as_deref().is_none_or(str::is_empty)
        || draft.is_none()
    {
        return Err(remote_finding(
            "github_pr_ready_target_mismatch",
            "authenticated PR identity, node id, draft state, and exact head are required",
        ));
    }
    Ok(GithubReadyTarget {
        number: number.unwrap_or_default(),
        node_id: node_id.unwrap_or_default(),
        head_sha: head_sha.unwrap_or_default(),
        draft: draft.unwrap_or(true),
    })
}

pub(super) fn dispatch_github_mutation_after_intent(
    repo_root: &Path,
    request: &GithubMutationRequest,
    context: GithubMutationDispatchContext<'_>,
    process: &mut impl ProcessAdapter,
) -> Result<(Option<String>, CommandInvocation), RemoteRouteFinding> {
    preflight_github_credential(context.credential_name, process)?;
    if context.recovery_intent_digest.is_some() {
        ensure_recovery_available(repo_root, context.operation_digest)?;
    }
    verify_coordination(repo_root, request, process)?;
    let input_path = write_mutation_input(
        repo_root,
        context.operation_digest,
        context.operation_marker,
        request,
        context.ready_target,
    )?;
    let prepared = (|| {
        let invocation = github_mutation_invocation(request, &input_path)?
            .with_child_credential(context.credential_name.to_owned())
            .map_err(|_| {
                remote_finding(
                    "github_credential_scope_invalid",
                    "GitHub credential name is not safe for child-process injection",
                )
            })?;
        if let Some(intent_digest) = context.recovery_intent_digest {
            persist_recovery_receipt(
                repo_root,
                request,
                context.operation_digest,
                intent_digest,
                context.ready_target,
            )?;
        }
        Ok(invocation)
    })();
    let invocation = match prepared {
        Ok(invocation) => invocation,
        Err(finding) => {
            let _ = fs::remove_file(&input_path);
            return Err(finding);
        }
    };
    let output = process.run(invocation.clone());
    let _ = fs::remove_file(&input_path);
    // curl's --fail-with-body contract uses 22 only for an authenticated HTTP
    // rejection. Other nonzero exits can occur after an uncertain transport
    // boundary, so they must fall through to exact authenticated readback.
    if output.status == ProcessStatus::Exit(22) {
        return Err(remote_finding(
            "github_mutation_rejected",
            "GitHub rejected the authenticated mutation request",
        ));
    }
    if output.status == ProcessStatus::Exit(0) && !output.truncated {
        validate_mutation_response(request, &output.stdout)?;
    }
    let response_digest = (!output.stdout.is_empty()).then(|| stable_digest(&[&output.stdout]));
    Ok((response_digest, invocation))
}
