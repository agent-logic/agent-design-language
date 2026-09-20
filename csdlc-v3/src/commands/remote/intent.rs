//! Intent orchestration inside the native remote owner. Durable remote receipts
//! remain the source of publication identity and replay authority.
use std::{fs, path::Path};

use crate::adapters::{CommandInvocation, ProcessAdapter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::authority::{read_canonical_authority_selector, verify_canonical_v3_authority};
use super::coordination::validate as validate_coordination;
use super::model::*;
pub use super::publication::validate_publication_metadata;
use super::publication::{
    body_closing_issue_references, body_has_relation, review_findings, same_principal,
    typed_review_receipt_matches, typed_review_receipt_payload_digest,
};
use super::routing::dispatch_operational_remote;
use super::storage::*;
use super::support::*;
use super::target::publication_target_inventory;
use super::transport::*;

/// Only an empty authenticated branch inventory establishes absence here.
/// A changed PR (title, draft state, or head) is still a possible performed
/// effect and must not be discarded just because exact reconciliation fails.
pub(crate) fn authenticated_absence_recovery(
    root: &Path,
    request: &GithubMutationRequest,
    process: &mut impl ProcessAdapter,
) -> Result<bool, RemoteRouteFinding> {
    if !matches!(request.mutation, GithubMutation::PullRequestCreate { .. })
        || !unsettled_absence_recovery(root, request)?
    {
        return Ok(false);
    }
    let operation_digest = github_mutation_operation_digest(request);
    let invocation = github_mutation_reconciliation_invocation(request, &operation_digest)?
        .with_child_credential(mutation_credential_name(request)?)
        .map_err(|_| {
            remote_finding(
                "github_credential_scope_invalid",
                "invalid credential scope",
            )
        })?;
    let value = read_mutation_reconciliation_page(invocation, process)?;
    let candidates = value.as_array().ok_or_else(|| {
        remote_finding(
            "github_mutation_reconciliation_invalid_json",
            "publication absence requires an authenticated pull-request array",
        )
    })?;
    Ok(candidates.is_empty())
}

/// Observe only retained native operations. No effect or reconciliation is
/// attempted while describing recovery, and settled operations are excluded.
pub fn pending_operations(
    root: &Path,
    repository: &str,
    issue: u64,
    head: &str,
) -> Result<Vec<Value>, RemoteRouteFinding> {
    let control = git_control_dir(root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "remote recovery requires Git metadata",
        )
    })?;
    let remote = control.join("csdlc-v3/remote");
    let mut pending = Vec::new();
    for directory in ["intents", "merges"] {
        let dir = remote.join(directory);
        for ancestor in [&remote, &dir, &remote.join("mutations")] {
            if ancestor
                .symlink_metadata()
                .is_ok_and(|m| m.file_type().is_symlink() || !m.is_dir())
            {
                return Err(remote_finding(
                    "remote_intent_inventory_invalid",
                    "remote evidence must remain real directories",
                ));
            }
        }
        let entries = match fs::read_dir(&dir) {
            Ok(v) => v,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(_) => {
                return Err(remote_finding(
                    "remote_intent_inventory_unreadable",
                    "remote evidence is unreadable",
                ))
            }
        };
        for entry in entries {
            let path = entry
                .map_err(|_| {
                    remote_finding(
                        "remote_intent_inventory_unreadable",
                        "remote inventory entry is unreadable",
                    )
                })?
                .path();
            let suffix = if directory == "intents" {
                ".json"
            } else {
                ".intent.json"
            };
            let Some(digest) = path
                .file_name()
                .and_then(|v| v.to_str())
                .and_then(|v| v.strip_suffix(suffix))
            else {
                continue;
            };
            if !path
                .symlink_metadata()
                .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
            {
                return Err(remote_finding(
                    "remote_intent_file_invalid",
                    "retained intent must be a regular file",
                ));
            }
            let bytes = fs::read(&path).map_err(|_| {
                remote_finding(
                    "remote_intent_file_invalid",
                    "retained intent cannot be read",
                )
            })?;
            let (request, intent_digest) = if directory == "intents" {
                let intent = load_mutation_intent(&path, digest)?;
                let hash = github_mutation_intent_digest(&intent);
                (intent.request, hash)
            } else {
                let value: Value = serde_json::from_slice(&bytes).map_err(|_| {
                    remote_finding("remote_merge_intent_invalid", "invalid merge intent")
                })?;
                let request: GithubMutationRequest =
                    serde_json::from_value(value["request"].clone()).map_err(|_| {
                        remote_finding("remote_merge_intent_invalid", "invalid merge request")
                    })?;
                if value["schema"] != "csdlc.v3.merge_intent.v1"
                    || github_mutation_operation_digest(&request) != digest
                    || !matches!(request.mutation, GithubMutation::PullRequestMerge { .. })
                {
                    return Err(remote_finding(
                        "remote_merge_intent_mismatch",
                        "merge intent identity mismatch",
                    ));
                }
                let hash = stable_digest(&[&serde_json::to_string(&value).map_err(|_| {
                    remote_finding(
                        "remote_merge_intent_invalid",
                        "merge intent encoding failed",
                    )
                })?]);
                (request, hash)
            };
            // IssueCreate's native target is zero: it does not retain the
            // coordinating issue. Shared Git metadata cannot establish that
            // attribution, even when two issue worktrees have the same HEAD.
            // Such operations require explicit replay of the original request;
            // never offer them through an unrelated issue's recovery preview.
            if request.repository != repository || request.issue == 0 || request.issue != issue {
                continue;
            }
            let receipt_path = github_mutation_receipt_path(root, digest)?;
            match receipt_path.symlink_metadata() {
                Ok(m) => {
                    if !m.is_file() || m.file_type().is_symlink() {
                        return Err(remote_finding(
                            "remote_mutation_receipt_invalid",
                            "remote receipt must be regular",
                        ));
                    }
                    let receipt = load_mutation_receipt(&receipt_path, digest)?;
                    if receipt.intent_digest != intent_digest
                        || receipt.repository != request.repository
                        || receipt.issue != request.issue
                        || receipt.expected_head_sha != request.expected_head_sha
                        || request
                            .pull_request
                            .is_some_and(|pr| receipt.pull_request != Some(pr))
                    {
                        return Err(remote_finding(
                            "remote_mutation_receipt_mismatch",
                            "remote receipt identity mismatch",
                        ));
                    }
                    if directory == "merges" {
                        let reconciliation_path = dir.join(format!("{digest}.reconciliation.json"));
                        if !reconciliation_path
                            .symlink_metadata()
                            .is_ok_and(|m| m.is_file() && !m.file_type().is_symlink())
                        {
                            return Err(remote_finding(
                                "remote_merge_reconciliation_invalid",
                                "merge reconciliation is absent or invalid",
                            ));
                        }
                        let reconciliation: GithubMutationReconciliationReceipt =
                            serde_json::from_slice(&fs::read(&reconciliation_path).map_err(
                                |_| {
                                    remote_finding(
                                        "remote_merge_reconciliation_invalid",
                                        "merge reconciliation unreadable",
                                    )
                                },
                            )?)
                            .map_err(|_| {
                                remote_finding(
                                    "remote_merge_reconciliation_invalid",
                                    "merge reconciliation invalid",
                                )
                            })?;
                        if receipt.reconciliation_digest
                            != github_mutation_reconciliation_digest(&reconciliation)
                            || !reconciliation.authenticated
                            || reconciliation.expected_head_sha != request.expected_head_sha
                        {
                            return Err(remote_finding(
                                "remote_merge_reconciliation_invalid",
                                "merge reconciliation does not settle exact request",
                            ));
                        }
                    }
                    continue;
                }
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                Err(_) => {
                    return Err(remote_finding(
                        "remote_mutation_receipt_invalid",
                        "remote receipt cannot be inspected",
                    ))
                }
            }
            if request.expected_head_sha != head {
                if let GithubMutation::PullRequestCreate {
                    head: published_branch,
                    ..
                } = &request.mutation
                {
                    if publication_target(root, repository, issue, published_branch, head)?
                        .is_some()
                    {
                        continue;
                    }
                }
                return Err(remote_finding(
                    "remote_recovery_head_mismatch",
                    "pending remote operation belongs to another exact checkout head",
                ));
            }
            pending.push(serde_json::json!({"operation_digest":digest,"intent_file_digest":blake3::hash(&bytes).to_hex().to_string(),"request":request}));
        }
    }
    pending.sort_by_key(|value| {
        value["operation_digest"]
            .as_str()
            .unwrap_or_default()
            .to_owned()
    });
    Ok(pending)
}

