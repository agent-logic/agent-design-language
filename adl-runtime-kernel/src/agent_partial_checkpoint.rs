use crate::{
    AgentArchiveState, AgentConversationCheckpoint, AgentPartialCheckpointInitConfig,
    AgentSnapshotState, CheckpointAgentSample,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
    sync::Mutex,
};
use thiserror::Error;

pub const AGENT_PARTIAL_CHECKPOINT_SCHEMA: &str = "adl.runtime_v3.agent_partial_checkpoint.v1";
pub const AGENT_PARTIAL_TOMBSTONE_SCHEMA: &str = "adl.runtime_v3.agent_partial_tombstone.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentPartialCheckpoint {
    pub schema: String,
    pub runtime_instance_id: String,
    pub polis_id: String,
    pub agent_id: String,
    pub agent_name: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub snapshot_sequence: u64,
    pub cadence_sequence: u64,
    pub parent_checkpoint_generation: u64,
    pub parent_checkpoint_digest: String,
    pub source_runtime_incarnation_id: String,
    pub created_at_unix_millis: u64,
    pub roster_state: CheckpointAgentSample,
    pub conversation_history: Vec<AgentConversationCheckpoint>,
    pub payload_sha256: String,
    pub checkpoint_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentPartialTombstone {
    pub schema: String,
    pub runtime_instance_id: String,
    pub polis_id: String,
    pub agent_id: String,
    pub snapshot_sequence: u64,
    pub parent_checkpoint_generation: u64,
    pub parent_checkpoint_digest: String,
    pub source_runtime_incarnation_id: String,
    pub created_at_unix_millis: u64,
    pub tombstone_digest: String,
}

enum StoredRecord {
    Partial(AgentPartialCheckpoint),
    Tombstone(AgentPartialTombstone),
}

impl StoredRecord {
    fn agent_id(&self) -> &str {
        match self {
            Self::Partial(value) => &value.agent_id,
            Self::Tombstone(value) => &value.agent_id,
        }
    }

    fn sequence(&self) -> u64 {
        match self {
            Self::Partial(value) => value.snapshot_sequence,
            Self::Tombstone(value) => value.snapshot_sequence,
        }
    }

    fn digest(&self) -> &str {
        match self {
            Self::Partial(value) => &value.checkpoint_digest,
            Self::Tombstone(value) => &value.tombstone_digest,
        }
    }

    fn validate(
        &self,
        runtime_instance_id: &str,
        polis_id: &str,
        parent: Option<(u64, &str)>,
    ) -> Result<(), AgentPartialError> {
        match self {
            Self::Partial(value) => {
                validate_partial_identity(value, runtime_instance_id, polis_id, parent)
            }
            Self::Tombstone(value) => {
                validate_tombstone(value, runtime_instance_id, polis_id, parent)
            }
        }
    }

    fn object_key(&self) -> String {
        let suffix = match self {
            Self::Partial(_) => "json",
            Self::Tombstone(_) => "tombstone.json",
        };
        format!(
            "v1/polis/{}/runtime/{}/agent/{}/sequence/{:020}.{suffix}",
            digest_label(match self {
                Self::Partial(value) => &value.polis_id,
                Self::Tombstone(value) => &value.polis_id,
            }),
            digest_label(match self {
                Self::Partial(value) => &value.runtime_instance_id,
                Self::Tombstone(value) => &value.runtime_instance_id,
            }),
            digest_label(self.agent_id()),
            self.sequence(),
        )
    }
}

