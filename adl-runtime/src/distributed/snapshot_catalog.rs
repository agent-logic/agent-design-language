use std::{
    fmt, fs,
    path::{Component, Path, PathBuf},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use prost::Message;
use redb::{Database, Durability, ReadableTable, ReadableTableMetadata, TableDefinition};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[cfg(not(test))]
use super::authority_store_adapters::{AuthorityBoundCertificateStore, AuthorityBoundFencingStore};
#[cfg(test)]
use super::certificates::{DistributedCertificateStore, AUTHORITY_BOUND_CERTIFICATE_ACCESS};
#[cfg(test)]
use super::fencing::{FencingStore, AUTHORITY_BOUND_FENCING_ACCESS};
use super::{
    certificates::{AuthorityCertificate, CertificatePurpose},
    fencing::ActiveLeaseCheck,
};
#[cfg(test)]
use std::sync::Arc;

mod fencing_authority_seal {
    pub trait Sealed {}
}

pub trait SnapshotFencingAuthority: fencing_authority_seal::Sealed {
    fn authorize_snapshot_lease(&self, check: ActiveLeaseCheck<'_>) -> SnapshotResult<()>;
}

#[cfg(not(test))]
impl fencing_authority_seal::Sealed for AuthorityBoundFencingStore {}
#[cfg(not(test))]
impl SnapshotFencingAuthority for AuthorityBoundFencingStore {
    fn authorize_snapshot_lease(&self, check: ActiveLeaseCheck<'_>) -> SnapshotResult<()> {
        self.authorize_active_lease(check)
            .map_err(|_| SnapshotError::FencedAuthority)
    }
}

#[cfg(test)]
impl fencing_authority_seal::Sealed for FencingStore {}
#[cfg(test)]
impl SnapshotFencingAuthority for FencingStore {
    fn authorize_snapshot_lease(&self, check: ActiveLeaseCheck<'_>) -> SnapshotResult<()> {
        self.authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)
            .map_err(|_| SnapshotError::FencedAuthority)
    }
}

pub const SNAPSHOT_CATALOG_SCHEMA: &str = "adl.distributed.snapshot_catalog_entry.v1";
pub const TRANSFER_MANIFEST_SCHEMA: &str = "adl.distributed.snapshot_transfer_manifest.v1";
const CATALOG_SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-SNAPSHOT-CATALOG-V1\0";
const MANIFEST_SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-SNAPSHOT-MANIFEST-V1\0";
const SIGNATURE_LEN: usize = 64;
const MAX_TEXT_LEN: usize = 128;
const ABSOLUTE_MAX_CHUNKS: usize = 65_536;
const ABSOLUTE_MAX_CONTENT_BYTES: u64 = 1 << 40;
const ABSOLUTE_MAX_LIFETIME_SECS: u64 = 86_400;
const ABSOLUTE_MAX_ENCODED_BYTES: usize = 16 * 1024 * 1024;
const ABSOLUTE_MAX_REPLAY_RECORDS: u64 = 1_000_000;

const ACCEPTED_TRANSFERS: TableDefinition<&[u8], &[u8]> =
    TableDefinition::new("distributed_snapshot_accepted_transfers_v1");

#[derive(Clone, PartialEq, Message)]
struct SnapshotDescriptorWire {
    #[prost(string, tag = "1")]
    trust_domain: String,
    #[prost(bytes = "vec", tag = "2")]
    lineage_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    source_owner_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    source_guardian_id: Vec<u8>,
    #[prost(uint64, tag = "5")]
    source_epoch: u64,
    #[prost(uint64, tag = "6")]
    authority_log_index: u64,
    #[prost(string, tag = "7")]
    snapshot_schema: String,
    #[prost(bytes = "vec", tag = "8")]
    content_sha256: Vec<u8>,
    #[prost(uint64, tag = "9")]
    byte_length: u64,
    #[prost(bytes = "vec", repeated, tag = "10")]
    chunk_sha256: Vec<Vec<u8>>,
    #[prost(uint64, tag = "11")]
    created_at_unix_secs: u64,
    #[prost(uint64, tag = "12")]
    expires_at_unix_secs: u64,
    #[prost(string, tag = "13")]
    encryption_context_id: String,
}

#[derive(Clone, PartialEq, Message)]
struct SnapshotCatalogEntryBodyWire {
    #[prost(string, tag = "1")]
    schema: String,
    #[prost(string, tag = "2")]
    signer_id: String,
    #[prost(uint64, tag = "3")]
    certificate_generation: u64,
    #[prost(uint64, tag = "4")]
    sequence: u64,
    #[prost(message, optional, tag = "5")]
    snapshot: Option<SnapshotDescriptorWire>,
}

#[derive(Clone, PartialEq, Message)]
struct SignedSnapshotCatalogEntryWire {
    #[prost(message, optional, tag = "1")]
    body: Option<SnapshotCatalogEntryBodyWire>,
    #[prost(bytes = "vec", tag = "2")]
    authority_certificate: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    signature: Vec<u8>,
}

#[derive(Clone, PartialEq, Message)]
struct TransferManifestBodyWire {
    #[prost(string, tag = "1")]
    schema: String,
    #[prost(bytes = "vec", tag = "2")]
    transfer_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    intended_target_id: Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    catalog_entry_sha256: Vec<u8>,
    #[prost(string, tag = "5")]
    signer_id: String,
    #[prost(uint64, tag = "6")]
    certificate_generation: u64,
    #[prost(message, optional, tag = "7")]
    snapshot: Option<SnapshotDescriptorWire>,
}

#[derive(Clone, PartialEq, Message)]
struct SignedTransferManifestWire {
    #[prost(message, optional, tag = "1")]
    body: Option<TransferManifestBodyWire>,
    #[prost(bytes = "vec", tag = "2")]
    authority_certificate: Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    signature: Vec<u8>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotDescriptor {
    pub trust_domain: String,
    pub lineage_id: Vec<u8>,
    pub source_owner_id: Vec<u8>,
    pub source_guardian_id: Vec<u8>,
    pub source_epoch: u64,
    pub authority_log_index: u64,
    pub snapshot_schema: String,
    pub content_sha256: [u8; 32],
    pub byte_length: u64,
    pub chunk_sha256: Vec<[u8; 32]>,
    pub created_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub encryption_context_id: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SnapshotCatalogEntryBody {
    pub schema: String,
    pub signer_id: String,
    pub certificate_generation: u64,
    pub sequence: u64,
    pub snapshot: SnapshotDescriptor,
}

impl SnapshotCatalogEntryBody {
    pub fn new(
        signer_id: impl Into<String>,
        certificate_generation: u64,
        sequence: u64,
        snapshot: SnapshotDescriptor,
        policy: &SnapshotCatalogPolicy,
    ) -> SnapshotResult<Self> {
        let body = Self {
            schema: SNAPSHOT_CATALOG_SCHEMA.to_owned(),
            signer_id: signer_id.into(),
            certificate_generation,
            sequence,
            snapshot,
        };
        policy.validate_catalog_body(&body)?;
        Ok(body)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedSnapshotCatalogEntry {
    pub body: SnapshotCatalogEntryBody,
    pub authority_certificate: AuthorityCertificate,
    pub signature: Vec<u8>,
}

impl SignedSnapshotCatalogEntry {
    pub fn issue(
        body: SnapshotCatalogEntryBody,
        authority_certificate: AuthorityCertificate,
        signing_key: &SigningKey,
        policy: &SnapshotCatalogPolicy,
    ) -> SnapshotResult<Self> {
        policy.validate_catalog_body(&body)?;
        validate_certificate_binding(
            &body.signer_id,
            body.certificate_generation,
            &body.snapshot,
            &authority_certificate,
            signing_key.verifying_key(),
        )?;
        let certificate_id = authority_certificate
            .certificate_id()
            .map_err(|_| SnapshotError::CertificateAuthorization)?;
        let signature = signing_key
            .sign(&catalog_signing_bytes(&body, &certificate_id)?)
            .to_bytes()
            .to_vec();
        let entry = Self {
            body,
            authority_certificate,
            signature,
        };
        if catalog_bytes(&entry)?.len() > policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        Ok(entry)
    }

    pub fn encode(&self, policy: &SnapshotCatalogPolicy) -> SnapshotResult<Vec<u8>> {
        let bytes = catalog_bytes(self)?;
        if bytes.len() > policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TransferManifestBody {
    pub schema: String,
    pub transfer_id: Vec<u8>,
    pub intended_target_id: Vec<u8>,
    pub catalog_entry_sha256: [u8; 32],
    pub signer_id: String,
    pub certificate_generation: u64,
    pub snapshot: SnapshotDescriptor,
}

impl TransferManifestBody {
    pub fn new(
        transfer_id: impl Into<Vec<u8>>,
        intended_target_id: impl Into<Vec<u8>>,
        catalog_entry: &SignedSnapshotCatalogEntry,
        policy: &SnapshotCatalogPolicy,
    ) -> SnapshotResult<Self> {
        let body = Self {
            schema: TRANSFER_MANIFEST_SCHEMA.to_owned(),
            transfer_id: transfer_id.into(),
            intended_target_id: intended_target_id.into(),
            catalog_entry_sha256: sha256(&catalog_bytes(catalog_entry)?),
            signer_id: catalog_entry.body.signer_id.clone(),
            certificate_generation: catalog_entry.body.certificate_generation,
            snapshot: catalog_entry.body.snapshot.clone(),
        };
        policy.validate_manifest_body(&body)?;
        Ok(body)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SignedTransferManifest {
    pub body: TransferManifestBody,
    pub authority_certificate: AuthorityCertificate,
    pub signature: Vec<u8>,
}

impl SignedTransferManifest {
    pub fn issue(
        body: TransferManifestBody,
        authority_certificate: AuthorityCertificate,
        signing_key: &SigningKey,
        policy: &SnapshotCatalogPolicy,
    ) -> SnapshotResult<Self> {
        policy.validate_manifest_body(&body)?;
        validate_certificate_binding(
            &body.signer_id,
            body.certificate_generation,
            &body.snapshot,
            &authority_certificate,
            signing_key.verifying_key(),
        )?;
        let certificate_id = authority_certificate
            .certificate_id()
            .map_err(|_| SnapshotError::CertificateAuthorization)?;
        let signature = signing_key
            .sign(&manifest_signing_bytes(&body, &certificate_id)?)
            .to_bytes()
            .to_vec();
        let manifest = Self {
            body,
            authority_certificate,
            signature,
        };
        if manifest_bytes(&manifest)?.len() > policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        Ok(manifest)
    }

    pub fn encode(&self, policy: &SnapshotCatalogPolicy) -> SnapshotResult<Vec<u8>> {
        let bytes = manifest_bytes(self)?;
        if bytes.len() > policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSnapshotCatalogEntry {
    pub catalog_entry_sha256: [u8; 32],
    pub certificate_id: String,
    pub sequence: u64,
    pub verification_deadline_unix_secs: u64,
    pub snapshot: SnapshotDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedSnapshotTransfer {
    pub transfer_id: Vec<u8>,
    pub catalog_entry_sha256: [u8; 32],
    pub certificate_id: String,
    pub verification_deadline_unix_secs: u64,
    pub snapshot: SnapshotDescriptor,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RedactedSnapshotProjection {
    pub schema: String,
    pub snapshot_schema: String,
    pub byte_length: u64,
    pub chunk_count: usize,
    pub created_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

impl VerifiedSnapshotCatalogEntry {
    /// This projection deliberately omits lineage, owners, target, certificate,
    /// encryption context, content digest, and chunk digests.
    pub fn redacted(&self) -> RedactedSnapshotProjection {
        redacted(&self.snapshot)
    }
}

impl VerifiedSnapshotTransfer {
    pub fn redacted(&self) -> RedactedSnapshotProjection {
        redacted(&self.snapshot)
    }
}

fn redacted(snapshot: &SnapshotDescriptor) -> RedactedSnapshotProjection {
    RedactedSnapshotProjection {
        schema: SNAPSHOT_CATALOG_SCHEMA.to_owned(),
        snapshot_schema: snapshot.snapshot_schema.clone(),
        byte_length: snapshot.byte_length,
        chunk_count: snapshot.chunk_sha256.len(),
        created_at_unix_secs: snapshot.created_at_unix_secs,
        expires_at_unix_secs: snapshot.expires_at_unix_secs,
    }
}

#[derive(Clone, Debug)]
pub struct SnapshotCatalogPolicy {
    trust_domain: String,
    max_chunks: usize,
    max_content_bytes: u64,
    max_lifetime_secs: u64,
    max_future_skew_secs: u64,
    max_encoded_bytes: usize,
    max_replay_records: u64,
}

impl SnapshotCatalogPolicy {
    pub fn new(trust_domain: impl Into<String>) -> SnapshotResult<Self> {
        let policy = Self {
            trust_domain: trust_domain.into(),
            max_chunks: 4096,
            max_content_bytes: 64 * 1024 * 1024 * 1024,
            max_lifetime_secs: 3600,
            max_future_skew_secs: 5,
            max_encoded_bytes: 4 * 1024 * 1024,
            max_replay_records: 65_536,
        };
        validate_text(&policy.trust_domain)?;
        Ok(policy)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn with_bounds(
        mut self,
        max_chunks: usize,
        max_content_bytes: u64,
        max_lifetime_secs: u64,
        max_future_skew_secs: u64,
        max_encoded_bytes: usize,
        max_replay_records: u64,
    ) -> SnapshotResult<Self> {
        if max_chunks == 0
            || max_chunks > ABSOLUTE_MAX_CHUNKS
            || max_content_bytes == 0
            || max_content_bytes > ABSOLUTE_MAX_CONTENT_BYTES
            || max_lifetime_secs == 0
            || max_lifetime_secs > ABSOLUTE_MAX_LIFETIME_SECS
            || max_future_skew_secs > 300
            || max_encoded_bytes == 0
            || max_encoded_bytes > ABSOLUTE_MAX_ENCODED_BYTES
            || max_replay_records == 0
            || max_replay_records > ABSOLUTE_MAX_REPLAY_RECORDS
        {
            return Err(SnapshotError::InvalidPolicy);
        }
        self.max_chunks = max_chunks;
        self.max_content_bytes = max_content_bytes;
        self.max_lifetime_secs = max_lifetime_secs;
        self.max_future_skew_secs = max_future_skew_secs;
        self.max_encoded_bytes = max_encoded_bytes;
        self.max_replay_records = max_replay_records;
        Ok(self)
    }

    fn validate_snapshot(&self, snapshot: &SnapshotDescriptor) -> SnapshotResult<()> {
        validate_text(&snapshot.trust_domain)?;
        validate_text(&snapshot.snapshot_schema)?;
        validate_text(&snapshot.encryption_context_id)?;
        if snapshot.trust_domain != self.trust_domain
            || snapshot.lineage_id.is_empty()
            || snapshot.lineage_id.len() > MAX_TEXT_LEN
            || snapshot.source_owner_id.is_empty()
            || snapshot.source_owner_id.len() > MAX_TEXT_LEN
            || snapshot.source_guardian_id.is_empty()
            || snapshot.source_guardian_id.len() > MAX_TEXT_LEN
            || snapshot.source_epoch == 0
            || snapshot.authority_log_index == 0
            || snapshot.byte_length == 0
            || snapshot.byte_length > self.max_content_bytes
            || snapshot.chunk_sha256.is_empty()
            || snapshot.chunk_sha256.len() > self.max_chunks
        {
            return Err(SnapshotError::InvalidSnapshot);
        }
        validate_identifier(&snapshot.lineage_id)?;
        validate_identifier(&snapshot.source_owner_id)?;
        validate_identifier(&snapshot.source_guardian_id)?;
        let lifetime = snapshot
            .expires_at_unix_secs
            .checked_sub(snapshot.created_at_unix_secs)
            .ok_or(SnapshotError::InvalidLifetime)?;
        if lifetime == 0 || lifetime > self.max_lifetime_secs {
            return Err(SnapshotError::InvalidLifetime);
        }
        Ok(())
    }

    fn validate_catalog_body(&self, body: &SnapshotCatalogEntryBody) -> SnapshotResult<()> {
        if body.schema != SNAPSHOT_CATALOG_SCHEMA
            || body.certificate_generation == 0
            || body.sequence == 0
        {
            return Err(SnapshotError::InvalidSnapshot);
        }
        validate_text(&body.signer_id)?;
        self.validate_snapshot(&body.snapshot)
    }

    fn validate_manifest_body(&self, body: &TransferManifestBody) -> SnapshotResult<()> {
        if body.schema != TRANSFER_MANIFEST_SCHEMA
            || body.transfer_id.is_empty()
            || body.transfer_id.len() > MAX_TEXT_LEN
            || body.intended_target_id.is_empty()
            || body.intended_target_id.len() > MAX_TEXT_LEN
            || body.certificate_generation == 0
        {
            return Err(SnapshotError::InvalidManifest);
        }
        validate_identifier(&body.transfer_id)?;
        validate_identifier(&body.intended_target_id)?;
        validate_text(&body.signer_id)?;
        self.validate_snapshot(&body.snapshot)
    }

    fn validate_time(
        &self,
        snapshot: &SnapshotDescriptor,
        now_unix_secs: u64,
    ) -> SnapshotResult<()> {
        let future_limit = now_unix_secs
            .checked_add(self.max_future_skew_secs)
            .ok_or(SnapshotError::InvalidLifetime)?;
        if snapshot.created_at_unix_secs > future_limit {
            return Err(SnapshotError::NotYetValid);
        }
        if now_unix_secs >= snapshot.expires_at_unix_secs {
            return Err(SnapshotError::Expired);
        }
        Ok(())
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct ReplayRecord {
    manifest_sha256: [u8; 32],
    retain_until_unix_secs: u64,
}

pub struct SnapshotCatalogVerifier {
    certificate_store: SnapshotCertificateAuthority,
    policy: SnapshotCatalogPolicy,
    replay_database: Database,
}

enum SnapshotCertificateAuthority {
    #[cfg(not(test))]
    Bound(AuthorityBoundCertificateStore),
    #[cfg(test)]
    TestRaw(Arc<DistributedCertificateStore>),
}

impl SnapshotCertificateAuthority {
    fn authorize(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> Result<super::certificates::VerifiedCertificate, ()> {
        match self {
            #[cfg(not(test))]
            Self::Bound(store) => store
                .authorize(holder_id, purpose, generation, now_unix_secs)
                .map_err(|_| ()),
            #[cfg(test)]
            Self::TestRaw(store) => store
                .authorize(
                    &AUTHORITY_BOUND_CERTIFICATE_ACCESS,
                    holder_id,
                    purpose,
                    generation,
                    now_unix_secs,
                )
                .map_err(|_| ()),
        }
    }
}

impl SnapshotCatalogVerifier {
    #[cfg(not(test))]
    pub fn open(
        certificate_store: AuthorityBoundCertificateStore,
        policy: SnapshotCatalogPolicy,
        replay_database_path: impl AsRef<Path>,
    ) -> SnapshotResult<Self> {
        let replay_database_path = validate_database_path(replay_database_path.as_ref())?;
        let replay_database =
            Database::create(replay_database_path).map_err(|_| SnapshotError::StateUnavailable)?;
        let mut write = replay_database
            .begin_write()
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .open_table(ACCEPTED_TRANSFERS)
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .commit()
            .map_err(|_| SnapshotError::StateUnavailable)?;
        Ok(Self {
            certificate_store: SnapshotCertificateAuthority::Bound(certificate_store),
            policy,
            replay_database,
        })
    }

    #[cfg(test)]
    pub fn open_for_test(
        certificate_store: Arc<DistributedCertificateStore>,
        policy: SnapshotCatalogPolicy,
        replay_database_path: impl AsRef<Path>,
    ) -> SnapshotResult<Self> {
        let replay_database_path = validate_database_path(replay_database_path.as_ref())?;
        let replay_database =
            Database::create(replay_database_path).map_err(|_| SnapshotError::StateUnavailable)?;
        let mut write = replay_database
            .begin_write()
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .open_table(ACCEPTED_TRANSFERS)
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .commit()
            .map_err(|_| SnapshotError::StateUnavailable)?;
        Ok(Self {
            certificate_store: SnapshotCertificateAuthority::TestRaw(certificate_store),
            policy,
            replay_database,
        })
    }

    pub fn decode_catalog_and_verify<F: SnapshotFencingAuthority>(
        &self,
        bytes: &[u8],
        fencing: &F,
        active_lease: ActiveLeaseCheck<'_>,
    ) -> SnapshotResult<VerifiedSnapshotCatalogEntry> {
        if bytes.is_empty() || bytes.len() > self.policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        let entry = decode_catalog(bytes)?;
        if catalog_bytes(&entry)? != bytes {
            return Err(SnapshotError::NonCanonical);
        }
        self.verify_catalog(&entry, fencing, active_lease)
    }

    pub fn verify_catalog<F: SnapshotFencingAuthority>(
        &self,
        entry: &SignedSnapshotCatalogEntry,
        fencing: &F,
        active_lease: ActiveLeaseCheck<'_>,
    ) -> SnapshotResult<VerifiedSnapshotCatalogEntry> {
        if catalog_bytes(entry)?.len() > self.policy.max_encoded_bytes {
            return Err(SnapshotError::Oversized);
        }
        self.policy.validate_catalog_body(&entry.body)?;
        let now_unix_secs = active_time(&active_lease)?;
        self.policy
            .validate_time(&entry.body.snapshot, now_unix_secs)?;
        verify_live_authority(&entry.body.snapshot, fencing, active_lease)?;
        let (certificate_id, deadline) = self.verify_signature(
            &entry.body.signer_id,
            entry.body.certificate_generation,
            &entry.body.snapshot,
            &entry.authority_certificate,
            &entry.signature,
            &catalog_signing_bytes,
            &entry.body,
            now_unix_secs,
        )?;
        Ok(VerifiedSnapshotCatalogEntry {
            catalog_entry_sha256: sha256(&catalog_bytes(entry)?),
            certificate_id,
            sequence: entry.body.sequence,
            verification_deadline_unix_secs: entry.body.snapshot.expires_at_unix_secs.min(deadline),
            snapshot: entry.body.snapshot.clone(),
        })
    }

    pub fn decode_transfer_and_verify<F: SnapshotFencingAuthority>(
        &self,
        encoded_manifest: &[u8],
        encoded_catalog: &[u8],
        chunks: &[Vec<u8>],
        expected_target_id: &[u8],
        fencing: &F,
        active_lease: ActiveLeaseCheck<'_>,
    ) -> SnapshotResult<VerifiedSnapshotTransfer> {
        if encoded_manifest.is_empty()
            || encoded_catalog.is_empty()
            || encoded_manifest.len() > self.policy.max_encoded_bytes
            || encoded_catalog.len() > self.policy.max_encoded_bytes
        {
            return Err(SnapshotError::Oversized);
        }
        let manifest = decode_manifest(encoded_manifest)?;
        let catalog = decode_catalog(encoded_catalog)?;
        if manifest_bytes(&manifest)? != encoded_manifest
            || catalog_bytes(&catalog)? != encoded_catalog
        {
            return Err(SnapshotError::NonCanonical);
        }
        self.verify_transfer(
            &manifest,
            &catalog,
            chunks,
            expected_target_id,
            fencing,
            active_lease,
        )
    }

    pub fn verify_transfer<F: SnapshotFencingAuthority>(
        &self,
        manifest: &SignedTransferManifest,
        catalog: &SignedSnapshotCatalogEntry,
        chunks: &[Vec<u8>],
        expected_target_id: &[u8],
        fencing: &F,
        active_lease: ActiveLeaseCheck<'_>,
    ) -> SnapshotResult<VerifiedSnapshotTransfer> {
        if manifest_bytes(manifest)?.len() > self.policy.max_encoded_bytes
            || catalog_bytes(catalog)?.len() > self.policy.max_encoded_bytes
        {
            return Err(SnapshotError::Oversized);
        }
        self.policy.validate_manifest_body(&manifest.body)?;
        self.policy.validate_catalog_body(&catalog.body)?;
        let now_unix_secs = active_time(&active_lease)?;
        self.policy
            .validate_time(&manifest.body.snapshot, now_unix_secs)?;
        if manifest.body.intended_target_id != expected_target_id {
            return Err(SnapshotError::WrongTarget);
        }
        let encoded_catalog = catalog_bytes(catalog)?;
        if manifest.body.catalog_entry_sha256 != sha256(&encoded_catalog)
            || manifest.body.snapshot != catalog.body.snapshot
            || manifest.body.signer_id != catalog.body.signer_id
            || manifest.body.certificate_generation != catalog.body.certificate_generation
        {
            return Err(SnapshotError::CatalogMismatch);
        }
        verify_live_authority(&manifest.body.snapshot, fencing, active_lease)?;
        let (catalog_certificate_id, _) = self.verify_signature(
            &catalog.body.signer_id,
            catalog.body.certificate_generation,
            &catalog.body.snapshot,
            &catalog.authority_certificate,
            &catalog.signature,
            &catalog_signing_bytes,
            &catalog.body,
            now_unix_secs,
        )?;
        let (manifest_certificate_id, deadline) = self.verify_signature(
            &manifest.body.signer_id,
            manifest.body.certificate_generation,
            &manifest.body.snapshot,
            &manifest.authority_certificate,
            &manifest.signature,
            &manifest_signing_bytes,
            &manifest.body,
            now_unix_secs,
        )?;
        if catalog_certificate_id != manifest_certificate_id {
            return Err(SnapshotError::CatalogMismatch);
        }
        verify_content(&manifest.body.snapshot, chunks)?;
        let manifest_sha256 = sha256(&manifest_bytes(manifest)?);
        self.record_transfer(
            &manifest.body.transfer_id,
            manifest_sha256,
            manifest.body.snapshot.expires_at_unix_secs,
            now_unix_secs,
        )?;
        Ok(VerifiedSnapshotTransfer {
            transfer_id: manifest.body.transfer_id.clone(),
            catalog_entry_sha256: manifest.body.catalog_entry_sha256,
            certificate_id: manifest_certificate_id,
            verification_deadline_unix_secs: manifest
                .body
                .snapshot
                .expires_at_unix_secs
                .min(deadline),
            snapshot: manifest.body.snapshot.clone(),
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn verify_signature<T>(
        &self,
        signer_id: &str,
        generation: u64,
        snapshot: &SnapshotDescriptor,
        certificate: &AuthorityCertificate,
        signature_bytes: &[u8],
        signing_bytes: &dyn Fn(&T, &str) -> SnapshotResult<Vec<u8>>,
        body: &T,
        now_unix_secs: u64,
    ) -> SnapshotResult<(String, u64)> {
        validate_certificate_body(signer_id, generation, snapshot, certificate)?;
        let certificate_id = certificate
            .certificate_id()
            .map_err(|_| SnapshotError::CertificateAuthorization)?;
        let authorized = self
            .certificate_store
            .authorize(
                signer_id,
                CertificatePurpose::SnapshotSigning,
                generation,
                now_unix_secs,
            )
            .map_err(|_| SnapshotError::CertificateAuthorization)?;
        if authorized.certificate_id != certificate_id
            || authorized.holder_id != signer_id
            || authorized.purpose != CertificatePurpose::SnapshotSigning
            || authorized.generation != generation
        {
            return Err(SnapshotError::CertificateAuthorization);
        }
        if signature_bytes.len() != SIGNATURE_LEN {
            return Err(SnapshotError::MalformedSignature);
        }
        let signature = Signature::from_slice(signature_bytes)
            .map_err(|_| SnapshotError::MalformedSignature)?;
        let key = VerifyingKey::from_bytes(&certificate.body.subject_public_key)
            .map_err(|_| SnapshotError::WrongSigner)?;
        if key
            .verify_strict(&signing_bytes(body, &certificate_id)?, &signature)
            .is_err()
        {
            return Err(SnapshotError::WrongSigner);
        }
        Ok((certificate_id, authorized.authorization_deadline_unix_secs))
    }

    fn record_transfer(
        &self,
        transfer_id: &[u8],
        manifest_sha256: [u8; 32],
        retain_until_unix_secs: u64,
        now_unix_secs: u64,
    ) -> SnapshotResult<()> {
        let mut write = self
            .replay_database
            .begin_write()
            .map_err(|_| SnapshotError::StateUnavailable)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(|_| SnapshotError::StateUnavailable)?;
        {
            let mut table = write
                .open_table(ACCEPTED_TRANSFERS)
                .map_err(|_| SnapshotError::StateUnavailable)?;
            let mut expired = Vec::new();
            for entry in table.iter().map_err(|_| SnapshotError::StateUnavailable)? {
                let (key, value) = entry.map_err(|_| SnapshotError::StateUnavailable)?;
                let state: ReplayRecord = serde_json::from_slice(value.value())
                    .map_err(|_| SnapshotError::StateUnavailable)?;
                if now_unix_secs >= state.retain_until_unix_secs {
                    expired.push(key.value().to_vec());
                }
            }
            for key in expired {
                table
                    .remove(key.as_slice())
                    .map_err(|_| SnapshotError::StateUnavailable)?;
            }
            if let Some(existing) = table
                .get(transfer_id)
                .map_err(|_| SnapshotError::StateUnavailable)?
            {
                let existing: ReplayRecord = serde_json::from_slice(existing.value())
                    .map_err(|_| SnapshotError::StateUnavailable)?;
                return if existing.manifest_sha256 == manifest_sha256 {
                    Err(SnapshotError::Replay)
                } else {
                    Err(SnapshotError::ReplayMismatch)
                };
            }
            if table.len().map_err(|_| SnapshotError::StateUnavailable)?
                >= self.policy.max_replay_records
            {
                return Err(SnapshotError::CapacityExceeded);
            }
            let encoded = serde_json::to_vec(&ReplayRecord {
                manifest_sha256,
                retain_until_unix_secs,
            })
            .map_err(|_| SnapshotError::Encoding)?;
            table
                .insert(transfer_id, encoded.as_slice())
                .map_err(|_| SnapshotError::StateUnavailable)?;
        }
        write.commit().map_err(|_| SnapshotError::StateUnavailable)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SnapshotError {
    InvalidPolicy,
    InvalidSnapshot,
    InvalidManifest,
    InvalidText,
    InvalidLifetime,
    NotYetValid,
    Expired,
    Oversized,
    CapacityExceeded,
    Malformed,
    NonCanonical,
    MalformedSignature,
    WrongSigner,
    CertificateAuthorization,
    FencedAuthority,
    AuthorityMismatch,
    WrongTarget,
    CatalogMismatch,
    IncompleteTransfer,
    CorruptChunk,
    ContentDigestMismatch,
    Replay,
    ReplayMismatch,
    StateUnavailable,
    Encoding,
}

impl SnapshotError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::InvalidPolicy => "invalid_policy",
            Self::InvalidSnapshot => "invalid_snapshot",
            Self::InvalidManifest => "invalid_manifest",
            Self::InvalidText => "invalid_text",
            Self::InvalidLifetime => "invalid_lifetime",
            Self::NotYetValid => "not_yet_valid",
            Self::Expired => "expired",
            Self::Oversized => "oversized",
            Self::CapacityExceeded => "capacity_exceeded",
            Self::Malformed => "malformed",
            Self::NonCanonical => "non_canonical",
            Self::MalformedSignature => "malformed_signature",
            Self::WrongSigner => "wrong_signer",
            Self::CertificateAuthorization => "certificate_authorization_failed",
            Self::FencedAuthority => "fenced_authority",
            Self::AuthorityMismatch => "authority_mismatch",
            Self::WrongTarget => "wrong_target",
            Self::CatalogMismatch => "catalog_mismatch",
            Self::IncompleteTransfer => "incomplete_transfer",
            Self::CorruptChunk => "corrupt_chunk",
            Self::ContentDigestMismatch => "content_digest_mismatch",
            Self::Replay => "replay",
            Self::ReplayMismatch => "replay_mismatch",
            Self::StateUnavailable => "state_unavailable",
            Self::Encoding => "encoding_failed",
        }
    }
}

impl fmt::Display for SnapshotError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.code())
    }
}

impl std::error::Error for SnapshotError {}
pub type SnapshotResult<T> = Result<T, SnapshotError>;

fn active_time(check: &ActiveLeaseCheck<'_>) -> SnapshotResult<u64> {
    u64::try_from(check.now_unix_seconds).map_err(|_| SnapshotError::InvalidLifetime)
}

fn verify_live_authority<F: SnapshotFencingAuthority>(
    snapshot: &SnapshotDescriptor,
    fencing: &F,
    check: ActiveLeaseCheck<'_>,
) -> SnapshotResult<()> {
    let lease = check.lease;
    if check
        .membership
        .is_none_or(|membership| snapshot.trust_domain.as_bytes() != membership.trust_domain_id)
        || snapshot.lineage_id != lease.lineage_id
        || snapshot.source_owner_id != lease.holder_node_id
        || snapshot.source_guardian_id != lease.holder_guardian_id
        || snapshot.source_epoch != lease.epoch
        || snapshot.authority_log_index != check.applied_log_index
        || snapshot.authority_log_index != lease.committed_log_index
    {
        return Err(SnapshotError::AuthorityMismatch);
    }
    fencing.authorize_snapshot_lease(check)
}

fn verify_content(snapshot: &SnapshotDescriptor, chunks: &[Vec<u8>]) -> SnapshotResult<()> {
    if chunks.len() != snapshot.chunk_sha256.len() {
        return Err(SnapshotError::IncompleteTransfer);
    }
    let mut total = 0_u64;
    let mut whole = Sha256::new();
    for (chunk, expected) in chunks.iter().zip(&snapshot.chunk_sha256) {
        let chunk_len = u64::try_from(chunk.len()).map_err(|_| SnapshotError::Oversized)?;
        total = total
            .checked_add(chunk_len)
            .ok_or(SnapshotError::Oversized)?;
        // Reject attacker-sized buffers before spending work hashing them. The
        // signed total is policy-bounded, so it is also the hard per-chunk cap.
        if total > snapshot.byte_length {
            return Err(SnapshotError::ContentDigestMismatch);
        }
        if sha256(chunk) != *expected {
            return Err(SnapshotError::CorruptChunk);
        }
        whole.update(chunk);
    }
    if total != snapshot.byte_length
        || <[u8; 32]>::from(whole.finalize()) != snapshot.content_sha256
    {
        return Err(SnapshotError::ContentDigestMismatch);
    }
    Ok(())
}

fn validate_certificate_binding(
    signer_id: &str,
    generation: u64,
    snapshot: &SnapshotDescriptor,
    certificate: &AuthorityCertificate,
    signing_key: VerifyingKey,
) -> SnapshotResult<()> {
    validate_certificate_body(signer_id, generation, snapshot, certificate)?;
    if certificate.body.subject_public_key != signing_key.to_bytes() {
        return Err(SnapshotError::WrongSigner);
    }
    Ok(())
}

fn validate_certificate_body(
    signer_id: &str,
    generation: u64,
    snapshot: &SnapshotDescriptor,
    certificate: &AuthorityCertificate,
) -> SnapshotResult<()> {
    let body = &certificate.body;
    if body.purpose != CertificatePurpose::SnapshotSigning
        || body.trust_domain != snapshot.trust_domain
        || body.holder_id != signer_id
        || body.holder_id.as_bytes() != snapshot.source_owner_id
        || body.generation != generation
        || snapshot.created_at_unix_secs < body.issued_at_unix_secs
        || snapshot.expires_at_unix_secs > body.expires_at_unix_secs
    {
        return Err(SnapshotError::CertificateAuthorization);
    }
    Ok(())
}

fn catalog_signing_bytes(
    body: &SnapshotCatalogEntryBody,
    certificate_id: &str,
) -> SnapshotResult<Vec<u8>> {
    signing_bytes(
        CATALOG_SIGNING_DOMAIN,
        &SnapshotCatalogEntryBodyWire::from(body).encode_to_vec(),
        certificate_id,
    )
}

fn manifest_signing_bytes(
    body: &TransferManifestBody,
    certificate_id: &str,
) -> SnapshotResult<Vec<u8>> {
    signing_bytes(
        MANIFEST_SIGNING_DOMAIN,
        &TransferManifestBodyWire::from(body).encode_to_vec(),
        certificate_id,
    )
}

fn signing_bytes(domain: &[u8], body: &[u8], certificate_id: &str) -> SnapshotResult<Vec<u8>> {
    let certificate_id = certificate_id.as_bytes();
    let id_len = u32::try_from(certificate_id.len()).map_err(|_| SnapshotError::Encoding)?;
    let body_len = u32::try_from(body.len()).map_err(|_| SnapshotError::Encoding)?;
    let mut bytes = Vec::with_capacity(domain.len() + 8 + certificate_id.len() + body.len());
    bytes.extend_from_slice(domain);
    bytes.extend_from_slice(&id_len.to_be_bytes());
    bytes.extend_from_slice(certificate_id);
    bytes.extend_from_slice(&body_len.to_be_bytes());
    bytes.extend_from_slice(body);
    Ok(bytes)
}

fn catalog_bytes(entry: &SignedSnapshotCatalogEntry) -> SnapshotResult<Vec<u8>> {
    Ok(SignedSnapshotCatalogEntryWire {
        body: Some(SnapshotCatalogEntryBodyWire::from(&entry.body)),
        authority_certificate: certificate_bytes(&entry.authority_certificate)?,
        signature: entry.signature.clone(),
    }
    .encode_to_vec())
}

fn manifest_bytes(manifest: &SignedTransferManifest) -> SnapshotResult<Vec<u8>> {
    Ok(SignedTransferManifestWire {
        body: Some(TransferManifestBodyWire::from(&manifest.body)),
        authority_certificate: certificate_bytes(&manifest.authority_certificate)?,
        signature: manifest.signature.clone(),
    }
    .encode_to_vec())
}

fn decode_catalog(bytes: &[u8]) -> SnapshotResult<SignedSnapshotCatalogEntry> {
    let wire =
        SignedSnapshotCatalogEntryWire::decode(bytes).map_err(|_| SnapshotError::Malformed)?;
    let body = SnapshotCatalogEntryBody::try_from(wire.body.ok_or(SnapshotError::Malformed)?)?;
    let authority_certificate = decode_certificate(&wire.authority_certificate)?;
    Ok(SignedSnapshotCatalogEntry {
        body,
        authority_certificate,
        signature: wire.signature,
    })
}

fn decode_manifest(bytes: &[u8]) -> SnapshotResult<SignedTransferManifest> {
    let wire = SignedTransferManifestWire::decode(bytes).map_err(|_| SnapshotError::Malformed)?;
    let body = TransferManifestBody::try_from(wire.body.ok_or(SnapshotError::Malformed)?)?;
    let authority_certificate = decode_certificate(&wire.authority_certificate)?;
    Ok(SignedTransferManifest {
        body,
        authority_certificate,
        signature: wire.signature,
    })
}

fn certificate_bytes(certificate: &AuthorityCertificate) -> SnapshotResult<Vec<u8>> {
    serde_jcs::to_vec(certificate).map_err(|_| SnapshotError::Encoding)
}

fn decode_certificate(bytes: &[u8]) -> SnapshotResult<AuthorityCertificate> {
    let certificate: AuthorityCertificate =
        serde_json::from_slice(bytes).map_err(|_| SnapshotError::Malformed)?;
    if certificate_bytes(&certificate)? != bytes {
        return Err(SnapshotError::NonCanonical);
    }
    Ok(certificate)
}

impl From<&SnapshotDescriptor> for SnapshotDescriptorWire {
    fn from(value: &SnapshotDescriptor) -> Self {
        Self {
            trust_domain: value.trust_domain.clone(),
            lineage_id: value.lineage_id.clone(),
            source_owner_id: value.source_owner_id.clone(),
            source_guardian_id: value.source_guardian_id.clone(),
            source_epoch: value.source_epoch,
            authority_log_index: value.authority_log_index,
            snapshot_schema: value.snapshot_schema.clone(),
            content_sha256: value.content_sha256.to_vec(),
            byte_length: value.byte_length,
            chunk_sha256: value
                .chunk_sha256
                .iter()
                .map(|digest| digest.to_vec())
                .collect(),
            created_at_unix_secs: value.created_at_unix_secs,
            expires_at_unix_secs: value.expires_at_unix_secs,
            encryption_context_id: value.encryption_context_id.clone(),
        }
    }
}

impl TryFrom<SnapshotDescriptorWire> for SnapshotDescriptor {
    type Error = SnapshotError;

    fn try_from(value: SnapshotDescriptorWire) -> SnapshotResult<Self> {
        Ok(Self {
            trust_domain: value.trust_domain,
            lineage_id: value.lineage_id,
            source_owner_id: value.source_owner_id,
            source_guardian_id: value.source_guardian_id,
            source_epoch: value.source_epoch,
            authority_log_index: value.authority_log_index,
            snapshot_schema: value.snapshot_schema,
            content_sha256: fixed_digest(value.content_sha256)?,
            byte_length: value.byte_length,
            chunk_sha256: value
                .chunk_sha256
                .into_iter()
                .map(fixed_digest)
                .collect::<SnapshotResult<Vec<_>>>()?,
            created_at_unix_secs: value.created_at_unix_secs,
            expires_at_unix_secs: value.expires_at_unix_secs,
            encryption_context_id: value.encryption_context_id,
        })
    }
}

impl From<&SnapshotCatalogEntryBody> for SnapshotCatalogEntryBodyWire {
    fn from(value: &SnapshotCatalogEntryBody) -> Self {
        Self {
            schema: value.schema.clone(),
            signer_id: value.signer_id.clone(),
            certificate_generation: value.certificate_generation,
            sequence: value.sequence,
            snapshot: Some(SnapshotDescriptorWire::from(&value.snapshot)),
        }
    }
}

impl TryFrom<SnapshotCatalogEntryBodyWire> for SnapshotCatalogEntryBody {
    type Error = SnapshotError;

    fn try_from(value: SnapshotCatalogEntryBodyWire) -> SnapshotResult<Self> {
        Ok(Self {
            schema: value.schema,
            signer_id: value.signer_id,
            certificate_generation: value.certificate_generation,
            sequence: value.sequence,
            snapshot: SnapshotDescriptor::try_from(
                value.snapshot.ok_or(SnapshotError::Malformed)?,
            )?,
        })
    }
}

impl From<&TransferManifestBody> for TransferManifestBodyWire {
    fn from(value: &TransferManifestBody) -> Self {
        Self {
            schema: value.schema.clone(),
            transfer_id: value.transfer_id.clone(),
            intended_target_id: value.intended_target_id.clone(),
            catalog_entry_sha256: value.catalog_entry_sha256.to_vec(),
            signer_id: value.signer_id.clone(),
            certificate_generation: value.certificate_generation,
            snapshot: Some(SnapshotDescriptorWire::from(&value.snapshot)),
        }
    }
}

impl TryFrom<TransferManifestBodyWire> for TransferManifestBody {
    type Error = SnapshotError;

    fn try_from(value: TransferManifestBodyWire) -> SnapshotResult<Self> {
        Ok(Self {
            schema: value.schema,
            transfer_id: value.transfer_id,
            intended_target_id: value.intended_target_id,
            catalog_entry_sha256: fixed_digest(value.catalog_entry_sha256)?,
            signer_id: value.signer_id,
            certificate_generation: value.certificate_generation,
            snapshot: SnapshotDescriptor::try_from(
                value.snapshot.ok_or(SnapshotError::Malformed)?,
            )?,
        })
    }
}

fn fixed_digest(bytes: Vec<u8>) -> SnapshotResult<[u8; 32]> {
    bytes.try_into().map_err(|_| SnapshotError::Malformed)
}

fn sha256(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn validate_text(value: &str) -> SnapshotResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_LEN
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(SnapshotError::InvalidText);
    }
    Ok(())
}

fn validate_identifier(value: &[u8]) -> SnapshotResult<()> {
    if !value
        .iter()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-' | b':'))
    {
        return Err(SnapshotError::InvalidText);
    }
    Ok(())
}

fn validate_database_path(path: &Path) -> SnapshotResult<PathBuf> {
    if !path.is_absolute() || path.file_name().is_none() {
        return Err(SnapshotError::InvalidPolicy);
    }
    let parent = path.parent().ok_or(SnapshotError::InvalidPolicy)?;
    fs::create_dir_all(parent).map_err(|_| SnapshotError::StateUnavailable)?;
    let mut cursor = PathBuf::new();
    for component in parent.components() {
        match component {
            Component::RootDir | Component::Prefix(_) => cursor.push(component.as_os_str()),
            Component::Normal(value) => {
                cursor.push(value);
                if fs::symlink_metadata(&cursor)
                    .map(|metadata| metadata.file_type().is_symlink())
                    .unwrap_or(false)
                {
                    return Err(SnapshotError::InvalidPolicy);
                }
            }
            Component::CurDir | Component::ParentDir => {
                return Err(SnapshotError::InvalidPolicy);
            }
        }
    }
    if fs::symlink_metadata(path)
        .map(|metadata| metadata.file_type().is_symlink())
        .unwrap_or(false)
    {
        return Err(SnapshotError::InvalidPolicy);
    }
    Ok(path.to_path_buf())
}
