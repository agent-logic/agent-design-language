//! Native proof, shadow, soak, and stable-install readiness operations.
//!
//! These routes may retain bounded evidence, but they do not grant live
//! lifecycle authority before explicit #505 cutover approval.

use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use serde::{Deserialize, Serialize};

pub const PROOF_ROUTE_NAMES: [&str; 4] = ["proof", "shadow", "soak", "install"];

#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct ProofRouteRequest {
    pub issue: u64,
    pub repository: String,
    pub cutover_issue: Option<u64>,
    #[serde(default)]
    pub operator_approval: Option<String>,
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
    pub performed_mutation: bool,
    pub evidence_refs: Vec<String>,
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
    let mut evidence_refs = Vec::new();
    let mut performed_mutation = false;
    match route {
        "proof" => match request.proof.as_ref() {
            Some(manifest) => {
                validate_proof_manifest(request.evidence_root.as_deref(), manifest, &mut findings);
                if findings.is_empty() {
                    match retain_proof_receipt(&request, manifest) {
                        Ok(reference) => {
                            performed_mutation = true;
                            evidence_refs.push(reference);
                        }
                        Err(code) => findings.push(code),
                    }
                }
            }
            None => findings.push(finding(
                "proof_manifest_missing",
                "proof route requires proof manifest evidence",
            )),
        },
        "shadow" => match request.shadow.as_ref() {
            Some(shadow) => {
                validate_shadow(request.evidence_root.as_deref(), shadow, &mut findings);
                if findings.is_empty() {
                    match execute_shadow(&request, shadow) {
                        Ok(references) => {
                            performed_mutation = true;
                            evidence_refs.extend(references);
                        }
                        Err(code) => findings.push(code),
                    }
                }
            }
            None => findings.push(finding(
                "shadow_comparison_missing",
                "shadow route requires paired v2/v3 observations",
            )),
        },
        "soak" => match request.soak.as_ref() {
            Some(soak) => {
                validate_soak(request.evidence_root.as_deref(), soak, &mut findings);
                if findings.is_empty() {
                    match execute_soak(&request, soak) {
                        Ok(reference) => {
                            performed_mutation = true;
                            evidence_refs.push(reference);
                        }
                        Err(code) => findings.push(code),
                    }
                }
            }
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
    if route == "install" && findings.is_empty() {
        if let Some(install) = request
            .install
            .as_ref()
            .filter(|value| value.executes_install)
        {
            if !has_explicit_505_operator_approval(request.operator_approval.as_deref()) {
                findings.push(finding(
                    "install_operator_approval_missing",
                    "executing install requires explicit #505 operator approval",
                ));
            } else {
                match execute_install(&request, install) {
                    Ok(reference) => {
                        performed_mutation = true;
                        evidence_refs.push(reference);
                    }
                    Err(code) => findings.push(code),
                }
            }
        }
    }
    ProofRouteReport {
        schema: "csdlc.v3.proof_route.v1",
        route: route.to_owned(),
        issue: request.issue,
        repository: request.repository,
        read_only: !performed_mutation,
        operational_authority: false,
        performed_mutation,
        evidence_refs,
        status: if findings.is_empty() {
            ProofRouteStatus::Ready
        } else {
            ProofRouteStatus::Blocked
        },
        findings,
    }
}

fn has_explicit_505_operator_approval(approval: Option<&str>) -> bool {
    approval.is_some_and(|value| {
        let normalized = value.to_ascii_lowercase();
        normalized.contains("operator")
            && normalized.contains("#505")
            && normalized.contains("approval")
    })
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
        let observed_provenance = serde_json::from_slice::<serde_json::Value>(&bytes)
            .ok()
            .filter(|value| {
                value.get("schema").and_then(serde_json::Value::as_str)
                    == Some("csdlc.v3.install_provenance.v1")
            });
        let observed_source = observed_provenance.as_ref().and_then(|value| {
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
    if install.executes_install && install.destination != ".adl/bin/csdlc" {
        findings.push(finding(
            "install_destination_not_canonical",
            "executing install requires the canonical .adl/bin/csdlc destination",
        ));
    }
}

fn retain_proof_receipt(
    request: &ProofRouteRequest,
    manifest: &ProofManifest,
) -> Result<String, ProofRouteFinding> {
    let reference = format!(
        ".csdlc/evidence/{}/v3-proof/{}.json",
        request.issue,
        safe_component(&manifest.manifest_id)?
    );
    let receipt = serde_json::json!({
        "schema": "csdlc.v3.proof_receipt.v1",
        "issue": request.issue,
        "repository": request.repository,
        "manifest_id": manifest.manifest_id,
        "lane": manifest.lane,
        "deterministic": true,
        "source_evidence_ref": manifest.evidence_ref,
        "source_evidence_digest": manifest.observed_digest,
    });
    write_canonical_evidence(request, &reference, &receipt)?;
    Ok(reference)
}

fn execute_shadow(
    request: &ProofRouteRequest,
    shadow: &ShadowComparison,
) -> Result<Vec<String>, ProofRouteFinding> {
    let root = request_root(request)?;
    let v2 = read_json(&root.join(&shadow.v2_observation_ref))?;
    let v3 = read_json(&root.join(&shadow.v3_observation_ref))?;
    let v2 = canonical_json(&v2);
    let v3 = canonical_json(&v3);
    if v2 != v3 {
        return Err(finding(
            "shadow_normalized_mismatch",
            "normalized v2 and v3 observations must match",
        ));
    }
    let base = format!(".csdlc/evidence/{}/v3-shadow", request.issue);
    let v2_ref = format!("{base}/v2.normalized.json");
    let v3_ref = format!("{base}/v3.normalized.json");
    let receipt_ref = format!("{base}/comparison.json");
    write_bytes_atomic(&root.join(&v2_ref), &v2)?;
    write_bytes_atomic(&root.join(&v3_ref), &v3)?;
    let digest = blake3::hash(&v2).to_hex().to_string();
    write_canonical_evidence(
        request,
        &receipt_ref,
        &serde_json::json!({
            "schema": "csdlc.v3.shadow_receipt.v1",
            "issue": request.issue,
            "bounded": true,
            "normalized_digest": digest,
            "v2_observation_ref": shadow.v2_observation_ref,
            "v3_observation_ref": shadow.v3_observation_ref,
            "v2_normalized_ref": v2_ref,
            "v3_normalized_ref": v3_ref,
        }),
    )?;
    Ok(vec![v2_ref, v3_ref, receipt_ref])
}

fn execute_soak(
    request: &ProofRouteRequest,
    soak: &SoakEvidence,
) -> Result<String, ProofRouteFinding> {
    const MAX_SAMPLES: u64 = 10_000;
    if soak.sample_count > MAX_SAMPLES {
        return Err(finding(
            "soak_sample_limit_exceeded",
            "bounded soak sample count cannot exceed 10000",
        ));
    }
    let root = request_root(request)?;
    let mut samples = Vec::with_capacity(soak.sample_count as usize);
    for sequence in 0..soak.sample_count {
        let sample = fs::read(root.join(&soak.evidence_ref)).map_err(|_| {
            finding(
                "soak_evidence_unreadable",
                "soak source evidence must remain readable during execution",
            )
        })?;
        samples.push(serde_json::json!({
            "sequence": sequence + 1,
            "digest": blake3::hash(&sample).to_hex().to_string(),
        }));
    }
    let reference = format!(".csdlc/evidence/{}/v3-soak/receipt.json", request.issue);
    write_canonical_evidence(
        request,
        &reference,
        &serde_json::json!({
            "schema": "csdlc.v3.soak_receipt.v1",
            "issue": request.issue,
            "duration_minutes": soak.duration_minutes,
            "sample_count": soak.sample_count,
            "source_evidence_ref": soak.evidence_ref,
            "samples": samples,
        }),
    )?;
    Ok(reference)
}

fn execute_install(
    request: &ProofRouteRequest,
    install: &InstallPlanInput,
) -> Result<String, ProofRouteFinding> {
    let root = request_root(request)?;
    let source = root.join(&install.artifact_ref);
    let destination = root.join(&install.destination);
    if destination
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err(finding(
            "install_destination_symlink",
            "stable install destination must not be a symlink",
        ));
    }
    let bytes = fs::read(&source).map_err(|_| {
        finding(
            "install_artifact_unreadable",
            "selected install artifact must remain readable during installation",
        )
    })?;
    let digest = blake3::hash(&bytes).to_hex().to_string();
    if digest != install.selected_binary_digest {
        return Err(finding(
            "install_artifact_changed",
            "selected install artifact changed after validation",
        ));
    }
    write_bytes_atomic(&destination, &bytes)?;
    fs::set_permissions(&destination, fs::Permissions::from_mode(0o755)).map_err(|_| {
        finding(
            "install_permission_failed",
            "stable installed binary must be executable",
        )
    })?;
    let installed = fs::read(&destination).map_err(|_| {
        finding(
            "install_verification_failed",
            "stable installed binary must be readable for digest verification",
        )
    })?;
    if blake3::hash(&installed).to_hex().to_string() != digest {
        return Err(finding(
            "install_verification_failed",
            "stable installed binary digest must match selected artifact",
        ));
    }
    let reference = format!(".csdlc/evidence/{}/v3-install/receipt.json", request.issue);
    write_canonical_evidence(
        request,
        &reference,
        &serde_json::json!({
            "schema": "csdlc.v3.install_receipt.v1",
            "issue": request.issue,
            "artifact_name": install.artifact_name,
            "artifact_ref": install.artifact_ref,
            "destination": install.destination,
            "installed_digest": digest,
            "source_provenance": install.source_provenance,
            "source_provenance_ref": install.source_provenance_ref,
            "selector_metadata_ref": install.selector_metadata_ref,
            "selector_metadata_digest": install.selector_metadata_digest,
            "verified": true,
        }),
    )?;
    Ok(reference)
}

fn request_root(request: &ProofRouteRequest) -> Result<PathBuf, ProofRouteFinding> {
    request
        .evidence_root
        .as_deref()
        .map(PathBuf::from)
        .ok_or_else(|| {
            finding(
                "evidence_root_missing",
                "evidence-backed routes require an evidence root",
            )
        })
}

fn safe_component(value: &str) -> Result<&str, ProofRouteFinding> {
    if value.is_empty()
        || value == "."
        || value == ".."
        || value.contains('/')
        || value.contains('\\')
    {
        Err(finding(
            "evidence_identity_unsafe",
            "evidence identity must be one safe path component",
        ))
    } else {
        Ok(value)
    }
}

fn read_json(path: &Path) -> Result<serde_json::Value, ProofRouteFinding> {
    let bytes = fs::read(path).map_err(|_| {
        finding(
            "shadow_observation_unreadable",
            "shadow observations must be readable JSON",
        )
    })?;
    serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "shadow_observation_invalid_json",
            "shadow observations must be valid JSON",
        )
    })
}

