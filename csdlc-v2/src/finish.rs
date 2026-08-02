use std::fs::{self, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use fs2::FileExt;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::error::{ErrorCode, Result, V2Error};
use crate::git::clean_commit_revision;
use crate::github::PrStatePacket;
use crate::merge::{MergeMethod, MergeRequest};
use crate::model::{IssueRecord, LifecyclePhase};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum FinishDisposition {
    Merged,
    ClosedUnmerged,
    ClosedNoPr,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishRequest {
    pub schema: String,
    pub issue: u64,
    pub expected_generation: u64,
    pub expected_digest: String,
    pub claim_id: String,
    pub actor: String,
    pub repository: String,
    pub pull_request: Option<u64>,
    pub base: Option<String>,
    pub head: Option<String>,
    pub expected_head_sha: Option<String>,
    pub merge_method: MergeMethod,
    #[serde(default)]
    pub required_checks: Vec<String>,
    #[serde(default)]
    pub require_review: bool,
    pub approved_no_pr_reason: Option<String>,
    pub token_file: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DerivedTerminalEnvelope {
    pub schema: String,
    pub issue: u64,
    pub repository: String,
    pub initialization_digest: String,
    pub pull_request: Option<u64>,
    pub disposition: FinishDisposition,
    pub head_sha: Option<String>,
    pub merge_sha: Option<String>,
    pub issue_state: String,
    pub pr_state: Option<String>,
    pub approved_reason: Option<String>,
    pub claim_logically_released: bool,
    pub source: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct FinishResult {
    pub schema: String,
    pub terminal: DerivedTerminalEnvelope,
    pub already_terminal: bool,
}

pub fn validate_request(request: &FinishRequest) -> Result<()> {
    if request.schema != "csdlc.finish_request.v1"
        || request.issue == 0
        || request.repository.split_once('/').is_none()
        || request.expected_digest.trim().is_empty()
        || request.claim_id.trim().is_empty()
        || request.actor.trim().is_empty()
    {
        return Err(V2Error::new(
            ErrorCode::InvalidInput,
            "finish request identity is incomplete",
        ));
    }
    match request.pull_request {
        Some(0) => Err(V2Error::new(
            ErrorCode::InvalidInput,
            "pull request must be nonzero",
        )),
        Some(_)
            if request.base.as_deref().is_none_or(str::is_empty)
                || request.head.as_deref().is_none_or(str::is_empty)
                || request
                    .expected_head_sha
                    .as_deref()
                    .is_none_or(str::is_empty) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "PR finish requires base, head, and expected head SHA",
            ))
        }
        None if request
            .approved_no_pr_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty()) =>
        {
            Err(V2Error::new(
                ErrorCode::InvalidInput,
                "no-PR finish requires an approved reason",
            ))
        }
        _ => Ok(()),
    }
}

pub fn validate_canonical_identity(record: &IssueRecord, request: &FinishRequest) -> Result<()> {
    validate_request(request)?;
    if record.issue != request.issue
        || record.repository != request.repository
        || record.generation != request.expected_generation
        || record.digest != request.expected_digest
    {
        return Err(V2Error::new(
            ErrorCode::StaleDigest,
            "finish request does not match canonical issue identity or digest",
        ));
    }
    if !matches!(
        record.phase,
        LifecyclePhase::Reviewed | LifecyclePhase::Published | LifecyclePhase::MergeReady
    ) {
        return Err(V2Error::new(
            ErrorCode::InvalidTransition,
            "finish requires reviewed, published, or merge_ready pre-merge truth",
        ));
    }
    if let Some(number) = request.pull_request {
        let publication = record.publication.as_ref().ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "PR finish requires canonical publication evidence",
            )
        })?;
        if publication.repository != request.repository
            || publication.issue != request.issue
            || publication.pull_request != number
            || publication.base != request.base.as_deref().unwrap_or_default()
            || publication.head != request.head.as_deref().unwrap_or_default()
            || publication.revision
                != clean_commit_revision(request.expected_head_sha.as_deref().unwrap_or_default())
        {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "canonical publication does not match the exact finish request",
            ));
        }
    }
    Ok(())
}

