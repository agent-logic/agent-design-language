use std::collections::BTreeSet;
use std::fs;
use std::path::{Component, Path, PathBuf};
use std::process::Command;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, EnumString};
use time::OffsetDateTime;

use crate::cutover::CutoverEvidence;
use crate::error::{ErrorCode, Result, V2Error};
use crate::proof::{require_clean_revision, PreSwitchEvidence};
use crate::{Generation, GenerationSelector};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum EntryDisposition {
    Remove,
    Retain,
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Serialize,
    Deserialize,
    JsonSchema,
    Display,
    EnumString,
    AsRefStr,
    EnumIter,
)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DeletionReason {
    MissingApproval,
    ApprovalEvidenceMismatch,
    PhaseEvidenceNotGreen,
    SelectorNotV2,
    ProtectedWindowActive,
    BelowMinimumRemoval,
    QualificationNotApproved,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionEntry {
    pub path: PathBuf,
    pub disposition: EntryDisposition,
    pub measured_lines: u64,
    pub owner: Option<String>,
    pub justification: Option<String>,
    pub protected_until: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionManifest {
    pub schema: String,
    pub baseline_lines: u64,
    pub target_percent: u16,
    pub minimum_percent: u16,
    pub entries: Vec<DeletionEntry>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionApproval {
    pub schema: String,
    pub approved_by: String,
    pub approved_at: String,
    pub phase_c_blake3: String,
    pub manifest_blake3: String,
    pub allow_qualified_80_to_89: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct DeletionEligibilityRequest {
    pub schema: String,
    pub issue: u64,
    pub phase_b_evidence: PathBuf,
    pub phase_c_evidence: PathBuf,
    pub selector: PathBuf,
    pub manifest: DeletionManifest,
    pub approval: Option<DeletionApproval>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct DeletionDecision {
    pub schema: String,
    pub issue: u64,
    pub code_revision: String,
    pub evaluated_at: String,
    pub phase_b_blake3: String,
    pub phase_c_blake3: String,
    pub selector_blake3: String,
    pub manifest_blake3: String,
    pub removed_lines: u64,
    pub retained_lines: u64,
    pub removal_basis_points: u16,
    pub target_met: bool,
    pub eligible: bool,
    pub reasons: Vec<DeletionReason>,
    pub deletion_executed: bool,
}

pub fn evaluate_deletion_eligibility(
    repo: &Path,
    request: &DeletionEligibilityRequest,
) -> Result<DeletionDecision> {
    evaluate_with_time(repo, request, OffsetDateTime::now_utc())
}

fn evaluate_with_time(
    repo: &Path,
    request: &DeletionEligibilityRequest,
    now: OffsetDateTime,
) -> Result<DeletionDecision> {
    validate_request(request)?;
    require_clean_revision(repo)?;
    let phase_b_bytes = read_regular_repo_file(repo, &request.phase_b_evidence)?;
    let phase_c_bytes = read_regular_repo_file(repo, &request.phase_c_evidence)?;
    let selector_bytes = read_regular_repo_file(repo, &request.selector)?;
    let phase_b: PreSwitchEvidence = serde_json::from_slice(&phase_b_bytes)?;
    let phase_c: CutoverEvidence = serde_json::from_slice(&phase_c_bytes)?;
    let selector: GenerationSelector = serde_json::from_slice(&selector_bytes)?;
    let manifest_bytes = serde_json::to_vec(&request.manifest)?;
    let phase_b_blake3 = digest(&phase_b_bytes);
    let phase_c_blake3 = digest(&phase_c_bytes);
    let selector_blake3 = digest(&selector_bytes);
    let manifest_blake3 = digest(&manifest_bytes);
    let mut reasons = BTreeSet::new();

    if phase_b.schema != "csdlc.pre_switch_evidence.v1"
        || phase_c.schema != "csdlc.cutover_evidence.v1"
        || !phase_b.passed
        || phase_b.default_before != Generation::V1
        || phase_b.default_after != Generation::V1
        || !phase_b.v1_paths_before
        || !phase_b.v1_paths_after
        || !phase_c.passed
        || phase_c.final_generation != Generation::V2
        || !phase_c.explicit_v1_override
        || !phase_c.v1_paths_before
        || !phase_c.v1_paths_after
        || phase_c.deletion_authorized
    {
        reasons.insert(DeletionReason::PhaseEvidenceNotGreen);
    }
    if selector.schema != "csdlc.generation_selector.v1"
        || selector.default_generation != Generation::V2
    {
        reasons.insert(DeletionReason::SelectorNotV2);
    }

    let removed_lines = request
        .manifest
        .entries
        .iter()
        .filter(|entry| entry.disposition == EntryDisposition::Remove)
        .map(|entry| entry.measured_lines)
        .sum::<u64>();
    let retained_lines = request.manifest.baseline_lines - removed_lines;
    let basis_points = removed_lines
        .saturating_mul(10_000)
        .checked_div(request.manifest.baseline_lines)
        .unwrap_or_default()
        .try_into()
        .unwrap_or(u16::MAX);
    if basis_points < request.manifest.minimum_percent.saturating_mul(100) {
        reasons.insert(DeletionReason::BelowMinimumRemoval);
    }

    for entry in request
        .manifest
        .entries
        .iter()
        .filter(|entry| entry.disposition == EntryDisposition::Remove)
    {
        if let Some(value) = &entry.protected_until {
            let until = parse_time(value)?;
            if now < until {
                reasons.insert(DeletionReason::ProtectedWindowActive);
            }
        }
    }

    match &request.approval {
        None => {
            reasons.insert(DeletionReason::MissingApproval);
        }
        Some(approval) => {
            validate_approval(approval)?;
            let approved_at = parse_time(&approval.approved_at)?;
            let cutover_at = parse_time(&phase_c.cutover_at)?;
            if approval.phase_c_blake3 != phase_c_blake3
                || approval.manifest_blake3 != manifest_blake3
                || approved_at < cutover_at
                || approved_at > now
            {
                reasons.insert(DeletionReason::ApprovalEvidenceMismatch);
            }
            if basis_points < request.manifest.target_percent.saturating_mul(100)
                && !approval.allow_qualified_80_to_89
            {
                reasons.insert(DeletionReason::QualificationNotApproved);
            }
        }
    }

    let reasons = reasons.into_iter().collect::<Vec<_>>();
    Ok(DeletionDecision {
        schema: "csdlc.deletion_eligibility.v1".into(),
        issue: request.issue,
        code_revision: git_text(repo, &["rev-parse", "HEAD"])?,
        evaluated_at: now
            .format(&time::format_description::well_known::Rfc3339)
            .map_err(time_error)?,
        phase_b_blake3,
        phase_c_blake3,
        selector_blake3,
        manifest_blake3,
        removed_lines,
        retained_lines,
        removal_basis_points: basis_points,
        target_met: basis_points >= request.manifest.target_percent.saturating_mul(100),
        eligible: reasons.is_empty(),
        reasons,
        deletion_executed: false,
    })
}

pub fn write_decision_atomic(path: &Path, decision: &DeletionDecision) -> Result<()> {
    let parent = path
        .parent()
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "decision output needs a parent"))?;
    fs::create_dir_all(parent)?;
    let temporary = path.with_extension("json.tmp");
    fs::write(&temporary, serde_json::to_vec_pretty(decision)?)?;
    fs::rename(temporary, path)?;
    Ok(())
}

fn validate_request(request: &DeletionEligibilityRequest) -> Result<()> {
    if request.schema != "csdlc.deletion_eligibility_request.v1" || request.issue != 5305 {
        return Err(invalid("request schema and issue must identify Gate 10D1"));
    }
    for path in [
        &request.phase_b_evidence,
        &request.phase_c_evidence,
        &request.selector,
    ] {
        validate_relative_path(path)?;
    }
    let manifest = &request.manifest;
    if manifest.schema != "csdlc.proposed_deletion_manifest.v1"
        || manifest.baseline_lines != 49_979
        || manifest.target_percent != 90
        || manifest.minimum_percent != 80
        || manifest.entries.is_empty()
    {
        return Err(invalid(
            "manifest must use the reviewed Gate 1 denominator and thresholds",
        ));
    }
    let mut paths = BTreeSet::new();
    let mut total = 0_u64;
    for entry in &manifest.entries {
        validate_relative_path(&entry.path)?;
        if !paths.insert(&entry.path) || entry.measured_lines == 0 {
            return Err(invalid(
                "manifest paths must be unique with nonzero measured lines",
            ));
        }
        total = total
            .checked_add(entry.measured_lines)
            .ok_or_else(|| invalid("manifest line total overflow"))?;
        if entry.disposition == EntryDisposition::Retain
            && (entry.owner.as_deref().is_none_or(str::is_empty)
                || entry.justification.as_deref().is_none_or(str::is_empty))
        {
            return Err(invalid(
                "every retained surface needs an owner and justification",
            ));
        }
        if let Some(value) = &entry.protected_until {
            parse_time(value)?;
        }
    }
    if total != manifest.baseline_lines {
        return Err(invalid(
            "manifest entries must exactly partition the baseline lines",
        ));
    }
    Ok(())
}

fn validate_approval(approval: &DeletionApproval) -> Result<()> {
    if approval.schema != "csdlc.deletion_approval.v1"
        || approval.approved_by.trim().is_empty()
        || approval.phase_c_blake3.len() != 64
        || approval.manifest_blake3.len() != 64
    {
        return Err(invalid("approval is malformed"));
    }
    parse_time(&approval.approved_at)?;
    Ok(())
}

fn validate_relative_path(path: &Path) -> Result<()> {
    if path.as_os_str().is_empty()
        || path.is_absolute()
        || path.components().any(|component| {
            matches!(
                component,
                Component::ParentDir | Component::RootDir | Component::Prefix(_)
            )
        })
    {
        return Err(invalid("paths must be safe repository-relative paths"));
    }
    Ok(())
}

fn read_regular_repo_file(repo: &Path, relative: &Path) -> Result<Vec<u8>> {
    validate_relative_path(relative)?;
    let path = repo.join(relative);
    let metadata = fs::symlink_metadata(&path)?;
    if !metadata.file_type().is_file() {
        return Err(invalid(format!(
            "input must be a regular file: {}",
            relative.display()
        )));
    }
    fs::read(path).map_err(Into::into)
}

fn parse_time(value: &str) -> Result<OffsetDateTime> {
    OffsetDateTime::parse(value, &time::format_description::well_known::Rfc3339)
        .map_err(|error| invalid(error.to_string()))
}

fn git_text(repo: &Path, args: &[&str]) -> Result<String> {
    let output = Command::new("git").args(args).current_dir(repo).output()?;
    if !output.status.success() {
        return Err(V2Error::new(ErrorCode::GitFailure, "git command failed"));
    }
    Ok(String::from_utf8_lossy(&output.stdout).trim().into())
}

fn digest(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

fn invalid(message: impl Into<String>) -> V2Error {
    V2Error::new(ErrorCode::InvalidManifest, message)
}

fn time_error(error: time::error::Format) -> V2Error {
    V2Error::new(ErrorCode::InvalidInput, error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fixture_repo() -> tempfile::TempDir {
        let repo = tempfile::tempdir().unwrap();
        for path in [
            "phase-b.json",
            "phase-c.json",
            "selector.json",
            "candidate-v1",
        ] {
            fs::write(repo.path().join(path), b"candidate").unwrap();
        }
        fs::write(
            repo.path().join("phase-b.json"),
            include_bytes!("../../docs/architecture/csdlc-v2/gate10b/PRE_SWITCH_EVIDENCE.json"),
        )
        .unwrap();
        fs::write(
            repo.path().join("phase-c.json"),
            include_bytes!("../../docs/architecture/csdlc-v2/gate10c/CUTOVER_EVIDENCE.json"),
        )
        .unwrap();
        fs::write(
            repo.path().join("selector.json"),
            br#"{"schema":"csdlc.generation_selector.v1","default_generation":"v2","opted_in_issues":[5293,5294]}"#,
        )
        .unwrap();
        Command::new("git")
            .args(["init", "-q"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.email", "eligibility@example.invalid"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["config", "user.name", "Eligibility"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["add", "."])
            .current_dir(repo.path())
            .status()
            .unwrap();
        Command::new("git")
            .args(["commit", "-qm", "fixture"])
            .current_dir(repo.path())
            .status()
            .unwrap();
        repo
    }

    fn request(
        repo: &Path,
        removed: u64,
        protected_until: Option<&str>,
    ) -> DeletionEligibilityRequest {
        let manifest = DeletionManifest {
            schema: "csdlc.proposed_deletion_manifest.v1".into(),
            baseline_lines: 49_979,
            target_percent: 90,
            minimum_percent: 80,
            entries: vec![
                DeletionEntry {
                    path: "candidate-v1".into(),
                    disposition: EntryDisposition::Remove,
                    measured_lines: removed,
                    owner: None,
                    justification: None,
                    protected_until: protected_until.map(str::to_owned),
                },
                DeletionEntry {
                    path: "retained-v1".into(),
                    disposition: EntryDisposition::Retain,
                    measured_lines: 49_979 - removed,
                    owner: Some("migration-owner".into()),
                    justification: Some("useful retained compatibility".into()),
                    protected_until: None,
                },
            ],
        };
        let phase_c = fs::read(repo.join("phase-c.json")).unwrap();
        let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
        DeletionEligibilityRequest {
            schema: "csdlc.deletion_eligibility_request.v1".into(),
            issue: 5305,
            phase_b_evidence: "phase-b.json".into(),
            phase_c_evidence: "phase-c.json".into(),
            selector: "selector.json".into(),
            approval: Some(DeletionApproval {
                schema: "csdlc.deletion_approval.v1".into(),
                approved_by: "operator".into(),
                approved_at: "2026-08-13T00:00:00Z".into(),
                phase_c_blake3: digest(&phase_c),
                manifest_blake3: digest(&manifest_bytes),
                allow_qualified_80_to_89: true,
            }),
            manifest,
        }
    }

    fn time(value: &str) -> OffsetDateTime {
        parse_time(value).unwrap()
    }

    #[test]
    fn missing_approval_is_ineligible_and_candidate_bytes_do_not_change() {
        let repo = fixture_repo();
        let before = fs::read(repo.path().join("candidate-v1")).unwrap();
        let mut input = request(repo.path(), 45_000, None);
        input.approval = None;
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).unwrap();
        assert!(!decision.eligible);
        assert_eq!(decision.reasons, vec![DeletionReason::MissingApproval]);
        assert!(!decision.deletion_executed);
        assert_eq!(fs::read(repo.path().join("candidate-v1")).unwrap(), before);
    }

    #[test]
    fn protected_window_fails_closed() {
        let repo = fixture_repo();
        let input = request(repo.path(), 45_000, Some("2026-08-12T02:03:02Z"));
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-07-20T00:00:00Z")).unwrap();
        assert!(!decision.eligible);
        assert!(decision
            .reasons
            .contains(&DeletionReason::ProtectedWindowActive));
    }

    #[test]
    fn below_eighty_percent_can_never_be_eligible() {
        let repo = fixture_repo();
        let input = request(repo.path(), 39_000, None);
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).unwrap();
        assert!(!decision.eligible);
        assert!(decision
            .reasons
            .contains(&DeletionReason::BelowMinimumRemoval));
    }

    #[test]
    fn qualified_eighty_to_eighty_nine_requires_approval_flag() {
        let repo = fixture_repo();
        let mut input = request(repo.path(), 42_000, None);
        input.approval.as_mut().unwrap().allow_qualified_80_to_89 = false;
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).unwrap();
        assert!(!decision.eligible);
        assert!(decision
            .reasons
            .contains(&DeletionReason::QualificationNotApproved));
    }

    #[test]
    fn current_approved_ninety_percent_manifest_is_eligible_but_non_mutating() {
        let repo = fixture_repo();
        let input = request(repo.path(), 45_000, None);
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).unwrap();
        assert!(decision.eligible, "{decision:#?}");
        assert!(decision.target_met);
        assert!(!decision.deletion_executed);
    }

    #[test]
    fn approval_is_bound_to_phase_c_and_manifest_digests() {
        let repo = fixture_repo();
        let mut input = request(repo.path(), 45_000, None);
        input.approval.as_mut().unwrap().manifest_blake3 = "0".repeat(64);
        let decision =
            evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).unwrap();
        assert!(!decision.eligible);
        assert!(decision
            .reasons
            .contains(&DeletionReason::ApprovalEvidenceMismatch));
    }

    #[test]
    fn unsafe_duplicate_and_unowned_retained_entries_are_invalid() {
        let repo = fixture_repo();
        let mut input = request(repo.path(), 45_000, None);
        input.manifest.entries[0].path = "../escape".into();
        assert!(evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).is_err());
        let mut input = request(repo.path(), 45_000, None);
        input.manifest.entries[1].owner = None;
        assert!(evaluate_with_time(repo.path(), &input, time("2026-08-13T00:00:00Z")).is_err());
    }
}
