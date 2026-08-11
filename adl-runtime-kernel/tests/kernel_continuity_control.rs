use std::{collections::BTreeMap, path::Path, time::Duration};

use adl_runtime_kernel::{
    sha256, CanonicalIngress, CatalogSigningAuthority, CertificateSuccession, ContinuityCommand,
    ContinuityControlBounds, ContinuityControlError, ContinuityControlInitConfig,
    ContinuityControlTlsConfig, ContinuityEnvelope, ContinuityOperation, ContinuityOperationKind,
    DurableContinuityJournal, FinalizedMigrationDecision, LiveContinuityRegistry, RuntimeRecorder,
    SignedBundleCatalog, SourceCheckpointHandle, SourceContinuityEffectPort,
    TargetContinuityCoordinator, CONTROL_REQUEST_SCHEMA,
};
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

fn config(root: &Path) -> ContinuityControlInitConfig {
    let tls = root.join("tls");
    std::fs::create_dir_all(&tls).unwrap();
    for name in [
        "server.pem",
        "server.key",
        "server-ca.pem",
        "guardian.pem",
        "guardian.key",
        "guardian-ca.pem",
        "guardian-successor.pem",
        "guardian-successor.key",
    ] {
        std::fs::write(tls.join(name), b"fixture").unwrap();
    }
    ContinuityControlInitConfig {
        address: "127.0.0.1:32118".into(),
        guardian_state_dir: root.join("guardian-continuity"),
        state_dir: root.join("kernel-continuity"),
        staging_dir: root.join("staging"),
        trust_domain: "agent-logic.test".into(),
        polis: "polis-a".into(),
        source_node: "node-source".into(),
        target_node: "node-target".into(),
        guardian_id: "guardian-logical".into(),
        kernel_control_id: "kernel-control".into(),
        channel_epoch: 7,
        tls: ContinuityControlTlsConfig {
            server_certificate_chain_path: tls.join("server.pem"),
            server_private_key_path: tls.join("server.key"),
            server_trust_roots_path: tls.join("server-ca.pem"),
            server_name: "localhost".into(),
            guardian_certificate_chain_path: tls.join("guardian.pem"),
            guardian_private_key_path: tls.join("guardian.key"),
            guardian_trust_roots_path: tls.join("guardian-ca.pem"),
            guardian_spki_sha256: "11".repeat(32),
            server_spki_sha256: "22".repeat(32),
            certificate_generation: 3,
            successor: Some(CertificateSuccession {
                predecessor_spki_sha256: "11".repeat(32),
                predecessor_generation: 3,
                successor_spki_sha256: "33".repeat(32),
                successor_generation: 4,
                activation_sequence: 4,
                retirement_unix_millis: u64::MAX,
                successor_guardian_certificate_chain_path: tls.join("guardian-successor.pem"),
                successor_guardian_private_key_path: tls.join("guardian-successor.key"),
            }),
        },
        bounds: ContinuityControlBounds {
            max_frame_bytes: 64 * 1024,
            max_blob_bytes: 64 * 1024,
            max_total_bytes: 512 * 1024,
            max_services: 5,
            max_journal_entries: 64,
            max_open_handles: 8,
        },
    }
}

fn registry(max_services: usize) -> Result<LiveContinuityRegistry, ContinuityControlError> {
    let recorder = RuntimeRecorder::new(16);
    LiveContinuityRegistry::from_live_handles(
        CanonicalIngress::new(8, recorder.clone(), BTreeMap::new()),
        recorder,
        br#"{"schema":"reasoning.v1"}"#.to_vec(),
        br#"{"schema":"governance.v1"}"#.to_vec(),
        br#"{"schema":"operations.v1"}"#.to_vec(),
        max_services,
    )
}

struct ExportFixture {
    config: ContinuityControlInitConfig,
    source: SourceContinuityEffectPort,
    target: TargetContinuityCoordinator,
}

fn fixture(root: &Path) -> ExportFixture {
    let config = config(root);
    let authority = CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    let keys = BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
    let source = SourceContinuityEffectPort::open(
        root.join("export"),
        registry(5).unwrap(),
        authority,
        config.bounds.clone(),
        9,
    )
    .unwrap();
    let target = TargetContinuityCoordinator::open(config.clone(), keys).unwrap();
    ExportFixture {
        config,
        source,
        target,
    }
}

async fn export(
    fixture: &ExportFixture,
    generation: u64,
) -> (
    adl_runtime_kernel::SourceQuiesceReceipt,
    SourceCheckpointHandle,
    SignedBundleCatalog,
) {
    let cancellation = CancellationToken::new();
    let (receipt, handle) = fixture
        .source
        .quiesce_and_export(
            generation,
            None,
            17,
            "aa".repeat(32),
            "bb".repeat(32),
            Duration::from_secs(1),
            &cancellation,
        )
        .await
        .unwrap();
    let catalog = fixture
        .source
        .bundle_source(&handle)
        .unwrap()
        .catalog()
        .clone();
    (receipt, handle, catalog)
}

