//! Isolated CodeFriend repository review execution.
//!
//! This module consumes already-admitted CodeFriend evidence and calls a
//! registered provider once per review perspective. It does not synthesize peer
//! findings; downstream synthesis is a separate concern.

pub mod lanes;
pub mod runner;
pub mod synthesis;