pub fn as_merge_request(request: &FinishRequest) -> Result<MergeRequest> {
    Ok(MergeRequest {
        schema: "csdlc.merge_request.v1".into(),
        issue: request.issue,
        expected_generation: request.expected_generation,
        expected_digest: request.expected_digest.clone(),
        claim_id: request.claim_id.clone(),
        actor: request.actor.clone(),
        repository: request.repository.clone(),
        pull_request: request.pull_request.ok_or_else(|| {
            V2Error::new(ErrorCode::InvalidInput, "merge requires a pull request")
        })?,
        base: request.base.clone().unwrap_or_default(),
        head: request.head.clone().unwrap_or_default(),
        expected_head_sha: request.expected_head_sha.clone().unwrap_or_default(),
        merge_method: request.merge_method,
        required_checks: request.required_checks.clone(),
        require_review: request.require_review,
        token_file: request.token_file.clone(),
    })
}

pub fn derive_terminal(
    record: &IssueRecord,
    request: &FinishRequest,
    issue_state: &str,
    packet: Option<&PrStatePacket>,
) -> Result<Option<DerivedTerminalEnvelope>> {
    validate_canonical_identity(record, request)?;
    let (disposition, pull_request, head_sha, merge_sha, pr_state) = match packet {
        Some(packet) => {
            validate_packet_identity(request, packet)?;
            if packet.merged {
                let merge_sha = packet.merge_commit_sha.clone().ok_or_else(|| {
                    V2Error::new(
                        ErrorCode::ReconciliationRequired,
                        "merged PR has no merge commit SHA",
                    )
                })?;
                (
                    FinishDisposition::Merged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    Some(merge_sha),
                    Some(packet.state.clone()),
                )
            } else if packet.state == "closed" && issue_state == "closed" {
                (
                    FinishDisposition::ClosedUnmerged,
                    Some(packet.pull_request),
                    Some(packet.head_sha.clone()),
                    None,
                    Some(packet.state.clone()),
                )
            } else {
                return Ok(None);
            }
        }
        None if issue_state == "closed" => (FinishDisposition::ClosedNoPr, None, None, None, None),
        None => return Ok(None),
    };
    let mut envelope = DerivedTerminalEnvelope {
        schema: "csdlc.derived_terminal.v1".into(),
        issue: record.issue,
        repository: record.repository.clone(),
        initialization_digest: record.initialization_digest.clone(),
        pull_request,
        disposition,
        head_sha,
        merge_sha,
        issue_state: if disposition == FinishDisposition::Merged {
            "closed_by_merged_pr".into()
        } else {
            issue_state.into()
        },
        pr_state,
        approved_reason: request.approved_no_pr_reason.clone(),
        claim_logically_released: true,
        source: "live_github".into(),
        digest: String::new(),
    };
    envelope.digest = envelope_digest(&envelope)?;
    Ok(Some(envelope))
}

fn validate_packet_identity(request: &FinishRequest, packet: &PrStatePacket) -> Result<()> {
    if packet.repository != request.repository
        || Some(packet.pull_request) != request.pull_request
        || packet.linked_issue != Some(request.issue)
        || packet.base_ref.as_deref() != request.base.as_deref()
        || packet.head_ref.as_deref() != request.head.as_deref()
        || Some(packet.head_sha.as_str()) != request.expected_head_sha.as_deref()
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "remote PR does not match the exact finish identity",
        ));
    }
    Ok(())
}

pub fn validate_envelope(envelope: &DerivedTerminalEnvelope) -> Result<()> {
    if envelope.schema != "csdlc.derived_terminal.v1"
        || envelope.issue == 0
        || envelope.repository.split_once('/').is_none()
        || envelope.initialization_digest.trim().is_empty()
        || !envelope.claim_logically_released
        || envelope.source != "live_github"
        || envelope.digest != envelope_digest(envelope)?
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal envelope is invalid",
        ));
    }
    if envelope.disposition == FinishDisposition::Merged
        && (envelope.pull_request.is_none()
            || envelope.head_sha.as_deref().is_none_or(str::is_empty)
            || envelope.merge_sha.as_deref().is_none_or(str::is_empty))
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "merged terminal envelope is incomplete",
        ));
    }
    if envelope.disposition == FinishDisposition::ClosedNoPr
        && envelope
            .approved_reason
            .as_deref()
            .is_none_or(|reason| reason.trim().is_empty())
    {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "closed-no-PR terminal envelope has no approved reason",
        ));
    }
    Ok(())
}

