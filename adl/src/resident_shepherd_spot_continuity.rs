//! Integration of resident model work with the existing ADL continuity authorities.
//!
//! This module deliberately owns no replacement agent checkpoint.  Runtime-v2
//! snapshot/rehydration artifacts and per-agent CSM continuity capsules remain
//! authoritative; this bridge only binds their exact hashes into the existing
//! signed live-kernel checkpoint and keeps admission closed until every member
//! validates and restores.

use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

use adl_runtime::{LiveContinuity, LiveKernelSnapshot, RuntimeRecorder};
use anyhow::{anyhow, bail, Context, Result};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{
    csm_continuity_capsule::{
        capture_capsule, restore_capsule, ContinuityCaptureOptions, ContinuityRestoreOptions,
    },
    long_lived_agent,
    runtime_v2::{
        runtime_v2_agent_lifecycle_state_model, runtime_v2_snapshot_rehydration_contract,
        runtime_v2_snapshot_rehydration_for_active_citizens, RuntimeV2AgentLifecycleState,
        RuntimeV2RehydrationReport, RuntimeV2SnapshotManifest,
    },
};

pub const RESIDENT_INTEGRATION_SCHEMA: &str =
    "adl.runtime.resident_shepherd_continuity_integration.v1";
pub const RESIDENT_RECEIPT_SCHEMA: &str = "adl.runtime.resident_shepherd_habitability.v2";

