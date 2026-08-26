use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    path::{Component, Path, PathBuf},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use redb::{
    Database, Durability, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const CERTIFICATE_SCHEMA: &str = "adl.distributed.authority_certificate.v1";
const SIGNING_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-AUTHORITY-CERTIFICATE-V1\0";
const SIGNATURE_LEN: usize = 64;
const MAX_TEXT_LEN: usize = 128;

const CERTIFICATES: TableDefinition<&str, &[u8]> =
    TableDefinition::new("distributed_certificates_v1");
const REVOCATIONS: TableDefinition<&str, &[u8]> =
    TableDefinition::new("distributed_certificate_revocations_v1");
const FENCES: TableDefinition<&str, &[u8]> =
    TableDefinition::new("distributed_certificate_fences_v1");

mod raw_access {
    const CERTIFICATE_STORE_ACCESS_MAGIC: [u8; 32] = [
        0x41, 0x44, 0x4c, 0x2d, 0x43, 0x45, 0x52, 0x54, 0x2d, 0x53, 0x54, 0x4f, 0x52, 0x45, 0x2d,
        0x41, 0x43, 0x43, 0x45, 0x53, 0x53, 0x2d, 0x53, 0x45, 0x41, 0x4c, 0x2d, 0x56, 0x31, 0x21,
        0x02, 0x58,
    ];

    #[derive(Debug)]
    struct CertificateStoreAccessSeal {
        magic: [u8; 32],
    }

    static AUTHORITY_BOUND_SEAL: CertificateStoreAccessSeal = CertificateStoreAccessSeal {
        magic: CERTIFICATE_STORE_ACCESS_MAGIC,
    };

    #[cfg(any(test, feature = "internal-test-fixtures"))]
    static TEST_FIXTURE_SEAL: CertificateStoreAccessSeal = CertificateStoreAccessSeal {
        magic: CERTIFICATE_STORE_ACCESS_MAGIC,
    };

    #[derive(Clone, Copy, Debug)]
    pub struct CertificateStoreAccess {
        seal: &'static CertificateStoreAccessSeal,
    }

    pub(crate) const AUTHORITY_BOUND: CertificateStoreAccess = CertificateStoreAccess {
        seal: &AUTHORITY_BOUND_SEAL,
    };

    #[cfg(test)]
    pub(crate) const TEST_FIXTURE: CertificateStoreAccess = CertificateStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    #[cfg(all(not(test), feature = "internal-test-fixtures"))]
    #[doc(hidden)]
    pub const TEST_FIXTURE: CertificateStoreAccess = CertificateStoreAccess {
        seal: &TEST_FIXTURE_SEAL,
    };

    pub(super) fn validate(access: &CertificateStoreAccess) -> bool {
        #[cfg(any(test, feature = "internal-test-fixtures"))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL)
            || std::ptr::eq(access.seal, &TEST_FIXTURE_SEAL);
        #[cfg(not(any(test, feature = "internal-test-fixtures")))]
        let known_seal = std::ptr::eq(access.seal, &AUTHORITY_BOUND_SEAL);
        known_seal && access.seal.magic == CERTIFICATE_STORE_ACCESS_MAGIC
    }
}

pub use raw_access::CertificateStoreAccess;
#[allow(unused_imports)]
pub(crate) use raw_access::AUTHORITY_BOUND as AUTHORITY_BOUND_CERTIFICATE_ACCESS;
#[cfg(test)]
#[allow(unused_imports)]
pub(crate) use raw_access::TEST_FIXTURE as TEST_CERTIFICATE_STORE_ACCESS;
#[cfg(all(not(test), feature = "internal-test-fixtures"))]
#[doc(hidden)]
#[allow(unused_imports)]
pub use raw_access::TEST_FIXTURE as TEST_CERTIFICATE_STORE_ACCESS;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificatePurpose {
    NodeIdentity,
    GuardianControl,
    Transport,
    AdvertisementSigning,
    SnapshotSigning,
}

impl CertificatePurpose {
    fn code(self) -> &'static str {
        match self {
            Self::NodeIdentity => "node_identity",
            Self::GuardianControl => "guardian_control",
            Self::Transport => "transport",
            Self::AdvertisementSigning => "advertisement_signing",
            Self::SnapshotSigning => "snapshot_signing",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CertificateBody {
    pub schema: String,
    pub trust_domain: String,
    pub holder_id: String,
    pub purpose: CertificatePurpose,
    pub generation: u64,
    pub issued_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
    pub subject_public_key: [u8; 32],
    pub issuer_root_id: String,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertificateValidity {
    pub issued_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

impl CertificateBody {
    pub fn new(
        trust_domain: impl Into<String>,
        holder_id: impl Into<String>,
        purpose: CertificatePurpose,
        generation: u64,
        validity: CertificateValidity,
        subject_public_key: VerifyingKey,
        issuer_root: &VerifyingKey,
    ) -> Self {
        Self {
            schema: CERTIFICATE_SCHEMA.to_owned(),
            trust_domain: trust_domain.into(),
            holder_id: holder_id.into(),
            purpose,
            generation,
            issued_at_unix_secs: validity.issued_at_unix_secs,
            expires_at_unix_secs: validity.expires_at_unix_secs,
            subject_public_key: subject_public_key.to_bytes(),
            issuer_root_id: root_id(issuer_root),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct AuthorityCertificate {
    pub body: CertificateBody,
    pub issuer_signature: Vec<u8>,
}

impl AuthorityCertificate {
    pub fn issue(body: CertificateBody, issuer: &SigningKey) -> CertificateResult<Self> {
        if body.issuer_root_id != root_id(&issuer.verifying_key()) {
            return Err(CertificateError::IssuerMismatch);
        }
        validate_body_shape(&body)?;
        let issuer_signature = issuer.sign(&signing_bytes(&body)?).to_bytes().to_vec();
        Ok(Self {
            body,
            issuer_signature,
        })
    }

    pub fn certificate_id(&self) -> CertificateResult<String> {
        let bytes = serde_jcs::to_vec(self).map_err(encoding_error)?;
        Ok(format!("cert_{}", hex::encode(Sha256::digest(bytes))))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CertificateStatus {
    Active,
    Superseded,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CertificateAuthorityRevision {
    state_sha256: [u8; 32],
}

impl CertificateAuthorityRevision {
    pub fn state_sha256(&self) -> [u8; 32] {
        self.state_sha256
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RedactedCertificateHealth {
    Active,
    RotationOverlap,
    Superseded,
    NotYetValid,
    Expired,
    Revoked,
    Fenced,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedCertificateRow {
    certificate_ref: String,
    node_ref: Option<String>,
    guardian_ref: Option<String>,
    purpose: CertificatePurpose,
    generation: u64,
    health: RedactedCertificateHealth,
}

impl RedactedCertificateRow {
    pub fn certificate_ref(&self) -> &str {
        &self.certificate_ref
    }

    pub fn node_ref(&self) -> Option<&str> {
        self.node_ref.as_deref()
    }

    pub fn guardian_ref(&self) -> Option<&str> {
        self.guardian_ref.as_deref()
    }

    pub fn purpose(&self) -> CertificatePurpose {
        self.purpose
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn health(&self) -> RedactedCertificateHealth {
        self.health
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RedactedCertificateSnapshot {
    trust_domain: String,
    captured_at_unix_secs: u64,
    revision: CertificateAuthorityRevision,
    rows: Vec<RedactedCertificateRow>,
}

impl RedactedCertificateSnapshot {
    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn captured_at_unix_secs(&self) -> u64 {
        self.captured_at_unix_secs
    }

    pub fn revision(&self) -> CertificateAuthorityRevision {
        self.revision
    }

    pub fn rows(&self) -> impl ExactSizeIterator<Item = &RedactedCertificateRow> {
        self.rows.iter()
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CertificateRecord {
    pub certificate: AuthorityCertificate,
    pub status: CertificateStatus,
    pub activated_at_unix_secs: u64,
    pub overlap_until_unix_secs: Option<u64>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RevocationReason {
    OperatorRevoked,
    KeyCompromise,
    ReEnrollmentRequired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RevocationRecord {
    pub certificate_id: String,
    pub revoked_at_unix_secs: u64,
    pub reason: RevocationReason,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct IdentityFence {
    pub holder_id: String,
    pub fenced_at_unix_secs: u64,
    pub reason: RevocationReason,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerifiedCertificate {
    pub certificate_id: String,
    pub holder_id: String,
    pub purpose: CertificatePurpose,
    pub generation: u64,
    pub authorization_deadline_unix_secs: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivationOutcome {
    Activated(CertificateRecord),
    AlreadyActive(CertificateRecord),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CertificateError {
    RelativeDatabasePath,
    DatabasePathIsSymlink,
    InvalidCertificate,
    InvalidTrustDomain,
    InvalidHolder,
    InvalidGeneration,
    InvalidLifetime,
    NotYetValid,
    Expired,
    WrongTrustDomain,
    WrongPurpose,
    IssuerNotApproved,
    IssuerMismatch,
    MalformedSignature,
    InvalidIssuerSignature,
    GenerationNotMonotonic,
    RotationOverlapExpired,
    Revoked,
    IdentityFenced,
    KeyPurposeConflict,
    DuplicateGuardianControlKey,
    CertificateNotFound,
    ResourceExhausted,
    DurableStateCorrupt,
    RevisionDrift,
    Storage(String),
    Encoding(String),
}

impl CertificateError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RelativeDatabasePath => "relative_database_path",
            Self::DatabasePathIsSymlink => "database_path_is_symlink",
            Self::InvalidCertificate => "invalid_certificate",
            Self::InvalidTrustDomain => "invalid_trust_domain",
            Self::InvalidHolder => "invalid_holder",
            Self::InvalidGeneration => "invalid_generation",
            Self::InvalidLifetime => "invalid_lifetime",
            Self::NotYetValid => "not_yet_valid",
            Self::Expired => "expired",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::WrongPurpose => "wrong_purpose",
            Self::IssuerNotApproved => "issuer_not_approved",
            Self::IssuerMismatch => "issuer_mismatch",
            Self::MalformedSignature => "malformed_signature",
            Self::InvalidIssuerSignature => "invalid_issuer_signature",
            Self::GenerationNotMonotonic => "generation_not_monotonic",
            Self::RotationOverlapExpired => "rotation_overlap_expired",
            Self::Revoked => "revoked",
            Self::IdentityFenced => "identity_fenced",
            Self::KeyPurposeConflict => "key_purpose_conflict",
            Self::DuplicateGuardianControlKey => "duplicate_guardian_control_key",
            Self::CertificateNotFound => "certificate_not_found",
            Self::ResourceExhausted => "resource_exhausted",
            Self::DurableStateCorrupt => "durable_state_corrupt",
            Self::RevisionDrift => "revision_drift",
            Self::Storage(_) => "storage_error",
            Self::Encoding(_) => "encoding_error",
        }
    }
}

impl fmt::Display for CertificateError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(detail) | Self::Encoding(detail) => {
                write!(formatter, "{}: {detail}", self.code())
            }
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for CertificateError {}

pub type CertificateResult<T> = Result<T, CertificateError>;

fn validate_raw_access(access: &CertificateStoreAccess) -> CertificateResult<()> {
    raw_access::validate(access)
        .then_some(())
        .ok_or(CertificateError::IssuerNotApproved)
}

#[derive(Clone)]
pub struct CertificatePolicy {
    trust_domain: String,
    approved_roots: BTreeMap<String, [u8; 32]>,
    max_certificate_lifetime_secs: u64,
    max_rotation_overlap_secs: u64,
    revocation_refresh_secs: u64,
    max_certificates: u64,
    max_revocations: u64,
}

impl CertificatePolicy {
    pub fn new(
        trust_domain: impl Into<String>,
        approved_roots: impl IntoIterator<Item = VerifyingKey>,
    ) -> CertificateResult<Self> {
        let trust_domain = trust_domain.into();
        validate_text(&trust_domain, CertificateError::InvalidTrustDomain)?;
        let approved_roots = approved_roots
            .into_iter()
            .map(|key| (root_id(&key), key.to_bytes()))
            .collect::<BTreeMap<_, _>>();
        if approved_roots.is_empty() {
            return Err(CertificateError::IssuerNotApproved);
        }
        Ok(Self {
            trust_domain,
            approved_roots,
            max_certificate_lifetime_secs: 24 * 60 * 60,
            max_rotation_overlap_secs: 5 * 60,
            revocation_refresh_secs: 60,
            max_certificates: 4096,
            max_revocations: 8192,
        })
    }

    pub fn with_bounds(
        mut self,
        max_certificate_lifetime_secs: u64,
        max_rotation_overlap_secs: u64,
        revocation_refresh_secs: u64,
        max_certificates: u64,
        max_revocations: u64,
    ) -> CertificateResult<Self> {
        if max_certificate_lifetime_secs == 0
            || max_rotation_overlap_secs == 0
            || revocation_refresh_secs == 0
            || max_certificates == 0
            || max_revocations == 0
        {
            return Err(CertificateError::ResourceExhausted);
        }
        self.max_certificate_lifetime_secs = max_certificate_lifetime_secs;
        self.max_rotation_overlap_secs = max_rotation_overlap_secs;
        self.revocation_refresh_secs = revocation_refresh_secs;
        self.max_certificates = max_certificates;
        self.max_revocations = max_revocations;
        Ok(self)
    }
}

pub struct DistributedCertificateStore {
    database_path: PathBuf,
    database: Database,
    policy: CertificatePolicy,
}

impl DistributedCertificateStore {
    pub fn open(
        access: &CertificateStoreAccess,
        database_path: impl AsRef<Path>,
        policy: CertificatePolicy,
    ) -> CertificateResult<Self> {
        validate_raw_access(access)?;
        let database_path = database_path.as_ref();
        if !database_path.is_absolute() {
            return Err(CertificateError::RelativeDatabasePath);
        }
        reject_symlink_components(database_path)?;
        fs::create_dir_all(
            database_path
                .parent()
                .ok_or(CertificateError::RelativeDatabasePath)?,
        )
        .map_err(storage_error)?;
        reject_symlink_components(database_path)?;
        let database = Database::create(database_path).map_err(storage_error)?;
        let store = Self {
            database_path: database_path.to_path_buf(),
            database,
            policy,
        };
        store.initialize_tables()?;
        store.validate_durable_state()?;
        Ok(store)
    }

    pub fn database_path(&self) -> &Path {
        &self.database_path
    }

    pub fn activate(
        &self,
        access: &CertificateStoreAccess,
        certificate: &AuthorityCertificate,
        now_unix_secs: u64,
    ) -> CertificateResult<ActivationOutcome> {
        validate_raw_access(access)?;
        self.validate_certificate(certificate, Some(now_unix_secs))?;
        if self.is_fenced(&certificate.body.holder_id)? {
            return Err(CertificateError::IdentityFenced);
        }
        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let mut records = read_records(&write)?;
        let key = record_key(&certificate.body);
        if let Some(existing) = records.iter().find(|(stored_key, _)| stored_key == &key) {
            if existing.1.certificate == *certificate
                && existing.1.status == CertificateStatus::Active
            {
                write.commit().map_err(storage_error)?;
                return Ok(ActivationOutcome::AlreadyActive(existing.1.clone()));
            }
            return Err(CertificateError::GenerationNotMonotonic);
        }
        if records.len() as u64 >= self.policy.max_certificates {
            return Err(CertificateError::ResourceExhausted);
        }
        let body = &certificate.body;
        let latest_generation = records
            .iter()
            .filter(|(_, record)| same_holder_purpose(&record.certificate.body, body))
            .map(|(_, record)| record.certificate.body.generation)
            .max()
            .unwrap_or(0);
        if body.generation <= latest_generation {
            return Err(CertificateError::GenerationNotMonotonic);
        }
        for (_, record) in &records {
            let stored = &record.certificate.body;
            if stored.subject_public_key == body.subject_public_key
                && stored.purpose != body.purpose
            {
                return Err(CertificateError::KeyPurposeConflict);
            }
            if body.purpose == CertificatePurpose::GuardianControl
                && stored.purpose == CertificatePurpose::GuardianControl
                && stored.subject_public_key == body.subject_public_key
                && stored.holder_id != body.holder_id
            {
                return Err(CertificateError::DuplicateGuardianControlKey);
            }
        }

        let overlap_cap = now_unix_secs
            .checked_add(self.policy.max_rotation_overlap_secs)
            .ok_or(CertificateError::InvalidLifetime)?;
        for (stored_key, record) in &mut records {
            if record.status == CertificateStatus::Active
                && same_holder_purpose(&record.certificate.body, body)
            {
                record.status = CertificateStatus::Superseded;
                record.overlap_until_unix_secs =
                    Some(overlap_cap.min(record.certificate.body.expires_at_unix_secs));
                let encoded = encode(record)?;
                write
                    .open_table(CERTIFICATES)
                    .map_err(storage_error)?
                    .insert(stored_key.as_str(), encoded.as_slice())
                    .map_err(storage_error)?;
            }
        }
        let record = CertificateRecord {
            certificate: certificate.clone(),
            status: CertificateStatus::Active,
            activated_at_unix_secs: now_unix_secs,
            overlap_until_unix_secs: None,
        };
        let encoded = encode(&record)?;
        write
            .open_table(CERTIFICATES)
            .map_err(storage_error)?
            .insert(key.as_str(), encoded.as_slice())
            .map_err(storage_error)?;
        write.commit().map_err(storage_error)?;
        Ok(ActivationOutcome::Activated(record))
    }

    pub fn authorize(
        &self,
        access: &CertificateStoreAccess,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> CertificateResult<VerifiedCertificate> {
        validate_raw_access(access)?;
        validate_text(holder_id, CertificateError::InvalidHolder)?;
        if self.is_fenced(holder_id)? {
            return Err(CertificateError::IdentityFenced);
        }
        let read = self.database.begin_read().map_err(storage_error)?;
        let key = record_key_parts(holder_id, purpose, generation);
        let bytes = read
            .open_table(CERTIFICATES)
            .map_err(storage_error)?
            .get(key.as_str())
            .map_err(storage_error)?
            .map(|value| value.value().to_vec())
            .ok_or(CertificateError::CertificateNotFound)?;
        let record: CertificateRecord = decode(&bytes)?;
        if record.certificate.body.purpose != purpose {
            return Err(CertificateError::WrongPurpose);
        }
        self.validate_certificate(&record.certificate, Some(now_unix_secs))?;
        let certificate_id = record.certificate.certificate_id()?;
        if self.is_revoked(&certificate_id)? {
            return Err(CertificateError::Revoked);
        }
        if record.status == CertificateStatus::Superseded
            && record
                .overlap_until_unix_secs
                .is_none_or(|deadline| now_unix_secs >= deadline)
        {
            return Err(CertificateError::RotationOverlapExpired);
        }
        let refresh_deadline = now_unix_secs
            .checked_add(self.policy.revocation_refresh_secs)
            .ok_or(CertificateError::InvalidLifetime)?;
        Ok(VerifiedCertificate {
            certificate_id,
            holder_id: holder_id.to_owned(),
            purpose,
            generation,
            authorization_deadline_unix_secs: refresh_deadline
                .min(record.certificate.body.expires_at_unix_secs),
        })
    }

    pub fn revoke(
        &self,
        access: &CertificateStoreAccess,
        certificate_id: &str,
        now_unix_secs: u64,
        reason: RevocationReason,
    ) -> CertificateResult<()> {
        validate_raw_access(access)?;
        if reason == RevocationReason::KeyCompromise {
            return self.mark_compromised(certificate_id, now_unix_secs);
        }
        validate_certificate_id(certificate_id)?;
        if !self.certificate_exists(certificate_id)? {
            return Err(CertificateError::CertificateNotFound);
        }
        let write = self.database.begin_write().map_err(storage_error)?;
        let mut table = write.open_table(REVOCATIONS).map_err(storage_error)?;
        if table.get(certificate_id).map_err(storage_error)?.is_none()
            && table.len().map_err(storage_error)? >= self.policy.max_revocations
        {
            return Err(CertificateError::ResourceExhausted);
        }
        let record = RevocationRecord {
            certificate_id: certificate_id.to_owned(),
            revoked_at_unix_secs: now_unix_secs,
            reason,
        };
        let encoded = encode(&record)?;
        table
            .insert(certificate_id, encoded.as_slice())
            .map_err(storage_error)?;
        drop(table);
        write.commit().map_err(storage_error)
    }

    pub fn mark_compromised(
        &self,
        certificate_id: &str,
        now_unix_secs: u64,
    ) -> CertificateResult<()> {
        validate_certificate_id(certificate_id)?;
        let holder_id = self
            .record_by_certificate_id(certificate_id)?
            .ok_or(CertificateError::CertificateNotFound)?
            .certificate
            .body
            .holder_id;
        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        {
            let mut revocations = write.open_table(REVOCATIONS).map_err(storage_error)?;
            if revocations
                .get(certificate_id)
                .map_err(storage_error)?
                .is_none()
                && revocations.len().map_err(storage_error)? >= self.policy.max_revocations
            {
                return Err(CertificateError::ResourceExhausted);
            }
            let revocation = RevocationRecord {
                certificate_id: certificate_id.to_owned(),
                revoked_at_unix_secs: now_unix_secs,
                reason: RevocationReason::KeyCompromise,
            };
            let encoded = encode(&revocation)?;
            revocations
                .insert(certificate_id, encoded.as_slice())
                .map_err(storage_error)?;
        }
        let fence = IdentityFence {
            holder_id: holder_id.clone(),
            fenced_at_unix_secs: now_unix_secs,
            reason: RevocationReason::KeyCompromise,
        };
        let encoded = encode(&fence)?;
        {
            let mut fences = write.open_table(FENCES).map_err(storage_error)?;
            if fences
                .get(holder_id.as_str())
                .map_err(storage_error)?
                .is_none()
                && fences.len().map_err(storage_error)? >= self.policy.max_certificates
            {
                return Err(CertificateError::ResourceExhausted);
            }
            fences
                .insert(holder_id.as_str(), encoded.as_slice())
                .map_err(storage_error)?;
        }
        write.commit().map_err(storage_error)
    }

    pub fn quorum_voter_count(&self, now_unix_secs: u64) -> CertificateResult<usize> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(CERTIFICATES).map_err(storage_error)?;
        let mut voters = BTreeSet::new();
        for row in table.iter().map_err(storage_error)? {
            let (_, value) = row.map_err(storage_error)?;
            let record: CertificateRecord = decode(value.value())?;
            let body = &record.certificate.body;
            if body.purpose == CertificatePurpose::GuardianControl
                && self.record_is_live(&record, now_unix_secs)?
                && !self.is_fenced(&body.holder_id)?
            {
                voters.insert(body.holder_id.clone());
            }
        }
        Ok(voters.len())
    }

    pub fn authority_revision(&self) -> CertificateResult<CertificateAuthorityRevision> {
        let read = self.database.begin_read().map_err(storage_error)?;
        Ok(CertificateAuthorityRevision {
            state_sha256: certificate_state_sha256(&read)?,
        })
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: CertificateAuthorityRevision,
        now_unix_secs: u64,
    ) -> CertificateResult<RedactedCertificateSnapshot> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let revision = CertificateAuthorityRevision {
            state_sha256: certificate_state_sha256(&read)?,
        };
        if revision != expected_revision {
            return Err(CertificateError::RevisionDrift);
        }
        let certificates = read.open_table(CERTIFICATES).map_err(storage_error)?;
        if certificates.len().map_err(storage_error)? > self.policy.max_certificates {
            return Err(CertificateError::ResourceExhausted);
        }
        let revocations = read.open_table(REVOCATIONS).map_err(storage_error)?;
        let fences = read.open_table(FENCES).map_err(storage_error)?;
        let mut rows = Vec::with_capacity(
            usize::try_from(certificates.len().map_err(storage_error)?)
                .map_err(|_| CertificateError::ResourceExhausted)?,
        );
        for entry in certificates.iter().map_err(storage_error)? {
            let (_, value) = entry.map_err(storage_error)?;
            let record: CertificateRecord = decode(value.value())?;
            let body = &record.certificate.body;
            let certificate_id = record.certificate.certificate_id()?;
            let revoked = revocations
                .get(certificate_id.as_str())
                .map_err(storage_error)?
                .is_some();
            let fenced = fences
                .get(body.holder_id.as_str())
                .map_err(storage_error)?
                .is_some();
            let health = if fenced {
                RedactedCertificateHealth::Fenced
            } else if revoked {
                RedactedCertificateHealth::Revoked
            } else if now_unix_secs < body.issued_at_unix_secs {
                RedactedCertificateHealth::NotYetValid
            } else if now_unix_secs >= body.expires_at_unix_secs {
                RedactedCertificateHealth::Expired
            } else if record.status == CertificateStatus::Active {
                RedactedCertificateHealth::Active
            } else if record
                .overlap_until_unix_secs
                .is_some_and(|deadline| now_unix_secs < deadline)
            {
                RedactedCertificateHealth::RotationOverlap
            } else {
                RedactedCertificateHealth::Superseded
            };
            let holder_ref = match body.purpose {
                CertificatePurpose::NodeIdentity | CertificatePurpose::Transport => (
                    Some(projection_ref(b"node", body.holder_id.as_bytes())),
                    None,
                ),
                CertificatePurpose::GuardianControl
                | CertificatePurpose::AdvertisementSigning
                | CertificatePurpose::SnapshotSigning => (
                    None,
                    Some(projection_ref(b"guardian", body.holder_id.as_bytes())),
                ),
            };
            rows.push(RedactedCertificateRow {
                certificate_ref: projection_ref(b"certificate", certificate_id.as_bytes()),
                node_ref: holder_ref.0,
                guardian_ref: holder_ref.1,
                purpose: body.purpose,
                generation: body.generation,
                health,
            });
        }
        Ok(RedactedCertificateSnapshot {
            trust_domain: self.policy.trust_domain.clone(),
            captured_at_unix_secs: now_unix_secs,
            revision,
            rows,
        })
    }

    fn initialize_tables(&self) -> CertificateResult<()> {
        let write = self.database.begin_write().map_err(storage_error)?;
        write.open_table(CERTIFICATES).map_err(storage_error)?;
        write.open_table(REVOCATIONS).map_err(storage_error)?;
        write.open_table(FENCES).map_err(storage_error)?;
        write.commit().map_err(storage_error)
    }

    fn validate_certificate(
        &self,
        certificate: &AuthorityCertificate,
        now_unix_secs: Option<u64>,
    ) -> CertificateResult<()> {
        validate_body_shape(&certificate.body)?;
        if certificate.body.trust_domain != self.policy.trust_domain {
            return Err(CertificateError::WrongTrustDomain);
        }
        let lifetime = certificate
            .body
            .expires_at_unix_secs
            .checked_sub(certificate.body.issued_at_unix_secs)
            .ok_or(CertificateError::InvalidLifetime)?;
        if lifetime == 0 || lifetime > self.policy.max_certificate_lifetime_secs {
            return Err(CertificateError::InvalidLifetime);
        }
        if let Some(now) = now_unix_secs {
            if now < certificate.body.issued_at_unix_secs {
                return Err(CertificateError::NotYetValid);
            }
            if now >= certificate.body.expires_at_unix_secs {
                return Err(CertificateError::Expired);
            }
        }
        if certificate.issuer_signature.len() != SIGNATURE_LEN {
            return Err(CertificateError::MalformedSignature);
        }
        let root_bytes = self
            .policy
            .approved_roots
            .get(&certificate.body.issuer_root_id)
            .ok_or(CertificateError::IssuerNotApproved)?;
        let root = VerifyingKey::from_bytes(root_bytes)
            .map_err(|_| CertificateError::IssuerNotApproved)?;
        if root_id(&root) != certificate.body.issuer_root_id {
            return Err(CertificateError::IssuerMismatch);
        }
        let signature = Signature::from_slice(&certificate.issuer_signature)
            .map_err(|_| CertificateError::MalformedSignature)?;
        root.verify_strict(&signing_bytes(&certificate.body)?, &signature)
            .map_err(|_| CertificateError::InvalidIssuerSignature)
    }

    fn validate_durable_state(&self) -> CertificateResult<()> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let certificates = read.open_table(CERTIFICATES).map_err(storage_error)?;
        if certificates.len().map_err(storage_error)? > self.policy.max_certificates {
            return Err(CertificateError::DurableStateCorrupt);
        }
        let mut keys = BTreeMap::<[u8; 32], CertificatePurpose>::new();
        let mut guardians = BTreeMap::<[u8; 32], String>::new();
        let mut certificate_ids = BTreeSet::new();
        let mut holders = BTreeSet::new();
        let mut active_holder_purposes = BTreeSet::new();
        for row in certificates.iter().map_err(storage_error)? {
            let (key, value) = row.map_err(storage_error)?;
            let record: CertificateRecord =
                decode(value.value()).map_err(|_| CertificateError::DurableStateCorrupt)?;
            self.validate_certificate(&record.certificate, None)
                .map_err(|_| CertificateError::DurableStateCorrupt)?;
            let body = &record.certificate.body;
            certificate_ids.insert(
                record
                    .certificate
                    .certificate_id()
                    .map_err(|_| CertificateError::DurableStateCorrupt)?,
            );
            holders.insert(body.holder_id.clone());
            if key.value() != record_key(body)
                || record.activated_at_unix_secs < body.issued_at_unix_secs
                || (record.status == CertificateStatus::Active
                    && record.overlap_until_unix_secs.is_some())
                || (record.status == CertificateStatus::Superseded
                    && record.overlap_until_unix_secs.is_none())
                || record.overlap_until_unix_secs.is_some_and(|deadline| {
                    deadline <= record.activated_at_unix_secs
                        || deadline > body.expires_at_unix_secs
                })
            {
                return Err(CertificateError::DurableStateCorrupt);
            }
            if record.status == CertificateStatus::Active
                && !active_holder_purposes.insert((body.holder_id.clone(), body.purpose))
            {
                return Err(CertificateError::DurableStateCorrupt);
            }
            if let Some(purpose) = keys.insert(body.subject_public_key, body.purpose) {
                if purpose != body.purpose {
                    return Err(CertificateError::DurableStateCorrupt);
                }
            }
            if body.purpose == CertificatePurpose::GuardianControl {
                if let Some(holder) =
                    guardians.insert(body.subject_public_key, body.holder_id.clone())
                {
                    if holder != body.holder_id {
                        return Err(CertificateError::DurableStateCorrupt);
                    }
                }
            }
        }
        let revocations = read.open_table(REVOCATIONS).map_err(storage_error)?;
        if revocations.len().map_err(storage_error)? > self.policy.max_revocations {
            return Err(CertificateError::DurableStateCorrupt);
        }
        for row in revocations.iter().map_err(storage_error)? {
            let (key, value) = row.map_err(storage_error)?;
            let revocation: RevocationRecord =
                decode(value.value()).map_err(|_| CertificateError::DurableStateCorrupt)?;
            if key.value() != revocation.certificate_id
                || validate_certificate_id(&revocation.certificate_id).is_err()
                || !certificate_ids.contains(&revocation.certificate_id)
            {
                return Err(CertificateError::DurableStateCorrupt);
            }
        }
        let fences = read.open_table(FENCES).map_err(storage_error)?;
        if fences.len().map_err(storage_error)? > self.policy.max_certificates {
            return Err(CertificateError::DurableStateCorrupt);
        }
        for row in fences.iter().map_err(storage_error)? {
            let (key, value) = row.map_err(storage_error)?;
            let fence: IdentityFence =
                decode(value.value()).map_err(|_| CertificateError::DurableStateCorrupt)?;
            if key.value() != fence.holder_id
                || validate_text(&fence.holder_id, CertificateError::InvalidHolder).is_err()
                || !holders.contains(&fence.holder_id)
            {
                return Err(CertificateError::DurableStateCorrupt);
            }
        }
        Ok(())
    }

    fn record_is_live(
        &self,
        record: &CertificateRecord,
        now_unix_secs: u64,
    ) -> CertificateResult<bool> {
        if self
            .validate_certificate(&record.certificate, Some(now_unix_secs))
            .is_err()
            || self.is_revoked(&record.certificate.certificate_id()?)?
        {
            return Ok(false);
        }
        Ok(record.status == CertificateStatus::Active
            || record
                .overlap_until_unix_secs
                .is_some_and(|deadline| now_unix_secs < deadline))
    }

    fn is_revoked(&self, certificate_id: &str) -> CertificateResult<bool> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let result = read
            .open_table(REVOCATIONS)
            .map_err(storage_error)?
            .get(certificate_id)
            .map_err(storage_error)?
            .is_some();
        Ok(result)
    }

    fn is_fenced(&self, holder_id: &str) -> CertificateResult<bool> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let result = read
            .open_table(FENCES)
            .map_err(storage_error)?
            .get(holder_id)
            .map_err(storage_error)?
            .is_some();
        Ok(result)
    }

    fn certificate_exists(&self, certificate_id: &str) -> CertificateResult<bool> {
        Ok(self.record_by_certificate_id(certificate_id)?.is_some())
    }

    fn record_by_certificate_id(
        &self,
        certificate_id: &str,
    ) -> CertificateResult<Option<CertificateRecord>> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(CERTIFICATES).map_err(storage_error)?;
        for row in table.iter().map_err(storage_error)? {
            let (_, value) = row.map_err(storage_error)?;
            let record: CertificateRecord = decode(value.value())?;
            if record.certificate.certificate_id()? == certificate_id {
                return Ok(Some(record));
            }
        }
        Ok(None)
    }

    #[cfg(test)]
    pub fn corrupt_certificate_for_test(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
    ) -> CertificateResult<()> {
        let write = self.database.begin_write().map_err(storage_error)?;
        write
            .open_table(CERTIFICATES)
            .map_err(storage_error)?
            .insert(
                record_key_parts(holder_id, purpose, generation).as_str(),
                b"{".as_slice(),
            )
            .map_err(storage_error)?;
        write.commit().map_err(storage_error)
    }
}

fn certificate_state_sha256(read: &redb::ReadTransaction) -> CertificateResult<[u8; 32]> {
    let mut digest = Sha256::new();
    digest.update(b"ADL-CERTIFICATE-AUTHORITY-REVISION-V1\0");
    for (domain, definition) in [
        (b"certificates".as_slice(), CERTIFICATES),
        (b"revocations".as_slice(), REVOCATIONS),
        (b"fences".as_slice(), FENCES),
    ] {
        digest.update((domain.len() as u64).to_be_bytes());
        digest.update(domain);
        let table = read.open_table(definition).map_err(storage_error)?;
        for row in table.iter().map_err(storage_error)? {
            let (key, value) = row.map_err(storage_error)?;
            digest.update((key.value().len() as u64).to_be_bytes());
            digest.update(key.value().as_bytes());
            digest.update((value.value().len() as u64).to_be_bytes());
            digest.update(value.value());
        }
    }
    Ok(digest.finalize().into())
}

fn projection_ref(kind: &[u8], value: &[u8]) -> String {
    let mut digest = Sha256::new();
    digest.update(b"adl-projection-ref-v1");
    digest.update((kind.len() as u64).to_be_bytes());
    digest.update(kind);
    digest.update((value.len() as u64).to_be_bytes());
    digest.update(value);
    format!("id_{}", hex::encode(digest.finalize()))
}

fn same_holder_purpose(left: &CertificateBody, right: &CertificateBody) -> bool {
    left.holder_id == right.holder_id && left.purpose == right.purpose
}

fn record_key(body: &CertificateBody) -> String {
    record_key_parts(&body.holder_id, body.purpose, body.generation)
}

fn record_key_parts(holder: &str, purpose: CertificatePurpose, generation: u64) -> String {
    format!("{holder}|{}|{generation:020}", purpose.code())
}

fn read_records(
    write: &redb::WriteTransaction,
) -> CertificateResult<Vec<(String, CertificateRecord)>> {
    let table = write.open_table(CERTIFICATES).map_err(storage_error)?;
    let mut records = Vec::new();
    for row in table.iter().map_err(storage_error)? {
        let (key, value) = row.map_err(storage_error)?;
        records.push((key.value().to_owned(), decode(value.value())?));
    }
    Ok(records)
}

fn validate_body_shape(body: &CertificateBody) -> CertificateResult<()> {
    if body.schema != CERTIFICATE_SCHEMA {
        return Err(CertificateError::InvalidCertificate);
    }
    validate_text(&body.trust_domain, CertificateError::InvalidTrustDomain)?;
    validate_text(&body.holder_id, CertificateError::InvalidHolder)?;
    if body.generation == 0 {
        return Err(CertificateError::InvalidGeneration);
    }
    if body.expires_at_unix_secs <= body.issued_at_unix_secs {
        return Err(CertificateError::InvalidLifetime);
    }
    VerifyingKey::from_bytes(&body.subject_public_key)
        .map_err(|_| CertificateError::InvalidCertificate)?;
    validate_root_id(&body.issuer_root_id)
}

fn validate_text(value: &str, error: CertificateError) -> CertificateResult<()> {
    if value.is_empty()
        || value.len() > MAX_TEXT_LEN
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(error);
    }
    Ok(())
}

fn validate_root_id(value: &str) -> CertificateResult<()> {
    validate_prefixed_hex(value, "root_").map_err(|_| CertificateError::IssuerMismatch)
}

fn validate_certificate_id(value: &str) -> CertificateResult<()> {
    validate_prefixed_hex(value, "cert_").map_err(|_| CertificateError::InvalidCertificate)
}

fn validate_prefixed_hex(value: &str, prefix: &str) -> CertificateResult<()> {
    let suffix = value
        .strip_prefix(prefix)
        .ok_or(CertificateError::InvalidCertificate)?;
    if suffix.len() != 64
        || !suffix
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(CertificateError::InvalidCertificate);
    }
    Ok(())
}

pub fn root_id(root: &VerifyingKey) -> String {
    format!("root_{}", hex::encode(Sha256::digest(root.as_bytes())))
}

fn signing_bytes(body: &CertificateBody) -> CertificateResult<Vec<u8>> {
    let body = serde_jcs::to_vec(body).map_err(encoding_error)?;
    let mut bytes = Vec::with_capacity(SIGNING_DOMAIN.len() + body.len());
    bytes.extend_from_slice(SIGNING_DOMAIN);
    bytes.extend_from_slice(&body);
    Ok(bytes)
}

fn encode<T: Serialize>(value: &T) -> CertificateResult<Vec<u8>> {
    serde_json::to_vec(value).map_err(encoding_error)
}

fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8]) -> CertificateResult<T> {
    serde_json::from_slice(bytes).map_err(|_| CertificateError::DurableStateCorrupt)
}

fn reject_symlink_components(path: &Path) -> CertificateResult<()> {
    let mut current = PathBuf::new();
    for component in path.components() {
        match component {
            Component::RootDir | Component::Prefix(_) => current.push(component.as_os_str()),
            Component::Normal(value) => current.push(value),
            _ => return Err(CertificateError::RelativeDatabasePath),
        }
        if fs::symlink_metadata(&current).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return Err(CertificateError::DatabasePathIsSymlink);
        }
    }
    Ok(())
}

fn storage_error(error: impl fmt::Display) -> CertificateError {
    CertificateError::Storage(error.to_string())
}

fn encoding_error(error: impl fmt::Display) -> CertificateError {
    CertificateError::Encoding(error.to_string())
}
