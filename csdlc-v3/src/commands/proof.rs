//! Non-authoritative proof, shadow, soak, and install route models.
//!
//! These routes are construction evidence for the one-binary v3 command
//! surface. They classify typed request packets and intentionally do not
//! execute lifecycle authority, provider calls, selector mutation, binary
//! installation, GitHub mutation, finish, cleanup, or #505 cutover.

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

pub const PROOF_ROUTE_NAMES: [&str; 4] = ["proof", "shadow", "soak", "install"];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProofRouteRequest {
    pub issue: u64,
    pub repository: String,
    pub cutover_issue: Option<u64>,
    pub evidence_root: Option<String>,
    pub proof: Option<ProofManifest>,
    pub shadow: Option<ShadowComparison>,
    pub soak: Option<SoakEvidence>,
    pub install: Option<InstallPlanInput>,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ProofManifest {
    pub manifest_id: String,
    pub lane: String,
    pub deterministic: bool,
    pub evidence_ref: String,
    pub evidence_digest: String,
    pub observed_digest: String,
    pub stale: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShadowComparison {
    pub v2_observation_ref: String,
    pub v2_digest: String,
    pub v3_observation_ref: String,
    pub v3_digest: String,
    pub bounded_v2: bool,
    pub bounded_v3: bool,
    pub broad_equivalence_claim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SoakEvidence {
    pub evidence_ref: String,
    pub duration_minutes: u64,
    pub sample_count: u64,
    pub hidden_state: bool,
    pub provider_side_effects: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct InstallPlanInput {
    pub artifact_name: String,
    pub artifact_ref: String,
    pub source_provenance_ref: String,
    pub selector_metadata_ref: String,
    pub source_provenance: String,
    pub selected_binary_digest: String,
    pub observed_binary_digest: String,
    pub selector_metadata_digest: String,
    pub destination: String,
    pub stable_destination: bool,
    pub executes_install: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProofRouteReport {
    pub schema: &'static str,
    pub route: String,
    pub issue: u64,
    pub repository: String,
    pub read_only: bool,
    pub operational_authority: bool,
    pub status: ProofRouteStatus,
    pub findings: Vec<ProofRouteFinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProofRouteStatus {
    Ready,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ProofRouteFinding {
    pub code: &'static str,
    pub message: &'static str,
}

pub fn classify_route(
    route: &str,
    request: ProofRouteRequest,
    repository_root: Option<&Path>,
) -> ProofRouteReport {
    let mut findings = common_findings(&request, repository_root);
    match route {
        "proof" => match request.proof.as_ref() {
            Some(manifest) => {
                validate_proof_manifest(request.evidence_root.as_deref(), manifest, &mut findings)
            }
            None => findings.push(finding(
                "proof_manifest_missing",
                "proof route requires proof manifest evidence",
            )),
        },
        "shadow" => match request.shadow.as_ref() {
            Some(shadow) => {
                validate_shadow(request.evidence_root.as_deref(), shadow, &mut findings)
            }
            None => findings.push(finding(
                "shadow_comparison_missing",
                "shadow route requires paired v2/v3 observations",
            )),
        },
        "soak" => match request.soak.as_ref() {
            Some(soak) => validate_soak(request.evidence_root.as_deref(), soak, &mut findings),
            None => findings.push(finding(
                "soak_evidence_missing",
                "soak route requires bounded soak evidence",
            )),
        },
        "install" => match request.install.as_ref() {
            Some(install) => validate_install(
                request.evidence_root.as_deref(),
                request.cutover_issue,
                install,
                &mut findings,
            ),
            None => findings.push(finding(
                "install_plan_missing",
                "install route requires a typed install plan input",
            )),
        },
        _ => findings.push(finding("route_unknown", "unsupported proof route")),
    }
    ProofRouteReport {
        schema: "csdlc.v3.proof_route.v1",
        route: route.to_owned(),
        issue: request.issue,
        repository: request.repository,
        read_only: true,
        operational_authority: false,
        status: if findings.is_empty() {
            ProofRouteStatus::Ready
        } else {
            ProofRouteStatus::Blocked
        },
        findings,
    }
}

fn common_findings(
    request: &ProofRouteRequest,
    repository_root: Option<&Path>,
) -> Vec<ProofRouteFinding> {
    let mut findings = Vec::new();
    if request.issue == 0 {
        findings.push(finding("issue_missing", "issue identity must be non-zero"));
    }
    if request.repository.trim().is_empty() {
        findings.push(finding(
            "repository_missing",
            "repository identity is required",
        ));
    }
    validate_evidence_root_binding(
        request.evidence_root.as_deref(),
        repository_root,
        &mut findings,
    );
    findings
}

fn validate_evidence_root_binding(
    evidence_root: Option<&str>,
    repository_root: Option<&Path>,
    findings: &mut Vec<ProofRouteFinding>,
) {
    let Some(evidence_root) = evidence_root else {
        return;
    };
    if evidence_root.trim().is_empty() {
        return;
    }
    let Some(repository_root) = repository_root else {
        findings.push(finding(
            "repository_root_unavailable",
            "proof evidence must be classified from a discovered repository checkout",
        ));
        return;
    };
    let Ok(evidence_root) = PathBuf::from(evidence_root).canonicalize() else {
        return;
    };
    let Ok(repository_root) = repository_root.canonicalize() else {
        findings.push(finding(
            "repository_root_unavailable",
            "proof evidence must be classified from a discovered repository checkout",
        ));
        return;
    };
    if evidence_root != repository_root {
        findings.push(finding(
            "evidence_root_not_repository_root",
            "proof evidence root must be the binary checkout repository root, not a request-controlled scratch tree",
        ));
    }
}

fn validate_proof_manifest(
    evidence_root: Option<&str>,
    manifest: &ProofManifest,
    findings: &mut Vec<ProofRouteFinding>,
) {
    require_nonempty(
        &manifest.manifest_id,
        "proof_manifest_id_missing",
        "proof manifest id is required",
        findings,
    );
    require_nonempty(
        &manifest.lane,
        "proof_lane_missing",
        "proof lane is required",
        findings,
    );
    require_nonempty(
        &manifest.evidence_ref,
        "proof_evidence_ref_missing",
        "proof evidence ref is required",
        findings,
    );
    require_nonempty(
        &manifest.evidence_digest,
        "proof_evidence_digest_missing",
        "proof evidence digest is required",
        findings,
    );
    if !manifest.deterministic {
        findings.push(finding(
            "proof_lane_not_deterministic",
            "proof lane must declare deterministic behavior",
        ));
    }
    if manifest.stale {
        findings.push(finding(
            "proof_evidence_stale",
            "stale proof evidence cannot authorize readiness",
        ));
    }
    if let Some(observed) = observed_ref_digest(evidence_root, &manifest.evidence_ref, findings) {
        if manifest.observed_digest != observed {
            findings.push(finding(
                "proof_observed_digest_mismatch",
                "proof observed digest must match the referenced evidence file",
            ));
        }
    }
    if manifest.evidence_digest != manifest.observed_digest {
        findings.push(finding(
            "proof_digest_mismatch",
            "proof evidence digest must match the observed digest",
        ));
    }
}

fn validate_shadow(
    evidence_root: Option<&str>,
    shadow: &ShadowComparison,
    findings: &mut Vec<ProofRouteFinding>,
) {
    require_nonempty(
        &shadow.v2_observation_ref,
        "shadow_v2_ref_missing",
        "v2 shadow observation ref is required",
        findings,
    );
    require_nonempty(
        &shadow.v3_observation_ref,
        "shadow_v3_ref_missing",
        "v3 shadow observation ref is required",
        findings,
    );
    require_nonempty(
        &shadow.v2_digest,
        "shadow_v2_digest_missing",
        "v2 shadow digest is required",
        findings,
    );
    require_nonempty(
        &shadow.v3_digest,
        "shadow_v3_digest_missing",
        "v3 shadow digest is required",
        findings,
    );
    if !shadow.bounded_v2 || !shadow.bounded_v3 {
        findings.push(finding(
            "shadow_observation_unbounded",
            "shadow comparison requires bounded v2 and v3 observations",
        ));
    }
    if shadow.broad_equivalence_claim {
        findings.push(finding(
            "shadow_broad_equivalence_claim",
            "shadow route refuses broad equivalence claims",
        ));
    }
    let observed_v2 = observed_ref_digest(evidence_root, &shadow.v2_observation_ref, findings);
    let observed_v3 = observed_ref_digest(evidence_root, &shadow.v3_observation_ref, findings);
    if observed_v2
        .as_ref()
        .is_some_and(|observed| observed != &shadow.v2_digest)
    {
        findings.push(finding(
            "shadow_v2_digest_mismatch",
            "v2 shadow digest must match the referenced observation file",
        ));
    }
    if observed_v3
        .as_ref()
        .is_some_and(|observed| observed != &shadow.v3_digest)
    {
        findings.push(finding(
            "shadow_v3_digest_mismatch",
            "v3 shadow digest must match the referenced observation file",
        ));
    }
    if shadow.v2_digest != shadow.v3_digest {
        findings.push(finding(
            "shadow_digest_mismatch",
            "v2 and v3 shadow digests must match",
        ));
    }
}

fn validate_soak(
    evidence_root: Option<&str>,
    soak: &SoakEvidence,
    findings: &mut Vec<ProofRouteFinding>,
) {
    require_nonempty(
        &soak.evidence_ref,
        "soak_evidence_ref_missing",
        "soak evidence ref is required",
        findings,
    );
    if soak.duration_minutes == 0 || soak.sample_count == 0 {
        findings.push(finding(
            "soak_sample_missing",
            "soak evidence requires non-zero duration and sample count",
        ));
    }
    if soak.hidden_state {
        findings.push(finding(
            "soak_hidden_state",
            "soak route refuses hidden state",
        ));
    }
    if soak.provider_side_effects {
        findings.push(finding(
            "soak_provider_side_effects",
            "soak route cannot perform provider side effects before cutover",
        ));
    }
    let _ = observed_ref_digest(evidence_root, &soak.evidence_ref, findings);
}

fn validate_install(
    evidence_root: Option<&str>,
    cutover_issue: Option<u64>,
    install: &InstallPlanInput,
    findings: &mut Vec<ProofRouteFinding>,
) {
    if cutover_issue != Some(505) {
        findings.push(finding(
            "install_cutover_issue_missing",
            "install route must be gated by #505 cutover",
        ));
    }
    if install.artifact_name != "csdlc" {
        findings.push(finding(
            "install_artifact_not_one_binary",
            "install route only plans the single csdlc binary",
        ));
    }
    require_nonempty(
        &install.source_provenance,
        "install_source_provenance_missing",
        "install source provenance is required",
        findings,
    );
    require_nonempty(
        &install.artifact_ref,
        "install_artifact_ref_missing",
        "install artifact ref is required",
        findings,
    );
    require_nonempty(
        &install.source_provenance_ref,
        "install_source_provenance_ref_missing",
        "install source provenance ref is required",
        findings,
    );
    require_nonempty(
        &install.selector_metadata_ref,
        "install_selector_metadata_ref_missing",
        "install selector metadata ref is required",
        findings,
    );
    require_nonempty(
        &install.selector_metadata_digest,
        "install_selector_metadata_missing",
        "selector metadata digest is required",
        findings,
    );
    require_nonempty(
        &install.selected_binary_digest,
        "install_selected_digest_missing",
        "selected binary digest is required",
        findings,
    );
    if install.selected_binary_digest != install.observed_binary_digest {
        findings.push(finding(
            "install_digest_mismatch",
            "selected binary digest must match observed binary digest",
        ));
    }
    if let Some(observed) = observed_ref_digest(evidence_root, &install.artifact_ref, findings) {
        if install.observed_binary_digest != observed {
            findings.push(finding(
                "install_observed_binary_digest_mismatch",
                "install observed binary digest must match the referenced artifact",
            ));
        }
    }
    if let Some(observed) =
        observed_ref_digest(evidence_root, &install.selector_metadata_ref, findings)
    {
        if install.selector_metadata_digest != observed {
            findings.push(finding(
                "install_selector_metadata_digest_mismatch",
                "install selector metadata digest must match the referenced selector metadata",
            ));
        }
    }
    if let Some(bytes) = observed_ref_bytes(evidence_root, &install.source_provenance_ref, findings)
    {
        let observed_source = serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .and_then(|value| {
                value
                    .get("source")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned)
            });
        match observed_source {
            Some(source) if source == install.source_provenance => {}
            Some(_) => findings.push(finding(
                "install_source_provenance_mismatch",
                "install source provenance must match the referenced provenance evidence",
            )),
            None => findings.push(finding(
                "install_source_provenance_invalid",
                "install source provenance evidence must be typed JSON with a source string",
            )),
        }
    }
    if !install.stable_destination || install.destination.contains("/target/") {
        findings.push(finding(
            "install_destination_not_stable",
            "install destination must be stable and outside Cargo target output",
        ));
    }
    if install.executes_install {
        findings.push(finding(
            "install_attempts_mutation",
            "install route is plan-only before #505 cutover",
        ));
    }
}

fn require_nonempty(
    value: &str,
    code: &'static str,
    message: &'static str,
    findings: &mut Vec<ProofRouteFinding>,
) {
    if value.trim().is_empty() {
        findings.push(finding(code, message));
    }
}

fn observed_ref_digest(
    evidence_root: Option<&str>,
    evidence_ref: &str,
    findings: &mut Vec<ProofRouteFinding>,
) -> Option<String> {
    observed_ref_bytes(evidence_root, evidence_ref, findings)
        .map(|bytes| blake3::hash(&bytes).to_hex().to_string())
}

fn observed_ref_bytes(
    evidence_root: Option<&str>,
    evidence_ref: &str,
    findings: &mut Vec<ProofRouteFinding>,
) -> Option<Vec<u8>> {
    let Some(root) = evidence_root else {
        findings.push(finding(
            "evidence_root_missing",
            "evidence-backed routes require an evidence root",
        ));
        return None;
    };
    if root.trim().is_empty() {
        findings.push(finding(
            "evidence_root_missing",
            "evidence-backed routes require an evidence root",
        ));
        return None;
    }
    if evidence_ref.trim().is_empty() {
        return None;
    }
    let ref_path = Path::new(evidence_ref);
    if ref_path.is_absolute()
        || evidence_ref
            .split('/')
            .any(|component| component == ".." || component.is_empty())
        || !evidence_ref.starts_with(".csdlc/evidence/")
    {
        findings.push(finding(
            "evidence_ref_not_repo_contained",
            "evidence refs must be repo-relative paths under .csdlc/evidence",
        ));
        return None;
    }
    let root_path = Path::new(root);
    let evidence_path = root_path.join(ref_path);
    let Ok(root_canonical) = root_path.canonicalize() else {
        findings.push(finding(
            "evidence_root_missing",
            "evidence root must exist before proof classification",
        ));
        return None;
    };
    let Ok(evidence_canonical) = evidence_path.canonicalize() else {
        findings.push(finding(
            "evidence_ref_missing",
            "referenced evidence file must exist before proof classification",
        ));
        return None;
    };
    if !evidence_canonical.starts_with(&root_canonical) {
        findings.push(finding(
            "evidence_ref_not_repo_contained",
            "evidence refs must remain under the evidence root after canonicalization",
        ));
        return None;
    }
    match fs::read(&evidence_canonical) {
        Ok(bytes) => Some(bytes),
        Err(_) => {
            findings.push(finding(
                "evidence_ref_unreadable",
                "referenced evidence file must be readable before proof classification",
            ));
            None
        }
    }
}

fn finding(code: &'static str, message: &'static str) -> ProofRouteFinding {
    ProofRouteFinding { code, message }
}
