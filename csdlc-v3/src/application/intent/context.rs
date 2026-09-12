use super::{read_json, AuthorityVersion, CheckoutIdentity, IntentPlan, IssueVersion, Snapshot};
use crate::commands::local::{required_local_commands, LocalPreparationRequest, PromptRegistry};
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
    pub state_root: PathBuf,
    pub issue_root: PathBuf,
    pub issue: u64,
    pub repository: String,
    pub branch: String,
    pub head: String,
    pub authority_digest: String,
    pub index: Value,
    pub cleanup_pending: bool,
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

impl Context {
    pub fn load(start: &Path, issue: u64) -> Result<Self, String> {
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
        let registrations = git(&invoking, &["worktree", "list", "--porcelain"])?;
        let mut candidates = Vec::new();
        let prepared = common.join(format!("csdlc-v3/local/issues/{issue}"));
        let prepared_recovery =
            crate::commands::local::intent::recovery_source(&common.join("csdlc-v3/local"), issue)
                .map_err(|_| "intent_recovery_journal_invalid")?;
        if prepared.join("index.json").exists() || prepared_recovery.is_some() {
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
        if candidates.len() > 1 {
            return Err("intent_issue_topology_ambiguous".into());
        }
        let mut archived_index = None;
        if candidates.is_empty() {
            let binding_path = common.join(format!("csdlc-v3/local/bindings/{issue}.json"));
            if binding_path.exists() {
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
                    if let Some(index) =
                        crate::commands::terminal::retained_cleanup_index(&primary, &target, issue)
                            .map_err(|finding| finding.code)?
                    {
                        candidates.push((
                            target.clone(),
                            target.join(format!(".csdlc/issues/{issue}")),
                        ));
                        archived_index = Some(index);
                    }
                }
            }
        }
        let (root, issue_root) = candidates.pop().unwrap_or((primary.clone(), prepared));
        let state_root = issue_root
            .parent()
            .and_then(Path::parent)
            .ok_or("intent_state_root_invalid")?
            .to_path_buf();
        let recovered_source = crate::commands::local::intent::recovery_source(&state_root, issue)
            .map_err(|_| "intent_recovery_journal_invalid")?;
        let read_root = recovered_source.as_ref().unwrap_or(&issue_root);
        let cleanup_pending = archived_index.is_some();
        let index = if read_root.join("index.json").exists() {
            read_json(&read_root.join("index.json"))?
        } else {
            archived_index.unwrap_or(Value::Null)
        };
        let remote = git(&root, &["remote", "get-url", "origin"])?;
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
        if !index.is_null() && (index["issue"] != issue || index["repository"] != repository) {
            return Err("intent_issue_repository_mismatch".into());
        }
        let authority_digest = crate::commands::remote::canonical_authority_selector_digest(&root)
            .map_err(|finding| finding.code)?;
        if crate::authority::canonical_v3_authority(&root)?.is_none() {
            return Err("intent_operational_authority_required".into());
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
            state_root,
            issue_root,
            issue,
            repository,
            branch,
            head,
            authority_digest,
            index,
            cleanup_pending,
        })
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
            checkout: CheckoutIdentity {
                branch: self.branch.clone(),
                head: self.head.clone(),
                root: self.root.clone(),
            },
        }
    }
    pub fn fresh(&self) -> Result<(), String> {
        if Self::load(&self.root, self.issue)?.snapshot() != self.snapshot() {
            return Err("intent_snapshot_stale".into());
        }
        Ok(())
    }
    pub fn fresh_integrity(&self) -> Result<(), String> {
        self.fresh()?;
        if !self.index.is_null() {
            crate::commands::local::intent::verify_integrity(&self.issue_root, &self.index)
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
        let plan: IntentPlan =
            serde_json::from_value(read_json(&self.issue_root.join("intent-plan.json"))?)
                .map_err(|_| "intent_plan_invalid")?;
        if plan.schema != "csdlc.v3.intent_plan.v1" {
            return Err("intent_plan_schema_unsupported".into());
        }
        Ok(plan)
    }
}
