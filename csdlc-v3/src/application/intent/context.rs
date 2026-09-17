use super::{read_json, AuthorityVersion, CheckoutIdentity, IntentPlan, IssueVersion, Snapshot};
use crate::commands::local::{required_local_commands, LocalPreparationRequest, PromptRegistry};
use crate::storage::{
    semantic::{
        protocol::{AttachmentAdmission, BindSource, EffectOrigin, OperationId},
        Admission as SemanticAdmission, Digest as SemanticDigest, IssueKey, Observation,
        SemanticRoot, Snapshot as SemanticSnapshot,
    },
    DurableTransactionStore,
};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
    process::Command,
};
thread_local! {static GIT_READS:std::cell::Cell<u64>=const {std::cell::Cell::new(0)};}
pub fn application_git_reads() -> u64 {
    GIT_READS.with(std::cell::Cell::get)
}

pub struct Context {
    pub root: PathBuf,
    pub primary: PathBuf,
    pub git_common: PathBuf,
    pub state_root: PathBuf,
    pub issue_root: PathBuf,
    pub issue: u64,
    pub repository: String,
    pub branch: String,
    pub head: String,
    pub authority_digest: String,
    pub index: Value,
    pub cleanup_pending: bool,
    rollback_admission: Option<RollbackAdmission>,
}

#[derive(Clone, Copy)]
enum RollbackAdmission {
    PendingRecovery,
    ExactReplay,
}

pub fn git(root: &Path, args: &[&str]) -> Result<String, String> {
    GIT_READS.with(|count| count.set(count.get() + 1));
    let out = Command::new("git")
        .env("GIT_OPTIONAL_LOCKS", "0")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .map_err(|_| "intent_git_observation_failed")?;
    if !out.status.success() {
        return Err("intent_git_observation_failed".into());
    }
    String::from_utf8(out.stdout)
        .map(|text| text.trim().into())
        .map_err(|_| "intent_git_output_invalid".into())
}

fn cleanup_archive_digest(
    root: &SemanticRoot,
    key: &IssueKey,
    snapshot: &SemanticSnapshot,
) -> Result<Option<String>, String> {
    let cleanup = crate::lifecycle::semantic::SemanticCommand::RecordCleanup;
    let selected = if let Some(pending) = snapshot.pending() {
        (pending.command() == cleanup).then(|| pending.id().clone())
    } else {
        let mut selected = None;
        for completed in snapshot.completed().iter().rev() {
            let inspection = DurableTransactionStore::inspect_effect(root, key, completed.id())
                .map_err(semantic_error)?;
            if inspection.request().command() == cleanup {
                selected = Some(completed.id().clone());
                break;
            }
        }
        selected
    };
    let Some(operation) = selected else {
        return Ok(None);
    };
    let inspection =
        DurableTransactionStore::inspect_effect(root, key, &operation).map_err(semantic_error)?;
    let bytes = inspection
        .request()
        .canonical_content()
        .map_err(semantic_error)?;
    let content: Value =
        serde_json::from_slice(&bytes).map_err(|_| "intent_cleanup_retained_request_invalid")?;
    content["archive_identity"]["inventory_digest"]
        .as_str()
        .map(str::to_owned)
        .map(Some)
        .ok_or_else(|| "intent_cleanup_archive_identity_invalid".into())
}

impl Context {
    pub fn load(start: &Path, issue: u64) -> Result<Self, String> {
        Self::load_with_rollback_admission(start, issue, None)
    }

    pub(crate) fn load_for_intent(start: &Path, issue: u64, command: &str) -> Result<Self, String> {
        let rollback_admission = match command {
            "recover" | "status" => Some(RollbackAdmission::PendingRecovery),
            "rollback" => Some(RollbackAdmission::ExactReplay),
            _ => None,
        };
        Self::load_with_rollback_admission(start, issue, rollback_admission)
    }

