//! Independent, additive runtime-kernel proof for ADL issue #5170.

pub mod acip;
pub mod adaptive_learning;
pub mod agent_roster;
pub mod assembly;
pub mod birth_witness;
pub mod birthday;
pub mod birthday_continuity;
pub mod birthday_identity;
pub mod capability_envelope;
pub mod channel;
pub mod cognition;
pub mod cognitive_profile;
pub mod component;
pub mod config;
pub mod continuity;
pub mod continuity_control;
pub mod contract;
pub mod control;
pub mod conversation_continuity;
pub mod conversation_history;
pub mod conversation_journal;
pub mod conversation_rooms;
pub mod durable_state;
pub mod governance;
pub mod identity_memory;
pub mod ingress;
pub mod layer8_authority;
pub mod live_continuity;
pub mod memory_palace;
pub mod memory_palace_authority;
pub mod operations;
pub mod operator_attention;
pub mod parity;
pub mod parity_b;
pub mod private_state;
pub mod production_birthday;
pub mod proof;
pub mod protocol_adapters;
pub mod reasoning;
pub mod resident_cycle;
pub mod shepherd;
pub mod supervisor;
pub mod telemetry;
pub mod time;
pub mod tls;
pub mod topology;
pub mod weather;

#[cfg(feature = "test-support")]
#[doc(hidden)]
pub mod test_support;

#[cfg(test)]
mod conversation_sessions_tests;

pub use acip::*;
pub use adaptive_learning::*;
pub use agent_roster::*;
pub use assembly::*;
pub use birth_witness::*;
pub use birthday::*;
pub use birthday_continuity::*;
pub use birthday_identity::*;
pub use capability_envelope::*;
pub use channel::{channel, BoundedReceiver, BoundedSender, ChannelFullPolicy, SendError};
pub use cognition::*;
pub use cognitive_profile::*;
pub use component::{
    Component, ComponentContext, ComponentError, ComponentFactory, ComponentId, ComponentPorts,
    ComponentSpec, FailurePolicy, LifecycleRole, PortAccessError, PortProtocol, PortSpec,
    RunningState, SupervisionScope,
};
pub use config::*;
pub use continuity::*;
pub use continuity_control::*;
pub use contract::*;
pub use control::*;
pub use conversation_continuity::*;
pub use conversation_history::*;
pub use conversation_journal::*;
pub use conversation_rooms::*;
pub use durable_state::*;
pub use governance::*;
pub use identity_memory::*;
pub use ingress::*;
pub use live_continuity::*;
pub use memory_palace::*;
pub use memory_palace_authority::*;
pub use operations::*;
pub use operator_attention::*;
pub use parity::*;
pub use parity_b::*;
pub use private_state::*;
pub use production_birthday::*;
pub use protocol_adapters::{
    build_production_operation_executors as build_protocol_production_operation_executors,
    protocol_operation_executors, protocol_operation_executors_from_env, ProtocolAdapter,
    ProtocolBuildError, ProtocolEndpoint, ProtocolFrame, ProtocolResponse, ProtocolSecret,
    ProtocolSecurity, ProtocolStatus, MAX_PROTOCOL_FRAME_FRESHNESS_MILLIS,
    MAX_PROTOCOL_RESPONSE_BYTES, PROTOCOL_FRAME_SCHEMA, PROTOCOL_RESPONSE_SCHEMA,
};
pub use reasoning::*;
pub use resident_cycle::*;
pub use shepherd::*;
pub use supervisor::{Kernel, KernelControl, KernelError, KernelExit, KernelHandle};
pub use telemetry::*;
pub use time::*;
pub use tls::*;
pub use topology::{
    ComponentRegistry, ConfiguredTopology, FactoryRegistration, FactoryRegistry, TopologyError,
    ValidatedPortRoute, ValidatedTopology,
};
pub use weather::*;