pub fn preflight(input: &DehydrationInput) -> Result<serde_json::Value> {
    validate_roots(input)?;
    validate_models(&input.residents)?;
    runtime_v2_agent_lifecycle_state_model()?.validate()?;
    runtime_v2_snapshot_rehydration_contract()?.validate()?;
    let _ = live_continuity(&input.retained_runtime_root, 0)?;
    Ok(serde_json::json!({
        "schema": "adl.runtime.resident_shepherd_preflight.v1",
        "status": "passed",
        "resident_count": input.residents.len(),
        "retained_build_roots_disjoint": true,
        "signing_key_exact_bytes": 32,
        "runtime_v2_authorities_valid": true
    }))
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidentModelBinding {
    pub agent_id: String,
    pub model: String,
    pub artifact_sha256: String,
    pub quantization: String,
    pub configuration_sha256: String,
    pub completed_task_sha256: String,
    pub continuation_request_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DehydrationInput {
    pub residents: Vec<ResidentModelBinding>,
    #[serde(default)]
    pub existing_agent_specs: Vec<PathBuf>,
    pub retained_runtime_root: PathBuf,
    pub build_cache_root: PathBuf,
    pub runtime_volume_identity_sha256: String,
    pub source_host: String,
    pub target_host: String,
    #[serde(default)]
    pub spot_notice: Option<SpotNoticeBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpotNoticeBinding {
    pub source: String,
    pub action: String,
    pub deadline_utc: String,
    pub notice_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationResident {
    pub agent_id: String,
    pub model: String,
    pub artifact_sha256: String,
    pub quantization: String,
    pub configuration_sha256: String,
    pub completed_task_sha256: String,
    pub continuation_request_sha256: String,
    pub next_task_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreInput {
    pub residents: Vec<ResidentModelBinding>,
    pub retained_runtime_root: PathBuf,
    pub build_cache_root: PathBuf,
    pub runtime_volume_identity_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContinuationInput {
    pub residents: Vec<ContinuationResident>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapsuleBinding {
    pub agent_id: String,
    pub bundle_ref: String,
    pub manifest_sha256: String,
    pub custody_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResidentContinuityIntegration {
    pub schema: String,
    pub runtime_v2_snapshot: RuntimeV2SnapshotManifest,
    pub model_bindings: Vec<ResidentModelBinding>,
    pub capsule_bindings: Vec<CapsuleBinding>,
    pub lifecycle_path: Vec<RuntimeV2AgentLifecycleState>,
    pub retained_runtime_root_sha256: String,
    pub build_cache_root_sha256: String,
    pub runtime_volume_identity_sha256: String,
    pub spot_notice: Option<SpotNoticeBinding>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DehydrationReceipt {
    pub schema: String,
    pub generation: u64,
    pub population_sha256: String,
    pub resident_count: usize,
    pub capsule_count: usize,
    pub admission_open: bool,
    pub dehydration_millis: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestoreReceipt {
    pub schema: String,
    pub generation: u64,
    pub population_sha256: String,
    pub resident_count: usize,
    pub capsule_count: usize,
    pub admission_open: bool,
    pub restore_millis: u64,
    pub continuation_verified: bool,
}

pub async fn dehydrate(input: &DehydrationInput, deadline: Duration) -> Result<DehydrationReceipt> {
    let started = Instant::now();
    let deadline_at = started + deadline;
    validate_roots(input)?;
    validate_models(&input.residents)?;
    fs::create_dir_all(&input.retained_runtime_root)?;
    close_admission_for_dehydration(&input.retained_runtime_root)?;

    let lifecycle = runtime_v2_agent_lifecycle_state_model()?;
    lifecycle.validate()?;
    let mut state = RuntimeV2AgentLifecycleState::Active;
    state
        .transition(RuntimeV2AgentLifecycleState::Quiescent)
        .map_err(|error| anyhow!(error))?;
    state
        .transition(RuntimeV2AgentLifecycleState::Active)
        .map_err(|error| anyhow!(error))?;
    state
        .transition(RuntimeV2AgentLifecycleState::Suspended)
        .map_err(|error| anyhow!(error))?;
    state
        .transition(RuntimeV2AgentLifecycleState::Dormant)
        .map_err(|error| anyhow!(error))?;

    let mut live = live_continuity(&input.retained_runtime_root, 0)?;
    live.restore_latest_with_resident_population(&RuntimeRecorder::new(32), |prior| {
        let integration: ResidentContinuityIntegration =
            serde_json::from_slice(prior).map_err(|error| error.to_string())?;
        validate_integration(&integration, &input.retained_runtime_root)
            .map_err(|error| error.to_string())
    })
    .await
    .context("restore existing signed resident lineage before advancing it")?;
    let capsules = capture_all(input, deadline_at)?;
    let result: Result<DehydrationReceipt> = async {
        // Each resident is one Runtime-v2 citizen and must have exactly one
        // existing-agent CSM capsule.  Capsule bindings are recovery evidence
        // for those same citizens, not additional citizens.
        let citizen_ids = input
            .residents
            .iter()
            .map(|resident| resident.agent_id.clone())
            .collect::<Vec<_>>();
        let snapshot = runtime_v2_snapshot_rehydration_for_active_citizens(&citizen_ids)?;
        snapshot.validate()?;
        let integration = ResidentContinuityIntegration {
            schema: RESIDENT_INTEGRATION_SCHEMA.to_string(),
            runtime_v2_snapshot: snapshot.snapshot,
            model_bindings: input.residents.clone(),
            capsule_bindings: capsules,
            lifecycle_path: vec![
                RuntimeV2AgentLifecycleState::Active,
                RuntimeV2AgentLifecycleState::Quiescent,
                RuntimeV2AgentLifecycleState::Active,
                RuntimeV2AgentLifecycleState::Suspended,
                RuntimeV2AgentLifecycleState::Dormant,
            ],
            retained_runtime_root_sha256: digest(
                input.retained_runtime_root.to_string_lossy().as_bytes(),
            ),
            build_cache_root_sha256: digest(input.build_cache_root.to_string_lossy().as_bytes()),
            runtime_volume_identity_sha256: input.runtime_volume_identity_sha256.clone(),
            spot_notice: input.spot_notice.clone(),
        };
        validate_integration(&integration, &input.retained_runtime_root)?;
        let bytes = serde_jcs::to_vec(&integration)?;
        let population_sha256 = digest(&bytes);
        live.set_resident_population(bytes);
        let remaining = deadline_at
            .checked_duration_since(Instant::now())
            .context("Spot dehydration deadline expired before signed checkpoint")?;
        let checkpoint = live
            .checkpoint(&RuntimeRecorder::new(32), remaining)
            .await
            .context("signed live-kernel checkpoint")?;
        let receipt = DehydrationReceipt {
            schema: "adl.runtime.resident_shepherd_dehydration_receipt.v1".to_string(),
            generation: checkpoint.generation,
            population_sha256,
            resident_count: integration.model_bindings.len(),
            capsule_count: integration.capsule_bindings.len(),
            admission_open: false,
            dehydration_millis: started.elapsed().as_millis() as u64,
        };
        write_atomic_json(
            &input.retained_runtime_root.join("dehydration-receipt.json"),
            &receipt,
        )?;
        write_closed_admission(&input.retained_runtime_root, checkpoint.generation)?;
        Ok(receipt)
    }
    .await;
    result
}

pub async fn restore_and_admit(
    retained_runtime_root: &Path,
    restore_input: &RestoreInput,
) -> Result<RestoreReceipt> {
    let started = Instant::now();
    if restore_input.retained_runtime_root != retained_runtime_root {
        bail!("restore input Runtime root differs from the root being opened");
    }
    validate_roots(&DehydrationInput {
        residents: restore_input.residents.clone(),
        existing_agent_specs: vec![],
        retained_runtime_root: restore_input.retained_runtime_root.clone(),
        build_cache_root: restore_input.build_cache_root.clone(),
        runtime_volume_identity_sha256: restore_input.runtime_volume_identity_sha256.clone(),
        source_host: "restore-validation".to_string(),
        target_host: "restore-validation".to_string(),
        spot_notice: None,
    })?;
    let prior: DehydrationReceipt =
        read_json(&retained_runtime_root.join("dehydration-receipt.json"))?;
    let mut validated: Option<ResidentContinuityIntegration> = None;
    let mut live = live_continuity(retained_runtime_root, prior.generation)?;
    let restored = live
        .restore_latest_with_resident_population(&RuntimeRecorder::new(32), |bytes| {
            let integration: ResidentContinuityIntegration =
                serde_json::from_slice(bytes).map_err(|error| error.to_string())?;
            validate_integration(&integration, retained_runtime_root)
                .map_err(|error| error.to_string())?;
            validate_restore_bindings(&integration, restore_input)
                .map_err(|error| error.to_string())?;
            validated = Some(integration);
            Ok(())
        })
        .await
        .context("validate signed resident subrecord before restore")?
        .context("signed live-kernel checkpoint is absent")?;
    let integration = validated.context("resident integration validator did not return state")?;
    let bytes = restored
        .resident_population
        .context("signed resident integration subrecord is absent")?;
    let population_sha256 = digest(&bytes);
    if population_sha256 != prior.population_sha256 {
        bail!("resident population digest differs from dehydration receipt");
    }

    let populations = retained_runtime_root.join("restored-populations");
    fs::create_dir_all(&populations)?;
    let restored_tmp = populations.join(format!("generation-{}.tmp", restored.generation));
    let restored_final = populations.join(format!("generation-{}", restored.generation));
    if restored_tmp.exists() {
        fs::remove_dir_all(&restored_tmp)?;
    }
    fs::create_dir_all(&restored_tmp)?;
    for capsule in &integration.capsule_bindings {
        validate_agent_id(&capsule.agent_id)?;
        restore_capsule(ContinuityRestoreOptions {
            bundle_dir: retained_runtime_root.join(&capsule.bundle_ref),
            out_dir: restored_tmp.join(&capsule.agent_id),
            target_host: "local".to_string(),
        })?;
        long_lived_agent::status(&restored_tmp.join(&capsule.agent_id).join("agent.yaml"))?;
    }
    let restored_ids = integration
        .model_bindings
        .iter()
        .map(|resident| resident.agent_id.clone())
        .collect::<Vec<_>>();
    let runtime_v2_rehydration = RuntimeV2RehydrationReport::after_successful_restore(
        &integration.runtime_v2_snapshot,
        restored_ids,
    )?;
    write_atomic_json(
        &restored_tmp.join("runtime-v2-rehydration-report.json"),
        &runtime_v2_rehydration,
    )?;
    if restored_final.exists() {
        bail!("restored population generation already exists");
    }
    fs::rename(&restored_tmp, &restored_final)?;
    let pointer_path = retained_runtime_root.join("active-population.json");
    let prior_pointer = fs::read(&pointer_path).ok();
    let receipt_path = retained_runtime_root.join("restore-receipt.json");
    let prior_receipt = fs::read(&receipt_path).ok();
    write_atomic_json(
        &pointer_path,
        &serde_json::json!({"generation": restored.generation, "path": restored_final, "admission_open": false}),
    )?;
    let receipt = RestoreReceipt {
        schema: "adl.runtime.resident_shepherd_restore_receipt.v1".to_string(),
        generation: restored.generation,
        population_sha256,
        resident_count: integration.model_bindings.len(),
        capsule_count: integration.capsule_bindings.len(),
        admission_open: true,
        restore_millis: started.elapsed().as_millis() as u64,
        continuation_verified: false,
    };
    if let Err(error) = write_atomic_json(&receipt_path, &receipt) {
        rollback_admission(
            &[],
            &pointer_path,
            prior_pointer.as_deref(),
            &receipt_path,
            prior_receipt.as_deref(),
            &restored_final,
        )?;
        return Err(error).context("prepare restore receipt before admission commit");
    }
    if let Err(error) = write_atomic_json(
        &pointer_path,
        &serde_json::json!({"generation": restored.generation, "path": restored_final, "admission_open": true}),
    ) {
        rollback_admission(
            &[],
            &pointer_path,
            prior_pointer.as_deref(),
            &receipt_path,
            prior_receipt.as_deref(),
            &restored_final,
        )?;
        return Err(error)
            .context("commit global admission pointer before local stop intents clear");
    }
    let mut admitted_specs: Vec<PathBuf> = Vec::new();
    for capsule in &integration.capsule_bindings {
        let spec = restored_final.join(&capsule.agent_id).join("agent.yaml");
        // Include the current spec before the fallible clear: that API removes
        // stop.json before writing its operator event/status, so an error can
        // otherwise leave this agent locally startable.
        admitted_specs.push(spec.clone());
        if let Err(error) = long_lived_agent::clear_stop_for_service_start(
            &spec,
            "resident-shepherd-continuity-restore",
        ) {
            rollback_admission(
                &admitted_specs,
                &pointer_path,
                prior_pointer.as_deref(),
                &receipt_path,
                prior_receipt.as_deref(),
                &restored_final,
            )?;
            return Err(error)
                .context("clear restored population stop intent after global admission commit");
        }
    }
    Ok(receipt)
}

fn capture_all(input: &DehydrationInput, deadline_at: Instant) -> Result<Vec<CapsuleBinding>> {
    capture_all_with(input, deadline_at, |options| {
        capture_capsule(options).map(|_| ())
    })
}

fn capture_all_with(
    input: &DehydrationInput,
    deadline_at: Instant,
    mut capture: impl FnMut(ContinuityCaptureOptions) -> Result<()>,
) -> Result<Vec<CapsuleBinding>> {
    let mut bindings = Vec::new();
    let mut ids = BTreeSet::new();
    for spec_path in &input.existing_agent_specs {
        let result = (|| -> Result<CapsuleBinding> {
            ensure_before_deadline(deadline_at, "load existing agent")?;
            let loaded = long_lived_agent::load_spec(spec_path)?;
            validate_agent_id(&loaded.spec.agent_instance_id)?;
            if !ids.insert(loaded.spec.agent_instance_id.clone()) {
                bail!("duplicate existing agent {}", loaded.spec.agent_instance_id);
            }
            let bundle_ref = format!("capsules/{}", loaded.spec.agent_instance_id);
            let bundle_dir = input.retained_runtime_root.join(&bundle_ref);
            long_lived_agent::status(spec_path)
                .context("validate existing agent checkpoint before dehydration")?;
            long_lived_agent::stop(spec_path, "confirmed Spot interruption dehydration")
                .context("close existing agent admission before capsule capture")?;
            ensure_before_deadline(deadline_at, "capture existing agent capsule")?;
            capture(ContinuityCaptureOptions {
                spec_path: spec_path.clone(),
                out_dir: bundle_dir.clone(),
                source_host: input.source_host.clone(),
                target_host: input.target_host.clone(),
            })?;
            ensure_before_deadline(deadline_at, "finish existing agent capsule")?;
            Ok(CapsuleBinding {
                agent_id: loaded.spec.agent_instance_id,
                bundle_ref,
                manifest_sha256: sha256_file(&bundle_dir.join("continuity_capsule_manifest.json"))?,
                custody_sha256: sha256_file(&bundle_dir.join("custody_manifest.json"))?,
            })
        })();
        match result {
            Ok(binding) => bindings.push(binding),
            Err(error) => return Err(error),
        }
    }
    bindings.sort_by(|a, b| a.agent_id.cmp(&b.agent_id));
    Ok(bindings)
}

fn validate_integration(value: &ResidentContinuityIntegration, root: &Path) -> Result<()> {
    if value.schema != RESIDENT_INTEGRATION_SCHEMA {
        bail!("unsupported resident integration schema");
    }
    value.runtime_v2_snapshot.validate()?;
    validate_models(&value.model_bindings)?;
    let expected_path = [
        RuntimeV2AgentLifecycleState::Active,
        RuntimeV2AgentLifecycleState::Quiescent,
        RuntimeV2AgentLifecycleState::Active,
        RuntimeV2AgentLifecycleState::Suspended,
        RuntimeV2AgentLifecycleState::Dormant,
    ];
    if value.lifecycle_path != expected_path {
        bail!("resident lifecycle path is not ACTIVE->QUIESCENT->ACTIVE then ACTIVE->SUSPENDED->DORMANT");
    }
    if let Some(notice) = &value.spot_notice {
        if notice.source != "aws_imdsv2_spot_instance_action"
            || !matches!(notice.action.as_str(), "terminate" | "stop")
            || notice.deadline_utc.trim().is_empty()
            || notice.notice_sha256.len() != 64
        {
            bail!("invalid IMDSv2 Spot interruption binding");
        }
    }
    let mut ids = BTreeSet::new();
    for binding in &value.capsule_bindings {
        validate_agent_id(&binding.agent_id)?;
        if !ids.insert(&binding.agent_id) {
            bail!("duplicate capsule agent {}", binding.agent_id);
        }
        let bundle = root.join(&binding.bundle_ref);
        if sha256_file(&bundle.join("continuity_capsule_manifest.json"))? != binding.manifest_sha256
            || sha256_file(&bundle.join("custody_manifest.json"))? != binding.custody_sha256
        {
            bail!("capsule binding mismatch for {}", binding.agent_id);
        }
    }
    let resident_ids = value
        .model_bindings
        .iter()
        .map(|binding| binding.agent_id.as_str())
        .collect::<BTreeSet<_>>();
    let capsule_ids = value
        .capsule_bindings
        .iter()
        .map(|binding| binding.agent_id.as_str())
        .collect::<BTreeSet<_>>();
    if resident_ids != capsule_ids || value.model_bindings.len() != value.capsule_bindings.len() {
        bail!("every resident must have exactly one matching CSM capsule binding");
    }
    Ok(())
}

fn validate_restore_bindings(
    integration: &ResidentContinuityIntegration,
    restore_input: &RestoreInput,
) -> Result<()> {
    if digest(
        restore_input
            .retained_runtime_root
            .to_string_lossy()
            .as_bytes(),
    ) != integration.retained_runtime_root_sha256
        || digest(restore_input.build_cache_root.to_string_lossy().as_bytes())
            != integration.build_cache_root_sha256
    {
        bail!("restore Runtime/build-root identity differs from signed dehydration binding");
    }
    if restore_input.residents != integration.model_bindings {
        bail!("restore model/artifact/quantization/configuration bindings are not exact");
    }
    if restore_input.runtime_volume_identity_sha256 != integration.runtime_volume_identity_sha256 {
        bail!("restore retained-volume identity differs from signed dehydration binding");
    }
    Ok(())
}

pub async fn validate_completed_continuation(
    retained_runtime_root: &Path,
    continuation: &ContinuationInput,
) -> Result<RestoreReceipt> {
    let latest: DehydrationReceipt =
        read_json(&retained_runtime_root.join("dehydration-receipt.json"))?;
    let pointer: serde_json::Value =
        read_json(&retained_runtime_root.join("active-population.json"))?;
    if pointer
        .get("admission_open")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        bail!("active population pointer has not committed admission open");
    }
    let mut receipt: RestoreReceipt =
        read_json(&retained_runtime_root.join("restore-receipt.json"))?;
    if !receipt.admission_open {
        bail!("admission is closed; warm continuation is forbidden");
    }
    if pointer
        .get("generation")
        .and_then(serde_json::Value::as_u64)
        != Some(latest.generation)
        || receipt.generation != latest.generation
        || receipt.population_sha256 != latest.population_sha256
    {
        bail!("admission authority does not match the latest dehydration generation");
    }
    let integration: ResidentContinuityIntegration = {
        let mut found = None;
        let mut live = live_continuity(retained_runtime_root, latest.generation)?;
        live.restore_latest_with_resident_population(&RuntimeRecorder::new(32), |bytes| {
            found = serde_json::from_slice(bytes).ok();
            Ok(())
        })
        .await?;
        found.context("signed resident integration subrecord is absent")?
    };
    if continuation.residents.len() != integration.model_bindings.len() {
        bail!("completed continuation population is incomplete");
    }
    for expected in &integration.model_bindings {
        let actual = continuation
            .residents
            .iter()
            .find(|item| item.agent_id == expected.agent_id)
            .with_context(|| format!("missing completed continuation for {}", expected.agent_id))?;
        if actual.model != expected.model
            || actual.artifact_sha256 != expected.artifact_sha256
            || actual.quantization != expected.quantization
            || actual.configuration_sha256 != expected.configuration_sha256
            || actual.completed_task_sha256 != expected.completed_task_sha256
            || actual.continuation_request_sha256 != expected.continuation_request_sha256
            || actual.next_task_sha256.is_empty()
            || actual.next_task_sha256 == actual.completed_task_sha256
        {
            bail!(
                "completed continuation binding mismatch for {}",
                expected.agent_id
            );
        }
    }
    receipt.continuation_verified = true;
    write_atomic_json(
        &retained_runtime_root.join("restore-receipt.json"),
        &receipt,
    )?;
    Ok(receipt)
}

fn close_admission_for_dehydration(retained_runtime_root: &Path) -> Result<()> {
    let prior_generation =
        read_json::<DehydrationReceipt>(&retained_runtime_root.join("dehydration-receipt.json"))
            .map(|receipt| receipt.generation)
            .unwrap_or(0);
    write_closed_admission(retained_runtime_root, prior_generation)
}

fn write_closed_admission(retained_runtime_root: &Path, generation: u64) -> Result<()> {
    write_atomic_json(
        &retained_runtime_root.join("active-population.json"),
        &serde_json::json!({
            "generation": generation,
            "path": null,
            "admission_open": false
        }),
    )?;
    let receipt_path = retained_runtime_root.join("restore-receipt.json");
    if receipt_path.is_file() {
        let mut receipt: RestoreReceipt = read_json(&receipt_path)?;
        receipt.admission_open = false;
        receipt.continuation_verified = false;
        write_atomic_json(&receipt_path, &receipt)?;
    }
    Ok(())
}

pub fn validate_habitability_receipt(path: &Path) -> Result<serde_json::Value> {
    let value: serde_json::Value = read_json(path)?;
    let field_u64 = |name: &str| {
        value
            .get(name)
            .and_then(serde_json::Value::as_u64)
            .with_context(|| format!("habitability receipt missing integer {name}"))
    };
    if value.get("schema").and_then(serde_json::Value::as_str) != Some(RESIDENT_RECEIPT_SCHEMA) {
        bail!("unsupported habitability receipt schema");
    }
    let resident_count = field_u64("resident_count")?;
    if resident_count < 2
        || value
            .get("residents")
            .and_then(serde_json::Value::as_array)
            .map(|items| items.len() as u64)
            != Some(resident_count)
    {
        bail!("habitability receipt population denominator is incomplete");
    }
    let qualification = value
        .get("qualification")
        .and_then(serde_json::Value::as_bool)
        == Some(true);
    let rss_measurement = value
        .get("rss_measurement")
        .and_then(serde_json::Value::as_str);
    let configuration_source = value
        .get("ollama_configuration_source")
        .and_then(serde_json::Value::as_str);
    if qualification
        && (field_u64("peak_ollama_rss_kib")? == 0
            || rss_measurement != Some("exact_managed_ollama_pid")
            || configuration_source != Some("proof_owned_process_environment_and_api_ps"))
    {
        bail!("qualified receipt lacks measured managed-Ollama resources/configuration");
    }
    if !qualification
        && (rss_measurement != Some("unavailable_external_reference_server")
            || configuration_source
                != Some("api_ps_and_request_contract_server_environment_unverified"))
    {
        bail!("reference receipt must preserve unverified server resource/configuration truth");
    }
    if value
        .get("capacity_headroom_pass")
        .and_then(serde_json::Value::as_bool)
        != Some(true)
    {
        bail!("habitability receipt capacity headroom is false");
    }
    if qualification
        && (value
            .get("instance_type")
            .and_then(serde_json::Value::as_str)
            != Some("r7i.2xlarge")
            || field_u64("vcpus")? != 8
            || !(64_000..=65_536).contains(&field_u64("memory_mib")?)
            || field_u64("swap_used_mib")? != 0)
    {
        bail!("qualified receipt does not match exact r7i.2xlarge envelope");
    }
    Ok(serde_json::json!({
        "schema": "adl.runtime.resident_shepherd_habitability_validation.v1",
        "status": "passed",
        "resident_count": resident_count,
        "qualification": value.get("qualification").cloned().unwrap_or(serde_json::Value::Bool(false)),
        "capacity_headroom_pass": true
    }))
}

fn validate_models(models: &[ResidentModelBinding]) -> Result<()> {
    if models.len() < 2 {
        bail!("resident integration requires multiple distinct local agents");
    }
    let mut ids = BTreeSet::new();
    let mut has_llama_baseline = false;
    for model in models {
        validate_agent_id(&model.agent_id)?;
        if !ids.insert(&model.agent_id) {
            bail!("duplicate resident agent {}", model.agent_id);
        }
        has_llama_baseline |= model.model == "llama3.1:8b";
        for (name, value) in [
            ("model", &model.model),
            ("quantization", &model.quantization),
        ] {
            if value.trim().is_empty() {
                bail!("resident {} has empty {}", model.agent_id, name);
            }
        }
        if !model.quantization.starts_with("Q4") {
            bail!(
                "resident {} is not a reviewed Q4 quantization",
                model.agent_id
            );
        }
        for (name, value) in [
            ("artifact_sha256", &model.artifact_sha256),
            ("configuration_sha256", &model.configuration_sha256),
            ("completed_task_sha256", &model.completed_task_sha256),
            (
                "continuation_request_sha256",
                &model.continuation_request_sha256,
            ),
        ] {
            if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
                bail!("resident {} has invalid {}", model.agent_id, name);
            }
        }
    }
    if !has_llama_baseline {
        bail!("resident integration requires pinned llama3.1:8b baseline");
    }
    Ok(())
}

fn validate_agent_id(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 128
        || matches!(value, "." | "..")
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.'))
    {
        bail!("agent id must be one safe portable path component");
    }
    Ok(())
}

fn ensure_before_deadline(deadline_at: Instant, operation: &str) -> Result<()> {
    if Instant::now() >= deadline_at {
        bail!("Spot dehydration deadline expired before {operation}");
    }
    Ok(())
}

fn validate_roots(input: &DehydrationInput) -> Result<()> {
    if input.retained_runtime_root == input.build_cache_root
        || input
            .retained_runtime_root
            .starts_with(&input.build_cache_root)
        || input
            .build_cache_root
            .starts_with(&input.retained_runtime_root)
    {
        bail!("retained Runtime volume must be distinct from build cache");
    }
    if input.runtime_volume_identity_sha256.len() != 64
        || !input
            .runtime_volume_identity_sha256
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit())
    {
        bail!("retained Runtime volume identity requires exact SHA256");
    }
    Ok(())
}

fn live_continuity(root: &Path, minimum_generation: u64) -> Result<LiveContinuity> {
    let secret = std::env::var("ADL_ISSUE414_SIGNING_KEY_HEX")
        .context("ADL_ISSUE414_SIGNING_KEY_HEX is required")?;
    let secret = LiveContinuity::signing_key_from_hex(&secret)?;
    Ok(LiveContinuity::new(
        root.join("live-kernel"),
        "resident-shepherd-continuity",
        &secret,
        LiveKernelSnapshot::new(
            digest("runtime-v2-resident-topology"),
            digest("runtime-v2-resident-configuration"),
            [("runtime-v2".to_string(), "resident-shepherd".to_string())]
                .into_iter()
                .collect(),
        ),
        minimum_generation,
    ))
}

fn digest(value: impl AsRef<[u8]>) -> String {
    hex::encode(Sha256::digest(value.as_ref()))
}

fn sha256_file(path: &Path) -> Result<String> {
    Ok(digest(
        fs::read(path).with_context(|| format!("read {}", path.display()))?,
    ))
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    serde_json::from_slice(&fs::read(path).with_context(|| format!("read {}", path.display()))?)
        .with_context(|| format!("parse {}", path.display()))
}

fn write_atomic_json(path: &Path, value: &impl Serialize) -> Result<()> {
    let parent = path.parent().context("receipt path has no parent")?;
    fs::create_dir_all(parent)?;
    let tmp = path.with_extension("json.tmp");
    fs::write(&tmp, serde_json::to_vec_pretty(value)?)?;
    fs::rename(tmp, path)?;
    Ok(())
}

fn restore_file(path: &Path, prior: Option<&[u8]>) -> Result<()> {
    if let Some(bytes) = prior {
        let parent = path.parent().context("rollback path has no parent")?;
        let tmp = path.with_extension("json.rollback-tmp");
        fs::write(&tmp, bytes)?;
        fs::rename(tmp, path)?;
        fs::File::open(parent)?.sync_all()?;
    } else if path.exists() {
        fs::remove_file(path)?;
    }
    Ok(())
}

fn rollback_admission(
    admitted_specs: &[PathBuf],
    pointer_path: &Path,
    prior_pointer: Option<&[u8]>,
    receipt_path: &Path,
    prior_receipt: Option<&[u8]>,
    restored_generation: &Path,
) -> Result<()> {
    let mut failures = Vec::new();
    for admitted in admitted_specs {
        if let Err(error) =
            long_lived_agent::stop(admitted, "resident population admission rollback")
        {
            failures.push(format!("re-stop {}: {error:#}", admitted.display()));
        }
    }
    if let Err(error) = restore_file(pointer_path, prior_pointer) {
        failures.push(format!("restore active pointer: {error:#}"));
    }
    if let Err(error) = restore_file(receipt_path, prior_receipt) {
        failures.push(format!("restore prior receipt: {error:#}"));
    }
    if let Err(error) = fs::remove_dir_all(restored_generation) {
        failures.push(format!("remove failed generation: {error}"));
    }
    if !failures.is_empty() {
        bail!(
            "fail-closed admission rollback failed: {}",
            failures.join("; ")
        );
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use base64::Engine;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::thread;

    static NEXT_TEST: AtomicU64 = AtomicU64::new(1);

    fn binding(id: &str) -> ResidentModelBinding {
        ResidentModelBinding {
            agent_id: id.to_string(),
            model: if id == "a" {
                "llama3.1:8b".to_string()
            } else {
                format!("model-{id}")
            },
            artifact_sha256: digest(format!("artifact-{id}")),
            quantization: "Q4_K_M".to_string(),
            configuration_sha256: digest(format!("config-{id}")),
            completed_task_sha256: digest(format!("cold-{id}")),
            continuation_request_sha256: digest(format!("next-{id}")),
        }
    }

    fn existing_agent(base: &Path, id: &str) -> PathBuf {
        let root = base.join(id);
        fs::create_dir_all(&root).unwrap();
        let spec = root.join("agent.yaml");
        fs::write(
            &spec,
            format!(
                "schema: adl.long_lived_agent_spec.v1\nagent_instance_id: {id}\ndisplay_name: {id}\nstate_root: state\nworkflow:\n  kind: demo_adapter\nheartbeat:\n  stale_lease_after_secs: 60\n"
            ),
        )
        .unwrap();
        long_lived_agent::status(&spec).unwrap();
        let daemon_spec = spec.clone();
        thread::spawn(move || {
            long_lived_agent::daemon(
                &daemon_spec,
                long_lived_agent::DaemonOptions {
                    bounded_test_restart_limit: Some(0),
                    checkpoint_interval_secs: 1,
                    interval_secs: Some(1),
                    api_bind: None,
                    no_sleep: true,
                    recover_stale_lease: false,
                    api_otel_status_path: None,
                    api_otel_log_path: None,
                },
            )
        })
        .join()
        .unwrap()
        .unwrap();
        spec
    }

    #[test]
    fn rejects_duplicate_and_partial_population() {
        assert!(validate_models(&[binding("a"), binding("a")]).is_err());
        let snapshot = runtime_v2_snapshot_rehydration_contract().unwrap();
        let integration = ResidentContinuityIntegration {
            schema: RESIDENT_INTEGRATION_SCHEMA.to_string(),
            runtime_v2_snapshot: snapshot.snapshot,
            model_bindings: vec![binding("a"), binding("b")],
            capsule_bindings: vec![],
            lifecycle_path: vec![
                RuntimeV2AgentLifecycleState::Active,
                RuntimeV2AgentLifecycleState::Quiescent,
                RuntimeV2AgentLifecycleState::Active,
                RuntimeV2AgentLifecycleState::Suspended,
                RuntimeV2AgentLifecycleState::Dormant,
            ],
            retained_runtime_root_sha256: digest("runtime"),
            build_cache_root_sha256: digest("build"),
            runtime_volume_identity_sha256: digest("volume"),
            spot_notice: None,
        };
        assert!(validate_integration(&integration, Path::new("runtime")).is_err());
        assert!(validate_restore_bindings(
            &integration,
            &RestoreInput {
                residents: vec![],
                retained_runtime_root: PathBuf::from("runtime"),
                build_cache_root: PathBuf::from("build"),
                runtime_volume_identity_sha256: digest("volume"),
            },
        )
        .is_err());
    }

    #[test]
    fn uses_existing_runtime_v2_authorities_and_exact_lifecycle() {
        let artifacts = runtime_v2_snapshot_rehydration_contract().unwrap();
        artifacts.validate().unwrap();
        let lifecycle = runtime_v2_agent_lifecycle_state_model().unwrap();
        lifecycle.validate().unwrap();
        let mut state = RuntimeV2AgentLifecycleState::Active;
        state
            .transition(RuntimeV2AgentLifecycleState::Suspended)
            .unwrap();
        state
            .transition(RuntimeV2AgentLifecycleState::Dormant)
            .unwrap();
        assert_eq!(state, RuntimeV2AgentLifecycleState::Dormant);
        let population = runtime_v2_snapshot_rehydration_for_active_citizens(&[
            "resident-a".to_string(),
            "resident-b".to_string(),
        ])
        .unwrap();
        assert_eq!(
            population.rehydration_report.restored_active_citizens,
            ["resident-a", "resident-b"]
        );
    }

    #[test]
    fn rejects_agent_ids_that_can_escape_population_roots() {
        for unsafe_id in ["../escape", "a/b", "/absolute", ".", "..", "a\\b"] {
            assert!(
                validate_agent_id(unsafe_id).is_err(),
                "accepted {unsafe_id}"
            );
        }
        assert!(validate_agent_id("resident-a.1_ok").is_ok());
    }

    #[test]
    fn delayed_second_capture_preserves_closed_admission_and_all_stops() {
        let id = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("issue414-capture-rollback-{id}"));
        let _ = fs::remove_dir_all(&base);
        let specs = vec![
            existing_agent(&base, "existing-a"),
            existing_agent(&base, "existing-b"),
        ];
        let input = DehydrationInput {
            residents: vec![binding("a"), binding("b")],
            existing_agent_specs: specs.clone(),
            retained_runtime_root: base.join("retained"),
            build_cache_root: base.join("build"),
            runtime_volume_identity_sha256: digest("volume"),
            source_host: "source".to_string(),
            target_host: "target".to_string(),
            spot_notice: None,
        };
        let mut calls = 0;
        let result = capture_all_with(
            &input,
            Instant::now() + Duration::from_millis(100),
            |options| {
                calls += 1;
                if calls == 2 {
                    thread::sleep(Duration::from_millis(150));
                }
                fs::create_dir_all(&options.out_dir)?;
                fs::write(
                    options.out_dir.join("continuity_capsule_manifest.json"),
                    b"{}",
                )?;
                fs::write(options.out_dir.join("custody_manifest.json"), b"{}")?;
                Ok(())
            },
        );
        assert!(result.is_err());
        for spec in &specs {
            assert!(
                long_lived_agent::stop_requested(spec).unwrap(),
                "stop was not preserved for {}",
                spec.display()
            );
        }
        fs::remove_dir_all(base).unwrap();
    }

    #[test]
    fn admission_rollback_restops_current_agent_and_restores_durable_authority() {
        let id = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("issue414-admission-rollback-{id}"));
        let _ = fs::remove_dir_all(&base);
        let specs = vec![
            existing_agent(&base, "restored-a"),
            existing_agent(&base, "restored-b"),
        ];
        let pointer = base.join("active-population.json");
        let receipt = base.join("restore-receipt.json");
        let generation = base.join("generation-failed");
        fs::create_dir_all(&generation).unwrap();
        fs::write(&pointer, b"{\"prior\":true}").unwrap();
        fs::write(&receipt, b"{\"prior\":true}").unwrap();
        let prior_pointer = fs::read(&pointer).unwrap();
        let prior_receipt = fs::read(&receipt).unwrap();
        fs::write(&pointer, b"{\"admission_open\":true}").unwrap();
        fs::write(&receipt, b"{\"admission_open\":true}").unwrap();
        rollback_admission(
            &specs,
            &pointer,
            Some(&prior_pointer),
            &receipt,
            Some(&prior_receipt),
            &generation,
        )
        .unwrap();
        assert_eq!(fs::read(&pointer).unwrap(), prior_pointer);
        assert_eq!(fs::read(&receipt).unwrap(), prior_receipt);
        assert!(!generation.exists());
        for spec in &specs {
            assert!(long_lived_agent::stop_requested(spec).unwrap());
        }
        fs::remove_dir_all(base).unwrap();
    }

    #[tokio::test]
    async fn signed_restore_validates_exact_models_before_admission() {
        std::env::set_var(
            "ADL_ISSUE414_SIGNING_KEY_HEX",
            "9999999999999999999999999999999999999999999999999999999999999999",
        );
        let custody_private_key = "CQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQkJCQk=";
        let key_bytes = base64::engine::general_purpose::STANDARD
            .decode(custody_private_key)
            .unwrap();
        let signing = p256::ecdsa::SigningKey::from_slice(&key_bytes).unwrap();
        let custody_public_key = base64::engine::general_purpose::STANDARD
            .encode(signing.verifying_key().to_encoded_point(false).as_bytes());
        std::env::set_var(
            "ADL_CSM_CUSTODY_P256_SIGNING_PRIVATE_KEY_B64",
            custody_private_key,
        );
        std::env::set_var("ADL_CSM_CUSTODY_SIGNING_KEY_ID", "issue414-test-key");
        std::env::set_var(
            "ADL_CSM_CUSTODY_TRUSTED_P256_PUBLIC_KEY_B64",
            custody_public_key,
        );
        let id = NEXT_TEST.fetch_add(1, Ordering::Relaxed);
        let base = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target")
            .join(format!("issue414-continuity-test-{id}"));
        let _ = fs::remove_dir_all(&base);
        let runtime_root = base.join("retained-runtime");
        let build_root = base.join("build-cache");
        let input = DehydrationInput {
            residents: vec![binding("a"), binding("b")],
            existing_agent_specs: vec![existing_agent(&base, "a"), existing_agent(&base, "b")],
            retained_runtime_root: runtime_root.clone(),
            build_cache_root: build_root.clone(),
            runtime_volume_identity_sha256: digest("test-volume"),
            source_host: "test".to_string(),
            target_host: "local".to_string(),
            spot_notice: None,
        };
        let dehydrated = dehydrate(&input, Duration::from_secs(2)).await.unwrap();
        assert!(!dehydrated.admission_open);
        assert_eq!(dehydrated.resident_count, 2);
        assert_eq!(dehydrated.capsule_count, 2);
        assert!(restore_and_admit(
            &runtime_root,
            &RestoreInput {
                residents: input.residents.clone(),
                retained_runtime_root: runtime_root.clone(),
                build_cache_root: base.join("substituted-build-cache"),
                runtime_volume_identity_sha256: digest("test-volume"),
            },
        )
        .await
        .is_err());
        let restored = restore_and_admit(
            &runtime_root,
            &RestoreInput {
                residents: input.residents.clone(),
                retained_runtime_root: runtime_root.clone(),
                build_cache_root: build_root.clone(),
                runtime_volume_identity_sha256: digest("test-volume"),
            },
        )
        .await
        .unwrap();
        assert!(restored.admission_open);
        assert_eq!(restored.resident_count, 2);
        assert_eq!(restored.capsule_count, 2);
        assert!(runtime_root
            .join("restored-populations/generation-1")
            .is_dir());
        assert!(runtime_root.join("active-population.json").is_file());
        let continuation = ContinuationInput {
            residents: input
                .residents
                .iter()
                .map(|resident| ContinuationResident {
                    agent_id: resident.agent_id.clone(),
                    model: resident.model.clone(),
                    artifact_sha256: resident.artifact_sha256.clone(),
                    quantization: resident.quantization.clone(),
                    configuration_sha256: resident.configuration_sha256.clone(),
                    completed_task_sha256: resident.completed_task_sha256.clone(),
                    continuation_request_sha256: resident.continuation_request_sha256.clone(),
                    next_task_sha256: digest(format!("next-{}", resident.agent_id)),
                })
                .collect(),
        };
        assert!(
            validate_completed_continuation(&runtime_root, &continuation)
                .await
                .unwrap()
                .continuation_verified
        );

        let second_dehydration = dehydrate(&input, Duration::from_secs(2)).await.unwrap();
        assert_eq!(second_dehydration.generation, 2);
        assert!(!second_dehydration.admission_open);
        assert!(
            validate_completed_continuation(&runtime_root, &continuation)
                .await
                .is_err()
        );
        let second_restore = restore_and_admit(
            &runtime_root,
            &RestoreInput {
                residents: input.residents.clone(),
                retained_runtime_root: runtime_root.clone(),
                build_cache_root: build_root.clone(),
                runtime_volume_identity_sha256: digest("test-volume"),
            },
        )
        .await
        .unwrap();
        assert_eq!(second_restore.generation, 2);
        assert!(runtime_root
            .join("restored-populations/generation-2")
            .is_dir());
        let mut substituted = input.residents.clone();
        substituted[0].artifact_sha256 = digest("substituted");
        assert!(restore_and_admit(
            &runtime_root,
            &RestoreInput {
                residents: substituted,
                retained_runtime_root: runtime_root.clone(),
                build_cache_root: build_root,
                runtime_volume_identity_sha256: digest("test-volume"),
            },
        )
        .await
        .is_err());
        fs::remove_dir_all(base).unwrap();
    }
}
