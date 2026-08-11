use std::{
    collections::BTreeMap,
    fs,
    path::Path,
    sync::{Arc, Mutex, OnceLock},
};

use super::*;
use crate::distributed::{
    authority_protocol::{
        mutate_test_reconciliation_token, test_published_reconciliation_token,
        TestReconciliationTokenMutation,
    },
    polis_runtime::{validate_authority_command_boundary, ConsensusCheckpoint, PolisCommand},
};

const MARKER: &str = "ADL_ISSUE_200_CASE_V1 ";
const ASSERTION_MARKER: &str = "ADL_ISSUE_200_ASSERTION_V1 ";

struct TempDir;

impl TempDir {
    fn new() -> std::io::Result<tempfile::TempDir> {
        let root = std::env::current_dir()?.canonicalize()?;
        tempfile::TempDir::new_in(root)
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Gate {
    #[default]
    Ready,
    NotReady,
    Unsafe,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum ReceiptMode {
    #[default]
    Exact,
    Reordered,
    Duplicate,
    Forged,
}

#[derive(Clone, Debug, Default)]
struct Hook {
    fault_once: Option<String>,
    gate: Gate,
    receipt_mode: ReceiptMode,
    executions: usize,
}

fn hooks() -> &'static Mutex<BTreeMap<String, Hook>> {
    static HOOKS: OnceLock<Mutex<BTreeMap<String, Hook>>> = OnceLock::new();
    HOOKS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn configure(operation_id: &str, configure: impl FnOnce(&mut Hook)) {
    let mut hooks = hooks().lock().unwrap();
    configure(hooks.entry(operation_id.to_owned()).or_default());
}

fn executions(operation_id: &str) -> usize {
    hooks()
        .lock()
        .unwrap()
        .get(operation_id)
        .map_or(0, |hook| hook.executions)
}

pub(super) fn test_fault(operation_id: &str, point: &str) -> AuthorityReconciliationResult<()> {
    let mut hooks = hooks().lock().unwrap();
    let hook = hooks.entry(operation_id.to_owned()).or_default();
    if hook.fault_once.as_deref() == Some(point) {
        hook.fault_once = None;
        return Err(AuthorityReconciliationError::Interrupted);
    }
    Ok(())
}

pub(super) fn execute_test_step(
    operation: &DurableReconciliationOperation,
    index: usize,
) -> AuthorityReconciliationResult<AuthorityStepReceipt> {
    let mut hooks = hooks().lock().unwrap();
    let hook = hooks.entry(operation.operation_id.clone()).or_default();
    match hook.gate {
        Gate::NotReady => return Err(AuthorityReconciliationError::ClockNotReady),
        Gate::Unsafe => return Err(AuthorityReconciliationError::ClockUnsafe),
        Gate::Ready => {}
    }
    hook.executions += 1;
    let plan = operation
        .plan
        .get(index)
        .ok_or(AuthorityReconciliationError::ReceiptMismatch)?;
    let mut receipt = AuthorityStepReceipt {
        index: plan.index,
        input_sha256: plan.input_sha256,
        output_sha256: plan.expected_output_sha256,
        receipt_sha256: domain_digest(
            b"ADL-AUTHORITY-RECONCILIATION-STEP-RECEIPT-V1\0",
            &(
                operation.token_sha256,
                operation.plan_sha256,
                plan.index,
                plan.input_sha256,
                plan.expected_output_sha256,
            ),
        )?,
    };
    match hook.receipt_mode {
        ReceiptMode::Exact => {}
        ReceiptMode::Reordered => receipt.index = receipt.index.saturating_add(1),
        ReceiptMode::Duplicate => receipt.index = 0,
        ReceiptMode::Forged => receipt.receipt_sha256[0] ^= 1,
    }
    Ok(receipt)
}

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

fn identity() -> AuthorityReconciliationIdentity {
    AuthorityReconciliationIdentity {
        trust_domain: "runtime-prod".to_owned(),
        polis_id: "polis-a".to_owned(),
        node_id: "node-a".to_owned(),
        guardian_id: "guardian-a".to_owned(),
        boot_generation: 7,
        protocol_instance: PROTOCOL_INSTANCE.to_owned(),
    }
}

fn artifact(lineage: &str, steps: usize) -> AuthorityReconciliationArtifact {
    AuthorityReconciliationArtifact::new(
        lineage.to_owned(),
        TEST_ADAPTER_KIND.to_owned(),
        TEST_ADAPTER_VERSION,
        "activate-authority".to_owned(),
        (0..steps)
            .map(|index| format!("step-{index}").into_bytes())
            .collect(),
        format!("published-{lineage}").into_bytes(),
        2_000_000_000,
    )
    .unwrap()
}

fn token(
    operation_id: &str,
    artifact: &AuthorityReconciliationArtifact,
    log_index: u64,
) -> PublishedAuthorityResult {
    test_published_reconciliation_token(
        AuthorityNodeIdentity {
            trust_domain: "runtime-prod".to_owned(),
            polis_id: "polis-a".to_owned(),
            node_id: "node-a".to_owned(),
            guardian_id: "guardian-a".to_owned(),
            boot_generation: 7,
        },
        operation_id,
        artifact.committed_artifact().unwrap(),
        log_index,
        CanonicalAuthorityTime {
            unix_seconds: 1_900_000_000,
            nanos: 17,
            uncertainty_millis: 25,
        },
    )
}

fn open(root: &Path, checkpoint: Arc<MemoryCheckpoint>) -> AuthorityReconciliationBarrier {
    AuthorityReconciliationBarrier::open(root, identity(), checkpoint).unwrap()
}

fn open_capacity(
    root: &Path,
    checkpoint: Arc<MemoryCheckpoint>,
    capacity: usize,
) -> AuthorityReconciliationBarrier {
    AuthorityReconciliationBarrier::open_with_capacity(root, identity(), checkpoint, capacity)
        .unwrap()
}

fn marker(name: &str, result: &str) {
    println!("{MARKER}{name} {result}");
}

fn assertion(case: &str, name: &str) {
    println!("{ASSERTION_MARKER}{case} {name}");
}

fn commit_test_view_mutation(
    barrier: &mut AuthorityReconciliationBarrier,
    lineage_id: &str,
    mutate: impl FnOnce(&mut PublishedView),
) {
    let mut next = barrier.envelope.payload().clone();
    next.revision = next.revision.checked_add(1).unwrap();
    mutate(next.published.get_mut(lineage_id).unwrap());
    barrier.envelope = barrier.store.commit(&barrier.envelope, next).unwrap();
}

fn crash_retry(point: &str, operation_id: &str, steps: usize) {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact(operation_id, steps);
    let operation_token = token(operation_id, &operation_artifact, 11);
    configure(operation_id, |hook| {
        hook.fault_once = Some(point.to_owned())
    });
    let mut barrier = open(root.path(), checkpoint.clone());
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::Interrupted)
    );
    drop(barrier);
    let mut barrier = open(root.path(), checkpoint);
    assert_eq!(
        barrier.reconcile(&operation_token).unwrap().result(),
        operation_artifact.result
    );
}

#[test]
fn authority_reconciliation_happy_single_step() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-happy-one", 1);
    let operation_token = token("happy-single", &operation_artifact, 11);
    let mut barrier = open(root.path(), checkpoint);
    let result = barrier.reconcile(&operation_token).unwrap();
    assert_eq!(result.result(), operation_artifact.result);
    assert_eq!(result.generation(), 1);
    marker("happy_single_step", "passed");
}

