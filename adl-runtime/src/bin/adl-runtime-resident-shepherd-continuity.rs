use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::{Path, PathBuf},
    time::Duration,
};

use adl_runtime::resident_shepherd_continuity::{
    persist_population_atomically, validate_population_before_restore, ExistingAgentLifecycleState,
    ResidentAgentCapsule, ResidentContinuityController, ResidentModelBinding,
    ResidentPopulationCheckpoint, RuntimeVolumeBinding, SpotInterruptionNotice, SPOT_NOTICE_SOURCE,
};
use adl_runtime_kernel::{LiveContinuity, LiveKernelSnapshot, RuntimeRecorder};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Deserialize)]
struct PreReceipt {
    residents: Vec<PreResident>,
}

#[derive(Deserialize)]
struct PreResident {
    agent_id: String,
    model: String,
    artifact_sha256: String,
    quantization: String,
    completed_task_sha256: String,
    continuation_request_sha256: String,
}

#[derive(Deserialize)]
struct FinalReceipt {
    residents: Vec<FinalResident>,
}

#[derive(Deserialize)]
struct FinalResident {
    agent_id: String,
    completed_task_sha256: String,
    continuation_request_sha256: String,
    next_task_sha256: String,
}

#[derive(Serialize, Deserialize)]
struct DehydrationState {
    schema: String,
    expected_agent_ids: BTreeSet<String>,
    population_sha256: String,
    generation: u64,
    quiescent_round_trip_count: usize,
}

fn digest(value: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(value.as_ref()))
}

fn volume(root: &Path) -> RuntimeVolumeBinding {
    RuntimeVolumeBinding {
        runtime_mount: root.to_path_buf(),
        build_cache_mount: root.parent().unwrap_or(root).join("ephemeral-build-cache"),
        volume_id_sha256: digest(root.to_string_lossy().as_bytes()),
        exclusive_attachment: true,
    }
}

fn parse_args() -> Result<(String, PathBuf, PathBuf, PathBuf), String> {
    let mut args = env::args().skip(1);
    let phase = args.next().ok_or("missing phase: dehydrate|restore")?;
    let mut input = None;
    let mut runtime_root = None;
    let mut output = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input = args.next().map(PathBuf::from),
            "--runtime-root" => runtime_root = args.next().map(PathBuf::from),
            "--output" => output = args.next().map(PathBuf::from),
            _ => return Err(format!("unknown argument {arg}")),
        }
    }
    Ok((
        phase,
        input.ok_or("missing --input")?,
        runtime_root.ok_or("missing --runtime-root")?,
        output.ok_or("missing --output")?,
    ))
}

#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        eprintln!("resident continuity proof failed: {error}");
        std::process::exit(1);
    }
}

async fn run() -> Result<(), String> {
    let (phase, input, runtime_root, output) = parse_args()?;
    match phase.as_str() {
        "dehydrate" => dehydrate(&input, &runtime_root, &output).await,
        "restore" => restore(&input, &runtime_root, &output).await,
        _ => Err("phase must be dehydrate or restore".to_string()),
    }
}

