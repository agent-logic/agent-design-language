use std::path::PathBuf;

use ed25519_dalek::VerifyingKey;
use serde::{Deserialize, Serialize};

use super::RefusalReason;

#[derive(Debug, Clone)]
pub struct CommunicationVerifyingIdentity {
    pub principal_id: String,
    pub polis_id: String,
    pub signing_key_id: String,
    pub credential_generation: u64,
    pub verifying_key: VerifyingKey,
    pub revoked: bool,
    pub not_before_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
}

impl CommunicationVerifyingIdentity {
    pub fn ensure_valid_at(&self, now_epoch_secs: u64) -> Result<(), RefusalReason> {
        if self.revoked {
            return Err(RefusalReason::IdentityRevoked);
        }
        if now_epoch_secs < self.not_before_epoch_secs
            || now_epoch_secs >= self.expires_at_epoch_secs
        {
            return Err(RefusalReason::IdentityExpired);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationKeyDescriptor {
    pub principal_id: String,
    pub polis_id: String,
    pub signing_key_id: String,
    pub credential_generation: u64,
    pub private_key_file: PathBuf,
    pub not_before_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CommunicationVerifyingDescriptor {
    pub principal_id: String,
    pub polis_id: String,
    pub signing_key_id: String,
    pub credential_generation: u64,
    pub verifying_key_hex: String,
    pub revoked: bool,
    pub not_before_epoch_secs: u64,
    pub expires_at_epoch_secs: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Layer8Principal {
    pub principal_id: String,
    pub polis_id: String,
    pub credential_generation: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeIdentityEvidence {
    pub principal_id: String,
    pub polis_id: String,
    pub signing_key_id: String,
    pub verifying_key_hex: String,
    pub credential_generation: u64,
    pub current_credential_generation: u64,
    pub expires_at_epoch_secs: u64,
    pub revoked: bool,
    pub authenticated: bool,
}

impl RuntimeIdentityEvidence {
    pub fn derive_principal(&self, now: u64) -> Result<Layer8Principal, RefusalReason> {
        if !self.authenticated
            || self.principal_id.trim().is_empty()
            || self.polis_id.trim().is_empty()
            || self.signing_key_id.trim().is_empty()
            || self.verifying_key_hex.trim().is_empty()
        {
            return Err(RefusalReason::IdentityUnavailable);
        }
        if self.revoked {
            return Err(RefusalReason::IdentityRevoked);
        }
        if now >= self.expires_at_epoch_secs {
            return Err(RefusalReason::IdentityExpired);
        }
        if self.credential_generation == 0
            || self.credential_generation != self.current_credential_generation
        {
            return Err(RefusalReason::StaleCredential);
        }
        Ok(Layer8Principal {
            principal_id: self.principal_id.clone(),
            polis_id: self.polis_id.clone(),
            credential_generation: self.credential_generation,
        })
    }
}
