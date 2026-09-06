use std::{
    fs,
    os::unix::fs::PermissionsExt,
    path::PathBuf,
    process::Command,
    str,
    sync::{Mutex, MutexGuard, OnceLock},
};

use serde_json::{json, Value};

const SHADOW_TARGET_ISSUE: u64 = 505;

struct ScratchGuard {
    _lock: MutexGuard<'static, ()>,
    evidence_dir: PathBuf,
    target_dir: PathBuf,
}

impl ScratchGuard {
    fn new() -> Self {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        let lock = LOCK
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("proof-route scratch lock");
        let evidence_dir = binary_repo_root()
            .join(".csdlc")
            .join("evidence")
            .join("631")
            .join("proof-route-tests")
            .join(std::process::id().to_string());
        let target_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join("issue-631-proof-route-tests")
            .join(std::process::id().to_string());
        let _ = fs::remove_dir_all(&evidence_dir);
        let _ = fs::remove_dir_all(&target_dir);
        fs::create_dir_all(&target_dir).expect("target scratch dir");
        Self {
            _lock: lock,
            evidence_dir,
            target_dir,
        }
    }
}

impl Drop for ScratchGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.evidence_dir);
        let _ = fs::remove_dir_all(&self.target_dir);
    }
}

fn scratch() -> PathBuf {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("issue-631-proof-route-tests")
        .join(std::process::id().to_string());
    fs::create_dir_all(&dir).expect("scratch dir");
    dir
}

fn write_request(name: &str, body: &str) -> PathBuf {
    let path = scratch().join(name);
    fs::write(&path, body).expect("write request");
    path
}

fn binary_repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("csdlc-v3 has repository parent")
        .to_path_buf()
}

fn scoped_evidence_ref(name: &str) -> String {
    format!(
        ".csdlc/evidence/631/proof-route-tests/{}/{}",
        std::process::id(),
        name
    )
}

fn issue_index(issue: u64) -> Value {
    serde_json::from_slice(
        &fs::read(binary_repo_root().join(format!(".csdlc/issues/{issue}/index.json")))
            .expect("issue index"),
    )
    .expect("issue index json")
}

fn issue_digest(issue: u64) -> String {
    issue_index(issue)["digest"]
        .as_str()
        .expect("issue digest")
        .to_string()
}

fn issue_phase(issue: u64) -> String {
    issue_index(issue)["phase"]
        .as_str()
        .expect("issue phase")
        .to_string()
}

fn write_evidence(name: &str, body: &[u8]) -> (PathBuf, String, String) {
    let root = binary_repo_root();
    let ref_path = scoped_evidence_ref(name);
    let path = root.join(&ref_path);
    let parent = path.parent().expect("evidence path has parent");
    fs::create_dir_all(parent).expect("evidence parent");
    fs::write(&path, body).expect("write evidence");
    (root, ref_path, blake3::hash(body).to_hex().to_string())
}

fn repo_ref(path: &std::path::Path) -> String {
    path.strip_prefix(binary_repo_root())
        .expect("fixture remains inside repository")
        .to_string_lossy()
        .to_string()
}

fn repo_local_v3_binary_ref() -> String {
    let source = PathBuf::from(env!("CARGO_BIN_EXE_csdlc"));
    let destination = scratch().join("csdlc-v3-shadow-bin");
    fs::copy(&source, &destination).expect("copy v3 binary into repo-local scratch");
    let mut permissions = fs::metadata(&destination)
        .expect("v3 binary metadata")
        .permissions();
    permissions.set_mode(0o755);
    fs::set_permissions(&destination, permissions).expect("v3 binary executable");
    repo_ref(&destination)
}

fn write_typed_request(name: &str, value: Value) -> String {
    let path = scratch().join(name);
    fs::write(&path, serde_json::to_vec_pretty(&value).unwrap()).expect("typed request fixture");
    repo_ref(&path)
}

fn v2_doctor_spec(issue_argument: u64) -> Value {
    let request_ref = write_typed_request(
        &format!("v2-doctor-{issue_argument}.json"),
        json!({
            "schema": "csdlc.v3.shadow_command_request.v1",
            "issue": issue_argument,
            "command": "doctor",
            "generation": "v2"
        }),
    );
    json!({
        "generation": "v2",
        "binary_ref": ".adl/bin/csdlc-v2/csdlc-doctor",
        "argv": ["--repo", ".", "--issue", issue_argument.to_string()],
        "request_ref": request_ref,
        "timeout_millis": 10_000,
        "side_effect_boundary_refs": [format!(".csdlc/issues/{issue_argument}/index.json")],
        "provider_side_effects": false
    })
}