#[test]
fn authority_reconciliation_happy_multi_step() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-happy-many", 4);
    let operation_token = token("happy-many", &operation_artifact, 12);
    let mut barrier = open(root.path(), checkpoint);
    assert_eq!(barrier.reconcile(&operation_token).unwrap().generation(), 1);
    assert_eq!(executions("happy-many"), 4);
    marker("happy_multi_step", "passed");
}

#[test]
fn authority_reconciliation_exact_retry_cached_result() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-cache", 3);
    let operation_token = token("cache", &operation_artifact, 13);
    configure("cache", |hook| {
        hook.fault_once = Some("after_result".to_owned())
    });
    let mut barrier = open(root.path(), checkpoint.clone());
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::Interrupted)
    );
    assert_eq!(executions("cache"), 3);
    drop(barrier);
    let mut barrier = open(root.path(), checkpoint);
    let first = barrier.reconcile(&operation_token).unwrap();
    let second = barrier.reconcile(&operation_token).unwrap();
    assert_eq!(first, second);
    assert_eq!(executions("cache"), 3);
    assertion("exact_retry_cached_result", "cached_result_no_reexecution");

    let conflict_root = TempDir::new().unwrap();
    let conflict_checkpoint = Arc::new(MemoryCheckpoint::default());
    let conflict_artifact = artifact("lineage-cache-conflict", 1);
    let conflict_token = token("cache-conflict", &conflict_artifact, 131);
    let mut conflict_barrier = open(conflict_root.path(), conflict_checkpoint);
    conflict_barrier.reconcile(&conflict_token).unwrap();
    commit_test_view_mutation(&mut conflict_barrier, "lineage-cache-conflict", |view| {
        view.operation_id = "conflicting-operation".to_owned()
    });
    assert_eq!(
        conflict_barrier.reconcile(&conflict_token),
        Err(AuthorityReconciliationError::StateRegression)
    );
    assertion("exact_retry_cached_result", "conflicting_view_rejected");

    let corrupt_root = TempDir::new().unwrap();
    let corrupt_checkpoint = Arc::new(MemoryCheckpoint::default());
    let corrupt_artifact = artifact("lineage-cache-corrupt", 1);
    let corrupt_token = token("cache-corrupt", &corrupt_artifact, 132);
    let mut corrupt_barrier = open(corrupt_root.path(), corrupt_checkpoint);
    corrupt_barrier.reconcile(&corrupt_token).unwrap();
    commit_test_view_mutation(&mut corrupt_barrier, "lineage-cache-corrupt", |view| {
        view.result_sha256[0] ^= 1
    });
    assert_eq!(
        corrupt_barrier.reconcile(&corrupt_token),
        Err(AuthorityReconciliationError::StateRegression)
    );
    assertion("exact_retry_cached_result", "corrupt_view_rejected");
    marker("exact_retry_cached_result", "passed");
}

