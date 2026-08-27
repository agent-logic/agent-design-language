use std::fs;
use std::path::{Path, PathBuf};

use ::adl::long_lived_agent::{self, RunOptions};
use ::adl::memory_palace::{
    build_context_from_agent_memory, context_packet_bytes, project_runtime_packet,
    MemoryPalaceAgentConfig, MemoryPalaceInput, MEMORY_PALACE_CONTEXT_REF,
    MEMORY_PALACE_CONTEXT_SCHEMA,
};
use adl_runtime::memory_palace::{
    RuntimeMemoryPalaceCheckpoint, RuntimeMemoryPalaceJournalEntry, RuntimeMemoryPalaceLatest,
};
use adl_runtime_kernel::{
    birthday_identity, build_memory_palace, build_memory_palace_context_cache,
    continuity_record_digest, BirthdayContinuityRecord, BirthdayIdentityRecord,
    MemoryPalaceInput as KernelMemoryPalaceInput, MemoryReference,
    MemoryTemporalAnchor as KernelMemoryTemporalAnchor, MemoryVisibility, ObsMemContextRecord,
};
use serde::Serialize;
use serde_json::json;
use sha2::{Digest, Sha256};

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
        input_ref: "memory-palace/latest.json".to_string(),
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

fn sha256_jcs<T: Serialize>(value: &T) -> String {
    format!("{:x}", Sha256::digest(serde_jcs::to_vec(value).unwrap()))
}

fn write_jcs<T: Serialize>(path: &Path, value: &T) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).unwrap();
    }
    fs::write(path, serde_jcs::to_vec(value).unwrap()).unwrap();
}

