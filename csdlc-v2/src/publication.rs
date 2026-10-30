use std::fs;
use std::path::{Path, PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumString};

use crate::error::{ErrorCode, Result, V2Error};
use crate::model::{IssueRecord, LifecyclePhase, PublicationEvidence};
use crate::review::evaluate_publication_review_in_repo;
use crate::Store;

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PublicationLinkageMode {
    #[default]
    Closing,
    PartOf,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub repository: String,
    #[serde(default)]
    pub code_repository: Option<String>,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub linkage_mode: PublicationLinkageMode,
    #[serde(default)]
    pub draft: bool,
    pub remote: String,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadyPublicationRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub actor: String,
    pub repository: String,
    #[serde(default)]
    pub code_repository: Option<String>,
    pub pull_request: u64,
    pub expected_head_sha: String,
    pub remote: String,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct ReadyPublicationReconciliationRequest {
    pub schema: String,
    pub ready: ReadyPublicationRequest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct PublicationIntent {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub issue_repository: String,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub linkage_mode: PublicationLinkageMode,
    pub draft: bool,
    pub revision: String,
    pub commit_sha: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RemotePullRequest {
    pub number: u64,
    pub url: String,
    pub repository: String,
    pub base: String,
    pub head: String,
    pub title: String,
    pub body: String,
    #[serde(default)]
    pub linkage_mode: PublicationLinkageMode,
    pub draft: bool,
    pub state: String,
    pub head_sha: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linked_issue: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub linkage_source: Option<String>,
}

pub fn body_has_github_closing_keyword(body: &str, issue: u64, repository: &str) -> bool {
    let issue_ref = format!("#{issue}");
    let qualified_issue_ref = format!("{repository}#{issue}").to_ascii_lowercase();
    body_has_closing_reference(body, |token| {
        token == issue_ref || token == qualified_issue_ref
    })
}

pub fn body_has_qualified_github_closing_keyword(body: &str, issue: u64, repository: &str) -> bool {
    let qualified_issue_ref = format!("{repository}#{issue}").to_ascii_lowercase();
    body_has_closing_reference(body, |token| token == qualified_issue_ref)
}

pub fn body_has_github_part_of_reference(body: &str, issue: u64, repository: &str) -> bool {
    let issue_ref = format!("#{issue}");
    let qualified_issue_ref = format!("{repository}#{issue}").to_ascii_lowercase();
    body_has_part_of_reference(body, |token| {
        token == issue_ref || token == qualified_issue_ref
    })
}

pub fn body_has_qualified_github_part_of_reference(
    body: &str,
    issue: u64,
    repository: &str,
) -> bool {
    let qualified_issue_ref = format!("{repository}#{issue}").to_ascii_lowercase();
    body_has_part_of_reference(body, |token| token == qualified_issue_ref)
}

fn body_has_part_of_reference(body: &str, references_issue: impl Fn(&str) -> bool) -> bool {
    body.lines().any(|line| {
        let tokens = line
            .split_whitespace()
            .map(|token| {
                token
                    .trim_matches(|c: char| {
                        matches!(
                            c,
                            ':' | ',' | ';' | '.' | '(' | ')' | '[' | ']' | '"' | '\''
                        )
                    })
                    .to_ascii_lowercase()
            })
            .collect::<Vec<_>>();
        tokens.len() == 3
            && tokens[0] == "part"
            && tokens[1] == "of"
            && references_issue(&tokens[2])
    })
}

pub fn body_has_exact_publication_linkage(
    body: &str,
    issue: u64,
    repository: &str,
    split_authority: bool,
    mode: PublicationLinkageMode,
) -> bool {
    let closing = if split_authority {
        body_has_qualified_github_closing_keyword(body, issue, repository)
    } else {
        body_has_github_closing_keyword(body, issue, repository)
    };
    let part_of = if split_authority {
        body_has_qualified_github_part_of_reference(body, issue, repository)
    } else {
        body_has_github_part_of_reference(body, issue, repository)
    };
    match mode {
        PublicationLinkageMode::Closing => closing && !part_of,
        PublicationLinkageMode::PartOf => part_of && !closing,
    }
}

fn body_has_closing_reference(body: &str, references_issue: impl Fn(&str) -> bool) -> bool {
    body.lines().any(|line| {
        let mut closing_keyword = false;
        for token in line.split_whitespace() {
            let token = token
                .trim_matches(|c: char| {
                    matches!(
                        c,
                        ':' | ',' | ';' | '.' | '(' | ')' | '[' | ']' | '"' | '\''
                    )
                })
                .to_ascii_lowercase();
            if matches!(
                token.as_str(),
                "close"
                    | "closes"
                    | "closed"
                    | "fix"
                    | "fixes"
                    | "fixed"
                    | "resolve"
                    | "resolves"
                    | "resolved"
            ) {
                closing_keyword = true;
                continue;
            }
            if closing_keyword && references_issue(&token) {
                return true;
            }
            closing_keyword = false;
        }
        false
    })
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PublicationAction {
    Create,
    Update,
    Noop,
}

pub fn reconcile_action(
    intent: &PublicationIntent,
    observed: Option<&RemotePullRequest>,
) -> Result<PublicationAction> {
    let Some(remote) = observed else {
        return Ok(PublicationAction::Create);
    };
    validate_remote_identity(intent, remote)?;
    if remote.title == intent.title && remote.body == intent.body && remote.draft == intent.draft {
        Ok(PublicationAction::Noop)
    } else {
        Ok(PublicationAction::Update)
    }
}

pub fn prepare_publication(
    store: &Store,
    request: &PublicationRequest,
) -> Result<PublicationIntent> {
    let split_authority = request
        .code_repository
        .as_deref()
        .is_some_and(|repository| repository != request.repository);
    let linkage_ok = body_has_exact_publication_linkage(
        &request.body,
        request.issue,
        &request.repository,
        split_authority,
        request.linkage_mode,
    );
    if request.schema != "csdlc.publication_request.v1"
        || request.repository.split_once('/').is_none()
        || request
            .code_repository
            .as_deref()
            .is_some_and(|repository| repository.split_once('/').is_none())
        || request.base.trim().is_empty()
        || request.head.trim().is_empty()
        || request.title.trim().is_empty()
        || !linkage_ok
        || !valid_remote_name(&request.remote)
        || !valid_ref_name(&request.base)
        || !valid_ref_name(&request.head)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "publication request identity or issue linkage is invalid",
        ));
    }
    let record = store.load_record(request.issue)?;
    verify_record(&record, request)?;
    let review = record
        .review
        .as_ref()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidTransition, "review evidence missing"))?;
    let revision = crate::git::substantive_revision(store.root(), &review.scope)?;
    let commit_sha = crate::git::run(store.root(), &["rev-parse", "HEAD"])?.stdout;
    if revision != crate::git::clean_commit_revision(&commit_sha) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication requires the reviewed substantive tree to be a clean commit",
        ));
    }
    let report =
        evaluate_publication_review_in_repo(store.root(), record.review.as_ref(), &revision);
    if !report.ready {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!(
                "publication review guard failed: {}",
                report.blocker_codes.join(",")
            ),
        ));
    }
    if crate::git::current_branch(store.root())? != request.head {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "current branch does not match publication head",
        ));
    }
    Ok(PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: request.issue,
        repository: request
            .code_repository
            .clone()
            .unwrap_or_else(|| request.repository.clone()),
        issue_repository: request.repository.clone(),
        base: request.base.clone(),
        head: request.head.clone(),
        title: request.title.clone(),
        body: request.body.clone(),
        linkage_mode: request.linkage_mode,
        draft: request.draft,
        revision,
        commit_sha,
    })
}

