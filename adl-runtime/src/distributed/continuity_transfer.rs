//! Issue #210 authenticated continuity-bundle transfer core.
//!
//! This module is deliberately narrower than migration/recovery authority. It
//! consumes a committed #201 continuity-transfer artifact, validates the exact
//! #210 source/target/bundle expectation, accepts only bounded ordered frames,
//! and returns redacted transfer possession evidence. It does not expose a
//! generic transport sender and it does not activate, fence, own, serve, delete,
//! or decide migration.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::distributed::authority_protocol::{
    validate_continuity_transfer_binding, CommittedAuthorityArtifact,
    ContinuityTransferGrantArtifact, CONTINUITY_TRANSFER_ADAPTER_210,
};

const DEFAULT_MAX_FRAME_BYTES: u64 = 1024 * 1024;
const DEFAULT_MAX_TOTAL_BYTES: u64 = 256 * 1024 * 1024;
const DEFAULT_MAX_CHUNKS: usize = 65_536;
const JOURNAL_SCHEMA: &str = "adl.runtime.continuity-transfer.journal.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContinuityTransferError {
    WrongAuthority,
    WrongRoute,
    WrongSource,
    WrongTarget,
    WrongMembershipCut,
    WrongCertificateGeneration,
    WrongBootGeneration,
    Bounds,
    Gap,
    Conflict,
    Digest,
    Predecessor,
    Incomplete,
    Encoding,
    Aborted,
    CleanupAuthority,
    CorruptJournal,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityTransferPolicy {
    max_frame_bytes: u64,
    max_total_bytes: u64,
    max_chunks: usize,
}

impl Default for ContinuityTransferPolicy {
    fn default() -> Self {
        Self {
            max_frame_bytes: DEFAULT_MAX_FRAME_BYTES,
            max_total_bytes: DEFAULT_MAX_TOTAL_BYTES,
            max_chunks: DEFAULT_MAX_CHUNKS,
        }
    }
}