    fn load_with_rollback_admission(
        start: &Path,
        issue: u64,
        rollback_admission: Option<RollbackAdmission>,
    ) -> Result<Self, String> {
        let start = start
            .canonicalize()
            .map_err(|_| "intent_repository_unavailable")?;
        let invoking = PathBuf::from(git(&start, &["rev-parse", "--show-toplevel"])?);
        let common = PathBuf::from(git(
            &invoking,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?);
        let primary = common
            .parent()
            .ok_or("intent_primary_unavailable")?
            .to_path_buf();
        let remote = git(&invoking, &["remote", "get-url", "origin"])?;
        let repository = remote
            .strip_prefix("https://github.com/")
            .or_else(|| remote.strip_prefix("git@github.com:"))
            .ok_or("intent_repository_identity_invalid")?
            .trim_end_matches(".git")
            .to_owned();
        if repository.split('/').count() != 2
            || repository.split('/').any(|part| {
                part.is_empty()
                    || !part
                        .chars()
                        .all(|c| c.is_ascii_alphanumeric() || "-_.".contains(c))
            })
        {
            return Err("intent_repository_identity_invalid".into());
        }
        let registrations = git(&invoking, &["worktree", "list", "--porcelain"])?;
        let mut candidates = Vec::new();
        let prepared = common.join(format!("csdlc-v3/local/issues/{issue}"));
        let binding_path = common.join(format!("csdlc-v3/local/bindings/{issue}.json"));
        let canonical_bound_worktree = if binding_path.exists() {
            let binding = read_json(&binding_path)?;
            if binding["schema"] != "csdlc.v3.binding.v1" || binding["issue"] != issue {
                return Err("intent_native_binding_invalid".into());
            }
            Some(PathBuf::from(
                binding["worktree"]
                    .as_str()
                    .ok_or("intent_native_binding_invalid")?,
            ))
        } else {
            None
        };
        let prepared_recovery =
            crate::commands::local::intent::recovery_source(&common.join("csdlc-v3/local"), issue)
                .map_err(|_| "intent_recovery_journal_invalid")?;
        if canonical_bound_worktree.is_none()
            && (prepared.join("index.json").exists() || prepared_recovery.is_some())
        {
            candidates.push((primary.clone(), prepared.clone()));
        }
        for line in registrations
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
        {
            let path = PathBuf::from(line);
            if path == primary {
                continue;
            }
            if canonical_bound_worktree
                .as_ref()
                .is_some_and(|canonical| &path != canonical)
            {
                continue;
            }
            let issue_root = path.join(format!(".csdlc/issues/{issue}"));
            // An unrelated retired checkout has no bearing on this issue.
            if issue_root.join("index.json").exists()
                || path
                    .join(format!(".csdlc/transactions/{issue}.json"))
                    .exists()
            {
                candidates.push((path, issue_root));
            }
        }
        if candidates.len() == 2 && prepared_recovery.is_some() {
            let native_state = common.join("csdlc-v3/local");
            if let Some((branch, target)) =
                crate::commands::local::intent::pending_bind_identity(&native_state, issue)
                    .map_err(|_| "intent_recovery_journal_invalid")?
            {
                let target_candidate = candidates.iter().find(|(root, _)| root == &target).cloned();
                let source_present = candidates
                    .iter()
                    .any(|(root, issue_root)| root == &primary && issue_root == &prepared);
                let target_exact = target_candidate.as_ref().is_some_and(|(_, issue_root)| {
                    read_json(&issue_root.join("index.json")).is_ok_and(|index| {
                        index["schema"] == "csdlc.v3.local_state.v1"
                            && index["repository"] == repository
                            && index["issue"] == issue
                            && index["branch"] == branch
                            && index["worktree"].as_str() == target.to_str()
                    })
                });
                if source_present && target_exact {
                    candidates = vec![target_candidate.expect("checked target candidate")];
                }
            }
        }
        if candidates.is_empty() {
            let semantic_root = SemanticRoot::from_git_common(&common, repository.clone())
                .map_err(semantic_error)?;
            let semantic_key = IssueKey::new(repository.clone(), issue).map_err(semantic_error)?;
            if matches!(
                DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                    .map_err(semantic_error)?,
                Observation::Current(ref snapshot)
                    | Observation::ProjectionRepairRequired(ref snapshot)
                    if snapshot.inputs().binding().is_none()
            ) {
                candidates.push((primary.clone(), prepared.clone()));
            }
        }
        if candidates.len() > 1 {
            return Err("intent_issue_topology_ambiguous".into());
        }
        let mut archived_index = None;
        if candidates.is_empty() && binding_path.exists() {
            let binding = read_json(&binding_path)?;
            if binding["schema"] != "csdlc.v3.binding.v1" || binding["issue"] != issue {
                return Err("intent_native_binding_invalid".into());
            }
            let target = PathBuf::from(
                binding["worktree"]
                    .as_str()
                    .ok_or("intent_native_binding_invalid")?,
            );
            if registrations
                .lines()
                .any(|line| line.strip_prefix("worktree ") == target.to_str())
                && target.exists()
            {
                let semantic_root = SemanticRoot::from_git_common(&common, repository.clone())
                    .map_err(semantic_error)?;
                let semantic_key =
                    IssueKey::new(repository.clone(), issue).map_err(semantic_error)?;
                let exact_digest =
                    match DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                        .map_err(semantic_error)?
                    {
                        Observation::Current(snapshot)
                        | Observation::ProjectionRepairRequired(snapshot) => {
                            cleanup_archive_digest(&semantic_root, &semantic_key, &snapshot)?
                        }
                        _ => None,
                    };
                let retained = if let Some(digest) = exact_digest {
                    crate::commands::terminal::matching_retained_cleanup_index(
                        &primary, &target, issue, &digest,
                    )
                    .map_err(|finding| finding.code)?
                } else {
                    crate::commands::terminal::retained_cleanup_index(&primary, &target, issue)
                        .map_err(|finding| finding.code)?
                };
                if let Some(index) = retained {
                    candidates.push((
                        target.clone(),
                        target.join(format!(".csdlc/issues/{issue}")),
                    ));
                    archived_index = Some(index);
                }
            }
        }
        let (root, issue_root) = candidates.pop().unwrap_or((primary.clone(), prepared));
        let mut partial_cleanup_pending = false;
        if archived_index.is_none() && root != primary {
            let semantic_root = SemanticRoot::from_git_common(&common, repository.clone())
                .map_err(semantic_error)?;
            let semantic_key = IssueKey::new(repository.clone(), issue).map_err(semantic_error)?;
            let cleanup_pending = matches!(
                DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                    .map_err(semantic_error)?,
                Observation::Current(ref snapshot)
                    | Observation::ProjectionRepairRequired(ref snapshot)
                    if snapshot.pending().is_some_and(|pending| {
                        pending.command()
                            == crate::lifecycle::semantic::SemanticCommand::RecordCleanup
                    }) && snapshot.inputs().binding().is_some_and(|binding| binding.worktree == root)
            );
            if cleanup_pending {
                if issue_root.join("index.json").exists() {
                    partial_cleanup_pending = true;
                } else {
                    let snapshot =
                        match DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                            .map_err(semantic_error)?
                        {
                            Observation::Current(snapshot)
                            | Observation::ProjectionRepairRequired(snapshot) => snapshot,
                            _ => return Err("intent_cleanup_semantic_state_required".into()),
                        };
                    let digest = cleanup_archive_digest(&semantic_root, &semantic_key, &snapshot)?
                        .ok_or("intent_cleanup_archive_identity_required")?;
                    archived_index = crate::commands::terminal::matching_retained_cleanup_index(
                        &primary, &root, issue, &digest,
                    )
                    .map_err(|finding| finding.code)?;
                }
            }
        }
        let state_root = issue_root
            .parent()
            .and_then(Path::parent)
            .ok_or("intent_state_root_invalid")?
            .to_path_buf();
        let recovered_source = crate::commands::local::intent::recovery_source(&state_root, issue)
            .map_err(|_| "intent_recovery_journal_invalid")?;
        let read_root = recovered_source.as_ref().unwrap_or(&issue_root);
        let cleanup_pending = archived_index.is_some() || partial_cleanup_pending;
        let index = if let Some(index) = archived_index.clone() {
            index
        } else if read_root.join("index.json").exists() {
            read_json(&read_root.join("index.json"))?
        } else {
            Value::Null
        };
        if !index.is_null() && (index["issue"] != issue || index["repository"] != repository) {
            return Err("intent_issue_repository_mismatch".into());
        }
        let authority_digest = crate::commands::remote::canonical_authority_selector_digest(&root)
            .map_err(|finding| finding.code)?;
        if crate::authority::canonical_v3_authority(&root)?.is_none() {
            // A tracked selector rollback may suspend ordinary v3 admission
            // while an already-reserved RecordRollback still needs exact
            // receipt reconciliation. It grants no new operation authority.
            let rollback_recovery = if crate::authority::canonical_v2_rollback(&root)? {
                let semantic_root = SemanticRoot::from_git_common(&common, repository.clone())
                    .map_err(semantic_error)?;
                let semantic_key =
                    IssueKey::new(repository.clone(), issue).map_err(semantic_error)?;
                match DurableTransactionStore::observe_issue(&semantic_root, &semantic_key)
                    .map_err(semantic_error)?
                {
                    Observation::Current(ref snapshot)
                    | Observation::ProjectionRepairRequired(ref snapshot) => {
                        let pending = snapshot.pending().is_some_and(|pending| {
                            pending.command()
                                == crate::lifecycle::semantic::SemanticCommand::RecordRollback
                        });
                        let completed = snapshot.completed().iter().any(|completed| {
                            DurableTransactionStore::inspect_effect(
                                &semantic_root,
                                &semantic_key,
                                completed.id(),
                            )
                            .is_ok_and(|inspection| {
                                inspection.request().command()
                                    == crate::lifecycle::semantic::SemanticCommand::RecordRollback
                            })
                        });
                        match rollback_admission {
                            Some(RollbackAdmission::PendingRecovery) => pending,
                            Some(RollbackAdmission::ExactReplay) => pending || completed,
                            None => false,
                        }
                    }
                    _ => false,
                }
            } else {
                false
            };
            if !rollback_recovery {
                return Err("intent_operational_authority_required".into());
            }
        }
        let branch = git(&root, &["symbolic-ref", "--short", "HEAD"])?;
        let head = git(&root, &["rev-parse", "HEAD"])?;
        if !index.is_null()
            && index["phase"] == "bound"
            && (index["branch"] != branch || index["worktree"].as_str() != root.to_str())
        {
            return Err("intent_bound_checkout_mismatch".into());
        }
        Ok(Self {
            root,
            primary,
            git_common: common,
            state_root,
            issue_root,
            issue,
            repository,
            branch,
            head,
            authority_digest,
            index,
            cleanup_pending,
            rollback_admission,
        })
    }

    /// Return the stable semantic storage location and positive issue identity.
    /// Context loading has already authenticated the canonical native authority and
    /// repository topology; this helper creates no semantic files or locks.
    pub(crate) fn semantic_root_key(&self) -> Result<(SemanticRoot, IssueKey), String> {
        let root = SemanticRoot::from_git_common(&self.git_common, self.repository.clone())
            .map_err(semantic_error)?;
        let key = IssueKey::new(self.repository.clone(), self.issue).map_err(semantic_error)?;
        Ok((root, key))
    }

    pub(crate) fn semantic_authority(&self) -> Result<SemanticDigest, String> {
        // canonical_v3_authority was established by Context::load. Hash the exact
        // selected bytes into the semantic domain instead of reusing a differently
        // scoped hexadecimal selector digest.
        let bytes = std::fs::read(self.root.join("csdlc-v3/operator/authority-selector.json"))
            .map_err(|_| "intent_semantic_authority_unreadable")?;
        Ok(SemanticDigest::authority(&bytes))
    }

    fn semantic_origin(&self, snapshot: &SemanticSnapshot) -> Result<EffectOrigin, String> {
        if let Some(binding) = snapshot.inputs().binding() {
            if binding.branch != self.branch
                || binding.head != self.head
                || binding.worktree != self.root
            {
                return Err("intent_semantic_binding_stale".into());
            }
            Ok(EffectOrigin::bound(binding.clone()))
        } else {
            let source = BindSource::from_native_owner(
                self.repository.clone(),
                self.git_common.clone(),
                self.root.clone(),
                self.head.clone(),
            )
            .map_err(semantic_error)?;
            Ok(EffectOrigin::prepared(source))
        }
    }

    /// Load one current, projection-coherent semantic snapshot and bind it to the
    /// authenticated native context. Legacy-only issues require the separately
    /// owned conversion flow and never seed semantic state here.
    pub(crate) fn semantic_context(&self) -> Result<SemanticContext, String> {
        self.fresh_integrity()?;
        let (root, key) = self.semantic_root_key()?;
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(snapshot) => *snapshot,
            Observation::ProjectionRepairRequired(_) => {
                return Err("intent_semantic_projection_repair_required".into())
            }
            Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
            Observation::LegacyMigrationRequired => {
                return Err("intent_semantic_migration_required".into())
            }
            Observation::Absent => return Err("intent_semantic_state_missing".into()),
        };
        if snapshot.projection_required() {
            return Err("intent_semantic_projection_repair_required".into());
        }
        let authority = self.semantic_authority()?;
        if snapshot.inputs().authority() != &authority {
            return Err("intent_semantic_authority_changed".into());
        }
        let origin = self.semantic_origin(&snapshot)?;
        let admission = SemanticAdmission::new(key.clone(), snapshot.version().clone(), authority);
        Ok(SemanticContext {
            root,
            key,
            snapshot,
            admission,
            origin,
            primary: self.primary.clone(),
            issue: self.issue,
        })
    }