pub fn persist_publication_intent(root: &Path, intent: &PublicationIntent) -> Result<()> {
    let dir = publication_intent_dir(root)?;
    fs::create_dir_all(&dir)?;
    let target = dir.join(format!("{}.intent.json", intent.issue));
    let temporary = dir.join(format!(".{}.intent.tmp", intent.issue));
    fs::write(&temporary, serde_json::to_vec_pretty(intent)?)?;
    fs::rename(temporary, target)?;
    Ok(())
}

pub fn publication_intent_dir(root: &Path) -> Result<PathBuf> {
    Ok(PathBuf::from(
        crate::git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    )
    .join("csdlc-v2")
    .join("publication"))
}

pub fn resume_recorded_publication_intent(
    store: &Store,
    request: &PublicationRequest,
) -> Result<Option<PublicationIntent>> {
    let record = store.load_record(request.issue)?;
    let Some(publication) = record.publication.as_ref() else {
        return Ok(None);
    };
    let record_code_repository = record
        .code_repository
        .as_deref()
        .unwrap_or(&record.repository);
    let request_code_repository = request
        .code_repository
        .as_deref()
        .unwrap_or(&request.repository);
    if record.issue != request.issue
        || record.repository != request.repository
        || !record_code_repository.eq_ignore_ascii_case(request_code_repository)
        || publication.repository != request_code_repository
        || publication.base != request.base
        || publication.head != request.head
        || publication.draft != request.draft
        || publication.linkage_mode.unwrap_or_default() != request.linkage_mode
    {
        return Ok(None);
    }
    let Some(commit_sha) = parse_publication_clean_commit(&publication.revision) else {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "recorded publication revision cannot prove exact clean commit authority",
        ));
    };
    Ok(Some(PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: request.issue,
        repository: request_code_repository.to_owned(),
        issue_repository: request.repository.clone(),
        base: request.base.clone(),
        head: request.head.clone(),
        title: request.title.clone(),
        body: request.body.clone(),
        linkage_mode: request.linkage_mode,
        draft: request.draft,
        revision: publication.revision.clone(),
        commit_sha,
    }))
}