/// Reviewer-authored judgment. Receipt envelopes and proof identities are
/// derived by the owner, while this source judgment is retained verbatim.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReviewJudgment {
    pub schema: String,
    pub implementer: String,
    pub reviewer: String,
    pub reviewed_revision: String,
    pub verdict: String,
    pub evidence: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalReview {
    #[serde(deserialize_with = "strict_review_receipt")]
    pub receipt: TypedReviewReceipt,
    pub receipt_digest: String,
    pub proof_path: String,
    pub proof_digest: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub judgment: Option<ReviewJudgment>,
}

fn strict_review_receipt<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<TypedReviewReceipt, D::Error> {
    let value = Value::deserialize(deserializer)?;
    let allowed = [
        "schema",
        "repository",
        "issue",
        "implementer",
        "reviewer",
        "reviewed_revision",
        "expected_head_sha",
        "evidence_digest",
        "publication_linkage",
    ];
    if !value
        .as_object()
        .is_some_and(|object| object.keys().all(|key| allowed.contains(&key.as_str())))
    {
        return Err(serde::de::Error::custom(
            "external review receipt contains unsupported fields",
        ));
    }
    serde_json::from_value(value).map_err(serde::de::Error::custom)
}

fn guard_review_path(root: &Path, path: &Path) -> Result<(), RemoteRouteFinding> {
    let relative = path.strip_prefix(root).map_err(|_| {
        remote_finding(
            "intent_review_path_escape",
            "review path must remain beneath the issue repository",
        )
    })?;
    let mut cursor = root.to_path_buf();
    for component in relative.components() {
        if !matches!(component, std::path::Component::Normal(_)) {
            return Err(remote_finding(
                "intent_review_path_escape",
                "review path must be normalized",
            ));
        }
        cursor.push(component);
        match fs::symlink_metadata(&cursor) {
            Ok(metadata) if metadata.file_type().is_symlink() => {
                return Err(remote_finding(
                    "intent_review_symlink_denied",
                    "review records must not traverse symlinks",
                ))
            }
            Ok(_) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(_) => {
                return Err(remote_finding(
                    "intent_review_path_unreadable",
                    "review path metadata is unavailable",
                ))
            }
        }
    }
    Ok(())
}