async fn dehydrate(input: &Path, runtime_root: &Path, output: &Path) -> Result<(), String> {
    let receipt: PreReceipt = serde_json::from_slice(&fs::read(input).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if receipt.residents.len() < 2 {
        return Err("multi-resident proof requires at least two residents".to_string());
    }
    let mut agents = Vec::with_capacity(receipt.residents.len());
    for (index, resident) in receipt.residents.iter().enumerate() {
        agents.push(ResidentAgentCapsule {
            agent_id: resident.agent_id.clone(),
            lifecycle_state: ExistingAgentLifecycleState::Active,
            checkpoint_ref: format!("agents/{}/continuity_checkpoint.json", resident.agent_id),
            sequence: index as u64 + 1,
            predecessor_sha256: None,
            state_sha256: digest(format!(
                "{}:{}",
                resident.agent_id, resident.completed_task_sha256
            )),
            completed_task_sha256: resident.completed_task_sha256.clone(),
            next_task_sha256: resident.continuation_request_sha256.clone(),
            busy: false,
            serialized_bytes: 4096,
            model: ResidentModelBinding {
                provider: "ollama".to_string(),
                model: resident.model.clone(),
                quantization: resident.quantization.clone(),
                artifact_sha256: resident.artifact_sha256.clone(),
                configuration_sha256: digest(format!(
                    "{}:{}:8192",
                    resident.model, resident.quantization
                )),
                authoritative_for_continuity: true,
            },
        });
    }
    let expected_agent_ids = agents
        .iter()
        .map(|a| a.agent_id.clone())
        .collect::<BTreeSet<_>>();
    let mut controller = ResidentContinuityController::new(agents)?;
    let quiescent_round_trip_count = controller.exercise_quiescent_habitability()?;
    let binding = volume(runtime_root);
    let checkpoint = controller.dehydrate_for_spot(
        &SpotInterruptionNotice {
            source: SPOT_NOTICE_SOURCE.to_string(),
            action: "terminate".to_string(),
            observed_unix_seconds: 1_000,
            deadline_unix_seconds: 1_120,
        },
        &binding,
        1_001,
        1_000_000,
    )?;
    persist_population_atomically(&binding, &checkpoint)?;
    let population = serde_jcs::to_vec(&checkpoint).map_err(|e| e.to_string())?;
    let identity = LiveKernelSnapshot::new(
        digest("issue-414-topology"),
        digest("issue-414-configuration"),
        BTreeMap::from([("runtime".to_string(), "resident-shepherd".to_string())]),
    );
    let mut live = LiveContinuity::new(
        runtime_root.join("live-kernel"),
        "issue-414-local-proof",
        &[0x9e; 32],
        identity,
        0,
    )
    .with_resident_population(population);
    live.checkpoint(&RuntimeRecorder::new(16), Duration::from_secs(2))
        .await
        .map_err(|e| e.to_string())?;
    let state = DehydrationState {
        schema: "adl.runtime.resident_shepherd_dehydration_state.v1".to_string(),
        expected_agent_ids,
        population_sha256: checkpoint.population_sha256,
        generation: checkpoint.generation,
        quiescent_round_trip_count,
    };
    fs::write(
        output,
        serde_json::to_vec_pretty(&state).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}

async fn restore(input: &Path, runtime_root: &Path, output: &Path) -> Result<(), String> {
    let receipt: FinalReceipt =
        serde_json::from_slice(&fs::read(input).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let state_path = runtime_root.join("dehydration-state.json");
    let state: DehydrationState =
        serde_json::from_slice(&fs::read(&state_path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let binding = volume(runtime_root);
    let identity = LiveKernelSnapshot::new(
        digest("issue-414-topology"),
        digest("issue-414-configuration"),
        BTreeMap::from([("runtime".to_string(), "resident-shepherd".to_string())]),
    );
    let mut live = LiveContinuity::new(
        runtime_root.join("live-kernel"),
        "issue-414-local-proof",
        &[0x9e; 32],
        identity,
        1,
    );
    let restored = live
        .restore_latest_with_resident_population(&RuntimeRecorder::new(16), |bytes| {
            let checkpoint: ResidentPopulationCheckpoint =
                serde_json::from_slice(bytes).map_err(|e| e.to_string())?;
            validate_population_before_restore(
                &checkpoint,
                &state.expected_agent_ids,
                &binding,
                state.generation,
            )
        })
        .await
        .map_err(|e| e.to_string())?
        .ok_or("signed live continuity generation missing")?;
    let checkpoint: ResidentPopulationCheckpoint = serde_json::from_slice(
        restored
            .resident_population
            .as_deref()
            .ok_or("resident subrecord missing")?,
    )
    .map_err(|e| e.to_string())?;
    let checkpoint_by_id = checkpoint
        .agents
        .iter()
        .map(|agent| (agent.agent_id.as_str(), agent))
        .collect::<BTreeMap<_, _>>();
    let final_by_id = receipt
        .residents
        .iter()
        .map(|resident| (resident.agent_id.as_str(), resident))
        .collect::<BTreeMap<_, _>>();
    if final_by_id.len() != state.expected_agent_ids.len() {
        return Err("final useful-work population is partial or duplicate".to_string());
    }
    for id in &state.expected_agent_ids {
        let agent = checkpoint_by_id
            .get(id.as_str())
            .ok_or("checkpoint resident missing")?;
        let resident = final_by_id
            .get(id.as_str())
            .ok_or("final useful-work resident missing")?;
        if agent.completed_task_sha256 != resident.completed_task_sha256
            || agent.next_task_sha256 != resident.continuation_request_sha256
        {
            return Err("post-restore useful work does not match signed task lineage".to_string());
        }
    }
    let (controller, recovery) = ResidentContinuityController::restore_before_admission(
        checkpoint,
        &state.expected_agent_ids,
        &binding,
        state.generation,
    )?;
    if !controller.status().admission_open || recovery.population_sha256 != state.population_sha256
    {
        return Err("admission or population digest mismatch after restore".to_string());
    }
    for id in &state.expected_agent_ids {
        let resident = final_by_id
            .get(id.as_str())
            .ok_or("final useful-work resident missing")?;
        if resident.completed_task_sha256 == resident.next_task_sha256
            || resident.continuation_request_sha256.trim().is_empty()
            || resident.next_task_sha256.trim().is_empty()
        {
            return Err("post-restore useful work did not advance".to_string());
        }
    }
    let result = serde_json::json!({
        "schema": "adl.runtime.resident_shepherd_end_to_end_receipt.v1",
        "generation": recovery.generation,
        "population_sha256": recovery.population_sha256,
        "resident_count": recovery.restored_agent_count,
        "quiescent_round_trip_count": state.quiescent_round_trip_count,
        "admission_open_after_validated_restore": true,
        "continuation_count": receipt.residents.len(),
    });
    fs::write(
        output,
        serde_json::to_vec_pretty(&result).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())
}
