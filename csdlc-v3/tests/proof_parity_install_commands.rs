//! PVF: deterministic small local CLI contract proof; required, no network.
//! The former construction parity/soak tests executed mutations in primary.
//! Operational proof and install positives now live in semantic_local_proof and
//! semantic_install. Only the historical shadow/soak request routes remain here.
use serde_json::{json, Value};
use std::{fs, path::PathBuf, process::Command};
#[test]
fn historical_proof_requests_cannot_retain_primary_checkout_mutation_authority() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target/legacy-proof-denial")
        .join(std::process::id().to_string());
    fs::create_dir_all(&dir).unwrap();
    let request = dir.join("request.json");
    fs::write(&request,serde_json::to_vec(&json!({"issue":505,"repository":"agent-logic/agent-design-language","cutover_issue":505,"evidence_root":dir,"operator_approval":"historical approval is not live ownership","proof":null,"shadow":null,"soak":null,"install":null})).unwrap()).unwrap();
    for route in ["shadow", "soak"] {
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .current_dir(&dir)
            .args([route, "--request"])
            .arg(&request)
            .output()
            .unwrap();
        assert!(!output.status.success());
        let report: Value =
            serde_json::from_slice(&output.stdout).expect("one JSON report on stdout");
        assert_eq!(report["performed_mutation"], false);
        assert_eq!(report["findings"][0]["code"], "historical_route_disabled");
        assert!(!dir.join(".csdlc").exists());
        assert!(String::from_utf8_lossy(&output.stderr).contains("see structured stdout findings"));
    }
    fs::remove_dir_all(dir).unwrap();
}