pub fn review_path(issue: u64, head: &str) -> String {
    format!(".csdlc/evidence/{issue}/intent-review/{head}.json")
}

/// Retain each proof-bound review independently, including reviews at the same HEAD.
pub fn proof_review_path(issue: u64, head: &str, proof_digest: &str) -> String {
    let key = blake3::hash(proof_digest.as_bytes()).to_hex();
    format!(".csdlc/evidence/{issue}/intent-review/{head}/{key}.json")
}

pub fn retained_review_path(root: &Path, issue: u64, head: &str, proof_digest: &str) -> String {
    let versioned = proof_review_path(issue, head, proof_digest);
    if root.join(&versioned).exists() {
        versioned
    } else {
        review_path(issue, head)
    }
}

pub fn semantic_proof_path(issue: u64) -> String {
    format!(".csdlc/v3/issues/{issue}/proof.json")
}

pub fn verify_external_review(
    root: &Path,
    repository: &str,
    issue: u64,
    head: &str,
    issue_digest: &str,
    evidence: &ExternalReview,
) -> Result<(), RemoteRouteFinding> {
    let receipt = &evidence.receipt;
    if let Some(judgment) = &evidence.judgment {
        let bytes = serde_json::to_vec(judgment).map_err(|_| {
            remote_finding(
                "intent_review_judgment_invalid",
                "review judgment is not serializable",
            )
        })?;
        if judgment.schema != "csdlc.v3.review_judgment.v1"
            || judgment.verdict != "pass"
            || judgment.evidence.trim().is_empty()
            || judgment.implementer != receipt.implementer
            || judgment.reviewer != receipt.reviewer
            || judgment.reviewed_revision != receipt.reviewed_revision
            || blake3::hash(&bytes).to_hex().as_str() != receipt.evidence_digest
        {
            return Err(remote_finding(
                "intent_review_judgment_invalid",
                "passing exact-head judgment must match its retained receipt",
            ));
        }
    }
    if receipt.schema != "csdlc.v3.typed_review_receipt.v1"
        || receipt.repository != repository
        || receipt.issue != issue
        || receipt.reviewed_revision != head
        || receipt.expected_head_sha != head
        || !is_full_git_sha(head)
        || receipt.implementer.trim().is_empty()
        || receipt.reviewer.trim().is_empty()
        || same_principal(Some(&receipt.implementer), Some(&receipt.reviewer))
        || receipt.evidence_digest.trim().is_empty()
        || typed_review_receipt_payload_digest(receipt) != evidence.receipt_digest
        || !receipt.publication_linkage.as_ref().is_some_and(|linkage| {
            linkage.repository == repository
                && linkage.issue == issue
                && linkage.mode == RemotePublicationMode::Closing
        })
    {
        return Err(remote_finding(
            "intent_external_review_invalid",
            "independent exact-head external review and publication linkage are required",
        ));
    }
    let legacy = format!(".csdlc/evidence/{issue}/intent-proof.json");
    let semantic = semantic_proof_path(issue);
    if evidence.proof_path != legacy && evidence.proof_path != semantic {
        return Err(remote_finding(
            "intent_review_proof_path_mismatch",
            "review must reference the canonical issue proof",
        ));
    }
    let proof_path = root.join(&evidence.proof_path);
    let canonical = proof_path.canonicalize().map_err(|_| {
        remote_finding("intent_review_proof_missing", "canonical proof is required")
    })?;
    let root_canonical = root.canonicalize().map_err(|_| {
        remote_finding(
            "intent_review_root_invalid",
            "repository root is unavailable",
        )
    })?;
    if !canonical.starts_with(root_canonical.join(".csdlc/evidence"))
        && !canonical.starts_with(root_canonical.join(".csdlc/v3/issues"))
    {
        return Err(remote_finding(
            "intent_review_proof_path_escape",
            "proof must remain within repository evidence",
        ));
    }
    let bytes = fs::read(canonical).map_err(|_| {
        remote_finding(
            "intent_review_proof_missing",
            "canonical proof is unreadable",
        )
    })?;
    let proof: Value = serde_json::from_slice(&bytes)
        .map_err(|_| remote_finding("intent_review_proof_invalid", "proof must be typed JSON"))?;
    crate::commands::proof::intent::verify_current_inputs(root, &proof).map_err(|_| {
        remote_finding(
            "intent_review_proof_inputs_stale",
            "proof inputs and validator plan must match the current candidate",
        )
    })?;
    let mut payload = proof.clone();
    let claimed_payload_digest = payload
        .as_object_mut()
        .and_then(|object| object.remove("payload_digest"))
        .and_then(|value| value.as_str().map(str::to_owned))
        .ok_or_else(|| {
            remote_finding(
                "intent_review_proof_payload_missing",
                "proof payload digest is required",
            )
        })?;
    fn canonical_payload(value: Value) -> Value {
        match value {
            Value::Object(object) => Value::Object(
                object
                    .into_iter()
                    .map(|(key, value)| (key, canonical_payload(value)))
                    .collect::<std::collections::BTreeMap<_, _>>()
                    .into_iter()
                    .collect(),
            ),
            Value::Array(values) => {
                Value::Array(values.into_iter().map(canonical_payload).collect())
            }
            value => value,
        }
    }
    let payload_bytes = serde_json::to_vec(&canonical_payload(payload)).map_err(|_| {
        remote_finding(
            "intent_review_proof_payload_invalid",
            "proof payload cannot serialize",
        )
    })?;
    if blake3::hash(&payload_bytes).to_hex().as_str() != claimed_payload_digest {
        return Err(remote_finding(
            "intent_review_proof_payload_mismatch",
            "proof payload digest must match its canonical bytes",
        ));
    }
    if blake3::hash(&bytes).to_hex().as_str() != evidence.proof_digest
        || proof["schema"] != "csdlc.v3.intent_proof.v1"
        || proof["issue"].as_u64() != Some(issue)
        || proof["repository"] != repository
        || proof["head"] != head
        || proof["issue_digest"] != issue_digest
        || proof["status"] != "passed"
        || !proof["validators"].as_array().is_some_and(|validators| {
            !validators.is_empty()
                && validators.iter().all(|validator| {
                    validator["exit_code"].as_i64() == Some(0)
                        && validator["tests_passed"]
                            .as_u64()
                            .is_some_and(|count| count > 0)
                })
        })
    {
        return Err(remote_finding(
            "intent_review_proof_mismatch",
            "review requires successful nonempty proof for this exact issue version and head",
        ));
    }
    Ok(())
}

