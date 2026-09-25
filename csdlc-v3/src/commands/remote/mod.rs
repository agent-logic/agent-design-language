#![allow(dead_code)]

//! Native C-SDLC v3 remote route contracts and stable public dispatch.

mod authority;
mod coordination;
mod delivery;
pub mod intent;
mod merge;
mod merge_linkage;
mod merge_retirement;
mod model;
mod mutation;
mod publication;
mod routing;
mod storage;
mod support;
mod target;
mod transport;

pub use authority::canonical_authority_selector_digest;
pub use delivery::*;
pub use merge::retained_merge_intent_exists;
pub use merge_linkage::{merge_linkage_query, merge_state_query};
pub(crate) use merge_retirement::retire_never_dispatched_merge;
pub use model::*;
pub use mutation::{
    execute_github_mutation, execute_staged_github_mutation, stage_github_mutation,
    stage_retained_github_mutation_recovery,
    stage_retained_github_mutation_recovery_after_rejection,
};
pub use publication::{
    github_adapter_receipt_payload_digest, github_readback_receipt_payload_digest,
    load_remote_route_receipts, observe_github_pr_readback, pending_mutation_finding,
    prepare_remote_publication_route, prepare_remote_publication_route_with_receipts,
    publication_body_is_valid, typed_review_receipt_payload_digest,
};
pub use routing::dispatch_operational_remote;
pub use storage::retained_mutation_request;
pub(crate) use storage::{
    repository_scoped_issue_creation_receipt, require_settled_transition_remote,
    settled_coordination_completion_receipt, settled_issue_mutation_receipt,
    settled_issue_scoped_mutation_receipt,
};
pub use support::{github_mutation_operation_digest, github_mutation_operation_marker};

// Existing white-box tests exercise private failure-path helpers through the
// historical parent-module boundary. Keep those aliases test-only so the
// production dependency graph remains explicit.
#[cfg(test)]
use mutation::*;
#[cfg(test)]
use storage::*;
#[cfg(test)]
use support::*;
#[cfg(test)]
use transport::*;

#[cfg(test)]
mod tests;
