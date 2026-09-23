//! Explicit retirement is local no-dispatch proof, never permission to retry a write.
use super::merge::*;
use super::model::*;
use super::storage::*;
use super::support::*;
use crate::adapters::ProcessAdapter;
use crate::storage::{
    semantic::{
        protocol::{EffectTruth, OperationId, OutcomeKind},
        IssueKey, SemanticRoot,
    },
    DurableTransactionStore,
};
use fs2::FileExt;
use serde_json::{json, Value};
use std::{
    fs,
    path::{Path, PathBuf},
};

fn reject(message: &str) -> RemoteRouteFinding {
    remote_finding("github_merge_retirement_ineligible", message)
}
fn directory(root: &Path) -> Result<PathBuf, RemoteRouteFinding> {
    Ok(git_control_dir(root)
        .ok_or_else(|| reject("Git control directory missing"))?
        .join("csdlc-v3/remote/merges"))
}
pub(super) fn present(path: &Path) -> Result<bool, RemoteRouteFinding> {
    match fs::symlink_metadata(path) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(_) => Err(reject("evidence presence is uncertain")),
    }
}
fn read(path: &Path) -> Result<Value, RemoteRouteFinding> {
    if !fs::symlink_metadata(path)
        .map_err(|_| reject("evidence unavailable"))?
        .file_type()
        .is_file()
    {
        return Err(reject("evidence is not a regular file"));
    }
    serde_json::from_slice(&fs::read(path).map_err(|_| reject("evidence unreadable"))?)
        .map_err(|_| reject("evidence malformed"))
}
fn digest(value: &Value) -> Result<String, RemoteRouteFinding> {
    Ok(stable_digest(&[
        &serde_json::to_string(value).map_err(|_| reject("evidence encoding failed"))?
    ]))
}
pub(super) fn not_retired(root: &Path, operation: &str) -> Result<(), RemoteRouteFinding> {
    if present(&directory(root)?.join(format!("{operation}.retirement.json")))? {
        return Err(reject("attempt retired; no dispatch or replay permitted"));
    }
    Ok(())
}

/// Called under the merge owner's per-PR lock. Return the sole create-only
/// successor slot, or None when the immutable chain already names this request.
pub(super) fn target_slot(
    root: &Path,
    target_path: &Path,
    target: &Value,
) -> Result<Option<PathBuf>, RemoteRouteFinding> {
    if !present(target_path)? {
        return Ok(Some(target_path.to_owned()));
    }
    let dir = directory(root)?;
    let mut current = read(target_path)?;
    let mut seen = std::collections::BTreeSet::new();
    loop {
        if current["schema"] != "csdlc.v3.merge_target.v1"
            || current["repository"] != target["repository"]
            || current["pull_request"] != target["pull_request"]
        {
            return Err(reject("target identity mismatch"));
        }
        let operation = current["operation_digest"]
            .as_str()
            .ok_or_else(|| reject("target digest missing"))?;
        if operation.len() != 64
            || !operation.bytes().all(|b| b.is_ascii_hexdigit())
            || !seen.insert(operation.to_owned())
        {
            return Err(reject("invalid target chain"));
        }
        if current == *target {
            if present(&dir.join(format!("{operation}.successor.json")))? {
                return Err(reject("attempt is not target chain leaf"));
            }
            return Ok(None);
        }
        if !verified_retirement(root, operation, &current)? {
            return Err(reject("predecessor retirement not settled"));
        }
        let slot = dir.join(format!("{operation}.successor.json"));
        if !present(&slot)? {
            return Ok(Some(slot));
        }
        current = read(&slot)?;
    }
}

