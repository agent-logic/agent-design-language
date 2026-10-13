//! Portable ADL document and planning contracts, with local process/session support.
//!
//! Execution, provider expansion and product implementations belong to their
//! installed owners. The public CLI delegates those commands without loading
//! private implementation crates into the ADL dependency graph.

pub use adl_legacy_contracts::{
    adl, execution_plan, plan, prompt, resolve, schema, trace_schema_v1,
};
pub mod control_plane;
pub mod session_ledger;