#[derive(Clone, Debug)]
pub struct AgentPartialCapture {
    pub agent_id: String,
    pub agent_name: String,
    pub provider: Option<String>,
    pub model: Option<String>,
    pub cadence_sequence: u64,
    pub parent_checkpoint_generation: u64,
    pub parent_checkpoint_digest: String,
    pub created_at_unix_millis: u64,
    pub roster_state: CheckpointAgentSample,
    pub conversation_history: Vec<AgentConversationCheckpoint>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct AgentContinuityProjection {
    pub last_snapshot_at_unix_millis: Option<u64>,
    pub last_archive_at_unix_millis: Option<u64>,
    pub snapshot_sequence: Option<u64>,
    pub pending_archive_count: u64,
    pub snapshot_state: AgentSnapshotState,
    pub archive_state: AgentArchiveState,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct ArchiveDegradedMarker {
    agent_id: String,
    dropped_archive_intervals: u64,
    consecutive_failures: u32,
    retry_after_unix_millis: u64,
}

#[derive(Default)]
struct StoreState {
    next_sequence: BTreeMap<String, u64>,
    projections: BTreeMap<String, AgentContinuityProjection>,
    cadence_sequence: u64,
    in_flight: BTreeSet<String>,
    archive_failures: u32,
    archive_retry_after_unix_millis: u64,
}

#[derive(Debug, Error)]
pub enum AgentPartialError {
    #[error("partial checkpoint encoding failed")]
    Encoding,
    #[error("partial checkpoint exceeds configured size")]
    TooLarge,
    #[error("local_store_saturated")]
    LocalStoreSaturated,
    #[error("spool_saturated")]
    SpoolSaturated,
    #[error("partial checkpoint identity or lineage is invalid")]
    InvalidLineage,
    #[error("partial checkpoint I/O failed: {0}")]
    Io(#[from] io::Error),
    #[error("S3 archive command failed")]
    ArchiveCommand,
}

pub struct AgentPartialCheckpointStore {
    root: PathBuf,
    local_root: PathBuf,
    spool_root: PathBuf,
    receipts_root: PathBuf,
    archive_state_root: PathBuf,
    config: AgentPartialCheckpointInitConfig,
    runtime_instance_id: String,
    polis_id: String,
    runtime_incarnation_id: String,
    state: Mutex<StoreState>,
    archive_command: PathBuf,
}

impl AgentPartialCheckpointStore {
    pub fn open(
        root: PathBuf,
        config: AgentPartialCheckpointInitConfig,
        runtime_instance_id: impl Into<String>,
        polis_id: impl Into<String>,
        runtime_incarnation_id: impl Into<String>,
    ) -> Result<Self, AgentPartialError> {
        let local_root = root.join("local");
        let spool_root = root.join("spool");
        let receipts_root = root.join("receipts");
        let archive_state_root = root.join("archive-state");
        fs::create_dir_all(&local_root)?;
        fs::create_dir_all(&spool_root)?;
        fs::create_dir_all(&receipts_root)?;
        fs::create_dir_all(&archive_state_root)?;
        let store = Self {
            root,
            local_root,
            spool_root,
            receipts_root,
            archive_state_root,
            config,
            runtime_instance_id: runtime_instance_id.into(),
            polis_id: polis_id.into(),
            runtime_incarnation_id: runtime_incarnation_id.into(),
            state: Mutex::new(StoreState::default()),
            archive_command: PathBuf::from("aws"),
        };
        store.rebuild_state()?;
        Ok(store)
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_archive_command(&mut self, command: PathBuf) {
        self.archive_command = command;
    }

    pub fn enabled(&self) -> bool {
        self.config.enabled
    }

    pub fn interval_seconds(&self) -> u64 {
        self.config.interval_seconds
    }

    pub fn snapshot_concurrency(&self) -> usize {
        self.config.snapshot_concurrency
    }

    pub fn archive_enabled(&self) -> bool {
        self.config.s3_archive.is_some()
    }

    pub fn next_cadence_sequence(&self) -> u64 {
        let mut state = self.state.lock().expect("partial store state poisoned");
        state.cadence_sequence = state.cadence_sequence.saturating_add(1);
        state.cadence_sequence
    }

    pub fn mark_overdue(&self, agent_id: &str) {
        let mut state = self.state.lock().expect("partial store state poisoned");
        state
            .projections
            .entry(agent_id.to_owned())
            .or_default()
            .snapshot_state = AgentSnapshotState::Overdue;
    }

    pub fn begin_capture(&self, agent_id: &str) -> bool {
        let mut state = self.state.lock().expect("partial store state poisoned");
        if !state.in_flight.insert(agent_id.to_owned()) {
            state
                .projections
                .entry(agent_id.to_owned())
                .or_default()
                .snapshot_state = AgentSnapshotState::Overdue;
            return false;
        }
        state
            .projections
            .entry(agent_id.to_owned())
            .or_default()
            .snapshot_state = AgentSnapshotState::Overdue;
        true
    }

    pub fn finish_capture(&self, agent_id: &str) {
        self.state
            .lock()
            .expect("partial store state poisoned")
            .in_flight
            .remove(agent_id);
    }

    pub fn mark_failed(&self, agent_id: &str) {
        let mut state = self.state.lock().expect("partial store state poisoned");
        state
            .projections
            .entry(agent_id.to_owned())
            .or_default()
            .snapshot_state = AgentSnapshotState::Failed;
    }

    pub fn projection(&self, agent_id: &str) -> AgentContinuityProjection {
        let state = self.state.lock().expect("partial store state poisoned");
        let mut projection = state.projections.get(agent_id).cloned().unwrap_or_default();
        if self.config.s3_archive.is_some()
            && projection.archive_state == AgentArchiveState::Disabled
        {
            projection.archive_state = AgentArchiveState::Current;
        }
        projection
    }

    pub fn write_partial(
        &self,
        capture: AgentPartialCapture,
    ) -> Result<AgentContinuityProjection, AgentPartialError> {
        let sequence = {
            let mut state = self.state.lock().expect("partial store state poisoned");
            let next = state
                .next_sequence
                .entry(capture.agent_id.clone())
                .or_insert(1);
            let sequence = *next;
            *next = next.saturating_add(1);
            sequence
        };
        let payload_sha256 = payload_digest(&capture)?;
        let mut partial = AgentPartialCheckpoint {
            schema: AGENT_PARTIAL_CHECKPOINT_SCHEMA.to_owned(),
            runtime_instance_id: self.runtime_instance_id.clone(),
            polis_id: self.polis_id.clone(),
            agent_id: capture.agent_id.clone(),
            agent_name: capture.agent_name,
            provider: capture.provider,
            model: capture.model,
            snapshot_sequence: sequence,
            cadence_sequence: capture.cadence_sequence,
            parent_checkpoint_generation: capture.parent_checkpoint_generation,
            parent_checkpoint_digest: capture.parent_checkpoint_digest,
            source_runtime_incarnation_id: self.runtime_incarnation_id.clone(),
            created_at_unix_millis: capture.created_at_unix_millis,
            roster_state: capture.roster_state,
            conversation_history: capture.conversation_history,
            payload_sha256,
            checkpoint_digest: String::new(),
        };
        partial.checkpoint_digest = canonical_digest(&partial)?;
        let bytes = serde_json::to_vec(&partial).map_err(|_| AgentPartialError::Encoding)?;
        if bytes.len() as u64 > self.config.max_partial_bytes {
            self.mark_failed(&partial.agent_id);
            return Err(AgentPartialError::TooLarge);
        }
        self.prune_local_for(&partial.agent_id)?;
        self.prune_global_for_write(bytes.len() as u64)?;
        if !fits(
            &self.local_root,
            self.config.local_max_bytes,
            self.config.local_max_files,
            bytes.len() as u64,
        )? {
            self.mark_failed(&partial.agent_id);
            return Err(AgentPartialError::LocalStoreSaturated);
        }
        let agent_digest = digest_label(&partial.agent_id);
        let local_dir = self.local_root.join(&agent_digest);
        fs::create_dir_all(&local_dir)?;
        let filename = format!("{:020}.json", partial.snapshot_sequence);
        let local_path = local_dir.join(&filename);
        persist_bytes_atomically(&local_path, &bytes)?;

        let mut archive_state = AgentArchiveState::Disabled;
        let mut pending = 0;
        if self.config.s3_archive.is_some() {
            let coalesced = self.coalesce_spool_for(&agent_digest)?;
            if coalesced > 0 {
                self.persist_archive_degraded(&partial.agent_id, true)?;
            }
            if fits(
                &self.spool_root,
                self.config.spool_max_bytes,
                self.config.spool_max_files,
                bytes.len() as u64,
            )? {
                let spool_dir = self.spool_root.join(&agent_digest);
                fs::create_dir_all(&spool_dir)?;
                persist_bytes_atomically(&spool_dir.join(filename), &bytes)?;
                archive_state = if coalesced > 0 {
                    AgentArchiveState::Degraded
                } else {
                    AgentArchiveState::Pending
                };
                pending = 1;
            } else {
                archive_state = AgentArchiveState::SpoolSaturated;
            }
        }
        let projection = AgentContinuityProjection {
            last_snapshot_at_unix_millis: Some(partial.created_at_unix_millis),
            last_archive_at_unix_millis: self
                .state
                .lock()
                .expect("partial store state poisoned")
                .projections
                .get(&partial.agent_id)
                .and_then(|projection| projection.last_archive_at_unix_millis),
            snapshot_sequence: Some(partial.snapshot_sequence),
            pending_archive_count: pending,
            snapshot_state: AgentSnapshotState::Current,
            archive_state,
        };
        self.state
            .lock()
            .expect("partial store state poisoned")
            .projections
            .insert(partial.agent_id, projection.clone());
        Ok(projection)
    }

    pub fn write_tombstone(
        &self,
        agent_id: &str,
        parent_checkpoint_generation: u64,
        parent_checkpoint_digest: String,
    ) -> Result<(), AgentPartialError> {
        if !self.enabled() {
            return Ok(());
        }
        let sequence = {
            let mut state = self.state.lock().expect("partial store state poisoned");
            let next = state.next_sequence.entry(agent_id.to_owned()).or_insert(1);
            let sequence = *next;
            *next = next.saturating_add(1);
            sequence
        };
        let mut tombstone = AgentPartialTombstone {
            schema: AGENT_PARTIAL_TOMBSTONE_SCHEMA.to_owned(),
            runtime_instance_id: self.runtime_instance_id.clone(),
            polis_id: self.polis_id.clone(),
            agent_id: agent_id.to_owned(),
            snapshot_sequence: sequence,
            parent_checkpoint_generation,
            parent_checkpoint_digest,
            source_runtime_incarnation_id: self.runtime_incarnation_id.clone(),
            created_at_unix_millis: now_unix_millis(),
            tombstone_digest: String::new(),
        };
        tombstone.tombstone_digest = canonical_tombstone_digest(&tombstone)?;
        let bytes = serde_json::to_vec(&tombstone).map_err(|_| AgentPartialError::Encoding)?;
        self.prune_local_for(agent_id)?;
        self.prune_global_for_write(bytes.len() as u64)?;
        let agent_digest = digest_label(agent_id);
        let filename = format!("{:020}.tombstone.json", sequence);
        persist_bytes_atomically(&self.local_root.join(&agent_digest).join(&filename), &bytes)?;
        if self.config.s3_archive.is_some() {
            let coalesced = self.coalesce_spool_for(&agent_digest)?;
            if coalesced > 0 {
                self.persist_archive_degraded(agent_id, true)?;
            }
            if !fits(
                &self.spool_root,
                self.config.spool_max_bytes,
                self.config.spool_max_files,
                bytes.len() as u64,
            )? {
                // The durable local tombstone is authoritative for removal.
                // Losing an archive interval must degrade archival, not keep a
                // removed agent resident or resurrect its earlier state.
                self.persist_archive_degraded(agent_id, true)?;
            } else {
                persist_bytes_atomically(
                    &self.spool_root.join(agent_digest).join(filename),
                    &bytes,
                )?;
            }
        }
        self.state
            .lock()
            .expect("partial store state poisoned")
            .projections
            .remove(agent_id);
        Ok(())
    }

    pub async fn archive_pending(&self) -> Result<usize, AgentPartialError> {
        let Some(archive) = self.config.s3_archive.as_ref() else {
            return Ok(0);
        };
        if self
            .state
            .lock()
            .expect("partial store state poisoned")
            .archive_retry_after_unix_millis
            > now_unix_millis()
        {
            return Ok(0);
        }
        let mut archived = 0;
        let mut paths = json_files(&self.spool_root)?;
        paths.sort();
        for path in paths {
            let bytes = fs::read(&path)?;
            let record = decode_record(&bytes)?;
            record.validate(&self.runtime_instance_id, &self.polis_id, None)?;
            let key = record.object_key();
            let mut upload = tokio::process::Command::new(&self.archive_command);
            upload
                .env("AWS_CLI_CONNECT_TIMEOUT", "3")
                .env("AWS_CLI_READ_TIMEOUT", "10");
            let output = upload
                .args([
                    "s3api",
                    "put-object",
                    "--region",
                    &archive.region,
                    "--bucket",
                    &archive.bucket,
                    "--key",
                    &key,
                    "--body",
                ])
                .arg(&path)
                .args([
                    "--server-side-encryption",
                    "aws:kms",
                    "--ssekms-key-id",
                    &archive.kms_key_arn,
                    "--metadata",
                    &format!("sha256={}", record.digest()),
                ])
                .output()
                .await?;
            if !output.status.success() {
                self.mark_archive_degraded(record.agent_id());
                self.schedule_archive_retry(record.agent_id())?;
                return Err(AgentPartialError::ArchiveCommand);
            }
            let mut verify = tokio::process::Command::new(&self.archive_command);
            verify
                .env("AWS_CLI_CONNECT_TIMEOUT", "3")
                .env("AWS_CLI_READ_TIMEOUT", "10");
            let verified = verify
                .args([
                    "s3api",
                    "head-object",
                    "--region",
                    &archive.region,
                    "--bucket",
                    &archive.bucket,
                    "--key",
                    &key,
                    "--output",
                    "json",
                ])
                .output()
                .await?;
            let metadata_matches = verified.status.success()
                && serde_json::from_slice::<serde_json::Value>(&verified.stdout)
                    .ok()
                    .and_then(|value| {
                        value["Metadata"]["sha256"]
                            .as_str()
                            .map(|digest| digest == record.digest())
                    })
                    .unwrap_or(false);
            if !metadata_matches {
                self.mark_archive_degraded(record.agent_id());
                self.schedule_archive_retry(record.agent_id())?;
                return Err(AgentPartialError::ArchiveCommand);
            }
            let receipt = serde_json::json!({
                "schema": "adl.runtime_v3.agent_partial_archive_receipt.v1",
                "agent_id": record.agent_id(),
                "snapshot_sequence": record.sequence(),
                "checkpoint_digest": record.digest(),
                "archived_at_unix_millis": now_unix_millis(),
            });
            let receipt_path = self
                .receipts_root
                .join(digest_label(record.agent_id()))
                .join(format!("{:020}.json", record.sequence()));
            let receipt_bytes =
                serde_json::to_vec(&receipt).map_err(|_| AgentPartialError::Encoding)?;
            persist_bytes_atomically(&receipt_path, &receipt_bytes)?;
            fs::remove_file(&path)?;
            let mut state = self.state.lock().expect("partial store state poisoned");
            let projection = state
                .projections
                .entry(record.agent_id().to_owned())
                .or_default();
            projection.last_archive_at_unix_millis = receipt["archived_at_unix_millis"].as_u64();
            projection.pending_archive_count = projection.pending_archive_count.saturating_sub(1);
            projection.archive_state = if projection.pending_archive_count == 0 {
                AgentArchiveState::Current
            } else {
                AgentArchiveState::Pending
            };
            archived += 1;
            drop(state);
            self.clear_archive_degraded(record.agent_id())?;
        }
        if archived > 0 {
            let mut state = self.state.lock().expect("partial store state poisoned");
            state.archive_failures = 0;
            state.archive_retry_after_unix_millis = 0;
        }
        Ok(archived)
    }

    pub fn latest_valid(
        &self,
        parent_generation: u64,
        parent_digest: &str,
    ) -> Result<Vec<AgentPartialCheckpoint>, AgentPartialError> {
        select_latest_records(
            json_files(&self.local_root)?,
            &self.runtime_instance_id,
            &self.polis_id,
            parent_generation,
            parent_digest,
        )
    }

    pub async fn latest_valid_with_archive(
        &self,
        parent_generation: u64,
        parent_digest: &str,
    ) -> Result<Vec<AgentPartialCheckpoint>, AgentPartialError> {
        let mut paths = json_files(&self.local_root)?;
        let Some(archive) = self.config.s3_archive.as_ref() else {
            return select_latest_records(
                paths,
                &self.runtime_instance_id,
                &self.polis_id,
                parent_generation,
                parent_digest,
            );
        };
        let prefix = format!(
            "v1/polis/{}/runtime/{}/agent/",
            digest_label(&self.polis_id),
            digest_label(&self.runtime_instance_id)
        );
        let mut list = tokio::process::Command::new(&self.archive_command);
        add_restore_profile(&mut list, archive);
        list.env("AWS_CLI_CONNECT_TIMEOUT", "3")
            .env("AWS_CLI_READ_TIMEOUT", "10");
        let listed = match list
            .args([
                "s3api",
                "list-objects-v2",
                "--region",
                &archive.region,
                "--bucket",
                &archive.bucket,
                "--prefix",
                &prefix,
                "--output",
                "json",
            ])
            .output()
            .await
        {
            Ok(output) => output,
            Err(_) => {
                return select_latest_records(
                    paths,
                    &self.runtime_instance_id,
                    &self.polis_id,
                    parent_generation,
                    parent_digest,
                );
            }
        };
        if !listed.status.success() {
            return select_latest_records(
                paths,
                &self.runtime_instance_id,
                &self.polis_id,
                parent_generation,
                parent_digest,
            );
        }
        let Ok(listing) = serde_json::from_slice::<serde_json::Value>(&listed.stdout) else {
            return select_latest_records(
                paths,
                &self.runtime_instance_id,
                &self.polis_id,
                parent_generation,
                parent_digest,
            );
        };
        let download_root = self.root.join("restore-downloads");
        fs::create_dir_all(&download_root)?;
        for key in listing["Contents"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|entry| entry["Key"].as_str())
            .filter(|key| key.ends_with(".json"))
        {
            let destination = download_root.join(format!("{}.json", digest_label(key)));
            let mut get = tokio::process::Command::new(&self.archive_command);
            add_restore_profile(&mut get, archive);
            get.env("AWS_CLI_CONNECT_TIMEOUT", "3")
                .env("AWS_CLI_READ_TIMEOUT", "10");
            let Ok(downloaded) = get
                .args([
                    "s3api",
                    "get-object",
                    "--region",
                    &archive.region,
                    "--bucket",
                    &archive.bucket,
                    "--key",
                    key,
                ])
                .arg(&destination)
                .output()
                .await
            else {
                continue;
            };
            if downloaded.status.success() {
                paths.push(destination);
            }
        }
        let selected = select_latest_records(
            paths,
            &self.runtime_instance_id,
            &self.polis_id,
            parent_generation,
            parent_digest,
        );
        let _ = fs::remove_dir_all(download_root);
        selected
    }

    fn rebuild_state(&self) -> Result<(), AgentPartialError> {
        let mut state = self.state.lock().expect("partial store state poisoned");
        let mut local_paths = json_files(&self.local_root)?;
        local_paths.sort();
        for path in local_paths {
            let bytes = fs::read(path)?;
            let Ok(record) = decode_record(&bytes) else {
                continue;
            };
            if record
                .validate(&self.runtime_instance_id, &self.polis_id, None)
                .is_err()
            {
                continue;
            }
            if let StoredRecord::Tombstone(tombstone) = record {
                let next = state
                    .next_sequence
                    .entry(tombstone.agent_id.clone())
                    .or_insert(1);
                *next = (*next).max(tombstone.snapshot_sequence.saturating_add(1));
                state.projections.remove(&tombstone.agent_id);
                continue;
            }
            let StoredRecord::Partial(partial) = record else {
                unreachable!();
            };
            let next = state
                .next_sequence
                .entry(partial.agent_id.clone())
                .or_insert(1);
            *next = (*next).max(partial.snapshot_sequence.saturating_add(1));
            state.cadence_sequence = state.cadence_sequence.max(partial.cadence_sequence);
            let projection = state.projections.entry(partial.agent_id).or_default();
            if projection
                .snapshot_sequence
                .is_none_or(|value| value < partial.snapshot_sequence)
            {
                projection.snapshot_sequence = Some(partial.snapshot_sequence);
                projection.last_snapshot_at_unix_millis = Some(partial.created_at_unix_millis);
                projection.snapshot_state = AgentSnapshotState::Current;
                projection.archive_state = if self.config.s3_archive.is_some() {
                    AgentArchiveState::Pending
                } else {
                    AgentArchiveState::Disabled
                };
            }
        }
        for path in json_files(&self.spool_root)? {
            let bytes = fs::read(path)?;
            if let Ok(StoredRecord::Partial(partial)) = decode_record(&bytes) {
                let projection = state.projections.entry(partial.agent_id).or_default();
                projection.pending_archive_count =
                    projection.pending_archive_count.saturating_add(1);
                projection.archive_state = AgentArchiveState::Pending;
            }
        }
        for path in json_files(&self.receipts_root)? {
            let bytes = fs::read(path)?;
            let Ok(receipt) = serde_json::from_slice::<serde_json::Value>(&bytes) else {
                continue;
            };
            let (Some(agent_id), Some(sequence), Some(archived_at)) = (
                receipt["agent_id"].as_str(),
                receipt["snapshot_sequence"].as_u64(),
                receipt["archived_at_unix_millis"].as_u64(),
            ) else {
                continue;
            };
            let projection = state.projections.entry(agent_id.to_owned()).or_default();
            if projection.snapshot_sequence == Some(sequence)
                && projection.pending_archive_count == 0
            {
                projection.last_archive_at_unix_millis = Some(archived_at);
                projection.archive_state = AgentArchiveState::Current;
            }
        }
        for path in json_files(&self.archive_state_root)? {
            let bytes = fs::read(path)?;
            let Ok(marker) = serde_json::from_slice::<ArchiveDegradedMarker>(&bytes) else {
                continue;
            };
            let projection = state
                .projections
                .entry(marker.agent_id.clone())
                .or_default();
            projection.archive_state = AgentArchiveState::Degraded;
            state.archive_failures = state.archive_failures.max(marker.consecutive_failures);
            state.archive_retry_after_unix_millis = state
                .archive_retry_after_unix_millis
                .max(marker.retry_after_unix_millis);
        }
        Ok(())
    }

    fn prune_local_for(&self, agent_id: &str) -> Result<(), AgentPartialError> {
        let agent_root = self.local_root.join(digest_label(agent_id));
        let mut files = json_files(&agent_root)?;
        files.sort();
        while files.len() >= self.config.retained_partials_per_agent {
            if let Some(path) = files.first().cloned() {
                fs::remove_file(path)?;
                files.remove(0);
            }
        }
        Ok(())
    }

    fn prune_global_for_write(&self, additional_bytes: u64) -> Result<(), AgentPartialError> {
        loop {
            if fits(
                &self.local_root,
                self.config.local_max_bytes,
                self.config.local_max_files,
                additional_bytes,
            )? {
                return Ok(());
            }
            let mut by_parent = BTreeMap::<PathBuf, Vec<PathBuf>>::new();
            for path in json_files(&self.local_root)? {
                if let Some(parent) = path.parent() {
                    by_parent
                        .entry(parent.to_path_buf())
                        .or_default()
                        .push(path);
                }
            }
            let mut candidates = Vec::new();
            for files in by_parent.values_mut() {
                files.sort();
                files.pop();
                candidates.extend(files.iter().cloned());
            }
            candidates.sort();
            let Some(oldest_superseded) = candidates.first() else {
                return Err(AgentPartialError::LocalStoreSaturated);
            };
            fs::remove_file(oldest_superseded)?;
        }
    }

    fn coalesce_spool_for(&self, agent_digest: &str) -> Result<usize, AgentPartialError> {
        let dir = self.spool_root.join(agent_digest);
        let mut files = json_files(&dir)?;
        files.sort();
        let removed = files.len();
        for path in files {
            fs::remove_file(path)?;
        }
        Ok(removed)
    }

    fn mark_archive_degraded(&self, agent_id: &str) {
        let mut state = self.state.lock().expect("partial store state poisoned");
        state
            .projections
            .entry(agent_id.to_owned())
            .or_default()
            .archive_state = AgentArchiveState::Degraded;
    }

    fn schedule_archive_retry(&self, agent_id: &str) -> Result<(), AgentPartialError> {
        let mut state = self.state.lock().expect("partial store state poisoned");
        state.archive_failures = state.archive_failures.saturating_add(1).min(16);
        let exponential = 5u64
            .saturating_mul(1u64 << state.archive_failures.saturating_sub(1).min(6))
            .min(300);
        let jitter = u64::from(
            self.runtime_incarnation_id
                .as_bytes()
                .first()
                .copied()
                .unwrap_or(0),
        ) % (exponential / 5 + 1);
        state.archive_retry_after_unix_millis = now_unix_millis()
            .saturating_add(exponential.saturating_add(jitter).saturating_mul(1_000));
        let failures = state.archive_failures;
        let retry_after = state.archive_retry_after_unix_millis;
        drop(state);
        let mut marker = self.read_archive_degraded(agent_id)?.unwrap_or_default();
        marker.agent_id = agent_id.to_owned();
        marker.consecutive_failures = failures;
        marker.retry_after_unix_millis = retry_after;
        self.write_archive_degraded(&marker)?;
        Ok(())
    }

    fn persist_archive_degraded(
        &self,
        agent_id: &str,
        dropped_interval: bool,
    ) -> Result<(), AgentPartialError> {
        let mut marker = self.read_archive_degraded(agent_id)?.unwrap_or_default();
        marker.agent_id = agent_id.to_owned();
        if dropped_interval {
            marker.dropped_archive_intervals = marker.dropped_archive_intervals.saturating_add(1);
        }
        self.write_archive_degraded(&marker)?;
        Ok(())
    }

    fn read_archive_degraded(
        &self,
        agent_id: &str,
    ) -> Result<Option<ArchiveDegradedMarker>, AgentPartialError> {
        let path = self
            .archive_state_root
            .join(format!("{}.json", digest_label(agent_id)));
        match fs::read(path) {
            Ok(bytes) => serde_json::from_slice(&bytes)
                .map(Some)
                .map_err(|_| AgentPartialError::Encoding),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(error.into()),
        }
    }

    fn write_archive_degraded(
        &self,
        marker: &ArchiveDegradedMarker,
    ) -> Result<(), AgentPartialError> {
        let bytes = serde_json::to_vec(marker).map_err(|_| AgentPartialError::Encoding)?;
        persist_bytes_atomically(
            &self
                .archive_state_root
                .join(format!("{}.json", digest_label(&marker.agent_id))),
            &bytes,
        )?;
        Ok(())
    }

    fn clear_archive_degraded(&self, agent_id: &str) -> Result<(), AgentPartialError> {
        let path = self
            .archive_state_root
            .join(format!("{}.json", digest_label(agent_id)));
        match fs::remove_file(path) {
            Ok(()) => Ok(()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(error.into()),
        }
    }

    pub fn root(&self) -> &Path {
        &self.root
    }
}

fn payload_digest(capture: &AgentPartialCapture) -> Result<String, AgentPartialError> {
    let bytes = serde_jcs::to_vec(&serde_json::json!({
        "roster_state": capture.roster_state,
        "conversation_history": capture.conversation_history,
    }))
    .map_err(|_| AgentPartialError::Encoding)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn payload_digest_from_partial(
    partial: &AgentPartialCheckpoint,
) -> Result<String, AgentPartialError> {
    let bytes = serde_jcs::to_vec(&serde_json::json!({
        "roster_state": partial.roster_state,
        "conversation_history": partial.conversation_history,
    }))
    .map_err(|_| AgentPartialError::Encoding)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn canonical_digest(partial: &AgentPartialCheckpoint) -> Result<String, AgentPartialError> {
    let mut value = serde_json::to_value(partial).map_err(|_| AgentPartialError::Encoding)?;
    value["checkpoint_digest"] = serde_json::Value::String(String::new());
    let bytes = serde_jcs::to_vec(&value).map_err(|_| AgentPartialError::Encoding)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn canonical_tombstone_digest(
    tombstone: &AgentPartialTombstone,
) -> Result<String, AgentPartialError> {
    let mut value = serde_json::to_value(tombstone).map_err(|_| AgentPartialError::Encoding)?;
    value["tombstone_digest"] = serde_json::Value::String(String::new());
    let bytes = serde_jcs::to_vec(&value).map_err(|_| AgentPartialError::Encoding)?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn validate_tombstone(
    tombstone: &AgentPartialTombstone,
    runtime_instance_id: &str,
    polis_id: &str,
    parent: Option<(u64, &str)>,
) -> Result<(), AgentPartialError> {
    if tombstone.schema != AGENT_PARTIAL_TOMBSTONE_SCHEMA
        || tombstone.runtime_instance_id != runtime_instance_id
        || tombstone.polis_id != polis_id
        || tombstone.snapshot_sequence == 0
        || tombstone.tombstone_digest != canonical_tombstone_digest(tombstone)?
        || parent.is_some_and(|(generation, digest)| {
            tombstone.parent_checkpoint_generation != generation
                || tombstone.parent_checkpoint_digest != digest
        })
    {
        return Err(AgentPartialError::InvalidLineage);
    }
    Ok(())
}

fn decode_record(bytes: &[u8]) -> Result<StoredRecord, AgentPartialError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| AgentPartialError::Encoding)?;
    match value["schema"].as_str() {
        Some(AGENT_PARTIAL_CHECKPOINT_SCHEMA) => serde_json::from_value(value)
            .map(StoredRecord::Partial)
            .map_err(|_| AgentPartialError::Encoding),
        Some(AGENT_PARTIAL_TOMBSTONE_SCHEMA) => serde_json::from_value(value)
            .map(StoredRecord::Tombstone)
            .map_err(|_| AgentPartialError::Encoding),
        _ => Err(AgentPartialError::Encoding),
    }
}

fn select_latest_records(
    paths: Vec<PathBuf>,
    runtime_instance_id: &str,
    polis_id: &str,
    parent_generation: u64,
    parent_digest: &str,
) -> Result<Vec<AgentPartialCheckpoint>, AgentPartialError> {
    let mut latest = BTreeMap::<String, StoredRecord>::new();
    let mut observed_sequences = BTreeMap::<(String, u64), String>::new();
    for path in paths {
        let bytes = fs::read(path)?;
        let Ok(record) = decode_record(&bytes) else {
            continue;
        };
        if record
            .validate(
                runtime_instance_id,
                polis_id,
                Some((parent_generation, parent_digest)),
            )
            .is_err()
        {
            continue;
        }
        let sequence_key = (record.agent_id().to_owned(), record.sequence());
        if let Some(existing_digest) = observed_sequences.get(&sequence_key) {
            if existing_digest != record.digest() {
                return Err(AgentPartialError::InvalidLineage);
            }
            continue;
        }
        observed_sequences.insert(sequence_key, record.digest().to_owned());
        let replace = latest
            .get(record.agent_id())
            .is_none_or(|current| current.sequence() < record.sequence());
        if replace {
            latest.insert(record.agent_id().to_owned(), record);
        }
    }
    Ok(latest
        .into_values()
        .filter_map(|record| match record {
            StoredRecord::Partial(partial) => Some(partial),
            StoredRecord::Tombstone(_) => None,
        })
        .collect())
}

fn validate_partial_identity(
    partial: &AgentPartialCheckpoint,
    runtime_instance_id: &str,
    polis_id: &str,
    parent: Option<(u64, &str)>,
) -> Result<(), AgentPartialError> {
    if partial.schema != AGENT_PARTIAL_CHECKPOINT_SCHEMA
        || partial.runtime_instance_id != runtime_instance_id
        || partial.polis_id != polis_id
        || partial.snapshot_sequence == 0
        || partial.payload_sha256 != payload_digest_from_partial(partial)?
        || partial.checkpoint_digest != canonical_digest(partial)?
        || parent.is_some_and(|(generation, digest)| {
            partial.parent_checkpoint_generation != generation
                || partial.parent_checkpoint_digest != digest
        })
    {
        return Err(AgentPartialError::InvalidLineage);
    }
    Ok(())
}

fn digest_label(value: &str) -> String {
    hex::encode(Sha256::digest(value.as_bytes()))
}

fn json_files(root: &Path) -> Result<Vec<PathBuf>, io::Error> {
    if !root.exists() {
        return Ok(Vec::new());
    }
    let mut files = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let file_type = entry.file_type()?;
            if file_type.is_dir() {
                stack.push(entry.path());
            } else if file_type.is_file()
                && entry.path().extension().and_then(|value| value.to_str()) == Some("json")
            {
                files.push(entry.path());
            }
        }
    }
    Ok(files)
}

fn usage(root: &Path) -> Result<(u64, usize), io::Error> {
    let files = json_files(root)?;
    let mut bytes = 0u64;
    for path in &files {
        bytes = bytes.saturating_add(fs::metadata(path)?.len());
    }
    Ok((bytes, files.len()))
}

fn fits(root: &Path, max_bytes: u64, max_files: usize, additional: u64) -> Result<bool, io::Error> {
    let (bytes, files) = usage(root)?;
    Ok(files < max_files && bytes.saturating_add(additional) <= max_bytes)
}

fn persist_bytes_atomically(path: &Path, bytes: &[u8]) -> Result<(), io::Error> {
    let parent = path
        .parent()
        .ok_or_else(|| io::Error::other("missing parent"))?;
    fs::create_dir_all(parent)?;
    let temporary = parent.join(format!(
        ".{}.{}.tmp",
        path.file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("partial"),
        std::process::id()
    ));
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temporary)?;
    file.write_all(bytes)?;
    file.sync_all()?;
    fs::rename(&temporary, path)?;
    File::open(parent)?.sync_all()?;
    Ok(())
}

fn now_unix_millis() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        .try_into()
        .unwrap_or(u64::MAX)
}

fn add_restore_profile(
    command: &mut tokio::process::Command,
    archive: &crate::AgentPartialS3ArchiveInitConfig,
) {
    if let Some(profile) = &archive.restore_profile {
        command.args(["--profile", profile]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;

    fn capture(agent_id: &str, created_at: u64) -> AgentPartialCapture {
        AgentPartialCapture {
            agent_id: agent_id.to_owned(),
            agent_name: format!("{agent_id}.axioma"),
            provider: Some("ollama".to_owned()),
            model: Some("test-model".to_owned()),
            cadence_sequence: created_at,
            parent_checkpoint_generation: 2,
            parent_checkpoint_digest: "a".repeat(64),
            created_at_unix_millis: created_at,
            roster_state: CheckpointAgentSample {
                id: agent_id.to_owned(),
                label: agent_id.to_owned(),
                role: "reasoning".to_owned(),
                state: "ready".to_owned(),
                detail: "ready".to_owned(),
                health: "healthy".to_owned(),
                availability: "available".to_owned(),
                activity: None,
                capabilities: Vec::new(),
                location: None,
                communication_eligible: true,
                observed_at_unix_millis: created_at,
                freshness_deadline_unix_millis: created_at + 30_000,
                source_revision: "test".to_owned(),
                provenance: "test".to_owned(),
            },
            conversation_history: Vec::new(),
        }
    }

    fn store_config(archive: bool) -> AgentPartialCheckpointInitConfig {
        AgentPartialCheckpointInitConfig {
            max_partial_bytes: 1_048_576,
            local_max_bytes: 2_097_152,
            local_max_files: 32,
            retained_partials_per_agent: 2,
            spool_max_bytes: 2_097_152,
            spool_max_files: 32,
            s3_archive: archive.then(|| crate::AgentPartialS3ArchiveInitConfig {
                region: "us-west-2".to_owned(),
                bucket: "example-agent-partials".to_owned(),
                kms_key_arn: "arn:aws:kms:us-west-2:111122223333:key/test".to_owned(),
                restore_profile: None,
            }),
            ..AgentPartialCheckpointInitConfig::default()
        }
    }

    #[test]
    fn object_keys_are_privacy_safe_and_stable() {
        let partial = AgentPartialCheckpoint {
            schema: AGENT_PARTIAL_CHECKPOINT_SCHEMA.to_owned(),
            runtime_instance_id: "runtime-a".to_owned(),
            polis_id: "polis-a".to_owned(),
            agent_id: "secret-agent-id".to_owned(),
            agent_name: "ember.axioma".to_owned(),
            provider: Some("ollama".to_owned()),
            model: Some("gemma4:e4b-mlx".to_owned()),
            snapshot_sequence: 7,
            cadence_sequence: 3,
            parent_checkpoint_generation: 2,
            parent_checkpoint_digest: "a".repeat(64),
            source_runtime_incarnation_id: "incarnation".to_owned(),
            created_at_unix_millis: 1,
            roster_state: CheckpointAgentSample {
                id: "secret-agent-id".to_owned(),
                label: "Ember Axioma".to_owned(),
                role: "reasoning".to_owned(),
                state: "ready".to_owned(),
                detail: "ready".to_owned(),
                health: "healthy".to_owned(),
                availability: "available".to_owned(),
                activity: None,
                capabilities: Vec::new(),
                location: None,
                communication_eligible: true,
                observed_at_unix_millis: 1,
                freshness_deadline_unix_millis: 2,
                source_revision: "test".to_owned(),
                provenance: "test".to_owned(),
            },
            conversation_history: Vec::new(),
            payload_sha256: "b".repeat(64),
            checkpoint_digest: String::new(),
        };
        let key = StoredRecord::Partial(partial).object_key();
        assert!(!key.contains("secret-agent-id"));
        assert!(key.ends_with("00000000000000000007.json"));
    }

    #[test]
    fn partials_are_atomic_bounded_and_restore_highest_valid_sequence() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        for created_at in 1..=3 {
            store.write_partial(capture("ember", created_at)).unwrap();
        }
        let files = json_files(&store.local_root).unwrap();
        assert_eq!(files.len(), 2);
        assert!(files.iter().all(|path| path.extension().unwrap() == "json"));
        let restored = store.latest_valid(2, &"a".repeat(64)).unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].snapshot_sequence, 3);
        assert_eq!(restored[0].created_at_unix_millis, 3);

        let newest = files.into_iter().max().unwrap();
        let mut corrupted = fs::read(&newest).unwrap();
        let index = corrupted.len() / 2;
        corrupted[index] ^= 1;
        fs::write(newest, corrupted).unwrap();
        let restored = store.latest_valid(2, &"a".repeat(64)).unwrap();
        assert_eq!(restored[0].snapshot_sequence, 2);
    }

    #[test]
    fn spool_coalesces_to_newest_and_reports_degraded_history() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(true),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        let projection = store.write_partial(capture("ember", 2)).unwrap();
        assert_eq!(json_files(&store.spool_root).unwrap().len(), 1);
        assert_eq!(projection.pending_archive_count, 1);
        assert_eq!(projection.archive_state, AgentArchiveState::Degraded);
        drop(store);
        let reopened = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(true),
            "runtime-a",
            "polis-a",
            "incarnation-b",
        )
        .unwrap();
        assert_eq!(
            reopened.projection("ember").archive_state,
            AgentArchiveState::Degraded
        );
    }

