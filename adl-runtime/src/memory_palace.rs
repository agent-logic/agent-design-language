use std::{
    fs::{self, File, OpenOptions},
    io::{self, Write},
    path::{Path, PathBuf},
};

use adl_runtime_kernel::{
    build_memory_palace, build_memory_palace_context_cache, continuity_record_digest,
    validate_memory_palace_context_cache, validate_memory_palace_packet, BirthdayContinuityRecord,
    MemoryPalaceContextCache, MemoryPalaceContextPacket, MemoryPalaceInput, MemoryPalaceRejection,
    MemoryReference, VerifiedMemoryPalaceAuthority,
};
use fs2::FileExt;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};

pub const CSM_MEMORY_PALACE_COMPONENT: &str = "memory_palace";
pub const CSM_MEMORY_PALACE_STATUS_SCHEMA: &str = "adl.csm.memory_palace.status.v1";
pub const CSM_MEMORY_PALACE_PACKET_REF: &str = "memory-palace/latest.json";

const CHECKPOINT_SCHEMA: &str = "adl.runtime.memory_palace.checkpoint.v1";
const JOURNAL_SCHEMA: &str = "adl.runtime.memory_palace.journal_entry.v1";
const LATEST_SCHEMA: &str = "adl.runtime.memory_palace.latest.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeMemoryPalaceCheckpoint {
    pub schema: String,
    pub memory_palace_generation: u64,
    pub birthday_continuity_generation: u64,
    pub predecessor_packet_sha256: Option<String>,
    pub identity_root: String,
    pub identity_record_sha256: String,
    pub continuity_head: String,
    pub continuity_record_sha256: String,
    pub topology_sha256: String,
    pub working_set_sha256: String,
    pub source_packet_ref: MemoryReference,
    pub canonical_input_sha256: String,
    pub packet_sha256: String,
    pub context_cache_sha256: String,
    pub trace_reference: MemoryReference,
    pub redaction_policy_sha256: String,
    pub checkpoint_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeMemoryPalaceJournalEntry {
    pub schema: String,
    pub generation: u64,
    pub predecessor_packet_sha256: Option<String>,
    pub packet_sha256: String,
    pub context_cache_sha256: String,
    pub checkpoint_sha256: String,
    pub entry_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeMemoryPalaceLatest {
    pub schema: String,
    pub generation: u64,
    pub packet_sha256: String,
    pub checkpoint_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RuntimeMemoryPalaceCommit {
    pub checkpoint: RuntimeMemoryPalaceCheckpoint,
    pub packet: MemoryPalaceContextPacket,
    pub context_cache: MemoryPalaceContextCache,
}

#[derive(Debug)]
pub enum RuntimeMemoryPalaceError {
    Io(String),
    Kernel(Vec<MemoryPalaceRejection>),
    KernelValidation(MemoryPalaceRejection),
    Corrupt(String),
    GenerationOverflow,
}

impl From<io::Error> for RuntimeMemoryPalaceError {
    fn from(error: io::Error) -> Self {
        Self::Io(error.to_string())
    }
}

pub struct RuntimeMemoryPalaceService {
    root: PathBuf,
}

impl RuntimeMemoryPalaceService {
    pub fn new(root: impl Into<PathBuf>) -> Self {
        Self { root: root.into() }
    }

    pub fn commit(
        &self,
        authority: &VerifiedMemoryPalaceAuthority,
        input: &MemoryPalaceInput,
    ) -> Result<RuntimeMemoryPalaceCommit, RuntimeMemoryPalaceError> {
        let packet =
            build_memory_palace(authority.identity(), authority.continuity().record(), input)
                .map_err(RuntimeMemoryPalaceError::Kernel)?;
        let birthday_generation = authority
            .continuity()
            .record()
            .cycles
            .last()
            .map(|cycle| cycle.generation)
            .unwrap_or(0);
        self.commit_authorized_packet(packet, birthday_generation, authority.continuity().record())
    }

    fn commit_authorized_packet(
        &self,
        packet: MemoryPalaceContextPacket,
        birthday_generation: u64,
        continuity: &BirthdayContinuityRecord,
    ) -> Result<RuntimeMemoryPalaceCommit, RuntimeMemoryPalaceError> {
        if continuity.identity_root != packet.identity_root
            || continuity.identity_record_sha256 != packet.identity_record_sha256
            || continuity.continuity_head != packet.continuity_head
            || continuity.record_sha256 != packet.continuity_record_sha256
            || continuity
                .cycles
                .last()
                .map(|cycle| cycle.generation)
                .unwrap_or(0)
                != birthday_generation
        {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "authorized continuity record does not match Memory Palace packet".to_owned(),
            ));
        }
        self.commit_packet(packet, birthday_generation, Some(continuity))
    }

    #[cfg(test)]
    fn commit_validated_packet(
        &self,
        packet: MemoryPalaceContextPacket,
        birthday_generation: u64,
    ) -> Result<RuntimeMemoryPalaceCommit, RuntimeMemoryPalaceError> {
        self.commit_packet(packet, birthday_generation, None)
    }

    fn commit_packet(
        &self,
        packet: MemoryPalaceContextPacket,
        birthday_generation: u64,
        continuity: Option<&BirthdayContinuityRecord>,
    ) -> Result<RuntimeMemoryPalaceCommit, RuntimeMemoryPalaceError> {
        fs::create_dir_all(self.root.join("journal"))?;
        fs::create_dir_all(self.root.join("generations"))?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.root.join("writer.lock"))?;
        lock.lock_exclusive()?;
        let head = self.validated_journal_head_locked()?;
        let generation = head
            .as_ref()
            .map(|entry| entry.generation.checked_add(1))
            .unwrap_or(Some(1))
            .ok_or(RuntimeMemoryPalaceError::GenerationOverflow)?;
        let predecessor_packet_sha256 = head.as_ref().map(|entry| entry.packet_sha256.clone());
        validate_memory_palace_packet(&packet)
            .map_err(RuntimeMemoryPalaceError::KernelValidation)?;
        if predecessor_packet_sha256.as_deref() == Some(packet.packet_sha256.as_str()) {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "candidate packet repeats predecessor digest".to_owned(),
            ));
        }
        self.validate_successor_binding_locked(
            head.as_ref(),
            &packet,
            birthday_generation,
            continuity,
        )?;
        let packet_ref = packet_ref(generation, &packet);
        let cache = build_memory_palace_context_cache(
            generation,
            birthday_generation,
            packet_ref.clone(),
            &packet,
        )
        .map_err(RuntimeMemoryPalaceError::Kernel)?;
        let checkpoint = checkpoint(
            generation,
            predecessor_packet_sha256,
            packet_ref,
            &packet,
            &cache,
        )?;
        let journal = journal_entry(&checkpoint);
        let generation_dir = self
            .root
            .join("generations")
            .join(format!("{generation:020}"));
        fs::create_dir_all(&generation_dir)?;
        write_json_atomic(&generation_dir.join("packet.json"), &packet)?;
        write_json_atomic(&generation_dir.join("context-cache.json"), &cache)?;
        write_json_atomic(&generation_dir.join("checkpoint.json"), &checkpoint)?;
        fsync_dir(&generation_dir)?;
        write_json_create_new(
            &self
                .root
                .join("journal")
                .join(format!("{generation:020}.json")),
            &journal,
        )?;
        fsync_dir(&self.root.join("journal"))?;
        write_json_atomic(
            &self.root.join("latest.json"),
            &RuntimeMemoryPalaceLatest {
                schema: LATEST_SCHEMA.to_owned(),
                generation,
                packet_sha256: packet.packet_sha256.clone(),
                checkpoint_sha256: checkpoint.checkpoint_sha256.clone(),
            },
        )?;
        fsync_dir(&self.root)?;
        Ok(RuntimeMemoryPalaceCommit {
            checkpoint,
            packet,
            context_cache: cache,
        })
    }

    fn validate_successor_binding_locked(
        &self,
        head: Option<&RuntimeMemoryPalaceJournalEntry>,
        packet: &MemoryPalaceContextPacket,
        birthday_generation: u64,
        continuity: Option<&BirthdayContinuityRecord>,
    ) -> Result<(), RuntimeMemoryPalaceError> {
        let Some(head) = head else {
            return Ok(());
        };
        let previous = self.load_generation_locked(head.generation)?.checkpoint;
        if packet.identity_root != previous.identity_root
            || packet.identity_record_sha256 != previous.identity_record_sha256
        {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "Memory Palace successor changes birthday identity binding".to_owned(),
            ));
        }
        if birthday_generation < previous.birthday_continuity_generation {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "Memory Palace successor regresses birthday continuity generation".to_owned(),
            ));
        }
        if birthday_generation == previous.birthday_continuity_generation
            && (packet.continuity_head != previous.continuity_head
                || packet.continuity_record_sha256 != previous.continuity_record_sha256)
        {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "Memory Palace successor changes continuity binding without generation advance"
                    .to_owned(),
            ));
        }
        if let Some(continuity) = continuity {
            self.validate_authorized_continuity_successor(
                &previous,
                birthday_generation,
                continuity,
            )?;
        }
        Ok(())
    }

    fn validate_authorized_continuity_successor(
        &self,
        previous: &RuntimeMemoryPalaceCheckpoint,
        birthday_generation: u64,
        continuity: &BirthdayContinuityRecord,
    ) -> Result<(), RuntimeMemoryPalaceError> {
        if birthday_generation <= previous.birthday_continuity_generation {
            return Ok(());
        }
        let prefix_cycles = continuity
            .cycles
            .iter()
            .filter(|cycle| cycle.generation <= previous.birthday_continuity_generation)
            .cloned()
            .collect::<Vec<_>>();
        if previous.birthday_continuity_generation > 0
            && prefix_cycles.last().map(|cycle| cycle.generation)
                != Some(previous.birthday_continuity_generation)
        {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "authorized continuity successor is missing prior generation prefix".to_owned(),
            ));
        }
        let mut prefix = BirthdayContinuityRecord {
            schema: continuity.schema.clone(),
            identity_root: continuity.identity_root.clone(),
            identity_record_sha256: continuity.identity_record_sha256.clone(),
            predecessor_head: continuity.predecessor_head.clone(),
            authority_context_sha256: continuity.authority_context_sha256.clone(),
            cycles: prefix_cycles,
            grade: continuity.grade.clone(),
            continuity_head: previous.continuity_head.clone(),
            record_sha256: String::new(),
        };
        prefix.record_sha256 = continuity_record_digest(&prefix)
            .map_err(|error| RuntimeMemoryPalaceError::Corrupt(error.to_string()))?;
        if prefix.record_sha256 != previous.continuity_record_sha256 {
            return Err(RuntimeMemoryPalaceError::Corrupt(
                "authorized continuity successor does not preserve prior continuity prefix"
                    .to_owned(),
            ));
        }
        Ok(())
    }

    pub fn load_latest(
        &self,
    ) -> Result<Option<RuntimeMemoryPalaceCommit>, RuntimeMemoryPalaceError> {
        fs::create_dir_all(self.root.join("journal"))?;
        fs::create_dir_all(self.root.join("generations"))?;
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(self.root.join("writer.lock"))?;
        lock.lock_exclusive()?;
        let Some(head) = self.validated_journal_head_locked()? else {
            return Ok(None);
        };
        match read_json::<RuntimeMemoryPalaceLatest>(&self.root.join("latest.json")) {
            Ok(latest)
                if latest.schema == LATEST_SCHEMA
                    && latest.generation == head.generation
                    && latest.packet_sha256 == head.packet_sha256
                    && latest.checkpoint_sha256 == head.checkpoint_sha256 => {}
            Ok(latest)
                if latest.generation < head.generation
                    && latest.packet_sha256 != head.packet_sha256 =>
            {
                write_json_atomic(
                    &self.root.join("latest.json"),
                    &RuntimeMemoryPalaceLatest {
                        schema: LATEST_SCHEMA.to_owned(),
                        generation: head.generation,
                        packet_sha256: head.packet_sha256.clone(),
                        checkpoint_sha256: head.checkpoint_sha256.clone(),
                    },
                )?;
                fsync_dir(&self.root)?;
            }
            Ok(_) => {
                return Err(RuntimeMemoryPalaceError::Corrupt(
                    "latest pointer is ahead of or differently bound from journal head".to_owned(),
                ))
            }
            Err(RuntimeMemoryPalaceError::Io(_)) => {
                write_json_atomic(
                    &self.root.join("latest.json"),
                    &RuntimeMemoryPalaceLatest {
                        schema: LATEST_SCHEMA.to_owned(),
                        generation: head.generation,
                        packet_sha256: head.packet_sha256.clone(),
                        checkpoint_sha256: head.checkpoint_sha256.clone(),
                    },
                )?;
                fsync_dir(&self.root)?;
            }
            Err(error) => return Err(error),
        }
        self.load_generation_locked(head.generation).map(Some)
    }

    pub fn retained_status(&self) -> Value {
        match self.load_latest() {
            Ok(Some(commit)) => json!({
                "schema": CSM_MEMORY_PALACE_STATUS_SCHEMA,
                "component": CSM_MEMORY_PALACE_COMPONENT,
                "status": "validated",
                "packet_schema": adl_runtime_kernel::MEMORY_PALACE_PACKET_SCHEMA,
                "memory_palace_generation": commit.checkpoint.memory_palace_generation,
                "birthday_continuity_generation": commit.checkpoint.birthday_continuity_generation,
                "packet_sha256": commit.checkpoint.packet_sha256,
                "checkpoint_sha256": commit.checkpoint.checkpoint_sha256,
                "context_cache_sha256": commit.checkpoint.context_cache_sha256,
                "continuity_head": commit.checkpoint.continuity_head,
                "source": "durable_memory_palace_service"
            }),
            Ok(None) => json!({
                "schema": CSM_MEMORY_PALACE_STATUS_SCHEMA,
                "component": CSM_MEMORY_PALACE_COMPONENT,
                "status": "unvalidated",
                "packet_schema": adl_runtime_kernel::MEMORY_PALACE_PACKET_SCHEMA,
                "source": "durable_memory_palace_service"
            }),
            Err(error) => json!({
                "schema": CSM_MEMORY_PALACE_STATUS_SCHEMA,
                "component": CSM_MEMORY_PALACE_COMPONENT,
                "status": "invalid",
                "packet_schema": adl_runtime_kernel::MEMORY_PALACE_PACKET_SCHEMA,
                "error_class": error.status_error_class(),
                "source": "durable_memory_palace_service"
            }),
        }
    }

    fn validated_journal_head_locked(
        &self,
    ) -> Result<Option<RuntimeMemoryPalaceJournalEntry>, RuntimeMemoryPalaceError> {
        let journal_dir = self.root.join("journal");
        let mut entries = Vec::new();
        for item in fs::read_dir(&journal_dir)? {
            let item = item?;
            let path = item.path();
            if path.extension().and_then(|v| v.to_str()) != Some("json") {
                continue;
            }
            entries.push(path);
        }
        entries.sort();
        let mut expected = 1;
        let mut predecessor = None;
        let mut head = None;
        for path in entries {
            let entry: RuntimeMemoryPalaceJournalEntry = read_json(&path)?;
            validate_journal_entry(&entry)?;
            if entry.generation != expected || entry.predecessor_packet_sha256 != predecessor {
                return Err(RuntimeMemoryPalaceError::Corrupt(
                    "journal generation gap, fork, or predecessor mismatch".to_owned(),
                ));
            }
            let loaded = self.load_generation_locked(entry.generation)?;
            if loaded.packet.packet_sha256 != entry.packet_sha256
                || loaded.context_cache.cache_sha256 != entry.context_cache_sha256
                || loaded.checkpoint.checkpoint_sha256 != entry.checkpoint_sha256
            {
                return Err(RuntimeMemoryPalaceError::Corrupt(
                    "journal entry does not match generation artifacts".to_owned(),
                ));
            }
            predecessor = Some(entry.packet_sha256.clone());
            expected += 1;
            head = Some(entry);
        }
        Ok(head)
    }

    fn load_generation_locked(
        &self,
        generation: u64,
    ) -> Result<RuntimeMemoryPalaceCommit, RuntimeMemoryPalaceError> {
        let dir = self
            .root
            .join("generations")
            .join(format!("{generation:020}"));
        let packet: MemoryPalaceContextPacket = read_json(&dir.join("packet.json"))?;
        validate_memory_palace_packet(&packet)
            .map_err(RuntimeMemoryPalaceError::KernelValidation)?;
        let cache: MemoryPalaceContextCache = read_json(&dir.join("context-cache.json"))?;
        validate_memory_palace_context_cache(&cache, &packet)
            .map_err(RuntimeMemoryPalaceError::KernelValidation)?;
        let checkpoint: RuntimeMemoryPalaceCheckpoint = read_json(&dir.join("checkpoint.json"))?;
        validate_checkpoint(&checkpoint, &packet, &cache)?;
        Ok(RuntimeMemoryPalaceCommit {
            checkpoint,
            packet,
            context_cache: cache,
        })
    }
}

