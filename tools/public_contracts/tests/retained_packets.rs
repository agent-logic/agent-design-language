// PVF: local deterministic structural contract, bounded disk fixtures, no runtime
// or lifecycle authority and no standalone release-gate acceptance.
use adl_public_contracts::{
    retained_packets::{check, requirements},
    root,
};
use std::{fs, process::Command};
fn exercise(kind: &str, directory: &str) {
    let packet = root()
        .join("docs/milestones/v0.91.3/review")
        .join(directory);
    check(kind, &packet).unwrap();
    let temp = tempfile::tempdir().unwrap();
    let required = requirements(kind).unwrap();
    for (file, snippets) in required {
        fs::write(temp.path().join(file), snippets.join("\n")).unwrap();
    }
    check(kind, temp.path()).unwrap();
    // Every required field is independently removed, not only the old single
    // negative example. Distinct marker lines prevent substring ambiguity.
    for (file, snippets) in required {
        let path = temp.path().join(file);
        let original = fs::read(&path).unwrap();
        fs::remove_file(&path).unwrap();
        assert!(check(kind, temp.path()).is_err());
        fs::write(&path, &original).unwrap();
        for (i, _) in snippets.iter().enumerate() {
            let changed = snippets
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, s)| *s)
                .collect::<Vec<_>>()
                .join("\n");
            fs::write(&path, changed).unwrap();
            assert!(check(kind, temp.path()).is_err(), "{file} marker {i}");
        }
        fs::write(&path, &original).unwrap();
    }
    assert!(check(kind, &temp.path().join("absent")).is_err());
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg(kind)
        .arg(&packet)
        .env("PATH", "")
        .current_dir(temp.path())
        .output()
        .unwrap();
    assert!(output.status.success());
    let invalid = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .arg(kind)
        .env("PATH", "")
        .output()
        .unwrap();
    assert_eq!(
        invalid.status.code(),
        Some(if kind == "csdlc-demo-proof-contract" {
            2
        } else {
            1
        })
    );
}
#[test]
fn evidence_bundle() {
    exercise("evidence-bundle", "evidence_bundle");
}
#[test]
fn transition_dag() {
    exercise("transition-dag", "transition_dag");
}
#[test]
fn csdlc_demo_proof_contract() {
    exercise("csdlc-demo-proof-contract", "csdlc_demo_proof_contract");
    let root = root().join("docs/milestones/v0.91.3/review/csdlc_demo_proof_contract");
    assert!(
        fs::read_to_string(root.join("C_SDLC_DEMO_PROOF_CONTRACT_v0.91.3.md"))
            .unwrap()
            .contains("Allowed top-level result classes")
    );
    assert!(
        fs::read_to_string(root.join("C_SDLC_DEMO_PROOF_PACKET_TEMPLATE_v0.91.3.md"))
            .unwrap()
            .contains("evidence type: `measured` | `estimated` | `not run`")
    );
}
