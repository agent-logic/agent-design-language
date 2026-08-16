//! Authenticated, bounded distributed Guardian contracts.
//!
//! Each submodule owns one authority boundary. This module is the production
//! registration point only; it deliberately does not duplicate or weaken the
//! validation performed by those authorities.

pub mod authority_protocol;
pub mod authority_reconciliation;
pub mod authority_store_adapters;
pub mod capability_advertisement;
pub mod certificates;
pub mod continuity_transfer;
pub mod discovery;
pub mod failure_detection;
pub mod fencing;
pub mod identity;
pub mod integrated_serving_authority_snapshot;
pub mod lease;
pub mod membership;
pub mod membership_coordinator;
pub mod migration;
pub mod placement;
pub mod projection;
// The recovery store retains a private compatibility persistence helper for
// restart-format parity. Registration makes the module production-visible,
// while that helper intentionally remains unused by the public path.
pub mod observatory_serving_eligibility;
#[allow(dead_code)]
pub mod recovery;
pub mod resource_weather;
pub mod runtime_continuity_bridge;
pub mod serving_authority;
pub mod shepherd_serving_eligibility;
pub mod snapshot_catalog;
#[path = "transport/root.rs"]
pub mod transport;
pub use transport::{learner_transport, polis_runtime};
