//! Shared Runtime v2 agent lifecycle transition authority.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuntimeV2AgentLifecycleState {
    Active,
    Quiescent,
    Suspended,
    Dormant,
    Simulation,
    InTransit,
    Bootstrap,
    Shutdown,
    ForcedSuspension,
    Quarantined,
    Rejected,
    Orphaned,
}

impl RuntimeV2AgentLifecycleState {
    pub fn from_contract_name(value: &str) -> Option<Self> {
        Some(match value {
            "ACTIVE" => Self::Active,
            "QUIESCENT" => Self::Quiescent,
            "SUSPENDED" => Self::Suspended,
            "DORMANT" => Self::Dormant,
            "SIMULATION" => Self::Simulation,
            "IN_TRANSIT" => Self::InTransit,
            "BOOTSTRAP" => Self::Bootstrap,
            "SHUTDOWN" => Self::Shutdown,
            "FORCED_SUSPENSION" => Self::ForcedSuspension,
            "QUARANTINED" => Self::Quarantined,
            "REJECTED" => Self::Rejected,
            "ORPHANED" => Self::Orphaned,
            _ => return None,
        })
    }

    pub const fn contract_name(self) -> &'static str {
        match self {
            Self::Active => "ACTIVE",
            Self::Quiescent => "QUIESCENT",
            Self::Suspended => "SUSPENDED",
            Self::Dormant => "DORMANT",
            Self::Simulation => "SIMULATION",
            Self::InTransit => "IN_TRANSIT",
            Self::Bootstrap => "BOOTSTRAP",
            Self::Shutdown => "SHUTDOWN",
            Self::ForcedSuspension => "FORCED_SUSPENSION",
            Self::Quarantined => "QUARANTINED",
            Self::Rejected => "REJECTED",
            Self::Orphaned => "ORPHANED",
        }
    }

    pub const fn allows(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Bootstrap, Self::Active)
                | (Self::Active, Self::Quiescent)
                | (Self::Quiescent, Self::Active)
                | (Self::Active, Self::Suspended)
                | (Self::Suspended, Self::Active)
                | (Self::Suspended, Self::Dormant)
                | (Self::Dormant, Self::Active)
                | (Self::Active, Self::Simulation)
                | (Self::Active, Self::InTransit)
                | (Self::Active, Self::Shutdown)
                | (Self::Active, Self::ForcedSuspension)
                | (Self::ForcedSuspension, Self::Quarantined)
                | (Self::Bootstrap, Self::Rejected)
                | (Self::Active, Self::Orphaned)
        )
    }

    pub fn transition(&mut self, next: Self) -> Result<(), String> {
        if !self.allows(next) {
            return Err(format!(
                "Runtime v2 lifecycle denies {} -> {}",
                self.contract_name(),
                next.contract_name()
            ));
        }
        *self = next;
        Ok(())
    }
}
