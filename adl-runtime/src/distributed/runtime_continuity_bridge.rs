//! Issue #193 bridge between the private Runtime kernel continuity plane and
//! the distributed migration/recovery authority state machines.
//!
//! The bridge is intentionally data-only. It verifies the signed kernel bundle
//! catalog and derives the distributed snapshot descriptor plus #210 continuity
//! grant from the same opaque source/target handles used by the private
//! continuity client. It does not expose paths, raw handles, or a generic
//! continuity command executor.

use std::collections::BTreeMap;

use ed25519_dalek::VerifyingKey;
use serde::Serialize;
use sha2::{Digest, Sha256};

use adl_runtime_kernel::{SignedBundleCatalog, SourceCheckpointHandle, TargetStageHandle};

use super::{
    authority_protocol::{
        CanonicalAuthorityTime, ContinuityTransferChunk, ContinuityTransferEntry,
        ContinuityTransferGrantArtifact,
    },
    snapshot_catalog::SnapshotDescriptor,
};

pub const KERNEL_CONTINUITY_SNAPSHOT_SCHEMA: &str = "adl.runtime.kernel_continuity.bundle.v1";
const ENTRY_SCHEMA: &str = "adl.runtime.kernel_continuity.bundle_entry.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum RuntimeContinuityBridgeError {
    InvalidIdentity,
    InvalidDigest,
    InvalidCatalog,
    Bounds,
    Encoding,
}

pub type RuntimeContinuityBridgeResult<T> = Result<T, RuntimeContinuityBridgeError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct KernelSnapshotBinding<'a> {
    pub trust_domain: String,
    pub lineage_id: &'a [u8],
    pub source_owner_id: &'a [u8],
    pub source_guardian_id: &'a [u8],
    pub source_epoch: u64,
    pub authority_log_index: u64,
    pub created_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub encryption_context_id: String,
}

#[allow(clippy::too_many_arguments)]
pub fn snapshot_descriptor_from_kernel_catalog(
    catalog: &SignedBundleCatalog,
    source: &SourceCheckpointHandle,
    trusted_catalog_keys: &BTreeMap<(String, u64), VerifyingKey>,
    binding: KernelSnapshotBinding<'_>,
) -> RuntimeContinuityBridgeResult<SnapshotDescriptor> {
    catalog
        .verify(trusted_catalog_keys)
        .map_err(|_| RuntimeContinuityBridgeError::InvalidCatalog)?;
    ensure_catalog_matches_source(catalog, source)?;
    if binding.trust_domain.is_empty()
        || binding.lineage_id.is_empty()
        || binding.source_owner_id.is_empty()
        || binding.source_guardian_id.is_empty()
        || binding.source_epoch == 0
        || binding.authority_log_index == 0
        || binding.authority_log_index != catalog.accepted_prefix
        || binding.created_at_unix_secs == 0
        || binding.expires_at_unix_secs <= binding.created_at_unix_secs
        || binding.encryption_context_id.is_empty()
    {
        return Err(RuntimeContinuityBridgeError::InvalidIdentity);
    }
    let chunk_sha256 = ordered_bundle_chunks(catalog)?
        .into_iter()
        .map(|(_, _, digest)| digest)
        .collect::<Vec<_>>();
    let byte_length = catalog.entries.iter().try_fold(0_u64, |total, entry| {
        total
            .checked_add(entry.bytes)
            .ok_or(RuntimeContinuityBridgeError::Bounds)
    })?;
    Ok(SnapshotDescriptor {
        trust_domain: binding.trust_domain,
        lineage_id: binding.lineage_id.to_vec(),
        source_owner_id: binding.source_owner_id.to_vec(),
        source_guardian_id: binding.source_guardian_id.to_vec(),
        source_epoch: binding.source_epoch,
        authority_log_index: catalog.accepted_prefix,
        snapshot_schema: KERNEL_CONTINUITY_SNAPSHOT_SCHEMA.to_owned(),
        content_sha256: kernel_bundle_digest(catalog)?,
        byte_length,
        chunk_sha256,
        created_at_unix_secs: binding.created_at_unix_secs,
        expires_at_unix_secs: binding.expires_at_unix_secs,
        encryption_context_id: binding.encryption_context_id,
    })
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ContinuityGrantBinding<'a> {
    pub trust_domain: String,
    pub polis_id: String,
    pub source_guardian_id: String,
    pub target_guardian_id: String,
    pub route_id: String,
    pub membership_epoch: u64,
    pub membership_log_index: u64,
    pub source_certificate_generation: u64,
    pub target_certificate_generation: u64,
    pub transfer_id: String,
    pub lineage_id: &'a [u8],
    pub signed_manifest_bytes: Vec<u8>,
    pub signed_catalog_bytes: Vec<u8>,
    pub trusted_key_generation: u64,
    pub inclusive_deadline: CanonicalAuthorityTime,
    pub cleanup_identity: String,
}

