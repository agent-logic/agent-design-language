//! Portable wire contracts. Syntax validation does not grant execution authority.
mod provider;
pub use provider::*;
pub mod model_identity;
pub mod syntax;
pub const CHRONOSENSE_EVENT_ANCHOR_SCHEMA: &str = "chronosense_event_anchor.v1";