fn v3_doctor_spec() -> Value {
    v3_doctor_spec_for(
        SHADOW_TARGET_ISSUE,
        "[v0.92.1][V3-F] C-SDLC v3 authority transition decision",
    )
}

fn v3_doctor_spec_for(issue: u64, title: &str) -> Value {
    let root = binary_repo_root();
    let request_ref = write_typed_request(
        "v3-doctor-request.json",
        json!({
            "issue": issue,
            "title": title,
            "repository": "agent-logic/agent-design-language",
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": root,
            "registry_version": "1.0.3",
            "expected_lifecycle_digest": issue_digest(issue),
            "commands": ["prepare_issue", "bind_worktree", "edit_cards", "plan_pvf", "doctor", "schedule", "shepherd", "eligibility"],
            "card_updates": {}
        }),
    );
    let registrations_ref = write_typed_request(
        "v3-doctor-registrations.json",
        json!([{
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": binary_repo_root(),
            "primary": false
        }]),
    );
    json!({
        "generation": "v3",
        "binary_ref": repo_local_v3_binary_ref(),
        "argv": [
            "doctor", "--request", request_ref,
            "--registry", "docs/templates/prompts/current.json",
            "--registrations", registrations_ref,
            "--repo-root", "."
        ],
        "request_ref": request_ref,
        "timeout_millis": 10_000,
        "side_effect_boundary_refs": [".csdlc/issues/505/index.json"],
        "provider_side_effects": false
    })
}

fn run_route(route: &str, body: &str) -> Value {
    let path = write_request(&format!("{route}.json"), body);
    let request_value: Value = serde_json::from_str(body).expect("request body json");
    let evidence_root = request_value
        .get("evidence_root")
        .and_then(Value::as_str)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_MANIFEST_DIR")));
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .current_dir(evidence_root)
        .args([route, "--request"])
        .arg(path)
        .output()
        .unwrap_or_else(|error| panic!("csdlc {route} should run: {error}"));
    assert!(
        output.status.success(),
        "{route} stderr: {}",
        str::from_utf8(&output.stderr).unwrap()
    );
    serde_json::from_slice(&output.stdout).expect("route stdout should be json")
}

fn run_route_value(route: &str, body: Value) -> Value {
    run_route(
        route,
        &serde_json::to_string_pretty(&body).expect("request json"),
    )
}

fn assert_ready_value(route: &str, body: Value) {
    let value = run_route_value(route, body);
    assert_eq!(value["schema"], "csdlc.v3.proof_route.v1");
    assert_eq!(value["route"], route);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["status"], "ready");
    assert!(value["findings"].as_array().unwrap().is_empty());
}

fn assert_blocked(route: &str, body: &str, code: &str) {
    let value = run_route(route, body);
    assert_eq!(value["status"], "blocked");
    let findings = value["findings"].as_array().unwrap();
    assert!(
        findings.iter().any(|finding| finding["code"] == code),
        "{route} findings should contain {code}: {findings:?}"
    );
}

fn assert_blocked_value(route: &str, body: Value, code: &str) {
    let value = run_route_value(route, body);
    assert_eq!(value["status"], "blocked");
    let findings = value["findings"].as_array().unwrap();
    assert!(
        findings.iter().any(|finding| finding["code"] == code),
        "{route} findings should contain {code}: {findings:?}"
    );
}

#[test]
fn proof_route_accepts_fresh_deterministic_manifest_only() {
    let _scratch = ScratchGuard::new();
    let (root, proof_ref, digest) = write_evidence("proof.json", br#"{"ok":true}"#);
    assert_ready_value(
        "proof",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "operator_approval": "operator #505 approval text is not typed authority",
          "cutover_issue": 505,
          "evidence_root": root,
          "proof": {
            "manifest_id": "pvf-631",
            "lane": "proof-parity-install",
            "deterministic": true,
            "evidence_ref": proof_ref,
            "evidence_digest": digest,
            "observed_digest": digest,
            "stale": false
          }
        }),
    );
    assert_blocked(
        "proof",
        r#"{
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "proof": {
            "manifest_id": "pvf-631",
            "lane": "proof-parity-install",
            "deterministic": false,
            "evidence_ref": ".csdlc/evidence/631/proof.json",
            "evidence_digest": "abc123",
            "observed_digest": "def456",
            "stale": true
          }
        }"#,
        "proof_lane_not_deterministic",
    );
    let (root, actual_ref, digest) = write_evidence("actual.json", br#"{"actual":true}"#);
    assert_blocked_value(
        "proof",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "proof": {
            "manifest_id": "pvf-631",
            "lane": "proof-parity-install",
            "deterministic": true,
            "evidence_ref": actual_ref,
            "evidence_digest": digest,
            "observed_digest": "caller-forged",
            "stale": false
          }
        }),
        "proof_observed_digest_mismatch",
    );
}

