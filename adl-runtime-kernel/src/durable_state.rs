use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};

use redb::{Database, Durability, ReadableDatabase, ReadableTable, TableDefinition};
use serde_json::Value;

pub const KERNEL_DURABLE_STATE_DB_FILE: &str = "runtime-kernel.redb";
pub const LOCAL_CHECKPOINT_SCHEMA: &str = "adl.runtime.local_checkpoint.v1";
pub const LOCAL_LIFELOG_SCHEMA: &str = "adl.runtime.local_lifelog.v1";
pub const GOVERNED_LIFELOG_SCHEMA: &str = "adl.runtime.parity_c.lifelog.v1";

const META: TableDefinition<&str, u64> = TableDefinition::new("kernel_meta_v1");
const LOCAL_CHECKPOINTS: TableDefinition<u64, &[u8]> =
    TableDefinition::new("local_checkpoint_records_v1");
const LOCAL_CHECKPOINT_BYTES: TableDefinition<u64, &[u8]> =
    TableDefinition::new("local_checkpoint_state_bytes_v1");
const LOCAL_LIFELOG: TableDefinition<u64, &[u8]> = TableDefinition::new("local_lifelog_v1");
const GOVERNED_STATE: TableDefinition<&str, &[u8]> = TableDefinition::new("governed_state_v1");
const GOVERNED_LIFELOG: TableDefinition<u64, &[u8]> = TableDefinition::new("governed_lifelog_v1");
const COMMUNICATION_OUTBOUND_SEQUENCES: TableDefinition<&str, u64> =
    TableDefinition::new("communication_outbound_sequences_v1");
const COMMUNICATION_INBOUND_SEQUENCES: TableDefinition<&str, u64> =
    TableDefinition::new("communication_inbound_sequences_v1");
const MAX_COMMUNICATION_SEQUENCE: u64 = 9_007_199_254_740_991;

#[derive(Debug, thiserror::Error)]
pub enum KernelDurableStateError {
    #[error("state root must be an absolute configured path")]
    RelativeRoot,
    #[error("legacy flat persistence file is present: {0}")]
    LegacyFlatPersistence(PathBuf),
    #[error("durable state I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("durable state database failed: {0}")]
    Database(String),
    #[error("durable state encoding failed: {0}")]
    Encoding(#[from] serde_json::Error),
    #[error("durable checkpoint unavailable")]
    CheckpointUnavailable,
    #[error("durable checkpoint corrupt")]
    CheckpointCorrupt,
    #[error("durable checkpoint identity or integrity mismatch")]
    CheckpointIdentityOrIntegrity,
    #[error("durable communication sequence exhausted")]
    CommunicationSequenceExhausted,
    #[error("durable communication sequence must advance")]
    CommunicationSequenceConflict,
}

pub type KernelDurableStateResult<T> = Result<T, KernelDurableStateError>;
pub type GovernedStateCompareAndSet<'a> = (&'a str, Option<&'a [u8]>, &'a [u8]);

pub struct KernelDurableState {
    database: Database,
}

impl KernelDurableState {
    pub fn open(root: impl AsRef<Path>) -> KernelDurableStateResult<Self> {
        let root = root.as_ref();
        if root.as_os_str().is_empty() || !root.is_absolute() {
            return Err(KernelDurableStateError::RelativeRoot);
        }
        fs::create_dir_all(root)?;
        reject_legacy_flat_persistence(root)?;
        let database =
            Database::create(root.join(KERNEL_DURABLE_STATE_DB_FILE)).map_err(database_error)?;
        let state = Self { database };
        state.initialize_tables()?;
        Ok(state)
    }