    /// Load canonical terminal state for the cleanup command without requiring
    /// generated tracked projections to be rewritten in the checkout that is
    /// about to be removed. All other semantic commands retain the strict
    /// projection-coherence requirement in `semantic_context`.
    pub(crate) fn semantic_cleanup_context(&self) -> Result<SemanticContext, String> {
        self.fresh_integrity()?;
        let (root, key) = self.semantic_root_key()?;
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                *snapshot
            }
            Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
            Observation::LegacyMigrationRequired => {
                return Err("intent_semantic_migration_required".into())
            }
            Observation::Absent => return Err("intent_semantic_state_missing".into()),
        };
        if snapshot.phase() != crate::lifecycle::LifecycleState::ClosedOut {
            return Err("intent_cleanup_semantic_terminal_required".into());
        }
        let authority = self.semantic_authority()?;
        if snapshot.inputs().authority() != &authority {
            return Err("intent_semantic_authority_changed".into());
        }
        let origin = self.semantic_origin(&snapshot)?;
        let admission = SemanticAdmission::new(key.clone(), snapshot.version().clone(), authority);
        Ok(SemanticContext {
            root,
            key,
            snapshot,
            admission,
            origin,
            primary: self.primary.clone(),
            issue: self.issue,
        })
    }

    /// Load terminal semantic state for an explicitly dispositioned cleanup
    /// reconciliation after the exact bound checkout has already disappeared.
    /// The caller must authenticate the retained binding and absence before it
    /// can construct the cleanup recovery origin.
    pub(crate) fn semantic_cleanup_absence_context(&self) -> Result<SemanticContext, String> {
        self.fresh_integrity()?;
        let (root, key) = self.semantic_root_key()?;
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                *snapshot
            }
            Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
            Observation::LegacyMigrationRequired => {
                return Err("intent_semantic_migration_required".into())
            }
            Observation::Absent => return Err("intent_semantic_state_missing".into()),
        };
        if snapshot.phase() != crate::lifecycle::LifecycleState::ClosedOut {
            return Err("intent_cleanup_semantic_terminal_required".into());
        }
        let authority = self.semantic_authority()?;
        if snapshot.inputs().authority() != &authority {
            return Err("intent_semantic_authority_changed".into());
        }
        let binding = snapshot
            .inputs()
            .binding()
            .ok_or("intent_cleanup_semantic_binding_required")?;
        let origin = EffectOrigin::bound(binding.clone());
        let admission = SemanticAdmission::new(key.clone(), snapshot.version().clone(), authority);
        Ok(SemanticContext {
            root,
            key,
            snapshot,
            admission,
            origin,
            primary: self.primary.clone(),
            issue: self.issue,
        })
    }

    /// Load the exact retained origin for an explicit pending/completed operation.
    /// This is the only context constructor valid after cleanup removed the bound
    /// checkout. It does not reconstruct that origin from the primary checkout.
    pub(crate) fn semantic_recovery_context(
        &self,
        operation: &OperationId,
    ) -> Result<SemanticContext, String> {
        let (root, key) = self.semantic_root_key()?;
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                *snapshot
            }
            Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
            Observation::LegacyMigrationRequired => {
                return Err("intent_semantic_migration_required".into())
            }
            Observation::Absent => return Err("intent_semantic_state_missing".into()),
        };
        let inspection = DurableTransactionStore::inspect_effect(&root, &key, operation)
            .map_err(semantic_error)?;
        let authority = if inspection.request().command()
            == crate::lifecycle::semantic::SemanticCommand::RecordRollback
            && crate::authority::canonical_v2_rollback(&self.root)?
        {
            snapshot.inputs().authority().clone()
        } else {
            let authority = self.semantic_authority()?;
            if snapshot.inputs().authority() != &authority {
                return Err("intent_semantic_authority_changed".into());
            }
            authority
        };
        let origin = inspection.request().origin().clone();
        let admission = SemanticAdmission::new(key.clone(), snapshot.version().clone(), authority);
        Ok(SemanticContext {
            root,
            key,
            snapshot,
            admission,
            origin,
            primary: self.primary.clone(),
            issue: self.issue,
        })
    }

    pub(crate) fn complete_semantic_projection(
        &self,
        snapshot: &SemanticSnapshot,
    ) -> Result<SemanticSnapshot, String> {
        let (root, key) = self.semantic_root_key()?;
        if snapshot.key() != &key {
            return Err("intent_semantic_issue_changed".into());
        }
        complete_projection(&root, &key, snapshot)
    }

    pub(crate) fn refresh_semantic_binding(&self) -> Result<bool, String> {
        self.fresh_integrity()?;
        let (root, key) = self.semantic_root_key()?;
        let snapshot = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(value) | Observation::ProjectionRepairRequired(value) => *value,
            _ => return Err("intent_semantic_state_missing".into()),
        };
        if snapshot.inputs().authority() != &self.semantic_authority()? {
            return Err("intent_semantic_authority_changed".into());
        }
        if snapshot.pending().is_some() {
            return Err("intent_semantic_recovery_required".into());
        }
        let Some(binding) = snapshot.inputs().binding() else {
            return Ok(false);
        };
        let registration = blake3::hash(
            serde_json::to_string(&serde_json::json!({"branch":self.branch,"worktree":self.root}))
                .map_err(|_| "intent_bind_identity_invalid")?
                .as_bytes(),
        )
        .to_hex()
        .to_string();
        if binding.branch != self.branch
            || binding.worktree != self.root
            || binding.registration != registration
        {
            return Err("intent_semantic_binding_stale".into());
        }
        if binding.head != self.head {
            let ancestry = Command::new("git")
                .env("GIT_OPTIONAL_LOCKS", "0")
                .arg("-C")
                .arg(&self.root)
                .args(["merge-base", "--is-ancestor", &binding.head, &self.head])
                .status()
                .map_err(|_| "intent_git_observation_failed")?;
            match ancestry.code() {
                Some(0) => {}
                Some(1) => return Err("intent_semantic_binding_stale".into()),
                _ => return Err("intent_git_observation_failed".into()),
            }
            let mut refreshed = binding.clone();
            refreshed.head = self.head.clone();
            let admission = SemanticAdmission::new(
                key.clone(),
                snapshot.version().clone(),
                self.semantic_authority()?,
            );
            let committed = match DurableTransactionStore::commit_issue_local(
                &root,
                admission,
                crate::storage::semantic::LocalChange::AmendBinding(
                    crate::storage::semantic::VerifiedBindingAmendment::from_native_owner(
                        refreshed,
                    ),
                ),
            )
            .map_err(semantic_error)?
            {
                crate::storage::semantic::CommitOutcome::Committed(value)
                | crate::storage::semantic::CommitOutcome::Unchanged(value) => *value,
            };
            complete_projection(&root, &key, &committed)?;
            return Ok(true);
        }
        Ok(false)
    }
    pub fn snapshot(&self) -> Snapshot {
        Snapshot {
            repository: self.repository.clone(),
            issue: self.issue,
            platform: std::env::consts::OS.into(),
            authority: AuthorityVersion {
                selector_digest: self.authority_digest.clone(),
            },
            version: IssueVersion {
                generation: self.index["generation"].as_u64(),
                digest: self.index["digest"].as_str().map(str::to_owned),
            },
            semantic_version: None,
            checkout: CheckoutIdentity {
                branch: self.branch.clone(),
                head: self.head.clone(),
                root: self.root.clone(),
            },
        }
    }
    pub fn snapshot_for_intent(&self, command: &str) -> Snapshot {
        let mut snapshot = self.snapshot();
        if matches!(command, "rebuild" | "edit") {
            snapshot.semantic_version = self.semantic_root_key().ok().and_then(|(root, key)| {
                match DurableTransactionStore::observe_issue(&root, &key).ok()? {
                    Observation::Current(value) | Observation::ProjectionRepairRequired(value) => {
                        Some(IssueVersion {
                            generation: Some(value.version().generation()),
                            digest: Some(value.version().digest().as_str().to_owned()),
                        })
                    }
                    Observation::RecoveryRequired
                    | Observation::LegacyMigrationRequired
                    | Observation::Absent => None,
                }
            });
        }
        snapshot
    }
    pub fn fresh(&self) -> Result<(), String> {
        if Self::load_with_rollback_admission(&self.root, self.issue, self.rollback_admission)?
            .snapshot()
            != self.snapshot()
        {
            return Err("intent_snapshot_stale".into());
        }
        Ok(())
    }
    pub fn fresh_integrity(&self) -> Result<(), String> {
        self.fresh()?;
        if !self.index.is_null() {
            let recovery =
                crate::commands::local::intent::recovery_source(&self.state_root, self.issue)
                    .map_err(|_| "intent_recovery_journal_invalid")?;
            crate::commands::local::intent::verify_integrity(
                recovery.as_ref().unwrap_or(&self.issue_root),
                &self.index,
            )
            .map_err(|_| "intent_issue_integrity_mismatch")?;
        }
        Ok(())
    }
    pub fn registry(&self) -> Result<PromptRegistry, String> {
        let bytes = std::fs::read(self.root.join("docs/templates/prompts/current.json"))
            .map_err(|_| "intent_registry_missing")?;
        let mut registry =
            PromptRegistry::from_current_json(&bytes).map_err(|_| "intent_registry_invalid")?;
        for path in registry.template_paths.values_mut() {
            *path = self.root.join(&*path).to_string_lossy().into_owned();
        }
        Ok(registry)
    }
    pub fn local_request(&self) -> Result<LocalPreparationRequest, String> {
        if self.index.is_null() {
            return Err("missing_local_lifecycle_state".into());
        }
        let recovery =
            crate::commands::local::intent::recovery_source(&self.state_root, self.issue)
                .map_err(|_| "intent_recovery_journal_invalid")?;
        let sip = read_json(
            &recovery
                .as_ref()
                .unwrap_or(&self.issue_root)
                .join("cards/sip.values.json"),
        )?;
        Ok(LocalPreparationRequest {
            issue: self.issue,
            title: sip["title"]
                .as_str()
                .ok_or("intent_issue_title_missing")?
                .into(),
            repository: self.repository.clone(),
            branch: self.index["branch"]
                .as_str()
                .ok_or("intent_branch_missing")?
                .into(),
            worktree: self.index["worktree"]
                .as_str()
                .ok_or("intent_worktree_missing")?
                .into(),
            registry_version: self.index["template_registry_version"]
                .as_str()
                .ok_or("intent_registry_version_missing")?
                .into(),
            expected_lifecycle_digest: self.index["digest"].as_str().map(str::to_owned),
            commands: required_local_commands().to_vec(),
            card_updates: BTreeMap::new(),
            schedule_readiness: None,
            shepherd_routing: None,
        })
    }
    pub fn plan(&self) -> Result<IntentPlan, String> {
        self.fresh_integrity()?;
        let (root, key) = self.semantic_root_key()?;
        let value =
            match DurableTransactionStore::observe_issue(&root, &key).map_err(semantic_error)? {
                Observation::Current(value) | Observation::ProjectionRepairRequired(value) => {
                    serde_json::to_value(value.inputs().accepted_plan())
                        .map_err(|_| "intent_plan_invalid")?
                }
                Observation::Absent => read_json(&self.issue_root.join("intent-plan.json"))?,
                _ => return Err("intent_plan_missing".into()),
            };
        let plan: IntentPlan = serde_json::from_value(value).map_err(|_| "intent_plan_invalid")?;
        if plan.schema != "csdlc.v3.intent_plan.v1" {
            return Err("intent_plan_schema_unsupported".into());
        }
        Ok(plan)
    }
}

