use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use adl_runtime_kernel::{RUNTIME_MASTER_LOG_AUDIT_SCHEMA, RUNTIME_MASTER_LOG_RECORD_SCHEMA};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

const SEQUENCE_CHECKPOINT_SCHEMA: &str = "adl.runtime_v3.observability.sequence.v1";
static ATOMIC_WRITE_COUNTER: AtomicU64 = AtomicU64::new(0);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MasterLogAuditReport {
    pub schema: String,
    pub status: String,
    pub platform: String,
    pub suite: String,
    pub revision: String,
    pub start_sequence: u64,
    pub end_sequence: u64,
    pub record_count: u64,
    pub malformed_records: u64,
    pub missing_required_fields: u64,
    pub sequence_gaps: u64,
    pub error_events: u64,
    pub degraded_events: u64,
    pub unexplained_restarts: u64,
    pub incomplete_drains: u64,
}

pub fn audit_master_log_file(
    path: &Path,
    platform: &str,
    suite: &str,
    revision: &str,
    start_sequence: u64,
    end_sequence: u64,
) -> std::io::Result<MasterLogAuditReport> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut report = MasterLogAuditReport {
        schema: RUNTIME_MASTER_LOG_AUDIT_SCHEMA.to_owned(),
        status: "pass".to_owned(),
        platform: platform.to_owned(),
        suite: suite.to_owned(),
        revision: revision.to_owned(),
        start_sequence,
        end_sequence,
        record_count: 0,
        malformed_records: 0,
        missing_required_fields: 0,
        sequence_gaps: 0,
        error_events: 0,
        degraded_events: 0,
        unexplained_restarts: 0,
        incomplete_drains: 1,
    };
    let mut records_by_sequence = BTreeMap::new();
    let mut line = Vec::new();
    loop {
        line.clear();
        let read = reader.read_until(b'\n', &mut line)?;
        if read == 0 {
            break;
        }
        let line = String::from_utf8_lossy(line.trim_ascii_end());
        if line.trim().is_empty() {
            continue;
        }
        let Ok(record) = serde_json::from_str::<Value>(&line) else {
            report.malformed_records = report.malformed_records.saturating_add(1);
            continue;
        };
        let Some(sequence) = record["sequence"].as_u64() else {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
            continue;
        };
        if sequence < start_sequence || sequence > end_sequence {
            continue;
        }
        if let Some(previous) = records_by_sequence.get(&sequence) {
            if previous != &record {
                report.malformed_records = report.malformed_records.saturating_add(1);
            }
            continue;
        }
        records_by_sequence.insert(sequence, record.clone());
        report.record_count = report.record_count.saturating_add(1);
        if !has_required_master_log_fields(&record) {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        if record["schema"] != RUNTIME_MASTER_LOG_RECORD_SCHEMA {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        if record["lifecycle_suite"].as_str() != Some(suite) {
            report.missing_required_fields = report.missing_required_fields.saturating_add(1);
        }
        let severity = record["severity"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let operation = record["operation"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        let reason = record["reason"]
            .as_str()
            .unwrap_or_default()
            .to_ascii_lowercase();
        if matches!(severity.as_str(), "error" | "fatal") {
            report.error_events = report.error_events.saturating_add(1);
        }
        if record["health"].as_str() == Some("degraded")
            || record["status"].as_str() == Some("degraded")
        {
            report.degraded_events = report.degraded_events.saturating_add(1);
        }
        if operation == "runtime_restart" && reason != "explained" {
            report.unexplained_restarts = report.unexplained_restarts.saturating_add(1);
        }
        if operation == "observability_drain_complete" && reason == "shutdown_drained" {
            report.incomplete_drains = 0;
        }
    }
    let expected_records = end_sequence
        .checked_sub(start_sequence)
        .and_then(|distance| distance.checked_add(1))
        .unwrap_or(0);
    report.sequence_gaps =
        expected_records.saturating_sub(u64::try_from(records_by_sequence.len()).unwrap_or(0));
    if report.record_count == 0
        || report.malformed_records != 0
        || report.missing_required_fields != 0
        || report.sequence_gaps != 0
        || report.error_events != 0
        || report.degraded_events != 0
        || report.unexplained_restarts != 0
        || report.incomplete_drains != 0
    {
        report.status = "fail".to_owned();
    }
    Ok(report)
}

fn has_required_master_log_fields(record: &Value) -> bool {
    [
        "timestamp",
        "severity",
        "level",
        "target",
        "runtime_instance_id",
        "guardian_id",
        "process_id",
        "lifecycle_suite",
        "lifecycle_run",
        "lifecycle_cycle",
        "component",
        "operation",
        "reason",
        "error_chain",
        "revision",
        "sequence",
    ]
    .iter()
    .all(|field| !record[*field].is_null())
}

pub(super) fn create_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

pub(super) fn write_json_atomic(path: &Path, value: &impl Serialize) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let temp = path.with_extension(format!(
        "tmp.{}.{}",
        std::process::id(),
        ATOMIC_WRITE_COUNTER.fetch_add(1, Ordering::SeqCst)
    ));
    {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp)?;
        serde_json::to_writer_pretty(&mut file, value)?;
        file.write_all(b"\n")?;
        file.sync_data()?;
    }
    fs::rename(&temp, path)?;
    sync_parent_dir(path)?;
    Ok(())
}

pub(super) fn write_sequence_checkpoint(
    path: &Path,
    checkpoint: SequenceCheckpoint,
) -> std::io::Result<()> {
    write_json_atomic(path, &checkpoint)
}

pub(super) fn recover_next_sequence(
    sequence_path: &Path,
    master_log_path: &Path,
    ingress_spool_path: &Path,
) -> std::io::Result<u64> {
    let checkpoint_next = match fs::read(sequence_path) {
        Ok(bytes) => {
            let checkpoint: SequenceCheckpoint =
                serde_json::from_slice(&bytes).map_err(|error| {
                    std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string())
                })?;
            checkpoint.validate()?;
            if checkpoint.next_sequence > 0
                && !master_log_path.is_file()
                && !ingress_spool_path.is_file()
            {
                return Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "sequence checkpoint exists without master log or ingress spool",
                ));
            }
            checkpoint.next_sequence
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(error) => return Err(error),
    };
    Ok(checkpoint_next
        .max(recover_next_sequence_from_log(master_log_path)?)
        .max(recover_next_sequence_from_log(ingress_spool_path)?))
}