fn write_runtime_memory_palace_service(
    spec_dir: &Path,
    packet: &adl_runtime_kernel::MemoryPalaceContextPacket,
) {
    let generation = 1;
    let service_root = spec_dir.join("memory-palace");
    let packet_ref = MemoryReference {
        id: "memory-palace-packet:00000000000000000001".to_owned(),
        path: "memory-palace/generations/00000000000000000001/packet.json".to_owned(),
        sha256: packet.packet_sha256.clone(),
    };
    let cache = build_memory_palace_context_cache(generation, 1, packet_ref.clone(), packet)
        .expect("build Runtime Memory Palace context cache");
    let mut checkpoint = RuntimeMemoryPalaceCheckpoint {
        schema: "adl.runtime.memory_palace.checkpoint.v1".to_owned(),
        memory_palace_generation: generation,
        birthday_continuity_generation: 1,
        predecessor_packet_sha256: None,
        identity_root: packet.identity_root.clone(),
        identity_record_sha256: packet.identity_record_sha256.clone(),
        continuity_head: packet.continuity_head.clone(),
        continuity_record_sha256: packet.continuity_record_sha256.clone(),
        topology_sha256: sha256_jcs(&packet.rooms),
        working_set_sha256: sha256_jcs(&packet.working_set),
        source_packet_ref: packet_ref,
        canonical_input_sha256: packet.canonical_input_sha256.clone(),
        packet_sha256: packet.packet_sha256.clone(),
        context_cache_sha256: cache.cache_sha256.clone(),
        trace_reference: packet.trace_reference.clone(),
        redaction_policy_sha256: packet.redaction_policy_sha256.clone(),
        checkpoint_sha256: String::new(),
    };
    checkpoint.checkpoint_sha256 = sha256_jcs(&checkpoint);
    let mut journal = RuntimeMemoryPalaceJournalEntry {
        schema: "adl.runtime.memory_palace.journal_entry.v1".to_owned(),
        generation,
        predecessor_packet_sha256: None,
        packet_sha256: packet.packet_sha256.clone(),
        context_cache_sha256: cache.cache_sha256.clone(),
        checkpoint_sha256: checkpoint.checkpoint_sha256.clone(),
        entry_sha256: String::new(),
    };
    journal.entry_sha256 = sha256_jcs(&journal);
    let latest = RuntimeMemoryPalaceLatest {
        schema: "adl.runtime.memory_palace.latest.v1".to_owned(),
        generation,
        packet_sha256: packet.packet_sha256.clone(),
        checkpoint_sha256: checkpoint.checkpoint_sha256.clone(),
    };
    let generation_dir = service_root.join("generations/00000000000000000001");
    write_jcs(&generation_dir.join("packet.json"), packet);
    write_jcs(&generation_dir.join("context-cache.json"), &cache);
    write_jcs(&generation_dir.join("checkpoint.json"), &checkpoint);
    write_jcs(
        &service_root.join("journal/00000000000000000001.json"),
        &journal,
    );
    write_jcs(&service_root.join("latest.json"), &latest);
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
fn memory_palace_adapter_rejects_direct_packet_file_without_runtime_service() {
    let root = unique_test_temp_dir("memory-palace-direct-packet-rejected");
    let legacy = memory_palace_fixture();
    let runtime_packet = runtime_packet_from_fixture(&legacy, 1);
    fs::write(
        root.join("memory_palace_packet.json"),
        serde_jcs::to_vec(&runtime_packet).unwrap(),
    )
    .expect("write self-consistent Runtime Memory Palace packet fixture");
    let memory = json!({
        "memory_palace": {
            "input_ref": "memory_palace_packet.json",
            "max_working_set_items": 1,
            "stale_after_ms": 1000,
            "required_continuity_id": h(),
            "observed_epoch_ms": 4102444800500u64
        }
    });

    let error = build_context_from_agent_memory(&memory, &root, "cycle-000001", 4102444800500)
        .expect_err("direct packet file must not be accepted as Runtime service authority");
    assert!(error
        .to_string()
        .contains("input_ref must point to Runtime service latest.json"));
}

#[test]
fn long_lived_agent_cycle_consumes_memory_palace_context_ref() {
    let root = unique_test_temp_dir("memory-palace-agent");
    let legacy = memory_palace_fixture();
    let runtime_packet = runtime_packet_from_fixture(&legacy, 1);
    write_runtime_memory_palace_service(&root, &runtime_packet);
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
    input_ref: memory-palace/latest.json
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

#[test]
fn v092_docs_name_memory_palace_production_authority_without_broad_completion_claim() {
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("adl crate has repository parent");
    let feature_list = fs::read_to_string(repo_root.join("docs/planning/ADL_FEATURE_LIST.md"))
        .expect("read ADL feature list");
    let feature_list_flat = feature_list
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(feature_list.contains("Memory Palace navigable context topology"));
    assert!(feature_list.contains("Implemented Runtime-kernel production-authority slice"));
    assert!(feature_list_flat
        .contains("`adl-runtime-kernel::memory_palace` as the production authority"));
    assert!(feature_list.contains("`adl-runtime::memory_palace`"));
    assert!(feature_list_flat.contains("owns durable retained checkpoint/latest/journal state"));
    assert!(feature_list_flat
        .contains("`adl` consumes that authority through a compatibility projection"));

    let proof_coverage =
        fs::read_to_string(repo_root.join("docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md"))
            .expect("read v0.92 feature proof coverage");
    let proof_coverage_flat = proof_coverage
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    assert!(proof_coverage.contains("Memory Palace production authority"));
    assert!(proof_coverage_flat.contains("issue `#450`, PR `#458`"));
    assert!(proof_coverage.contains("C-SDLC generation 24 digest"));

    let milestone_readme = fs::read_to_string(repo_root.join("docs/milestones/v0.92/README.md"))
        .expect("read v0.92 README");
    assert!(!milestone_readme.contains("- Full memory palace implementation."));
    assert!(milestone_readme.contains("Memory Palace context"));

    let feature_doc = fs::read_to_string(
        repo_root.join("docs/milestones/v0.92/features/MEMORY_PALACE_CONTEXT_TOPOLOGY_v0.92.md"),
    )
    .expect("read Memory Palace feature doc");
    let feature_doc_flat = feature_doc.split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(feature_doc_flat
        .contains("WP-11's Runtime-kernel production-authority slice is implemented by `#450`"));
    assert!(feature_doc.contains("`adl-runtime-kernel::memory_palace`"));
    assert!(feature_doc_flat.contains("the production authority"));
    assert!(feature_doc_flat
        .contains("broader distributed or unbounded Memory Palace product remains later work"));
}