/// Persist caller-supplied independent evidence, never an internally authored approval.
pub fn record_external_review(
    root: &Path,
    repository: &str,
    issue: u64,
    head: &str,
    issue_digest: &str,
    authority_digest: &str,
    evidence: &ExternalReview,
) -> Result<bool, RemoteRouteFinding> {
    verify_canonical_v3_authority(root, Some(authority_digest), head)?;
    verify_external_review(root, repository, issue, head, issue_digest, evidence)?;
    let receipt_path = root.join(proof_review_path(issue, head, &evidence.proof_digest));
    guard_review_path(root, &receipt_path)?;
    // One immutable file contains both the native receipt and source judgment.
    // Existing two-file records remain readable and are never rewritten.
    if receipt_path.exists() {
        let retained = load_external_review(root, issue, head)?;
        if serde_json::to_value(&retained).ok() != serde_json::to_value(evidence).ok() {
            return Err(remote_finding(
                "intent_review_record_conflict",
                "existing exact-head review cannot be replaced",
            ));
        }
        return Ok(false);
    }
    let mut record = serde_json::to_value(&evidence.receipt).map_err(|_| {
        remote_finding(
            "intent_review_serialization_failed",
            "review cannot serialize",
        )
    })?;
    record["external_review"] = serde_json::to_value(evidence).map_err(|_| {
        remote_finding(
            "intent_review_serialization_failed",
            "judgment cannot serialize",
        )
    })?;
    persist_json_create_new(&receipt_path, &record)?;
    let wrote = true;
    Ok(wrote)
}