    #[test]
    fn default_cadence_is_five_minutes_and_overlap_is_skipped_per_agent() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        assert_eq!(store.interval_seconds(), 300);
        assert!(store.begin_capture("ember"));
        assert!(!store.begin_capture("ember"));
        assert_eq!(
            store.projection("ember").snapshot_state,
            AgentSnapshotState::Overdue
        );
        store.finish_capture("ember");
        assert!(store.begin_capture("ember"));
    }

    #[test]
    fn restart_ignores_uncommitted_temporary_partial() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        let agent_dir = store.local_root.join(digest_label("ember"));
        fs::write(
            agent_dir.join(".00000000000000000002.json.crash.tmp"),
            b"partial",
        )
        .unwrap();
        drop(store);

        let reopened = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-b",
        )
        .unwrap();
        let restored = reopened.latest_valid(2, &"a".repeat(64)).unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].snapshot_sequence, 1);
    }

    #[test]
    fn later_tombstone_prevents_removed_agent_state_from_restoring() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        store.write_tombstone("ember", 2, "a".repeat(64)).unwrap();
        assert!(store.latest_valid(2, &"a".repeat(64)).unwrap().is_empty());
        drop(store);

        let reopened = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-b",
        )
        .unwrap();
        assert!(reopened
            .latest_valid(2, &"a".repeat(64))
            .unwrap()
            .is_empty());
        reopened.write_partial(capture("ember", 2)).unwrap();
        assert_eq!(
            reopened.latest_valid(2, &"a".repeat(64)).unwrap()[0].snapshot_sequence,
            3
        );
    }

    #[test]
    fn conflicting_records_at_the_same_sequence_fail_closed() {
        let temp = tempfile::tempdir().unwrap();
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            store_config(false),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        let original = json_files(&store.local_root).unwrap().pop().unwrap();
        let mut conflicting = match decode_record(&fs::read(&original).unwrap()).unwrap() {
            StoredRecord::Partial(value) => value,
            StoredRecord::Tombstone(_) => unreachable!(),
        };
        conflicting.created_at_unix_millis = 2;
        conflicting.checkpoint_digest = canonical_digest(&conflicting).unwrap();
        let conflict_path = store.local_root.join("conflicting-copy.json");
        fs::write(conflict_path, serde_json::to_vec(&conflicting).unwrap()).unwrap();

        assert!(matches!(
            store.latest_valid(2, &"a".repeat(64)),
            Err(AgentPartialError::InvalidLineage)
        ));
    }

    #[test]
    fn archive_saturation_does_not_prevent_agent_removal() {
        let temp = tempfile::tempdir().unwrap();
        let mut config = store_config(true);
        config.spool_max_bytes = 1;
        let store = AgentPartialCheckpointStore::open(
            temp.path().to_path_buf(),
            config,
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        let projection = store.write_partial(capture("ember", 1)).unwrap();
        assert_eq!(projection.archive_state, AgentArchiveState::SpoolSaturated);
        store.write_tombstone("ember", 2, "a".repeat(64)).unwrap();

        assert!(store.latest_valid(2, &"a".repeat(64)).unwrap().is_empty());
        assert!(store.read_archive_degraded("ember").unwrap().is_some());
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn archived_partial_restores_when_local_copy_is_absent() {
        let temp = tempfile::tempdir().unwrap();
        let mut store = AgentPartialCheckpointStore::open(
            temp.path().join("state"),
            store_config(true),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        let local_path = json_files(&store.local_root).unwrap().pop().unwrap();
        let fixture = temp.path().join("archived.json");
        fs::copy(&local_path, &fixture).unwrap();
        let record = decode_record(&fs::read(&fixture).unwrap()).unwrap();
        let key = record.object_key();
        fs::remove_dir_all(&store.local_root).unwrap();
        fs::create_dir_all(&store.local_root).unwrap();
        let script = temp.path().join("fake-aws");
        fs::write(
            &script,
            format!(
                "#!/bin/sh\nif [ \"$2\" = \"list-objects-v2\" ]; then printf '%s' '{{\"Contents\":[{{\"Key\":\"{key}\"}}]}}'; exit 0; fi\nlast=''\nfor arg in \"$@\"; do last=\"$arg\"; done\ncp '{}' \"$last\"\nprintf '%s' '{{}}'\n",
                fixture.display()
            ),
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        store.set_archive_command(script);

        let restored = store
            .latest_valid_with_archive(2, &"a".repeat(64))
            .await
            .unwrap();
        assert_eq!(restored.len(), 1);
        assert_eq!(restored[0].agent_id, "ember");
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn verified_archive_receipt_survives_store_restart() {
        let temp = tempfile::tempdir().unwrap();
        let mut store = AgentPartialCheckpointStore::open(
            temp.path().join("state"),
            store_config(true),
            "runtime-a",
            "polis-a",
            "incarnation-a",
        )
        .unwrap();
        store.write_partial(capture("ember", 1)).unwrap();
        let spool = json_files(&store.spool_root).unwrap().pop().unwrap();
        let record = decode_record(&fs::read(spool).unwrap()).unwrap();
        let script = temp.path().join("fake-aws");
        fs::write(
            &script,
            format!(
                "#!/bin/sh\nif [ \"$2\" = \"head-object\" ]; then printf '%s' '{{\"Metadata\":{{\"sha256\":\"{}\"}}}}'; fi\nexit 0\n",
                record.digest()
            ),
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        store.set_archive_command(script);
        assert_eq!(store.archive_pending().await.unwrap(), 1);
        let before = store.projection("ember");
        assert_eq!(before.archive_state, AgentArchiveState::Current);
        assert!(before.last_archive_at_unix_millis.is_some());
        drop(store);

        let reopened = AgentPartialCheckpointStore::open(
            temp.path().join("state"),
            store_config(true),
            "runtime-a",
            "polis-a",
            "incarnation-b",
        )
        .unwrap();
        assert_eq!(reopened.projection("ember"), before);
    }
}
