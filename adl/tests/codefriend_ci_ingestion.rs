//! PVF runtime, deterministic installed-route contract, local Git/CPU/disk only.
//! Required #880 gate; hosted installation/upload proof remains a separate CI job.
#[test]
fn ci_production_route_conformance() {
    let temp = tempfile::tempdir().unwrap();
    let script =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tools/codefriend/ci_smoke.py");
    let result = std::process::Command::new("python3")
        .arg(script)
        .arg("--binary")
        .arg(env!("CARGO_BIN_EXE_adl"))
        .arg("--candidate-revision")
        .arg("0123456789012345678901234567890123456789")
        .arg("--output")
        .arg(temp.path().join("proof"))
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: serde_json::Value =
        serde_json::from_slice(&std::fs::read(temp.path().join("proof/proof.json")).unwrap())
            .unwrap();
    assert_eq!(report["cases_passed"], 10);
    let packet =
        adl::codefriend::ingestion::AdmissionInput::read(&temp.path().join("proof/packet-1.json"))
            .unwrap();
    let mut receipt: adl::codefriend::ingestion::ci::Receipt =
        serde_json::from_slice(&std::fs::read(temp.path().join("proof/receipt-1.json")).unwrap())
            .unwrap();
    receipt.validate(packet.packet()).unwrap();
    receipt.source_revision = "f".repeat(40);
    assert!(receipt.validate(packet.packet()).is_err());
    receipt.source_revision = packet.packet().revision.clone();
    receipt.delivery = "delivered".into();
    assert!(receipt.validate(packet.packet()).is_err());
}