pub fn prepare_ready_publication(
    store: &Store,
    request: &ReadyPublicationRequest,
) -> Result<PublicationEvidence> {
    if request.schema != "csdlc.ready_publication_request.v1"
        || request.actor.trim().is_empty()
        || request.repository.split_once('/').is_none()
        || request
            .code_repository
            .as_deref()
            .is_some_and(|repository| repository.split_once('/').is_none())
        || request.pull_request == 0
        || !valid_remote_name(&request.remote)
        || !valid_head_sha(&request.expected_head_sha)
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "ready publication request identity is invalid",
        ));
    }
    let record = store.load_record(request.issue)?;
    if record.generation != request.expected_generation || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "ready publication request does not match canonical record",
        ));
    }
    if record.phase != LifecyclePhase::Published {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "mark-ready requires published phase",
        ));
    }
    let record_code_repository = record
        .code_repository
        .as_deref()
        .unwrap_or(&record.repository);
    let request_code_repository = request
        .code_repository
        .as_deref()
        .unwrap_or(&request.repository);
    if record.issue != request.issue
        || record.repository != request.repository
        || !record_code_repository.eq_ignore_ascii_case(request_code_repository)
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "ready publication request does not match canonical issue or code repository identity",
        ));
    }
    let mut publication = record.publication.clone().ok_or_else(|| {
        V2Error::new(ErrorCode::InvalidTransition, "publication evidence missing")
    })?;
    if publication.repository != request_code_repository
        || publication.issue != request.issue
        || publication.pull_request != request.pull_request
        || publication.observed_state != "open"
        || !publication.draft
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "mark-ready request differs from exact governed draft",
        ));
    }
    let current_head = current_head_sha(store.root())?;
    if current_head != request.expected_head_sha {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "ready publication requires the expected PR head to match the current checkout",
        ));
    }
    let observed_revision = crate::git::clean_commit_revision(&request.expected_head_sha);
    if publication.revision != observed_revision {
        let Some(from_commit) = parse_publication_clean_commit(&publication.revision) else {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "governed publication revision is not a clean commit identity",
            ));
        };
        let changed = crate::git::metadata_only_changed_paths(
            store.root(),
            &from_commit,
            &request.expected_head_sha,
        )
        .map_err(|_| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "ready head is not a forward metadata-only publication revision",
            )
        })?;
        if changed.is_empty() {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "ready head changed without typed publication metadata",
            ));
        }
        publication.revision = observed_revision;
    }
    let review = evaluate_publication_review_in_repo(
        store.root(),
        record.review.as_ref(),
        &publication.revision,
    );
    if !review.ready {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            format!(
                "mark-ready review guard failed: {}",
                review.blocker_codes.join(",")
            ),
        ));
    }
    Ok(publication)
}

pub fn prepare_ready_reconciliation(
    store: &Store,
    request: &ReadyPublicationReconciliationRequest,
) -> Result<PublicationEvidence> {
    if request.schema != "csdlc.ready_publication_reconciliation_request.v1" {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "ready publication reconciliation request identity is invalid",
        ));
    }
    prepare_ready_publication(store, &request.ready)
}

