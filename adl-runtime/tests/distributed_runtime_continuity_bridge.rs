// PVF: lane=exact-child-tests; proof=kernel continuity catalog to migration/transfer bridge;
// deterministic=true; resource_profile=fast; release_gate=true.

use std::collections::BTreeMap;

use adl_runtime::distributed::{
    authority_protocol::{CanonicalAuthorityTime, CommittedAuthorityArtifact},
    continuity_transfer::{
        ContinuityTransferExpectation, ContinuityTransferPolicy, ContinuityTransferSession,
    },
    runtime_continuity_bridge::{
        continuity_transfer_grant_from_kernel_catalog, snapshot_descriptor_from_kernel_catalog,
        ContinuityGrantBinding, KernelSnapshotBinding, RuntimeContinuityBridgeError,
        KERNEL_CONTINUITY_SNAPSHOT_SCHEMA,
    },
};
use adl_runtime_kernel::{
    BundleEntry, SignedBundleCatalog, SourceCheckpointHandle, TargetStageHandle,
};
use ed25519_dalek::{Signer, SigningKey};
use sha2::{Digest, Sha256};

const DOMAIN: &str = "polis.example";
const LINEAGE: &[u8] = b"lineage-a";

fn sha(bytes: &[u8]) -> [u8; 32] {
    Sha256::digest(bytes).into()
}

fn sha_hex(bytes: &[u8]) -> String {
    hex::encode(sha(bytes))
}

fn kernel_bundle_digest(catalog: &SignedBundleCatalog) -> [u8; 32] {
    let mut hash = Sha256::new();
    hash.update(catalog.digest().unwrap().as_bytes());
    for entry in &catalog.entries {
        hash.update(entry.ordinal.to_be_bytes());
        hash.update(entry.sha256.as_bytes());
    }
    hash.finalize().into()
}

fn sign_catalog(mut catalog: SignedBundleCatalog, key: &SigningKey) -> SignedBundleCatalog {
    catalog.signature.clear();
    catalog.signature = hex::encode(key.sign(&catalog.unsigned_bytes().unwrap()).to_bytes());
    catalog
}

fn catalog() -> (SignedBundleCatalog, SigningKey, Vec<Vec<u8>>) {
    let key = SigningKey::from_bytes(&[71; 32]);
    let chunks = vec![
        b"live-kernel-state-a".to_vec(),
        b"live-kernel-state-b".to_vec(),
    ];
    let catalog = SignedBundleCatalog {
        generation: 9,
        predecessor_sha256: Some("11".repeat(32)),
        accepted_prefix: 44,
        topology_sha256: "22".repeat(32),
        config_sha256: "33".repeat(32),
        entries: vec![
            BundleEntry {
                ordinal: 0,
                service: "operation_state".to_owned(),
                schema: "adl.runtime.operation_continuity.v1".to_owned(),
                file: "0000-operation_state.bin".to_owned(),
                offset: 0,
                bytes: chunks[0].len() as u64,
                sha256: sha_hex(&chunks[0]),
            },
            BundleEntry {
                ordinal: 1,
                service: "reasoning_mutation".to_owned(),
                schema: "adl.runtime.reasoning_mutation.v1".to_owned(),
                file: "0001-reasoning_mutation.bin".to_owned(),
                offset: chunks[0].len() as u64,
                bytes: chunks[1].len() as u64,
                sha256: sha_hex(&chunks[1]),
            },
        ],
        key_id: "runtime-continuity".to_owned(),
        key_generation: 7,
        signature: String::new(),
    };
    (sign_catalog(catalog, &key), key, chunks)
}

fn source_handle(catalog: &SignedBundleCatalog, chunks: &[Vec<u8>]) -> SourceCheckpointHandle {
    let _ = chunks;
    serde_json::from_value(serde_json::json!({
        "generation": catalog.generation,
        "root_generation": catalog.generation,
        "catalog_sha256": catalog.digest().unwrap(),
        "bundle_sha256": hex::encode(kernel_bundle_digest(catalog)),
    }))
    .unwrap()
}