pub fn load_external_review(
    root: &Path,
    issue: u64,
    head: &str,
) -> Result<ExternalReview, RemoteRouteFinding> {
    let proof = fs::read(root.join(semantic_proof_path(issue)))
        .or_else(|_| fs::read(root.join(format!(".csdlc/evidence/{issue}/intent-proof.json"))))
        .map_err(|_| remote_finding("intent_review_proof_missing", "current proof is required"))?;
    let digest = blake3::hash(&proof).to_hex().to_string();
    let versioned = root.join(proof_review_path(issue, head, &digest));
    // Older installations retained a single immutable review per HEAD.
    let receipt_path = if versioned.exists() {
        versioned
    } else {
        root.join(review_path(issue, head))
    };
    guard_review_path(root, &receipt_path)?;
    let record: Value = serde_json::from_slice(&fs::read(&receipt_path).map_err(|_| {
        remote_finding(
            "intent_external_review_missing",
            "record independent exact-head review before publication",
        )
    })?)
    .map_err(|_| remote_finding("intent_review_record_invalid", "review record is invalid"))?;
    let evidence: ExternalReview = if let Some(packet) = record.get("external_review") {
        serde_json::from_value(packet.clone()).map_err(|_| {
            remote_finding(
                "intent_external_review_invalid",
                "retained judgment is invalid",
            )
        })?
    } else {
        let path = receipt_path.with_extension("external.json");
        guard_review_path(root, &path)?;
        serde_json::from_slice(&fs::read(path).map_err(|_| {
            remote_finding(
                "intent_external_review_missing",
                "legacy external review is missing",
            )
        })?)
        .map_err(|_| {
            remote_finding(
                "intent_external_review_invalid",
                "retained external review is invalid",
            )
        })?
    };
    let receipt: TypedReviewReceipt = serde_json::from_value(record).map_err(|_| {
        remote_finding(
            "intent_review_record_invalid",
            "native review receipt is invalid",
        )
    })?;
    if receipt != evidence.receipt {
        return Err(remote_finding(
            "intent_review_record_conflict",
            "native review receipt and retained external packet disagree",
        ));
    }
    Ok(evidence)
}

