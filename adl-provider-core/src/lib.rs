//! Provider-only DTOs, transport adapters and validated definition candidates.
//! This leaf has no dependency on ADL documents or Runtime lifecycle packages.
mod spec;
pub use spec::*;
pub mod model_identity;
pub mod profiles;
pub mod provider;
pub mod provider_substrate;
pub use provider::*;
pub mod candidate;
// Registry module is added by the runtime composition owner.

pub mod registry;