#[test]
fn authority_reconciliation_pending_blocks_read() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-pending-read", 1);
    let operation_token = token("pending-read", &operation_artifact, 14);
    configure("pending-read", |hook| {
        hook.fault_once = Some("after_journal".to_owned())
    });
    let mut barrier = open(root.path(), checkpoint);
    assert!(barrier.reconcile(&operation_token).is_err());
    assert_eq!(
        barrier.read_permit("lineage-pending-read"),
        Err(AuthorityReconciliationError::ReconciliationRequired)
    );
    marker("pending_blocks_read", "rejected");
}

#[test]
fn authority_reconciliation_pending_blocks_mutation() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-pending-mutation", 1);
    let operation_token = token("pending-mutation", &operation_artifact, 15);
    configure("pending-mutation", |hook| {
        hook.fault_once = Some("after_journal".to_owned())
    });
    let mut barrier = open(root.path(), checkpoint);
    assert!(barrier.reconcile(&operation_token).is_err());
    assert_eq!(
        barrier.mutation_permit("lineage-pending-mutation", "activate-authority"),
        Err(AuthorityReconciliationError::ReconciliationRequired)
    );
    marker("pending_blocks_mutation", "rejected");
}

#[test]
fn authority_reconciliation_published_permit_current() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let first_artifact = artifact("lineage-permit", 1);
    let first_token = token("permit-one", &first_artifact, 16);
    let mut barrier = open(root.path(), checkpoint);
    barrier.reconcile(&first_token).unwrap();
    let read = barrier.read_permit("lineage-permit").unwrap();
    let mutation = barrier
        .mutation_permit("lineage-permit", "activate-authority")
        .unwrap();
    barrier
        .validate_permit(&read, &AuthorityPermitAction::Read)
        .unwrap();
    assertion("published_permit_current", "current_read_valid");
    barrier
        .validate_permit(
            &mutation,
            &AuthorityPermitAction::Mutation("activate-authority".to_owned()),
        )
        .unwrap();
    assertion("published_permit_current", "current_mutation_valid");
    assert_eq!(
        barrier.validate_permit(
            &read,
            &AuthorityPermitAction::Mutation("activate-authority".to_owned())
        ),
        Err(AuthorityReconciliationError::PermitDenied)
    );
    assertion("published_permit_current", "read_escalation_denied");
    let mut wrong_lineage = mutation.clone();
    wrong_lineage.lineage_id = "lineage-other".to_owned();
    assert_eq!(
        barrier.validate_permit(
            &wrong_lineage,
            &AuthorityPermitAction::Mutation("activate-authority".to_owned())
        ),
        Err(AuthorityReconciliationError::ReconciliationRequired)
    );
    assertion("published_permit_current", "wrong_lineage_denied");
    assert_eq!(
        barrier.validate_permit(
            &mutation,
            &AuthorityPermitAction::Mutation("wrong-action".to_owned())
        ),
        Err(AuthorityReconciliationError::PermitDenied)
    );
    assert_eq!(
        barrier.mutation_permit("lineage-permit", "wrong-action"),
        Err(AuthorityReconciliationError::PermitDenied)
    );
    assertion("published_permit_current", "wrong_mutation_action_denied");
    let second_artifact = artifact("lineage-permit", 1);
    let second_token = token("permit-two", &second_artifact, 17);
    configure("permit-two", |hook| {
        hook.fault_once = Some("after_journal".to_owned())
    });
    assert!(barrier.reconcile(&second_token).is_err());
    assert_eq!(
        barrier.validate_permit(&read, &AuthorityPermitAction::Read),
        Err(AuthorityReconciliationError::PermitDenied)
    );
    assertion(
        "published_permit_current",
        "retained_read_denied_after_pending",
    );
    assert_eq!(
        barrier.validate_permit(
            &mutation,
            &AuthorityPermitAction::Mutation("activate-authority".to_owned())
        ),
        Err(AuthorityReconciliationError::PermitDenied)
    );
    assertion(
        "published_permit_current",
        "retained_mutation_denied_after_pending",
    );
    marker("published_permit_current", "passed");
}

