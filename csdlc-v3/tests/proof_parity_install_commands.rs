use std::{fs, path::PathBuf, process::Command, str};

use serde_json::{json, Value};

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

fn write_evidence(name: &str, body: &[u8]) -> (PathBuf, String, String) {
    let root = binary_repo_root();
    let ref_path = scoped_evidence_ref(name);
    let path = root.join(&ref_path);
    let parent = path.parent().expect("evidence path has parent");
    fs::create_dir_all(parent).expect("evidence parent");
    fs::write(&path, body).expect("write evidence");
    (root, ref_path, blake3::hash(body).to_hex().to_string())
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
    assert_eq!(value["read_only"], true);
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
    let (root, proof_ref, digest) = write_evidence("proof.json", br#"{"ok":true}"#);
    assert_ready_value(
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
fn shadow_route_requires_bounded_matching_observations() {
    let (root, v2_ref, digest) = write_evidence("v2.json", br#"{"same":true}"#);
    let v3_ref = scoped_evidence_ref("v3.json");
    let v3_path = root.join(&v3_ref);
    fs::create_dir_all(v3_path.parent().expect("v3 evidence parent")).expect("v3 parent");
    fs::write(&v3_path, br#"{"same":true}"#).expect("write v3 evidence");
    assert_ready_value(
        "shadow",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "v2_observation_ref": v2_ref,
            "v2_digest": digest,
            "v3_observation_ref": v3_ref,
            "v3_digest": digest,
            "bounded_v2": true,
            "bounded_v3": true,
            "broad_equivalence_claim": false
          }
        }),
    );
    assert_blocked(
        "shadow",
        r#"{
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "shadow": {
            "v2_observation_ref": ".csdlc/evidence/631/v2.json",
            "v2_digest": "old",
            "v3_observation_ref": ".csdlc/evidence/631/v3.json",
            "v3_digest": "new",
            "bounded_v2": true,
            "bounded_v3": false,
            "broad_equivalence_claim": true
          }
        }"#,
        "shadow_observation_unbounded",
    );
    let (root, v2_ref, digest) = write_evidence("v2-attack.json", br#"{"real":true}"#);
    let v3_ref = scoped_evidence_ref("v3-attack.json");
    fs::write(root.join(&v3_ref), br#"{"real":false}"#).expect("write attack v3 evidence");
    assert_blocked_value(
        "shadow",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "v2_observation_ref": v2_ref,
            "v2_digest": digest,
            "v3_observation_ref": v3_ref,
            "v3_digest": digest,
            "bounded_v2": true,
            "bounded_v3": true,
            "broad_equivalence_claim": false
          }
        }),
        "shadow_v3_digest_mismatch",
    );
}

#[test]
fn soak_route_refuses_hidden_state_and_provider_side_effects() {
    let (root, soak_ref, _) = write_evidence("soak.json", br#"{"samples":3}"#);
    assert_ready_value(
        "soak",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "soak": {
            "evidence_ref": soak_ref,
            "duration_minutes": 15,
            "sample_count": 3,
            "hidden_state": false,
            "provider_side_effects": false
          }
        }),
    );
    assert_blocked(
        "soak",
        r#"{
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "soak": {
            "evidence_ref": ".csdlc/evidence/631/soak.json",
            "duration_minutes": 15,
            "sample_count": 3,
            "hidden_state": true,
            "provider_side_effects": true
          }
        }"#,
        "soak_hidden_state",
    );
}

#[test]
fn install_route_is_one_binary_plan_gated_by_505() {
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