impl ContinuityTransferPolicy {
    pub fn bounded(max_frame_bytes: u64, max_total_bytes: u64, max_chunks: usize) -> Self {
        Self {
            max_frame_bytes,
            max_total_bytes,
            max_chunks,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityTransferExpectation {
    pub source_guardian_id: String,
    pub target_guardian_id: String,
    pub route_id: String,
    pub membership_epoch: u64,
    pub membership_log_index: u64,
    pub source_certificate_generation: u64,
    pub target_certificate_generation: u64,
    pub source_boot_generation: u64,
    pub target_boot_generation: u64,
    pub lineage_id: Vec<u8>,
    pub source_checkpoint_handle_identity: Vec<u8>,
    pub bundle_handle_identity: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityTransferFrame {
    pub transfer_id: String,
    pub chunk_index: u64,
    pub absolute_start: u64,
    pub predecessor_sha256: Option<[u8; 32]>,
    pub payload: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferFrameReceipt {
    pub transfer_id_sha256: [u8; 32],
    pub chunk_index: u64,
    pub absolute_start: u64,
    pub length: u64,
    pub payload_sha256: [u8; 32],
    pub accepted_prefix: u64,
    pub duplicate: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerifiedContinuityTransferReceipt {
    pub transfer_id_sha256: [u8; 32],
    pub source_guardian_sha256: [u8; 32],
    pub target_guardian_sha256: [u8; 32],
    pub route_sha256: [u8; 32],
    pub lineage_sha256: [u8; 32],
    pub signed_manifest_sha256: [u8; 32],
    pub signed_catalog_sha256: [u8; 32],
    pub chunk_count: u64,
    pub total_bytes: u64,
    pub final_payload_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferAbortReceipt {
    pub transfer_id_sha256: [u8; 32],
    pub cleanup_identity_sha256: [u8; 32],
    pub accepted_prefix: u64,
    pub zero_residue_attested: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferJournalState {
    pub schema: String,
    pub transfer_id_sha256: [u8; 32],
    pub accepted_prefix: u64,
    pub next_chunk_index: u64,
    pub predecessor_sha256: Option<[u8; 32]>,
    pub accepted: BTreeMap<u64, ContinuityTransferFrameReceipt>,
    pub aborted: Option<ContinuityTransferAbortReceipt>,
}

#[derive(Clone, Debug)]
pub struct ContinuityTransferSession {
    grant: ContinuityTransferGrantArtifact,
    policy: ContinuityTransferPolicy,
    accepted_prefix: u64,
    next_chunk_index: u64,
    predecessor_sha256: Option<[u8; 32]>,
    accepted: BTreeMap<u64, ContinuityTransferFrameReceipt>,
    aborted: Option<ContinuityTransferAbortReceipt>,
}

impl ContinuityTransferSession {
    pub fn open(
        artifact: &CommittedAuthorityArtifact,
        expected: ContinuityTransferExpectation,
        policy: ContinuityTransferPolicy,
    ) -> Result<Self, ContinuityTransferError> {
        validate_continuity_transfer_binding(
            artifact,
            CONTINUITY_TRANSFER_ADAPTER_210,
            &expected.lineage_id,
            &expected.source_checkpoint_handle_identity,
            &expected.bundle_handle_identity,
        )
        .map_err(|_| ContinuityTransferError::WrongAuthority)?;
        let grant: ContinuityTransferGrantArtifact = serde_json::from_slice(&artifact.bytes)
            .map_err(|_| ContinuityTransferError::Encoding)?;
        validate_expected(&grant, &expected)?;
        validate_bounds(&grant, &policy)?;
        Ok(Self {
            grant,
            policy,
            accepted_prefix: 0,
            next_chunk_index: 0,
            predecessor_sha256: None,
            accepted: BTreeMap::new(),
            aborted: None,
        })
    }

    pub fn accepted_prefix(&self) -> u64 {
        self.accepted_prefix
    }

    pub fn restore(
        artifact: &CommittedAuthorityArtifact,
        expected: ContinuityTransferExpectation,
        policy: ContinuityTransferPolicy,
        journal: ContinuityTransferJournalState,
    ) -> Result<Self, ContinuityTransferError> {
        let mut session = Self::open(artifact, expected, policy)?;
        session.apply_journal(journal)?;
        Ok(session)
    }

    pub fn journal(&self) -> ContinuityTransferJournalState {
        ContinuityTransferJournalState {
            schema: JOURNAL_SCHEMA.to_owned(),
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            accepted_prefix: self.accepted_prefix,
            next_chunk_index: self.next_chunk_index,
            predecessor_sha256: self.predecessor_sha256,
            accepted: self.accepted.clone(),
            aborted: self.aborted.clone(),
        }
    }

    pub fn accept_frame(
        &mut self,
        frame: ContinuityTransferFrame,
    ) -> Result<ContinuityTransferFrameReceipt, ContinuityTransferError> {
        if self.aborted.is_some() {
            return Err(ContinuityTransferError::Aborted);
        }
        if frame.transfer_id != self.grant.transfer_id {
            return Err(ContinuityTransferError::WrongAuthority);
        }
        let payload_len =
            u64::try_from(frame.payload.len()).map_err(|_| ContinuityTransferError::Bounds)?;
        if payload_len == 0 || payload_len > self.policy.max_frame_bytes {
            return Err(ContinuityTransferError::Bounds);
        }
        let payload_sha256 = sha256_array(&frame.payload);
        if let Some(existing) = self.accepted.get(&frame.chunk_index) {
            if existing.absolute_start == frame.absolute_start
                && existing.length == payload_len
                && existing.payload_sha256 == payload_sha256
                && frame.predecessor_sha256
                    == self
                        .grant
                        .chunks
                        .get(frame.chunk_index as usize)
                        .and_then(|chunk| chunk.predecessor_sha256)
            {
                let mut duplicate = existing.clone();
                duplicate.duplicate = true;
                return Ok(duplicate);
            }
            return Err(ContinuityTransferError::Conflict);
        }
        if frame.chunk_index != self.next_chunk_index
            || frame.absolute_start != self.accepted_prefix
        {
            return Err(ContinuityTransferError::Gap);
        }
        let expected = self
            .grant
            .chunks
            .get(frame.chunk_index as usize)
            .ok_or(ContinuityTransferError::Bounds)?;
        if expected.index != frame.chunk_index
            || expected.absolute_start != frame.absolute_start
            || expected.length != payload_len
        {
            return Err(ContinuityTransferError::Bounds);
        }
        if expected.predecessor_sha256 != frame.predecessor_sha256
            || self.predecessor_sha256 != frame.predecessor_sha256
        {
            return Err(ContinuityTransferError::Predecessor);
        }
        if expected.sha256 != payload_sha256 {
            return Err(ContinuityTransferError::Digest);
        }
        let accepted_prefix = self
            .accepted_prefix
            .checked_add(payload_len)
            .ok_or(ContinuityTransferError::Bounds)?;
        if accepted_prefix > self.grant.total_bytes || accepted_prefix > self.policy.max_total_bytes
        {
            return Err(ContinuityTransferError::Bounds);
        }
        let receipt = ContinuityTransferFrameReceipt {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            chunk_index: frame.chunk_index,
            absolute_start: frame.absolute_start,
            length: payload_len,
            payload_sha256,
            accepted_prefix,
            duplicate: false,
        };
        self.accepted.insert(frame.chunk_index, receipt.clone());
        self.accepted_prefix = accepted_prefix;
        self.next_chunk_index = self
            .next_chunk_index
            .checked_add(1)
            .ok_or(ContinuityTransferError::Bounds)?;
        self.predecessor_sha256 = Some(payload_sha256);
        Ok(receipt)
    }

    pub fn finish(&self) -> Result<VerifiedContinuityTransferReceipt, ContinuityTransferError> {
        if self.aborted.is_some() {
            return Err(ContinuityTransferError::Aborted);
        }
        if self.accepted_prefix != self.grant.total_bytes
            || self.accepted.len() != self.grant.chunks.len()
        {
            return Err(ContinuityTransferError::Incomplete);
        }
        Ok(VerifiedContinuityTransferReceipt {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            source_guardian_sha256: sha256_array(self.grant.source_guardian_id.as_bytes()),
            target_guardian_sha256: sha256_array(self.grant.target_guardian_id.as_bytes()),
            route_sha256: sha256_array(self.grant.route_id.as_bytes()),
            lineage_sha256: sha256_array(&self.grant.lineage_id),
            signed_manifest_sha256: self.grant.signed_manifest_sha256,
            signed_catalog_sha256: self.grant.signed_catalog_sha256,
            chunk_count: self.grant.chunks.len() as u64,
            total_bytes: self.grant.total_bytes,
            final_payload_sha256: self
                .predecessor_sha256
                .ok_or(ContinuityTransferError::Incomplete)?,
        })
    }

    pub fn abort(
        &mut self,
        transfer_id: &str,
        cleanup_identity: &str,
    ) -> Result<ContinuityTransferAbortReceipt, ContinuityTransferError> {
        if transfer_id != self.grant.transfer_id {
            return Err(ContinuityTransferError::WrongAuthority);
        }
        if cleanup_identity != self.grant.cleanup_identity {
            return Err(ContinuityTransferError::CleanupAuthority);
        }
        if let Some(existing) = &self.aborted {
            return Ok(existing.clone());
        }
        let receipt = ContinuityTransferAbortReceipt {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            cleanup_identity_sha256: sha256_array(self.grant.cleanup_identity.as_bytes()),
            accepted_prefix: self.accepted_prefix,
            zero_residue_attested: true,
        };
        self.aborted = Some(receipt.clone());
        Ok(receipt)
    }

    fn apply_journal(
        &mut self,
        journal: ContinuityTransferJournalState,
    ) -> Result<(), ContinuityTransferError> {
        if journal.schema != JOURNAL_SCHEMA
            || journal.transfer_id_sha256 != sha256_array(self.grant.transfer_id.as_bytes())
            || journal.accepted.len() > self.grant.chunks.len()
            || journal.next_chunk_index as usize != journal.accepted.len()
            || journal.next_chunk_index > self.grant.chunks.len() as u64
        {
            return Err(ContinuityTransferError::CorruptJournal);
        }
        let mut expected_prefix = 0_u64;
        let mut predecessor = None;
        for index in 0..journal.next_chunk_index {
            let receipt = journal
                .accepted
                .get(&index)
                .ok_or(ContinuityTransferError::CorruptJournal)?;
            let chunk = self
                .grant
                .chunks
                .get(index as usize)
                .ok_or(ContinuityTransferError::CorruptJournal)?;
            if receipt.duplicate
                || receipt.transfer_id_sha256 != journal.transfer_id_sha256
                || receipt.chunk_index != index
                || receipt.absolute_start != expected_prefix
                || receipt.length != chunk.length
                || receipt.payload_sha256 != chunk.sha256
                || receipt.accepted_prefix
                    != expected_prefix
                        .checked_add(chunk.length)
                        .ok_or(ContinuityTransferError::CorruptJournal)?
                || chunk.predecessor_sha256 != predecessor
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
            expected_prefix = receipt.accepted_prefix;
            predecessor = Some(receipt.payload_sha256);
        }
        if journal.accepted_prefix != expected_prefix || journal.predecessor_sha256 != predecessor {
            return Err(ContinuityTransferError::CorruptJournal);
        }
        if let Some(abort) = &journal.aborted {
            if abort.transfer_id_sha256 != journal.transfer_id_sha256
                || abort.cleanup_identity_sha256
                    != sha256_array(self.grant.cleanup_identity.as_bytes())
                || abort.accepted_prefix != journal.accepted_prefix
                || !abort.zero_residue_attested
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
        }
        self.accepted_prefix = journal.accepted_prefix;
        self.next_chunk_index = journal.next_chunk_index;
        self.predecessor_sha256 = journal.predecessor_sha256;
        self.accepted = journal.accepted;
        self.aborted = journal.aborted;
        Ok(())
    }
}

fn validate_expected(
    grant: &ContinuityTransferGrantArtifact,
    expected: &ContinuityTransferExpectation,
) -> Result<(), ContinuityTransferError> {
    if grant.source_guardian_id != expected.source_guardian_id {
        return Err(ContinuityTransferError::WrongSource);
    }
    if grant.target_guardian_id != expected.target_guardian_id {
        return Err(ContinuityTransferError::WrongTarget);
    }
    if grant.route_id != expected.route_id {
        return Err(ContinuityTransferError::WrongRoute);
    }
    if grant.membership_epoch != expected.membership_epoch
        || grant.membership_log_index != expected.membership_log_index
    {
        return Err(ContinuityTransferError::WrongMembershipCut);
    }
    if grant.source_certificate_generation != expected.source_certificate_generation
        || grant.target_certificate_generation != expected.target_certificate_generation
    {
        return Err(ContinuityTransferError::WrongCertificateGeneration);
    }
    if grant.source_boot_generation != expected.source_boot_generation
        || grant.target_boot_generation != expected.target_boot_generation
    {
        return Err(ContinuityTransferError::WrongBootGeneration);
    }
    Ok(())
}

fn validate_bounds(
    grant: &ContinuityTransferGrantArtifact,
    policy: &ContinuityTransferPolicy,
) -> Result<(), ContinuityTransferError> {
    if grant.total_bytes > policy.max_total_bytes || grant.chunks.len() > policy.max_chunks {
        return Err(ContinuityTransferError::Bounds);
    }
    if grant
        .chunks
        .iter()
        .any(|chunk| chunk.length > policy.max_frame_bytes)
    {
        return Err(ContinuityTransferError::Bounds);
    }
    Ok(())
}

fn sha256_array(bytes: &[u8]) -> [u8; 32] {
    <[u8; 32]>::from(Sha256::digest(bytes))
}