#[test]
fn proof_route_retains_a_deterministic_native_receipt() {
    let _scratch = ScratchGuard::new();
    let (root, proof_ref, digest) = write_evidence("native-proof.json", br#"{"ok":true}"#);
    let value = run_route_value(
        "proof",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "proof": {
            "manifest_id": "native-proof",
            "lane": "deterministic",
            "deterministic": true,
            "evidence_ref": proof_ref,
            "evidence_digest": digest,
            "observed_digest": digest,
            "stale": false
          }
        }),
    );
    assert_eq!(value["performed_mutation"], true);
    let receipt = binary_repo_root().join(value["evidence_refs"][0].as_str().unwrap());
    let first = fs::read(&receipt).expect("native proof receipt");
    let rerun = run_route_value(
        "proof",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": binary_repo_root(),
          "proof": {
            "manifest_id": "native-proof",
            "lane": "deterministic",
            "deterministic": true,
            "evidence_ref": scoped_evidence_ref("native-proof.json"),
            "evidence_digest": digest,
            "observed_digest": digest,
            "stale": false
          }
        }),
    );
    assert_eq!(rerun["status"], "ready");
    assert_eq!(fs::read(receipt).unwrap(), first);
}

#[test]
fn shadow_route_executes_real_typed_v2_and_v3_doctor_commands() {
    let _scratch = ScratchGuard::new();
    let root = binary_repo_root();
    let value = run_route_value(
        "shadow",
        json!({
          "issue": 505,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "normalization": "doctor_issue_phase_v1",
            "v2": v2_doctor_spec(SHADOW_TARGET_ISSUE),
            "v3": v3_doctor_spec(),
            "broad_equivalence_claim": false
          }
        }),
    );
    assert_eq!(value["status"], "ready");
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["performed_mutation"], true);
    assert_eq!(value["evidence_refs"].as_array().unwrap().len(), 4);
    let v2_receipt = binary_repo_root().join(value["evidence_refs"][1].as_str().unwrap());
    let receipt: Value = serde_json::from_slice(&fs::read(v2_receipt).unwrap()).unwrap();
    assert_eq!(receipt["schema"], "csdlc.v3.shadow_execution.v1");
    assert!(receipt["exit"]["success"].is_boolean());
    assert_eq!(receipt["provider_side_effects"], false);
    assert_eq!(receipt["operational_authority"], false);
    assert!(receipt["binary"]["digest"].as_str().unwrap().len() >= 64);
    assert!(receipt["stdout_digest"].as_str().unwrap().len() >= 64);
    assert!(receipt["stderr_digest"].as_str().unwrap().len() >= 64);
    assert_eq!(receipt["normalized_output"]["issue"], SHADOW_TARGET_ISSUE);
    assert_eq!(
        receipt["normalized_output"]["phase"],
        issue_phase(SHADOW_TARGET_ISSUE)
    );
    assert_eq!(receipt["side_effect_boundary"][0]["changed"], false);
}

#[test]
fn shadow_route_fails_closed_on_real_lifecycle_mismatch_and_provider_effects() {
    let _scratch = ScratchGuard::new();
    let root = binary_repo_root();
    assert_blocked_value(
        "shadow",
        json!({
          "issue": 505,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "normalization": "doctor_issue_phase_v1",
            "v2": v2_doctor_spec(SHADOW_TARGET_ISSUE),
            "v3": v3_doctor_spec_for(210, "[v0.91.6] Shadow parity ready issue fixture"),
            "broad_equivalence_claim": false
          }
        }),
        "shadow_normalized_mismatch",
    );
    let mut provider_spec = v3_doctor_spec();
    provider_spec["provider_side_effects"] = json!(true);
    assert_blocked_value(
        "shadow",
        json!({
          "issue": 505,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": binary_repo_root(),
          "shadow": {
            "normalization": "doctor_issue_phase_v1",
            "v2": v2_doctor_spec(SHADOW_TARGET_ISSUE),
            "v3": provider_spec,
            "broad_equivalence_claim": false
          }
        }),
        "shadow_provider_side_effects",
    );
}

