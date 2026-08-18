use std::collections::{BTreeMap, BTreeSet};

use adl_runtime::resident_shepherd_continuity::{
    load_population, persist_population_atomically, validate_habitability,
    ExistingAgentLifecycleState, ResidentAgentCapsule, ResidentContinuityController,
    ResidentHabitabilityReceipt, ResidentModelBinding, RuntimeVolumeBinding,
    SpotInterruptionNotice, RESIDENT_POPULATION_SCHEMA, SPOT_NOTICE_SOURCE,
};
use tempfile::TempDir;

fn sha(seed: &str) -> String {
    use sha2::{Digest, Sha256};
    hex::encode(Sha256::digest(seed.as_bytes()))
}

fn agent(id: &str, model: &str, quantization: &str) -> ResidentAgentCapsule {
    ResidentAgentCapsule {
        agent_id: id.to_string(),
        lifecycle_state: ExistingAgentLifecycleState::Active,
        checkpoint_ref: format!("agents/{id}/continuity_checkpoint.json"),
        sequence: 41,
        predecessor_sha256: Some(sha(&format!("{id}-previous"))),
        state_sha256: sha(&format!("{id}-state")),
        completed_task_sha256: sha(&format!("{id}-completed")),
        next_task_sha256: sha(&format!("{id}-next")),
        busy: false,
        serialized_bytes: 4096,
        model: ResidentModelBinding {
            provider: "ollama".to_string(),
            model: model.to_string(),
            quantization: quantization.to_string(),
            artifact_sha256: sha(&format!("{model}-artifact")),
            configuration_sha256: sha(&format!("{model}-configuration")),
            authoritative_for_continuity: true,
        },
    }
}

fn agents() -> Vec<ResidentAgentCapsule> {
    vec![
        agent("shepherd-primary", "llama3.1:8b", "Q4_K_M"),
        agent("shepherd-structured", "qwen3:8b", "Q4_K_M"),
        agent("shepherd-utility", "phi4-mini", "Q4_K_M"),
    ]
}

fn notice() -> SpotInterruptionNotice {
    SpotInterruptionNotice {
        source: SPOT_NOTICE_SOURCE.to_string(),
        action: "terminate".to_string(),
        observed_unix_seconds: 1_000,
        deadline_unix_seconds: 1_120,
    }
}

fn volume(root: &TempDir) -> RuntimeVolumeBinding {
    RuntimeVolumeBinding {
        runtime_mount: root.path().join("runtime-volume"),
        build_cache_mount: root.path().join("build-cache"),
        volume_id_sha256: sha("runtime-volume-id"),
        exclusive_attachment: true,
    }
}

fn expected_ids() -> BTreeSet<String> {
    agents().into_iter().map(|agent| agent.agent_id).collect()
}

#[test]
fn complete_population_dehydrates_atomically_and_restores_before_admission() {
    let root = TempDir::new().unwrap();
    let volume = volume(&root);
    let mut controller = ResidentContinuityController::new(agents()).unwrap();
    assert!(controller.status().admission_open);

    let checkpoint = controller
        .dehydrate_for_spot(&notice(), &volume, 1_001, 1_000_000)
        .unwrap();
    assert_eq!(checkpoint.schema, RESIDENT_POPULATION_SCHEMA);
    assert!(checkpoint.admission_closed);
    assert_eq!(checkpoint.agents.len(), 3);
    assert!(!controller.status().admission_open);
    assert!(checkpoint
        .agents
        .iter()
        .all(|agent| { agent.lifecycle_state == ExistingAgentLifecycleState::Dormant }));

    let path = persist_population_atomically(&volume, &checkpoint).unwrap();
    let loaded = load_population(&path).unwrap();
    let (restored, receipt) =
        ResidentContinuityController::restore_before_admission(loaded, &expected_ids(), &volume, 1)
            .unwrap();
    assert!(restored.status().admission_open);
    assert_eq!(restored.status().resident_count, 3);
    assert_eq!(receipt.restored_agent_count, 3);
    assert_eq!(
        receipt.restored_agent_ids,
        expected_ids().into_iter().collect::<Vec<_>>()
    );
}