macro_rules! identity_rejection {
    ($test:ident, $case:literal, $mutation:expr, $error:expr) => {
        #[test]
        fn $test() {
            let root = TempDir::new().unwrap();
            let checkpoint = Arc::new(MemoryCheckpoint::default());
            let operation_artifact = artifact(concat!("lineage-", $case), 1);
            let mut operation_token = token($case, &operation_artifact, 20);
            mutate_test_reconciliation_token(&mut operation_token, $mutation);
            let mut barrier = open(root.path(), checkpoint);
            assert_eq!(barrier.reconcile(&operation_token), Err($error));
            marker($case, "rejected");
        }
    };
}

identity_rejection!(
    authority_reconciliation_wrong_domain,
    "wrong_domain",
    TestReconciliationTokenMutation::TrustDomain,
    AuthorityReconciliationError::WrongTrustDomain
);
identity_rejection!(
    authority_reconciliation_wrong_polis,
    "wrong_polis",
    TestReconciliationTokenMutation::Polis,
    AuthorityReconciliationError::WrongPolis
);
identity_rejection!(
    authority_reconciliation_wrong_node,
    "wrong_node",
    TestReconciliationTokenMutation::Node,
    AuthorityReconciliationError::WrongNode
);
identity_rejection!(
    authority_reconciliation_wrong_guardian,
    "wrong_guardian",
    TestReconciliationTokenMutation::Guardian,
    AuthorityReconciliationError::WrongGuardian
);
identity_rejection!(
    authority_reconciliation_wrong_boot,
    "wrong_boot",
    TestReconciliationTokenMutation::BootGeneration,
    AuthorityReconciliationError::WrongBootGeneration
);
identity_rejection!(
    authority_reconciliation_wrong_membership,
    "wrong_membership",
    TestReconciliationTokenMutation::Membership,
    AuthorityReconciliationError::WrongMembership
);