impl RuntimeMemoryPalaceError {
    fn status_error_class(&self) -> &'static str {
        match self {
            RuntimeMemoryPalaceError::Io(_) => "io",
            RuntimeMemoryPalaceError::Kernel(_) => "kernel_rejection",
            RuntimeMemoryPalaceError::KernelValidation(_) => "kernel_validation",
            RuntimeMemoryPalaceError::Corrupt(_) => "corrupt",
            RuntimeMemoryPalaceError::GenerationOverflow => "generation_overflow",
        }
    }
}

fn packet_ref(generation: u64, packet: &MemoryPalaceContextPacket) -> MemoryReference {
    MemoryReference {
        id: format!("memory-palace-packet:{generation:020}"),
        path: format!("memory-palace/generations/{generation:020}/packet.json"),
        sha256: packet.packet_sha256.clone(),
    }
}

fn checkpoint(
    generation: u64,
    predecessor_packet_sha256: Option<String>,
    packet_ref: MemoryReference,
    packet: &MemoryPalaceContextPacket,
    cache: &MemoryPalaceContextCache,
) -> Result<RuntimeMemoryPalaceCheckpoint, RuntimeMemoryPalaceError> {
    let mut checkpoint = RuntimeMemoryPalaceCheckpoint {
        schema: CHECKPOINT_SCHEMA.to_owned(),
        memory_palace_generation: generation,
        birthday_continuity_generation: cache.birthday_continuity_generation,
        predecessor_packet_sha256,
        identity_root: packet.identity_root.clone(),
        identity_record_sha256: packet.identity_record_sha256.clone(),
        continuity_head: packet.continuity_head.clone(),
        continuity_record_sha256: packet.continuity_record_sha256.clone(),
        topology_sha256: sha256_jcs(&packet.rooms)?,
        working_set_sha256: sha256_jcs(&packet.working_set)?,
        source_packet_ref: packet_ref,
        canonical_input_sha256: packet.canonical_input_sha256.clone(),
        packet_sha256: packet.packet_sha256.clone(),
        context_cache_sha256: cache.cache_sha256.clone(),
        trace_reference: packet.trace_reference.clone(),
        redaction_policy_sha256: packet.redaction_policy_sha256.clone(),
        checkpoint_sha256: String::new(),
    };
    checkpoint.checkpoint_sha256 = sha256_jcs(&checkpoint)?;
    Ok(checkpoint)
}

