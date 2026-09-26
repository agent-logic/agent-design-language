//! Compatibility exports for the single public model identity contract.
pub use adl_schema::model_identity::*;
use std::time::{SystemTime, UNIX_EPOCH};

pub fn observed_at_now_v1() -> String {
    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default();
    format!("unix:{seconds}")
}
