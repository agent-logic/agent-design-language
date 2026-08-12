//! Authority-bound adapters for existing distributed certificate, lease, and
//! fencing stores.
//!
//! Raw stores remain independently durable, but this module is the production
//! serving surface for #203: callers obtain handles only through a registry
//! backed by the published #200 reconciliation barrier, and every exposed
//! operation revalidates the current lineage permit before touching the store.

use std::sync::{Arc, Mutex};

#[cfg(debug_assertions)]
use std::path::Path;

#[cfg(debug_assertions)]
use super::authority_protocol::{
    test_published_reconciliation_token, AuthorityNodeIdentity, CanonicalAuthorityTime,
};
#[cfg(debug_assertions)]
use super::authority_reconciliation::{
    AuthorityReconciliationArtifact, AuthorityReconciliationIdentity,
};
#[cfg(debug_assertions)]
use super::polis_runtime::ConsensusCheckpointAuthority;
use super::{
    authority_reconciliation::{
        AuthorityPermitAction, AuthorityReconciliationBarrier, AuthorityReconciliationError,
    },
    certificates::{
        ActivationOutcome, AuthorityCertificate, CertificateAuthorityRevision, CertificateError,
        CertificatePurpose, DistributedCertificateStore, RedactedCertificateSnapshot,
        RevocationReason, VerifiedCertificate, AUTHORITY_BOUND_CERTIFICATE_ACCESS,
    },
    fencing::{
        ActiveLeaseCheck, FenceCommit, FenceReceipt, FencingAuthorityRevision, FencingError,
        FencingStore, RedactedFencingSnapshot, AUTHORITY_BOUND_FENCING_ACCESS,
    },
    lease::{
        AuthorityApplication, AuthorityError, AuthorityLedger, AuthorityMembership,
        LeaseAuthorityRevision, LeaseState, MutationAuthorization, RedactedLeaseSnapshot,
        AUTHORITY_BOUND_LEASE_ACCESS,
    },
};

const CERTIFICATE_ACTIVATE: &str = "certificate_activate";
const CERTIFICATE_REVOKE: &str = "certificate_revoke";
const LEASE_APPLY: &str = "lease_apply";
const LEASE_MUTATION: &str = "lease_mutation";
const FENCING_COMMIT: &str = "fencing_commit";
const FENCING_ACTIVE_LEASE: &str = "fencing_active_lease";
const PUBLISHED_VIEW_ACTION_FENCE: &str = "fence";
const PUBLISHED_VIEW_ACTION_OWNER_COMMIT: &str = "owner_commit";
#[cfg(debug_assertions)]
const TEST_ADAPTER_KIND: &str = "adl.test.deterministic-authority";
#[cfg(debug_assertions)]
const TEST_ADAPTER_VERSION: u32 = 1;

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
    action_class: String,
    adapter_kind: String,
    adapter_version: u32,
    generation: u64,
    receipt_sha256: [u8; 32],
    result_sha256: [u8; 32],
}

impl PublishedStoreAuthorityReceiptView {
    pub fn lineage_id(&self) -> &str {
        &self.lineage_id
    }

    pub fn operation_id(&self) -> &str {
        &self.operation_id
    }

    pub fn action_class(&self) -> &str {
        &self.action_class
    }

    pub fn adapter_kind(&self) -> &str {
        &self.adapter_kind
    }