fn journal_entry(checkpoint: &RuntimeMemoryPalaceCheckpoint) -> RuntimeMemoryPalaceJournalEntry {
    let mut entry = RuntimeMemoryPalaceJournalEntry {
        schema: JOURNAL_SCHEMA.to_owned(),
        generation: checkpoint.memory_palace_generation,
        predecessor_packet_sha256: checkpoint.predecessor_packet_sha256.clone(),
        packet_sha256: checkpoint.packet_sha256.clone(),
        context_cache_sha256: checkpoint.context_cache_sha256.clone(),
        checkpoint_sha256: checkpoint.checkpoint_sha256.clone(),
        entry_sha256: String::new(),
    };
    entry.entry_sha256 = sha256_jcs(&entry).expect("journal entry encodes");
    entry
}

fn validate_journal_entry(
    entry: &RuntimeMemoryPalaceJournalEntry,
) -> Result<(), RuntimeMemoryPalaceError> {
    let mut unsigned = entry.clone();
    unsigned.entry_sha256.clear();
    if entry.schema != JOURNAL_SCHEMA
        || entry.generation == 0
        || entry.entry_sha256 != sha256_jcs(&unsigned)?
    {
        return Err(RuntimeMemoryPalaceError::Corrupt(
            "invalid Memory Palace journal entry".to_owned(),
        ));
    }
    Ok(())
}

