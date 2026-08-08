use std::{
    collections::{BTreeMap, BTreeSet},
    fmt, fs,
    path::{Path, PathBuf},
};

use ed25519_dalek::{Signature, Signer, SigningKey, VerifyingKey};
use rand::RngCore;
use redb::{
    Database, Durability, ReadableDatabase, ReadableTable, ReadableTableMetadata, TableDefinition,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const IDENTITY_SCHEMA: &str = "adl.distributed.identity.v1";
pub const ENROLLMENT_SCHEMA: &str = "adl.distributed.enrollment.v1";
pub const AUDIT_SCHEMA: &str = "adl.distributed.enrollment_audit.v1";

const LOCAL_IDENTITY_KEY: &str = "local_identity";
const NEXT_AUDIT_SEQUENCE_KEY: &str = "next_audit_sequence";
const NODE_ID_PREFIX: &str = "node_";
const GUARDIAN_ID_PREFIX: &str = "guardian_";
const ROOT_ID_PREFIX: &str = "root_";
const IDENTIFIER_HEX_LEN: usize = 64;
const NONCE_LEN: usize = 32;
const SIGNATURE_LEN: usize = 64;
const OPERATOR_SIGNATURE_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-ENROLLMENT-OPERATOR-V1\0";
const NODE_PROOF_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-ENROLLMENT-NODE-POP-V1\0";
const GUARDIAN_PROOF_DOMAIN: &[u8] = b"ADL-DISTRIBUTED-ENROLLMENT-GUARDIAN-POP-V1\0";

const META: TableDefinition<&str, &[u8]> = TableDefinition::new("distributed_identity_meta_v1");
const ENROLLMENTS: TableDefinition<&str, &[u8]> =
    TableDefinition::new("distributed_enrollments_v1");
const USED_NONCES: TableDefinition<&[u8], u64> =
    TableDefinition::new("distributed_enrollment_nonces_v1");
const AUDIT: TableDefinition<u64, &[u8]> = TableDefinition::new("distributed_enrollment_audit_v1");

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdentityError {
    RelativeDatabasePath,
    DatabasePathIsSymlink,
    InvalidTrustDomain,
    InvalidIdentityGeneration,
    IdentityUnavailable,
    IdentityTrustDomainMismatch,
    IdentityGenerationMismatch,
    IdentityCorrupt,
    RequestTooLarge,
    RequestNotYetValid,
    RequestExpired,
    RequestLifetimeInvalid,
    WrongTrustDomain,
    InvalidNonce,
    Replay,
    MalformedSignature,
    OperatorRootNotApproved,
    InvalidOperatorSignature,
    InvalidNodeProof,
    InvalidGuardianProof,
    NodeIdentityMismatch,
    GuardianIdentityMismatch,
    ConflictingNodeIdentity,
    ConflictingGuardianIdentity,
    DuplicateGuardianControlKey,
    ResourceExhausted,
    DurableStateCorrupt,
    Storage(String),
    Encoding(String),
}

impl IdentityError {
    pub fn code(&self) -> &'static str {
        match self {
            Self::RelativeDatabasePath => "relative_database_path",
            Self::DatabasePathIsSymlink => "database_path_is_symlink",
            Self::InvalidTrustDomain => "invalid_trust_domain",
            Self::InvalidIdentityGeneration => "invalid_identity_generation",
            Self::IdentityUnavailable => "identity_unavailable",
            Self::IdentityTrustDomainMismatch => "identity_trust_domain_mismatch",
            Self::IdentityGenerationMismatch => "identity_generation_mismatch",
            Self::IdentityCorrupt => "identity_corrupt",
            Self::RequestTooLarge => "request_too_large",
            Self::RequestNotYetValid => "request_not_yet_valid",
            Self::RequestExpired => "request_expired",
            Self::RequestLifetimeInvalid => "request_lifetime_invalid",
            Self::WrongTrustDomain => "wrong_trust_domain",
            Self::InvalidNonce => "invalid_nonce",
            Self::Replay => "replay",
            Self::MalformedSignature => "malformed_signature",
            Self::OperatorRootNotApproved => "operator_root_not_approved",
            Self::InvalidOperatorSignature => "invalid_operator_signature",
            Self::InvalidNodeProof => "invalid_node_proof",
            Self::InvalidGuardianProof => "invalid_guardian_proof",
            Self::NodeIdentityMismatch => "node_identity_mismatch",
            Self::GuardianIdentityMismatch => "guardian_identity_mismatch",
            Self::ConflictingNodeIdentity => "conflicting_node_identity",
            Self::ConflictingGuardianIdentity => "conflicting_guardian_identity",
            Self::DuplicateGuardianControlKey => "duplicate_guardian_control_key",
            Self::ResourceExhausted => "resource_exhausted",
            Self::DurableStateCorrupt => "durable_state_corrupt",
            Self::Storage(_) => "storage_error",
            Self::Encoding(_) => "encoding_error",
        }
    }
}

