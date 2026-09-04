//! Pre-cutover replacement verifiers for remaining C-SDLC v3 command gaps.
//!
//! These commands make the single `csdlc` command surface executable before
//! #505 cutover without granting live authority. They validate typed,
//! repo-local evidence envelopes and emit machine-readable readiness/blocker
//! reports. They do not install binaries, generate proof, run soak jobs,
//! switch authority, mutate GitHub, or remove files.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementCommand {
    Cutover,
    Install,
    Proof,
    Shadow,
    Soak,
}

impl ReplacementCommand {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cutover => "cutover",
            Self::Install => "install",
            Self::Proof => "proof",
            Self::Shadow => "shadow",
            Self::Soak => "soak",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementEvidenceRef {
    pub path: String,
    pub digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementVerifierRequest {
    pub schema: String,
    pub command: ReplacementCommand,
    pub issue: u64,
    pub authority_issue: u64,
    pub repository: String,
    #[serde(default)]
    pub operator_cutover_approved: bool,
    #[serde(default)]
    pub evidence: Vec<ReplacementEvidenceRef>,
    #[serde(default)]
    pub blockers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplacementVerifierReport {
    pub schema: &'static str,
    pub command: ReplacementCommand,
    pub issue: u64,
    pub authority_issue: u64,
    pub repository: String,
    pub implemented: bool,
    pub mutation_allowed: bool,
    pub operational_authority: bool,
    pub status: ReplacementVerifierStatus,
    pub blockers: Vec<String>,
    pub evidence: Vec<VerifiedReplacementEvidence>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReplacementVerifierStatus {
    ReadyForCutoverDecision,
    BlockedBeforeOperatorApproval,
    BlockedByEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct VerifiedReplacementEvidence {
    pub path: String,
    pub digest: String,
    pub bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplacementVerifierError {
    pub code: &'static str,
    pub message: String,
}

pub fn command_from_route(route: &str) -> Option<ReplacementCommand> {
    match route {
        "cutover" => Some(ReplacementCommand::Cutover),
        "install" => Some(ReplacementCommand::Install),
        "proof" => Some(ReplacementCommand::Proof),
        "shadow" => Some(ReplacementCommand::Shadow),
        "soak" => Some(ReplacementCommand::Soak),
        _ => None,
    }
}

pub fn verify_replacement_request(
    repo_root: &Path,
    route: ReplacementCommand,
    request: ReplacementVerifierRequest,
) -> Result<ReplacementVerifierReport, ReplacementVerifierError> {
    if request.schema != "csdlc.v3.replacement_verifier_request.v1" {
        return Err(error(
            "invalid_schema",
            "replacement verifier request schema is invalid",
        ));
    }
    if request.command != route {
        return Err(error(
            "command_mismatch",
            format!(
                "request command {} does not match route {}",
                request.command.as_str(),
                route.as_str()
            ),
        ));
    }
    if request.issue == 0 || request.authority_issue != 505 {
        return Err(error(
            "invalid_authority_issue",
            "replacement verifier requires issue identity and authority_issue 505",
        ));
    }
    if request.repository != "agent-logic/agent-design-language" {
        return Err(error(
            "repository_mismatch",
            "replacement verifier is bound to agent-logic/agent-design-language",
        ));
    }
    let root = canonical_root(repo_root)?;
    let mut verified = Vec::new();
    let mut evidence_blockers = Vec::new();
    for evidence in &request.evidence {
        match verify_evidence_ref(&root, evidence) {
            Ok(item) => verified.push(item),
            Err(err) => evidence_blockers.push(format!("{}: {}", err.code, err.message)),
        }
    }
    let status = if !evidence_blockers.is_empty() {
        ReplacementVerifierStatus::BlockedByEvidence
    } else if request.operator_cutover_approved && request.blockers.is_empty() {
        ReplacementVerifierStatus::ReadyForCutoverDecision
    } else {
        ReplacementVerifierStatus::BlockedBeforeOperatorApproval
    };
    let mut blockers = request.blockers;
    if !request.operator_cutover_approved {
        blockers.push(
            "operator_cutover_approval_missing: v3 remains non-authoritative before #505 approval"
                .to_owned(),
        );
    }
    blockers.extend(evidence_blockers);
    Ok(ReplacementVerifierReport {
        schema: "csdlc.v3.replacement_verifier.v1",
        command: route,
        issue: request.issue,
        authority_issue: request.authority_issue,
        repository: request.repository,
        implemented: true,
        mutation_allowed: false,
        operational_authority: false,
        status,
        blockers,
        evidence: verified,
    })
}

fn canonical_root(root: &Path) -> Result<PathBuf, ReplacementVerifierError> {
    fs::canonicalize(root).map_err(|err| {
        error(
            "repo_root_unreadable",
            format!("repo root is unreadable: {err}"),
        )
    })
}

fn verify_evidence_ref(
    root: &Path,
    evidence: &ReplacementEvidenceRef,
) -> Result<VerifiedReplacementEvidence, ReplacementVerifierError> {
    if evidence.path.trim().is_empty()
        || evidence.path.starts_with('/')
        || evidence.path.split('/').any(|part| part == "..")
    {
        return Err(error(
            "evidence_ref_not_repo_relative",
            "evidence paths must be nonempty repo-relative paths",
        ));
    }
    if evidence.digest.trim().is_empty() {
        return Err(error(
            "evidence_digest_missing",
            "evidence digest must be nonempty",
        ));
    }
    let joined = root.join(&evidence.path);
    let canonical = fs::canonicalize(&joined).map_err(|err| {
        error(
            "evidence_ref_unreadable",
            format!("evidence ref {} is unreadable: {err}", evidence.path),
        )
    })?;
    if !canonical.starts_with(root) || !canonical.is_file() {
        return Err(error(
            "evidence_ref_escapes_repo",
            "evidence ref must resolve to a regular file beneath the repository",
        ));
    }
    let bytes = fs::read(&canonical).map_err(|err| {
        error(
            "evidence_ref_unreadable",
            format!("evidence ref {} is unreadable: {err}", evidence.path),
        )
    })?;
    let digest = blake3::hash(&bytes).to_hex().to_string();
    if digest != evidence.digest {
        return Err(error(
            "evidence_digest_mismatch",
            format!("evidence ref {} digest mismatch", evidence.path),
        ));
    }
    Ok(VerifiedReplacementEvidence {
        path: evidence.path.clone(),
        digest,
        bytes: bytes.len(),
    })
}

fn error(code: &'static str, message: impl Into<String>) -> ReplacementVerifierError {
    ReplacementVerifierError {
        code,
        message: message.into(),
    }
}
