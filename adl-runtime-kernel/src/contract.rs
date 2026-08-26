use std::collections::{BTreeMap, BTreeSet};

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::{ComponentId, ComponentSpec, FailurePolicy, LifecycleRole, PortSpec};

pub const SERVICE_CONTRACT_SCHEMA: &str = "adl.runtime.service_contract.v2";
pub const NONDETERMINISTIC_CAPABILITY: &str = "runtime.nondeterministic";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct Capability {
    pub name: String,
    pub version: Version,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub name: String,
    pub version: VersionReq,
    #[serde(default)]
    pub optional: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DeterminismClass {
    DeterministicCore,
    GovernedNondeterministicShell,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct LifecycleGuarantees {
    pub readiness_required: bool,
    pub bounded_shutdown_millis: u64,
    pub restart_safe: bool,
    pub idempotent_start: bool,
    pub role: LifecycleRole,
    pub required_core: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ServiceContract {
    pub schema: String,
    pub component: ComponentId,
    pub service: String,
    pub version: Version,
    pub config_schema: String,
    pub determinism: DeterminismClass,
    pub lifecycle: LifecycleGuarantees,
    pub provides: Vec<Capability>,
    pub requires: Vec<CapabilityRequirement>,
    pub inputs: Vec<PortSpec>,
    pub outputs: Vec<PortSpec>,
    pub failure_policy: FailurePolicy,
}

impl ServiceContract {
    fn validate_shape(&self) -> Result<(), ContractError> {
        if self.schema != SERVICE_CONTRACT_SCHEMA {
            return Err(ContractError::UnsupportedSchema(self.schema.clone()));
        }
        if self.service.trim().is_empty() || self.config_schema.trim().is_empty() {
            return Err(ContractError::EmptyIdentity(self.component.clone()));
        }
        if self.lifecycle.bounded_shutdown_millis == 0 {
            return Err(ContractError::UnboundedLifecycle(self.component.clone()));
        }
        if matches!(self.failure_policy, FailurePolicy::Restart { .. })
            && (!self.lifecycle.restart_safe || !self.lifecycle.idempotent_start)
        {
            return Err(ContractError::UnsafeRestartLifecycle(
                self.component.clone(),
            ));
        }
        for port in self.inputs.iter().chain(&self.outputs) {
            if port.name.trim().is_empty() || port.protocol.trim().is_empty() || port.capacity == 0
            {
                return Err(ContractError::InvalidPort {
                    component: self.component.clone(),
                    port: port.name.clone(),
                });
            }
            if port.protocol.starts_with("adl.runtime.port.") {
                return Err(ContractError::InferredPortProtocol {
                    component: self.component.clone(),
                    port: port.name.clone(),
                });
            }
        }
        let declares_nondeterminism = self
            .requires
            .iter()
            .any(|requirement| requirement.name == NONDETERMINISTIC_CAPABILITY);
        if self.determinism == DeterminismClass::DeterministicCore && declares_nondeterminism {
            return Err(ContractError::DeterminismViolation(self.component.clone()));
        }
        Ok(())
    }

    pub fn validate_component(&self, spec: &ComponentSpec) -> Result<(), ContractError> {
        self.validate_shape()?;
        if self.component != spec.id {
            return Err(ContractError::ComponentMismatch {
                contract: self.component.clone(),
                component: spec.id.clone(),
            });
        }
        if self.inputs != spec.inputs
            || self.outputs != spec.outputs
            || self.failure_policy != spec.failure_policy
        {
            return Err(ContractError::ComponentSurfaceMismatch(spec.id.clone()));
        }
        Ok(())
    }
}

#[derive(Debug, Error, Eq, PartialEq)]
pub enum ContractError {
    #[error("unsupported service contract schema: {0}")]
    UnsupportedSchema(String),
    #[error("service contract {contract} does not bind component {component}")]
    ComponentMismatch {
        contract: ComponentId,
        component: ComponentId,
    },
    #[error("service contract does not match component ports or failure policy: {0}")]
    ComponentSurfaceMismatch(ComponentId),
    #[error("service or configuration identity is empty for component: {0}")]
    EmptyIdentity(ComponentId),
    #[error("component has no bounded shutdown guarantee: {0}")]
    UnboundedLifecycle(ComponentId),
    #[error("restart policy requires restart-safe idempotent lifecycle: {0}")]
    UnsafeRestartLifecycle(ComponentId),
    #[error("contract lifecycle role/core authority does not match factory: {0}")]
    LifecycleAuthorityMismatch(ComponentId),
    #[error("component {component} has an invalid bounded port contract: {port}")]
    InvalidPort {
        component: ComponentId,
        port: String,
    },
    #[error("component {component} port {port} lacks an explicit stable protocol identity")]
    InferredPortProtocol {
        component: ComponentId,
        port: String,
    },
    #[error("deterministic component declares nondeterministic authority: {0}")]
    DeterminismViolation(ComponentId),
    #[error("duplicate service contract: {0}")]
    DuplicateService(String),
    #[error("service {service} declares capability {capability} more than once")]
    DuplicateCapability { service: String, capability: String },
    #[error("service {service} requires missing capability {capability}")]
    MissingCapability { service: String, capability: String },
    #[error("service {service} requires {capability} {required}, provider has {actual}")]
    IncompatibleCapability {
        service: String,
        capability: String,
        required: VersionReq,
        actual: Version,
    },
    #[error("deterministic service {service} depends on nondeterministic capability {capability}")]
    NondeterministicDependency { service: String, capability: String },
}

#[derive(Debug)]
pub struct ValidatedContracts {
    contracts: BTreeMap<String, ServiceContract>,
    providers: BTreeMap<String, Vec<CapabilityBinding>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CapabilityBinding {
    pub service: String,
    pub capability: Capability,
}

impl ValidatedContracts {
    pub fn contracts(&self) -> impl Iterator<Item = &ServiceContract> {
        self.contracts.values()
    }

    pub fn providers(&self, capability: &str) -> &[CapabilityBinding] {
        self.providers
            .get(capability)
            .map(Vec::as_slice)
            .unwrap_or(&[])
    }

    pub fn resolve(&self, requirement: &CapabilityRequirement) -> Option<&CapabilityBinding> {
        let matching = self
            .providers(&requirement.name)
            .iter()
            .filter(|provider| requirement.version.matches(&provider.capability.version));
        matching
            .clone()
            .find(|provider| {
                self.contracts[&provider.service].determinism == DeterminismClass::DeterministicCore
            })
            .or_else(|| matching.into_iter().next())
    }
}

pub fn validate_contracts(
    contracts: impl IntoIterator<Item = ServiceContract>,
) -> Result<ValidatedContracts, ContractError> {
    let mut by_service = BTreeMap::new();
    let mut providers = BTreeMap::new();
    for contract in contracts {
        contract.validate_shape()?;
        if by_service.contains_key(&contract.service) {
            return Err(ContractError::DuplicateService(contract.service));
        }
        let mut local = BTreeSet::new();
        for capability in &contract.provides {
            if !local.insert(capability.name.clone()) {
                return Err(ContractError::DuplicateCapability {
                    service: contract.service.clone(),
                    capability: capability.name.clone(),
                });
            }
            providers
                .entry(capability.name.clone())
                .or_insert_with(Vec::new)
                .push(CapabilityBinding {
                    service: contract.service.clone(),
                    capability: capability.clone(),
                });
        }
        by_service.insert(contract.service.clone(), contract);
    }

    for contract in by_service.values() {
        for requirement in &contract.requires {
            let Some(candidates) = providers.get(&requirement.name) else {
                if requirement.optional {
                    continue;
                }
                return Err(ContractError::MissingCapability {
                    service: contract.service.clone(),
                    capability: requirement.name.clone(),
                });
            };
            let compatible = candidates
                .iter()
                .filter(|provider| requirement.version.matches(&provider.capability.version))
                .collect::<Vec<_>>();
            if compatible.is_empty() {
                if requirement.optional {
                    continue;
                }
                return Err(ContractError::IncompatibleCapability {
                    service: contract.service.clone(),
                    capability: requirement.name.clone(),
                    required: requirement.version.clone(),
                    actual: candidates[0].capability.version.clone(),
                });
            }
            if contract.determinism == DeterminismClass::DeterministicCore
                && compatible.iter().all(|provider| {
                    by_service[&provider.service].determinism
                        == DeterminismClass::GovernedNondeterministicShell
                })
            {
                return Err(ContractError::NondeterministicDependency {
                    service: contract.service.clone(),
                    capability: requirement.name.clone(),
                });
            }
        }
    }

    for candidates in providers.values_mut() {
        candidates.sort_by(|left, right| {
            right
                .capability
                .version
                .cmp(&left.capability.version)
                .then_with(|| left.service.cmp(&right.service))
        });
    }

    Ok(ValidatedContracts {
        contracts: by_service,
        providers,
    })
}
