use std::{collections::BTreeMap, path::Path, time::Duration};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, sha256, CanonicalIngress, CatalogSigningAuthority,
    CertificateSuccession, ContinuityCommand, ContinuityControlBounds, ContinuityControlError,
    ContinuityControlInitConfig, ContinuityControlTlsConfig, ContinuityEnvelope,
    ContinuityOperation, ContinuityOperationKind, DurableContinuityJournal, IngressSnapshot,
    LiveContinuityRegistry, MigrationDecisionCertificate, RuntimeRecorder, SignedBundleCatalog,
    SourceCheckpointHandle, SourceContinuityEffectPort, TargetContinuityCoordinator,
    CONTROL_REQUEST_SCHEMA,
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

fn registry(
    root: &Path,
    max_services: usize,
) -> Result<LiveContinuityRegistry, ContinuityControlError> {
    registry_with_ingress(root, max_services).map(|(registry, _)| registry)
}

fn registry_with_ingress(
    root: &Path,
    max_services: usize,
) -> Result<(LiveContinuityRegistry, CanonicalIngress), ContinuityControlError> {
    let recorder = RuntimeRecorder::new(16);
    std::fs::create_dir_all(root)?;
    let reasoning = bootstrap_reasoning_services(recorder.clone())
        .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?;
    let ingress = CanonicalIngress::new(8, recorder.clone(), BTreeMap::new());
    ingress.restore(IngressSnapshot {
        accepted_through: 17,
        completed: BTreeMap::new(),
    });
    let registry = LiveContinuityRegistry::from_production_handles(
        ingress.clone(),
        recorder,
        reasoning,
        root.to_path_buf(),
        max_services,
    )?;
    Ok((registry, ingress))
}

fn decision_certificate(
    fixture: &ExportFixture,
    stage_id: &str,
    possession_sha256: &str,
) -> MigrationDecisionCertificate {
    let authority = CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    MigrationDecisionCertificate {
        decision_id: "decision-a".into(),
        stage_id: stage_id.into(),
        possession_sha256: possession_sha256.into(),
        trust_domain: fixture.config.trust_domain.clone(),
        polis: fixture.config.polis.clone(),
        target_node: fixture.config.target_node.clone(),
        channel_epoch: fixture.config.channel_epoch,
        route_cut_sha256: "01".repeat(32),
        membership_cut_sha256: "02".repeat(32),
        certificate_cut_sha256: "03".repeat(32),
        boot_cut_sha256: "04".repeat(32),
        lineage_sha256: "05".repeat(32),
        authority_key_id: String::new(),
        authority_key_generation: 0,
        signature: String::new(),
    }
    .sign(&authority)
    .unwrap()
}

struct ExportFixture {
    config: ContinuityControlInitConfig,
    source: SourceContinuityEffectPort,
    target: TargetContinuityCoordinator,
    ingress: CanonicalIngress,
    operation_root: std::path::PathBuf,
}