    pub fn store_local_checkpoint(
        &self,
        adapter: &str,
        operation: &str,
        request_id: &str,
        principal: &str,
        writer_id: &str,
        state: &[u8],
    ) -> KernelDurableStateResult<Value> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let generation = next_sequence_in_transaction(&write, "next_local_checkpoint_generation")?;
        let state_hex = hex::encode(state);
        let value = serde_json::json!({
            "schema": LOCAL_CHECKPOINT_SCHEMA,
            "adapter": adapter,
            "operation": operation,
            "request_id": request_id,
            "principal": principal,
            "generation": generation,
            "writer_id": writer_id,
            "payload_hash": blake3::hash(state).to_hex().to_string(),
            "state_hex": state_hex
        });
        validate_local_checkpoint_value(&value, principal, state)?;
        let encoded = serde_json::to_vec(&value)?;
        {
            let mut records = write
                .open_table(LOCAL_CHECKPOINTS)
                .map_err(database_error)?;
            records
                .insert(generation, encoded.as_slice())
                .map_err(database_error)?;
        }
        {
            let mut bytes = write
                .open_table(LOCAL_CHECKPOINT_BYTES)
                .map_err(database_error)?;
            bytes.insert(generation, state).map_err(database_error)?;
        }
        {
            let mut meta = write.open_table(META).map_err(database_error)?;
            meta.insert("local_checkpoint_head", generation)
                .map_err(database_error)?;
        }
        write.commit().map_err(database_error)?;
        Ok(value)
    }

    pub fn restore_local_checkpoint(&self, principal: &str) -> KernelDurableStateResult<Value> {
        let read = self.database.begin_read().map_err(database_error)?;
        let meta = read.open_table(META).map_err(database_error)?;
        let generation = meta
            .get("local_checkpoint_head")
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value();
        drop(meta);
        let records = read.open_table(LOCAL_CHECKPOINTS).map_err(database_error)?;
        let encoded = records
            .get(generation)
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value()
            .to_vec();
        drop(records);
        let bytes = read
            .open_table(LOCAL_CHECKPOINT_BYTES)
            .map_err(database_error)?
            .get(generation)
            .map_err(database_error)?
            .ok_or(KernelDurableStateError::CheckpointUnavailable)?
            .value()
            .to_vec();
        let value: Value = serde_json::from_slice(&encoded)
            .map_err(|_| KernelDurableStateError::CheckpointCorrupt)?;
        validate_local_checkpoint_value(&value, principal, &bytes)?;
        Ok(value)
    }

    pub fn append_local_lifelog(
        &self,
        adapter: &str,
        operation: &str,
        request_id: &str,
        principal: &str,
        payload: &[u8],
        redacted: bool,
    ) -> KernelDurableStateResult<Value> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let sequence = next_sequence_in_transaction(&write, "next_local_lifelog_sequence")?;
        let value = serde_json::json!({
            "schema": LOCAL_LIFELOG_SCHEMA,
            "adapter": adapter,
            "operation": operation,
            "request_id": request_id,
            "principal": principal,
            "sequence": sequence,
            "payload_hash": blake3::hash(payload).to_hex().to_string(),
            "redacted": redacted,
            "authoritative": false
        });
        let encoded = serde_json::to_vec(&value)?;
        write
            .open_table(LOCAL_LIFELOG)
            .map_err(database_error)?
            .insert(sequence, encoded.as_slice())
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(value)
    }

    pub fn load_governed_state(&self, domain: &str) -> KernelDurableStateResult<Option<Vec<u8>>> {
        let read = self.database.begin_read().map_err(database_error)?;
        let table = read.open_table(GOVERNED_STATE).map_err(database_error)?;
        let value = table
            .get(domain)
            .map_err(database_error)?
            .map(|bytes| bytes.value().to_vec());
        Ok(value)
    }

    pub fn store_governed_state(&self, domain: &str, state: &[u8]) -> KernelDurableStateResult<()> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        write
            .open_table(GOVERNED_STATE)
            .map_err(database_error)?
            .insert(domain, state)
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }

    /// Atomically replaces one governed-state value only when its current bytes exactly match
    /// `expected`. `None` means the domain must not exist. A false result makes no mutation.
    pub fn compare_and_set_governed_state(
        &self,
        domain: &str,
        expected: Option<&[u8]>,
        replacement: &[u8],
    ) -> KernelDurableStateResult<bool> {
        self.compare_and_set_governed_states(&[(domain, expected, replacement)])
    }

    /// Atomically compares and replaces distinct governed-state domains in one transaction.
    /// Any mismatch leaves every domain unchanged.
    pub fn compare_and_set_governed_states(
        &self,
        changes: &[GovernedStateCompareAndSet<'_>],
    ) -> KernelDurableStateResult<bool> {
        if changes.is_empty()
            || changes
                .iter()
                .map(|(domain, _, _)| *domain)
                .collect::<BTreeSet<_>>()
                .len()
                != changes.len()
        {
            return Ok(false);
        }
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let applied = {
            let mut table = write.open_table(GOVERNED_STATE).map_err(database_error)?;
            let mut matches = true;
            for (domain, expected, _) in changes {
                let current = table.get(*domain).map_err(database_error)?;
                if current.as_ref().map(|value| value.value()) != *expected {
                    matches = false;
                    break;
                }
            }
            if matches {
                for (domain, _, replacement) in changes {
                    table
                        .insert(*domain, *replacement)
                        .map_err(database_error)?;
                }
            }
            matches
        };
        if applied {
            write.commit().map_err(database_error)?;
        } else {
            write.abort().map_err(database_error)?;
        }
        Ok(applied)
    }

    pub fn append_governed_lifelog(&self, entry: &Value) -> KernelDurableStateResult<()> {
        if entry["schema"] != GOVERNED_LIFELOG_SCHEMA {
            return Err(KernelDurableStateError::CheckpointCorrupt);
        }
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let sequence = next_sequence_in_transaction(&write, "next_governed_lifelog_sequence")?;
        let encoded = serde_json::to_vec(entry)?;
        write
            .open_table(GOVERNED_LIFELOG)
            .map_err(database_error)?
            .insert(sequence, encoded.as_slice())
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }

    pub fn next_communication_outbound_sequence(
        &self,
        principal_id: &str,
    ) -> KernelDurableStateResult<u64> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let next = {
            let mut sequences = write
                .open_table(COMMUNICATION_OUTBOUND_SEQUENCES)
                .map_err(database_error)?;
            let next = sequences
                .get(principal_id)
                .map_err(database_error)?
                .map(|value| next_communication_sequence(Some(value.value())))
                .transpose()?
                .unwrap_or(1);
            sequences
                .insert(principal_id, next)
                .map_err(database_error)?;
            next
        };
        write.commit().map_err(database_error)?;
        Ok(next)
    }

    pub fn communication_inbound_sequences(
        &self,
    ) -> KernelDurableStateResult<std::collections::BTreeMap<String, u64>> {
        let read = self.database.begin_read().map_err(database_error)?;
        let table = read
            .open_table(COMMUNICATION_INBOUND_SEQUENCES)
            .map_err(database_error)?;
        let mut sequences = std::collections::BTreeMap::new();
        for row in table.iter().map_err(database_error)? {
            let (principal, sequence) = row.map_err(database_error)?;
            sequences.insert(principal.value().to_owned(), sequence.value());
        }
        Ok(sequences)
    }

    pub fn reserve_communication_inbound_sequence(
        &self,
        principal_id: &str,
        sequence: u64,
    ) -> KernelDurableStateResult<Option<u64>> {
        if sequence == 0 || sequence > MAX_COMMUNICATION_SEQUENCE {
            return Err(KernelDurableStateError::CommunicationSequenceExhausted);
        }
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        let previous = {
            let mut sequences = write
                .open_table(COMMUNICATION_INBOUND_SEQUENCES)
                .map_err(database_error)?;
            let previous = sequences
                .get(principal_id)
                .map_err(database_error)?
                .map(|value| value.value());
            if sequence <= previous.unwrap_or(0) {
                return Err(KernelDurableStateError::CommunicationSequenceConflict);
            }
            sequences
                .insert(principal_id, sequence)
                .map_err(database_error)?;
            previous
        };
        write.commit().map_err(database_error)?;
        Ok(previous)
    }

    pub fn rollback_communication_inbound_sequence(
        &self,
        principal_id: &str,
        sequence: u64,
        previous: Option<u64>,
    ) -> KernelDurableStateResult<()> {
        let mut write = self.database.begin_write().map_err(database_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(database_error)?;
        {
            let mut sequences = write
                .open_table(COMMUNICATION_INBOUND_SEQUENCES)
                .map_err(database_error)?;
            let current = sequences
                .get(principal_id)
                .map_err(database_error)?
                .map(|value| value.value());
            if current == Some(sequence) {
                match previous {
                    Some(previous) => {
                        sequences
                            .insert(principal_id, previous)
                            .map_err(database_error)?;
                    }
                    None => {
                        sequences.remove(principal_id).map_err(database_error)?;
                    }
                }
            }
        }
        write.commit().map_err(database_error)?;
        Ok(())
    }

    pub fn local_lifelog_len(&self) -> KernelDurableStateResult<usize> {
        table_len(&self.database, LOCAL_LIFELOG)
    }

    pub fn governed_lifelog_len(&self) -> KernelDurableStateResult<usize> {
        table_len(&self.database, GOVERNED_LIFELOG)
    }

    pub fn governed_lifelog_entries(&self) -> KernelDurableStateResult<Vec<Value>> {
        let read = self.database.begin_read().map_err(database_error)?;
        let table = read.open_table(GOVERNED_LIFELOG).map_err(database_error)?;
        let mut entries = Vec::new();
        for row in table.iter().map_err(database_error)? {
            let (_, bytes) = row.map_err(database_error)?;
            entries.push(serde_json::from_slice(bytes.value())?);
        }
        Ok(entries)
    }

    fn initialize_tables(&self) -> KernelDurableStateResult<()> {
        let write = self.database.begin_write().map_err(database_error)?;
        write.open_table(META).map_err(database_error)?;
        write
            .open_table(LOCAL_CHECKPOINTS)
            .map_err(database_error)?;
        write
            .open_table(LOCAL_CHECKPOINT_BYTES)
            .map_err(database_error)?;
        write.open_table(LOCAL_LIFELOG).map_err(database_error)?;
        write.open_table(GOVERNED_STATE).map_err(database_error)?;
        write.open_table(GOVERNED_LIFELOG).map_err(database_error)?;
        write
            .open_table(COMMUNICATION_OUTBOUND_SEQUENCES)
            .map_err(database_error)?;
        write
            .open_table(COMMUNICATION_INBOUND_SEQUENCES)
            .map_err(database_error)?;
        write.commit().map_err(database_error)?;
        Ok(())
    }
}