#[test]
fn authority_reconciliation_wrong_protocol_instance() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = artifact("lineage-protocol", 1);
    let operation_token = token("wrong-protocol", &operation_artifact, 21);
    let mut barrier = open(root.path(), checkpoint);
    barrier.identity.protocol_instance.push('x');
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::WrongProtocolInstance)
    );
    marker("wrong_protocol_instance", "rejected");
}

#[test]
fn authority_reconciliation_wrong_operation_kind() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let committed = CommittedAuthorityArtifact::new(
        AuthorityOperationKind::Membership,
        b"not-reconciliation".to_vec(),
    )
    .unwrap();
    let operation_token = test_published_reconciliation_token(
        AuthorityNodeIdentity {
            trust_domain: "runtime-prod".into(),
            polis_id: "polis-a".into(),
            node_id: "node-a".into(),
            guardian_id: "guardian-a".into(),
            boot_generation: 7,
        },
        "wrong-kind",
        committed,
        22,
        CanonicalAuthorityTime {
            unix_seconds: 1_900_000_000,
            nanos: 0,
            uncertainty_millis: 0,
        },
    );
    let mut barrier = open(root.path(), checkpoint);
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::WrongOperationKind)
    );
    marker("wrong_operation_kind", "rejected");
}

#[test]
fn authority_reconciliation_wrong_adapter_version() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = AuthorityReconciliationArtifact::new(
        "lineage-adapter".into(),
        TEST_ADAPTER_KIND.into(),
        99,
        "activate-authority".into(),
        vec![b"step".to_vec()],
        b"result".to_vec(),
        2_000_000_000,
    )
    .unwrap();
    let operation_token = token("wrong-adapter", &operation_artifact, 23);
    let mut barrier = open(root.path(), checkpoint);
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::UnknownAdapter)
    );
    marker("wrong_adapter_version", "rejected");
}

#[test]
fn authority_reconciliation_wrong_time_digest() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let operation_artifact = AuthorityReconciliationArtifact::new(
        "lineage-time".into(),
        TEST_ADAPTER_KIND.into(),
        TEST_ADAPTER_VERSION,
        "activate-authority".into(),
        vec![b"step".to_vec()],
        b"result".to_vec(),
        1_800_000_000,
    )
    .unwrap();
    let operation_token = token("wrong-time", &operation_artifact, 24);
    let mut barrier = open(root.path(), checkpoint);
    assert_eq!(
        barrier.reconcile(&operation_token),
        Err(AuthorityReconciliationError::WrongTimeEvidence)
    );
    assert!(barrier.envelope.payload().operations.is_empty());

    for (suffix, gate, error) in [
        (
            "not-ready",
            Gate::NotReady,
            AuthorityReconciliationError::ClockNotReady,
        ),
        (
            "unsafe",
            Gate::Unsafe,
            AuthorityReconciliationError::ClockUnsafe,
        ),
    ] {
        let gate_root = TempDir::new().unwrap();
        let gate_checkpoint = Arc::new(MemoryCheckpoint::default());
        let operation_id = format!("clock-{suffix}");
        let gate_artifact = artifact(&format!("lineage-{suffix}"), 1);
        let gate_token = token(&operation_id, &gate_artifact, 25);
        configure(&operation_id, |hook| hook.gate = gate);
        let mut gate_barrier = open(gate_root.path(), gate_checkpoint);
        assert_eq!(gate_barrier.reconcile(&gate_token), Err(error));
        let durable = &gate_barrier.envelope.payload().operations[&operation_id];
        assert!(durable.receipts.is_empty());
        assert!(!durable.result_cached);
        configure(&operation_id, |hook| hook.gate = Gate::Ready);
        gate_barrier.reconcile(&gate_token).unwrap();
    }
    marker("wrong_time_digest", "rejected");
}

