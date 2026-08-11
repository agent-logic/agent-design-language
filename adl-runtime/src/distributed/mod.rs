//! Authenticated, bounded distributed Guardian contracts.
//!
//! Each submodule owns one authority boundary. This module is the production
//! registration point only; it deliberately does not duplicate or weaken the
//! validation performed by those authorities.

pub mod authority_protocol;
pub mod capability_advertisement;
pub mod certificates;
pub mod discovery;
pub mod failure_detection;
pub mod fencing;
pub mod identity;
pub mod lease;
pub mod membership;
pub mod migration;
pub mod placement;
pub mod polis_runtime;
pub mod projection;
// The recovery store retains a private compatibility persistence helper for
// restart-format parity. Registration makes the module production-visible,
// while that helper intentionally remains unused by the public path.
#[allow(dead_code)]
pub mod recovery;
pub mod resource_weather;
pub mod snapshot_catalog;
pub mod transport;
