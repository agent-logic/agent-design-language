//! Portable legacy syntax, deterministic planning and wire contracts.
use anyhow::{anyhow, Result};
pub mod adl;
pub mod cognitive_transition_schema;
pub mod execution_plan;
pub mod plan;
pub mod prompt;
pub mod resolve;
pub mod schema;
pub mod trace_schema_v1;
pub use adl_schema::model_identity;
mod chronosense {
    pub use adl_schema::CHRONOSENSE_EVENT_ANCHOR_SCHEMA;
}

pub fn validate_run_id_path_segment(run_id: &str) -> Result<String> {
    let trimmed = run_id.trim();
    if trimmed.is_empty() {
        return Err(anyhow!("run_id must not be empty for artifact paths"));
    }
    if trimmed == "." || trimmed == ".." {
        return Err(anyhow!(
            "run_id must be a safe path segment, not '.' or '..'"
        ));
    }
    if trimmed.contains('/') || trimmed.contains('\\') {
        return Err(anyhow!(
            "run_id must be a safe path segment and must not contain path separators"
        ));
    }
    if trimmed.contains(':') {
        return Err(anyhow!(
            "run_id must be a safe path segment and must not contain drive-like ':' prefixes"
        ));
    }
    Ok(trimmed.to_string())
}