#[test]
fn authority_reconciliation_conflicting_retry() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let first_artifact = artifact("lineage-retry", 1);
    let second_artifact = artifact("lineage-retry-other", 1);
    let first = token("retry-conflict", &first_artifact, 25);
    let second = token("retry-conflict", &second_artifact, 26);
    let mut barrier = open(root.path(), checkpoint);
    barrier.reconcile(&first).unwrap();
    assert_eq!(
        barrier.reconcile(&second),
        Err(AuthorityReconciliationError::RetryConflict)
    );
    marker("conflicting_retry", "rejected");
}

macro_rules! receipt_rejection {
    ($test:ident, $case:literal, $mode:expr, $steps:expr) => {
        #[test]
        fn $test() {
            let root = TempDir::new().unwrap();
            let checkpoint = Arc::new(MemoryCheckpoint::default());
            let operation_artifact = artifact(concat!("lineage-", $case), $steps);
            let operation_token = token($case, &operation_artifact, 30);
            configure($case, |hook| hook.receipt_mode = $mode);
            let mut barrier = open(root.path(), checkpoint);
            assert_eq!(
                barrier.reconcile(&operation_token),
                Err(AuthorityReconciliationError::ReceiptMismatch)
            );
            marker($case, "rejected");
        }
    };
}

receipt_rejection!(
    authority_reconciliation_reordered_step,
    "reordered_step",
    ReceiptMode::Reordered,
    2
);
receipt_rejection!(
    authority_reconciliation_duplicate_step,
    "duplicate_step",
    ReceiptMode::Duplicate,
    2
);
receipt_rejection!(
    authority_reconciliation_forged_step_receipt,
    "forged_step_receipt",
    ReceiptMode::Forged,
    1
);

#[test]
fn authority_reconciliation_missing_step() {
    assert_eq!(
        AuthorityReconciliationArtifact::new(
            "lineage-missing-step".into(),
            TEST_ADAPTER_KIND.into(),
            TEST_ADAPTER_VERSION,
            "activate-authority".into(),
            Vec::new(),
            b"result".to_vec(),
            2_000_000_000,
        ),
        Err(AuthorityReconciliationError::InvalidArtifact)
    );
    marker("missing_step", "rejected");
}

#[test]
fn authority_reconciliation_crash_after_journal() {
    crash_retry("after_journal", "crash-journal", 2);
    marker("crash_after_journal", "reconciled");
}

#[test]
fn authority_reconciliation_crash_each_step() {
    for (index, point) in [
        "before_step_0",
        "after_effect_0",
        "after_receipt_0",
        "before_step_1",
        "after_effect_1",
        "after_receipt_1",
    ]
    .into_iter()
    .enumerate()
    {
        crash_retry(point, &format!("crash-step-{index}"), 2);
    }
    marker("crash_each_step", "reconciled");
}

#[test]
fn authority_reconciliation_crash_after_result() {
    crash_retry("after_result", "crash-result", 2);
    marker("crash_after_result", "reconciled");
}

#[test]
fn authority_reconciliation_crash_before_checkpoint() {
    crash_retry("before_checkpoint", "crash-before-checkpoint", 2);
    marker("crash_before_checkpoint", "reconciled");
}

#[test]
fn authority_reconciliation_crash_after_checkpoint() {
    for (point, operation, subassertion) in [
        (
            "after_checkpoint",
            "crash-after-checkpoint",
            "missing_marker_and_view_retry",
        ),
        (
            "after_marker",
            "crash-after-marker",
            "committed_marker_missing_view_retry",
        ),
        (
            "after_view",
            "crash-after-view",
            "published_view_exact_retry",
        ),
    ] {
        crash_retry(point, operation, 2);
        assertion("crash_after_checkpoint", subassertion);
    }
    marker("crash_after_checkpoint", "reconciled");
}