#[test]
fn missing_duplicate_busy_oversized_and_expired_populations_fail_closed() {
    let root = TempDir::new().unwrap();
    let volume = volume(&root);

    let mut duplicate = agents();
    duplicate.push(duplicate[0].clone());
    assert!(ResidentContinuityController::new(duplicate)
        .unwrap_err()
        .contains("duplicate"));

    let mut busy = agents();
    busy[0].busy = true;
    let mut controller = ResidentContinuityController::new(busy).unwrap();
    assert!(controller
        .dehydrate_for_spot(&notice(), &volume, 1_001, 1_000_000)
        .unwrap_err()
        .contains("busy"));
    assert!(!controller.status().admission_open);

    let mut oversized = agents();
    oversized[0].serialized_bytes = 10_000;
    let mut controller = ResidentContinuityController::new(oversized).unwrap();
    assert!(controller
        .dehydrate_for_spot(&notice(), &volume, 1_001, 9_999)
        .unwrap_err()
        .contains("oversized"));

    let mut controller = ResidentContinuityController::new(agents()).unwrap();
    assert!(controller
        .dehydrate_for_spot(&notice(), &volume, 1_120, 1_000_000)
        .unwrap_err()
        .contains("expired"));

    let mut unconfirmed = notice();
    unconfirmed.source = "operator-claim".to_string();
    let mut controller = ResidentContinuityController::new(agents()).unwrap();
    assert!(controller
        .dehydrate_for_spot(&unconfirmed, &volume, 1_001, 1_000_000)
        .unwrap_err()
        .contains("unconfirmed"));
}

#[test]
fn tamper_rollback_partial_substitution_and_volume_alias_fail_closed() {
    let root = TempDir::new().unwrap();
    let volume = volume(&root);
    let mut controller = ResidentContinuityController::new(agents()).unwrap();
    let checkpoint = controller
        .dehydrate_for_spot(&notice(), &volume, 1_001, 1_000_000)
        .unwrap();

    let mut tampered = checkpoint.clone();
    tampered.agents[0].state_sha256 = sha("tampered");
    assert!(ResidentContinuityController::restore_before_admission(
        tampered,
        &expected_ids(),
        &volume,
        1,
    )
    .unwrap_err()
    .contains("digest"));

    assert!(ResidentContinuityController::restore_before_admission(
        checkpoint.clone(),
        &expected_ids(),
        &volume,
        2,
    )
    .unwrap_err()
    .contains("generation"));

    let mut partial = checkpoint.clone();
    partial.agents.pop();
    partial.population_sha256.clear();
    assert!(ResidentContinuityController::restore_before_admission(
        partial,
        &expected_ids(),
        &volume,
        1,
    )
    .is_err());

    let substituted = RuntimeVolumeBinding {
        volume_id_sha256: sha("another-volume"),
        ..volume.clone()
    };
    assert!(ResidentContinuityController::restore_before_admission(
        checkpoint,
        &expected_ids(),
        &substituted,
        1,
    )
    .unwrap_err()
    .contains("volume"));

    let aliased = RuntimeVolumeBinding {
        runtime_mount: root.path().join("cache/runtime"),
        build_cache_mount: root.path().join("cache"),
        volume_id_sha256: sha("runtime-volume-id"),
        exclusive_attachment: true,
    };
    let mut controller = ResidentContinuityController::new(agents()).unwrap();
    assert!(controller
        .dehydrate_for_spot(&notice(), &aliased, 1_001, 1_000_000)
        .unwrap_err()
        .contains("aliases"));
}

#[test]
fn operator_status_is_redacted_and_has_no_private_agent_state() {
    let status = ResidentContinuityController::new(agents())
        .unwrap()
        .status();
    let json = serde_json::to_string(&status).unwrap();
    assert!(json.contains("resident_count"));
    for forbidden in ["checkpoint_ref", "completed_task", "next_task", "model"] {
        assert!(!json.contains(forbidden));
    }
}

#[test]
fn cpu_runner_contract() {
    let required = ["llama3.1:8b", "qwen3:8b", "phi4-mini"];
    let cold_latency_millis = required
        .iter()
        .map(|model| (model.to_string(), 2_000))
        .collect::<BTreeMap<_, _>>();
    let warm_latency_millis = required
        .iter()
        .map(|model| (model.to_string(), 800))
        .collect::<BTreeMap<_, _>>();
    let task_result_sha256 = required
        .iter()
        .map(|model| (model.to_string(), sha(model)))
        .collect::<BTreeMap<_, _>>();
    let receipt = ResidentHabitabilityReceipt {
        schema: "adl.runtime.resident_shepherd_habitability.v1".to_string(),
        instance_type: "r7i.2xlarge".to_string(),
        vcpus: 8,
        memory_mib: 65_536,
        context_tokens: 8_192,
        parallelism: 2,
        max_loaded_models: 2,
        compilation_concurrent: false,
        peak_rss_mib: 32_000,
        swap_used_mib: 0,
        cold_latency_millis,
        warm_latency_millis,
        task_result_sha256,
    };
    validate_habitability(&receipt).unwrap();

    let mut no_headroom = receipt.clone();
    no_headroom.peak_rss_mib = 55_000;
    assert!(validate_habitability(&no_headroom).is_err());
    let mut swap = receipt;
    swap.swap_used_mib = 1;
    assert!(validate_habitability(&swap).is_err());
}