pub fn validate_ready_remote(
    governed: &PublicationEvidence,
    request: &ReadyPublicationRequest,
    remote: &RemotePullRequest,
    expected_draft: bool,
) -> Result<()> {
    let split_authority = request
        .code_repository
        .as_deref()
        .is_some_and(|repository| repository != request.repository);
    let linkage_mode = governed.linkage_mode.unwrap_or_default();
    let remote_closing_linkage_ok = match linkage_mode {
        PublicationLinkageMode::Closing => {
            remote.linked_issue == Some(request.issue)
                && remote.linkage_source.as_deref() == Some("github_closing_issues_references")
        }
        PublicationLinkageMode::PartOf => {
            remote.linked_issue.is_none() && remote.linkage_source.is_none()
        }
    };
    if governed.issue != request.issue
        || remote.repository != governed.repository
        || remote.number != request.pull_request
        || remote.base != governed.base
        || remote.head != governed.head
        || remote.head_sha != request.expected_head_sha
        || remote.draft != expected_draft
        || remote.state != "open"
        || !body_has_exact_publication_linkage(
            &remote.body,
            request.issue,
            &request.repository,
            split_authority,
            linkage_mode,
        )
        || !remote_closing_linkage_ok
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "ready PR observation differs from exact governed publication",
        ));
    }
    Ok(())
}

pub fn record_ready_publication(
    store: &Store,
    request: &ReadyPublicationRequest,
    governed: &PublicationEvidence,
    observed: RemotePullRequest,
) -> Result<IssueRecord> {
    validate_ready_remote(governed, request, &observed, false)?;
    let mut evidence = publication_evidence(request.issue, governed, observed);
    evidence.draft = false;
    store.commit_publication(
        request.issue,
        &request.expected_digest,
        request.actor.clone(),
        evidence,
    )
}

pub fn record_ready_reconciliation(
    store: &Store,
    request: &ReadyPublicationReconciliationRequest,
    governed: &PublicationEvidence,
    observed: RemotePullRequest,
) -> Result<IssueRecord> {
    record_ready_publication(store, &request.ready, governed, observed)
}

fn parse_publication_clean_commit(revision: &str) -> Option<String> {
    let commit = revision
        .strip_prefix("git-blake3:")
        .and_then(|value| value.split_once(':'))
        .map(|(commit, _)| commit)
        .filter(|commit| {
            commit.len() == 40 && commit.bytes().all(|byte| byte.is_ascii_hexdigit())
        })?;
    (revision == crate::git::clean_commit_revision(commit)).then(|| commit.to_owned())
}

pub fn commit_publication_metadata_tail(root: &Path, issue: u64) -> Result<Option<String>> {
    let issue_dir = format!(".csdlc/issues/{issue}");
    let already_staged = crate::git::run(root, &["diff", "--cached", "--name-only"])?.stdout;
    if already_staged
        .lines()
        .any(|path| !publication_metadata_path(issue, path))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication metadata tail cannot proceed with pre-staged non-governed paths",
        ));
    }
    let status = crate::git::run(
        root,
        &[
            "status",
            "--porcelain",
            "--untracked-files=all",
            "--",
            &issue_dir,
        ],
    )?
    .stdout;
    if status.is_empty() {
        return Ok(None);
    }
    let dirty_issue_paths = crate::git::run(
        root,
        &[
            "ls-files",
            "--others",
            "--modified",
            "--deleted",
            "--exclude-standard",
            "--",
            &issue_dir,
        ],
    )?
    .stdout;
    if dirty_issue_paths
        .lines()
        .any(|path| !publication_metadata_path(issue, path))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication metadata tail contains non-governed paths",
        ));
    }

    let before = current_head_sha(root)?;
    crate::git::run(root, &["add", &issue_dir])?;
    let staged =
        crate::git::run(root, &["diff", "--cached", "--name-only", "--", &issue_dir])?.stdout;
    if staged.is_empty() {
        return Ok(None);
    }
    if staged
        .lines()
        .any(|path| !publication_metadata_path(issue, path))
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication metadata tail contains non-governed paths",
        ));
    }

    let message = format!("Record C-SDLC publication metadata for #{issue}");
    crate::git::run(root, &["commit", "-m", &message, "--", &issue_dir])?;
    let after = current_head_sha(root)?;
    if !matches!(
        crate::git::metadata_only_changed_paths(root, &before, &after),
        Ok(paths) if !paths.is_empty()
    ) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication metadata tail was not metadata-only",
        ));
    }
    Ok(Some(after))
}

pub fn current_head_sha(root: &Path) -> Result<String> {
    Ok(crate::git::run(root, &["rev-parse", "HEAD"])?
        .stdout
        .trim()
        .to_owned())
}

fn publication_metadata_path(issue: u64, path: &str) -> bool {
    let prefix = format!(".csdlc/issues/{issue}/");
    let Some(rest) = path.strip_prefix(&prefix) else {
        return false;
    };
    if rest.contains('/') {
        let Some(file) = rest.strip_prefix("cards/") else {
            return false;
        };
        let names = ["sip", "stp", "spp", "vpp", "srp", "sor"];
        return names
            .iter()
            .any(|name| file == format!("{name}.md") || file == format!("{name}.values.json"));
    }
    matches!(rest, "index.json" | "audit.jsonl")
}