/// Resolve a single branch publication from native mutation intent/receipt pairs.
/// An unresolved create for a different head must not silently create another PR.
pub fn publication_target(
    root: &Path,
    repository: &str,
    issue: u64,
    branch: &str,
    head: &str,
) -> Result<Option<u64>, RemoteRouteFinding> {
    let targets = publication_target_inventory(root, repository, issue, branch, head, false)?;
    if targets.len() > 1 {
        return Err(remote_finding(
            "intent_publication_ambiguous",
            "multiple native publication targets require explicit recovery",
        ));
    }
    Ok(targets.into_iter().next())
}

/// Terminal reconciliation may name the actual closing PR after a retained
/// publication attempt was proven absent. Ordinary publication keeps the
/// stricter behavior above; this compatibility is read-only and cannot create
/// or update a PR.
pub fn publication_targets_for_finish(
    root: &Path,
    repository: &str,
    issue: u64,
    branch: &str,
    head: &str,
) -> Result<Vec<u64>, RemoteRouteFinding> {
    publication_target_inventory(root, repository, issue, branch, head, true)
}

pub(crate) fn settled_publication_identity_for_finish(
    root: &Path,
    repository: &str,
    issue: u64,
    requested_pull_request: Option<u64>,
) -> Result<Option<super::target::SettledPublicationIdentity>, RemoteRouteFinding> {
    let mut identities = super::target::settled_publication_identities(root, repository, issue)?;
    if let Some(requested) = requested_pull_request {
        identities.retain(|identity| identity.pull_request == requested);
    }
    if identities.len() > 1 {
        return Err(remote_finding(
            "intent_publication_ambiguous",
            "multiple settled publication identities require an explicit pull request",
        ));
    }
    Ok(identities.pop())
}

/// Shared preparation/edit/publication guard. This validates metadata, not review
/// authority, and deliberately performs no remote operation.
pub fn publication_create_admission(
    request: &RemoteRouteRequest,
    evidence: &ExternalReview,
) -> Result<(), RemoteRouteFinding> {
    if !typed_review_receipt_matches(request, Some(&evidence.receipt))
        || !review_findings(request).is_empty()
        || request.mode != Some(RemotePublicationMode::Closing)
        || !body_has_relation(request.body.as_deref(), "Closes", request.issue)
        || body_closing_issue_references(request.body.as_deref())
            .iter()
            .any(|issue| *issue != request.issue)
    {
        return Err(remote_finding("intent_publication_review_ineligible", "publication requires independent exact-head review and only the canonical closing relation"));
    }
    Ok(())
}

