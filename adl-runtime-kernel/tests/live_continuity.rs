use std::{collections::BTreeMap, time::Duration};

use adl_runtime_kernel::{
    CheckpointAuthority, CheckpointManifest, CheckpointingControl, ComponentRegistry, Kernel,
    KernelExit, LifecycleControl, LiveContinuity, LiveKernelSnapshot, RuntimeRecorder,
    LIVE_KERNEL_SNAPSHOT_SCHEMA,
};

fn snapshot() -> LiveKernelSnapshot {
    LiveKernelSnapshot::new(
        blake3::hash(b"topology").to_hex().to_string(),
        blake3::hash(b"config").to_hex().to_string(),
        BTreeMap::from([(
            "agent_runtime".to_owned(),
            "adl.runtime.agent_runtime.config.v1".to_owned(),
        )]),
    )
}

#[tokio::test]
async fn signed_live_checkpoint_round_trips() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[41; 32], snapshot(), 0);
    assert_eq!(continuity.restore_latest(&recorder).await.unwrap(), None);
    let manifest = continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(manifest.generation, 1);
    assert_eq!(manifest.previous_integrity, None);
    assert_eq!(manifest.signing_algorithm, "ed25519");
    let second = continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    assert_eq!(second.previous_integrity, Some(manifest.integrity));

    let mut restored = LiveContinuity::new(root.path(), "live", &[41; 32], snapshot(), 1);
    let restored_recorder = RuntimeRecorder::new(16);
    assert_eq!(
        restored.restore_latest(&restored_recorder).await.unwrap(),
        Some(2)
    );
    assert_eq!(
        restored_recorder
            .snapshot()
            .continuity_head
            .unwrap()
            .generation,
        2
    );
}

#[tokio::test]
async fn forged_manifest_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[42; 32], snapshot(), 0);
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let path = root.path().join("generation-1/manifest.json");
    let mut value: serde_json::Value =
        serde_json::from_slice(&tokio::fs::read(&path).await.unwrap()).unwrap();
    value["topology_hash"] = serde_json::Value::String("forged".to_owned());
    tokio::fs::write(&path, serde_json::to_vec(&value).unwrap())
        .await
        .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[42; 32], snapshot(), 0);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .is_err());
}

#[tokio::test]
async fn minimum_generation_refuses_rollback_or_missing_state() {
    let root = tempfile::tempdir().unwrap();
    let mut continuity = LiveContinuity::new(root.path(), "live", &[43; 32], snapshot(), 2);
    let error = continuity
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err();
    assert!(error.to_string().contains("below required minimum 2"));
}

#[tokio::test]
async fn renamed_checkpoint_cannot_spoof_the_signed_generation() {
    let root = tempfile::tempdir().unwrap();
    let mut continuity = LiveContinuity::new(root.path(), "live", &[45; 32], snapshot(), 0);
    continuity
        .checkpoint(&RuntimeRecorder::new(16), Duration::from_secs(1))
        .await
        .unwrap();
    tokio::fs::rename(
        root.path().join("generation-1"),
        root.path().join("generation-100"),
    )
    .await
    .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[45; 32], snapshot(), 100);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err()
        .to_string()
        .contains("does not match signed generation"));
}

#[tokio::test]
async fn validly_signed_broken_predecessor_chain_is_refused() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[46; 32], snapshot(), 0);
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let path = root.path().join("generation-2/manifest.json");
    let mut manifest: CheckpointManifest =
        serde_json::from_slice(&tokio::fs::read(&path).await.unwrap()).unwrap();
    manifest.previous_integrity = Some("substituted-parent".to_owned());
    CheckpointAuthority::from_bytes("live", &[46; 32])
        .sign_manifest(&mut manifest)
        .unwrap();
    tokio::fs::write(&path, serde_json::to_vec(&manifest).unwrap())
        .await
        .unwrap();

    let mut restored = LiveContinuity::new(root.path(), "live", &[46; 32], snapshot(), 0);
    assert!(restored
        .restore_latest(&RuntimeRecorder::new(16))
        .await
        .unwrap_err()
        .to_string()
        .contains("invalid predecessor integrity"));
}

#[tokio::test]
async fn remote_shutdown_request_cannot_bypass_checkpoint() {
    let root = tempfile::tempdir().unwrap();
    let recorder = RuntimeRecorder::new(16);
    let mut continuity = LiveContinuity::new(root.path(), "live", &[44; 32], snapshot(), 0);
    let topology = ComponentRegistry::new().validate().unwrap();
    let handle = Kernel::new(topology, recorder.clone())
        .start()
        .await
        .unwrap();
    let (control, mut requests) = CheckpointingControl::channel(1);
    let caller = tokio::spawn(async move { control.shutdown(Duration::from_secs(1)).await });
    let request = requests.recv().await.unwrap();
    continuity
        .checkpoint(&recorder, Duration::from_secs(1))
        .await
        .unwrap();
    let exit = handle.shutdown(request.grace).await.unwrap();
    request.respond(Ok(exit));
    assert_eq!(caller.await.unwrap().unwrap(), KernelExit::Clean);
    assert!(root.path().join("generation-1/manifest.json").exists());
}

#[test]
fn live_snapshot_schema_is_stable() {
    assert_eq!(snapshot().schema, LIVE_KERNEL_SNAPSHOT_SCHEMA);
}
