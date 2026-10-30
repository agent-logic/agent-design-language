use adl_runtime_kernel::*;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::{
    fs,
    path::{Component, Path},
};

const H: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";

fn identity() -> BirthdayIdentityRecord {
    let mut record: BirthdayIdentityRecord = serde_json::from_value(json!({
        "schema":"adl.birthday.identity_record.v2","stable_name":"Aster","identity_root":H,"aliases":[],
        "origin":{"event_id":"origin","provenance_id":"origin","reference":{"id":"origin","path":"evidence/origin.json","sha256":H}},
        "continuity":{"identity_root":H,"head_sha256":H,"reference":{"id":"continuity","path":"evidence/continuity.json","sha256":H}},
        "provenance":[{"id":"origin","path":"evidence/origin.json","sha256":H}],
        "witnesses":[{"id":"witness","path":"evidence/witness.json","sha256":H}],
        "governed_projection":{"id":"projection","path":"evidence/projection.json","sha256":H},
        "projection_receipt":{"schema":"adl.birthday.verified_projection_receipt.v1","subject_id":"Aster","lineage_id":"lineage","generation":1,"signing_key_id":"key","accepted_record_hash":H,"projection_sha256":H,"visible_fields":{},"redacted_fields":[]},
        "record_sha256":""
    })).unwrap();
    record.record_sha256 = birthday_identity::record_digest(&record).unwrap();
    record
}

fn continuity(identity: &BirthdayIdentityRecord) -> BirthdayContinuityRecord {
    let mut record: BirthdayContinuityRecord = serde_json::from_value(json!({
        "schema":"adl.birthday.continuity_record.v1","identity_root":identity.identity_root,
        "identity_record_sha256":identity.record_sha256,"predecessor_head":H,"authority_context_sha256":H,
        "cycles":[{"generation":1,"accepted_through":1,"previous_integrity":H,"integrity":H,"signing_key_id":"key","reference":{"id":"cycle-1","path":"evidence/continuity/cycle-1.json","sha256":H}}],
        "grade":"evidence_backed","continuity_head":H,"record_sha256":""
    })).unwrap();
    record.record_sha256 = continuity_record_digest(&record).unwrap();
    record
}

fn record(id: &str, workflow: &str, effective: u64) -> ObsMemContextRecord {
    let trace = trace_reference();
    let source = NormalizedObsMemRecord {
        id: id.into(),
        run_id: format!("run-{id}"),
        workflow_id: workflow.into(),
        tags: vec!["memory-palace".into(), "memory-palace".into()],
        payload: format!("bounded summary {id}"),
        score: "1.0".into(),
        citations: vec![NormalizedObsMemCitation {
            path: trace.path.clone(),
            hash: trace.sha256.clone(),
        }],
        trace_event_refs: trace_event_refs(),
        temporal_anchor: Some(NormalizedObsMemTemporalAnchor {
            t_created_epoch_ms: u128::from(effective - 10),
            t_observed_epoch_ms: Some(u128::from(effective)),
            t_effective_epoch_ms: Some(u128::from(effective)),
            continuity_id: Some(H.into()),
            event_sequence: Some(1),
        }),
        review_findings: Vec::new(),
        residual_risks: Vec::new(),
        follow_on_refs: Vec::new(),
    };
    adapt_normalized_obsmem_record(source, MemoryVisibility::Public, H, H, &trace).unwrap()
}

fn trace_event_refs() -> Vec<NormalizedObsMemTraceRef> {
    vec![NormalizedObsMemTraceRef {
        event_sequence: 1,
        event_kind: "runtime_event".into(),
        step_id: Some("step-5828".into()),
        delegation_id: None,
    }]
}

fn trace_reference() -> MemoryReference {
    let bytes = serde_jcs::to_vec(&trace_event_refs()).unwrap();
    MemoryReference {
        id: format!("trace:{:x}", Sha256::digest(bytes)),
        path: ".adl/runtime-v3/observability/trace-5828.json".into(),
        sha256: H.into(),
    }
}

