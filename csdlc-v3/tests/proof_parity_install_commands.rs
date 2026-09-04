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

fn write_evidence(ref_path: &str, body: &[u8]) -> (PathBuf, String) {
    let root = scratch().join("repo");
    let path = root.join(ref_path);
    let parent = path.parent().expect("evidence path has parent");
    fs::create_dir_all(parent).expect("evidence parent");
    fs::write(&path, body).expect("write evidence");
    (root, blake3::hash(body).to_hex().to_string())
}

fn run_route(route: &str, body: &str) -> Value {
    let path = write_request(&format!("{route}.json"), body);
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
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
    let (root, digest) = write_evidence(".csdlc/evidence/631/proof.json", br#"{"ok":true}"#);
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
            "evidence_ref": ".csdlc/evidence/631/proof.json",
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
    let (root, digest) = write_evidence(".csdlc/evidence/631/actual.json", br#"{"actual":true}"#);
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
            "evidence_ref": ".csdlc/evidence/631/actual.json",
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
    let (root, digest) = write_evidence(".csdlc/evidence/631/v2.json", br#"{"same":true}"#);
    let v3_path = root.join(".csdlc/evidence/631/v3.json");
    fs::write(&v3_path, br#"{"same":true}"#).expect("write v3 evidence");
    assert_ready_value(
        "shadow",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "v2_observation_ref": ".csdlc/evidence/631/v2.json",
            "v2_digest": digest,
            "v3_observation_ref": ".csdlc/evidence/631/v3.json",
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
    let (root, digest) = write_evidence(".csdlc/evidence/631/v2-attack.json", br#"{"real":true}"#);
    fs::write(
        root.join(".csdlc/evidence/631/v3-attack.json"),
        br#"{"real":false}"#,
    )
    .expect("write attack v3 evidence");
    assert_blocked_value(
        "shadow",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "shadow": {
            "v2_observation_ref": ".csdlc/evidence/631/v2-attack.json",
            "v2_digest": digest,
            "v3_observation_ref": ".csdlc/evidence/631/v3-attack.json",
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
    let (root, _) = write_evidence(".csdlc/evidence/631/soak.json", br#"{"samples":3}"#);
    assert_ready_value(
        "soak",
        json!({
          "issue": 631,
          "repository": "agent-logic/agent-design-language",
          "evidence_root": root,
          "soak": {
            "evidence_ref": ".csdlc/evidence/631/soak.json",
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
    let (root, artifact_digest) = write_evidence(
        ".csdlc/evidence/631/install/csdlc",
        b"single-binary-artifact",
    );
    let selector_path = root.join(".csdlc/evidence/631/install/selector.json");
    fs::write(&selector_path, br#"{"selected":"csdlc"}"#).expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    fs::write(
        root.join(".csdlc/evidence/631/install/provenance.json"),
        br#"{"source":"git:abc123"}"#,
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
            "artifact_ref": ".csdlc/evidence/631/install/csdlc",
            "source_provenance_ref": ".csdlc/evidence/631/install/provenance.json",
            "selector_metadata_ref": ".csdlc/evidence/631/install/selector.json",
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
    let (root, artifact_digest) = write_evidence(
        ".csdlc/evidence/631/install/actual-csdlc",
        b"actual artifact bytes",
    );
    fs::write(
        root.join(".csdlc/evidence/631/install/selector-actual.json"),
        br#"{"selected":"actual"}"#,
    )
    .expect("write selector metadata");
    fs::write(
        root.join(".csdlc/evidence/631/install/provenance-actual.json"),
        br#"{"source":"git:def456"}"#,
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
            "artifact_ref": ".csdlc/evidence/631/install/actual-csdlc",
            "source_provenance_ref": ".csdlc/evidence/631/install/provenance-actual.json",
            "selector_metadata_ref": ".csdlc/evidence/631/install/selector-actual.json",
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
    let (root, artifact_digest) = write_evidence(
        ".csdlc/evidence/631/install/forged-provenance-csdlc",
        b"forged provenance artifact bytes",
    );
    fs::write(
        root.join(".csdlc/evidence/631/install/forged-selector.json"),
        br#"{"selected":"csdlc"}"#,
    )
    .expect("write selector metadata");
    let selector_digest = blake3::hash(br#"{"selected":"csdlc"}"#)
        .to_hex()
        .to_string();
    fs::write(
        root.join(".csdlc/evidence/631/install/forged-provenance.json"),
        br#"{"source":"git:real-source"}"#,
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
            "artifact_ref": ".csdlc/evidence/631/install/forged-provenance-csdlc",
            "source_provenance_ref": ".csdlc/evidence/631/install/forged-provenance.json",
            "selector_metadata_ref": ".csdlc/evidence/631/install/forged-selector.json",
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
}