fn next_communication_sequence(previous: Option<u64>) -> KernelDurableStateResult<u64> {
    match previous {
        None => Ok(1),
        Some(previous) if previous < MAX_COMMUNICATION_SEQUENCE => Ok(previous + 1),
        Some(_) => Err(KernelDurableStateError::CommunicationSequenceExhausted),
    }
}

fn next_sequence_in_transaction(
    write: &redb::WriteTransaction,
    key: &str,
) -> KernelDurableStateResult<u64> {
    let mut meta = write.open_table(META).map_err(database_error)?;
    let next = meta
        .get(key)
        .map_err(database_error)?
        .map_or(1, |value| value.value());
    meta.insert(key, next.saturating_add(1))
        .map_err(database_error)?;
    Ok(next)
}

fn reject_legacy_flat_persistence(root: &Path) -> KernelDurableStateResult<()> {
    for name in ["checkpoint.json", "lifelog.jsonl"] {
        let path = root.join(name);
        if path.exists() {
            return Err(KernelDurableStateError::LegacyFlatPersistence(path));
        }
    }
    for entry in fs::read_dir(root)? {
        let path = entry?.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if name.starts_with("checkpoint.") && name.ends_with(".tmp") {
            return Err(KernelDurableStateError::LegacyFlatPersistence(path));
        }
    }
    Ok(())
}

