use std::{
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::{BirthdayCandidate, BirthdayDecision};

pub const PRODUCTION_BIRTHDAY_SCHEMA: &str = "adl.runtime.production_birthday.v1";
pub const PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA: &str =
    "adl.runtime.production_birthday.tool_binding.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedToolAuthorityBinding {
    pub schema: String,
    pub resident_id: String,
    pub cycle_id: String,
    pub continuity_head_sha256: String,
    pub capability_envelope_sha256: String,
    pub cognitive_profile_sha256: String,
    pub implementation_revision_sha256: String,
    pub decision: String,
    pub authentication_key_id: String,
    pub receipt_sha256: String,
    pub authentication_sha256: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionBirthdayInput {
    pub resident_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub implementation_revision_sha256: String,
    pub identity_root_sha256: String,
    pub continuity_head_sha256: String,
    pub memory_palace_authority_sha256: String,
    pub capability_envelope_sha256: String,
    pub cognitive_profile_sha256: String,
    pub adaptive_learning_receipt_sha256: String,
    pub witness_packet_sha256: String,
    pub candidate: BirthdayCandidate,
    pub decision: BirthdayDecision,
    pub tool_authority: VerifiedToolAuthorityBinding,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProductionBirthdayReceipt {
    pub schema: String,
    pub generation: u64,
    pub resident_id: String,
    pub cycle_id: String,
    pub transaction_id: String,
    pub identity_root_sha256: String,
    pub continuity_head_sha256: String,
    pub memory_palace_authority_sha256: String,
    pub capability_envelope_sha256: String,
    pub cognitive_profile_sha256: String,
    pub adaptive_learning_receipt_sha256: String,
    pub tool_authority_receipt_sha256: String,
    pub witness_packet_sha256: String,
    pub candidate_packet_sha256: String,
    pub implementation_revision_sha256: String,
    pub previous_receipt_sha256: Option<String>,
    pub receipt_sha256: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProductionBirthdayFailpoint {
    AfterIntentSync,
    AfterReceiptSync,
    AfterCommitRename,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ProductionBirthdayError {
    InvalidInput,
    BirthdayDenied,
    CrossBindingMismatch,
    AlreadyCommitted,
    ConflictingTransaction,
    PendingRecoveryRequired,
    CorruptState,
    DurableWrite,
    InjectedInterruption,
}

#[derive(Clone)]
pub struct ProductionBirthdayStore {
    root: PathBuf,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct PendingIntent {
    schema: String,
    resident_id: String,
    transaction_id: String,
    input_sha256: String,
}

impl ProductionBirthdayStore {
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, ProductionBirthdayError> {
        let root = root.into();
        fs::create_dir_all(&root).map_err(|_| ProductionBirthdayError::DurableWrite)?;
        let metadata =
            fs::symlink_metadata(&root).map_err(|_| ProductionBirthdayError::DurableWrite)?;
        if metadata.file_type().is_symlink() || !metadata.is_dir() {
            return Err(ProductionBirthdayError::InvalidInput);
        }
        Ok(Self { root })
    }

    pub fn activate(
        &self,
        input: &ProductionBirthdayInput,
    ) -> Result<ProductionBirthdayReceipt, ProductionBirthdayError> {
        self.activate_with_failpoint(input, None)
    }

    pub fn restore(
        &self,
        resident_id: &str,
    ) -> Result<Option<ProductionBirthdayReceipt>, ProductionBirthdayError> {
        validate_identifier(resident_id)?;
        let path = self.committed_path(resident_id);
        if !path.exists() {
            return Ok(None);
        }
        let bytes = fs::read(path).map_err(|_| ProductionBirthdayError::CorruptState)?;
        let receipt: ProductionBirthdayReceipt =
            serde_json::from_slice(&bytes).map_err(|_| ProductionBirthdayError::CorruptState)?;
        validate_receipt(&receipt)?;
        Ok(Some(receipt))
    }

    pub fn recover_pending(
        &self,
        input: &ProductionBirthdayInput,
    ) -> Result<ProductionBirthdayReceipt, ProductionBirthdayError> {
        validate_input(input)?;
        if let Some(receipt) = self.restore(&input.resident_id)? {
            if !receipt_matches_input(&receipt, input) {
                return Err(ProductionBirthdayError::AlreadyCommitted);
            }
            let pending = self.pending_path(&input.resident_id);
            if pending.exists() {
                fs::remove_file(pending).map_err(|_| ProductionBirthdayError::DurableWrite)?;
                sync_directory(&self.root)?;
            }
            return Ok(receipt);
        }
        let pending_path = self.pending_path(&input.resident_id);
        let pending: PendingIntent = serde_json::from_slice(
            &fs::read(&pending_path)
                .map_err(|_| ProductionBirthdayError::PendingRecoveryRequired)?,
        )
        .map_err(|_| ProductionBirthdayError::CorruptState)?;
        if pending.resident_id != input.resident_id
            || pending.transaction_id != input.transaction_id
            || pending.input_sha256 != canonical_sha256(input)?
        {
            return Err(ProductionBirthdayError::ConflictingTransaction);
        }
        fs::remove_file(&pending_path).map_err(|_| ProductionBirthdayError::DurableWrite)?;
        let staging = self.staging_path(&input.resident_id);
        if staging.exists() {
            fs::remove_file(staging).map_err(|_| ProductionBirthdayError::DurableWrite)?;
        }
        sync_directory(&self.root)?;
        self.activate(input)
    }

    pub fn activate_with_failpoint(
        &self,
        input: &ProductionBirthdayInput,
        failpoint: Option<ProductionBirthdayFailpoint>,
    ) -> Result<ProductionBirthdayReceipt, ProductionBirthdayError> {
        validate_input(input)?;
        let _ownership = Ownership::acquire(self.lock_path(&input.resident_id))?;
        if let Some(receipt) = self.restore(&input.resident_id)? {
            return if receipt_matches_input(&receipt, input) {
                Ok(receipt)
            } else {
                Err(ProductionBirthdayError::AlreadyCommitted)
            };
        }
        if self.pending_path(&input.resident_id).exists() {
            return Err(ProductionBirthdayError::PendingRecoveryRequired);
        }
        let intent = PendingIntent {
            schema: "adl.runtime.production_birthday.pending.v1".to_owned(),
            resident_id: input.resident_id.clone(),
            transaction_id: input.transaction_id.clone(),
            input_sha256: canonical_sha256(input)?,
        };
        write_create_new_synced(&self.pending_path(&input.resident_id), &intent)?;
        sync_directory(&self.root)?;
        if failpoint == Some(ProductionBirthdayFailpoint::AfterIntentSync) {
            return Err(ProductionBirthdayError::InjectedInterruption);
        }
        let mut receipt = receipt_from_input(input);
        receipt.receipt_sha256 = receipt_digest(&receipt)?;
        let staging = self.staging_path(&input.resident_id);
        write_create_new_synced(&staging, &receipt)?;
        if failpoint == Some(ProductionBirthdayFailpoint::AfterReceiptSync) {
            return Err(ProductionBirthdayError::InjectedInterruption);
        }
        fs::rename(&staging, self.committed_path(&input.resident_id))
            .map_err(|_| ProductionBirthdayError::DurableWrite)?;
        if failpoint == Some(ProductionBirthdayFailpoint::AfterCommitRename) {
            return Err(ProductionBirthdayError::InjectedInterruption);
        }
        sync_directory(&self.root)?;
        fs::remove_file(self.pending_path(&input.resident_id))
            .map_err(|_| ProductionBirthdayError::DurableWrite)?;
        sync_directory(&self.root)?;
        Ok(receipt)
    }

    fn lock_path(&self, resident: &str) -> PathBuf {
        self.root.join(format!("{resident}.lock"))
    }
    fn pending_path(&self, resident: &str) -> PathBuf {
        self.root.join(format!("{resident}.pending.json"))
    }
    fn staging_path(&self, resident: &str) -> PathBuf {
        self.root.join(format!(".{resident}.receipt.stage"))
    }
    fn committed_path(&self, resident: &str) -> PathBuf {
        self.root.join(format!("{resident}.receipt.json"))
    }
}

struct Ownership {
    path: PathBuf,
}
impl Ownership {
    fn acquire(path: PathBuf) -> Result<Self, ProductionBirthdayError> {
        OpenOptions::new()
            .create_new(true)
            .write(true)
            .open(&path)
            .map_err(|error| {
                if error.kind() == std::io::ErrorKind::AlreadyExists {
                    ProductionBirthdayError::ConflictingTransaction
                } else {
                    ProductionBirthdayError::DurableWrite
                }
            })?
            .sync_all()
            .map_err(|_| ProductionBirthdayError::DurableWrite)?;
        Ok(Self { path })
    }
}
impl Drop for Ownership {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

fn validate_input(input: &ProductionBirthdayInput) -> Result<(), ProductionBirthdayError> {
    validate_identifier(&input.resident_id)?;
    validate_identifier(&input.cycle_id)?;
    validate_identifier(&input.transaction_id)?;
    validate_identifier(&input.tool_authority.authentication_key_id)?;
    let hashes = [
        &input.implementation_revision_sha256,
        &input.identity_root_sha256,
        &input.continuity_head_sha256,
        &input.memory_palace_authority_sha256,
        &input.capability_envelope_sha256,
        &input.cognitive_profile_sha256,
        &input.adaptive_learning_receipt_sha256,
        &input.witness_packet_sha256,
        &input.tool_authority.receipt_sha256,
        &input.tool_authority.authentication_sha256,
    ];
    if hashes.iter().any(|value| !is_sha256(value))
        || input.decision.candidate_id != input.candidate.candidate_id
        || !input.decision.accepted
        || !input.decision.rejections.is_empty()
    {
        return Err(ProductionBirthdayError::BirthdayDenied);
    }
    if input.candidate.identity_root != input.identity_root_sha256
        || input.candidate.continuity_head != input.continuity_head_sha256
        || input.candidate.packet_sha256
            != crate::candidate_digest(&input.candidate)
                .map_err(|_| ProductionBirthdayError::InvalidInput)?
        || input.tool_authority.schema != PRODUCTION_BIRTHDAY_TOOL_BINDING_SCHEMA
        || input.tool_authority.resident_id != input.resident_id
        || input.tool_authority.cycle_id != input.cycle_id
        || input.tool_authority.continuity_head_sha256 != input.continuity_head_sha256
        || input.tool_authority.capability_envelope_sha256 != input.capability_envelope_sha256
        || input.tool_authority.cognitive_profile_sha256 != input.cognitive_profile_sha256
        || input.tool_authority.implementation_revision_sha256
            != input.implementation_revision_sha256
        || input.tool_authority.decision != "executed"
    {
        return Err(ProductionBirthdayError::CrossBindingMismatch);
    }
    Ok(())
}

fn receipt_from_input(input: &ProductionBirthdayInput) -> ProductionBirthdayReceipt {
    ProductionBirthdayReceipt {
        schema: PRODUCTION_BIRTHDAY_SCHEMA.to_owned(),
        generation: 1,
        resident_id: input.resident_id.clone(),
        cycle_id: input.cycle_id.clone(),
        transaction_id: input.transaction_id.clone(),
        identity_root_sha256: input.identity_root_sha256.clone(),
        continuity_head_sha256: input.continuity_head_sha256.clone(),
        memory_palace_authority_sha256: input.memory_palace_authority_sha256.clone(),
        capability_envelope_sha256: input.capability_envelope_sha256.clone(),
        cognitive_profile_sha256: input.cognitive_profile_sha256.clone(),
        adaptive_learning_receipt_sha256: input.adaptive_learning_receipt_sha256.clone(),
        tool_authority_receipt_sha256: input.tool_authority.receipt_sha256.clone(),
        witness_packet_sha256: input.witness_packet_sha256.clone(),
        candidate_packet_sha256: input.candidate.packet_sha256.clone(),
        implementation_revision_sha256: input.implementation_revision_sha256.clone(),
        previous_receipt_sha256: None,
        receipt_sha256: String::new(),
    }
}
fn receipt_matches_input(
    receipt: &ProductionBirthdayReceipt,
    input: &ProductionBirthdayInput,
) -> bool {
    receipt.resident_id == input.resident_id
        && receipt.cycle_id == input.cycle_id
        && receipt.transaction_id == input.transaction_id
        && receipt.candidate_packet_sha256 == input.candidate.packet_sha256
        && receipt.implementation_revision_sha256 == input.implementation_revision_sha256
}
fn validate_receipt(receipt: &ProductionBirthdayReceipt) -> Result<(), ProductionBirthdayError> {
    if receipt.schema != PRODUCTION_BIRTHDAY_SCHEMA
        || receipt.generation != 1
        || receipt.receipt_sha256 != receipt_digest(receipt)?
    {
        Err(ProductionBirthdayError::CorruptState)
    } else {
        Ok(())
    }
}
fn receipt_digest(receipt: &ProductionBirthdayReceipt) -> Result<String, ProductionBirthdayError> {
    let mut value = receipt.clone();
    value.receipt_sha256.clear();
    canonical_sha256(&value)
}
fn canonical_sha256<T: Serialize>(value: &T) -> Result<String, ProductionBirthdayError> {
    serde_jcs::to_vec(value)
        .map(|b| hex::encode(Sha256::digest(b)))
        .map_err(|_| ProductionBirthdayError::InvalidInput)
}
fn validate_identifier(value: &str) -> Result<(), ProductionBirthdayError> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
    {
        Err(ProductionBirthdayError::InvalidInput)
    } else {
        Ok(())
    }
}
fn is_sha256(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn write_create_new_synced<T: Serialize>(
    path: &Path,
    value: &T,
) -> Result<(), ProductionBirthdayError> {
    let bytes = serde_jcs::to_vec(value).map_err(|_| ProductionBirthdayError::InvalidInput)?;
    let mut file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|_| ProductionBirthdayError::DurableWrite)?;
    file.write_all(&bytes)
        .and_then(|_| file.sync_all())
        .map_err(|_| ProductionBirthdayError::DurableWrite)
}
fn sync_directory(path: &Path) -> Result<(), ProductionBirthdayError> {
    OpenOptions::new()
        .read(true)
        .open(path)
        .and_then(|f| f.sync_all())
        .map_err(|_| ProductionBirthdayError::DurableWrite)
}