#[allow(clippy::too_many_arguments)]
pub fn continuity_transfer_grant_from_kernel_catalog(
    catalog: &SignedBundleCatalog,
    source: &SourceCheckpointHandle,
    target: &TargetStageHandle,
    trusted_catalog_keys: &BTreeMap<(String, u64), VerifyingKey>,
    binding: ContinuityGrantBinding<'_>,
) -> RuntimeContinuityBridgeResult<ContinuityTransferGrantArtifact> {
    catalog
        .verify(trusted_catalog_keys)
        .map_err(|_| RuntimeContinuityBridgeError::InvalidCatalog)?;
    ensure_catalog_matches_source(catalog, source)?;
    if target.catalog_digest() != source.catalog_digest()
        || binding.trust_domain.is_empty()
        || binding.polis_id.is_empty()
        || binding.source_guardian_id.is_empty()
        || binding.target_guardian_id.is_empty()
        || binding.source_guardian_id == binding.target_guardian_id
        || binding.route_id.is_empty()
        || binding.transfer_id.is_empty()
        || binding.lineage_id.is_empty()
        || binding.signed_manifest_bytes.is_empty()
        || binding.signed_catalog_bytes.is_empty()
        || binding.membership_epoch == 0
        || binding.membership_log_index == 0
        || binding.source_certificate_generation == 0
        || binding.target_certificate_generation == 0
        || binding.trusted_key_generation == 0
        || target.root_generation() == 0
    {
        return Err(RuntimeContinuityBridgeError::InvalidIdentity);
    }
    let chunks = ordered_bundle_chunks(catalog)?
        .into_iter()
        .enumerate()
        .map(
            |(index, (absolute_start, length, sha256))| ContinuityTransferChunk {
                index: index as u64,
                absolute_start,
                length,
                sha256,
                predecessor_sha256: if index == 0 {
                    None
                } else {
                    Some(
                        decode_sha256(&catalog.entries[index - 1].sha256)
                            .expect("validated previous digest"),
                    )
                },
            },
        )
        .collect::<Vec<_>>();
    let entries = chunks
        .iter()
        .map(|chunk| ContinuityTransferEntry {
            schema: ENTRY_SCHEMA.to_owned(),
            absolute_start: chunk.absolute_start,
            length: chunk.length,
            sha256: chunk.sha256,
        })
        .collect::<Vec<_>>();
    let total_bytes = chunks.iter().try_fold(0_u64, |total, chunk| {
        total
            .checked_add(chunk.length)
            .ok_or(RuntimeContinuityBridgeError::Bounds)
    })?;
    Ok(ContinuityTransferGrantArtifact {
        trust_domain: binding.trust_domain,
        polis_id: binding.polis_id,
        source_guardian_id: binding.source_guardian_id,
        target_guardian_id: binding.target_guardian_id,
        route_id: binding.route_id,
        membership_epoch: binding.membership_epoch,
        membership_log_index: binding.membership_log_index,
        source_certificate_generation: binding.source_certificate_generation,
        target_certificate_generation: binding.target_certificate_generation,
        source_boot_generation: source.generation(),
        target_boot_generation: target.root_generation(),
        transfer_id: binding.transfer_id,
        lineage_id: binding.lineage_id.to_vec(),
        source_checkpoint_handle_identity: canonical_identity(source)?,
        bundle_handle_identity: canonical_identity(target)?,
        signed_manifest_sha256: sha256_array(&binding.signed_manifest_bytes),
        signed_manifest_bytes: binding.signed_manifest_bytes,
        signed_catalog_sha256: sha256_array(&binding.signed_catalog_bytes),
        signed_catalog_bytes: binding.signed_catalog_bytes,
        trusted_key_generation: binding.trusted_key_generation,
        entries,
        chunks,
        total_bytes,
        inclusive_deadline: binding.inclusive_deadline,
        cleanup_identity: binding.cleanup_identity,
    })
}