fn input(
    records: Vec<ObsMemContextRecord>,
    max: usize,
) -> (
    BirthdayIdentityRecord,
    BirthdayContinuityRecord,
    MemoryPalaceInput,
) {
    let identity = identity();
    let continuity = continuity(&identity);
    let input = MemoryPalaceInput {
        schema: MEMORY_PALACE_INPUT_SCHEMA.into(),
        identity_record_sha256: identity.record_sha256.clone(),
        continuity_record_sha256: continuity.record_sha256.clone(),
        trace_reference: trace_reference(),
        redaction_policy_sha256: H.into(),
        observed_epoch_ms: 2_000,
        stale_after_ms: 1_000,
        max_working_set_items: max,
        records,
    };
    (identity, continuity, input)
}

#[test]
fn canonical_topology_is_deterministic_and_native_semantic_output_is_contained() {
    let fixture: serde_json::Value =
        serde_json::from_str(include_str!("fixtures/memory_palace/matrix.json")).unwrap();
    assert_eq!(fixture["schema"], "adl.memory_palace.fixture_matrix.v1");
    assert!(fixture["negative_cases"].as_array().unwrap().len() >= 20);
    let (identity, continuity, first) = input(
        vec![record("b", "beta", 1500), record("a", "alpha", 1500)],
        8,
    );
    let (_, _, second) = input(
        vec![record("a", "alpha", 1500), record("b", "beta", 1500)],
        8,
    );
    let left = build_memory_palace(&identity, &continuity, &first).unwrap();
    let right = build_memory_palace(&identity, &continuity, &second).unwrap();
    assert_eq!(left, right);
    validate_memory_palace_packet(&left).unwrap();
    if let Ok(relative) = std::env::var("ADL_NATIVE_SEMANTIC_OUTPUT") {
        let path = Path::new(&relative);
        assert!(
            !path.is_absolute()
                && path
                    .components()
                    .all(|part| matches!(part, Component::Normal(_)))
        );
        assert!(relative.starts_with(".csdlc/evidence/5828/native-platform/"));
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
        let output = root.join(path);
        fs::create_dir_all(output.parent().unwrap()).unwrap();
        fs::write(output, serde_jcs::to_vec(&left).unwrap()).unwrap();
    }
}

#[test]
fn bounded_selection_records_overflow_without_loading_it() {
    let (identity, continuity, input) = input(
        vec![record("b", "alpha", 1500), record("a", "alpha", 1500)],
        1,
    );
    let packet = build_memory_palace(&identity, &continuity, &input).unwrap();
    assert_eq!(packet.working_set.len(), 1);
    assert_eq!(packet.overflow.len(), 1);
    assert_eq!(packet.overflow[0].record_id, "b");
    assert_eq!(packet.record_index.len(), 2);
    assert_eq!(packet.record_index[0].record_id, "a");
    assert_eq!(
        packet.record_index[0].status,
        MemoryPalaceRecordStatus::Selected
    );
    assert_eq!(packet.record_index[1].record_id, "b");
    assert_eq!(
        packet.record_index[1].status,
        MemoryPalaceRecordStatus::Excluded
    );
    assert_eq!(
        packet.record_index[1].record_sha256,
        packet.overflow[0].record_sha256
    );
    let mut forged = packet.clone();
    forged.record_index[1].disposition_reason = "selected somewhere else".to_owned();
    forged.packet_sha256.clear();
    forged.packet_sha256 = format!("{:x}", Sha256::digest(serde_jcs::to_vec(&forged).unwrap()));
    assert!(validate_memory_palace_packet(&forged).is_err());
    let mut forged = packet;
    forged.record_index[0].record_sha256 = "c".repeat(64);
    forged.packet_sha256.clear();
    forged.packet_sha256 = format!("{:x}", Sha256::digest(serde_jcs::to_vec(&forged).unwrap()));
    assert!(validate_memory_palace_packet(&forged).is_err());
}