fn validate_local_checkpoint_value(
    value: &Value,
    principal: &str,
    state: &[u8],
) -> KernelDurableStateResult<()> {
    let Some(state_hex) = value["state_hex"].as_str() else {
        return Err(KernelDurableStateError::CheckpointCorrupt);
    };
    let decoded = hex::decode(state_hex).map_err(|_| KernelDurableStateError::CheckpointCorrupt)?;
    if value["schema"] != LOCAL_CHECKPOINT_SCHEMA
        || value["principal"] != principal
        || decoded != state
        || value["payload_hash"] != blake3::hash(state).to_hex().to_string()
    {
        return Err(KernelDurableStateError::CheckpointIdentityOrIntegrity);
    }
    Ok(())
}

fn table_len<K, V>(
    database: &Database,
    definition: TableDefinition<K, V>,
) -> KernelDurableStateResult<usize>
where
    K: redb::Key + 'static,
    V: redb::Value + 'static,
{
    let read = database.begin_read().map_err(database_error)?;
    let table = read.open_table(definition).map_err(database_error)?;
    let mut count = 0;
    for row in table.iter().map_err(database_error)? {
        row.map_err(database_error)?;
        count += 1;
    }
    Ok(count)
}

fn database_error(error: impl std::fmt::Display) -> KernelDurableStateError {
    KernelDurableStateError::Database(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn communication_sequence_fails_closed_at_the_interoperable_maximum() {
        assert_eq!(next_communication_sequence(None).unwrap(), 1);
        assert_eq!(
            next_communication_sequence(Some(MAX_COMMUNICATION_SEQUENCE - 1)).unwrap(),
            MAX_COMMUNICATION_SEQUENCE
        );
        assert!(matches!(
            next_communication_sequence(Some(MAX_COMMUNICATION_SEQUENCE)),
            Err(KernelDurableStateError::CommunicationSequenceExhausted)
        ));
    }
}