#[test]
fn soak_route_observes_real_monotonic_duration_with_bounded_commands() {
    let _scratch = ScratchGuard::new();
    let root = binary_repo_root();
    let value = run_route_value(
        "soak",
        json!({
          "issue": 505,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "soak": {
            "normalization": "doctor_issue_phase_v1",
            "command": v2_doctor_spec(SHADOW_TARGET_ISSUE),
            "duration_millis": 25,
            "sample_interval_millis": 5,
            "hidden_state": false,
            "provider_side_effects": false
          }
        }),
    );
    let receipt = binary_repo_root().join(value["evidence_refs"][0].as_str().unwrap());
    let receipt: Value = serde_json::from_slice(&fs::read(receipt).unwrap()).unwrap();
    assert!(receipt["observed_elapsed_millis"].as_u64().unwrap() >= 25);
    assert_eq!(receipt["requested_duration_millis"], 25);
    assert!(receipt["sample_count"].as_u64().unwrap() >= 1);
    assert_eq!(receipt["operational_authority"], false);
    assert_eq!(receipt["provider_side_effects"], false);
    assert!(receipt["started_unix_millis"].as_u64().unwrap() > 0);
    assert!(receipt["samples"][0]["execution"]["elapsed_millis"]
        .as_u64()
        .is_some());
}