fn validate_checkpoint(
    checkpoint: &RuntimeMemoryPalaceCheckpoint,
    packet: &MemoryPalaceContextPacket,
    cache: &MemoryPalaceContextCache,
) -> Result<(), RuntimeMemoryPalaceError> {
    let mut unsigned = checkpoint.clone();
    unsigned.checkpoint_sha256.clear();
    if checkpoint.schema != CHECKPOINT_SCHEMA
        || checkpoint.memory_palace_generation == 0
        || checkpoint.memory_palace_generation != cache.memory_palace_generation
        || checkpoint.birthday_continuity_generation != cache.birthday_continuity_generation
        || checkpoint.identity_root != packet.identity_root
        || checkpoint.identity_record_sha256 != packet.identity_record_sha256
        || checkpoint.continuity_head != packet.continuity_head
        || checkpoint.continuity_record_sha256 != packet.continuity_record_sha256
        || checkpoint.topology_sha256 != sha256_jcs(&packet.rooms)?
        || checkpoint.working_set_sha256 != sha256_jcs(&packet.working_set)?
        || checkpoint.source_packet_ref != cache.source_packet_ref
        || checkpoint.canonical_input_sha256 != packet.canonical_input_sha256
        || checkpoint.packet_sha256 != packet.packet_sha256
        || checkpoint.context_cache_sha256 != cache.cache_sha256
        || checkpoint.trace_reference != packet.trace_reference
        || checkpoint.redaction_policy_sha256 != packet.redaction_policy_sha256
        || checkpoint.checkpoint_sha256 != sha256_jcs(&unsigned)?
    {
        return Err(RuntimeMemoryPalaceError::Corrupt(
            "invalid Memory Palace checkpoint".to_owned(),
        ));
    }
    Ok(())
}