/// An uncertain create is retried only as the identical retained operation.
pub fn admit_publication_attempt(
    root: &Path,
    request: &GithubMutationRequest,
) -> Result<(), RemoteRouteFinding> {
    if !matches!(request.mutation, GithubMutation::PullRequestCreate { .. }) {
        return Ok(());
    }
    let control = git_control_dir(root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required",
        )
    })?;
    let directory = control.join("csdlc-v3/remote/intents");
    if !directory.exists() {
        return Ok(());
    }
    let requested = github_mutation_operation_digest(request);
    for entry in fs::read_dir(directory).map_err(|_| {
        remote_finding(
            "intent_publication_inventory_unreadable",
            "native intent inventory cannot be read",
        )
    })? {
        let path = entry
            .map_err(|_| {
                remote_finding(
                    "intent_publication_inventory_unreadable",
                    "native intent entry cannot be read",
                )
            })?
            .path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let digest = path.file_stem().and_then(|s| s.to_str()).ok_or_else(|| {
            remote_finding(
                "intent_publication_inventory_invalid",
                "intent filename is invalid",
            )
        })?;
        let intent = load_mutation_intent(&path, digest)?;
        if intent.request.repository == request.repository
            && intent.request.issue == request.issue
            && matches!(
                intent.request.mutation,
                GithubMutation::PullRequestCreate { .. }
            )
            && !github_mutation_receipt_path(root, digest)?.exists()
            && digest != requested
        {
            return Err(remote_finding(
                "intent_publication_uncertain_operation",
                "reconcile the identical retained publication before changing its inputs",
            ));
        }
    }
    Ok(())
}

/// Shape/identity admission is read-only and shared by preview and execution.
pub fn validate_intent_mutation(
    root: &Path,
    request: &GithubMutationRequest,
) -> Result<(), RemoteRouteFinding> {
    validate_repository_name(&request.repository)?;
    mutation_credential_name(request)?;
    validate_mutation(request)?;
    validate_coordination(request)?;
    verify_canonical_v3_authority(root, None, &request.expected_head_sha)?;
    admit_publication_attempt(root, request)
}

/// Authenticate mutable target identity immediately before the production owner
/// can create an intent or dispatch a write. Local publication receipts identify
/// the target; they do not establish its current remote candidate.
pub fn verify_publication_target(
    request: &GithubMutationRequest,
    base: &str,
    branch: &str,
    process: &mut impl ProcessAdapter,
) -> Result<(), RemoteRouteFinding> {
    let number = request
        .pull_request
        .filter(|number| *number > 0)
        .ok_or_else(|| {
            remote_finding(
                "intent_publication_target_missing",
                "existing publication requires a numeric target",
            )
        })?;
    validate_repository_name(&request.repository)?;
    let invocation = CommandInvocation::new(
        GITHUB_READ_ONLY_ADAPTER,
        [
            "pull-request".to_owned(),
            request.repository.clone(),
            number.to_string(),
        ],
    )
    .and_then(|invocation| {
        invocation.with_child_credential(mutation_credential_name(request).unwrap_or_default())
    })
    .map_err(|_| {
        remote_finding(
            "intent_publication_observation_invalid",
            "publication readback invocation is invalid",
        )
    })?;
    let value = read_mutation_reconciliation_page(invocation, process)?;
    if value["number"].as_u64() != Some(number)
        || value["head"]["sha"] != request.expected_head_sha
        || value["head"]["ref"] != branch
        || value["base"]["ref"] != base
        || value["state"] != "open"
        || value["merged"] != false
        || !body_has_relation(value["body"].as_str(), "Closes", request.issue)
        || body_closing_issue_references(value["body"].as_str())
            .iter()
            .any(|issue| *issue != request.issue)
    {
        return Err(remote_finding("intent_publication_remote_identity_mismatch","authenticated PR must match current head, branch, base, open state and canonical closing linkage before mutation"));
    }
    Ok(())
}

pub fn dispatch_intent_mutation(
    root: &Path,
    dispatch: &OperationalRemoteDispatchRequest,
    base: &str,
    branch: &str,
    process: &mut impl ProcessAdapter,
) -> Result<OperationalRemoteDispatchResult, RemoteRouteFinding> {
    if let OperationalRemoteOperation::GithubMutation(request) = &dispatch.operation {
        if matches!(
            request.mutation,
            GithubMutation::PullRequestUpdate { .. } | GithubMutation::PullRequestReady
        ) {
            verify_publication_target(request, base, branch, process)?;
        }
    }
    dispatch_operational_remote(root, dispatch, process)
}

/// Explicit owner mapping: repository creation must never reserve an issue-zero
/// transaction or borrow the invoking issue's generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SemanticMutationTarget {
    RepositoryCreation,
    Issue(crate::lifecycle::semantic::SemanticCommand),
}