/// A fence alone is not settlement. Both succession and the read-only pending
/// inventory require the canonical completed semantic failure and its exact
/// retained native evidence. Pending attachment remains visible to recovery.
pub(super) fn verified_retirement(
    root: &Path,
    operation: &str,
    target: &Value,
) -> Result<bool, RemoteRouteFinding> {
    let dir = directory(root)?;
    let retirement_path = dir.join(format!("{operation}.retirement.json"));
    if !present(&retirement_path)? {
        return Ok(false);
    }
    let retirement = read(&retirement_path)?;
    let original: MergeIntent =
        serde_json::from_value(read(&dir.join(format!("{operation}.intent.json")))?)
            .map_err(|_| reject("intent malformed"))?;
    if retirement["schema"] != "csdlc.v3.merge_retirement.v1"
        || retirement["operation_digest"] != operation
        || original.schema != "csdlc.v3.merge_intent.v1"
        || original.request.repository != target["repository"]
        || serde_json::to_value(original.request.pull_request)
            .map_err(|_| reject("PR encoding failed"))?
            != target["pull_request"]
        || github_mutation_operation_digest(&original.request) != operation
        || stable_digest(&[
            &serde_json::to_string(&original).map_err(|_| reject("intent encoding failed"))?
        ]) != retirement["intent_digest"]
        || !dispatch_evidence_is_absent(root, operation)?
    {
        return Err(reject("retirement intent or dispatch evidence changed"));
    }
    let common = git_control_dir(root).ok_or_else(|| reject("Git control directory missing"))?;
    let semantic_root = SemanticRoot::from_git_common(common, original.request.repository.clone())
        .map_err(|_| reject("semantic root invalid"))?;
    let key = IssueKey::new(original.request.repository.clone(), original.request.issue)
        .map_err(|_| reject("semantic key invalid"))?;
    let id: OperationId = serde_json::from_value(retirement["semantic_operation"].clone())
        .map_err(|_| reject("semantic operation invalid"))?;
    let completed = DurableTransactionStore::inspect_effect(&semantic_root, &key, &id)
        .map_err(|_| reject("semantic completion missing"))?;
    let content: Value = serde_json::from_slice(
        &completed
            .request()
            .canonical_content()
            .map_err(|_| reject("semantic request invalid"))?,
    )
    .map_err(|_| reject("semantic request malformed"))?;
    if completed.request().command() != crate::lifecycle::semantic::SemanticCommand::RecordMerge
        || content["schema"] != "csdlc.v3.staged_github_mutation.v1"
        || content["operation_digest"] != operation
        || content["intent_digest"] != retirement["intent_digest"]
        || content["request"]
            != serde_json::to_value(&original.request)
                .map_err(|_| reject("request encoding failed"))?
    {
        return Err(reject("retirement semantic identity changed"));
    }
    if completed.ticket().is_some() {
        return Ok(false);
    }
    let retained_evidence: Value = serde_json::from_slice(
        completed
            .evidence()
            .ok_or_else(|| reject("semantic evidence missing"))?,
    )
    .map_err(|_| reject("semantic evidence malformed"))?;
    if retained_evidence != retirement
        || completed.effect_truth() != Some(EffectTruth::NotPerformed)
        || completed.outcome_kind() != Some(OutcomeKind::Failure)
    {
        return Err(reject("predecessor retirement not settled"));
    }
    Ok(true)
}

pub(crate) fn retire_never_dispatched_merge(
    root: &Path,
    request: &GithubMutationRequest,
    expected_intent: &str,
    semantic_operation: &str,
    preview: &str,
    rationale: &str,
    process: &mut impl ProcessAdapter,
) -> Result<Value, RemoteRouteFinding> {
    let operation = github_mutation_operation_digest(request);
    let dir = directory(root)?;
    let lock = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(dir.join(format!(
            "{}.lock",
            stable_digest(&[
                &request.repository,
                &request.pull_request.unwrap_or_default().to_string()
            ])
        )))
        .map_err(|_| reject("merge lock missing"))?;
    lock.try_lock_exclusive()
        .map_err(|_| reject("merge invocation active"))?;
    let _lock = RetirementLock(lock);
    if !matches!(request.mutation, GithubMutation::PullRequestMerge { .. })
        || !dispatch_evidence_is_absent(root, &operation)?
    {
        return Err(reject("dispatch evidence forbids retirement"));
    }
    let intent: MergeIntent =
        serde_json::from_value(read(&dir.join(format!("{operation}.intent.json")))?)
            .map_err(|_| reject("intent malformed"))?;
    let actual = stable_digest(&[
        &serde_json::to_string(&intent).map_err(|_| reject("intent encoding failed"))?
    ]);
    if intent.request != *request || actual != expected_intent {
        return Err(reject("retained intent identity changed"));
    }
    let target_path = dir.join(format!(
        "{}.target.json",
        stable_digest(&[
            &request.repository,
            &request.pull_request.unwrap_or_default().to_string()
        ])
    ));
    let target = json!({"schema":"csdlc.v3.merge_target.v1","repository":request.repository,"pull_request":request.pull_request,"operation_digest":operation});
    if target_slot(root, &target_path, &target)?.is_some() {
        return Err(reject("retained target missing"));
    }
    let path = dir.join(format!("{operation}.retirement.json"));
    if present(&path)? {
        let saved = read(&path)?;
        if saved["operation_digest"] != operation
            || saved["intent_digest"] != actual
            || saved["semantic_operation"] != semantic_operation
            || saved["rationale"] != rationale
        {
            return Err(reject("retirement identity changed"));
        }
        return Ok(saved);
    }
    let (observed, _) = observe(
        request,
        "pull-request-merge-linkage",
        intent.publication_linkage.observation_target(request),
        process,
    )?;
    let repo = &observed["data"]["repository"];
    let pr = &repo["pullRequest"];
    if repo["nameWithOwner"] != request.repository
        || pr["number"] != request.pull_request.unwrap_or_default()
        || pr["state"] != "OPEN"
        || pr["merged"] != false
        || !pr["headRefOid"].as_str().is_some_and(is_full_git_sha)
        || pr["url"]
            != format!(
                "https://github.com/{}/pull/{}",
                request.repository,
                request.pull_request.unwrap_or_default()
            )
    {
        return Err(reject("authenticated open PR identity missing"));
    }
    if !dispatch_evidence_is_absent(root, &operation)? {
        return Err(reject("dispatch evidence appeared"));
    }
    let evidence = json!({"schema":"csdlc.v3.merge_retirement.v1","operation_digest":operation,"intent_digest":actual,
        "semantic_operation":semantic_operation,"preview_digest":preview,"rationale":rationale,"observation":observed});
    persist_json_create_new(&path, &evidence)?;
    Ok(evidence)
}

struct RetirementLock(fs::File);
impl Drop for RetirementLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.0);
    }
}
