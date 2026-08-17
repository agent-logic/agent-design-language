use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use fs2::FileExt;
use serde::{Deserialize, Serialize};

pub const CONVERSATION_JOURNAL_SCHEMA: &str = "adl.runtime.conversation_journal.v1";
pub const CONVERSATION_JOURNAL_FILE: &str = "conversation-journal.jsonl";

const GENESIS_HASH: &str = "conversation-journal-genesis-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationJournalEvent {
    pub event_id: String,
    pub conversation_id: String,
    pub principal_id: String,
    pub authority_audit_hash: String,
    pub payload_hash: String,
    pub receipt_ref: Option<String>,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationRetentionMarker {
    pub conversation_id: String,
    pub reason: String,
    pub retain_until_epoch_ms: Option<u64>,
    pub authority_audit_hash: String,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationDeletionMarker {
    pub conversation_id: String,
    pub reason: String,
    pub authority_audit_hash: String,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ConversationJournalEntry {
    Event(ConversationJournalEvent),
    Retention(ConversationRetentionMarker),
    Deletion(ConversationDeletionMarker),
    Migration(ConversationJournalMigration),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationJournalMigration {
    pub from_schema_version: u32,
    pub to_schema_version: u32,
    pub migration_id: String,
    pub authority_audit_hash: String,
    pub created_at_epoch_ms: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConversationJournalRecord {
    pub schema: String,
    pub schema_version: u32,
    pub sequence: u64,
    pub previous_hash: String,
    pub record_hash: String,
    pub entry: ConversationJournalEntry,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConversationJournalSnapshot {
    pub schema_version: u32,
    pub head_sequence: u64,
    pub head_hash: String,
    pub records: Vec<ConversationJournalRecord>,
    pub deleted_conversations: BTreeSet<String>,
    pub retention_by_conversation: BTreeMap<String, ConversationRetentionMarker>,
}

#[derive(Debug, thiserror::Error)]
pub enum ConversationJournalError {
    #[error("conversation journal root must be an absolute path")]
    RelativeRoot,
    #[error("conversation journal I/O failed: {0}")]
    Io(#[from] std::io::Error),
    #[error("conversation journal encoding failed: {0}")]
    Encoding(#[from] serde_json::Error),
    #[error("conversation journal contains a corrupt or partial record")]
    CorruptRecord,
    #[error("conversation journal schema is unsupported")]
    UnsupportedSchema,
    #[error("conversation journal entry is invalid: {0}")]
    InvalidEntry(&'static str),
}

pub type ConversationJournalResult<T> = Result<T, ConversationJournalError>;

pub struct ConversationJournal {
    path: PathBuf,
}

impl ConversationJournal {
    pub fn open(root: impl AsRef<Path>) -> ConversationJournalResult<Self> {
        let root = root.as_ref();
        if root.as_os_str().is_empty() || !root.is_absolute() {
            return Err(ConversationJournalError::RelativeRoot);
        }
        fs::create_dir_all(root)?;
        let journal = Self {
            path: root.join(CONVERSATION_JOURNAL_FILE),
        };
        journal.snapshot()?;
        Ok(journal)
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn append_event(
        &self,
        event: ConversationJournalEvent,
    ) -> ConversationJournalResult<ConversationJournalRecord> {
        self.append(ConversationJournalEntry::Event(event))
    }

    pub fn record_retention(
        &self,
        marker: ConversationRetentionMarker,
    ) -> ConversationJournalResult<ConversationJournalRecord> {
        self.append(ConversationJournalEntry::Retention(marker))
    }

    pub fn record_deletion(
        &self,
        marker: ConversationDeletionMarker,
    ) -> ConversationJournalResult<ConversationJournalRecord> {
        self.append(ConversationJournalEntry::Deletion(marker))
    }

    pub fn record_migration(
        &self,
        migration: ConversationJournalMigration,
    ) -> ConversationJournalResult<ConversationJournalRecord> {
        self.append(ConversationJournalEntry::Migration(migration))
    }

    pub fn snapshot(&self) -> ConversationJournalResult<ConversationJournalSnapshot> {
        let file = match File::open(&self.path) {
            Ok(file) => file,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                return Ok(ConversationJournalSnapshot::empty());
            }
            Err(error) => return Err(error.into()),
        };
        load_snapshot(file)
    }

    fn append(
        &self,
        entry: ConversationJournalEntry,
    ) -> ConversationJournalResult<ConversationJournalRecord> {
        validate_entry(&entry)?;
        let mut file = OpenOptions::new()
            .create(true)
            .read(true)
            .append(true)
            .open(&self.path)?;
        file.lock_exclusive()?;
        let snapshot = load_snapshot(file.try_clone()?)?;
        let record = build_record(snapshot.head_sequence + 1, &snapshot.head_hash, entry)?;
        let bytes = serde_json::to_vec(&record)?;
        file.seek(SeekFrom::End(0))?;
        file.write_all(&bytes)?;
        file.write_all(b"\n")?;
        file.sync_data()?;
        Ok(record)
    }
}

impl ConversationJournalSnapshot {
    fn empty() -> Self {
        Self {
            schema_version: 1,
            head_sequence: 0,
            head_hash: GENESIS_HASH.to_string(),
            records: Vec::new(),
            deleted_conversations: BTreeSet::new(),
            retention_by_conversation: BTreeMap::new(),
        }
    }

    pub fn committed_events(&self) -> impl Iterator<Item = &ConversationJournalEvent> {
        self.records
            .iter()
            .filter_map(|record| match &record.entry {
                ConversationJournalEntry::Event(event)
                    if !self.deleted_conversations.contains(&event.conversation_id) =>
                {
                    Some(event)
                }
                _ => None,
            })
    }
}

fn load_snapshot(mut file: File) -> ConversationJournalResult<ConversationJournalSnapshot> {
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;
    if contents.is_empty() {
        return Ok(ConversationJournalSnapshot::empty());
    }
    if contents.last() != Some(&b'\n') {
        return Err(ConversationJournalError::CorruptRecord);
    }
    let mut snapshot = ConversationJournalSnapshot::empty();
    for line in contents.split(|byte| *byte == b'\n') {
        if line.is_empty() {
            continue;
        }
        let record: ConversationJournalRecord =
            serde_json::from_slice(line).map_err(|_| ConversationJournalError::CorruptRecord)?;
        validate_record(&record, &snapshot)?;
        apply_record(&mut snapshot, record);
    }
    Ok(snapshot)
}

fn validate_record(
    record: &ConversationJournalRecord,
    previous: &ConversationJournalSnapshot,
) -> ConversationJournalResult<()> {
    if record.schema != CONVERSATION_JOURNAL_SCHEMA {
        return Err(ConversationJournalError::UnsupportedSchema);
    }
    if record.schema_version != 1 {
        return Err(ConversationJournalError::UnsupportedSchema);
    }
    if record.sequence != previous.head_sequence + 1 || record.previous_hash != previous.head_hash {
        return Err(ConversationJournalError::CorruptRecord);
    }
    if record.record_hash != calculate_record_hash(record)? {
        return Err(ConversationJournalError::CorruptRecord);
    }
    validate_entry(&record.entry)?;
    Ok(())
}

fn apply_record(snapshot: &mut ConversationJournalSnapshot, record: ConversationJournalRecord) {
    match &record.entry {
        ConversationJournalEntry::Retention(marker) => {
            snapshot
                .retention_by_conversation
                .insert(marker.conversation_id.clone(), marker.clone());
        }
        ConversationJournalEntry::Deletion(marker) => {
            snapshot
                .deleted_conversations
                .insert(marker.conversation_id.clone());
        }
        ConversationJournalEntry::Migration(migration) => {
            snapshot.schema_version = migration.to_schema_version;
        }
        ConversationJournalEntry::Event(_) => {}
    }
    snapshot.head_sequence = record.sequence;
    snapshot.head_hash = record.record_hash.clone();
    snapshot.records.push(record);
}

fn build_record(
    sequence: u64,
    previous_hash: &str,
    entry: ConversationJournalEntry,
) -> ConversationJournalResult<ConversationJournalRecord> {
    let mut record = ConversationJournalRecord {
        schema: CONVERSATION_JOURNAL_SCHEMA.to_string(),
        schema_version: 1,
        sequence,
        previous_hash: previous_hash.to_string(),
        record_hash: String::new(),
        entry,
    };
    record.record_hash = calculate_record_hash(&record)?;
    Ok(record)
}

#[derive(Serialize)]
struct RecordHashInput<'a> {
    schema: &'a str,
    schema_version: u32,
    sequence: u64,
    previous_hash: &'a str,
    entry: &'a ConversationJournalEntry,
}

fn calculate_record_hash(record: &ConversationJournalRecord) -> ConversationJournalResult<String> {
    let input = RecordHashInput {
        schema: &record.schema,
        schema_version: record.schema_version,
        sequence: record.sequence,
        previous_hash: &record.previous_hash,
        entry: &record.entry,
    };
    Ok(blake3::hash(&serde_jcs::to_vec(&input)?)
        .to_hex()
        .to_string())
}

fn validate_entry(entry: &ConversationJournalEntry) -> ConversationJournalResult<()> {
    match entry {
        ConversationJournalEntry::Event(event) => {
            require_non_empty(&event.event_id, "event_id")?;
            require_non_empty(&event.conversation_id, "conversation_id")?;
            require_non_empty(&event.principal_id, "principal_id")?;
            require_digest(&event.authority_audit_hash, "authority_audit_hash")?;
            require_digest(&event.payload_hash, "payload_hash")?;
        }
        ConversationJournalEntry::Retention(marker) => {
            require_non_empty(&marker.conversation_id, "conversation_id")?;
            require_non_empty(&marker.reason, "reason")?;
            require_digest(&marker.authority_audit_hash, "authority_audit_hash")?;
        }
        ConversationJournalEntry::Deletion(marker) => {
            require_non_empty(&marker.conversation_id, "conversation_id")?;
            require_non_empty(&marker.reason, "reason")?;
            require_digest(&marker.authority_audit_hash, "authority_audit_hash")?;
        }
        ConversationJournalEntry::Migration(migration) => {
            require_non_empty(&migration.migration_id, "migration_id")?;
            require_digest(&migration.authority_audit_hash, "authority_audit_hash")?;
            if migration.from_schema_version != 1 || migration.to_schema_version != 1 {
                return Err(ConversationJournalError::UnsupportedSchema);
            }
        }
    }
    Ok(())
}

fn require_non_empty(value: &str, field: &'static str) -> ConversationJournalResult<()> {
    if value.trim().is_empty() {
        return Err(ConversationJournalError::InvalidEntry(field));
    }
    Ok(())
}

fn require_digest(value: &str, field: &'static str) -> ConversationJournalResult<()> {
    if value.len() != 64 || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(ConversationJournalError::InvalidEntry(field));
    }
    Ok(())
}
