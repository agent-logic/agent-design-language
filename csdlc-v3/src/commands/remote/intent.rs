//! Intent orchestration inside the native remote owner. Durable remote receipts
//! remain the source of publication identity and replay authority.
use super::*;
use serde_json::Value;

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
            if request.repository != repository || (request.issue != issue && request.issue != 0) {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExternalReview {
    #[serde(deserialize_with = "strict_review_receipt")]
    pub receipt: TypedReviewReceipt,
    pub receipt_digest: String,
    pub proof_path: String,
    pub proof_digest: String,
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

pub fn verify_external_review(
    root: &Path,
    repository: &str,
    issue: u64,
    head: &str,
    issue_digest: &str,
    evidence: &ExternalReview,
) -> Result<(), RemoteRouteFinding> {
    let receipt = &evidence.receipt;
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
    let expected = format!(".csdlc/evidence/{issue}/intent-proof.json");
    if evidence.proof_path != expected {
        return Err(remote_finding(
            "intent_review_proof_path_mismatch",
            "review must reference the canonical issue proof",
        ));
    }
    let proof_path = root.join(&expected);
    let canonical = proof_path.canonicalize().map_err(|_| {
        remote_finding("intent_review_proof_missing", "canonical proof is required")
    })?;
    let root_canonical = root.canonicalize().map_err(|_| {
        remote_finding(
            "intent_review_root_invalid",
            "repository root is unavailable",
        )
    })?;
    if !canonical.starts_with(root_canonical.join(".csdlc/evidence")) {
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
    let receipt_path = root.join(review_path(issue, head));
    let evidence_path = receipt_path.with_extension("external.json");
    guard_review_path(root, &receipt_path)?;
    guard_review_path(root, &evidence_path)?;
    // The full external packet is retained alongside the native receipt. A crash
    // between the two writes is recoverable only with the identical packet.
    let mut wrote = false;
    for (path, value) in [
        (evidence_path, serde_json::to_value(evidence)),
        (receipt_path, serde_json::to_value(&evidence.receipt)),
    ] {
        let value = value.map_err(|_| {
            remote_finding(
                "intent_review_serialization_failed",
                "external review cannot serialize",
            )
        })?;
        if path.exists() {
            let existing: Value = serde_json::from_slice(&fs::read(&path).map_err(|_| {
                remote_finding(
                    "intent_review_record_unreadable",
                    "existing review record is unreadable",
                )
            })?)
            .map_err(|_| {
                remote_finding(
                    "intent_review_record_invalid",
                    "existing review record is invalid",
                )
            })?;
            if existing != value {
                return Err(remote_finding(
                    "intent_review_record_conflict",
                    "existing exact-head review cannot be replaced",
                ));
            }
        } else {
            persist_json_create_new(&path, &value)?;
            wrote = true;
        }
    }
    Ok(wrote)
}

pub fn load_external_review(
    root: &Path,
    issue: u64,
    head: &str,
) -> Result<ExternalReview, RemoteRouteFinding> {
    let path = root
        .join(review_path(issue, head))
        .with_extension("external.json");
    guard_review_path(root, &path)?;
    let bytes = fs::read(path).map_err(|_| {
        remote_finding(
            "intent_external_review_missing",
            "record independent exact-head review before publication",
        )
    })?;
    let evidence: ExternalReview = serde_json::from_slice(&bytes).map_err(|_| {
        remote_finding(
            "intent_external_review_invalid",
            "retained external review is invalid",
        )
    })?;
    let receipt_path = root.join(review_path(issue, head));
    guard_review_path(root, &receipt_path)?;
    let receipt: TypedReviewReceipt =
        serde_json::from_slice(&fs::read(receipt_path).map_err(|_| {
            remote_finding(
                "intent_review_record_missing",
                "native review receipt is required",
            )
        })?)
        .map_err(|_| {
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
    let control = git_control_dir(root).ok_or_else(|| {
        remote_finding(
            "git_control_dir_unavailable",
            "Git control directory is required",
        )
    })?;
    let directory = control.join("csdlc-v3/remote/intents");
    if !directory.exists() {
        return Ok(None);
    }
    let mut targets = std::collections::BTreeSet::new();
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
        if intent.request.repository != repository || intent.request.issue != issue {
            continue;
        }
        let GithubMutation::PullRequestCreate {
            head: ref published_branch,
            ..
        } = intent.request.mutation
        else {
            continue;
        };
        if published_branch != branch {
            return Err(remote_finding(
                "intent_publication_branch_conflict",
                "issue has publication intent on another branch",
            ));
        }
        let receipt_path = github_mutation_receipt_path(root, digest)?;
        if !receipt_path.exists() {
            if intent.request.expected_head_sha != head {
                return Err(remote_finding(
                    "intent_publication_uncertain_head",
                    "reconcile the retained publication intent before changing its candidate",
                ));
            }
            continue;
        }
        let receipt = load_mutation_receipt(&receipt_path, digest)?;
        if receipt.intent_digest != github_mutation_intent_digest(&intent)
            || receipt.repository != repository
            || receipt.issue != issue
            || receipt.expected_head_sha != intent.request.expected_head_sha
        {
            return Err(remote_finding(
                "intent_publication_receipt_mismatch",
                "publication receipt does not bind its native intent",
            ));
        }
        let number = receipt.pull_request.filter(|n| *n > 0).ok_or_else(|| {
            remote_finding(
                "intent_publication_target_missing",
                "authenticated publication receipt lacks PR identity",
            )
        })?;
        targets.insert(number);
    }
    if targets.len() > 1 {
        return Err(remote_finding(
            "intent_publication_ambiguous",
            "multiple native publication targets require explicit recovery",
        ));
    }
    Ok(targets.into_iter().next())
}

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