pub(crate) struct SemanticContext {
    pub(crate) root: SemanticRoot,
    pub(crate) key: IssueKey,
    pub(crate) snapshot: SemanticSnapshot,
    pub(crate) admission: SemanticAdmission,
    pub(crate) origin: EffectOrigin,
    primary: PathBuf,
    issue: u64,
}

impl SemanticContext {
    /// Strict pre-dispatch freshness. This re-runs native authority, repository,
    /// card and checkout guards after reservation while allowing its CAS increment.
    pub(crate) fn admit_before_effect(&self, operation: &OperationId) -> Result<(), String> {
        let retained = DurableTransactionStore::inspect_effect(&self.root, &self.key, operation)
            .map_err(semantic_error)?;
        let rollback = retained.request().command()
            == crate::lifecycle::semantic::SemanticCommand::RecordRollback;
        let refreshed = if rollback {
            Context::load_for_intent(&self.primary, self.issue, "recover")?
        } else {
            Context::load(&self.primary, self.issue)?
        };
        let current = if refreshed.cleanup_pending || rollback {
            refreshed.semantic_recovery_context(operation)?
        } else {
            refreshed.semantic_context()?
        };
        if current.key != self.key
            || current.snapshot.inputs_version() != self.snapshot.inputs_version()
            || current.snapshot.inputs().authority() != self.snapshot.inputs().authority()
        {
            return Err("intent_semantic_admission_changed".into());
        }
        if retained.ticket().is_none() {
            return Err("intent_semantic_operation_completed".into());
        }
        Ok(())
    }

