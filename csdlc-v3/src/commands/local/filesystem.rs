//! Atomic filesystem operations and rollback cleanup for the native local owner.

use std::{
    fs,
    io::Write,
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use serde_json::Value;

use super::{finding, DoctorFinding, PlanStatus};

pub(super) fn atomic_write_json(path: &Path, value: &Value) -> Result<(), Vec<DoctorFinding>> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "json_serialize_failed",
            &error.to_string(),
        )]
    })?;
    atomic_write(path, &bytes)
}

pub(super) fn atomic_write(path: &Path, bytes: &[u8]) -> Result<(), Vec<DoctorFinding>> {
    let parent = path.parent().ok_or_else(|| {
        vec![finding(
            PlanStatus::Failed,
            "write_parent_missing",
            "target has no parent",
        )]
    })?;
    fs::create_dir_all(parent).map_err(io_finding("write_parent_create_failed"))?;
    let temporary = parent.join(format!(
        ".{}.tmp-{}-{}",
        path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("state"),
        std::process::id(),
        next_local_temp_sequence()
    ));
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&temporary)
        .map_err(io_finding("temporary_create_failed"))?;
    file.write_all(bytes)
        .map_err(io_finding("temporary_write_failed"))?;
    file.sync_all()
        .map_err(io_finding("temporary_sync_failed"))?;
    fs::rename(&temporary, path).map_err(io_finding("atomic_rename_failed"))
}

pub(super) fn next_local_temp_sequence() -> u64 {
    static SEQUENCE: AtomicU64 = AtomicU64::new(0);
    SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

pub(super) fn io_finding(code: &'static str) -> impl FnOnce(std::io::Error) -> Vec<DoctorFinding> {
    move |error| vec![finding(PlanStatus::Failed, code, &error.to_string())]
}

pub(super) fn append_directory_rollback_finding(path: &Path, findings: &mut Vec<DoctorFinding>) {
    if let Err(error) = fs::remove_dir_all(path) {
        findings.push(finding(
            PlanStatus::Failed,
            "issue_state_rollback_failed",
            &error.to_string(),
        ));
    }
}