fn ensure_catalog_matches_source(
    catalog: &SignedBundleCatalog,
    source: &SourceCheckpointHandle,
) -> RuntimeContinuityBridgeResult<()> {
    if catalog.generation == 0
        || catalog.generation != source.generation()
        || catalog
            .digest()
            .map_err(|_| RuntimeContinuityBridgeError::Encoding)?
            != source.catalog_digest()
        || kernel_bundle_digest(catalog)? != decode_sha256(source.bundle_digest())?
    {
        return Err(RuntimeContinuityBridgeError::InvalidDigest);
    }
    ordered_bundle_chunks(catalog).map(|_| ())
}

fn kernel_bundle_digest(catalog: &SignedBundleCatalog) -> RuntimeContinuityBridgeResult<[u8; 32]> {
    let mut hash = Sha256::new();
    hash.update(
        catalog
            .digest()
            .map_err(|_| RuntimeContinuityBridgeError::Encoding)?
            .as_bytes(),
    );
    for entry in &catalog.entries {
        hash.update(entry.ordinal.to_be_bytes());
        hash.update(entry.sha256.as_bytes());
    }
    Ok(hash.finalize().into())
}

fn ordered_bundle_chunks(
    catalog: &SignedBundleCatalog,
) -> RuntimeContinuityBridgeResult<Vec<(u64, u64, [u8; 32])>> {
    if catalog.entries.is_empty() {
        return Err(RuntimeContinuityBridgeError::InvalidCatalog);
    }
    let mut expected_offset = 0_u64;
    let mut chunks = Vec::with_capacity(catalog.entries.len());
    for (index, entry) in catalog.entries.iter().enumerate() {
        if entry.ordinal != index as u32
            || entry.offset != expected_offset
            || entry.bytes == 0
            || entry.service.is_empty()
            || entry.schema.is_empty()
            || entry.file.is_empty()
        {
            return Err(RuntimeContinuityBridgeError::InvalidCatalog);
        }
        let digest = decode_sha256(&entry.sha256)?;
        chunks.push((entry.offset, entry.bytes, digest));
        expected_offset = expected_offset
            .checked_add(entry.bytes)
            .ok_or(RuntimeContinuityBridgeError::Bounds)?;
    }
    Ok(chunks)
}

fn canonical_identity<T: Serialize>(value: &T) -> RuntimeContinuityBridgeResult<Vec<u8>> {
    Ok(
        sha256_array(
            &serde_jcs::to_vec(value).map_err(|_| RuntimeContinuityBridgeError::Encoding)?,
        )
        .to_vec(),
    )
}

fn decode_sha256(value: &str) -> RuntimeContinuityBridgeResult<[u8; 32]> {
    let bytes = hex::decode(value).map_err(|_| RuntimeContinuityBridgeError::InvalidDigest)?;
    bytes
        .try_into()
        .map_err(|_| RuntimeContinuityBridgeError::InvalidDigest)
}

fn sha256_array(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}