    /// Re-authenticate after reservation without requiring the pre-reservation CAS.
    /// Pending identity and evidence-input version must remain exact. The returned
    /// capability is only attachment admission; it cannot authorize an effect.
    pub(crate) fn fresh_for_effect(
        &self,
        operation: &OperationId,
    ) -> Result<AttachmentAdmission, String> {
        let refreshed = Context::load(&self.primary, self.issue)?;
        self.fresh_for_effect_from(refreshed, operation)
    }

    pub(crate) fn fresh_for_recovery_effect(
        &self,
        operation: &OperationId,
    ) -> Result<AttachmentAdmission, String> {
        let refreshed = Context::load_for_intent(&self.primary, self.issue, "recover")?;
        self.fresh_for_effect_from(refreshed, operation)
    }

    fn fresh_for_effect_from(
        &self,
        refreshed: Context,
        operation: &OperationId,
    ) -> Result<AttachmentAdmission, String> {
        let (root, key) = refreshed.semantic_root_key()?;
        if key != self.key {
            return Err("intent_semantic_issue_changed".into());
        }
        let current = match DurableTransactionStore::observe_issue(&root, &key)
            .map_err(semantic_error)?
        {
            Observation::Current(snapshot) | Observation::ProjectionRepairRequired(snapshot) => {
                *snapshot
            }
            Observation::RecoveryRequired => return Err("intent_semantic_recovery_required".into()),
            Observation::LegacyMigrationRequired => {
                return Err("intent_semantic_migration_required".into())
            }
            Observation::Absent => return Err("intent_semantic_state_missing".into()),
        };
        if current.inputs_version() != self.snapshot.inputs_version() {
            return Err("intent_semantic_inputs_changed".into());
        }
        let inspection = DurableTransactionStore::inspect_effect(&root, &key, operation)
            .map_err(semantic_error)?;
        if inspection.ticket().is_none() {
            return Err("intent_semantic_operation_completed".into());
        }
        if let Some(target) = inspection.request().origin().bind_target() {
            let registration = blake3::hash(
                serde_json::to_string(&serde_json::json!({
                    "branch":target.branch,"worktree":target.worktree
                }))
                .map_err(|_| "intent_bind_identity_invalid")?
                .as_bytes(),
            )
            .to_hex()
            .to_string();
            if refreshed.root != target.worktree
                || refreshed.branch != target.branch
                || refreshed.head != target.head
                || target.registration != registration
            {
                return Err("intent_semantic_bind_target_changed".into());
            }
        }
        let authority = refreshed.semantic_authority()?;
        Ok(AttachmentAdmission::from_native_owner(
            authority,
            inspection.request().origin().clone(),
        ))
    }

    pub(crate) fn complete_projection(
        &self,
        snapshot: &SemanticSnapshot,
    ) -> Result<SemanticSnapshot, String> {
        complete_projection(&self.root, &self.key, snapshot)
    }
}

fn complete_projection(
    root: &SemanticRoot,
    key: &IssueKey,
    snapshot: &SemanticSnapshot,
) -> Result<SemanticSnapshot, String> {
    let proof =
        DurableTransactionStore::write_issue_projection(root, snapshot).map_err(semantic_error)?;
    let admission = SemanticAdmission::new(
        key.clone(),
        snapshot.version().clone(),
        snapshot.inputs().authority().clone(),
    );
    match DurableTransactionStore::commit_issue_local(
        root,
        admission,
        crate::storage::semantic::LocalChange::AcknowledgeProjection(proof),
    )
    .map_err(semantic_error)?
    {
        crate::storage::semantic::CommitOutcome::Committed(snapshot)
        | crate::storage::semantic::CommitOutcome::Unchanged(snapshot) => Ok(*snapshot),
    }
}

fn semantic_error(error: crate::storage::semantic::Error) -> String {
    format!("intent_semantic_{error:?}")
}
