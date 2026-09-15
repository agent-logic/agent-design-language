//! PVF: deterministic installed CLI retirement guard for the former direct
//! proof writer. Current proof and install identity/confinement behavior is
//! exercised through semantic_local_proof and semantic_install.
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod fixture;

use serde_json::Value;

#[test]
fn installed_legacy_proof_writer_is_retired_without_effects() {
    let mut fixture = fixture::Fixture::new("retired-proof-writer");
    let root = fixture.root.clone();
    let request = fixture.write_json(
        "legacy-proof-request.json",
        &serde_json::json!({"issue":870,"proof":{"executes":true}}),
    );
    let before = fixture::inventory(&root);
    let output = fixture.run(&root, &["proof", "--request", request.to_str().unwrap()]);
    assert!(!output.status.success(), "{output:?}");
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(report["envelope"]["reason_code"], "legacy_writer_retired");
    assert_eq!(report["envelope"]["effects"]["outcome"], "none");
    assert_eq!(report["performed_mutation"], false);
    assert_eq!(before, fixture::inventory(&root));
    assert_eq!(fixture.remote_effects(), 0);
}