pub fn semantic_mutation_target(
    request: &GithubMutationRequest,
) -> Result<SemanticMutationTarget, RemoteRouteFinding> {
    use crate::lifecycle::semantic::SemanticCommand;
    validate_repository_name(&request.repository)?;
    validate_mutation(request)?;
    validate_coordination(request)?;
    Ok(match &request.mutation {
        GithubMutation::IssueCreate { .. } => SemanticMutationTarget::RepositoryCreation,
        GithubMutation::IssueComment { .. }
        | GithubMutation::IssueEdit { .. }
        | GithubMutation::IssueCompleteCoordination { .. }
        | GithubMutation::IssueClose { .. } => {
            SemanticMutationTarget::Issue(SemanticCommand::RecordIssueMutation)
        }
        GithubMutation::PullRequestCreate { .. } | GithubMutation::PullRequestUpdate { .. } => {
            SemanticMutationTarget::Issue(SemanticCommand::Publish)
        }
        GithubMutation::PullRequestReady => {
            SemanticMutationTarget::Issue(SemanticCommand::MarkMergeReady)
        }
        GithubMutation::PullRequestMerge { .. } => {
            SemanticMutationTarget::Issue(SemanticCommand::RecordMerge)
        }
    })
}

/// Repository admission is minted only after the existing native authority owner
/// verifies this repository and exact revision. It creates no semantic issue.
pub(crate) fn semantic_creation_admission(
    root: &Path,
    request: &GithubMutationRequest,
) -> Result<crate::storage::semantic::protocol::creation::RepositoryAdmission, RemoteRouteFinding> {
    if semantic_mutation_target(request)? != SemanticMutationTarget::RepositoryCreation {
        return Err(remote_finding(
            "semantic_creation_scope_invalid",
            "repository creation requires native issue zero",
        ));
    }
    verify_canonical_v3_authority(root, None, &request.expected_head_sha)?;
    let common = git_control_dir(root)
        .and_then(|path| {
            if path.join("commondir").exists() {
                git_common_dir(&path)
            } else {
                Some(path)
            }
        })
        .ok_or_else(|| {
            remote_finding(
                "semantic_repository_unavailable",
                "canonical Git-common metadata is required",
            )
        })?;
    let common = common.canonicalize().map_err(|_| {
        remote_finding(
            "semantic_repository_unavailable",
            "canonical Git-common metadata is required",
        )
    })?;
    crate::storage::semantic::protocol::creation::RepositoryAdmission::from_native_owner(
        request.repository.clone(),
        crate::storage::semantic::Digest::authority(&read_canonical_authority_selector(root)?),
        common,
        request.expected_head_sha.clone(),
    )
    .map_err(|_| {
        remote_finding(
            "semantic_creation_admission_invalid",
            "native repository admission is invalid",
        )
    })
}

/// Preserve native uncertainty as durable, sanitized evidence. This cannot grant
/// a success transition and does not claim that the transport made no effect.
pub(crate) fn semantic_uncertain_outcome(
    native: crate::storage::semantic::protocol::NativeIdentity,
    finding: &RemoteRouteFinding,
) -> Result<crate::storage::semantic::protocol::VerifiedOutcome, RemoteRouteFinding> {
    use crate::storage::semantic::protocol::{EffectTruth, OutcomeKind, VerifiedOutcome};
    let evidence = serde_json::to_vec(&serde_json::json!({
        "schema":"csdlc.v3.semantic_remote_uncertainty.v1", "code":finding.code,
        "effects":"unknown", "recovery":"reconcile_original_native_operation"
    }))
    .map_err(|_| remote_finding("semantic_outcome_encoding_failed", "cannot encode outcome"))?;
    VerifiedOutcome::from_native_owner(
        OutcomeKind::Unresolved,
        EffectTruth::Unknown,
        evidence,
        crate::lifecycle::semantic::Facts::default(),
        native,
    )
    .map_err(|_| {
        remote_finding(
            "semantic_outcome_invalid",
            "cannot retain native uncertainty",
        )
    })
}