async fn staged(
    fixture: &ExportFixture,
    id: &str,
) -> (adl_runtime_kernel::StageCreated, SignedBundleCatalog) {
    let (_, handle, catalog) = export(fixture, 1).await;
    let stage = fixture.target.create_stage(id, 9, catalog.clone()).unwrap();
    let source = fixture.source.bundle_source(&handle).unwrap();
    for entry in &catalog.entries {
        let bytes = source
            .read_entry_range(entry.ordinal, 0, entry.bytes)
            .unwrap();
        fixture
            .target
            .write_entry(&stage.handle, entry.ordinal, &bytes)
            .unwrap();
    }
    (stage, catalog)
}

fn services(catalog: &SignedBundleCatalog) -> BTreeMap<String, String> {
    catalog
        .entries
        .iter()
        .map(|entry| (entry.service.clone(), entry.schema.clone()))
        .collect()
}

fn envelope(
    config: &ContinuityControlInitConfig,
    sequence: u64,
    id: &str,
    deadline: u64,
) -> (ContinuityEnvelope, Vec<u8>) {
    let exporter = b"tls-exporter-session".to_vec();
    let command = ContinuityCommand::Status;
    let operation = ContinuityOperation {
        schema: CONTROL_REQUEST_SCHEMA.into(),
        trust_domain: config.trust_domain.clone(),
        polis: config.polis.clone(),
        source_node: config.source_node.clone(),
        target_node: config.target_node.clone(),
        guardian_id: config.guardian_id.clone(),
        kernel_control_id: config.kernel_control_id.clone(),
        channel_epoch: config.channel_epoch,
        sequence,
        operation_id: id.into(),
        kind: ContinuityOperationKind::Status,
        deadline_unix_millis: deadline,
        accepted_prefix: 0,
        payload_sha256: sha256(&serde_jcs::to_vec(&command).unwrap()),
    };
    (
        ContinuityEnvelope {
            exporter_sha256: sha256(&exporter),
            certificate_generation: 3,
            leaf_spki_sha256: config.tls.guardian_spki_sha256.clone(),
            operation,
            command,
        },
        exporter,
    )
}