impl fmt::Display for IdentityError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Storage(detail) => write!(formatter, "{}: {detail}", self.code()),
            Self::Encoding(detail) => write!(formatter, "{}: {detail}", self.code()),
            _ => formatter.write_str(self.code()),
        }
    }
}

impl std::error::Error for IdentityError {}

pub type IdentityResult<T> = Result<T, IdentityError>;

#[derive(Clone)]
pub struct EnrollmentPolicy {
    trust_domain: String,
    approved_operator_roots: BTreeMap<String, [u8; 32]>,
    max_request_bytes: usize,
    max_request_lifetime_secs: u64,
    max_enrollments: u64,
    max_outstanding_nonces: u64,
    max_audit_events: u64,
}

impl EnrollmentPolicy {
    pub fn new(
        trust_domain: impl Into<String>,
        approved_operator_roots: impl IntoIterator<Item = VerifyingKey>,
    ) -> IdentityResult<Self> {
        let trust_domain = trust_domain.into();
        validate_trust_domain(&trust_domain)?;
        let approved_operator_roots = approved_operator_roots
            .into_iter()
            .map(|key| (operator_root_id(&key), key.to_bytes()))
            .collect::<BTreeMap<_, _>>();
        if approved_operator_roots.is_empty() {
            return Err(IdentityError::OperatorRootNotApproved);
        }
        Ok(Self {
            trust_domain,
            approved_operator_roots,
            max_request_bytes: 16 * 1024,
            max_request_lifetime_secs: 15 * 60,
            max_enrollments: 4096,
            max_outstanding_nonces: 8192,
            max_audit_events: 4096,
        })
    }

    pub fn with_limits(
        mut self,
        max_enrollments: u64,
        max_outstanding_nonces: u64,
        max_audit_events: u64,
    ) -> IdentityResult<Self> {
        if max_enrollments == 0 || max_outstanding_nonces == 0 || max_audit_events == 0 {
            return Err(IdentityError::ResourceExhausted);
        }
        self.max_enrollments = max_enrollments;
        self.max_outstanding_nonces = max_outstanding_nonces;
        self.max_audit_events = max_audit_events;
        Ok(self)
    }

    pub fn trust_domain(&self) -> &str {
        &self.trust_domain
    }

