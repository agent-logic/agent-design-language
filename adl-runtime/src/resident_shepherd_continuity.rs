//! Resident Shepherd continuity adapter for the existing Runtime lifecycle.
//!
//! This module deliberately does not define a new lifecycle or recovery
//! authority. It serializes the existing per-agent lifecycle/capsule facts as
//! one deterministic population payload before the signed `live_kernel`
//! checkpoint boundary, and validates that payload before admission reopens.

use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const RESIDENT_POPULATION_SCHEMA: &str = "adl.runtime.resident_shepherd_population.v1";
pub const RESIDENT_RECEIPT_SCHEMA: &str = "adl.runtime.resident_shepherd_continuity_receipt.v1";
pub const SPOT_NOTICE_SOURCE: &str = "aws_imds_v2/spot/instance-action";
pub const R7I_2XLARGE_VCPUS: u16 = 8;
pub const R7I_2XLARGE_MEMORY_MIB: u64 = 65_536;
pub const REQUIRED_CONTEXT_TOKENS: u32 = 8_192;
pub const REQUIRED_PARALLELISM: u16 = 2;
pub const REQUIRED_MAX_LOADED_MODELS: u16 = 2;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExistingAgentLifecycleState {
    Active,
    Quiescent,
    Suspended,
    Dormant,
    Simulation,
    InTransit,
    Bootstrap,
    Shutdown,
    ForcedSuspension,
    Quarantined,
    Rejected,
    Orphaned,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentModelBinding {
    pub provider: String,
    pub model: String,
    pub quantization: String,
    pub artifact_sha256: String,
    pub configuration_sha256: String,
    pub authoritative_for_continuity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentAgentCapsule {
    pub agent_id: String,
    pub lifecycle_state: ExistingAgentLifecycleState,
    pub checkpoint_ref: String,
    pub sequence: u64,
    pub predecessor_sha256: Option<String>,
    pub state_sha256: String,
    pub completed_task_sha256: String,
    pub next_task_sha256: String,
    pub busy: bool,
    pub serialized_bytes: u64,
    pub model: ResidentModelBinding,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RuntimeVolumeBinding {
    pub runtime_mount: PathBuf,
    pub build_cache_mount: PathBuf,
    pub volume_id_sha256: String,
    pub exclusive_attachment: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SpotInterruptionNotice {
    pub source: String,
    pub action: String,
    pub observed_unix_seconds: u64,
    pub deadline_unix_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentPopulationCheckpoint {
    pub schema: String,
    pub generation: u64,
    pub previous_population_sha256: Option<String>,
    pub admission_closed: bool,
    pub lifecycle_path: Vec<ExistingAgentLifecycleState>,
    pub volume_id_sha256: String,
    pub agents: Vec<ResidentAgentCapsule>,
    pub population_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentRecoveryReceipt {
    pub schema: String,
    pub generation: u64,
    pub population_sha256: String,
    pub restored_agent_count: usize,
    pub restored_agent_ids: Vec<String>,
    pub admission_reopened: bool,
    pub next_transition_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentHabitabilityReceipt {
    pub schema: String,
    pub instance_type: String,
    pub vcpus: u16,
    pub memory_mib: u64,
    pub context_tokens: u32,
    pub parallelism: u16,
    pub max_loaded_models: u16,
    pub compilation_concurrent: bool,
    pub peak_rss_mib: u64,
    pub swap_used_mib: u64,
    pub cold_latency_millis: BTreeMap<String, u64>,
    pub warm_latency_millis: BTreeMap<String, u64>,
    pub task_result_sha256: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentOperatorStatus {
    pub generation: u64,
    pub admission_open: bool,
    pub resident_count: usize,
    pub lifecycle_counts: BTreeMap<ExistingAgentLifecycleState, usize>,
    pub population_sha256: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ResidentContinuityController {
    generation: u64,
    admission_open: bool,
    agents: BTreeMap<String, ResidentAgentCapsule>,
    previous_population_sha256: Option<String>,
}

impl ResidentContinuityController {
    pub fn new(agents: Vec<ResidentAgentCapsule>) -> Result<Self, String> {
        let agents = exact_agent_map(agents)?;
        if agents.is_empty() {
            return Err("resident population must not be empty".to_string());
        }
        Ok(Self {
            generation: 0,
            admission_open: true,
            agents,
            previous_population_sha256: None,
        })
    }

    pub fn status(&self) -> ResidentOperatorStatus {
        let mut lifecycle_counts = BTreeMap::new();
        for agent in self.agents.values() {
            *lifecycle_counts.entry(agent.lifecycle_state).or_insert(0) += 1;
        }
        ResidentOperatorStatus {
            generation: self.generation,
            admission_open: self.admission_open,
            resident_count: self.agents.len(),
            lifecycle_counts,
            population_sha256: self.previous_population_sha256.clone(),
        }
    }

    pub fn dehydrate_for_spot(
        &mut self,
        notice: &SpotInterruptionNotice,
        volume: &RuntimeVolumeBinding,
        now_unix_seconds: u64,
        max_agent_bytes: u64,
    ) -> Result<ResidentPopulationCheckpoint, String> {
        validate_notice(notice, now_unix_seconds)?;
        validate_volume(volume)?;
        self.admission_open = false;
        for agent in self.agents.values() {
            validate_agent(agent, max_agent_bytes)?;
            if agent.lifecycle_state != ExistingAgentLifecycleState::Active {
                return Err(format!("resident {} is not ACTIVE", agent.agent_id));
            }
        }
        let generation = self
            .generation
            .checked_add(1)
            .ok_or("generation overflow")?;
        let mut dormant = self.agents.values().cloned().collect::<Vec<_>>();
        for agent in &mut dormant {
            agent.lifecycle_state = ExistingAgentLifecycleState::Dormant;
        }
        let mut checkpoint = ResidentPopulationCheckpoint {
            schema: RESIDENT_POPULATION_SCHEMA.to_string(),
            generation,
            previous_population_sha256: self.previous_population_sha256.clone(),
            admission_closed: true,
            lifecycle_path: vec![
                ExistingAgentLifecycleState::Active,
                ExistingAgentLifecycleState::Quiescent,
                ExistingAgentLifecycleState::Suspended,
                ExistingAgentLifecycleState::Dormant,
            ],
            volume_id_sha256: volume.volume_id_sha256.clone(),
            agents: dormant,
            population_sha256: String::new(),
        };
        checkpoint.population_sha256 = population_digest(&checkpoint)?;
        self.generation = generation;
        self.previous_population_sha256 = Some(checkpoint.population_sha256.clone());
        self.agents = exact_agent_map(checkpoint.agents.clone())?;
        Ok(checkpoint)
    }

    pub fn restore_before_admission(
        checkpoint: ResidentPopulationCheckpoint,
        expected_agent_ids: &BTreeSet<String>,
        volume: &RuntimeVolumeBinding,
        minimum_generation: u64,
    ) -> Result<(Self, ResidentRecoveryReceipt), String> {
        validate_volume(volume)?;
        validate_checkpoint(&checkpoint, expected_agent_ids, volume, minimum_generation)?;
        let population_sha256 = checkpoint.population_sha256.clone();
        let generation = checkpoint.generation;
        let mut restored = checkpoint.agents;
        for agent in &mut restored {
            agent.lifecycle_state = ExistingAgentLifecycleState::Active;
            agent.sequence = agent.sequence.checked_add(1).ok_or("sequence overflow")?;
            agent.predecessor_sha256 = Some(agent.state_sha256.clone());
            agent.state_sha256 = digest_bytes(
                format!(
                    "{}:{}:{}:{}",
                    agent.agent_id, agent.sequence, agent.next_task_sha256, population_sha256
                )
                .as_bytes(),
            );
        }
        let agents = exact_agent_map(restored)?;
        let restored_agent_ids = agents.keys().cloned().collect::<Vec<_>>();
        let next_transition_sha256 = digest_json(&restored_agent_ids)?;
        let controller = Self {
            generation,
            admission_open: true,
            agents,
            previous_population_sha256: Some(population_sha256.clone()),
        };
        let receipt = ResidentRecoveryReceipt {
            schema: RESIDENT_RECEIPT_SCHEMA.to_string(),
            generation,
            population_sha256,
            restored_agent_count: restored_agent_ids.len(),
            restored_agent_ids,
            admission_reopened: true,
            next_transition_sha256,
        };
        Ok((controller, receipt))
    }
}

pub fn persist_population_atomically(
    volume: &RuntimeVolumeBinding,
    checkpoint: &ResidentPopulationCheckpoint,
) -> Result<PathBuf, String> {
    validate_volume(volume)?;
    let expected = population_digest(checkpoint)?;
    if checkpoint.population_sha256 != expected {
        return Err("population digest mismatch".to_string());
    }
    let root = volume.runtime_mount.join("resident-continuity");
    fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let target = root.join(format!("generation-{}.json", checkpoint.generation));
    let temporary = root.join(format!(".generation-{}.tmp", checkpoint.generation));
    let bytes = serde_jcs::to_vec(checkpoint).map_err(|error| error.to_string())?;
    fs::write(&temporary, bytes).map_err(|error| error.to_string())?;
    fs::rename(&temporary, &target).map_err(|error| error.to_string())?;
    Ok(target)
}

pub fn load_population(path: &Path) -> Result<ResidentPopulationCheckpoint, String> {
    let bytes = fs::read(path).map_err(|error| error.to_string())?;
    serde_json::from_slice(&bytes).map_err(|error| error.to_string())
}

pub fn validate_habitability(receipt: &ResidentHabitabilityReceipt) -> Result<(), String> {
    if receipt.schema != "adl.runtime.resident_shepherd_habitability.v1"
        || receipt.instance_type != "r7i.2xlarge"
        || receipt.vcpus != R7I_2XLARGE_VCPUS
        || receipt.memory_mib != R7I_2XLARGE_MEMORY_MIB
    {
        return Err("habitability receipt is not the exact r7i.2xlarge envelope".to_string());
    }
    if receipt.context_tokens != REQUIRED_CONTEXT_TOKENS
        || receipt.parallelism != REQUIRED_PARALLELISM
        || receipt.max_loaded_models != REQUIRED_MAX_LOADED_MODELS
        || receipt.compilation_concurrent
    {
        return Err("habitability tuning contract mismatch".to_string());
    }
    if receipt.peak_rss_mib >= receipt.memory_mib.saturating_sub(16_384)
        || receipt.swap_used_mib != 0
    {
        return Err("resident population lacks required memory or swap headroom".to_string());
    }
    let required = ["llama3.1:8b", "qwen3:8b", "phi4-mini"];
    for model in required {
        if !receipt.cold_latency_millis.contains_key(model)
            || !receipt.warm_latency_millis.contains_key(model)
            || !receipt.task_result_sha256.contains_key(model)
        {
            return Err(format!("missing nonzero useful-work evidence for {model}"));
        }
        if receipt.cold_latency_millis[model] == 0
            || receipt.warm_latency_millis[model] == 0
            || !is_sha256(&receipt.task_result_sha256[model])
        {
            return Err(format!("invalid useful-work evidence for {model}"));
        }
    }
    Ok(())
}

fn validate_checkpoint(
    checkpoint: &ResidentPopulationCheckpoint,
    expected_agent_ids: &BTreeSet<String>,
    volume: &RuntimeVolumeBinding,
    minimum_generation: u64,
) -> Result<(), String> {
    if checkpoint.schema != RESIDENT_POPULATION_SCHEMA
        || !checkpoint.admission_closed
        || checkpoint.generation < minimum_generation
        || checkpoint.volume_id_sha256 != volume.volume_id_sha256
    {
        return Err("checkpoint authority, generation, admission, or volume mismatch".to_string());
    }
    let expected_path = [
        ExistingAgentLifecycleState::Active,
        ExistingAgentLifecycleState::Quiescent,
        ExistingAgentLifecycleState::Suspended,
        ExistingAgentLifecycleState::Dormant,
    ];
    if checkpoint.lifecycle_path != expected_path {
        return Err("checkpoint does not use the existing dehydration lifecycle path".to_string());
    }
    if checkpoint.population_sha256 != population_digest(checkpoint)? {
        return Err("population digest mismatch".to_string());
    }
    let agents = exact_agent_map(checkpoint.agents.clone())?;
    let actual = agents.keys().cloned().collect::<BTreeSet<_>>();
    if &actual != expected_agent_ids {
        return Err("restored population is partial or substituted".to_string());
    }
    for agent in agents.values() {
        validate_agent(agent, u64::MAX)?;
        if agent.lifecycle_state != ExistingAgentLifecycleState::Dormant {
            return Err(format!("resident {} was not dehydrated", agent.agent_id));
        }
    }
    Ok(())
}

fn validate_agent(agent: &ResidentAgentCapsule, max_agent_bytes: u64) -> Result<(), String> {
    if agent.agent_id.trim().is_empty()
        || !is_relative_normal_path(&agent.checkpoint_ref)
        || agent.sequence == 0
        || !is_sha256(&agent.state_sha256)
        || !is_sha256(&agent.completed_task_sha256)
        || !is_sha256(&agent.next_task_sha256)
        || agent.busy
        || agent.serialized_bytes == 0
        || agent.serialized_bytes > max_agent_bytes
    {
        return Err(format!(
            "resident {} capsule is incomplete, busy, or oversized",
            agent.agent_id
        ));
    }
    if agent.model.provider != "ollama"
        || agent.model.model.trim().is_empty()
        || agent.model.quantization.trim().is_empty()
        || !is_sha256(&agent.model.artifact_sha256)
        || !is_sha256(&agent.model.configuration_sha256)
    {
        return Err(format!(
            "resident {} model binding is invalid",
            agent.agent_id
        ));
    }
    Ok(())
}

fn validate_notice(notice: &SpotInterruptionNotice, now: u64) -> Result<(), String> {
    if notice.source != SPOT_NOTICE_SOURCE
        || !matches!(notice.action.as_str(), "terminate" | "stop")
        || notice.observed_unix_seconds > now
        || now >= notice.deadline_unix_seconds
    {
        return Err("unconfirmed or expired Spot interruption notice".to_string());
    }
    Ok(())
}

fn validate_volume(volume: &RuntimeVolumeBinding) -> Result<(), String> {
    if !volume.runtime_mount.is_absolute()
        || !volume.build_cache_mount.is_absolute()
        || !is_normal_path(&volume.runtime_mount)
        || !is_normal_path(&volume.build_cache_mount)
        || volume.runtime_mount == volume.build_cache_mount
        || volume.runtime_mount.starts_with(&volume.build_cache_mount)
        || volume.build_cache_mount.starts_with(&volume.runtime_mount)
        || !is_sha256(&volume.volume_id_sha256)
        || !volume.exclusive_attachment
    {
        return Err(
            "Runtime continuity volume is missing, ambiguous, or aliases build cache".to_string(),
        );
    }
    Ok(())
}

fn exact_agent_map(
    agents: Vec<ResidentAgentCapsule>,
) -> Result<BTreeMap<String, ResidentAgentCapsule>, String> {
    let expected = agents.len();
    let map = agents
        .into_iter()
        .map(|agent| (agent.agent_id.clone(), agent))
        .collect::<BTreeMap<_, _>>();
    if map.len() != expected {
        return Err("resident population contains duplicate agent IDs".to_string());
    }
    Ok(map)
}

fn population_digest(checkpoint: &ResidentPopulationCheckpoint) -> Result<String, String> {
    let mut unsigned = checkpoint.clone();
    unsigned.population_sha256.clear();
    digest_json(&unsigned)
}

fn digest_json(value: &impl Serialize) -> Result<String, String> {
    let bytes = serde_jcs::to_vec(value).map_err(|error| error.to_string())?;
    Ok(digest_bytes(&bytes))
}

fn digest_bytes(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

fn is_sha256(value: &str) -> bool {
    value.len() == 64 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

fn is_relative_normal_path(value: &str) -> bool {
    let path = Path::new(value);
    !path.as_os_str().is_empty() && !path.is_absolute() && is_normal_path(path)
}

fn is_normal_path(path: &Path) -> bool {
    path.components()
        .all(|component| matches!(component, Component::RootDir | Component::Normal(_)))
}