fn recover_next_sequence_from_log(path: &Path) -> std::io::Result<u64> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(0),
        Err(error) => return Err(error),
    };
    let mut next = 0_u64;
    for line in BufReader::new(file).lines() {
        let line = line?;
        let record = serde_json::from_str::<Value>(&line).map_err(|error| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string())
        })?;
        let sequence = record["sequence"].as_u64().ok_or_else(|| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, "missing sequence")
        })?;
        next = next.max(sequence.saturating_add(1));
    }
    Ok(next)
}

pub(super) fn recover_run_start_sequence(
    path: &Path,
    lifecycle_run: &str,
    lifecycle_cycle: &str,
    fallback: u64,
) -> std::io::Result<u64> {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(fallback),
        Err(error) => return Err(error),
    };
    let mut start = None;
    for line in BufReader::new(file).lines() {
        let record = serde_json::from_str::<Value>(&line?).map_err(|error| {
            std::io::Error::new(std::io::ErrorKind::InvalidData, error.to_string())
        })?;
        if record["lifecycle_run"].as_str() == Some(lifecycle_run)
            && record["lifecycle_cycle"].as_str() == Some(lifecycle_cycle)
        {
            let sequence = record["sequence"].as_u64().ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::InvalidData, "missing sequence")
            })?;
            start = Some(start.map_or(sequence, |current: u64| current.min(sequence)));
        }
    }
    Ok(start.unwrap_or(fallback))
}