    pub fn adapter_version(&self) -> u32 {
        self.adapter_version
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn receipt_sha256(&self) -> [u8; 32] {
        self.receipt_sha256
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
        let action_class = published_view_action_class(result.mutation_kind())
            .ok_or(AuthorityReconciliationError::ReconciliationRequired)?;
        Ok(PublishedStoreAuthorityReceiptView {
            lineage_id: result.lineage_id().to_owned(),
            operation_id: result.operation_id().to_owned(),
            action_class: action_class.to_owned(),
            adapter_kind: result.adapter_kind().to_owned(),
            adapter_version: result.adapter_version(),
            generation: result.generation(),
            receipt_sha256: result.receipts_sha256(),
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

fn published_view_action_class(mutation_kind: &str) -> Option<&'static str> {
    match mutation_kind {
        CERTIFICATE_ACTIVATE | CERTIFICATE_REVOKE | LEASE_APPLY | LEASE_MUTATION => {
            Some(PUBLISHED_VIEW_ACTION_OWNER_COMMIT)
        }
        FENCING_COMMIT | FENCING_ACTIVE_LEASE => Some(PUBLISHED_VIEW_ACTION_FENCE),
        _ => None,
    }
}

/// Debug/focused-proof fixture seam for external integration tests.
///
/// The helper never returns a raw certificate authority handle: it publishes a
/// deterministic reconciliation result through the same barrier that production
/// adapters use, then returns the sealed authority-bound certificate store.
#[cfg(debug_assertions)]
#[doc(hidden)]
pub fn authority_bound_certificate_store_for_test_fixture(
    root: &Path,
    identity: AuthorityReconciliationIdentity,
    checkpoint_authority: Arc<dyn ConsensusCheckpointAuthority>,
    lineage_id: &str,
    store: Arc<DistributedCertificateStore>,
) -> AuthorityStoreAdapterResult<AuthorityBoundCertificateStore> {
    let artifact = AuthorityReconciliationArtifact::new(
        lineage_id.to_owned(),
        TEST_ADAPTER_KIND.to_owned(),
        TEST_ADAPTER_VERSION,
        CERTIFICATE_ACTIVATE.to_owned(),
        vec![b"authority-bound-certificate-fixture-step".to_vec()],
        b"authority-bound-certificate-fixture-result".to_vec(),
        2_000_000_000,
    )?;
    let committed = artifact.committed_artifact().map_err(|_| {
        AuthorityStoreAdapterError::Reconciliation(AuthorityReconciliationError::InvalidArtifact)
    })?;
    let authority_identity = AuthorityNodeIdentity {
        trust_domain: identity.trust_domain.clone(),
        polis_id: identity.polis_id.clone(),
        node_id: identity.node_id.clone(),
        guardian_id: identity.guardian_id.clone(),
        boot_generation: identity.boot_generation,
    };
    let operation_id = format!("authority-bound-certificate-fixture-{lineage_id}");
    let token = test_published_reconciliation_token(
        authority_identity,
        &operation_id,
        committed,
        1,
        CanonicalAuthorityTime {
            unix_seconds: 1_900_000_000,
            nanos: 0,
            uncertainty_millis: 0,
        },
    );
    let barrier_root = root.join(format!("authority-reconciliation-barrier-{lineage_id}"));
    std::fs::create_dir_all(&barrier_root).map_err(|_| {
        AuthorityStoreAdapterError::Reconciliation(AuthorityReconciliationError::Storage)
    })?;
    let mut barrier =
        AuthorityReconciliationBarrier::open(&barrier_root, identity, checkpoint_authority)?;
    barrier.reconcile(&token)?;
    AuthorityStoreAdapterRegistry::new(Arc::new(barrier)).certificate_store(lineage_id, store)
}

#[derive(Clone)]
pub struct AuthorityBoundCertificateStore {
    lineage_id: String,
    barrier: Arc<AuthorityReconciliationBarrier>,
    store: Arc<DistributedCertificateStore>,
}

impl AuthorityBoundCertificateStore {
    pub fn authority_revision(&self) -> AuthorityStoreAdapterResult<CertificateAuthorityRevision> {
        self.require_read()?;
        self.store.authority_revision().map_err(Into::into)
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: CertificateAuthorityRevision,
        now_unix_secs: u64,
    ) -> AuthorityStoreAdapterResult<RedactedCertificateSnapshot> {
        self.require_read()?;
        self.store
            .redacted_snapshot_at(expected_revision, now_unix_secs)
            .map_err(Into::into)
    }

    pub fn authorize(
        &self,
        holder_id: &str,
        purpose: CertificatePurpose,
        generation: u64,
        now_unix_secs: u64,
    ) -> AuthorityStoreAdapterResult<VerifiedCertificate> {
        self.require_read()?;
        self.store
            .authorize(
                &AUTHORITY_BOUND_CERTIFICATE_ACCESS,
                holder_id,
                purpose,
                generation,
                now_unix_secs,
            )
            .map_err(Into::into)
    }

    pub fn activate(
        &self,
        certificate: &AuthorityCertificate,
        now_unix_secs: u64,
    ) -> AuthorityStoreAdapterResult<ActivationOutcome> {
        self.require_mutation(CERTIFICATE_ACTIVATE)?;
        self.store
            .activate(
                &AUTHORITY_BOUND_CERTIFICATE_ACCESS,
                certificate,
                now_unix_secs,
            )
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
            .revoke(
                &AUTHORITY_BOUND_CERTIFICATE_ACCESS,
                certificate_id,
                now_unix_secs,
                reason,
            )
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

    pub fn snapshot(&self) -> AuthorityStoreAdapterResult<Vec<u8>> {
        self.require_read()?;
        self.ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .snapshot()
            .map_err(Into::into)
    }

    pub fn applied_log_index(&self) -> AuthorityStoreAdapterResult<u64> {
        self.require_read()?;
        Ok(self
            .ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .applied_log_index())
    }

    pub fn lease(&self, lineage_id: &[u8]) -> AuthorityStoreAdapterResult<Option<LeaseState>> {
        self.require_read()?;
        Ok(self
            .ledger
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .lease(lineage_id)
            .cloned())
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
            .apply(
                &AUTHORITY_BOUND_LEASE_ACCESS,
                certificate_bytes,
                membership,
                application,
            )?
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
            .authorize_mutation(&AUTHORITY_BOUND_LEASE_ACCESS, authorization)
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
    pub fn authority_revision(&self) -> AuthorityStoreAdapterResult<FencingAuthorityRevision> {
        self.require_read()?;
        self.store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .authority_revision()
            .map_err(Into::into)
    }

    pub fn floor(&self, lineage_id: &[u8]) -> AuthorityStoreAdapterResult<Option<FenceReceipt>> {
        self.require_read()?;
        Ok(self
            .store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .floor(lineage_id)
            .cloned())
    }

    pub fn commit(&self, request: FenceCommit<'_>) -> AuthorityStoreAdapterResult<FenceReceipt> {
        self.require_mutation(FENCING_COMMIT)?;
        self.store
            .lock()
            .map_err(|_| AuthorityStoreAdapterError::LockPoisoned)?
            .commit(&AUTHORITY_BOUND_FENCING_ACCESS, request)
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
            .authorize_active_lease(&AUTHORITY_BOUND_FENCING_ACCESS, check)
            .map_err(Into::into)
    }

    pub fn redacted_snapshot_at(
        &self,
        expected_revision: FencingAuthorityRevision,
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
        certificates::{
            CertificateBody, CertificatePolicy, CertificateValidity, TEST_CERTIFICATE_STORE_ACCESS,
        },
        fencing::{
            FencingCheckpoint, FencingCheckpointAuthority, FencingPolicy, TEST_FENCING_STORE_ACCESS,
        },
        lease::{LeasePolicy, TEST_LEASE_STORE_ACCESS},
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

    #[derive(Debug, Default)]
    struct MemoryFencingCheckpoint(Mutex<Option<FencingCheckpoint>>);

    impl FencingCheckpointAuthority for MemoryFencingCheckpoint {
        fn current(&self) -> Result<Option<FencingCheckpoint>, FencingError> {
            Ok(*self.0.lock().unwrap())
        }

        fn compare_and_swap(
            &self,
            expected: Option<FencingCheckpoint>,
            next: FencingCheckpoint,
        ) -> Result<(), FencingError> {
            let mut current = self.0.lock().unwrap();
            if *current != expected {
                return Err(FencingError::Rollback);
            }
            *current = Some(next);
            Ok(())
        }
    }

    fn lease_policy() -> LeasePolicy {
        LeasePolicy {
            max_lease_duration_millis: 2_000,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
            max_lineages: 64,
            max_snapshot_bytes: 1024 * 1024,
        }
    }

    fn fencing_policy() -> FencingPolicy {
        FencingPolicy {
            max_lineages: 64,
            max_receipts: 64,
            max_state_bytes: 1024 * 1024,
            max_clock_uncertainty_millis: 10,
            message_delay_margin_millis: 5,
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
                    expires_at_unix_secs: NOW + 600,
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
            DistributedCertificateStore::open(
                &TEST_CERTIFICATE_STORE_ACCESS,
                temp.path().join("certificates.redb"),
                policy,
            )
            .unwrap(),
        );
        let certificate = certificate(
            &signing_root,
            "node-a",
            CertificatePurpose::Transport,
            1,
            &subject,
        );
        raw.activate(&TEST_CERTIFICATE_STORE_ACCESS, &certificate, NOW)
            .unwrap();

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

    #[test]
    fn published_view_exposes_all_store_authority_kinds() {
        let root = std::env::current_dir()
            .and_then(std::fs::canonicalize)
            .unwrap();
        let expected = [
            (CERTIFICATE_ACTIVATE, PUBLISHED_VIEW_ACTION_OWNER_COMMIT),
            (CERTIFICATE_REVOKE, PUBLISHED_VIEW_ACTION_OWNER_COMMIT),
            (LEASE_APPLY, PUBLISHED_VIEW_ACTION_OWNER_COMMIT),
            (LEASE_MUTATION, PUBLISHED_VIEW_ACTION_OWNER_COMMIT),
            (FENCING_COMMIT, PUBLISHED_VIEW_ACTION_FENCE),
            (FENCING_ACTIVE_LEASE, PUBLISHED_VIEW_ACTION_FENCE),
        ];

        for (mutation_kind, action_class) in expected {
            let temp = tempfile::TempDir::new_in(&root).unwrap();
            let lineage_id = format!("lineage-{mutation_kind}");
            let registry = AuthorityStoreAdapterRegistry::new(publish_lineage(
                temp.path(),
                &lineage_id,
                mutation_kind,
            ));
            let view = registry.published_view(&lineage_id).unwrap();

            assert_eq!(view.lineage_id(), lineage_id);
            assert_eq!(view.operation_id(), "issue-203-operation");
            assert_eq!(view.action_class(), action_class);
            assert_eq!(view.adapter_kind(), "adl.test.deterministic-authority");
            assert_eq!(view.adapter_version(), 1);
            assert_eq!(view.generation(), 1);
            assert_ne!(view.receipt_sha256(), [0; 32]);
            assert_ne!(view.result_sha256(), [0; 32]);
        }
    }

    #[test]
    fn lease_and_fencing_handles_require_published_200_barrier() {
        let root = std::env::current_dir()
            .and_then(std::fs::canonicalize)
            .unwrap();
        let unpublished_temp = tempfile::TempDir::new_in(&root).unwrap();
        let unpublished = AuthorityStoreAdapterRegistry::new(Arc::new(
            AuthorityReconciliationBarrier::open(
                unpublished_temp.path(),
                reconciliation_identity(),
                Arc::new(MemoryCheckpoint::default()),
            )
            .unwrap(),
        ));
        let ledger = Arc::new(Mutex::new(
            AuthorityLedger::new(&TEST_LEASE_STORE_ACCESS, lease_policy()).unwrap(),
        ));
        let fencing_root = unpublished_temp.path().join("fencing");
        std::fs::create_dir(&fencing_root).unwrap();
        let fencing = Arc::new(Mutex::new(
            FencingStore::create(
                &TEST_FENCING_STORE_ACCESS,
                &fencing_root,
                fencing_policy(),
                Arc::new(MemoryFencingCheckpoint::default()),
            )
            .unwrap(),
        ));

        assert!(matches!(
            unpublished.lease_ledger("lineage-a", Arc::clone(&ledger)),
            Err(AuthorityStoreAdapterError::Reconciliation(
                AuthorityReconciliationError::ReconciliationRequired
            ))
        ));
        assert!(matches!(
            unpublished.fencing_store("lineage-a", Arc::clone(&fencing)),
            Err(AuthorityStoreAdapterError::Reconciliation(
                AuthorityReconciliationError::ReconciliationRequired
            ))
        ));

        let published_temp = tempfile::TempDir::new_in(root).unwrap();
        let published = AuthorityStoreAdapterRegistry::new(publish_lineage(
            published_temp.path(),
            "lineage-a",
            "lease_apply",
        ));
        let bound_ledger = published
            .lease_ledger("lineage-a", Arc::clone(&ledger))
            .unwrap();
        let bound_fencing = published
            .fencing_store("lineage-a", Arc::clone(&fencing))
            .unwrap();

        assert_eq!(
            bound_ledger
                .authority_revision()
                .unwrap()
                .applied_log_index(),
            0
        );
        assert!(!bound_ledger.snapshot().unwrap().is_empty());
        assert_eq!(
            bound_fencing
                .authority_revision()
                .unwrap()
                .checkpoint_generation(),
            0
        );
        assert_eq!(bound_fencing.floor(b"lineage-a").unwrap(), None);
    }
}
