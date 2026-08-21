use std::fs;
use std::path::{Path, PathBuf};

use ::adl::long_lived_agent::{self, RunOptions};
use ::adl::memory_palace::{
    context_packet_bytes, project_runtime_packet, MemoryPalaceAgentConfig, MemoryPalaceInput,
    MEMORY_PALACE_CONTEXT_REF, MEMORY_PALACE_CONTEXT_SCHEMA,
};
use adl_runtime_kernel::{
    birthday_identity, build_memory_palace, continuity_record_digest, BirthdayContinuityRecord,
    BirthdayIdentityRecord, MemoryPalaceInput as KernelMemoryPalaceInput, MemoryReference,
    MemoryTemporalAnchor as KernelMemoryTemporalAnchor, MemoryVisibility, ObsMemContextRecord,
};
use serde_json::json;

mod helpers;
use helpers::unique_test_temp_dir;

fn fixture_path(relative: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join(relative)
}

fn memory_palace_fixture() -> MemoryPalaceInput {
    let fixture = fixture_path("tests/fixtures/memory_palace/long_running_context.json");
    serde_json::from_slice(&fs::read(fixture).expect("read Memory Palace fixture"))
        .expect("parse Memory Palace fixture")
}

fn memory_palace_config(max_working_set_items: usize) -> MemoryPalaceAgentConfig {
    MemoryPalaceAgentConfig {
        input_ref: "memory_palace_packet.json".to_string(),
        max_working_set_items,
        stale_after_ms: 1000,
        required_continuity_id: Some(h().to_string()),
        observed_epoch_ms: Some(4102444800500),
    }
}

fn h() -> &'static str {
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
}

fn identity() -> BirthdayIdentityRecord {
    let mut record: BirthdayIdentityRecord = serde_json::from_value(json!({
        "schema":"adl.birthday.identity_record.v2","stable_name":"Aster","identity_root":h(),"aliases":[],
        "origin":{"event_id":"origin","provenance_id":"origin","reference":{"id":"origin","path":"evidence/origin.json","sha256":h()}},
        "continuity":{"identity_root":h(),"head_sha256":h(),"reference":{"id":"continuity","path":"evidence/continuity.json","sha256":h()}},
        "provenance":[{"id":"origin","path":"evidence/origin.json","sha256":h()}],
        "witnesses":[{"id":"witness","path":"evidence/witness.json","sha256":h()}],
        "governed_projection":{"id":"projection","path":"evidence/projection.json","sha256":h()},
        "projection_receipt":{"schema":"adl.birthday.verified_projection_receipt.v1","subject_id":"Aster","lineage_id":"lineage","generation":1,"signing_key_id":"key","accepted_record_hash":h(),"projection_sha256":h(),"visible_fields":{},"redacted_fields":[]},
        "record_sha256":""
    }))
    .unwrap();
    record.record_sha256 = birthday_identity::record_digest(&record).unwrap();
    record
}

fn continuity(identity: &BirthdayIdentityRecord) -> BirthdayContinuityRecord {
    let mut record: BirthdayContinuityRecord = serde_json::from_value(json!({
        "schema":"adl.birthday.continuity_record.v1","identity_root":identity.identity_root,
        "identity_record_sha256":identity.record_sha256,"predecessor_head":h(),"authority_context_sha256":h(),
        "cycles":[{"generation":1,"accepted_through":1,"previous_integrity":h(),"integrity":h(),"signing_key_id":"key","reference":{"id":"cycle-1","path":"evidence/continuity/cycle-1.json","sha256":h()}}],
        "grade":"evidence_backed","continuity_head":h(),"record_sha256":""
    }))
    .unwrap();
    record.record_sha256 = continuity_record_digest(&record).unwrap();
    record
}

fn runtime_packet_from_fixture(
    legacy: &MemoryPalaceInput,
    max_working_set_items: usize,
) -> adl_runtime_kernel::MemoryPalaceContextPacket {
    let identity = identity();
    let continuity = continuity(&identity);
    let trace_reference = MemoryReference {
        id: "trace:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
        path: ".adl/runtime-v3/observability/trace.json".to_owned(),
        sha256: h().to_owned(),
    };
    let records = legacy
        .records
        .iter()
        .map(|record| {
            let anchor = record.temporal_anchor.as_ref().expect("fixture anchor");
            let mut citations = record
                .citations
                .iter()
                .map(|citation| MemoryReference {
                    id: format!("citation:{}", citation.path.replace('/', "-")),
                    path: citation.path.clone(),
                    sha256: citation.hash.strip_prefix("sha256:").unwrap().to_owned(),
                })
                .collect::<Vec<_>>();
            citations.push(trace_reference.clone());
            ObsMemContextRecord {
                id: record.id.clone(),
                run_id: record.run_id.clone(),
                workflow_id: record.workflow_id.clone(),
                payload: record.payload.clone(),
                visibility: MemoryVisibility::Public,
                identity_root: identity.identity_root.clone(),
                continuity_head: continuity.continuity_head.clone(),
                trace_id: trace_reference.id.clone(),
                citations,
                temporal_anchor: KernelMemoryTemporalAnchor {
                    created_epoch_ms: anchor.t_created_epoch_ms as u64,
                    observed_epoch_ms: anchor.t_observed_epoch_ms.unwrap() as u64,
                    effective_epoch_ms: anchor.t_effective_epoch_ms.unwrap() as u64,
                    continuity_head: continuity.continuity_head.clone(),
                    event_sequence: anchor.event_sequence.unwrap() as u64,
                },
            }
        })
        .collect();
    build_memory_palace(
        &identity,
        &continuity,
        &KernelMemoryPalaceInput {
            schema: adl_runtime_kernel::MEMORY_PALACE_INPUT_SCHEMA.to_owned(),
            identity_record_sha256: identity.record_sha256.clone(),
            continuity_record_sha256: continuity.record_sha256.clone(),
            trace_reference,
            redaction_policy_sha256: h().to_owned(),
            observed_epoch_ms: 4102444800500,
            stale_after_ms: 1000,
            max_working_set_items,
            records,
        },
    )
    .unwrap()
}

