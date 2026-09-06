//! Native proof, shadow, soak, and stable-install readiness operations.
//!
//! These routes may retain bounded evidence, but they do not grant live
//! lifecycle authority before explicit #505 cutover approval.

use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
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
    pub normalization: ShadowNormalizationContract,
    pub v2: ShadowCommandSpec,
    pub v3: ShadowCommandSpec,
    pub broad_equivalence_claim: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct SoakEvidence {
    pub normalization: ShadowNormalizationContract,
    pub command: ShadowCommandSpec,
    pub duration_millis: u64,
    pub sample_interval_millis: u64,
    pub hidden_state: bool,
    pub provider_side_effects: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShadowNormalizationContract {
    DoctorIssuePhaseV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShadowGeneration {
    V2,
    V3,
}

#[derive(Debug, Clone, PartialEq, Eq, Deserialize, Serialize)]
pub struct ShadowCommandSpec {
    pub generation: ShadowGeneration,
    pub binary_ref: String,
    pub argv: Vec<String>,
    pub request_ref: String,
    pub timeout_millis: u64,
    pub side_effect_boundary_refs: Vec<String>,
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
    #[serde(default)]
    pub exact_head: String,
    #[serde(default)]
    pub cutover_approval_ref: Option<String>,
    #[serde(default)]
    pub cutover_approval_digest: Option<String>,
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
            match authorize_install_execution(&request, install) {
                Ok(()) => match execute_install(&request, install) {
                    Ok(reference) => {
                        performed_mutation = true;
                        evidence_refs.push(reference);
                    }
                    Err(code) => findings.push(code),
                },
                Err(code) => findings.push(code),
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
    _evidence_root: Option<&str>,
    shadow: &ShadowComparison,
    findings: &mut Vec<ProofRouteFinding>,
) {
    validate_command_spec(&shadow.v2, ShadowGeneration::V2, findings);
    validate_command_spec(&shadow.v3, ShadowGeneration::V3, findings);
    if shadow.broad_equivalence_claim {
        findings.push(finding(
            "shadow_broad_equivalence_claim",
            "shadow route refuses broad equivalence claims",
        ));
    }
}

fn validate_soak(
    _evidence_root: Option<&str>,
    soak: &SoakEvidence,
    findings: &mut Vec<ProofRouteFinding>,
) {
    validate_command_spec(&soak.command, soak.command.generation, findings);
    if soak.duration_millis == 0 || soak.sample_interval_millis == 0 {
        findings.push(finding(
            "soak_sample_missing",
            "soak evidence requires non-zero observed duration and sample interval",
        ));
    }
    if soak.sample_interval_millis > soak.duration_millis {
        findings.push(finding(
            "soak_interval_exceeds_duration",
            "soak sample interval cannot exceed the requested observed duration",
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
}

fn validate_command_spec(
    spec: &ShadowCommandSpec,
    expected_generation: ShadowGeneration,
    findings: &mut Vec<ProofRouteFinding>,
) {
    if spec.generation != expected_generation {
        findings.push(finding(
            "shadow_generation_mismatch",
            "shadow command generation must match its typed lane",
        ));
    }
    require_nonempty(
        &spec.binary_ref,
        "shadow_binary_missing",
        "shadow command requires a repository-relative binary reference",
        findings,
    );
    require_nonempty(
        &spec.request_ref,
        "shadow_request_missing",
        "shadow command requires typed request evidence",
        findings,
    );
    if spec.argv.is_empty() {
        findings.push(finding(
            "shadow_argv_missing",
            "shadow command requires explicit shell-free argv",
        ));
    }
    if spec.timeout_millis == 0 || spec.timeout_millis > 300_000 {
        findings.push(finding(
            "shadow_timeout_invalid",
            "shadow command timeout must be between 1 and 300000 milliseconds",
        ));
    }
    if spec.side_effect_boundary_refs.is_empty() {
        findings.push(finding(
            "shadow_side_effect_boundary_missing",
            "shadow command requires explicit side-effect boundary observations",
        ));
    }
    if spec.provider_side_effects {
        findings.push(finding(
            "shadow_provider_side_effects",
            "shadow and soak commands cannot perform provider side effects before cutover",
        ));
    }
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
    let v2 = execute_shadow_command(&root, &shadow.v2, shadow.normalization)?;
    let v3 = execute_shadow_command(&root, &shadow.v3, shadow.normalization)?;
    if v2.normalized_output != v3.normalized_output {
        return Err(finding(
            "shadow_normalized_mismatch",
            "normalized outputs from executed v2 and v3 commands must match",
        ));
    }
    let base = format!(".csdlc/evidence/{}/v3-shadow", request.issue);
    let request_ref = format!("{base}/request.json");
    let v2_ref = format!("{base}/v2.execution.json");
    let v3_ref = format!("{base}/v3.execution.json");
    let receipt_ref = format!("{base}/comparison.json");
    let request_value = serde_json::to_value(shadow).map_err(|_| {
        finding(
            "shadow_request_serialize_failed",
            "typed shadow request must serialize for retained evidence",
        )
    })?;
    write_canonical_evidence(request, &request_ref, &request_value)?;
    write_canonical_evidence(request, &v2_ref, &v2.to_receipt(request.issue))?;
    write_canonical_evidence(request, &v3_ref, &v3.to_receipt(request.issue))?;
    let digest = blake3::hash(&canonical_json(&v2.normalized_output))
        .to_hex()
        .to_string();
    write_canonical_evidence(
        request,
        &receipt_ref,
        &serde_json::json!({
            "schema": "csdlc.v3.shadow_receipt.v1",
            "issue": request.issue,
            "bounded": true,
            "operational_authority": false,
            "provider_side_effects": false,
            "normalization": shadow.normalization,
            "normalized_digest": digest,
            "request_ref": request_ref,
            "v2_execution_ref": v2_ref,
            "v3_execution_ref": v3_ref,
        }),
    )?;
    Ok(vec![request_ref, v2_ref, v3_ref, receipt_ref])
}

fn execute_soak(
    request: &ProofRouteRequest,
    soak: &SoakEvidence,
) -> Result<String, ProofRouteFinding> {
    const MAX_DURATION_MILLIS: u64 = 86_400_000;
    const MAX_SAMPLES: usize = 10_000;
    if soak.duration_millis > MAX_DURATION_MILLIS {
        return Err(finding(
            "soak_duration_limit_exceeded",
            "bounded soak duration cannot exceed 24 hours",
        ));
    }
    let root = request_root(request)?;
    let requested = Duration::from_millis(soak.duration_millis);
    let interval = Duration::from_millis(soak.sample_interval_millis);
    let started_unix_millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            finding(
                "soak_clock_invalid",
                "system clock must be after the Unix epoch",
            )
        })?
        .as_millis();
    let started = Instant::now();
    let mut samples = Vec::new();
    loop {
        if started.elapsed() >= requested && !samples.is_empty() {
            break;
        }
        if samples.len() >= MAX_SAMPLES {
            return Err(finding(
                "soak_sample_limit_exceeded",
                "bounded soak sample count cannot exceed 10000",
            ));
        }
        let execution = execute_shadow_command(&root, &soak.command, soak.normalization)?;
        samples.push(serde_json::json!({
            "sequence": samples.len() + 1,
            "observed_elapsed_millis": started.elapsed().as_millis(),
            "execution": execution.to_receipt(request.issue),
        }));
        let next_sample = interval.saturating_mul(samples.len() as u32);
        let deadline = next_sample.min(requested);
        if started.elapsed() < deadline {
            thread::sleep(deadline - started.elapsed());
        }
    }
    let elapsed_millis = started.elapsed().as_millis();
    if elapsed_millis < soak.duration_millis as u128 {
        return Err(finding(
            "soak_duration_not_observed",
            "soak cannot retain a duration that was not actually observed monotonically",
        ));
    }
    let reference = format!(".csdlc/evidence/{}/v3-soak/receipt.json", request.issue);
    write_canonical_evidence(
        request,
        &reference,
        &serde_json::json!({
            "schema": "csdlc.v3.soak_receipt.v1",
            "issue": request.issue,
            "operational_authority": false,
            "provider_side_effects": false,
            "normalization": soak.normalization,
            "started_unix_millis": started_unix_millis,
            "requested_duration_millis": soak.duration_millis,
            "observed_elapsed_millis": elapsed_millis,
            "sample_interval_millis": soak.sample_interval_millis,
            "sample_count": samples.len(),
            "request": soak,
            "samples": samples,
        }),
    )?;
    Ok(reference)
}

#[derive(Debug)]
struct ShadowExecution {
    spec: ShadowCommandSpec,
    request_digest: String,
    request_value: serde_json::Value,
    binary_digest: String,
    exit_code: Option<i32>,
    stdout_digest: String,
    stderr_digest: String,
    stdout_len: usize,
    stderr_len: usize,
    normalized_output: serde_json::Value,
    elapsed_millis: u128,
    side_effect_boundary: Vec<serde_json::Value>,
}

impl ShadowExecution {
    fn to_receipt(&self, issue: u64) -> serde_json::Value {
        serde_json::json!({
            "schema": "csdlc.v3.shadow_execution.v1",
            "issue": issue,
            "request": self.spec,
            "request_evidence": {
                "ref": self.spec.request_ref,
                "digest": self.request_digest,
                "value": self.request_value,
            },
            "argv": self.spec.argv,
            "binary": {"identity": self.spec.binary_ref, "digest": self.binary_digest},
            "exit": {"code": self.exit_code, "success": self.exit_code == Some(0), "timed_out": false},
            "stdout_digest": self.stdout_digest,
            "stderr_digest": self.stderr_digest,
            "stdout_len": self.stdout_len,
            "stderr_len": self.stderr_len,
            "normalized_output": self.normalized_output,
            "elapsed_millis": self.elapsed_millis,
            "side_effect_boundary": self.side_effect_boundary,
            "environment": "cleared_no_provider_credentials",
            "provider_side_effects": false,
            "operational_authority": false,
        })
    }
}

fn execute_shadow_command(
    root: &Path,
    spec: &ShadowCommandSpec,
    normalization: ShadowNormalizationContract,
) -> Result<ShadowExecution, ProofRouteFinding> {
    let binary = resolve_repo_path(root, &spec.binary_ref, true)?;
    let request_path = resolve_repo_path(root, &spec.request_ref, true)?;
    let binary_bytes = fs::read(&binary).map_err(|_| {
        finding(
            "shadow_binary_unreadable",
            "shadow command binary must be readable",
        )
    })?;
    let request_bytes = fs::read(&request_path).map_err(|_| {
        finding(
            "shadow_request_unreadable",
            "shadow request evidence must be readable",
        )
    })?;
    let request_value: serde_json::Value =
        serde_json::from_slice(&request_bytes).map_err(|_| {
            finding(
                "shadow_request_invalid",
                "shadow request evidence must be typed JSON",
            )
        })?;
    let request_issue = request_value
        .get("issue")
        .and_then(serde_json::Value::as_u64)
        .ok_or_else(|| {
            finding(
                "shadow_request_issue_missing",
                "shadow request evidence must bind the executed issue",
            )
        })?;
    let before = snapshot_boundaries(root, &spec.side_effect_boundary_refs)?;
    let started = Instant::now();
    let mut child = Command::new(&binary)
        .current_dir(root)
        .args(&spec.argv)
        .env_clear()
        .env("PATH", "/usr/bin:/bin")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| {
            finding(
                "shadow_command_spawn_failed",
                "shadow command could not start",
            )
        })?;
    let mut stdout = child.stdout.take().ok_or_else(|| {
        finding(
            "shadow_stdout_unavailable",
            "shadow stdout must be captured",
        )
    })?;
    let mut stderr = child.stderr.take().ok_or_else(|| {
        finding(
            "shadow_stderr_unavailable",
            "shadow stderr must be captured",
        )
    })?;
    let stdout_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let stderr_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    let timeout = Duration::from_millis(spec.timeout_millis);
    let status = loop {
        if let Some(status) = child.try_wait().map_err(|_| {
            finding(
                "shadow_command_wait_failed",
                "shadow command status could not be observed",
            )
        })? {
            break status;
        }
        if started.elapsed() >= timeout {
            let _ = child.kill();
            let _ = child.wait();
            return Err(finding(
                "shadow_command_timeout",
                "shadow command exceeded its typed bounded timeout",
            ));
        }
        thread::sleep(Duration::from_millis(5));
    };
    let stdout = stdout_reader
        .join()
        .ok()
        .and_then(Result::ok)
        .ok_or_else(|| finding("shadow_stdout_unreadable", "shadow stdout capture failed"))?;
    let stderr = stderr_reader
        .join()
        .ok()
        .and_then(Result::ok)
        .ok_or_else(|| finding("shadow_stderr_unreadable", "shadow stderr capture failed"))?;
    let normalized_output = normalize_shadow_output(spec.generation, normalization, &stdout)?;
    if normalized_output
        .get("issue")
        .and_then(serde_json::Value::as_u64)
        != Some(request_issue)
    {
        return Err(finding(
            "shadow_request_issue_mismatch",
            "shadow request evidence must match the executed issue observed in typed output",
        ));
    }
    let after = snapshot_boundaries(root, &spec.side_effect_boundary_refs)?;
    let side_effect_boundary = spec.side_effect_boundary_refs.iter().map(|reference| {
        let before_digest = before.get(reference).cloned().flatten();
        let after_digest = after.get(reference).cloned().flatten();
        serde_json::json!({"ref": reference, "before_digest": before_digest, "after_digest": after_digest, "changed": before_digest != after_digest})
    }).collect::<Vec<_>>();
    if side_effect_boundary
        .iter()
        .any(|entry| entry["changed"] == true)
    {
        return Err(finding(
            "shadow_side_effect_boundary_changed",
            "shadow command changed a declared read-only side-effect boundary",
        ));
    }
    Ok(ShadowExecution {
        spec: spec.clone(),
        request_digest: blake3::hash(&request_bytes).to_hex().to_string(),
        request_value,
        binary_digest: blake3::hash(&binary_bytes).to_hex().to_string(),
        exit_code: status.code(),
        stdout_digest: blake3::hash(&stdout).to_hex().to_string(),
        stderr_digest: blake3::hash(&stderr).to_hex().to_string(),
        stdout_len: stdout.len(),
        stderr_len: stderr.len(),
        normalized_output,
        elapsed_millis: started.elapsed().as_millis(),
        side_effect_boundary,
    })
}

fn normalize_shadow_output(
    generation: ShadowGeneration,
    contract: ShadowNormalizationContract,
    stdout: &[u8],
) -> Result<serde_json::Value, ProofRouteFinding> {
    let value: serde_json::Value = serde_json::from_slice(stdout).map_err(|_| {
        finding(
            "shadow_output_not_json",
            "shadow stdout must be one typed JSON document",
        )
    })?;
    let (issue, phase) = match (generation, contract) {
        (ShadowGeneration::V2, ShadowNormalizationContract::DoctorIssuePhaseV1) => {
            if value["schema"] != "csdlc.doctor.report.v1" {
                return Err(finding(
                    "shadow_output_schema_mismatch",
                    "v2 doctor output must use the typed doctor report schema",
                ));
            }
            (value.get("issue"), value.get("phase"))
        }
        (ShadowGeneration::V3, ShadowNormalizationContract::DoctorIssuePhaseV1) => {
            if value["schema"] != "csdlc.v3.local_preparation.v1" || value["command"] != "doctor" {
                return Err(finding(
                    "shadow_output_schema_mismatch",
                    "v3 doctor output must use the typed local preparation schema",
                ));
            }
            (
                value.pointer("/result/issue"),
                value.pointer("/result/lifecycle_state/phase"),
            )
        }
    };
    let issue = issue.and_then(serde_json::Value::as_u64).ok_or_else(|| {
        finding(
            "shadow_normalization_issue_missing",
            "doctor normalization requires a typed issue identity",
        )
    })?;
    let phase = phase.and_then(serde_json::Value::as_str).ok_or_else(|| {
        finding(
            "shadow_normalization_phase_missing",
            "doctor normalization requires an observed lifecycle phase",
        )
    })?;
    Ok(
        serde_json::json!({"contract": "doctor_issue_phase.v1", "command": "doctor", "issue": issue, "phase": phase}),
    )
}

fn snapshot_boundaries(
    root: &Path,
    references: &[String],
) -> Result<BTreeMap<String, Option<String>>, ProofRouteFinding> {
    references
        .iter()
        .map(|reference| {
            let path = resolve_repo_path(root, reference, false)?;
            Ok((reference.clone(), digest_path(&path)?))
        })
        .collect()
}

fn resolve_repo_path(
    root: &Path,
    reference: &str,
    must_exist: bool,
) -> Result<PathBuf, ProofRouteFinding> {
    let relative = Path::new(reference);
    if relative.is_absolute()
        || relative
            .components()
            .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(finding(
            "shadow_path_not_repo_relative",
            "shadow paths must be repository-relative and cannot traverse parents",
        ));
    }
    let path = root.join(relative);
    if must_exist && !path.is_file() {
        return Err(finding(
            "shadow_path_unavailable",
            "shadow command input must exist as a repository file",
        ));
    }
    Ok(path)
}

fn digest_path(path: &Path) -> Result<Option<String>, ProofRouteFinding> {
    if !path.exists() {
        return Ok(None);
    }
    let metadata = path.symlink_metadata().map_err(|_| {
        finding(
            "shadow_boundary_unreadable",
            "shadow side-effect boundary must be readable",
        )
    })?;
    if metadata.file_type().is_symlink() {
        return Err(finding(
            "shadow_boundary_symlink",
            "shadow side-effect boundaries cannot be symlinks",
        ));
    }
    if metadata.is_file() {
        return fs::read(path)
            .map(|bytes| Some(blake3::hash(&bytes).to_hex().to_string()))
            .map_err(|_| {
                finding(
                    "shadow_boundary_unreadable",
                    "shadow side-effect boundary must be readable",
                )
            });
    }
    let mut entries = fs::read_dir(path)
        .map_err(|_| {
            finding(
                "shadow_boundary_unreadable",
                "shadow boundary directory must be readable",
            )
        })?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| {
            finding(
                "shadow_boundary_unreadable",
                "shadow boundary directory must be readable",
            )
        })?;
    entries.sort_by_key(|entry| entry.file_name());
    let mut manifest = BTreeMap::new();
    for entry in entries {
        manifest.insert(
            entry.file_name().to_string_lossy().to_string(),
            digest_path(&entry.path())?,
        );
    }
    let value = serde_json::to_value(manifest).map_err(|_| {
        finding(
            "shadow_boundary_digest_failed",
            "shadow boundary digest could not be serialized",
        )
    })?;
    Ok(Some(
        blake3::hash(&canonical_json(&value)).to_hex().to_string(),
    ))
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

fn authorize_install_execution(
    request: &ProofRouteRequest,
    install: &InstallPlanInput,
) -> Result<(), ProofRouteFinding> {
    let root = request_root(request)?;
    if active_canonical_v3_selector(&root, install)? {
        return Ok(());
    }
    let approval_ref = install.cutover_approval_ref.as_deref().ok_or_else(|| finding("install_typed_authority_missing", "executing install requires active canonical v3 authority or an exact typed #505 approval receipt"))?;
    if !approval_ref.starts_with(".csdlc/evidence/505/") {
        return Err(finding(
            "install_approval_ref_not_canonical",
            "typed cutover approval must be retained under .csdlc/evidence/505",
        ));
    }
    let bytes = fs::read(root.join(approval_ref)).map_err(|_| {
        finding(
            "install_approval_unreadable",
            "typed cutover approval receipt must be readable",
        )
    })?;
    let digest = blake3::hash(&bytes).to_hex().to_string();
    if install.cutover_approval_digest.as_deref() != Some(digest.as_str()) {
        return Err(finding(
            "install_approval_digest_mismatch",
            "typed cutover approval digest must match retained evidence",
        ));
    }
    let approval: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "install_approval_invalid",
            "typed cutover approval receipt must be valid JSON",
        )
    })?;
    let exact_head_valid = install.exact_head.len() == 40
        && install
            .exact_head
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit());
    if approval["schema"] != "csdlc.v3.cutover_approval.v1"
        || approval["authority_issue"] != 505
        || approval["repository"] != request.repository
        || approval["decision"] != "approved"
        || approval["exact_head"] != install.exact_head
        || approval["selected_binary_digest"] != install.selected_binary_digest
        || approval["selector_metadata_digest"] != install.selector_metadata_digest
        || !exact_head_valid
    {
        return Err(finding("install_approval_not_exact", "typed #505 approval must bind repository, exact head, binary digest, and selector digest"));
    }
    Ok(())
}

fn active_canonical_v3_selector(
    root: &Path,
    install: &InstallPlanInput,
) -> Result<bool, ProofRouteFinding> {
    let Ok(bytes) = fs::read(root.join(".csdlc/authority-selector.json")) else {
        return Ok(false);
    };
    let selector: serde_json::Value = serde_json::from_slice(&bytes).map_err(|_| {
        finding(
            "install_canonical_selector_invalid",
            "canonical v3 authority selector must be typed valid JSON",
        )
    })?;
    if selector["schema"] != "csdlc.authority_selector.v1"
        || selector["authority_issue"] != 505
        || selector["default_generation"] != "v3"
        || selector["operational_authority"] != "csdlc-v3"
        || selector["binary"] != ".adl/bin/csdlc"
        || selector["selected_binary_digest"] != install.selected_binary_digest
    {
        return Err(finding(
            "install_canonical_selector_mismatch",
            "canonical selector does not grant v3 authority for the selected binary digest",
        ));
    }
    Ok(true)
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
            exact_head: "git:test".into(),
            cutover_approval_ref: Some("#505 operator approval".into()),
            cutover_approval_digest: Some(
                blake3::hash(b"#505 operator approval").to_hex().to_string(),
            ),
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