pub fn envelope_matches_record(
    envelope: &DerivedTerminalEnvelope,
    record: &IssueRecord,
) -> Result<bool> {
    validate_envelope(envelope)?;
    Ok(envelope.issue == record.issue
        && envelope.repository == record.repository
        && envelope.initialization_digest == record.initialization_digest)
}

pub fn terminal_cache_path(root: &Path, issue: u64) -> Result<PathBuf> {
    let common = crate::git::run(
        root,
        &["rev-parse", "--path-format=absolute", "--git-common-dir"],
    )?
    .stdout;
    Ok(PathBuf::from(common)
        .join("csdlc-v2/derived-terminal")
        .join(format!("{issue}.json")))
}

pub fn load_cached_terminal(root: &Path, issue: u64) -> Result<Option<DerivedTerminalEnvelope>> {
    let path = terminal_cache_path(root, issue)?;
    if !path.exists() {
        return Ok(None);
    }
    validate_cache_parent(root, false)?;
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache is not a regular file",
        ));
    }
    let envelope: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(path)?)?;
    validate_envelope(&envelope)?;
    if envelope.issue != issue {
        return Err(V2Error::new(
            ErrorCode::CorruptRecord,
            "derived terminal cache namespace mismatch",
        ));
    }
    Ok(Some(envelope))
}

