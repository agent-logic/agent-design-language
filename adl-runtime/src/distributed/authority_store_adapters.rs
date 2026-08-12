//! Authority-bound adapters for existing distributed certificate, lease, and
//! fencing stores.
//!
//! Raw stores remain independently durable, but this module is the production
//! serving surface for #203: callers obtain handles only through a registry
//! backed by the published #200 reconciliation barrier, and every exposed
//! operation revalidates the current lineage permit before touching the store.

use std::sync::{Arc, Mutex};

use super::{
    authority_reconciliation::{
        AuthorityPermitAction, AuthorityReconciliationBarrier, AuthorityReconciliationError,
    },
    certificates::{
        ActivationOutcome, AuthorityCertificate, CertificateError, CertificatePurpose,
        DistributedCertificateStore, RevocationReason, VerifiedCertificate,
    },
    fencing::{
        ActiveLeaseCheck, FenceCommit, FenceReceipt, FencingError, FencingStore,
        RedactedFencingSnapshot,
    },
    lease::{
        AuthorityApplication, AuthorityError, AuthorityLedger, AuthorityMembership,
        LeaseAuthorityRevision, LeaseState, MutationAuthorization, RedactedLeaseSnapshot,
    },
    transport::RuntimeCertificateAuthority,
};

const CERTIFICATE_ACTIVATE: &str = "certificate_activate";
const CERTIFICATE_REVOKE: &str = "certificate_revoke";
const LEASE_APPLY: &str = "lease_apply";
const LEASE_MUTATION: &str = "lease_mutation";
const FENCING_COMMIT: &str = "fencing_commit";
const FENCING_ACTIVE_LEASE: &str = "fencing_active_lease";

pub type AuthorityStoreAdapterResult<T> = Result<T, AuthorityStoreAdapterError>;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AuthorityStoreAdapterError {
    Reconciliation(AuthorityReconciliationError),
    Certificate(CertificateError),
    Lease(AuthorityError),
    Fencing(FencingError),
    LockPoisoned,
}

impl From<AuthorityReconciliationError> for AuthorityStoreAdapterError {
    fn from(error: AuthorityReconciliationError) -> Self {
        Self::Reconciliation(error)
    }
}

impl From<CertificateError> for AuthorityStoreAdapterError {
    fn from(error: CertificateError) -> Self {
        Self::Certificate(error)
    }
}

impl From<AuthorityError> for AuthorityStoreAdapterError {
    fn from(error: AuthorityError) -> Self {
        Self::Lease(error)
    }
}

