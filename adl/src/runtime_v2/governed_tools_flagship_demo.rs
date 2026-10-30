//! Runtime-v2 governed-tools flagship demo proof bundle.
//!
//! This module composes existing fixture-backed governed-tools surfaces into one
//! reviewer-facing D11 packet without claiming arbitrary production execution.

use super::*;

#[path = "governed_tools_flagship_demo_parts/core.rs"]
mod governed_tools_flagship_demo_core;

pub use governed_tools_flagship_demo_core::*;