pub fn retain_cached_terminal(root: &Path, envelope: &DerivedTerminalEnvelope) -> Result<PathBuf> {
    validate_envelope(envelope)?;
    let path = terminal_cache_path(root, envelope.issue)?;
    let parent = validate_cache_parent(root, true)?;
    let lock_path = parent.join(format!(".{}.lock", envelope.issue));
    if fs::symlink_metadata(&lock_path)
        .is_ok_and(|metadata| metadata.file_type().is_symlink() || !metadata.file_type().is_file())
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache lock is not a regular file",
        ));
    }
    let mut lock_options = OpenOptions::new();
    lock_options
        .create(true)
        .truncate(false)
        .read(true)
        .write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        lock_options.custom_flags(libc::O_NOFOLLOW);
    }
    let lock = lock_options.open(&lock_path)?;
    lock.lock_exclusive()?;
    if path.exists() {
        let existing: DerivedTerminalEnvelope = serde_json::from_slice(&fs::read(&path)?)?;
        validate_envelope(&existing)?;
        if existing == *envelope {
            FileExt::unlock(&lock)?;
            return Ok(path);
        }
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "derived terminal cache conflicts with a retained terminal observation",
        ));
    }
    let nonce = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|error| V2Error::new(ErrorCode::InvalidInput, error.to_string()))?
        .as_nanos();
    let temp = parent.join(format!(
        ".{}.{}.{}.tmp",
        envelope.issue,
        std::process::id(),
        nonce
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    serde_json::to_writer_pretty(&mut file, envelope)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    fs::rename(&temp, &path)?;
    File::open(parent)?.sync_all()?;
    FileExt::unlock(&lock)?;
    Ok(path)
}

fn validate_cache_parent(root: &Path, create: bool) -> Result<PathBuf> {
    let common = PathBuf::from(
        crate::git::run(
            root,
            &["rev-parse", "--path-format=absolute", "--git-common-dir"],
        )?
        .stdout,
    );
    let mut current = common.clone();
    for component in ["csdlc-v2", "derived-terminal"] {
        current.push(component);
        match fs::symlink_metadata(&current) {
            Ok(metadata) if metadata.file_type().is_dir() && !metadata.file_type().is_symlink() => {
            }
            Ok(_) => {
                return Err(V2Error::new(
                    ErrorCode::UnsafeCheckout,
                    "derived terminal cache directory is not a real directory",
                ));
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound && create => {
                match fs::create_dir(&current) {
                    Ok(()) => {}
                    Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => {
                        let metadata = fs::symlink_metadata(&current)?;
                        if metadata.file_type().is_symlink() || !metadata.file_type().is_dir() {
                            return Err(V2Error::new(
                                ErrorCode::UnsafeCheckout,
                                "concurrently created terminal cache path is unsafe",
                            ));
                        }
                    }
                    Err(error) => return Err(error.into()),
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(current),
            Err(error) => return Err(error.into()),
        }
    }
    let canonical_common = fs::canonicalize(&common)?;
    let canonical_parent = fs::canonicalize(&current)?;
    if !canonical_parent.starts_with(canonical_common) {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "derived terminal cache escapes the Git common directory",
        ));
    }
    Ok(current)
}

fn envelope_digest(envelope: &DerivedTerminalEnvelope) -> Result<String> {
    let mut canonical = envelope.clone();
    canonical.digest.clear();
    Ok(blake3::hash(&serde_json::to_vec(&canonical)?)
        .to_hex()
        .to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::github::PrCheck;
    use crate::model::{DesignReview, IssueRecord};
    use std::collections::BTreeMap;

    fn record() -> IssueRecord {
        IssueRecord {
            schema: "csdlc.issue.v2".into(),
            issue: 7,
            repository: "owner/repo".into(),
            initialization_digest: "init".into(),
            phase: LifecyclePhase::MergeReady,
            generation: 3,
            digest: "digest".into(),
            claim: None,
            review_assignment: None,
            review: None,
            publication: Some(crate::model::PublicationEvidence {
                repository: "owner/repo".into(),
                issue: 7,
                pull_request: 9,
                url: "https://example.test/pr/9".into(),
                base: "main".into(),
                head: "codex/7".into(),
                revision: clean_commit_revision("abc"),
                draft: false,
                observed_state: "open".into(),
            }),
            readiness: None,
            terminal: None,
            migration: None,
            design_path: "design.md".into(),
            diagram_path: "diagram.mmd".into(),
            design_review: DesignReview::Approved {
                reviewer: "reviewer".into(),
                revision: "abc".into(),
            },
            cards: BTreeMap::new(),
            transitions: vec![],
            audit: vec![],
        }
    }

    fn request() -> FinishRequest {
        FinishRequest {
            schema: "csdlc.finish_request.v1".into(),
            issue: 7,
            expected_generation: 3,
            expected_digest: "digest".into(),
            claim_id: "claim".into(),
            actor: "agent".into(),
            repository: "owner/repo".into(),
            pull_request: Some(9),
            base: Some("main".into()),
            head: Some("codex/7".into()),
            expected_head_sha: Some("abc".into()),
            merge_method: MergeMethod::Squash,
            required_checks: vec!["ci".into()],
            require_review: true,
            approved_no_pr_reason: None,
            token_file: None,
        }
    }

    fn packet() -> PrStatePacket {
        PrStatePacket {
            schema: "csdlc.github_pr_state.v1".into(),
            repository: "owner/repo".into(),
            pull_request: 9,
            linked_issue: Some(7),
            linkage_source: Some("github".into()),
            state: "closed".into(),
            draft: false,
            merge_state: "unknown".into(),
            review_decision: "approved".into(),
            base_ref: Some("main".into()),
            head_ref: Some("codex/7".into()),
            head_sha: "abc".into(),
            url: None,
            body: None,
            merged: true,
            merge_commit_sha: Some("def".into()),
            checks: vec![PrCheck {
                name: "ci".into(),
                required: true,
                conclusion: "success".into(),
                details_url: None,
            }],
            required_check_names: vec!["ci".into()],
            classification: "merged".into(),
        }
    }

    #[test]
    fn merged_pr_derives_terminal_without_mutating_record() {
        let terminal = derive_terminal(&record(), &request(), "closed", Some(&packet()))
            .expect("derive")
            .expect("terminal");
        assert_eq!(terminal.disposition, FinishDisposition::Merged);
        assert!(envelope_matches_record(&terminal, &record()).unwrap());
    }

    #[test]
    fn open_pr_is_not_terminal() {
        let mut packet = packet();
        packet.state = "open".into();
        packet.merged = false;
        packet.merge_commit_sha = None;
        assert!(
            derive_terminal(&record(), &request(), "open", Some(&packet))
                .unwrap()
                .is_none()
        );
    }
}