impl From<FencingError> for AuthorityStoreAdapterError {
    fn from(error: FencingError) -> Self {
        Self::Fencing(error)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedStoreAuthorityReceiptView {
    lineage_id: String,
    operation_id: String,
    generation: u64,
    result_sha256: [u8; 32],
}

impl PublishedStoreAuthorityReceiptView {
    pub fn lineage_id(&self) -> &str {
        &self.lineage_id
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn result_sha256(&self) -> [u8; 32] {
        self.result_sha256
    }
}

#[derive(Clone)]
pub struct AuthorityStoreAdapterRegistry {
    barrier: Arc<AuthorityReconciliationBarrier>,
}

impl AuthorityStoreAdapterRegistry {
    pub fn new(barrier: Arc<AuthorityReconciliationBarrier>) -> Self {
        Self { barrier }
    }

    pub fn published_view(
        &self,
        lineage_id: &str,
    ) -> AuthorityStoreAdapterResult<PublishedStoreAuthorityReceiptView> {
        let permit = self.barrier.read_permit(lineage_id)?;
        self.barrier
            .validate_permit(&permit, &AuthorityPermitAction::Read)?;
        let result = self
            .barrier
            .published_result(lineage_id)
            .ok_or(AuthorityReconciliationError::ReconciliationRequired)?;
        Ok(PublishedStoreAuthorityReceiptView {
            lineage_id: result.lineage_id().to_owned(),
            operation_id: result.operation_id().to_owned(),
            generation: result.generation(),
            result_sha256: result.result_sha256(),
        })
    }

    pub fn certificate_store(
        &self,
        lineage_id: impl Into<String>,
        store: Arc<DistributedCertificateStore>,
    ) -> AuthorityStoreAdapterResult<AuthorityBoundCertificateStore> {
        let lineage_id = lineage_id.into();
        self.require_read(&lineage_id)?;
        Ok(AuthorityBoundCertificateStore {
            lineage_id,
            barrier: Arc::clone(&self.barrier),
            store,
        })
    }

    pub fn lease_ledger(
        &self,
        lineage_id: impl Into<String>,
        ledger: Arc<Mutex<AuthorityLedger>>,
    ) -> AuthorityStoreAdapterResult<AuthorityBoundLeaseLedger> {
        let lineage_id = lineage_id.into();
        self.require_read(&lineage_id)?;
        Ok(AuthorityBoundLeaseLedger {
            lineage_id,
            barrier: Arc::clone(&self.barrier),
            ledger,
        })
    }

    pub fn fencing_store(
        &self,
        lineage_id: impl Into<String>,
        store: Arc<Mutex<FencingStore>>,
    ) -> AuthorityStoreAdapterResult<AuthorityBoundFencingStore> {
        let lineage_id = lineage_id.into();
        self.require_read(&lineage_id)?;
        Ok(AuthorityBoundFencingStore {
            lineage_id,
            barrier: Arc::clone(&self.barrier),
            store,
        })
    }

    fn require_read(&self, lineage_id: &str) -> AuthorityStoreAdapterResult<()> {
        let permit = self.barrier.read_permit(lineage_id)?;
        self.barrier
            .validate_permit(&permit, &AuthorityPermitAction::Read)?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AuthorityBoundCertificateStore {
    lineage_id: String,
    barrier: Arc<AuthorityReconciliationBarrier>,
    store: Arc<DistributedCertificateStore>,
}

impl AuthorityBoundCertificateStore {
    pub fn authorize(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> AuthorityStoreAdapterResult<VerifiedCertificate> {
        self.require_read()?;
        self.store
            .authorize(holder_id, purpose, generation, now_unix_secs)
            .map_err(Into::into)
    }

    pub fn activate(
        &self,
        certificate: &AuthorityCertificate,
        now_unix_secs: u64,
    ) -> AuthorityStoreAdapterResult<ActivationOutcome> {
        self.require_mutation(CERTIFICATE_ACTIVATE)?;
        self.store
            .activate(certificate, now_unix_secs)
            .map_err(Into::into)
    }

    pub fn revoke(
        &self,
        certificate_id: &str,
        now_unix_secs: u64,
        reason: RevocationReason,
    ) -> AuthorityStoreAdapterResult<()> {
        self.require_mutation(CERTIFICATE_REVOKE)?;
        self.store
            .revoke(certificate_id, now_unix_secs, reason)
            .map_err(Into::into)
    }

    fn require_read(&self) -> AuthorityStoreAdapterResult<()> {
        let permit = self.barrier.read_permit(&self.lineage_id)?;
        self.barrier
            .validate_permit(&permit, &AuthorityPermitAction::Read)?;
        Ok(())
    }

    fn require_mutation(&self, mutation_kind: &str) -> AuthorityStoreAdapterResult<()> {
        let permit = self
            .barrier
            .mutation_permit(&self.lineage_id, mutation_kind)?;
        self.barrier.validate_permit(
            &permit,
            &AuthorityPermitAction::Mutation(mutation_kind.to_owned()),
        )?;
        Ok(())
    }
}

impl RuntimeCertificateAuthority for AuthorityBoundCertificateStore {
    fn authorize_runtime_certificate(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_seconds: u64,
    ) -> Result<VerifiedCertificate, ()> {
        self.authorize(holder_id, purpose, generation, now_unix_seconds)
            .map_err(|_| ())
    }
}

#[derive(Clone)]
pub struct AuthorityBoundLeaseLedger {
    lineage_id: String,
    barrier: Arc<AuthorityReconciliationBarrier>,
    ledger: Arc<Mutex<AuthorityLedger>>,
}

impl AuthorityBoundLeaseLedger {
    pub fn authority_revision(&self) -> AuthorityStoreAdapterResult<LeaseAuthorityRevision> {
        self.require_read()?;
        self.ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .authority_revision()
            .map_err(Into::into)
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: LeaseAuthorityRevision,
        membership: &AuthorityMembership,
        now_elapsed_millis: u64,
    ) -> AuthorityStoreAdapterResult<RedactedLeaseSnapshot> {
        self.require_read()?;
        self.ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .redacted_snapshot_at(expected_revision, membership, now_elapsed_millis)
            .map_err(Into::into)
    }

    pub fn apply(
        &self,
        certificate_bytes: &[u8],
        membership: &AuthorityMembership,
        application: AuthorityApplication<'_>,
    ) -> AuthorityStoreAdapterResult<LeaseState> {
        self.require_mutation(LEASE_APPLY)?;
        let lease = self
            .ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .apply(certificate_bytes, membership, application)?
            .clone();
        Ok(lease)
    }

    pub fn authorize_mutation(
        &self,
        authorization: MutationAuthorization<'_>,
    ) -> AuthorityStoreAdapterResult<()> {
        self.require_mutation(LEASE_MUTATION)?;
        self.ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .authorize_mutation(authorization)
            .map_err(Into::into)
    }

    fn require_read(&self) -> AuthorityStoreAdapterResult<()> {
        let permit = self.barrier.read_permit(&self.lineage_id)?;
        self.barrier
            .validate_permit(&permit, &AuthorityPermitAction::Read)?;
        Ok(())
    }

    fn require_mutation(&self, mutation_kind: &str) -> AuthorityStoreAdapterResult<()> {
        let permit = self
            .barrier
            .mutation_permit(&self.lineage_id, mutation_kind)?;
        self.barrier.validate_permit(
            &permit,
            &AuthorityPermitAction::Mutation(mutation_kind.to_owned()),
        )?;
        Ok(())
    }
}

#[derive(Clone)]
pub struct AuthorityBoundFencingStore {
    lineage_id: String,
    barrier: Arc<AuthorityReconciliationBarrier>,
    store: Arc<Mutex<FencingStore>>,
}

impl AuthorityBoundFencingStore {
    pub fn commit(&self, request: FenceCommit<'_>) -> AuthorityStoreAdapterResult<FenceReceipt> {
        self.require_mutation(FENCING_COMMIT)?;
        self.store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .commit(request)
            .map_err(Into::into)
    }

    pub fn authorize_active_lease(
        &self,
        check: ActiveLeaseCheck<'_>,
    ) -> AuthorityStoreAdapterResult<()> {
        self.require_mutation(FENCING_ACTIVE_LEASE)?;
        self.store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .authorize_active_lease(check)
            .map_err(Into::into)
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: super::fencing::FencingAuthorityRevision,
        membership: &AuthorityMembership,
    ) -> AuthorityStoreAdapterResult<RedactedFencingSnapshot> {
        self.require_read()?;
        self.store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .redacted_snapshot_at(expected_revision, membership)
            .map_err(Into::into)
    }

    fn require_read(&self) -> AuthorityStoreAdapterResult<()> {
        let permit = self.barrier.read_permit(&self.lineage_id)?;
        self.barrier
            .validate_permit(&permit, &AuthorityPermitAction::Read)?;
        Ok(())
    }

    fn require_mutation(&self, mutation_kind: &str) -> AuthorityStoreAdapterResult<()> {
        let permit = self
            .barrier
            .mutation_permit(&self.lineage_id, mutation_kind)?;
        self.barrier.validate_permit(
            &permit,
            &AuthorityPermitAction::Mutation(mutation_kind.to_owned()),
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        path::Path,
        sync::{Arc, Mutex},
    };

    use ed25519_dalek::SigningKey;

    use super::*;
    use crate::distributed::{
        authority_protocol::{
            test_published_reconciliation_token, AuthorityNodeIdentity, CanonicalAuthorityTime,
        },
        authority_reconciliation::{
            AuthorityReconciliationArtifact, AuthorityReconciliationIdentity,
        },
        certificates::{CertificateBody, CertificatePolicy, CertificateValidity},
        polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
    };

    const DOMAIN: &str = "runtime-prod";
    const NOW: u64 = 1_900_000_000;

    #[derive(Default)]
    struct MemoryCheckpoint {
        values: Mutex<BTreeMap<String, ConsensusCheckpoint>>,
    }

    impl ConsensusCheckpointAuthority for MemoryCheckpoint {
        fn load(&self, object: &str) -> Result<Option<ConsensusCheckpoint>, PolisRuntimeError> {
            Ok(self.values.lock().unwrap().get(object).cloned())
        }

        fn compare_and_swap(
            &self,
            expected: Option<&ConsensusCheckpoint>,
            candidate: &ConsensusCheckpoint,
        ) -> Result<(), PolisRuntimeError> {
            let mut values = self.values.lock().unwrap();
            if values.get(&candidate.object) != expected {
                return Err(PolisRuntimeError::StateRegression);
            }
            values.insert(candidate.object.clone(), candidate.clone());
            Ok(())
        }
    }

    fn reconciliation_identity() -> AuthorityReconciliationIdentity {
        AuthorityReconciliationIdentity {
            trust_domain: DOMAIN.to_owned(),
            polis_id: "polis-a".to_owned(),
            node_id: "node-a".to_owned(),
            guardian_id: "guardian-a".to_owned(),
            boot_generation: 7,
            protocol_instance: "adl.authority-reconciliation.v1".to_owned(),
        }
    }

    fn authority_node_identity() -> AuthorityNodeIdentity {
        AuthorityNodeIdentity {
            trust_domain: DOMAIN.to_owned(),
            polis_id: "polis-a".to_owned(),
            node_id: "node-a".to_owned(),
            guardian_id: "guardian-a".to_owned(),
            boot_generation: 7,
        }
    }

    fn publish_lineage(
        root: &Path,
        lineage_id: &str,
        mutation_kind: &str,
    ) -> Arc<AuthorityReconciliationBarrier> {
        let mut barrier = AuthorityReconciliationBarrier::open(
            root,
            reconciliation_identity(),
            Arc::new(MemoryCheckpoint::default()),
        )
        .unwrap();
        let artifact = AuthorityReconciliationArtifact::new(
            lineage_id.to_owned(),
            "adl.test.deterministic-authority".to_owned(),
            1,
            mutation_kind.to_owned(),
            vec![b"step-0".to_vec()],
            b"published-store-authority".to_vec(),
            2_000_000_000,
        )
        .unwrap();
        let token = test_published_reconciliation_token(
            authority_node_identity(),
            "issue-203-operation",
            artifact.committed_artifact().unwrap(),
            200,
            CanonicalAuthorityTime {
                unix_seconds: NOW as i64,
                nanos: 17,
                uncertainty_millis: 25,
            },
        );
        barrier.reconcile(&token).unwrap();
        Arc::new(barrier)
    }

    fn certificate(
        root: &SigningKey,
        holder: &str,
        purpose: CertificatePurpose,
        generation: u64,
        subject: &SigningKey,
    ) -> AuthorityCertificate {
        AuthorityCertificate::issue(
            CertificateBody::new(
                DOMAIN,
                holder,
                purpose,
                generation,
                CertificateValidity {
                    issued_at_unix_secs: NOW - 10,
                    expires_at_unix_secs: NOW + 300,
                },
                subject.verifying_key(),
                &root.verifying_key(),
            ),
            root,
        )
        .unwrap()
    }

    #[test]
    fn published_certificate_adapter_authorizes_through_200_barrier() {
        let root = std::env::current_dir()
            .and_then(std::fs::canonicalize)
            .unwrap();
        let temp = tempfile::TempDir::new_in(root).unwrap();
        let signing_root = SigningKey::from_bytes(&[41; 32]);
        let subject = SigningKey::from_bytes(&[42; 32]);
        let policy = CertificatePolicy::new(DOMAIN, [signing_root.verifying_key()]).unwrap();
        let raw = Arc::new(
            DistributedCertificateStore::open(temp.path().join("certificates.redb"), policy)
                .unwrap(),
        );
        let certificate = certificate(
            &signing_root,
            "node-a",
            CertificatePurpose::Transport,
            1,
            &subject,
        );
        raw.activate(&certificate, NOW).unwrap();

        let registry = AuthorityStoreAdapterRegistry::new(publish_lineage(
            temp.path(),
            "lineage-a",
            "certificate_activate",
        ));
        let bound = registry
            .certificate_store("lineage-a", Arc::clone(&raw))
            .unwrap();
        let verified = bound
            .authorize("node-a", CertificatePurpose::Transport, 1, NOW + 1)
            .unwrap();

        assert_eq!(verified.holder_id, "node-a");
        assert_eq!(verified.purpose, CertificatePurpose::Transport);
    }
}