fn target_handle(catalog: &SignedBundleCatalog) -> TargetStageHandle {
    serde_json::from_value(serde_json::json!({
        "stage_id": "stage-193",
        "root_generation": 10,
        "catalog_sha256": catalog.digest().unwrap(),
    }))
    .unwrap()
}

fn trusted(key: &SigningKey) -> BTreeMap<(String, u64), ed25519_dalek::VerifyingKey> {
    BTreeMap::from([(("runtime-continuity".to_owned(), 7), key.verifying_key())])
}

#[test]
fn signed_kernel_catalog_derives_snapshot_descriptor_and_continuity_grant() {
    let (catalog, key, chunks) = catalog();
    let source = source_handle(&catalog, &chunks);
    let target = target_handle(&catalog);
    let trusted = trusted(&key);

    let descriptor = snapshot_descriptor_from_kernel_catalog(
        &catalog,
        &source,
        &trusted,
        KernelSnapshotBinding {
            trust_domain: DOMAIN.to_owned(),
            lineage_id: LINEAGE,
            source_owner_id: b"node-0",
            source_guardian_id: b"guardian-0",
            source_epoch: 5,
            authority_log_index: 44,
            created_at_unix_secs: 100,
            expires_at_unix_secs: 700,
            encryption_context_id: "kernel-continuity-context".to_owned(),
        },
    )
    .unwrap();
    assert_eq!(
        descriptor.snapshot_schema,
        KERNEL_CONTINUITY_SNAPSHOT_SCHEMA
    );
    assert_eq!(descriptor.content_sha256, kernel_bundle_digest(&catalog));
    assert_eq!(
        descriptor.chunk_sha256,
        vec![sha(&chunks[0]), sha(&chunks[1])]
    );
    assert_eq!(descriptor.authority_log_index, catalog.accepted_prefix);

    let manifest_bytes = serde_jcs::to_vec(&catalog).unwrap();
    let catalog_bytes = serde_jcs::to_vec(&descriptor).unwrap();
    let grant = continuity_transfer_grant_from_kernel_catalog(
        &catalog,
        &source,
        &target,
        &trusted,
        ContinuityGrantBinding {
            trust_domain: DOMAIN.to_owned(),
            polis_id: "polis-a".to_owned(),
            source_guardian_id: "guardian-0".to_owned(),
            target_guardian_id: "guardian-1".to_owned(),
            route_id: "route-193".to_owned(),
            membership_epoch: 12,
            membership_log_index: 44,
            source_certificate_generation: 2,
            target_certificate_generation: 3,
            transfer_id: "transfer-193".to_owned(),
            lineage_id: LINEAGE,
            signed_manifest_bytes: manifest_bytes,
            signed_catalog_bytes: catalog_bytes,
            trusted_key_generation: 7,
            inclusive_deadline: CanonicalAuthorityTime {
                unix_seconds: 1_900_000_000,
                nanos: 0,
                uncertainty_millis: 10,
            },
            cleanup_identity: "cleanup-193".to_owned(),
        },
    )
    .unwrap();
    let artifact = CommittedAuthorityArtifact::continuity_transfer(&grant).unwrap();
    ContinuityTransferSession::open(
        &artifact,
        ContinuityTransferExpectation {
            trust_domain: DOMAIN.to_owned(),
            polis_id: "polis-a".to_owned(),
            source_guardian_id: "guardian-0".to_owned(),
            target_guardian_id: "guardian-1".to_owned(),
            route_id: "route-193".to_owned(),
            membership_epoch: 12,
            membership_log_index: 44,
            source_certificate_generation: 2,
            target_certificate_generation: 3,
            source_boot_generation: source.generation(),
            target_boot_generation: target.root_generation(),
            lineage_id: LINEAGE.to_vec(),
            source_checkpoint_handle_identity: grant.source_checkpoint_handle_identity.clone(),
            bundle_handle_identity: grant.bundle_handle_identity.clone(),
        },
        ContinuityTransferPolicy::bounded(64, 128, 4),
    )
    .unwrap();
}