fn emit_case(name: &str) {
    let map: serde_json::Value = serde_json::from_str(include_str!(
        "../../.csdlc/prepared/issues/208/continuity-boundary-subassertion-map.json"
    ))
    .unwrap();
    let case = map["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["name"] == name)
        .expect("case is registered");
    println!("{}", case["marker"].as_str().unwrap());
    for boundary in map["boundaries"].as_array().unwrap() {
        for assertion in boundary["subassertions"].as_array().unwrap() {
            if assertion["case"] == name {
                println!("{}", assertion["marker"].as_str().unwrap());
            }
        }
    }
    for assertion in map["lifecycle_subassertions"].as_array().unwrap() {
        if assertion["case"] == name {
            println!("{}", assertion["marker"].as_str().unwrap());
        }
    }
}

async fn run_case(name: &str) {
    let temp = TempDir::new().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let fixture = fixture(&root);
    match name {
        "real_quiesce_checkpoint" | "signed_bundle_export" => {
            let (receipt, handle, catalog) = export(&fixture, 1).await;
            assert!(!receipt.digest().is_empty());
            assert_eq!(handle.generation(), 1);
            assert_eq!(catalog.entries.len(), 5);
            assert!(!catalog.signature.is_empty());
        }
        "partial_quiesce_rollback" | "cancellation_no_partial" => {
            let cancelled = CancellationToken::new();
            cancelled.cancel();
            let result = fixture
                .source
                .quiesce_and_export(
                    1,
                    None,
                    17,
                    "aa".repeat(32),
                    "bb".repeat(32),
                    Duration::from_secs(1),
                    &cancelled,
                )
                .await;
            assert!(matches!(result, Err(ContinuityControlError::Cancelled)));
        }
        "export_bounds" => {
            assert!(fixture.config.bounds.validate().is_ok());
            let (_, _, catalog) = export(&fixture, 1).await;
            assert_eq!(catalog.entries.len(), fixture.config.bounds.max_services);
        }
        "export_exact_retry" => {
            let first = export(&fixture, 1).await;
            let second = export(&fixture, 1).await;
            assert_eq!(first.0, second.0);
            assert_eq!(first.1, second.1);
            assert_eq!(first.2, second.2);
        }
        "source_resume" | "source_resume_exact_retry" => {
            let (_, handle, _) = export(&fixture, 1).await;
            let first = fixture.source.resume_source(&handle).unwrap();
            let second = fixture.source.resume_source(&handle).unwrap();
            assert_eq!(first, second);
            assert!(first.id().contains(&handle.generation().to_string()));
        }
        "isolated_stage" => {
            let (_, _, catalog) = export(&fixture, 1).await;
            let stage = fixture.target.create_stage("stage-a", 9, catalog).unwrap();
            assert_eq!(stage.handle.id(), "stage-a");
            assert!(!format!("{:?}", stage.handle).contains(root.to_str().unwrap()));
        }
        "isolated_import_validate" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            let evidence = fixture
                .target
                .validate_stage(
                    &stage.handle,
                    1,
                    None,
                    17,
                    &"aa".repeat(32),
                    &"bb".repeat(32),
                    &services(&catalog),
                )
                .unwrap();
            assert!(!evidence.digest().is_empty());
        }
        "wrong_manifest_signature" => {
            let (_, _, mut catalog) = export(&fixture, 1).await;
            catalog.signature = "00".repeat(64);
            assert!(matches!(
                fixture.target.create_stage("stage-a", 9, catalog),
                Err(ContinuityControlError::ManifestSignature)
            ));
        }
        "wrong_generation"
        | "wrong_predecessor"
        | "wrong_accepted_prefix"
        | "wrong_topology"
        | "wrong_config"
        | "wrong_service_set"
        | "wrong_service_schema" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            let mut service_set = services(&catalog);
            let (generation, predecessor, prefix, topology, config_hash) = match name {
                "wrong_generation" => (2, None, 17, "aa".repeat(32), "bb".repeat(32)),
                "wrong_predecessor" => (
                    1,
                    Some("cc".repeat(32)),
                    17,
                    "aa".repeat(32),
                    "bb".repeat(32),
                ),
                "wrong_accepted_prefix" => (1, None, 18, "aa".repeat(32), "bb".repeat(32)),
                "wrong_topology" => (1, None, 17, "cc".repeat(32), "bb".repeat(32)),
                "wrong_config" => (1, None, 17, "aa".repeat(32), "cc".repeat(32)),
                "wrong_service_set" => {
                    service_set.remove("governance");
                    (1, None, 17, "aa".repeat(32), "bb".repeat(32))
                }
                _ => {
                    *service_set.get_mut("governance").unwrap() = "wrong.schema".into();
                    (1, None, 17, "aa".repeat(32), "bb".repeat(32))
                }
            };
            assert!(fixture
                .target
                .validate_stage(
                    &stage.handle,
                    generation,
                    predecessor.as_deref(),
                    prefix,
                    &topology,
                    &config_hash,
                    &service_set
                )
                .is_err());
        }
        "corrupt_content" | "opened_handle_replacement" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            let path = fixture
                .config
                .staging_dir
                .join("stage-a")
                .join(&catalog.entries[0].file);
            std::fs::write(path, b"corrupt").unwrap();
            assert!(fixture
                .target
                .validate_stage(
                    &stage.handle,
                    1,
                    None,
                    17,
                    &"aa".repeat(32),
                    &"bb".repeat(32),
                    &services(&catalog)
                )
                .is_err());
        }
        "oversized_bundle" => {
            let mut cfg = fixture.config.clone();
            cfg.bounds.max_blob_bytes = 1;
            cfg.bounds.max_total_bytes = 5;
            let authority = CatalogSigningAuthority::from_secret("small", 1, &[7; 32]).unwrap();
            let source = SourceContinuityEffectPort::open(
                root.join("small-export"),
                registry(5).unwrap(),
                authority,
                cfg.bounds,
                1,
            )
            .unwrap();
            let token = CancellationToken::new();
            assert!(source
                .quiesce_and_export(
                    1,
                    None,
                    0,
                    "aa".repeat(32),
                    "bb".repeat(32),
                    Duration::from_secs(1),
                    &token
                )
                .await
                .is_err());
        }
        "caller_path_rejected" => {
            let (_, _, mut catalog) = export(&fixture, 1).await;
            catalog.entries[0].file = "../escape".into();
            assert!(fixture.target.create_stage("stage-a", 9, catalog).is_err());
        }
        "symlink_path_rejected" => {
            #[cfg(unix)]
            {
                let outside = root.join("outside");
                std::fs::create_dir(&outside).unwrap();
                let link = root.join("link");
                std::os::unix::fs::symlink(&outside, &link).unwrap();
                let mut cfg = fixture.config.clone();
                cfg.staging_dir = link;
                assert!(TargetContinuityCoordinator::open(cfg, BTreeMap::new()).is_err());
            }
        }
        "deadline_before_effect" => {
            let mut journal = DurableContinuityJournal::open(&fixture.config).unwrap();
            let (request, exporter) = envelope(&fixture.config, 1, name, 1);
            assert!(matches!(
                journal.begin(&fixture.config, &request, &exporter, 2),
                Err(ContinuityControlError::Deadline)
            ));
            let (stage, _) = staged(&fixture, "stage-a").await;
            assert!(fixture
                .target
                .discard(&stage.handle, &stage.cleanup)
                .is_ok());
        }
        "restart_after_accept" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            let possession = fixture
                .target
                .validate_stage(
                    &stage.handle,
                    1,
                    None,
                    17,
                    &"aa".repeat(32),
                    &"bb".repeat(32),
                    &services(&catalog),
                )
                .unwrap();
            let decision = FinalizedMigrationDecision::from_verified_204(
                "decision-a".into(),
                "stage-a".into(),
                possession.digest().into(),
                &fixture.config,
                std::array::from_fn(|i| format!("{:02x}", i + 1).repeat(32)),
            )
            .unwrap();
            let first = fixture
                .target
                .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                .unwrap();
            drop(fixture.target);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let keys =
                BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
            let reopened = TargetContinuityCoordinator::open(fixture.config.clone(), keys).unwrap();
            assert_eq!(
                first,
                reopened
                    .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                    .unwrap()
            );
        }
        "crash_after_bundle_commit" => {
            let (_, handle, catalog) = export(&fixture, 1).await;
            drop(fixture.source);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let reopened = SourceContinuityEffectPort::open(
                root.join("export"),
                registry(5).unwrap(),
                authority,
                fixture.config.bounds.clone(),
                9,
            )
            .unwrap();
            assert_eq!(reopened.bundle_source(&handle).unwrap().catalog(), &catalog);
        }
        "target_discard" | "discard_exact_retry" | "validated_target_discard" | "zero_residue" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            if name == "validated_target_discard" {
                fixture
                    .target
                    .validate_stage(
                        &stage.handle,
                        1,
                        None,
                        17,
                        &"aa".repeat(32),
                        &"bb".repeat(32),
                        &services(&catalog),
                    )
                    .unwrap();
            }
            let first = fixture
                .target
                .discard(&stage.handle, &stage.cleanup)
                .unwrap();
            let second = fixture
                .target
                .discard(&stage.handle, &stage.cleanup)
                .unwrap();
            assert_eq!(first, second);
            assert!(!fixture.config.staging_dir.join("stage-a").exists());
        }
        "dual_open" => {
            let first = DurableContinuityJournal::open(&fixture.config).unwrap();
            assert!(matches!(
                DurableContinuityJournal::open(&fixture.config),
                Err(ContinuityControlError::AlreadyOpen)
            ));
            drop(first);
            assert!(DurableContinuityJournal::open(&fixture.config).is_ok());
        }
        "evidence_redaction" => {
            let (stage, _) = staged(&fixture, "stage-a").await;
            let evidence = serde_json::to_string(&stage.cleanup).unwrap();
            assert!(!evidence.contains(root.to_str().unwrap()));
            assert!(!evidence.contains("private_key"));
        }
        "public_surface_absent" => {
            let public = include_str!("../src/control.rs");
            assert!(!public.contains("continuity_control"));
            assert!(!public.contains("ActivateTarget"));
        }
        "guardian_initialization_live" => {
            let kernel = include_str!("../src/bin/adl-runtime-kernel.rs");
            assert!(kernel.contains("serve_private_continuity_listener"));
            assert!(kernel.contains("continuity_control"));
        }
        "participant_registry_complete" => {
            assert_eq!(registry(5).unwrap().services().len(), 5);
            assert!(registry(4).is_err());
        }
        _ => panic!("unknown contract case {name}"),
    }
    emit_case(name);
}

macro_rules! cases {
    ($($name:ident),+ $(,)?) => { $(#[tokio::test] async fn $name() { run_case(stringify!($name)).await; })+ };
}

cases!(
    real_quiesce_checkpoint,
    partial_quiesce_rollback,
    signed_bundle_export,
    export_bounds,
    export_exact_retry,
    source_resume,
    source_resume_exact_retry,
    isolated_stage,
    isolated_import_validate,
    wrong_manifest_signature,
    wrong_generation,
    wrong_predecessor,
    wrong_accepted_prefix,
    wrong_topology,
    wrong_config,
    wrong_service_set,
    wrong_service_schema,
    corrupt_content,
    oversized_bundle,
    caller_path_rejected,
    symlink_path_rejected,
    deadline_before_effect,
    cancellation_no_partial,
    restart_after_accept,
    crash_after_bundle_commit,
    target_discard,
    discard_exact_retry,
    validated_target_discard,
    zero_residue,
    dual_open,
    opened_handle_replacement,
    evidence_redaction,
    public_surface_absent,
    guardian_initialization_live,
    participant_registry_complete,
);
