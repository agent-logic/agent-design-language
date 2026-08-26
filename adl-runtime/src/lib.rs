//! CSM runtime crate boundary.
//!
//! This crate is intentionally limited to runtime-owned contracts that can be
//! built without ADL compiler or C-SDLC control-plane crates.

pub use adl_resilience as shared_resilience;
pub use adl_runtime_kernel::tls;
pub use adl_runtime_kernel::{LiveContinuity, LiveKernelSnapshot, RuntimeRecorder};

pub mod acip;
pub mod backpressure;
pub mod cav;
pub mod config_reload;
pub mod constructability;
pub mod continuity_history;
pub mod curiosity;
pub mod determinism;
pub mod distributed;
pub mod freedom_gate;
pub mod guardian;
mod kernel_continuity_client;
pub mod memory_palace;
pub mod networking;
pub mod observability;
pub mod reasoning_runtime;
pub mod resident_agent;
pub mod runtime_api;
pub mod runtime_api_auth;
pub mod runtime_v3_soak;
pub mod shutdown;
pub mod supervision;
pub mod topology;
pub mod weather;

pub const CSM_RUNTIME_OWNER: &str = "csm";
pub const ADL_TOOLING_ROLE: &str = "tooling_control_plane";