    pub fn approved_root_id(&self, key: &VerifyingKey) -> Option<String> {
        let id = operator_root_id(key);
        self.approved_operator_roots.contains_key(&id).then_some(id)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GuardianEnrollmentRole {
    Voter,
    NonVoting,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PublicNodeGuardianIdentity {
    pub schema: String,
    pub trust_domain: String,
    pub identity_generation: u64,
    pub node_id: String,
    pub guardian_id: String,
    pub node_public_key: [u8; 32],
    pub guardian_control_public_key: [u8; 32],
}

impl PublicNodeGuardianIdentity {
    pub fn validate(&self) -> IdentityResult<()> {
        if self.schema != IDENTITY_SCHEMA {
            return Err(IdentityError::IdentityCorrupt);
        }
        validate_trust_domain(&self.trust_domain)?;
        if self.identity_generation == 0 {
            return Err(IdentityError::InvalidIdentityGeneration);
        }
        validate_identifier(&self.node_id, NODE_ID_PREFIX)
            .map_err(|_| IdentityError::NodeIdentityMismatch)?;
        validate_identifier(&self.guardian_id, GUARDIAN_ID_PREFIX)
            .map_err(|_| IdentityError::GuardianIdentityMismatch)?;
        VerifyingKey::from_bytes(&self.node_public_key)
            .map_err(|_| IdentityError::NodeIdentityMismatch)?;
        VerifyingKey::from_bytes(&self.guardian_control_public_key)
            .map_err(|_| IdentityError::GuardianIdentityMismatch)?;
        Ok(())
    }
}

pub struct LocalNodeGuardianIdentity {
    public: PublicNodeGuardianIdentity,
    node_signing_key: SigningKey,
    guardian_control_signing_key: SigningKey,
}

impl fmt::Debug for LocalNodeGuardianIdentity {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("LocalNodeGuardianIdentity")
            .field("public", &self.public)
            .field("private_keys", &"[REDACTED]")
            .finish()
    }
}

impl LocalNodeGuardianIdentity {
    pub fn generate(trust_domain: &str, identity_generation: u64) -> IdentityResult<Self> {
        validate_trust_domain(trust_domain)?;
        if identity_generation == 0 {
            return Err(IdentityError::InvalidIdentityGeneration);
        }
        let mut node_id_bytes = [0_u8; 32];
        let mut guardian_id_bytes = [0_u8; 32];
        let mut node_seed = [0_u8; 32];
        let mut guardian_seed = [0_u8; 32];
        rand::rngs::OsRng.fill_bytes(&mut node_id_bytes);
        rand::rngs::OsRng.fill_bytes(&mut guardian_id_bytes);
        rand::rngs::OsRng.fill_bytes(&mut node_seed);
        rand::rngs::OsRng.fill_bytes(&mut guardian_seed);
        let node_signing_key = SigningKey::from_bytes(&node_seed);
        let guardian_control_signing_key = SigningKey::from_bytes(&guardian_seed);
        Ok(Self {
            public: PublicNodeGuardianIdentity {
                schema: IDENTITY_SCHEMA.to_owned(),
                trust_domain: trust_domain.to_owned(),
                identity_generation,
                node_id: format!("{NODE_ID_PREFIX}{}", hex::encode(node_id_bytes)),
                guardian_id: format!("{GUARDIAN_ID_PREFIX}{}", hex::encode(guardian_id_bytes)),
                node_public_key: node_signing_key.verifying_key().to_bytes(),
                guardian_control_public_key: guardian_control_signing_key
                    .verifying_key()
                    .to_bytes(),
            },
            node_signing_key,
            guardian_control_signing_key,
        })
    }

    pub fn public_identity(&self) -> &PublicNodeGuardianIdentity {
        &self.public
    }

    pub fn sign_enrollment(
        &self,
        role: GuardianEnrollmentRole,
        operator_signing_key: &SigningKey,
        nonce: [u8; NONCE_LEN],
        created_at_unix_secs: u64,
        expires_at_unix_secs: u64,
    ) -> IdentityResult<SignedEnrollmentRequest> {
        let claims = EnrollmentClaims::new(
            self.public.clone(),
            role,
            operator_root_id(&operator_signing_key.verifying_key()),
            nonce,
            created_at_unix_secs,
            expires_at_unix_secs,
        );
        SignedEnrollmentRequest::sign(
            claims,
            &self.node_signing_key,
            &self.guardian_control_signing_key,
            operator_signing_key,
        )
    }

    fn to_stored(&self) -> StoredIdentity {
        StoredIdentity {
            public: self.public.clone(),
            node_signing_seed: self.node_signing_key.to_bytes(),
            guardian_control_signing_seed: self.guardian_control_signing_key.to_bytes(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
struct StoredIdentity {
    public: PublicNodeGuardianIdentity,
    node_signing_seed: [u8; 32],
    guardian_control_signing_seed: [u8; 32],
}

impl StoredIdentity {
    fn restore(self, expected_trust_domain: &str) -> IdentityResult<LocalNodeGuardianIdentity> {
        self.public
            .validate()
            .map_err(|_| IdentityError::IdentityCorrupt)?;
        if self.public.trust_domain != expected_trust_domain {
            return Err(IdentityError::IdentityTrustDomainMismatch);
        }
        let node_signing_key = SigningKey::from_bytes(&self.node_signing_seed);
        let guardian_control_signing_key =
            SigningKey::from_bytes(&self.guardian_control_signing_seed);
        if node_signing_key.verifying_key().to_bytes() != self.public.node_public_key
            || guardian_control_signing_key.verifying_key().to_bytes()
                != self.public.guardian_control_public_key
        {
            return Err(IdentityError::IdentityCorrupt);
        }
        Ok(LocalNodeGuardianIdentity {
            public: self.public,
            node_signing_key,
            guardian_control_signing_key,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentClaims {
    pub schema: String,
    pub operation: String,
    pub identity: PublicNodeGuardianIdentity,
    pub guardian_role: GuardianEnrollmentRole,
    pub operator_root_id: String,
    pub nonce: [u8; NONCE_LEN],
    pub created_at_unix_secs: u64,
    pub expires_at_unix_secs: u64,
}

impl EnrollmentClaims {
    pub fn new(
        identity: PublicNodeGuardianIdentity,
        guardian_role: GuardianEnrollmentRole,
        operator_root_id: String,
        nonce: [u8; NONCE_LEN],
        created_at_unix_secs: u64,
        expires_at_unix_secs: u64,
    ) -> Self {
        Self {
            schema: ENROLLMENT_SCHEMA.to_owned(),
            operation: "enroll".to_owned(),
            identity,
            guardian_role,
            operator_root_id,
            nonce,
            created_at_unix_secs,
            expires_at_unix_secs,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SignedEnrollmentRequest {
    pub claims: EnrollmentClaims,
    pub operator_signature: Vec<u8>,
    pub node_proof_signature: Vec<u8>,
    pub guardian_proof_signature: Vec<u8>,
}

impl SignedEnrollmentRequest {
    pub fn sign(
        claims: EnrollmentClaims,
        node_signing_key: &SigningKey,
        guardian_control_signing_key: &SigningKey,
        operator_signing_key: &SigningKey,
    ) -> IdentityResult<Self> {
        validate_claim_shape(&claims)?;
        if node_signing_key.verifying_key().to_bytes() != claims.identity.node_public_key {
            return Err(IdentityError::NodeIdentityMismatch);
        }
        if guardian_control_signing_key.verifying_key().to_bytes()
            != claims.identity.guardian_control_public_key
        {
            return Err(IdentityError::GuardianIdentityMismatch);
        }
        if operator_root_id(&operator_signing_key.verifying_key()) != claims.operator_root_id {
            return Err(IdentityError::OperatorRootNotApproved);
        }
        Ok(Self {
            operator_signature: operator_signing_key
                .sign(&signing_bytes(OPERATOR_SIGNATURE_DOMAIN, &claims)?)
                .to_bytes()
                .to_vec(),
            node_proof_signature: node_signing_key
                .sign(&signing_bytes(NODE_PROOF_DOMAIN, &claims)?)
                .to_bytes()
                .to_vec(),
            guardian_proof_signature: guardian_control_signing_key
                .sign(&signing_bytes(GUARDIAN_PROOF_DOMAIN, &claims)?)
                .to_bytes()
                .to_vec(),
            claims,
        })
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnrollmentStatus {
    Enrolled,
    Active,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentRecord {
    pub request: SignedEnrollmentRequest,
    pub status: EnrollmentStatus,
    pub enrolled_at_unix_secs: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EnrollmentOutcome {
    Enrolled(EnrollmentRecord),
    AlreadyEnrolled(EnrollmentRecord),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct EnrollmentAuditEvent {
    pub schema: String,
    pub sequence: u64,
    pub occurred_at_unix_secs: u64,
    pub request_sha256: String,
    pub result: String,
    pub reason_code: String,
}

pub struct DistributedIdentityStore {
    database_path: PathBuf,
    database: Database,
    policy: EnrollmentPolicy,
}

impl DistributedIdentityStore {
    pub fn open(database_path: impl AsRef<Path>, policy: EnrollmentPolicy) -> IdentityResult<Self> {
        let database_path = database_path.as_ref();
        if !database_path.is_absolute() {
            return Err(IdentityError::RelativeDatabasePath);
        }
        if let Ok(metadata) = fs::symlink_metadata(database_path) {
            if metadata.file_type().is_symlink() {
                return Err(IdentityError::DatabasePathIsSymlink);
            }
        }
        let parent = database_path
            .parent()
            .ok_or(IdentityError::RelativeDatabasePath)?;
        fs::create_dir_all(parent).map_err(storage_error)?;
        let database = Database::create(database_path).map_err(storage_error)?;
        restrict_database_permissions(database_path)?;
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

    pub fn load_or_create_local_identity(
        &self,
        identity_generation: u64,
    ) -> IdentityResult<LocalNodeGuardianIdentity> {
        if identity_generation == 0 {
            return Err(IdentityError::InvalidIdentityGeneration);
        }
        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        let existing = {
            let table = write.open_table(META).map_err(storage_error)?;
            let existing = table
                .get(LOCAL_IDENTITY_KEY)
                .map_err(storage_error)?
                .map(|value| value.value().to_vec());
            existing
        };
        if let Some(bytes) = existing {
            let stored: StoredIdentity = decode(&bytes, IdentityError::IdentityCorrupt)?;
            let identity = stored.restore(&self.policy.trust_domain)?;
            if identity.public.identity_generation != identity_generation {
                return Err(IdentityError::IdentityGenerationMismatch);
            }
            write.commit().map_err(storage_error)?;
            return Ok(identity);
        }

        let identity =
            LocalNodeGuardianIdentity::generate(&self.policy.trust_domain, identity_generation)?;
        let encoded = encode(&identity.to_stored())?;
        write
            .open_table(META)
            .map_err(storage_error)?
            .insert(LOCAL_IDENTITY_KEY, encoded.as_slice())
            .map_err(storage_error)?;
        write.commit().map_err(storage_error)?;
        Ok(identity)
    }

    pub fn load_local_identity(&self) -> IdentityResult<LocalNodeGuardianIdentity> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let bytes = read
            .open_table(META)
            .map_err(storage_error)?
            .get(LOCAL_IDENTITY_KEY)
            .map_err(storage_error)?
            .map(|value| value.value().to_vec())
            .ok_or(IdentityError::IdentityUnavailable)?;
        let stored: StoredIdentity = decode(&bytes, IdentityError::IdentityCorrupt)?;
        stored.restore(&self.policy.trust_domain)
    }

    pub fn enroll(
        &self,
        request: &SignedEnrollmentRequest,
        now_unix_secs: u64,
    ) -> IdentityResult<EnrollmentOutcome> {
        if let Err(error) = self.validate_request(request, Some(now_unix_secs)) {
            self.record_preflight_rejection(request, now_unix_secs, &error)?;
            return Err(error);
        }

        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        prune_expired_nonces(&write, now_unix_secs)?;

        let nonce_replayed = {
            let nonces = write.open_table(USED_NONCES).map_err(storage_error)?;
            let replayed = nonces
                .get(request.claims.nonce.as_slice())
                .map_err(storage_error)?
                .is_some();
            replayed
        };
        if nonce_replayed {
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::Replay,
            );
        }

        let outstanding_nonce_count = write
            .open_table(USED_NONCES)
            .map_err(storage_error)?
            .len()
            .map_err(storage_error)?;
        if outstanding_nonce_count >= self.policy.max_outstanding_nonces {
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::ResourceExhausted,
            );
        }

        let records = read_enrollments(&write)?;
        if let Some(existing) = records.iter().find(|record| {
            record.request.claims.identity.node_id == request.claims.identity.node_id
        }) {
            if same_identity_generation(&existing.request.claims, &request.claims) {
                consume_nonce(&write, request)?;
                append_audit(
                    &write,
                    &self.policy,
                    request,
                    now_unix_secs,
                    "accepted",
                    "already_enrolled",
                )?;
                write.commit().map_err(storage_error)?;
                return Ok(EnrollmentOutcome::AlreadyEnrolled(existing.clone()));
            }
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::ConflictingNodeIdentity,
            );
        }

        if records.iter().any(|record| {
            record.request.claims.identity.guardian_id == request.claims.identity.guardian_id
        }) {
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::ConflictingGuardianIdentity,
            );
        }

        if request.claims.guardian_role == GuardianEnrollmentRole::Voter
            && records.iter().any(|record| {
                record.request.claims.guardian_role == GuardianEnrollmentRole::Voter
                    && record.request.claims.identity.guardian_control_public_key
                        == request.claims.identity.guardian_control_public_key
            })
        {
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::DuplicateGuardianControlKey,
            );
        }

        if records.len() as u64 >= self.policy.max_enrollments {
            return reject_transaction(
                write,
                &self.policy,
                request,
                now_unix_secs,
                IdentityError::ResourceExhausted,
            );
        }

        let record = EnrollmentRecord {
            request: request.clone(),
            status: EnrollmentStatus::Enrolled,
            enrolled_at_unix_secs: now_unix_secs,
        };
        let encoded = encode(&record)?;
        write
            .open_table(ENROLLMENTS)
            .map_err(storage_error)?
            .insert(request.claims.identity.node_id.as_str(), encoded.as_slice())
            .map_err(storage_error)?;
        consume_nonce(&write, request)?;
        append_audit(
            &write,
            &self.policy,
            request,
            now_unix_secs,
            "accepted",
            "enrolled",
        )?;
        write.commit().map_err(storage_error)?;
        Ok(EnrollmentOutcome::Enrolled(record))
    }

    pub fn enrollment(&self, node_id: &str) -> IdentityResult<Option<EnrollmentRecord>> {
        validate_identifier(node_id, NODE_ID_PREFIX)
            .map_err(|_| IdentityError::NodeIdentityMismatch)?;
        let read = self.database.begin_read().map_err(storage_error)?;
        let record = read
            .open_table(ENROLLMENTS)
            .map_err(storage_error)?
            .get(node_id)
            .map_err(storage_error)?
            .map(|value| value.value().to_vec());
        record
            .map(|bytes| decode(&bytes, IdentityError::DurableStateCorrupt))
            .transpose()
    }

    pub fn enrollment_count(&self) -> IdentityResult<u64> {
        let read = self.database.begin_read().map_err(storage_error)?;
        read.open_table(ENROLLMENTS)
            .map_err(storage_error)?
            .len()
            .map_err(storage_error)
    }

    pub fn audit_events(&self) -> IdentityResult<Vec<EnrollmentAuditEvent>> {
        let read = self.database.begin_read().map_err(storage_error)?;
        let table = read.open_table(AUDIT).map_err(storage_error)?;
        let mut events = Vec::new();
        for row in table.iter().map_err(storage_error)? {
            let (_, value) = row.map_err(storage_error)?;
            events.push(decode(value.value(), IdentityError::DurableStateCorrupt)?);
        }
        Ok(events)
    }

    fn initialize_tables(&self) -> IdentityResult<()> {
        let write = self.database.begin_write().map_err(storage_error)?;
        write.open_table(META).map_err(storage_error)?;
        write.open_table(ENROLLMENTS).map_err(storage_error)?;
        write.open_table(USED_NONCES).map_err(storage_error)?;
        write.open_table(AUDIT).map_err(storage_error)?;
        write.commit().map_err(storage_error)
    }

    fn validate_request(
        &self,
        request: &SignedEnrollmentRequest,
        now_unix_secs: Option<u64>,
    ) -> IdentityResult<()> {
        let encoded = encode(request)?;
        if encoded.len() > self.policy.max_request_bytes {
            return Err(IdentityError::RequestTooLarge);
        }
        validate_claim_shape(&request.claims)?;
        if request.claims.identity.trust_domain != self.policy.trust_domain {
            return Err(IdentityError::WrongTrustDomain);
        }
        let lifetime = request
            .claims
            .expires_at_unix_secs
            .checked_sub(request.claims.created_at_unix_secs)
            .ok_or(IdentityError::RequestLifetimeInvalid)?;
        if lifetime == 0 || lifetime > self.policy.max_request_lifetime_secs {
            return Err(IdentityError::RequestLifetimeInvalid);
        }
        if let Some(now) = now_unix_secs {
            if request.claims.created_at_unix_secs > now {
                return Err(IdentityError::RequestNotYetValid);
            }
            if request.claims.expires_at_unix_secs <= now {
                return Err(IdentityError::RequestExpired);
            }
        }

        let root_bytes = self
            .policy
            .approved_operator_roots
            .get(&request.claims.operator_root_id)
            .ok_or(IdentityError::OperatorRootNotApproved)?;
        let operator_key = VerifyingKey::from_bytes(root_bytes)
            .map_err(|_| IdentityError::OperatorRootNotApproved)?;
        verify_signature(
            &operator_key,
            &signing_bytes(OPERATOR_SIGNATURE_DOMAIN, &request.claims)?,
            &request.operator_signature,
            IdentityError::InvalidOperatorSignature,
        )?;
        let node_key = VerifyingKey::from_bytes(&request.claims.identity.node_public_key)
            .map_err(|_| IdentityError::NodeIdentityMismatch)?;
        verify_signature(
            &node_key,
            &signing_bytes(NODE_PROOF_DOMAIN, &request.claims)?,
            &request.node_proof_signature,
            IdentityError::InvalidNodeProof,
        )?;
        let guardian_key =
            VerifyingKey::from_bytes(&request.claims.identity.guardian_control_public_key)
                .map_err(|_| IdentityError::GuardianIdentityMismatch)?;
        verify_signature(
            &guardian_key,
            &signing_bytes(GUARDIAN_PROOF_DOMAIN, &request.claims)?,
            &request.guardian_proof_signature,
            IdentityError::InvalidGuardianProof,
        )?;
        Ok(())
    }

    fn validate_durable_state(&self) -> IdentityResult<()> {
        let read = self.database.begin_read().map_err(storage_error)?;
        if let Some(bytes) = read
            .open_table(META)
            .map_err(storage_error)?
            .get(LOCAL_IDENTITY_KEY)
            .map_err(storage_error)?
            .map(|value| value.value().to_vec())
        {
            let stored: StoredIdentity = decode(&bytes, IdentityError::IdentityCorrupt)?;
            stored.restore(&self.policy.trust_domain)?;
        }

        let table = read.open_table(ENROLLMENTS).map_err(storage_error)?;
        if table.len().map_err(storage_error)? > self.policy.max_enrollments {
            return Err(IdentityError::DurableStateCorrupt);
        }
        let mut node_ids = BTreeSet::new();
        let mut guardian_ids = BTreeSet::new();
        let mut voter_keys = BTreeSet::new();
        for row in table.iter().map_err(storage_error)? {
            let (key, value) = row.map_err(storage_error)?;
            let record: EnrollmentRecord =
                decode(value.value(), IdentityError::DurableStateCorrupt)?;
            self.validate_request(&record.request, None)
                .map_err(|_| IdentityError::DurableStateCorrupt)?;
            let identity = &record.request.claims.identity;
            if key.value() != identity.node_id
                || !node_ids.insert(identity.node_id.clone())
                || !guardian_ids.insert(identity.guardian_id.clone())
            {
                return Err(IdentityError::DurableStateCorrupt);
            }
            if record.request.claims.guardian_role == GuardianEnrollmentRole::Voter
                && !voter_keys.insert(identity.guardian_control_public_key)
            {
                return Err(IdentityError::DurableStateCorrupt);
            }
        }
        if read
            .open_table(USED_NONCES)
            .map_err(storage_error)?
            .len()
            .map_err(storage_error)?
            > self.policy.max_outstanding_nonces
            || read
                .open_table(AUDIT)
                .map_err(storage_error)?
                .len()
                .map_err(storage_error)?
                > self.policy.max_audit_events
        {
            return Err(IdentityError::DurableStateCorrupt);
        }
        Ok(())
    }

    fn record_preflight_rejection(
        &self,
        request: &SignedEnrollmentRequest,
        now_unix_secs: u64,
        error: &IdentityError,
    ) -> IdentityResult<()> {
        let mut write = self.database.begin_write().map_err(storage_error)?;
        write
            .set_durability(Durability::Immediate)
            .map_err(storage_error)?;
        append_audit(
            &write,
            &self.policy,
            request,
            now_unix_secs,
            "rejected",
            error.code(),
        )?;
        write.commit().map_err(storage_error)
    }
}

fn validate_claim_shape(claims: &EnrollmentClaims) -> IdentityResult<()> {
    if claims.schema != ENROLLMENT_SCHEMA || claims.operation != "enroll" {
        return Err(IdentityError::DurableStateCorrupt);
    }
    claims.identity.validate()?;
    validate_identifier(&claims.operator_root_id, ROOT_ID_PREFIX)
        .map_err(|_| IdentityError::OperatorRootNotApproved)?;
    if claims.nonce == [0_u8; NONCE_LEN] {
        return Err(IdentityError::InvalidNonce);
    }
    Ok(())
}

fn validate_trust_domain(value: &str) -> IdentityResult<()> {
    if value.is_empty()
        || value.len() > 128
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-'))
    {
        return Err(IdentityError::InvalidTrustDomain);
    }
    Ok(())
}

fn validate_identifier(value: &str, prefix: &str) -> IdentityResult<()> {
    let Some(suffix) = value.strip_prefix(prefix) else {
        return Err(IdentityError::IdentityCorrupt);
    };
    if suffix.len() != IDENTIFIER_HEX_LEN
        || !suffix
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
    {
        return Err(IdentityError::IdentityCorrupt);
    }
    Ok(())
}

pub fn operator_root_id(key: &VerifyingKey) -> String {
    format!(
        "{ROOT_ID_PREFIX}{}",
        hex::encode(Sha256::digest(key.as_bytes()))
    )
}

fn signing_bytes(domain: &[u8], claims: &EnrollmentClaims) -> IdentityResult<Vec<u8>> {
    let claims = serde_jcs::to_vec(claims).map_err(encoding_error)?;
    let mut bytes = Vec::with_capacity(domain.len() + claims.len());
    bytes.extend_from_slice(domain);
    bytes.extend_from_slice(&claims);
    Ok(bytes)
}

fn verify_signature(
    key: &VerifyingKey,
    message: &[u8],
    bytes: &[u8],
    invalid_error: IdentityError,
) -> IdentityResult<()> {
    if bytes.len() != SIGNATURE_LEN {
        return Err(IdentityError::MalformedSignature);
    }
    let signature = Signature::from_slice(bytes).map_err(|_| IdentityError::MalformedSignature)?;
    key.verify_strict(message, &signature)
        .map_err(|_| invalid_error)
}

fn same_identity_generation(left: &EnrollmentClaims, right: &EnrollmentClaims) -> bool {
    left.identity == right.identity && left.guardian_role == right.guardian_role
}

fn read_enrollments(write: &redb::WriteTransaction) -> IdentityResult<Vec<EnrollmentRecord>> {
    let table = write.open_table(ENROLLMENTS).map_err(storage_error)?;
    let mut records = Vec::new();
    for row in table.iter().map_err(storage_error)? {
        let (_, value) = row.map_err(storage_error)?;
        records.push(decode(value.value(), IdentityError::DurableStateCorrupt)?);
    }
    Ok(records)
}

fn consume_nonce(
    write: &redb::WriteTransaction,
    request: &SignedEnrollmentRequest,
) -> IdentityResult<()> {
    write
        .open_table(USED_NONCES)
        .map_err(storage_error)?
        .insert(
            request.claims.nonce.as_slice(),
            request.claims.expires_at_unix_secs,
        )
        .map_err(storage_error)?;
    Ok(())
}

fn prune_expired_nonces(write: &redb::WriteTransaction, now_unix_secs: u64) -> IdentityResult<()> {
    let mut table = write.open_table(USED_NONCES).map_err(storage_error)?;
    let expired = table
        .iter()
        .map_err(storage_error)?
        .filter_map(|row| match row {
            Ok((key, value)) if value.value() <= now_unix_secs => Some(Ok(key.value().to_vec())),
            Ok(_) => None,
            Err(error) => Some(Err(storage_error(error))),
        })
        .collect::<IdentityResult<Vec<_>>>()?;
    for nonce in expired {
        table.remove(nonce.as_slice()).map_err(storage_error)?;
    }
    Ok(())
}

fn reject_transaction(
    write: redb::WriteTransaction,
    policy: &EnrollmentPolicy,
    request: &SignedEnrollmentRequest,
    now_unix_secs: u64,
    error: IdentityError,
) -> IdentityResult<EnrollmentOutcome> {
    append_audit(
        &write,
        policy,
        request,
        now_unix_secs,
        "rejected",
        error.code(),
    )?;
    write.commit().map_err(storage_error)?;
    Err(error)
}

fn append_audit(
    write: &redb::WriteTransaction,
    policy: &EnrollmentPolicy,
    request: &SignedEnrollmentRequest,
    now_unix_secs: u64,
    result: &str,
    reason_code: &str,
) -> IdentityResult<()> {
    let sequence = {
        let mut meta = write.open_table(META).map_err(storage_error)?;
        let sequence = meta
            .get(NEXT_AUDIT_SEQUENCE_KEY)
            .map_err(storage_error)?
            .map_or(1, |value| decode_u64(value.value()).unwrap_or(1));
        let next = sequence
            .checked_add(1)
            .ok_or(IdentityError::ResourceExhausted)?;
        let encoded = next.to_be_bytes();
        meta.insert(NEXT_AUDIT_SEQUENCE_KEY, encoded.as_slice())
            .map_err(storage_error)?;
        sequence
    };
    let request_sha256 = hex::encode(Sha256::digest(encode(request)?));
    let event = EnrollmentAuditEvent {
        schema: AUDIT_SCHEMA.to_owned(),
        sequence,
        occurred_at_unix_secs: now_unix_secs,
        request_sha256,
        result: result.to_owned(),
        reason_code: reason_code.to_owned(),
    };
    let encoded = encode(&event)?;
    let mut audit = write.open_table(AUDIT).map_err(storage_error)?;
    audit
        .insert(sequence, encoded.as_slice())
        .map_err(storage_error)?;
    while audit.len().map_err(storage_error)? > policy.max_audit_events {
        let oldest = audit
            .first()
            .map_err(storage_error)?
            .map(|(key, _)| key.value())
            .ok_or(IdentityError::DurableStateCorrupt)?;
        audit.remove(oldest).map_err(storage_error)?;
    }
    Ok(())
}

fn decode_u64(bytes: &[u8]) -> Option<u64> {
    let bytes: [u8; 8] = bytes.try_into().ok()?;
    Some(u64::from_be_bytes(bytes))
}

fn encode<T: Serialize>(value: &T) -> IdentityResult<Vec<u8>> {
    serde_json::to_vec(value).map_err(encoding_error)
}

fn decode<T: for<'de> Deserialize<'de>>(bytes: &[u8], corrupt: IdentityError) -> IdentityResult<T> {
    serde_json::from_slice(bytes).map_err(|_| corrupt)
}

fn storage_error(error: impl fmt::Display) -> IdentityError {
    IdentityError::Storage(error.to_string())
}

fn encoding_error(error: impl fmt::Display) -> IdentityError {
    IdentityError::Encoding(error.to_string())
}

fn restrict_database_permissions(path: &Path) -> IdentityResult<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let mut permissions = fs::metadata(path).map_err(storage_error)?.permissions();
        if permissions.mode() & 0o077 != 0 {
            permissions.set_mode(0o600);
            fs::set_permissions(path, permissions).map_err(storage_error)?;
        }
    }
    Ok(())
}