#[test]
fn identity_continuity_and_trace_substitution_fail_closed() {
    let (identity, continuity, input) = input(vec![record("a", "alpha", 1500)], 1);
    for mutate in 0..5 {
        let mut bad = input.clone();
        match mutate {
            0 => bad.identity_record_sha256 = H.into(),
            1 => bad.continuity_record_sha256 = H.into(),
            2 => bad.records[0].identity_root = "b".repeat(64),
            3 => bad.records[0].continuity_head = "b".repeat(64),
            _ => bad.records[0].trace_id = "other".into(),
        }
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
}

#[test]
fn stale_future_and_invalid_temporal_anchors_fail_closed() {
    let (identity, continuity, base) = input(vec![record("a", "alpha", 1500)], 1);
    for mutate in 0..3 {
        let mut bad = base.clone();
        match mutate {
            0 => bad.records[0].temporal_anchor.effective_epoch_ms = 500,
            1 => bad.records[0].temporal_anchor.observed_epoch_ms = 2100,
            _ => bad.records[0].temporal_anchor.event_sequence = 0,
        };
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
}

#[test]
fn citation_path_hash_and_presence_fail_closed() {
    let (identity, continuity, base) = input(vec![record("a", "alpha", 1500)], 1);
    for mutate in 0..5 {
        let mut bad = base.clone();
        match mutate {
            0 => bad.records[0].citations.clear(),
            1 => bad.records[0].citations[0].path = "/Users/operator/private.json".into(),
            2 => bad.records[0].citations[0].path = "evidence/../private.json".into(),
            3 => bad.records[0].citations[0].sha256 = "short".into(),
            _ => bad.records[0].citations[0].sha256 = "b".repeat(64),
        };
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
}

#[test]
fn private_raw_and_unredacted_memory_never_enters_working_set() {
    let (identity, continuity, base) = input(vec![record("a", "alpha", 1500)], 1);
    for visibility in [MemoryVisibility::Private, MemoryVisibility::RawPrivate] {
        let mut bad = base.clone();
        bad.records[0].visibility = visibility;
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
    let mut bad = base;
    bad.records[0].visibility = MemoryVisibility::Redacted;
    assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
}

#[test]
fn secret_like_payloads_and_unknown_json_fields_fail_closed() {
    let (identity, continuity, base) = input(vec![record("a", "alpha", 1500)], 1);
    for payload in [
        "Bearer secret",
        "api_key=secret",
        "/private/state",
        "raw_chat_transcript",
    ] {
        let mut bad = base.clone();
        bad.records[0].payload = payload.into();
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
    let mut value = serde_json::to_value(base).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("hidden_private_state".into(), json!(true));
    assert!(serde_json::from_value::<MemoryPalaceInput>(value).is_err());
}

#[test]
fn duplicate_records_and_citations_are_rejected() {
    let mut a = record("a", "alpha", 1500);
    a.citations.push(a.citations[0].clone());
    let (identity, continuity, bad) = input(vec![a.clone(), a], 8);
    assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
}

#[test]
fn packet_validation_rejects_self_rehashed_privacy_and_topology_forgery() {
    let (identity, continuity, input) = input(vec![record("a", "alpha", 1500)], 1);
    let packet = build_memory_palace(&identity, &continuity, &input).unwrap();
    for mutate in 0..4 {
        let mut forged = packet.clone();
        match mutate {
            0 => forged.working_set[0].visibility = MemoryVisibility::RawPrivate,
            1 => forged.working_set[0].payload = "/home/operator/private".into(),
            2 => forged.working_set[0].room_id = "room:invented".into(),
            _ => forged.working_set[0].citations.clear(),
        }
        forged.packet_sha256.clear();
        forged.packet_sha256 = format!("{:x}", Sha256::digest(serde_jcs::to_vec(&forged).unwrap()));
        assert!(validate_memory_palace_packet(&forged).is_err());
    }
}

#[test]
fn identifiers_secrets_and_normalized_room_collisions_fail_closed() {
    let (identity, continuity, base) = input(vec![record("a", "alpha", 1500)], 8);
    for unsafe_value in [
        "/Users/operator/private",
        "/home/operator/private",
        "gho_secret",
        "sk-secret",
    ] {
        for field in 0..3 {
            let mut bad = base.clone();
            match field {
                0 => bad.records[0].id = unsafe_value.into(),
                1 => bad.records[0].run_id = unsafe_value.into(),
                _ => bad.records[0].workflow_id = unsafe_value.into(),
            }
            assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
        }
        let mut bad = base.clone();
        bad.records[0].payload = unsafe_value.into();
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
    let (_, _, collision_input) = input(
        vec![record("a", "Alpha", 1500), record("b", "alpha", 1500)],
        8,
    );
    let packet = build_memory_palace(&identity, &continuity, &collision_input).unwrap();
    assert_ne!(packet.rooms[0].room_id, packet.rooms[1].room_id);
    validate_memory_palace_packet(&packet).unwrap();
}

#[test]
fn normalized_obsmem_adapter_binds_trace_events_and_citation() {
    let trace = trace_reference();
    let source = NormalizedObsMemRecord {
        id: "record-a".into(),
        run_id: "run-a".into(),
        workflow_id: "flow-a".into(),
        tags: vec!["z".into(), "a".into(), "a".into()],
        payload: "bounded".into(),
        score: "1.0".into(),
        citations: vec![NormalizedObsMemCitation {
            path: trace.path.clone(),
            hash: trace.sha256.clone(),
        }],
        trace_event_refs: trace_event_refs(),
        temporal_anchor: Some(NormalizedObsMemTemporalAnchor {
            t_created_epoch_ms: 1,
            t_observed_epoch_ms: Some(2),
            t_effective_epoch_ms: Some(2),
            continuity_id: Some(H.into()),
            event_sequence: Some(1),
        }),
        review_findings: vec![],
        residual_risks: vec![],
        follow_on_refs: vec![],
    };
    assert!(
        adapt_normalized_obsmem_record(source.clone(), MemoryVisibility::Public, H, H, &trace)
            .is_ok()
    );
    let mut wrong_trace = trace.clone();
    wrong_trace.id = "trace:invented".into();
    assert!(adapt_normalized_obsmem_record(
        source.clone(),
        MemoryVisibility::Public,
        H,
        H,
        &wrong_trace
    )
    .is_err());
    let mut wrong_citation = source;
    wrong_citation.citations[0].hash = "b".repeat(64);
    assert!(
        adapt_normalized_obsmem_record(wrong_citation, MemoryVisibility::Public, H, H, &trace)
            .is_err()
    );
    let mut wrong_continuity = NormalizedObsMemRecord {
        id: "record-b".into(),
        run_id: "run-b".into(),
        workflow_id: "flow-b".into(),
        tags: vec![],
        payload: "bounded".into(),
        score: "1.0".into(),
        citations: vec![NormalizedObsMemCitation {
            path: trace.path.clone(),
            hash: trace.sha256.clone(),
        }],
        trace_event_refs: trace_event_refs(),
        temporal_anchor: Some(NormalizedObsMemTemporalAnchor {
            t_created_epoch_ms: 1,
            t_observed_epoch_ms: Some(2),
            t_effective_epoch_ms: Some(2),
            continuity_id: Some("b".repeat(64)),
            event_sequence: Some(1),
        }),
        review_findings: vec![],
        residual_risks: vec![],
        follow_on_refs: vec![],
    };
    assert!(adapt_normalized_obsmem_record(
        wrong_continuity.clone(),
        MemoryVisibility::Public,
        H,
        H,
        &trace
    )
    .is_err());
    wrong_continuity
        .temporal_anchor
        .as_mut()
        .unwrap()
        .continuity_id = None;
    assert!(adapt_normalized_obsmem_record(
        wrong_continuity,
        MemoryVisibility::Public,
        H,
        H,
        &trace
    )
    .is_err());
}

#[test]
fn reference_identifier_privacy_and_canonical_packet_bounds_fail_closed() {
    let (identity, continuity, base) = input(
        vec![record("a", "alpha", 1500), record("b", "beta", 1500)],
        1,
    );
    for unsafe_id in ["/Users/operator/private", "gho_secret", "sk-secret"] {
        let mut bad = base.clone();
        bad.trace_reference.id = unsafe_id.into();
        bad.records[0].trace_id = unsafe_id.into();
        bad.records[0].citations[0].id = unsafe_id.into();
        assert!(build_memory_palace(&identity, &continuity, &bad).is_err());
    }
    let packet = build_memory_palace(&identity, &continuity, &base).unwrap();
    for mutate in 0..3 {
        let mut forged = packet.clone();
        match mutate {
            0 => forged.rooms.reverse(),
            1 => forged.overflow[0].reason = "bounded_after_64".into(),
            _ => forged.max_working_set_items = 65,
        }
        forged.packet_sha256.clear();
        forged.packet_sha256 = format!("{:x}", Sha256::digest(serde_jcs::to_vec(&forged).unwrap()));
        assert!(validate_memory_palace_packet(&forged).is_err());
    }
}