fn read_json<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T, RuntimeMemoryPalaceError> {
    let bytes = fs::read(path)?;
    serde_json::from_slice(&bytes)
        .map_err(|error| RuntimeMemoryPalaceError::Corrupt(error.to_string()))
}

fn write_json_atomic<T: Serialize>(path: &Path, value: &T) -> Result<(), RuntimeMemoryPalaceError> {
    let temp = path.with_extension("tmp");
    write_json_create_new(&temp, value)?;
    fs::rename(&temp, path)?;
    Ok(())
}

fn write_json_create_new<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), RuntimeMemoryPalaceError> {
    let bytes = serde_jcs::to_vec(value)
        .map_err(|error| RuntimeMemoryPalaceError::Corrupt(error.to_string()))?;
    let mut file = OpenOptions::new().write(true).create_new(true).open(path)?;
    file.write_all(&bytes)?;
    file.sync_all()?;
    Ok(())
}

fn fsync_dir(path: &Path) -> Result<(), RuntimeMemoryPalaceError> {
    File::open(path)?.sync_all()?;
    Ok(())
}

fn sha256_jcs<T: Serialize>(value: &T) -> Result<String, RuntimeMemoryPalaceError> {
    let bytes = serde_jcs::to_vec(value)
        .map_err(|error| RuntimeMemoryPalaceError::Corrupt(error.to_string()))?;
    Ok(format!("{:x}", Sha256::digest(bytes)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use adl_runtime_kernel::{
        birthday_identity, build_memory_palace, continuity_record_digest, BirthdayContinuityRecord,
        BirthdayIdentityRecord, MemoryPalaceInput, MemoryReference, MemoryTemporalAnchor,
        MemoryVisibility, ObsMemContextRecord,
    };
    use serde_json::json;

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
        }))
        .unwrap();
        record.record_sha256 = birthday_identity::record_digest(&record).unwrap();
        record
    }

    fn continuity(identity: &BirthdayIdentityRecord) -> BirthdayContinuityRecord {
        let mut record: BirthdayContinuityRecord = serde_json::from_value(json!({
            "schema":"adl.birthday.continuity_record.v1","identity_root":identity.identity_root,
            "identity_record_sha256":identity.record_sha256,"predecessor_head":H,"authority_context_sha256":H,
            "cycles":[{"generation":1,"accepted_through":1,"previous_integrity":H,"integrity":H,"signing_key_id":"key","reference":{"id":"cycle-1","path":"evidence/continuity/cycle-1.json","sha256":H}}],
            "grade":"evidence_backed","continuity_head":H,"record_sha256":""
        }))
        .unwrap();
        record.record_sha256 = continuity_record_digest(&record).unwrap();
        record
    }

    fn packet(id: &str, payload: &str) -> MemoryPalaceContextPacket {
        let identity = identity();
        let continuity = continuity(&identity);
        let trace_reference = MemoryReference {
            id: "trace:aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_owned(),
            path: ".adl/runtime-v3/observability/trace.json".to_owned(),
            sha256: H.to_owned(),
        };
        let input = MemoryPalaceInput {
            schema: adl_runtime_kernel::MEMORY_PALACE_INPUT_SCHEMA.to_owned(),
            identity_record_sha256: identity.record_sha256.clone(),
            continuity_record_sha256: continuity.record_sha256.clone(),
            trace_reference: trace_reference.clone(),
            redaction_policy_sha256: H.to_owned(),
            observed_epoch_ms: 2_000,
            stale_after_ms: 1_000,
            max_working_set_items: 8,
            records: vec![ObsMemContextRecord {
                id: id.to_owned(),
                run_id: format!("run-{id}"),
                workflow_id: "workflow-alpha".to_owned(),
                payload: payload.to_owned(),
                visibility: MemoryVisibility::Public,
                identity_root: identity.identity_root.clone(),
                continuity_head: continuity.continuity_head.clone(),
                trace_id: trace_reference.id.clone(),
                citations: vec![trace_reference],
                temporal_anchor: MemoryTemporalAnchor {
                    created_epoch_ms: 1_500,
                    observed_epoch_ms: 1_500,
                    effective_epoch_ms: 1_500,
                    continuity_head: continuity.continuity_head.clone(),
                    event_sequence: 1,
                },
            }],
        };
        build_memory_palace(&identity, &continuity, &input).unwrap()
    }

    fn rehash_packet(packet: &mut MemoryPalaceContextPacket) {
        packet.packet_sha256.clear();
        packet.packet_sha256 = sha256_jcs(packet).unwrap();
    }

    #[test]
    fn commits_two_generations_and_loads_latest_from_journal() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        let first = service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();
        let second = service
            .commit_validated_packet(packet("b", "second"), 1)
            .unwrap();
        assert_eq!(first.checkpoint.memory_palace_generation, 1);
        assert_eq!(second.checkpoint.memory_palace_generation, 2);
        assert_eq!(
            second.checkpoint.predecessor_packet_sha256.as_deref(),
            Some(first.packet.packet_sha256.as_str())
        );
        let loaded = service.load_latest().unwrap().unwrap();
        assert_eq!(loaded.checkpoint, second.checkpoint);
        assert_eq!(loaded.context_cache.working_set, second.packet.working_set);
    }

    #[test]
    fn successor_binding_rejects_identity_swap_same_generation_fork_and_rollback() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();

        let mut identity_swap = packet("b", "second");
        identity_swap.identity_record_sha256 =
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_owned();
        rehash_packet(&mut identity_swap);
        assert!(service.commit_validated_packet(identity_swap, 1).is_err());

        let mut same_generation_fork = packet("c", "third");
        same_generation_fork.continuity_record_sha256 =
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_owned();
        rehash_packet(&mut same_generation_fork);
        assert!(service
            .commit_validated_packet(same_generation_fork, 1)
            .is_err());

        let rollback = packet("d", "fourth");
        assert!(service.commit_validated_packet(rollback, 0).is_err());
    }

    #[test]
    fn retained_status_reports_only_validated_durable_state_as_ready_source() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        let empty = service.retained_status();
        assert_eq!(empty["status"], "unvalidated");
        assert_eq!(
            empty["packet_schema"],
            adl_runtime_kernel::MEMORY_PALACE_PACKET_SCHEMA
        );
        assert_eq!(empty["source"], "durable_memory_palace_service");

        let committed = service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();
        let retained = service.retained_status();
        assert_eq!(retained["status"], "validated");
        assert_eq!(
            retained["packet_sha256"],
            committed.checkpoint.packet_sha256
        );
        assert_eq!(
            retained["checkpoint_sha256"],
            committed.checkpoint.checkpoint_sha256
        );
    }

    #[test]
    fn stale_or_missing_latest_repairs_from_valid_journal_head() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        let first = service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();
        let second = service
            .commit_validated_packet(packet("b", "second"), 1)
            .unwrap();
        write_json_atomic(
            &root.path().join("memory-palace/latest.json"),
            &RuntimeMemoryPalaceLatest {
                schema: LATEST_SCHEMA.to_owned(),
                generation: 1,
                packet_sha256: first.packet.packet_sha256,
                checkpoint_sha256: first.checkpoint.checkpoint_sha256,
            },
        )
        .unwrap();
        let loaded = service.load_latest().unwrap().unwrap();
        assert_eq!(
            loaded.checkpoint.checkpoint_sha256,
            second.checkpoint.checkpoint_sha256
        );
        let repaired: RuntimeMemoryPalaceLatest =
            read_json(&root.path().join("memory-palace/latest.json")).unwrap();
        assert_eq!(repaired.generation, 2);
        fs::remove_file(root.path().join("memory-palace/latest.json")).unwrap();
        let loaded = service.load_latest().unwrap().unwrap();
        assert_eq!(
            loaded.checkpoint.checkpoint_sha256,
            second.checkpoint.checkpoint_sha256
        );
    }

    #[test]
    fn forged_artifact_digest_fails_restart_validation() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();
        let packet_path = root
            .path()
            .join("memory-palace/generations/00000000000000000001/packet.json");
        let mut value: serde_json::Value =
            serde_json::from_slice(&fs::read(&packet_path).unwrap()).unwrap();
        value["working_set"][0]["payload"] = json!("forged");
        fs::write(&packet_path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert!(service.load_latest().is_err());
    }

    #[test]
    fn ahead_or_differently_bound_latest_fails_closed() {
        let root = tempfile::tempdir().unwrap();
        let service = RuntimeMemoryPalaceService::new(root.path().join("memory-palace"));
        let first = service
            .commit_validated_packet(packet("a", "first"), 1)
            .unwrap();
        write_json_atomic(
            &root.path().join("memory-palace/latest.json"),
            &RuntimeMemoryPalaceLatest {
                schema: LATEST_SCHEMA.to_owned(),
                generation: 2,
                packet_sha256: first.packet.packet_sha256.clone(),
                checkpoint_sha256: first.checkpoint.checkpoint_sha256.clone(),
            },
        )
        .unwrap();
        assert!(service.load_latest().is_err());
        write_json_atomic(
            &root.path().join("memory-palace/latest.json"),
            &RuntimeMemoryPalaceLatest {
                schema: LATEST_SCHEMA.to_owned(),
                generation: 1,
                packet_sha256: H.to_owned(),
                checkpoint_sha256: first.checkpoint.checkpoint_sha256,
            },
        )
        .unwrap();
        assert!(service.load_latest().is_err());
    }
}
