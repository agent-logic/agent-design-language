//! PVF: deterministic contract/integration proof, offline Python stdlib and local
//! files only; bounded CPU/memory, no providers/network/installed-owner effects.
//! Required RD02 preparation gate; does not prove extraction or policy approval.
use std::path::PathBuf;
use std::process::Command;

fn validator() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../docs/milestones/v0.93.1/repository-decomposition/rd02-1182/validate_contract.py")
}

#[test]
fn complete_pending_policy_contract_and_adversarial_inputs() {
    let output = Command::new("python3")
        .arg(validator())
        .arg("--self-test")
        .output()
        .expect("Python 3 is required for the offline RD02 contract gate");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(
        output.stderr.is_empty(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["ok"], true);
    assert_eq!(result["ownership_paths"], 36899);
    assert_eq!(result["opening_paths"], 42);
    assert_eq!(result["repositories"], 7);
    assert_eq!(result["artifacts"], 8);
    assert_eq!(result["policy_acceptance"], "pending");
    let scenarios = result["adversarial_tests"]["scenarios"].as_array().unwrap();
    assert!(
        scenarios.len() >= 31,
        "nonzero complete adversarial denominator"
    );
    assert_eq!(
        result["adversarial_tests"]["passed"].as_u64().unwrap(),
        scenarios.len() as u64
    );
}

#[test]
fn policy_acceptance_is_not_inferred_from_technical_success() {
    let output = Command::new("python3")
        .arg(validator())
        .arg("--require-policy-acceptance")
        .output()
        .unwrap();
    assert_eq!(output.status.code(), Some(1));
    assert!(output.stderr.is_empty());
    let result: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(result["ok"], false);
    assert!(result["error"]
        .as_str()
        .unwrap()
        .starts_with("policy_acceptance_pending:"));
}
