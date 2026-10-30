use std::{collections::BTreeMap, sync::Mutex};

use adl_runtime::distributed::{
    authority_reconciliation::{
        AuthorityReconciliationBarrier, AuthorityReconciliationError,
        AuthorityReconciliationIdentity,
    },
    polis_runtime::{ConsensusCheckpoint, ConsensusCheckpointAuthority, PolisRuntimeError},
};

const MARKER: &str = "ADL_ISSUE_200_CASE_V1 ";

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
        protocol_instance: "adl.authority-reconciliation.v1".to_owned(),
    }
}

fn temp_root() -> tempfile::TempDir {
    let root = std::env::current_dir()
        .and_then(std::fs::canonicalize)
        .expect("current test directory must have a canonical symlink-free path");
    tempfile::TempDir::new_in(root).expect("portable repository-local test root")
}

#[test]
fn authority_reconciliation_missing_201_token() {
    let root = temp_root();
    let barrier = AuthorityReconciliationBarrier::open(
        root.path(),
        identity(),
        std::sync::Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    assert!(barrier.published_result("missing-lineage").is_none());
    println!("{MARKER}missing_201_token rejected");
}

#[test]
fn authority_reconciliation_public_token_forgery_denied() {
    let root = temp_root();
    let barrier = AuthorityReconciliationBarrier::open(
        root.path(),
        identity(),
        std::sync::Arc::new(MemoryCheckpoint::default()),
    )
    .unwrap();
    assert_eq!(
        barrier.reject_untrusted_reconciliation(br#"{"operation_id":"forged"}"#),
        Err(AuthorityReconciliationError::UntrustedAuthority)
    );
    println!("{MARKER}public_token_forgery_denied rejected");
}