pub fn governed_publication_metadata_path(issue: u64, path: &str) -> bool {
    publication_metadata_path(issue, path)
}

pub fn governed_publication_metadata_followup_paths(
    root: &Path,
    issue: u64,
    published_head: &str,
    metadata_head: &str,
) -> Result<Vec<String>> {
    let paths = crate::git::metadata_only_changed_paths(root, published_head, metadata_head)?;
    if paths.is_empty()
        || paths
            .iter()
            .any(|path| !governed_publication_metadata_path(issue, path))
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "metadata publication head is not a governed metadata-only follow-up",
        ));
    }
    Ok(paths)
}

fn verify_record(record: &IssueRecord, request: &PublicationRequest) -> Result<()> {
    let record_code_repository = record
        .code_repository
        .as_deref()
        .unwrap_or(&record.repository);
    let request_code_repository = request
        .code_repository
        .as_deref()
        .unwrap_or(&request.repository);
    if record.issue != request.issue
        || record.repository != request.repository
        || !record_code_repository.eq_ignore_ascii_case(request_code_repository)
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "publication request does not match canonical issue or code repository identity",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "publication requires reviewed or published phase",
        ));
    }
    Ok(())
}

pub fn validate_remote(intent: &PublicationIntent, remote: &RemotePullRequest) -> Result<()> {
    validate_remote_identity(intent, remote)?;
    if remote.head_sha != intent.commit_sha
        || remote.title != intent.title
        || remote.body != intent.body
        || remote.draft != intent.draft
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR did not converge to the exact reviewed publication intent",
        ));
    }
    Ok(())
}

fn validate_remote_identity(intent: &PublicationIntent, remote: &RemotePullRequest) -> Result<()> {
    let linkage_ok = body_has_exact_publication_linkage(
        &remote.body,
        intent.issue,
        &intent.issue_repository,
        intent.repository != intent.issue_repository,
        intent.linkage_mode,
    );
    let remote_closing_linkage_ok = match intent.linkage_mode {
        PublicationLinkageMode::Closing => {
            remote.linked_issue == Some(intent.issue)
                && remote.linkage_source.as_deref() == Some("github_closing_issues_references")
        }
        PublicationLinkageMode::PartOf => {
            remote.linked_issue.is_none() && remote.linkage_source.is_none()
        }
    };
    if remote.repository != intent.repository
        || remote.base != intent.base
        || remote.head != intent.head
        || remote.linkage_mode != intent.linkage_mode
        || !linkage_ok
        || !remote_closing_linkage_ok
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR identity differs from publication intent",
        ));
    }
    Ok(())
}

fn valid_remote_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-'))
}

pub fn record_publication(
    store: &Store,
    request: &PublicationRequest,
    intent: &PublicationIntent,
    remote: RemotePullRequest,
) -> Result<IssueRecord> {
    validate_remote(intent, &remote)?;
    let evidence = publication_evidence(request.issue, intent, remote);
    let current = store.load_record(request.issue)?;
    if current.digest == request.expected_digest && current.publication.as_ref() == Some(&evidence)
    {
        return Ok(current);
    }
    store.commit_publication(
        request.issue,
        &request.expected_digest,
        request.actor.clone(),
        evidence,
    )
}

fn publication_evidence(
    issue: u64,
    intent: &impl PublicationEvidenceSource,
    remote: RemotePullRequest,
) -> PublicationEvidence {
    PublicationEvidence {
        repository: remote.repository,
        issue,
        pull_request: remote.number,
        url: remote.url,
        base: remote.base,
        head: remote.head,
        revision: intent.revision().to_owned(),
        linkage_mode: Some(intent.linkage_mode()),
        draft: remote.draft,
        observed_state: remote.state,
    }
}

trait PublicationEvidenceSource {
    fn revision(&self) -> &str;
    fn linkage_mode(&self) -> PublicationLinkageMode;
}

impl PublicationEvidenceSource for PublicationIntent {
    fn revision(&self) -> &str {
        &self.revision
    }

    fn linkage_mode(&self) -> PublicationLinkageMode {
        self.linkage_mode
    }
}

impl PublicationEvidenceSource for PublicationEvidence {
    fn revision(&self) -> &str {
        &self.revision
    }

    fn linkage_mode(&self) -> PublicationLinkageMode {
        self.linkage_mode.unwrap_or_default()
    }
}

fn valid_ref_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.contains("..")
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/'))
}

fn valid_head_sha(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}