pub(super) fn master_log_contains_sequence(path: &Path, sequence: u64) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .any(|line| {
            serde_json::from_str::<Value>(&line)
                .ok()
                .and_then(|record| record["sequence"].as_u64())
                == Some(sequence)
        })
}

pub(super) fn master_log_highest_sequence(path: &Path) -> Option<u64> {
    let file = File::open(path).ok()?;
    BufReader::new(file)
        .lines()
        .map_while(Result::ok)
        .filter_map(|line| {
            serde_json::from_str::<Value>(&line)
                .ok()
                .and_then(|record| record["sequence"].as_u64())
        })
        .max()
}

#[derive(Debug, Serialize, Deserialize)]
pub(super) struct SequenceCheckpoint {
    schema: String,
    next_sequence: u64,
    last_drained_sequence: u64,
    updated_at: String,
}

impl SequenceCheckpoint {
    pub(super) fn new(next_sequence: u64, last_drained_sequence: u64) -> Self {
        Self {
            schema: SEQUENCE_CHECKPOINT_SCHEMA.to_owned(),
            next_sequence,
            last_drained_sequence,
            updated_at: OffsetDateTime::now_utc()
                .format(&Rfc3339)
                .unwrap_or_else(|_| "1970-01-01T00:00:00Z".to_owned()),
        }
    }

    fn validate(&self) -> std::io::Result<()> {
        if self.schema != SEQUENCE_CHECKPOINT_SCHEMA
            || self.next_sequence < self.last_drained_sequence
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "invalid observability sequence checkpoint",
            ));
        }
        Ok(())
    }
}

pub(super) fn rotate_file_if_large(
    path: &Path,
    max_bytes: u64,
    retain: usize,
) -> std::io::Result<()> {
    if max_bytes == 0 || retain == 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "rotation bounds must be positive",
        ));
    }
    if path
        .metadata()
        .map(|metadata| metadata.len() <= max_bytes)
        .unwrap_or(true)
    {
        return Ok(());
    }
    let suffix = format!(
        "{}.{}",
        OffsetDateTime::now_utc().unix_timestamp_nanos(),
        std::process::id()
    );
    let rotated = path.with_extension(format!("jsonl.{suffix}"));
    fs::rename(path, rotated)?;
    prune_rotated_files(path, retain)
}

fn prune_rotated_files(path: &Path, retain: usize) -> std::io::Result<()> {
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
        return Ok(());
    };
    let mut entries = fs::read_dir(parent)?
        .filter_map(Result::ok)
        .filter(|entry| {
            entry
                .file_name()
                .to_str()
                .map(|name| name.starts_with(file_name) && name != file_name)
                .unwrap_or(false)
        })
        .filter_map(|entry| {
            let modified = entry
                .metadata()
                .and_then(|metadata| metadata.modified())
                .ok()?;
            Some((modified, entry.path()))
        })
        .collect::<Vec<_>>();
    entries.sort_by_key(|(modified, _)| *modified);
    let remove_count = entries.len().saturating_sub(retain);
    for (_, path) in entries.into_iter().take(remove_count) {
        fs::remove_file(path)?;
    }
    Ok(())
}

pub(crate) fn absolute_path(path: &Path) -> std::io::Result<PathBuf> {
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        Ok(std::env::current_dir()?.join(path))
    }
}

pub(crate) fn vector_path(path: &Path) -> String {
    let mut value = path.to_string_lossy().replace('\\', "/");
    if let Some(stripped) = value.strip_prefix("//?/") {
        value = stripped.to_owned();
    }
    value
}

#[cfg(unix)]
fn sync_parent_dir(path: &Path) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        File::open(parent)?.sync_all()?;
    }
    Ok(())
}

#[cfg(not(unix))]
fn sync_parent_dir(_path: &Path) -> std::io::Result<()> {
    Ok(())
}

pub(super) fn current_platform() -> String {
    std::env::consts::OS.to_owned()
}
