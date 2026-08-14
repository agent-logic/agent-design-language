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

use adl_runtime_kernel::{SourceCheckpointHandle, TargetStageHandle};

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
    WrongTrustDomain,
    WrongPolis,
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
    Deadline,
    Storage,
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
    pub trust_domain: String,
    pub polis_id: String,
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
    pub observed_unix_millis: u64,
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
    pub possession_evidence_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferEndpointReceipt {
    pub transfer_id_sha256: [u8; 32],
    pub source_checkpoint_handle_sha256: [u8; 32],
    pub target_stage_handle_sha256: [u8; 32],
    pub accepted_prefix: u64,
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
pub struct ContinuityTransferCleanupRequest {
    pub transfer_id_sha256: [u8; 32],
    pub cleanup_identity_sha256: [u8; 32],
    pub accepted_prefix: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetContinuityFrameEffect {
    pub transfer_id_sha256: [u8; 32],
    pub target_stage_handle_sha256: [u8; 32],
    pub chunk_index: u64,
    pub absolute_start: u64,
    pub length: u64,
    pub payload_sha256: [u8; 32],
    pub predecessor_sha256: Option<[u8; 32]>,
    pub accepted_prefix: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetContinuityFrameEffectReceipt {
    pub transfer_id_sha256: [u8; 32],
    pub chunk_index: u64,
    pub accepted_prefix: u64,
    pub payload_sha256: [u8; 32],
    pub verifier_prefix_sha256: [u8; 32],
    pub fsync_attested: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TargetContinuityPossessionEvidence {
    pub transfer_id_sha256: [u8; 32],
    pub target_stage_handle_sha256: [u8; 32],
    pub accepted_prefix: u64,
    pub total_bytes: u64,
    pub final_payload_sha256: [u8; 32],
    pub possession_evidence_sha256: [u8; 32],
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ContinuityTransferJournalState {
    pub schema: String,
    pub transfer_id_sha256: [u8; 32],
    pub accepted_prefix: u64,
    pub next_chunk_index: u64,
    pub predecessor_sha256: Option<[u8; 32]>,
    pub pending_effect: Option<TargetContinuityFrameEffect>,
    pub pending_abort: Option<ContinuityTransferCleanupRequest>,
    pub accepted: BTreeMap<u64, ContinuityTransferFrameReceipt>,
    pub aborted: Option<ContinuityTransferAbortReceipt>,
    pub completed: Option<VerifiedContinuityTransferReceipt>,
}

pub trait ContinuityTransferJournalWriter {
    fn write_journal(
        &mut self,
        journal: &ContinuityTransferJournalState,
    ) -> Result<(), ContinuityTransferError>;
}

pub trait ContinuityTransferCleanupAuthority {
    fn discard(
        &mut self,
        request: ContinuityTransferCleanupRequest,
    ) -> Result<ContinuityTransferAbortReceipt, ContinuityTransferError>;
}

pub trait TargetContinuityEffectPort {
    fn write_frame(
        &mut self,
        effect: &TargetContinuityFrameEffect,
        payload: &[u8],
    ) -> Result<TargetContinuityFrameEffectReceipt, ContinuityTransferError>;

    fn verify_possession(
        &mut self,
        transfer_id_sha256: [u8; 32],
        target_stage_handle_sha256: [u8; 32],
        accepted_prefix: u64,
        total_bytes: u64,
        final_payload_sha256: [u8; 32],
    ) -> Result<TargetContinuityPossessionEvidence, ContinuityTransferError>;
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
    completed: Option<VerifiedContinuityTransferReceipt>,
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
            completed: None,
        })
    }

    pub fn accepted_prefix(&self) -> u64 {
        self.accepted_prefix
    }

    pub fn bind_real_endpoints(
        &self,
        source: &SourceCheckpointHandle,
        target: &TargetStageHandle,
    ) -> Result<ContinuityTransferEndpointReceipt, ContinuityTransferError> {
        let source_identity = canonical_identity(source)?;
        if source_identity != self.grant.source_checkpoint_handle_identity {
            return Err(ContinuityTransferError::WrongSource);
        }
        let target_identity = canonical_identity(target)?;
        if target_identity != self.grant.bundle_handle_identity {
            return Err(ContinuityTransferError::WrongTarget);
        }
        if source.generation() != self.grant.source_boot_generation
            || target.root_generation() != self.grant.target_boot_generation
        {
            return Err(ContinuityTransferError::WrongBootGeneration);
        }
        Ok(ContinuityTransferEndpointReceipt {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            source_checkpoint_handle_sha256: sha256_array(&source_identity),
            target_stage_handle_sha256: sha256_array(&target_identity),
            accepted_prefix: self.accepted_prefix,
        })
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
            pending_effect: None,
            pending_abort: None,
            accepted: self.accepted.clone(),
            aborted: self.aborted.clone(),
            completed: self.completed.clone(),
        }
    }

    pub fn checkpoint_journal(
        &self,
        writer: &mut dyn ContinuityTransferJournalWriter,
    ) -> Result<ContinuityTransferJournalState, ContinuityTransferError> {
        let journal = self.journal();
        writer.write_journal(&journal)?;
        Ok(journal)
    }

    pub fn accept_frame(
        &mut self,
        frame: ContinuityTransferFrame,
        target: &mut dyn TargetContinuityEffectPort,
        writer: &mut dyn ContinuityTransferJournalWriter,
    ) -> Result<ContinuityTransferFrameReceipt, ContinuityTransferError> {
        let mut staged = self.clone();
        let payload = frame.payload.clone();
        let receipt = staged.accept_frame_unpersisted(frame)?;
        if !receipt.duplicate {
            let effect = staged.target_effect(&receipt);
            let mut pending_journal = self.journal();
            pending_journal.pending_effect = Some(effect.clone());
            writer.write_journal(&pending_journal)?;
            Self::write_target_effect(&effect, &payload, target)?;
            writer.write_journal(&staged.journal())?;
            *self = staged;
        }
        Ok(receipt)
    }

    fn accept_frame_unpersisted(
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
        validate_observed_deadline(frame.observed_unix_millis, &self.grant.inclusive_deadline)?;
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
        self.completed = None;
        Ok(receipt)
    }

    pub fn finish(
        &mut self,
        target: &mut dyn TargetContinuityEffectPort,
    ) -> Result<VerifiedContinuityTransferReceipt, ContinuityTransferError> {
        if self.aborted.is_some() {
            return Err(ContinuityTransferError::Aborted);
        }
        if let Some(existing) = &self.completed {
            return Ok(existing.clone());
        }
        if self.accepted_prefix != self.grant.total_bytes
            || self.accepted.len() != self.grant.chunks.len()
        {
            return Err(ContinuityTransferError::Incomplete);
        }
        let final_payload_sha256 = self
            .predecessor_sha256
            .ok_or(ContinuityTransferError::Incomplete)?;
        let possession = target.verify_possession(
            sha256_array(self.grant.transfer_id.as_bytes()),
            sha256_array(&self.grant.bundle_handle_identity),
            self.accepted_prefix,
            self.grant.total_bytes,
            final_payload_sha256,
        )?;
        self.validate_possession(&possession, final_payload_sha256)?;
        let receipt = completion_receipt(
            &self.grant,
            final_payload_sha256,
            possession.possession_evidence_sha256,
        );
        self.completed = Some(receipt.clone());
        Ok(receipt)
    }

    pub fn abort(
        &mut self,
        transfer_id: &str,
        cleanup_identity: &str,
        cleanup: &mut dyn ContinuityTransferCleanupAuthority,
        writer: &mut dyn ContinuityTransferJournalWriter,
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
        let mut staged = self.clone();
        let request = ContinuityTransferCleanupRequest {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            cleanup_identity_sha256: sha256_array(self.grant.cleanup_identity.as_bytes()),
            accepted_prefix: self.accepted_prefix,
        };
        let mut pending_journal = self.journal();
        pending_journal.pending_abort = Some(request.clone());
        writer.write_journal(&pending_journal)?;
        let receipt = cleanup.discard(request.clone())?;
        if receipt.transfer_id_sha256 != request.transfer_id_sha256
            || receipt.cleanup_identity_sha256 != request.cleanup_identity_sha256
            || receipt.accepted_prefix != request.accepted_prefix
            || !receipt.zero_residue_attested
        {
            return Err(ContinuityTransferError::CleanupAuthority);
        }
        staged.aborted = Some(receipt.clone());
        staged.completed = None;
        writer.write_journal(&staged.journal())?;
        *self = staged;
        Ok(receipt)
    }

    fn target_effect(
        &self,
        receipt: &ContinuityTransferFrameReceipt,
    ) -> TargetContinuityFrameEffect {
        TargetContinuityFrameEffect {
            transfer_id_sha256: sha256_array(self.grant.transfer_id.as_bytes()),
            target_stage_handle_sha256: sha256_array(&self.grant.bundle_handle_identity),
            chunk_index: receipt.chunk_index,
            absolute_start: receipt.absolute_start,
            length: receipt.length,
            payload_sha256: receipt.payload_sha256,
            predecessor_sha256: self
                .grant
                .chunks
                .get(receipt.chunk_index as usize)
                .and_then(|chunk| chunk.predecessor_sha256),
            accepted_prefix: receipt.accepted_prefix,
        }
    }

    fn write_target_effect(
        effect: &TargetContinuityFrameEffect,
        payload: &[u8],
        target: &mut dyn TargetContinuityEffectPort,
    ) -> Result<(), ContinuityTransferError> {
        let target_receipt = target.write_frame(effect, payload)?;
        if target_receipt.transfer_id_sha256 != effect.transfer_id_sha256
            || target_receipt.chunk_index != effect.chunk_index
            || target_receipt.accepted_prefix != effect.accepted_prefix
            || target_receipt.payload_sha256 != effect.payload_sha256
            || target_receipt.verifier_prefix_sha256 == [0; 32]
            || !target_receipt.fsync_attested
        {
            return Err(ContinuityTransferError::Storage);
        }
        Ok(())
    }

    fn validate_possession(
        &self,
        possession: &TargetContinuityPossessionEvidence,
        final_payload_sha256: [u8; 32],
    ) -> Result<(), ContinuityTransferError> {
        if possession.transfer_id_sha256 != sha256_array(self.grant.transfer_id.as_bytes())
            || possession.target_stage_handle_sha256
                != sha256_array(&self.grant.bundle_handle_identity)
            || possession.accepted_prefix != self.accepted_prefix
            || possession.total_bytes != self.grant.total_bytes
            || possession.final_payload_sha256 != final_payload_sha256
            || possession.possession_evidence_sha256 == [0; 32]
        {
            return Err(ContinuityTransferError::Storage);
        }
        Ok(())
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
        if let Some(pending) = &journal.pending_effect {
            let chunk = self
                .grant
                .chunks
                .get(journal.next_chunk_index as usize)
                .ok_or(ContinuityTransferError::CorruptJournal)?;
            let pending_prefix = journal
                .accepted_prefix
                .checked_add(chunk.length)
                .ok_or(ContinuityTransferError::CorruptJournal)?;
            if pending.transfer_id_sha256 != journal.transfer_id_sha256
                || pending.target_stage_handle_sha256
                    != sha256_array(&self.grant.bundle_handle_identity)
                || pending.chunk_index != journal.next_chunk_index
                || pending.absolute_start != journal.accepted_prefix
                || pending.length != chunk.length
                || pending.payload_sha256 != chunk.sha256
                || pending.predecessor_sha256 != chunk.predecessor_sha256
                || pending.accepted_prefix != pending_prefix
                || journal.aborted.is_some()
                || journal.completed.is_some()
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
        }
        if let Some(pending_abort) = &journal.pending_abort {
            if pending_abort.transfer_id_sha256 != journal.transfer_id_sha256
                || pending_abort.cleanup_identity_sha256
                    != sha256_array(self.grant.cleanup_identity.as_bytes())
                || pending_abort.accepted_prefix != journal.accepted_prefix
                || journal.aborted.is_some()
                || journal.completed.is_some()
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
        }
        if journal.pending_effect.is_some() && journal.pending_abort.is_some() {
            return Err(ContinuityTransferError::CorruptJournal);
        }
        if let Some(abort) = &journal.aborted {
            if abort.transfer_id_sha256 != journal.transfer_id_sha256
                || abort.cleanup_identity_sha256
                    != sha256_array(self.grant.cleanup_identity.as_bytes())
                || abort.accepted_prefix != journal.accepted_prefix
                || !abort.zero_residue_attested
                || journal.completed.is_some()
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
        }
        if let Some(completed) = &journal.completed {
            let final_payload_sha256 = journal
                .predecessor_sha256
                .ok_or(ContinuityTransferError::CorruptJournal)?;
            if journal.aborted.is_some()
                || journal.accepted_prefix != self.grant.total_bytes
                || journal.accepted.len() != self.grant.chunks.len()
                || completed.possession_evidence_sha256 == [0; 32]
                || completed
                    != &completion_receipt(
                        &self.grant,
                        final_payload_sha256,
                        completed.possession_evidence_sha256,
                    )
            {
                return Err(ContinuityTransferError::CorruptJournal);
            }
        }
        self.accepted_prefix = journal.accepted_prefix;
        self.next_chunk_index = journal.next_chunk_index;
        self.predecessor_sha256 = journal.predecessor_sha256;
        self.accepted = journal.accepted;
        self.aborted = journal.aborted;
        self.completed = journal.completed;
        Ok(())
    }
}

fn validate_expected(
    grant: &ContinuityTransferGrantArtifact,
    expected: &ContinuityTransferExpectation,
) -> Result<(), ContinuityTransferError> {
    if grant.trust_domain != expected.trust_domain {
        return Err(ContinuityTransferError::WrongTrustDomain);
    }
    if grant.polis_id != expected.polis_id {
        return Err(ContinuityTransferError::WrongPolis);
    }
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

fn validate_observed_deadline(
    observed_unix_millis: u64,
    deadline: &crate::distributed::authority_protocol::CanonicalAuthorityTime,
) -> Result<(), ContinuityTransferError> {
    let deadline_millis = canonical_time_unix_millis(deadline)?;
    if observed_unix_millis == 0 || observed_unix_millis > deadline_millis {
        return Err(ContinuityTransferError::Deadline);
    }
    Ok(())
}

fn canonical_time_unix_millis(
    time: &crate::distributed::authority_protocol::CanonicalAuthorityTime,
) -> Result<u64, ContinuityTransferError> {
    if time.unix_seconds <= 0 || time.nanos >= 1_000_000_000 {
        return Err(ContinuityTransferError::Deadline);
    }
    let seconds =
        u64::try_from(time.unix_seconds).map_err(|_| ContinuityTransferError::Deadline)?;
    let nanos_millis = u64::from(time.nanos / 1_000_000);
    seconds
        .checked_mul(1_000)
        .and_then(|millis| millis.checked_add(nanos_millis))
        .ok_or(ContinuityTransferError::Deadline)
}

fn completion_receipt(
    grant: &ContinuityTransferGrantArtifact,
    final_payload_sha256: [u8; 32],
    possession_evidence_sha256: [u8; 32],
) -> VerifiedContinuityTransferReceipt {
    VerifiedContinuityTransferReceipt {
        transfer_id_sha256: sha256_array(grant.transfer_id.as_bytes()),
        source_guardian_sha256: sha256_array(grant.source_guardian_id.as_bytes()),
        target_guardian_sha256: sha256_array(grant.target_guardian_id.as_bytes()),
        route_sha256: sha256_array(grant.route_id.as_bytes()),
        lineage_sha256: sha256_array(&grant.lineage_id),
        signed_manifest_sha256: grant.signed_manifest_sha256,
        signed_catalog_sha256: grant.signed_catalog_sha256,
        chunk_count: grant.chunks.len() as u64,
        total_bytes: grant.total_bytes,
        final_payload_sha256,
        possession_evidence_sha256,
    }
}

fn canonical_identity<T: Serialize>(value: &T) -> Result<Vec<u8>, ContinuityTransferError> {
    Ok(
        sha256_array(&serde_jcs::to_vec(value).map_err(|_| ContinuityTransferError::Encoding)?)
            .to_vec(),
    )
}

fn sha256_array(bytes: &[u8]) -> [u8; 32] {
    <[u8; 32]>::from(Sha256::digest(bytes))
}