#[test]
fn memory_palace_fixture_builds_deterministic_obs_mem_handoff() {
    let input = memory_palace_fixture();
    let config = memory_palace_config(1);
    let runtime = runtime_packet_from_fixture(&input, 1);

    let first = project_runtime_packet("cycle-000001", &config, &runtime, 4102444800500)
        .expect("build first Memory Palace packet");
    let second = project_runtime_packet("cycle-000001", &config, &runtime, 4102444800500)
        .expect("build second Memory Palace packet");

    assert_eq!(
        context_packet_bytes(&first).unwrap(),
        context_packet_bytes(&second).unwrap()
    );
    assert_eq!(first.schema, MEMORY_PALACE_CONTEXT_SCHEMA);
    assert_eq!(first.topology.rooms.len(), 1);
    assert_eq!(first.topology.anchors.len(), 2);
    assert_eq!(first.working_set.selected.len(), 1);
    assert_eq!(first.working_set.excluded.len(), 1);
    assert_eq!(first.working_set.selected[0].record_id, "context-a");
    assert_eq!(
        first.working_set.selected[0]
            .temporal_anchor
            .continuity_id
            .as_deref(),
        Some(h())
    );
    assert!(first.working_set.selected[0]
        .provenance
        .iter()
        .any(|citation| citation.path == "runs/run-context-a/trace.json"));
    assert_eq!(first.working_set.excluded[0].record_id, "context-b");
    assert!(first
        .stale_context_report
        .dispositions
        .iter()
        .any(|disposition| disposition.status == "selected"));
    assert!(
        serde_json::from_value::<adl_runtime_kernel::MemoryPalaceContextPacket>(
            serde_json::to_value(input).unwrap()
        )
        .is_err()
    );
}

#[test]
fn long_lived_agent_cycle_consumes_memory_palace_context_ref() {
    let root = unique_test_temp_dir("memory-palace-agent");
    let legacy = memory_palace_fixture();
    let runtime_packet = runtime_packet_from_fixture(&legacy, 1);
    fs::write(
        root.join("memory_palace_packet.json"),
        serde_jcs::to_vec(&runtime_packet).unwrap(),
    )
    .expect("write Runtime Memory Palace packet fixture");
    let spec = root.join("agent.yaml");
    fs::write(
        &spec,
        r#"schema: adl.long_lived_agent_spec.v1
agent_instance_id: memory-palace-agent
display_name: Memory Palace Agent
state_root: state
workflow:
  kind: demo_adapter
  name: memory_palace_context_probe
heartbeat:
  interval_secs: 1
  max_cycles: 1
  stale_lease_after_secs: 60
safety:
  allow_network: false
  allow_broker: false
  allow_filesystem_writes_outside_state_root: false
  allow_real_world_side_effects: false
  require_public_artifact_sanitization: true
  financial_advice: false
  max_cycle_runtime_secs: 120
  max_consecutive_failures: 2
memory:
  namespace: smoke/memory-palace
  write_policy: append_only
  memory_palace:
    input_ref: memory_palace_packet.json
    max_working_set_items: 1
    stale_after_ms: 1000
    required_continuity_id: aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
    observed_epoch_ms: 4102444800500
"#,
    )
    .expect("write agent spec");

    let status = long_lived_agent::run(
        &spec,
        RunOptions {
            max_cycles: 1,
            interval_secs: None,
            no_sleep: true,
            recover_stale_lease: false,
        },
    )
    .expect("run one long-lived-agent cycle");

    assert_eq!(status.last_cycle_id.as_deref(), Some("cycle-000001"));
    assert_eq!(status.last_cycle_status.as_deref(), Some("success"));
    let cycle_dir = root.join("state/cycles/cycle-000001");
    let decision_request: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join("decision_request.json")).unwrap())
            .expect("parse decision request");
    assert_eq!(
        decision_request["memory_refs"],
        serde_json::json!([MEMORY_PALACE_CONTEXT_REF])
    );

    let packet: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join(MEMORY_PALACE_CONTEXT_REF)).unwrap())
            .expect("parse Memory Palace packet");
    assert_eq!(packet["schema"], MEMORY_PALACE_CONTEXT_SCHEMA);
    assert_eq!(
        packet["working_set"]["selected"][0]["record_id"],
        "context-a"
    );
    assert_eq!(
        packet["working_set"]["selected"][0]["temporal_anchor"]["continuity_id"],
        h()
    );

    let manifest: serde_json::Value =
        serde_json::from_slice(&fs::read(cycle_dir.join("cycle_manifest.json")).unwrap())
            .expect("parse cycle manifest");
    assert_eq!(
        manifest["artifacts"]["memory_palace_context"],
        MEMORY_PALACE_CONTEXT_REF
    );
}
