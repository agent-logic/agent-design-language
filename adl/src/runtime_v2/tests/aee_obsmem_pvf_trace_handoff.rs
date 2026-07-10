use super::*;

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use crate::runtime_v2::tests::common::unique_temp_path;
#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
use std::fs;

#[test]
fn runtime_v2_aee_obsmem_pvf_trace_handoff_contract_is_stable() {
    let artifacts = runtime_v2_aee_obsmem_pvf_trace_handoff_contract()
        .expect("AEE ObsMem PVF trace handoff artifacts");
    artifacts
        .validate()
        .expect("valid AEE ObsMem PVF trace handoff artifacts");

    assert_eq!(
        artifacts.packet.schema_version,
        RUNTIME_V2_AEE_OBSMEM_PVF_HANDOFF_SCHEMA
    );
    assert_eq!(artifacts.packet.issue, 4697);
    assert_eq!(artifacts.packet.pvf_lane, "runtime");
    assert!(artifacts
        .packet
        .retained_evidence_refs
        .iter()
        .any(|reference| reference == RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE));
    assert!(artifacts
        .packet
        .pvf_trace_handoff_refs
        .iter()
        .all(|reference| reference.starts_with("pvf://")));
}

#[test]
fn runtime_v2_aee_obsmem_pvf_trace_handoff_rejects_scope_drift() {
    let mut artifacts = runtime_v2_aee_obsmem_pvf_trace_handoff_contract()
        .expect("AEE ObsMem PVF trace handoff artifacts");
    artifacts.packet.issue = 4696;
    assert!(artifacts
        .validate()
        .expect_err("wrong issue should fail")
        .to_string()
        .contains("#4697"));

    let mut artifacts = runtime_v2_aee_obsmem_pvf_trace_handoff_contract()
        .expect("AEE ObsMem PVF trace handoff artifacts");
    artifacts
        .packet
        .retained_evidence_refs
        .retain(|reference| reference != RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL);
    assert!(artifacts
        .validate()
        .expect_err("missing retrieval evidence should fail")
        .to_string()
        .contains("obsmem_retrieval_result"));
}

#[cfg(any(feature = "slow-proof-tests", feature = "slow-proof-runtime"))]
#[test]
fn runtime_v2_aee_obsmem_pvf_trace_handoff_writes_retained_evidence() {
    let temp_root = unique_temp_path("aee-obsmem-pvf-handoff");
    let artifacts = runtime_v2_aee_obsmem_pvf_trace_handoff_contract()
        .expect("AEE ObsMem PVF trace handoff artifacts");

    artifacts
        .write_to_root(&temp_root)
        .expect("write AEE ObsMem PVF trace handoff artifacts");

    for retained_ref in &artifacts.packet.retained_evidence_refs {
        assert!(
            temp_root.join(retained_ref).is_file(),
            "missing retained evidence ref: {retained_ref}"
        );
    }

    let write_text =
        fs::read_to_string(temp_root.join(RUNTIME_V2_AEE_OBSMEM_PVF_MEMORY_WRITE))
            .expect("memory write text");
    assert!(write_text.contains("issue:4697"));
    assert!(write_text.contains("pvf:runtime"));

    let retrieval_text =
        fs::read_to_string(temp_root.join(RUNTIME_V2_AEE_OBSMEM_PVF_RETRIEVAL))
            .expect("retrieval text");
    assert!(retrieval_text.contains("AEE observed the governed Runtime v2 action"));
    assert!(!retrieval_text.contains(temp_root.to_string_lossy().as_ref()));

    fs::remove_dir_all(temp_root).ok();
}