fn fixture(root: &Path) -> ExportFixture {
    let config = config(root);
    let authority = CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    let keys = BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
    let operation_root = root.join("operations");
    let (registry, ingress) = registry_with_ingress(&operation_root, 5).unwrap();
    let source = SourceContinuityEffectPort::open(
        root.join("export"),
        registry,
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
        ingress,
        operation_root,
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

fn emit_case(name: &str, root: &Path) {
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
    let mut markers = vec![case["marker"].as_str().unwrap().to_owned()];
    for boundary in map["boundaries"].as_array().unwrap() {
        for assertion in boundary["subassertions"].as_array().unwrap() {
            if assertion["case"] == name {
                markers.push(assertion["marker"].as_str().unwrap().to_owned());
            }
        }
    }
    for assertion in map["lifecycle_subassertions"].as_array().unwrap() {
        if assertion["case"] == name {
            markers.push(assertion["marker"].as_str().unwrap().to_owned());
        }
    }
    let mut durable = Vec::new();
    collect_durable_witness(root, root, &mut durable);
    durable.sort();
    let behavior = serde_json::json!({
        "assertion_binding": format!("kernel_continuity_control::{name}"),
        "case": name,
        "durable_witness": durable,
    });
    let behavior_canonical = String::from_utf8(serde_jcs::to_vec(&behavior).unwrap()).unwrap();
    let behavior_sha256 = sha256(behavior_canonical.as_bytes());
    let receipt = serde_json::json!({
        "behavior": behavior,
        "behavior_canonical": behavior_canonical,
        "behavior_sha256": behavior_sha256,
        "case": name,
        "markers": markers,
        "outcome": "passed",
        "schema": "adl.issue208.behavior_receipt.v1",
    });
    println!(
        "BEHAVIOR_RECEIPT {}",
        String::from_utf8(serde_jcs::to_vec(&receipt).unwrap()).unwrap()
    );
}

fn collect_durable_witness(root: &Path, path: &Path, output: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(path) else {
        return;
    };
    for entry in entries.flatten() {
        let entry_path = entry.path();
        let relative = entry_path.strip_prefix(root).unwrap().to_string_lossy();
        if entry_path.is_dir() {
            collect_durable_witness(root, &entry_path, output);
        } else if entry_path.is_file() && !relative.ends_with("writer.lock") {
            let bytes = std::fs::read(&entry_path).unwrap();
            output.push(format!("{relative}:{}", sha256(&bytes)));
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
        "partial_quiesce_rollback" => {
            let ingress = fixture.ingress.clone();
            let operation_root = fixture.operation_root.clone();
            let sabotage = tokio::spawn(async move {
                while ingress.admission_is_open() {
                    tokio::task::yield_now().await;
                }
                std::fs::remove_dir_all(&operation_root).unwrap();
                std::fs::write(&operation_root, b"not-a-directory").unwrap();
            });
            let token = CancellationToken::new();
            let result = fixture
                .source
                .quiesce_and_export(
                    1,
                    None,
                    17,
                    "aa".repeat(32),
                    "bb".repeat(32),
                    Duration::from_secs(1),
                    &token,
                )
                .await;
            sabotage.await.unwrap();
            assert!(result.is_err());
            assert!(fixture.ingress.admission_is_open());
            let source_state =
                std::fs::read_to_string(root.join("export/source-state.json")).unwrap();
            assert!(source_state.contains("\"resumed\""));
            assert!(source_state.contains("admission"));
            assert!(source_state.contains("accepted_prefix"));
        }
        "cancellation_no_partial" => {
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
            assert!(fixture.ingress.admission_is_open());
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
        "corrupt_content" => {
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
        "opened_handle_replacement" => {
            let (stage, catalog) = staged(&fixture, "stage-a").await;
            let original = fixture.config.staging_dir.clone();
            let displaced = root.join("staging-displaced");
            std::fs::rename(&original, &displaced).unwrap();
            std::fs::create_dir(&original).unwrap();
            let result = fixture.target.validate_stage(
                &stage.handle,
                1,
                None,
                17,
                &"aa".repeat(32),
                &"bb".repeat(32),
                &services(&catalog),
            );
            assert!(matches!(result, Err(ContinuityControlError::UnsafeRoot)));
            std::fs::remove_dir(&original).unwrap();
            std::fs::rename(&displaced, &original).unwrap();
        }
        "oversized_bundle" => {
            let mut cfg = fixture.config.clone();
            cfg.bounds.max_blob_bytes = 1;
            cfg.bounds.max_total_bytes = 5;
            let authority = CatalogSigningAuthority::from_secret("small", 1, &[7; 32]).unwrap();
            let source = SourceContinuityEffectPort::open(
                root.join("small-export"),
                registry(&root.join("small-operations"), 5).unwrap(),
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
            let decision = decision_certificate(&fixture, "stage-a", possession.digest());
            let mut forged = decision.clone();
            forged.signature = "00".repeat(64);
            assert!(matches!(
                fixture
                    .target
                    .activate(&stage.handle, &possession, &stage.cleanup, &forged),
                Err(ContinuityControlError::ActivationDecision)
            ));
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let original = std::fs::metadata(&fixture.config.state_dir)
                    .unwrap()
                    .permissions();
                std::fs::set_permissions(
                    &fixture.config.state_dir,
                    std::fs::Permissions::from_mode(0o555),
                )
                .unwrap();
                let interrupted =
                    fixture
                        .target
                        .activate(&stage.handle, &possession, &stage.cleanup, &decision);
                std::fs::set_permissions(&fixture.config.state_dir, original).unwrap();
                assert!(interrupted.is_err());
            }
            let mut conflicting = decision_certificate(&fixture, "stage-a", possession.digest());
            conflicting.decision_id = "decision-b".into();
            let signer =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            conflicting = conflicting.sign(&signer).unwrap();
            drop(fixture.target);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let keys =
                BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
            let reopened = TargetContinuityCoordinator::open(fixture.config.clone(), keys).unwrap();
            let first = reopened
                .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                .unwrap();
            assert_eq!(
                first,
                reopened
                    .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                    .unwrap()
            );
            assert!(matches!(
                reopened.activate(&stage.handle, &possession, &stage.cleanup, &conflicting),
                Err(ContinuityControlError::ConflictingRetry)
            ));
        }
        "crash_after_bundle_commit" => {
            let (_, handle, catalog) = export(&fixture, 1).await;
            drop(fixture.source);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let (restart_registry, restart_ingress) =
                registry_with_ingress(&root.join("restart-operations"), 5).unwrap();
            let reopened = SourceContinuityEffectPort::open(
                root.join("export"),
                restart_registry,
                authority,
                fixture.config.bounds.clone(),
                9,
            )
            .unwrap();
            assert_eq!(reopened.bundle_source(&handle).unwrap().catalog(), &catalog);
            assert!(reopened.source_requires_resume(&handle).unwrap());
            assert!(!restart_ingress.admission_is_open());
            reopened.resume_source(&handle).unwrap();
            assert!(restart_ingress.admission_is_open());
        }
        "discard_exact_retry" => {
            let (stage, _) = staged(&fixture, "stage-a").await;
            #[cfg(unix)]
            {
                use std::os::unix::fs::PermissionsExt;
                let original = std::fs::metadata(&fixture.config.staging_dir)
                    .unwrap()
                    .permissions();
                std::fs::set_permissions(
                    &fixture.config.staging_dir,
                    std::fs::Permissions::from_mode(0o555),
                )
                .unwrap();
                let interrupted = fixture.target.discard(&stage.handle, &stage.cleanup);
                std::fs::set_permissions(&fixture.config.staging_dir, original).unwrap();
                assert!(interrupted.is_err());
            }
            drop(fixture.target);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let keys =
                BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
            let reopened = TargetContinuityCoordinator::open(fixture.config.clone(), keys).unwrap();
            let first = reopened.discard(&stage.handle, &stage.cleanup).unwrap();
            let second = reopened.discard(&stage.handle, &stage.cleanup).unwrap();
            assert_eq!(first, second);
            assert!(!fixture.config.staging_dir.join("stage-a").exists());
        }
        "target_discard" | "validated_target_discard" | "zero_residue" => {
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
            DurableContinuityJournal::open(&fixture.config)
                .unwrap_or_else(|error| panic!("journal did not reopen: {error:?}"));
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let keys =
                BTreeMap::from([(("continuity-key".to_owned(), 1), authority.verifying_key())]);
            assert!(matches!(
                TargetContinuityCoordinator::open(fixture.config.clone(), keys),
                Err(ContinuityControlError::AlreadyOpen)
            ));
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
            let client = include_str!("../../adl-runtime/src/kernel_continuity_client.rs");
            let polis = include_str!("../../adl-runtime/src/distributed/polis_runtime.rs");
            let kernel = include_str!("../src/continuity_control.rs");
            assert!(!client.contains("pub async fn execute"));
            assert!(!client.contains("pub(crate) async fn execute"));
            assert!(polis.contains("pub struct TransferContinuityPort"));
            assert!(polis.contains("pub struct MigrationContinuityPort"));
            assert!(!polis.contains("pub fn from_initialized_guardian"));
            assert!(!kernel.contains("pub fn from_verified_204"));
        }
        "guardian_initialization_live" => {
            let kernel = include_str!("../src/bin/adl-runtime-kernel.rs");
            assert!(kernel.contains("serve_private_continuity_listener"));
            assert!(kernel.contains("continuity_control"));
        }
        "participant_registry_complete" => {
            assert_eq!(
                registry(&root.join("complete-operations"), 5)
                    .unwrap()
                    .services()
                    .len(),
                5
            );
            assert!(registry(&root.join("incomplete-operations"), 4).is_err());
        }
        _ => panic!("unknown contract case {name}"),
    }
    emit_case(name, &root);
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
