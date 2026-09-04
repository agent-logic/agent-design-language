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
use serde_json::Value;

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
    pub schema: String,
    pub bytes: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<bool>,
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
    evidence_blockers.extend(required_evidence_blockers(route, &verified));
    if request.operator_cutover_approved {
        evidence_blockers.extend(operator_approval_blockers(&verified));
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
    let parsed: Value = serde_json::from_slice(&bytes).map_err(|err| {
        error(
            "evidence_schema_invalid",
            format!("evidence ref {} is not JSON: {err}", evidence.path),
        )
    })?;
    let schema = parsed
        .get("schema")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            error(
                "evidence_schema_missing",
                format!("evidence ref {} has no schema", evidence.path),
            )
        })?
        .to_owned();
    let approved = parsed.get("approved").and_then(Value::as_bool);
    Ok(VerifiedReplacementEvidence {
        path: evidence.path.clone(),
        digest,
        schema,
        bytes: bytes.len(),
        approved,
    })
}

fn required_evidence_blockers(
    command: ReplacementCommand,
    verified: &[VerifiedReplacementEvidence],
) -> Vec<String> {
    let required_path = command_proof_path(command);
    let required_schema = command_proof_schema(command);
    if verified
        .iter()
        .any(|item| item.path == required_path && item.schema == required_schema)
    {
        Vec::new()
    } else {
        vec![format!(
            "required_command_evidence_missing: {} requires {} with schema {}",
            command.as_str(),
            required_path,
            required_schema
        )]
    }
}

fn operator_approval_blockers(verified: &[VerifiedReplacementEvidence]) -> Vec<String> {
    let mut blockers = Vec::new();
    let approval = verified
        .iter()
        .find(|item| item.path == OPERATOR_APPROVAL_PATH);
    match approval {
        Some(item) if item.schema == OPERATOR_APPROVAL_SCHEMA && item.approved == Some(true) => {}
        Some(item) if item.schema != OPERATOR_APPROVAL_SCHEMA => blockers.push(format!(
            "operator_cutover_approval_schema_mismatch: {} must use schema {}",
            OPERATOR_APPROVAL_PATH, OPERATOR_APPROVAL_SCHEMA
        )),
        Some(_) => blockers.push(format!(
            "operator_cutover_approval_not_granted: {} must set approved=true",
            OPERATOR_APPROVAL_PATH
        )),
        None => blockers.push(format!(
            "operator_cutover_approval_evidence_missing: {} is required before readiness can be derived",
            OPERATOR_APPROVAL_PATH
        )),
    }

    let approval_count = verified
        .iter()
        .filter(|item| item.path == OPERATOR_APPROVAL_PATH)
        .count();
    if approval_count > 1 {
        blockers.push(format!(
            "operator_cutover_approval_ambiguous: {} must be unique",
            OPERATOR_APPROVAL_PATH
        ));
    }
    blockers
}

fn command_proof_path(command: ReplacementCommand) -> String {
    format!(
        ".csdlc/evidence/csdlc-v3/{}-replacement-proof.json",
        command.as_str()
    )
}

fn command_proof_schema(command: ReplacementCommand) -> String {
    format!("csdlc.v3.{}_replacement_proof.v1", command.as_str())
}

const OPERATOR_APPROVAL_PATH: &str = ".csdlc/evidence/csdlc-v3/operator-cutover-approval.json";
const OPERATOR_APPROVAL_SCHEMA: &str = "csdlc.v3.operator_cutover_approval.v1";

fn error(code: &'static str, message: impl Into<String>) -> ReplacementVerifierError {
    ReplacementVerifierError {
        code,
        message: message.into(),
    }
}