#[test]
fn bridge_denies_unsigned_stale_or_noncontiguous_kernel_catalogs() {
    let (catalog, key, chunks) = catalog();
    let source = source_handle(&catalog, &chunks);
    let trusted = trusted(&key);
    let mut untrusted = catalog.clone();
    untrusted.signature = "00".repeat(64);
    assert_eq!(
        snapshot_descriptor_from_kernel_catalog(
            &untrusted,
            &source,
            &trusted,
            KernelSnapshotBinding {
                trust_domain: DOMAIN.to_owned(),
                lineage_id: LINEAGE,
                source_owner_id: b"node-0",
                source_guardian_id: b"guardian-0",
                source_epoch: 5,
                authority_log_index: 44,
                created_at_unix_secs: 100,
                expires_at_unix_secs: 700,
                encryption_context_id: "kernel-continuity-context".to_owned(),
            },
        )
        .unwrap_err(),
        RuntimeContinuityBridgeError::InvalidCatalog
    );

    assert_eq!(
        snapshot_descriptor_from_kernel_catalog(
            &catalog,
            &source,
            &trusted,
            KernelSnapshotBinding {
                trust_domain: DOMAIN.to_owned(),
                lineage_id: LINEAGE,
                source_owner_id: b"node-0",
                source_guardian_id: b"guardian-0",
                source_epoch: 5,
                authority_log_index: catalog.accepted_prefix + 1,
                created_at_unix_secs: 100,
                expires_at_unix_secs: 700,
                encryption_context_id: "kernel-continuity-context".to_owned(),
            },
        )
        .unwrap_err(),
        RuntimeContinuityBridgeError::InvalidIdentity
    );

    let stale_source: SourceCheckpointHandle = serde_json::from_value(serde_json::json!({
        "generation": catalog.generation,
        "root_generation": catalog.generation,
        "catalog_sha256": "44".repeat(32),
        "bundle_sha256": hex::encode(kernel_bundle_digest(&catalog)),
    }))
    .unwrap();
    assert_eq!(
        snapshot_descriptor_from_kernel_catalog(
            &catalog,
            &stale_source,
            &trusted,
            KernelSnapshotBinding {
                trust_domain: DOMAIN.to_owned(),
                lineage_id: LINEAGE,
                source_owner_id: b"node-0",
                source_guardian_id: b"guardian-0",
                source_epoch: 5,
                authority_log_index: 44,
                created_at_unix_secs: 100,
                expires_at_unix_secs: 700,
                encryption_context_id: "kernel-continuity-context".to_owned(),
            },
        )
        .unwrap_err(),
        RuntimeContinuityBridgeError::InvalidDigest
    );

    let wrong_bundle_source: SourceCheckpointHandle = serde_json::from_value(serde_json::json!({
        "generation": catalog.generation,
        "root_generation": catalog.generation,
        "catalog_sha256": catalog.digest().unwrap(),
        "bundle_sha256": "55".repeat(32),
    }))
    .unwrap();
    assert_eq!(
        snapshot_descriptor_from_kernel_catalog(
            &catalog,
            &wrong_bundle_source,
            &trusted,
            KernelSnapshotBinding {
                trust_domain: DOMAIN.to_owned(),
                lineage_id: LINEAGE,
                source_owner_id: b"node-0",
                source_guardian_id: b"guardian-0",
                source_epoch: 5,
                authority_log_index: 44,
                created_at_unix_secs: 100,
                expires_at_unix_secs: 700,
                encryption_context_id: "kernel-continuity-context".to_owned(),
            },
        )
        .unwrap_err(),
        RuntimeContinuityBridgeError::InvalidDigest
    );

    let mut gap = catalog.clone();
    gap.entries[1].offset += 1;
    gap = sign_catalog(gap, &key);
    let gap_source = source_handle(&gap, &chunks);
    assert_eq!(
        snapshot_descriptor_from_kernel_catalog(
            &gap,
            &gap_source,
            &trusted,
            KernelSnapshotBinding {
                trust_domain: DOMAIN.to_owned(),
                lineage_id: LINEAGE,
                source_owner_id: b"node-0",
                source_guardian_id: b"guardian-0",
                source_epoch: 5,
                authority_log_index: 44,
                created_at_unix_secs: 100,
                expires_at_unix_secs: 700,
                encryption_context_id: "kernel-continuity-context".to_owned(),
            },
        )
        .unwrap_err(),
        RuntimeContinuityBridgeError::InvalidCatalog
    );
}