#[test]
fn install_route_is_one_binary_plan_gated_by_505() {
    let _scratch = ScratchGuard::new();
    let (root, artifact_ref, artifact_digest) =
        write_evidence("install/csdlc", b"single-binary-artifact");
    let selector_ref = scoped_evidence_ref("install/selector.json");
    let selector_path = root.join(&selector_ref);
    fs::create_dir_all(selector_path.parent().expect("selector parent")).expect("selector parent");
    fs::write(&selector_path, br#"{"selected":"csdlc"}"#).expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    let provenance_ref = scoped_evidence_ref("install/provenance.json");
    fs::write(
        root.join(&provenance_ref),
        br#"{"schema":"csdlc.v3.install_provenance.v1","source":"git:abc123"}"#,
    )
    .expect("write install provenance");
    assert_ready_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": artifact_ref,
            "source_provenance_ref": provenance_ref,
            "selector_metadata_ref": selector_ref,
            "source_provenance": "git:abc123",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": artifact_digest,
            "selector_metadata_digest": selector_digest,
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": false
          }
        }),
    );
    assert_blocked(
        "install",
        r#"{
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 504,
          "install": {
            "artifact_name": "csdlc-v3",
            "artifact_ref": ".csdlc/evidence/631/install/csdlc",
            "source_provenance_ref": ".csdlc/evidence/631/install/provenance.json",
            "selector_metadata_ref": ".csdlc/evidence/631/install/selector.json",
            "source_provenance": "git:abc123",
            "selected_binary_digest": "bin123",
            "observed_binary_digest": "different",
            "selector_metadata_digest": "selector123",
            "destination": "csdlc-v3/target/debug/csdlc",
            "stable_destination": false,
            "executes_install": true
          }
        }"#,
        "install_cutover_issue_missing",
    );
    let (root, artifact_ref, artifact_digest) =
        write_evidence("install/actual-csdlc", b"actual artifact bytes");
    let selector_ref = scoped_evidence_ref("install/selector-actual.json");
    fs::write(root.join(&selector_ref), br#"{"selected":"actual"}"#)
        .expect("write selector metadata");
    let provenance_ref = scoped_evidence_ref("install/provenance-actual.json");
    fs::write(
        root.join(&provenance_ref),
        br#"{"schema":"csdlc.v3.install_provenance.v1","source":"git:def456"}"#,
    )
    .expect("write install provenance");
    assert_blocked_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": artifact_ref,
            "source_provenance_ref": provenance_ref,
            "selector_metadata_ref": selector_ref,
            "source_provenance": "git:def456",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": "caller-forged",
            "selector_metadata_digest": "selector-forged",
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": false
          }
        }),
        "install_observed_binary_digest_mismatch",
    );
    let (root, artifact_ref, artifact_digest) = write_evidence(
        "install/forged-provenance-csdlc",
        b"forged provenance artifact bytes",
    );
    let selector_ref = scoped_evidence_ref("install/forged-selector.json");
    fs::write(root.join(&selector_ref), br#"{"selected":"csdlc"}"#)
        .expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    let provenance_ref = scoped_evidence_ref("install/forged-provenance.json");
    fs::write(
        root.join(&provenance_ref),
        br#"{"schema":"csdlc.v3.install_provenance.v1","source":"git:real-source"}"#,
    )
    .expect("write install provenance");
    assert_blocked_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": artifact_ref,
            "source_provenance_ref": provenance_ref,
            "selector_metadata_ref": selector_ref,
            "source_provenance": "git:caller-forged-source",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": artifact_digest,
            "selector_metadata_digest": selector_digest,
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": false
          }
        }),
        "install_source_provenance_mismatch",
    );
    let (root, artifact_ref, artifact_digest) = write_evidence(
        "install/untyped-provenance-csdlc",
        b"untyped provenance artifact bytes",
    );
    let selector_ref = scoped_evidence_ref("install/untyped-selector.json");
    fs::write(root.join(&selector_ref), br#"{"selected":"csdlc"}"#)
        .expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    let provenance_ref = scoped_evidence_ref("install/untyped-provenance.json");
    fs::write(root.join(&provenance_ref), br#"{"source":"git:untyped"}"#)
        .expect("write untyped install provenance");
    assert_blocked_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": artifact_ref,
            "source_provenance_ref": provenance_ref,
            "selector_metadata_ref": selector_ref,
            "source_provenance": "git:untyped",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": artifact_digest,
            "selector_metadata_digest": selector_digest,
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": false
          }
        }),
        "install_source_provenance_invalid",
    );
    let (root, artifact_ref, artifact_digest) =
        write_evidence("install/no-approval-csdlc", b"no approval artifact bytes");
    let selector_ref = scoped_evidence_ref("install/no-approval-selector.json");
    fs::write(root.join(&selector_ref), br#"{"selected":"csdlc"}"#)
        .expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    let provenance_ref = scoped_evidence_ref("install/no-approval-provenance.json");
    fs::write(
        root.join(&provenance_ref),
        br#"{"schema":"csdlc.v3.install_provenance.v1","source":"git:no-approval"}"#,
    )
    .expect("write install provenance");
    assert_blocked_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": artifact_ref,
            "source_provenance_ref": provenance_ref,
            "selector_metadata_ref": selector_ref,
            "source_provenance": "git:no-approval",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": artifact_digest,
            "selector_metadata_digest": selector_digest,
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": true
          }
        }),
        "install_typed_authority_missing",
    );
    let scratch_root = scratch().join("caller-controlled-evidence-root");
    fs::create_dir_all(scratch_root.join(".csdlc/evidence/631/install"))
        .expect("scratch evidence root");
    fs::write(
        scratch_root.join(".csdlc/evidence/631/install/scratch-csdlc"),
        b"scratch artifact bytes",
    )
    .expect("write scratch artifact");
    fs::write(
        scratch_root.join(".csdlc/evidence/631/install/scratch-selector.json"),
        br#"{"selected":"csdlc"}"#,
    )
    .expect("write scratch selector");
    fs::write(
        scratch_root.join(".csdlc/evidence/631/install/scratch-provenance.json"),
        br#"{"schema":"csdlc.v3.install_provenance.v1","source":"git:scratch"}"#,
    )
    .expect("write scratch provenance");
    let artifact_digest = blake3::hash(b"scratch artifact bytes").to_hex().to_string();
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    assert_blocked_value(
        "install",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "cutover_issue": 505,
          "evidence_root": scratch_root,
          "install": {
            "artifact_name": "csdlc",
            "artifact_ref": ".csdlc/evidence/631/install/scratch-csdlc",
            "source_provenance_ref": ".csdlc/evidence/631/install/scratch-provenance.json",
            "selector_metadata_ref": ".csdlc/evidence/631/install/scratch-selector.json",
            "source_provenance": "git:scratch",
            "selected_binary_digest": artifact_digest,
            "observed_binary_digest": artifact_digest,
            "selector_metadata_digest": selector_digest,
            "destination": ".adl/bin/csdlc",
            "stable_destination": true,
            "executes_install": false
          }
        }),
        "evidence_root_not_repository_root",
    );
}