fn canonical_json(value: &serde_json::Value) -> Vec<u8> {
    fn normalize(value: &serde_json::Value) -> serde_json::Value {
        match value {
            serde_json::Value::Object(map) => {
                let sorted: BTreeMap<_, _> = map
                    .iter()
                    .map(|(key, value)| (key.clone(), normalize(value)))
                    .collect();
                serde_json::Value::Object(sorted.into_iter().collect())
            }
            serde_json::Value::Array(values) => {
                serde_json::Value::Array(values.iter().map(normalize).collect())
            }
            other => other.clone(),
        }
    }
    serde_json::to_vec(&normalize(value)).expect("JSON value serialization cannot fail")
}

fn write_canonical_evidence(
    request: &ProofRouteRequest,
    reference: &str,
    value: &serde_json::Value,
) -> Result<(), ProofRouteFinding> {
    let root = request_root(request)?;
    let mut bytes = canonical_json(value);
    bytes.push(b'\n');
    write_bytes_atomic(&root.join(reference), &bytes)
}

fn write_bytes_atomic(path: &Path, bytes: &[u8]) -> Result<(), ProofRouteFinding> {
    static TEMP_SEQUENCE: AtomicU64 = AtomicU64::new(0);
    let parent = path.parent().ok_or_else(|| {
        finding(
            "evidence_destination_invalid",
            "evidence destination must have a parent directory",
        )
    })?;
    fs::create_dir_all(parent).map_err(|_| {
        finding(
            "evidence_directory_create_failed",
            "evidence destination directory could not be created",
        )
    })?;
    let sequence = TEMP_SEQUENCE.fetch_add(1, Ordering::Relaxed);
    let temp = path.with_extension(format!("csdlc-v3-{}-{sequence}.tmp", std::process::id()));
    let mut file = fs::File::create(&temp).map_err(|_| {
        finding(
            "evidence_write_failed",
            "evidence temporary file could not be created",
        )
    })?;
    file.write_all(bytes).map_err(|_| {
        finding(
            "evidence_write_failed",
            "evidence temporary file could not be written",
        )
    })?;
    file.sync_all().map_err(|_| {
        finding(
            "evidence_write_failed",
            "evidence temporary file could not be synchronized",
        )
    })?;
    fs::rename(&temp, path).map_err(|_| {
        finding(
            "evidence_commit_failed",
            "evidence file could not be committed atomically",
        )
    })
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn native_install_copies_verifies_and_records_provenance() {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/proof-native-unit")
            .join(std::process::id().to_string());
        let artifact_ref = ".csdlc/evidence/505/install/csdlc";
        let selector_ref = ".csdlc/evidence/505/install/selector.json";
        let provenance_ref = ".csdlc/evidence/505/install/provenance.json";
        fs::create_dir_all(root.join(".csdlc/evidence/505/install")).unwrap();
        fs::write(root.join(artifact_ref), b"native-v3-binary").unwrap();
        fs::write(root.join(selector_ref), b"{\"selected\":\"v3\"}").unwrap();
        fs::write(
            root.join(provenance_ref),
            b"{\"schema\":\"csdlc.v3.install_provenance.v1\",\"source\":\"git:test\"}",
        )
        .unwrap();
        let artifact_digest = blake3::hash(b"native-v3-binary").to_hex().to_string();
        let request = ProofRouteRequest {
            issue: 505,
            repository: "agent-logic/agent-design-language".into(),
            cutover_issue: Some(505),
            operator_approval: Some("operator #505 approval".into()),
            evidence_root: Some(root.to_string_lossy().into_owned()),
            proof: None,
            shadow: None,
            soak: None,
            install: None,
        };
        let install = InstallPlanInput {
            artifact_name: "csdlc".into(),
            artifact_ref: artifact_ref.into(),
            source_provenance_ref: provenance_ref.into(),
            selector_metadata_ref: selector_ref.into(),
            source_provenance: "git:test".into(),
            selected_binary_digest: artifact_digest.clone(),
            observed_binary_digest: artifact_digest,
            selector_metadata_digest: blake3::hash(b"{\"selected\":\"v3\"}").to_hex().to_string(),
            destination: ".adl/bin/csdlc".into(),
            stable_destination: true,
            executes_install: true,
        };

        let receipt_ref = execute_install(&request, &install).unwrap();
        assert_eq!(
            fs::read(root.join(".adl/bin/csdlc")).unwrap(),
            b"native-v3-binary"
        );
        assert_ne!(
            fs::metadata(root.join(".adl/bin/csdlc"))
                .unwrap()
                .permissions()
                .mode()
                & 0o111,
            0
        );
        let receipt: serde_json::Value =
            serde_json::from_slice(&fs::read(root.join(receipt_ref)).unwrap()).unwrap();
        assert_eq!(receipt["verified"], true);
        assert_eq!(receipt["source_provenance"], "git:test");
        fs::remove_dir_all(root).unwrap();
    }
}
