//! Operator-pinned Runtime authority admission. This constructs, but never starts,
//! the production assembly: no adapter operation, listener or time sample runs.
use adl_runtime_kernel::*;
use anyhow::{anyhow, bail, Result};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs,
    io::Read,
    path::{Component, Path},
    sync::Arc,
    time::Duration,
};

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Trust {
    pub schema: String,
    pub identity_key_id: String,
    pub identity_public_key: String,
    pub private_key_id: String,
    pub private_public_key: String,
    pub continuity_key_id: String,
    pub continuity_public_key: String,
    pub identity_generation: u64,
    pub continuity_generation: u64,
    pub projection_generation: u64,
    pub runtime_state_dir: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub schema: String,
    pub identity_record: BirthdayIdentityRecord,
    pub identity_binding: IdentityBinding,
    pub identity_checkpoint: MemoryCheckpoint,
    pub private_record: PrivateStateRecord,
    pub available_projection: BTreeMap<String, String>,
    pub continuity_record: BirthdayContinuityRecord,
    pub continuity_manifests: Vec<CheckpointManifest>,
}

fn read<T: serde::de::DeserializeOwned>(path: &Path, max: u64) -> Result<T> {
    reject_links(path)?;
    let metadata =
        fs::symlink_metadata(path).map_err(|_| anyhow!("authority input unavailable"))?;
    if !metadata.is_file() || metadata.len() > max {
        bail!("authority input bounds rejected");
    }
    let mut bytes = Vec::new();
    fs::File::open(path)
        .map_err(|_| anyhow!("authority input unavailable"))?
        .take(max + 1)
        .read_to_end(&mut bytes)
        .map_err(|_| anyhow!("authority input unreadable"))?;
    if bytes.len() as u64 > max {
        bail!("authority input bounds rejected");
    }
    serde_json::from_slice(&bytes).map_err(|_| anyhow!("authority input invalid"))
}

fn reject_links(path: &Path) -> Result<()> {
    if path.components().any(|c| matches!(c, Component::ParentDir)) {
        bail!("authority path rejected");
    }
    for ancestor in path.ancestors() {
        if ancestor.as_os_str().is_empty() {
            continue;
        }
        match fs::symlink_metadata(ancestor) {
            Ok(m) if m.file_type().is_symlink() => bail!("authority symlink rejected"),
            Ok(_) => (),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => (),
            Err(_) => bail!("authority path unavailable"),
        }
    }
    Ok(())
}

/// Build the same Runtime-owned verifier used by live assembly, with only
/// operator trust material. Exposed for deterministic signed-fixture generation.
pub fn assembly(trust: &Trust, trust_parent: &Path) -> Result<LiveAssembly> {
    if trust.schema != "codefriend.palace.trust.v1"
        || [
            trust.identity_generation,
            trust.continuity_generation,
            trust.projection_generation,
        ]
        .contains(&0)
        || [
            &trust.identity_key_id,
            &trust.private_key_id,
            &trust.continuity_key_id,
        ]
        .iter()
        .any(|id| {
            id.is_empty()
                || id.len() > 128
                || !id
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
        })
        || trust.runtime_state_dir.is_empty()
        || trust.runtime_state_dir.len() > 256
        || !Path::new(&trust.runtime_state_dir)
            .components()
            .all(|c| matches!(c, Component::Normal(_)))
        || trust.runtime_state_dir.contains('\\')
    {
        bail!("authority trust contract rejected");
    }
    let identity = verifying_key_from_hex(&trust.identity_public_key)
        .map_err(|_| anyhow!("authority public key invalid"))?;
    let private = verifying_key_from_hex(&trust.private_public_key)
        .map_err(|_| anyhow!("authority public key invalid"))?;
    let continuity = verifying_key_from_hex(&trust.continuity_public_key)
        .map_err(|_| anyhow!("authority public key invalid"))?;
    if identity == private || identity == continuity || private == continuity {
        bail!("authority keys must be distinct");
    }
    let state = std::path::absolute(trust_parent.join(&trust.runtime_state_dir))
        .map_err(|_| anyhow!("authority path unavailable"))?;
    reject_links(&state)?;
    let recorder = RuntimeRecorder::new(64);
    let executors = build_production_operation_executors_with_recorder(state, recorder.clone())
        .map_err(|_| anyhow!("Runtime authority state unavailable"))?;
    build_live_assembly(LiveBindings {
        recorder: recorder.clone(),
        canonical_ingress_capacity: 64,
        operation_executors: executors,
        permit_keys: BTreeMap::from([(trust.identity_key_id.clone(), identity)]),
        birthday_authority: birthday_authority_bootstrap_from_runtime_keys(
            &trust.identity_key_id,
            identity,
            &trust.private_key_id,
            private,
            &trust.continuity_key_id,
            continuity,
            trust.identity_generation,
            trust.continuity_generation,
            trust.projection_generation,
        ),
        reasoning: bootstrap_reasoning_services(recorder)
            .map_err(|_| anyhow!("Runtime authority assembly unavailable"))?,
        // Assembly is not started; this source is never sampled.
        time_source: Arc::new(RsntpTimeSampleSource::new("localhost")),
        time_bounds: TimeQualificationBounds {
            timeout: Duration::from_secs(1),
            max_offset: Duration::ZERO,
            max_round_trip: Duration::ZERO,
            retry_delay: Duration::from_millis(10),
            refresh_interval: Duration::from_secs(60),
        },
    })
    .map_err(|_| anyhow!("Runtime authority assembly rejected"))
}

/// Trust is an independently selected operator file, never taken from evidence.
pub fn provision(trust_path: &Path, evidence_path: &Path) -> Result<VerifiedMemoryPalaceAuthority> {
    let trust: Trust = read(trust_path, 16 * 1024)?;
    let evidence: Evidence = read(evidence_path, 2 * 1024 * 1024)?;
    if evidence.schema != "codefriend.palace.authority-evidence.v1"
        || evidence.continuity_manifests.is_empty()
        || evidence.continuity_manifests.len() > 64
        || evidence.available_projection.len() > 32
    {
        bail!("authority evidence bounds rejected");
    }
    let live = assembly(&trust, trust_path.parent().unwrap_or(Path::new(".")))?;
    live.provision_memory_palace_authority(MemoryPalaceAuthorityEvidence {
        identity_record: &evidence.identity_record,
        identity_binding: &evidence.identity_binding,
        identity_checkpoint: &evidence.identity_checkpoint,
        private_record: &evidence.private_record,
        private_lineage: &mut PrivateStateLineage::default(),
        available_projection: &evidence.available_projection,
        continuity_record: &evidence.continuity_record,
        continuity_manifests: &evidence.continuity_manifests,
    })
    .map_err(|_| anyhow!("Runtime Memory Palace authority rejected"))
}