#[test]
fn authority_reconciliation_coherent_rollback() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let initial = {
        let barrier = open(root.path(), checkpoint.clone());
        let bytes = fs::read(root.path().join("authority-reconciliation.json")).unwrap();
        drop(barrier);
        bytes
    };
    let operation_artifact = artifact("lineage-rollback", 1);
    let operation_token = token("rollback", &operation_artifact, 40);
    let mut barrier = open(root.path(), checkpoint.clone());
    barrier.reconcile(&operation_token).unwrap();
    drop(barrier);
    fs::write(root.path().join("authority-reconciliation.json"), initial).unwrap();
    assert!(AuthorityReconciliationBarrier::open(root.path(), identity(), checkpoint).is_err());
    marker("coherent_rollback", "rejected");
}

#[test]
fn authority_reconciliation_capacity_n_plus_one_no_partial() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let first_artifact = artifact("lineage-capacity-one", 1);
    let second_artifact = artifact("lineage-capacity-two", 1);
    let first = token("capacity-one", &first_artifact, 41);
    let second = token("capacity-two", &second_artifact, 42);
    let mut barrier = open_capacity(root.path(), checkpoint, 1);
    let retained = barrier.reconcile(&first).unwrap();
    assert_eq!(
        barrier.reconcile(&second),
        Err(AuthorityReconciliationError::CapacityExceeded)
    );
    assert_eq!(
        barrier.published_result("lineage-capacity-one"),
        Some(retained)
    );
    assert_eq!(executions("capacity-two"), 0);
    marker("capacity_n_plus_one_no_partial", "rejected");
}

#[test]
fn authority_reconciliation_state_or_lock_symlink_rejected() {
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        for file in [
            "authority-reconciliation.json",
            ".authority-reconciliation.json.lock",
        ] {
            let root = TempDir::new().unwrap();
            let target = root.path().join("target");
            fs::write(&target, b"{}").unwrap();
            symlink(&target, root.path().join(file)).unwrap();
            assert!(AuthorityReconciliationBarrier::open(
                root.path(),
                identity(),
                Arc::new(MemoryCheckpoint::default())
            )
            .is_err());
        }
    }
    marker("state_or_lock_symlink_rejected", "rejected");
}

#[test]
fn authority_reconciliation_corrupt_journal_rejected() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let barrier = open(root.path(), checkpoint.clone());
    drop(barrier);
    fs::write(
        root.path().join(".authority-reconciliation.json.journal"),
        b"not-json",
    )
    .unwrap();
    assert!(AuthorityReconciliationBarrier::open(root.path(), identity(), checkpoint).is_err());
    marker("corrupt_journal_rejected", "rejected");
}

#[test]
fn authority_reconciliation_noncanonical_state_rejected() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let barrier = open(root.path(), checkpoint.clone());
    drop(barrier);
    let path = root.path().join("authority-reconciliation.json");
    let mut bytes = fs::read(&path).unwrap();
    bytes.push(b'\n');
    fs::write(path, bytes).unwrap();
    assert!(AuthorityReconciliationBarrier::open(root.path(), identity(), checkpoint).is_err());
    marker("noncanonical_state_rejected", "rejected");
}

#[test]
fn authority_reconciliation_opened_handle_growth_rejected() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let barrier = open(root.path(), checkpoint.clone());
    drop(barrier);
    fs::write(
        root.path().join("authority-reconciliation.json"),
        vec![b'x'; 10 * 1024 * 1024],
    )
    .unwrap();
    assert!(AuthorityReconciliationBarrier::open(root.path(), identity(), checkpoint).is_err());
    marker("opened_handle_growth_rejected", "rejected");
}

#[test]
fn authority_reconciliation_checkpoint_object_collision() {
    let root = TempDir::new().unwrap();
    let checkpoint = Arc::new(MemoryCheckpoint::default());
    let barrier = open(root.path(), checkpoint.clone());
    drop(barrier);
    let mut other = identity();
    other.node_id = "node-b".to_owned();
    assert!(AuthorityReconciliationBarrier::open(root.path(), other, checkpoint).is_err());
    marker("checkpoint_object_collision", "rejected");
}

#[test]
fn authority_reconciliation_legacy_command_denied() {
    assert!(
        validate_authority_command_boundary(&PolisCommand::FenceVoter {
            voter_id: "node-a".to_owned(),
            epoch: 1,
        })
        .is_err()
    );
    marker("legacy_command_denied", "rejected");
}
