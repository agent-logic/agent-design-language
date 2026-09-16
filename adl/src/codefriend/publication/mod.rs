//! Exact-artifact publication approval and fail-closed local admission.
pub mod approval;
pub mod manifest;

pub use approval::{
    admit_local, write_json_create_only, AdmissionReceipt, DecisionKind, DecisionRecord,
};
pub use manifest::{read_publication, read_review, verify_artifacts, ManifestInput};
