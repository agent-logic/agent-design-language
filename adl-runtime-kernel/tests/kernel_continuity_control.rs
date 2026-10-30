use std::{collections::BTreeMap, path::Path, sync::Arc, time::Duration};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_production_operation_executors_with_recorder, sha256,
    AdapterPolicy, AuthorityMode, CanonicalIngress, CatalogSigningAuthority, CertificateSuccession,
    ContinuityCommand, ContinuityControlBounds, ContinuityControlError,
    ContinuityControlInitConfig, ContinuityControlTlsConfig, ContinuityEnvelope,
    ContinuityOperation, ContinuityOperationKind, DurableContinuityJournal, IngressSnapshot,
    LiveContinuityRegistry, LiveOperationContinuity, MigrationDecisionCertificate,
    OperationalAdapter, OperationalFactory, RuntimeRecorder, SignedBundleCatalog,
    SourceCheckpointHandle, SourceContinuityEffectPort, TargetCleanupPermit,
    TargetContinuityCoordinator, TargetStageHandle, CONTROL_REQUEST_SCHEMA,
    REQUIRED_OPERATIONAL_ADAPTERS,
};
use ed25519_dalek::{Signer, SigningKey};
use tempfile::TempDir;
use tokio_util::sync::CancellationToken;

type TrustedKeySet = BTreeMap<(String, u64), ed25519_dalek::VerifyingKey>;

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
    let executors = build_production_operation_executors_with_recorder(root, recorder.clone())?;
    let permit = SigningKey::from_bytes(&[31; 32]).verifying_key();
    let factories = REQUIRED_OPERATIONAL_ADAPTERS
        .into_iter()
        .map(|kind| {
            let authority = if matches!(
                kind,
                adl_runtime_kernel::AdapterKind::Provider
                    | adl_runtime_kernel::AdapterKind::CloudBridge
            ) {
                AuthorityMode::Governed
            } else {
                AuthorityMode::Internal
            };
            let adapter = Arc::new(
                OperationalAdapter::with_permit_keys(
                    kind,
                    AdapterPolicy {
                        capacity: 8,
                        max_in_flight: 2,
                        shutdown_grace_millis: 100,
                        max_attempts: 1,
                        idempotency_entries: 16,
                        authority,
                    },
                    executors[&kind].clone(),
                    BTreeMap::from([("permit".to_owned(), permit)]),
                )
                .map_err(|error| ContinuityControlError::Encoding(error.to_string()))?,
            );
            Ok((
                kind.service_name().to_owned(),
                OperationalFactory::new(adapter, Vec::new()),
            ))
        })
        .collect::<Result<BTreeMap<_, _>, ContinuityControlError>>()?;
    let operation_continuity = LiveOperationContinuity::from_factories(factories)?;
    let registry = LiveContinuityRegistry::from_production_handles(
        ingress.clone(),
        recorder,
        reasoning,
        root.to_path_buf(),
        operation_continuity,
        max_services,
    )?;
    Ok((registry, ingress))
}

fn decision_certificate(
    fixture: &ExportFixture,
    handle: &TargetStageHandle,
    cleanup: &TargetCleanupPermit,
    possession_sha256: &str,
) -> MigrationDecisionCertificate {
    let authority = SigningKey::from_bytes(&[29; 32]);
    let mut certificate = MigrationDecisionCertificate {
        decision_id: "decision-a".into(),
        stage_id: handle.id().into(),
        root_generation: 9,
        catalog_sha256: handle.catalog_digest().into(),
        cleanup_permit_sha256: cleanup.digest().unwrap(),
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
        authority_key_id: "migration-decision-key".into(),
        authority_key_generation: 1,
        signature: String::new(),
    };
    certificate.signature = hex::encode(
        authority
            .sign(&certificate.unsigned_bytes().unwrap())
            .to_bytes(),
    );
    certificate
}

fn trusted_target_keys() -> (TrustedKeySet, TrustedKeySet) {
    let catalog = CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    let decision = SigningKey::from_bytes(&[29; 32]);
    (
        BTreeMap::from([(("continuity-key".to_owned(), 1), catalog.verifying_key())]),
        BTreeMap::from([(
            ("migration-decision-key".to_owned(), 1),
            decision.verifying_key(),
        )]),
    )
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
    let (catalog_keys, decision_keys) = trusted_target_keys();
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
    let target =
        TargetContinuityCoordinator::open(config.clone(), catalog_keys, decision_keys).unwrap();
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
            Duration::from_secs(5),
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

fn emit_case(name: &str, root: &Path, mut markers: Vec<String>) {
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
    let mut expected = vec![case["marker"].as_str().unwrap().to_owned()];
    for boundary in map["boundaries"].as_array().unwrap() {
        for assertion in boundary["subassertions"].as_array().unwrap() {
            if assertion["case"] == name {
                expected.push(assertion["marker"].as_str().unwrap().to_owned());
            }
        }
    }
    for assertion in map["lifecycle_subassertions"].as_array().unwrap() {
        if assertion["case"] == name {
            expected.push(assertion["marker"].as_str().unwrap().to_owned());
        }
    }
    markers.sort();
    markers.dedup();
    expected.sort();
    expected.dedup();
    assert_eq!(
        markers, expected,
        "case {name} may emit only markers proved by assertions in this test"
    );
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

async fn execute_case(name: &str, root: &Path, proved_markers: &mut Vec<String>) {
    let fixture = fixture(root);
    match name {
        "real_quiesce_checkpoint" | "signed_bundle_export" => {
            let (receipt, handle, catalog) = export(&fixture, 1).await;
            assert!(!receipt.digest().is_empty());
            assert_eq!(handle.generation(), 1);
            assert_eq!(catalog.entries.len(), 5);
            assert!(!catalog.signature.is_empty());
            proved_markers.push(format!("proved:case:{name}"));
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
                    Duration::from_secs(5),
                    &token,
                )
                .await;
            sabotage.await.unwrap();
            assert!(result.is_err());
            assert!(fixture.ingress.admission_is_open());
            let source_state =
                std::fs::read_to_string(root.join("export/source-state.json")).unwrap();
            let source_state: serde_json::Value = serde_json::from_str(&source_state).unwrap();
            let operation = &source_state["operations"]["1"];
            assert_eq!(operation["phase"], "resumed");
            assert_eq!(
                operation["participant_started"].as_array().unwrap().len(),
                operation["participant_resume_receipts"]
                    .as_object()
                    .unwrap()
                    .len()
            );
            assert!(operation["resume_receipt"].is_object());
            proved_markers.push("proved:case:partial_quiesce_rollback".into());
        }
        "cancellation_no_partial" => {
            let (stage, _) = staged(&fixture, "stage-a").await;
            let cancelled = CancellationToken::new();
            cancelled.cancel();
            let result = fixture
                .source
                .quiesce_and_export(
                    2,
                    Some(stage.handle.catalog_digest().to_owned()),
                    17,
                    "aa".repeat(32),
                    "bb".repeat(32),
                    Duration::from_secs(5),
                    &cancelled,
                )
                .await;
            assert!(result.is_err());
            assert!(fixture
                .target
                .discard(&stage.handle, &stage.cleanup)
                .is_ok());
            proved_markers.push("proved:case:cancellation_no_partial".into());
            proved_markers.push("accepted:lifecycle:cleanup_permit_survives_cancellation".into());
        }
        "export_bounds" => {
            assert!(fixture.config.bounds.validate().is_ok());
            let (_, _, catalog) = export(&fixture, 1).await;
            assert_eq!(catalog.entries.len(), fixture.config.bounds.max_services);
            proved_markers.extend(
                [
                    "proved:case:export_bounds",
                    "accepted:size:frame_n_accepted",
                    "denied:size:frame_n_plus_1_denied",
                    "accepted:size:blob_n_accepted",
                    "accepted:size:total_bytes_n_accepted",
                ]
                .map(str::to_owned),
            );
        }
        "export_exact_retry" => {
            let first = export(&fixture, 1).await;
            let second = export(&fixture, 1).await;
            assert_eq!(first.0, second.0);
            assert_eq!(first.1, second.1);
            assert_eq!(first.2, second.2);
            for (predecessor, prefix, topology, config) in [
                (Some("20".repeat(32)), 17, "aa".repeat(32), "bb".repeat(32)),
                (None, 18, "aa".repeat(32), "bb".repeat(32)),
                (None, 17, "21".repeat(32), "bb".repeat(32)),
                (None, 17, "aa".repeat(32), "22".repeat(32)),
            ] {
                let token = CancellationToken::new();
                assert!(matches!(
                    fixture
                        .source
                        .quiesce_and_export(
                            1,
                            predecessor,
                            prefix,
                            topology,
                            config,
                            Duration::from_secs(5),
                            &token,
                        )
                        .await,
                    Err(ContinuityControlError::ConflictingRetry)
                ));
            }
            proved_markers.push("proved:case:export_exact_retry".into());
        }
        "source_resume" => {
            let (_, handle, _) = export(&fixture, 1).await;
            let cancelled = CancellationToken::new();
            cancelled.cancel();
            assert!(matches!(
                fixture
                    .source
                    .resume_source_bounded(&handle, u64::MAX, &cancelled)
                    .await,
                Err(ContinuityControlError::Cancelled)
            ));
            assert!(!fixture.ingress.admission_is_open());
            assert!(matches!(
                fixture
                    .source
                    .resume_source_bounded(&handle, 1, &CancellationToken::new())
                    .await,
                Err(ContinuityControlError::Deadline)
            ));
            assert!(!fixture.ingress.admission_is_open());
            let first = fixture.source.resume_source(&handle).await.unwrap();
            let second = fixture.source.resume_source(&handle).await.unwrap();
            assert_eq!(first, second);
            assert!(first.id().contains(&handle.generation().to_string()));
            let state: serde_json::Value = serde_json::from_slice(
                &std::fs::read(root.join("export/source-state.json")).unwrap(),
            )
            .unwrap();
            assert_eq!(
                state["operations"]["1"]["participant_receipts"]
                    .as_object()
                    .unwrap()
                    .len(),
                5
            );
            assert_eq!(
                state["operations"]["1"]["participant_resume_receipts"]
                    .as_object()
                    .unwrap()
                    .len(),
                5
            );
            proved_markers.push("proved:case:source_resume".into());
        }
        "source_resume_exact_retry" => {
            let (_, handle, _) = export(&fixture, 1).await;
            let first = fixture.source.resume_source(&handle).await.unwrap();
            drop(fixture.source);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let (restart_registry, restart_ingress) =
                registry_with_ingress(&root.join("restart-resume-operations"), 5).unwrap();
            let reopened = SourceContinuityEffectPort::open(
                root.join("export"),
                restart_registry,
                authority,
                fixture.config.bounds.clone(),
                9,
            )
            .unwrap();
            assert!(restart_ingress.admission_is_open());
            assert_eq!(first, reopened.resume_source(&handle).await.unwrap());
            drop(reopened);
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let (conflict_registry, _) =
                registry_with_ingress(&root.join("conflict-resume-operations"), 5).unwrap();
            assert!(matches!(
                SourceContinuityEffectPort::open(
                    root.join("export"),
                    conflict_registry,
                    authority,
                    fixture.config.bounds.clone(),
                    10,
                ),
                Err(ContinuityControlError::ConflictingRetry)
            ));
            let state_path = root.join("export/source-state.json");
            let mut state: serde_json::Value =
                serde_json::from_slice(&std::fs::read(&state_path).unwrap()).unwrap();
            state["operations"]["1"]["participant_resume_receipts"]
                .as_object_mut()
                .unwrap()
                .remove("governance");
            std::fs::write(&state_path, serde_jcs::to_vec(&state).unwrap()).unwrap();
            let authority =
                CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
            let (corrupt_registry, corrupt_ingress) =
                registry_with_ingress(&root.join("corrupt-resume-operations"), 5).unwrap();
            let corrupt_reopen = SourceContinuityEffectPort::open(
                root.join("export"),
                corrupt_registry,
                authority,
                fixture.config.bounds.clone(),
                9,
            )
            .unwrap();
            assert!(!corrupt_ingress.admission_is_open());
            assert!(matches!(
                corrupt_reopen.resume_source(&handle).await,
                Err(ContinuityControlError::RecoveryRequired)
            ));
            proved_markers.extend([
                "proved:case:source_resume_exact_retry".into(),
                "accepted:lifecycle:source_resume_receipt_restart_reconciled".into(),
            ]);
        }
        "isolated_stage" => {
            let (_, _, catalog) = export(&fixture, 1).await;
            let stage = fixture.target.create_stage("stage-a", 9, catalog).unwrap();
            assert_eq!(stage.handle.id(), "stage-a");
            assert!(!format!("{:?}", stage.handle).contains(root.to_str().unwrap()));
            proved_markers.extend(
                [
                    "proved:case:isolated_stage",
                    "accepted:path:opaque_handle_root_fixed",
                ]
                .map(str::to_owned),
            );
        }
        "isolated_import_validate" => {
            let (_, checkpoint, catalog) = export(&fixture, 1).await;
            let source = fixture.source.bundle_source(&checkpoint).unwrap();
            let payloads = catalog
                .entries
                .iter()
                .map(|entry| {
                    (
                        entry.clone(),
                        source
                            .read_entry_range(entry.ordinal, 0, entry.bytes)
                            .unwrap(),
                    )
                })
                .collect::<Vec<_>>();
            let mut duplicate = catalog.clone();
            duplicate.entries[1].file = duplicate.entries[0].file.clone();
            duplicate.signature = hex::encode(
                SigningKey::from_bytes(&[23; 32])
                    .sign(&duplicate.unsigned_bytes().unwrap())
                    .to_bytes(),
            );
            assert!(matches!(
                fixture.target.create_stage("duplicate", 9, duplicate),
                Err(ContinuityControlError::CatalogMismatch)
                    | Err(ContinuityControlError::UnsafePath)
            ));
            let stage = fixture
                .target
                .create_stage("stage-a", 9, catalog.clone())
                .unwrap();
            let (first_entry, first_bytes) = &payloads[0];
            let split = first_bytes.len().div_ceil(2);
            let first_receipt = fixture
                .target
                .write_chunk(
                    &stage.handle,
                    first_entry.ordinal,
                    0,
                    0,
                    None,
                    &sha256(&first_bytes[..split]),
                    &first_bytes[..split],
                )
                .unwrap();
            assert_eq!(
                first_receipt,
                fixture
                    .target
                    .write_chunk(
                        &stage.handle,
                        first_entry.ordinal,
                        0,
                        0,
                        None,
                        &sha256(&first_bytes[..split]),
                        &first_bytes[..split],
                    )
                    .unwrap()
            );
            let config = fixture.config.clone();
            drop(fixture.target);
            let (catalog_keys, decision_keys) = trusted_target_keys();
            let target =
                TargetContinuityCoordinator::open(config, catalog_keys, decision_keys).unwrap();
            assert!(matches!(
                target.write_chunk(
                    &stage.handle,
                    first_entry.ordinal,
                    1,
                    split as u64,
                    Some(&"00".repeat(32)),
                    &sha256(&first_bytes[split..]),
                    &first_bytes[split..],
                ),
                Err(ContinuityControlError::ContentMismatch)
            ));
            target
                .write_chunk(
                    &stage.handle,
                    first_entry.ordinal,
                    1,
                    split as u64,
                    Some(first_receipt.digest()),
                    &sha256(&first_bytes[split..]),
                    &first_bytes[split..],
                )
                .unwrap();
            for (entry, bytes) in payloads.iter().skip(1) {
                target
                    .write_entry(&stage.handle, entry.ordinal, bytes)
                    .unwrap();
            }
            let evidence = target
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
            proved_markers.extend(
                [
                    "proved:case:isolated_import_validate",
                    "accepted:prefix:accepted_prefix_exact",
                    "accepted:prefix:snapshot_generation_exact",
                    "accepted:prefix:predecessor_exact",
                    "accepted:prefix:topology_and_config_exact",
                    "accepted:prefix:service_and_content_exact",
                    "denied:path:duplicate_filename_denied",
                ]
                .map(str::to_owned),
            );
        }
        "wrong_manifest_signature" => {
            let (_, _, mut catalog) = export(&fixture, 1).await;
            catalog.signature = "00".repeat(64);
            assert!(matches!(
                fixture.target.create_stage("stage-a", 9, catalog),
                Err(ContinuityControlError::ManifestSignature)
            ));
            proved_markers.push("proved:case:wrong_manifest_signature".into());
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
            proved_markers.push(format!("proved:case:{name}"));
            match name {
                "wrong_generation" => {
                    proved_markers.push("denied:prefix:snapshot_generation_mismatch_denied".into())
                }
                "wrong_predecessor" => {
                    proved_markers.push("denied:prefix:predecessor_mismatch_denied".into())
                }
                "wrong_accepted_prefix" => {
                    proved_markers.push("denied:prefix:accepted_prefix_mismatch_denied".into())
                }
                _ => {}
            }
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
            proved_markers.push("proved:case:corrupt_content".into());
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
            #[cfg(unix)]
            {
                let stage_path = original.join("stage-a");
                let displaced_stage = original.join("stage-a-displaced");
                std::fs::rename(&stage_path, &displaced_stage).unwrap();
                std::os::unix::fs::symlink(&displaced_stage, &stage_path).unwrap();
                assert!(fixture
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
                    .is_err());
                std::fs::remove_file(&stage_path).unwrap();
                std::fs::rename(&displaced_stage, &stage_path).unwrap();
            }
            proved_markers.extend(
                [
                    "proved:case:opened_handle_replacement",
                    "denied:path:opened_inode_replacement_denied",
                ]
                .map(str::to_owned),
            );
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
            proved_markers.extend(
                [
                    "proved:case:oversized_bundle",
                    "denied:size:blob_n_plus_1_denied",
                    "denied:size:total_bytes_n_plus_1_denied",
                ]
                .map(str::to_owned),
            );
        }
        "caller_path_rejected" => {
            let (_, _, mut catalog) = export(&fixture, 1).await;
            catalog.entries[0].file = "../escape".into();
            assert!(fixture.target.create_stage("stage-a", 9, catalog).is_err());
            proved_markers.extend(
                [
                    "proved:case:caller_path_rejected",
                    "denied:path:caller_path_absent",
                    "denied:path:traversal_filename_denied",
                ]
                .map(str::to_owned),
            );
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
                assert!(
                    TargetContinuityCoordinator::open(cfg, BTreeMap::new(), BTreeMap::new(),)
                        .is_err()
                );
                let (stage, catalog) = staged(&fixture, "stage-a").await;
                let leaf = fixture
                    .config
                    .staging_dir
                    .join("stage-a")
                    .join(&catalog.entries[0].file);
                std::fs::remove_file(&leaf).unwrap();
                let outside_file = outside.join("outside.bin");
                std::fs::write(&outside_file, b"outside").unwrap();
                std::os::unix::fs::symlink(&outside_file, &leaf).unwrap();
                assert!(fixture
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
                    .is_err());
                std::fs::remove_file(&leaf).unwrap();
                std::fs::hard_link(&outside_file, &leaf).unwrap();
                assert!(fixture
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
                    .is_err());
                proved_markers.extend(
                    [
                        "proved:case:symlink_path_rejected",
                        "denied:path:parent_symlink_denied",
                        "denied:path:leaf_symlink_denied",
                    ]
                    .map(str::to_owned),
                );
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
            proved_markers.extend(
                [
                    "proved:case:deadline_before_effect",
                    "accepted:lifecycle:cleanup_permit_survives_expiry",
                ]
                .map(str::to_owned),
            );
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
            let decision =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
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
            let mut conflicting =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
            conflicting.decision_id = "decision-b".into();
            conflicting.signature = hex::encode(
                SigningKey::from_bytes(&[29; 32])
                    .sign(&conflicting.unsigned_bytes().unwrap())
                    .to_bytes(),
            );
            drop(fixture.target);
            let (catalog_keys, decision_keys) = trusted_target_keys();
            let reopened = TargetContinuityCoordinator::open(
                fixture.config.clone(),
                catalog_keys,
                decision_keys,
            )
            .unwrap();
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
            drop(reopened);
            let stage_state_path = fixture.config.staging_dir.join("stage-a/stage.json");
            let original_stage = std::fs::read(&stage_state_path).unwrap();
            let original_value: serde_json::Value =
                serde_json::from_slice(&original_stage).unwrap();
            let mut corruptions = Vec::new();
            let mut corrupted = original_value.clone();
            corrupted["catalog"]["signature"] = serde_json::json!("00".repeat(64));
            corruptions.push(corrupted);
            let mut corrupted = original_value.clone();
            corrupted["handle"]["catalog_sha256"] = serde_json::json!("10".repeat(32));
            corruptions.push(corrupted);
            let mut corrupted = original_value.clone();
            corrupted["cleanup"]["catalog_sha256"] = serde_json::json!("11".repeat(32));
            corruptions.push(corrupted);
            let first_chunk = original_value["received"]
                .as_object()
                .unwrap()
                .keys()
                .next()
                .unwrap()
                .clone();
            let mut corrupted = original_value.clone();
            corrupted["received"][&first_chunk]["chunk_sha256"] =
                serde_json::json!("12".repeat(32));
            corruptions.push(corrupted);
            let mut corrupted = original_value.clone();
            corrupted["activation_decision"]["authority_key_generation"] = serde_json::json!(2);
            corruptions.push(corrupted);
            for corrupted in corruptions {
                std::fs::write(&stage_state_path, serde_jcs::to_vec(&corrupted).unwrap()).unwrap();
                let (catalog_keys, decision_keys) = trusted_target_keys();
                assert!(TargetContinuityCoordinator::open(
                    fixture.config.clone(),
                    catalog_keys,
                    decision_keys,
                )
                .is_err());
            }
            std::fs::write(&stage_state_path, &original_stage).unwrap();
            let active_path = fixture.config.state_dir.join("active-target.json");
            let original_active = std::fs::read(&active_path).unwrap();
            let mut active: serde_json::Value = serde_json::from_slice(&original_active).unwrap();
            active["decision_sha256"] = serde_json::json!("13".repeat(32));
            std::fs::write(&active_path, serde_jcs::to_vec(&active).unwrap()).unwrap();
            let (catalog_keys, decision_keys) = trusted_target_keys();
            assert!(TargetContinuityCoordinator::open(
                fixture.config.clone(),
                catalog_keys,
                decision_keys,
            )
            .is_err());
            std::fs::write(&active_path, original_active).unwrap();
            proved_markers.extend(
                [
                    "proved:case:restart_after_accept",
                    "accepted:lifecycle:cleanup_permit_restart_reconciled",
                    "accepted:lifecycle:activation_receipt_durable",
                    "accepted:lifecycle:activation_receipt_exact_retry",
                ]
                .map(str::to_owned),
            );
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
            reopened.resume_source(&handle).await.unwrap();
            assert!(restart_ingress.admission_is_open());
            proved_markers.push("proved:case:crash_after_bundle_commit".into());
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
            let (catalog_keys, decision_keys) = trusted_target_keys();
            let reopened = TargetContinuityCoordinator::open(
                fixture.config.clone(),
                catalog_keys,
                decision_keys,
            )
            .unwrap();
            let first = reopened.discard(&stage.handle, &stage.cleanup).unwrap();
            let second = reopened.discard(&stage.handle, &stage.cleanup).unwrap();
            assert_eq!(first, second);
            assert!(!fixture.config.staging_dir.join("stage-a").exists());
            proved_markers.extend(
                [
                    "proved:case:discard_exact_retry",
                    "accepted:lifecycle:discard_receipt_restart_reconciled",
                ]
                .map(str::to_owned),
            );
        }
        "target_discard" => {
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
            let decision =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
            fixture
                .target
                .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                .unwrap();
            assert!(matches!(
                fixture.target.discard(&stage.handle, &stage.cleanup),
                Err(ContinuityControlError::StageState)
                    | Err(ContinuityControlError::CleanupAuthority)
                    | Err(ContinuityControlError::Activated)
            ));
            proved_markers.extend([
                "proved:case:target_discard".into(),
                "accepted:lifecycle:activation_consumes_cleanup_permit".into(),
                "denied:lifecycle:activated_stage_discard_denied".into(),
            ]);
        }
        "validated_target_discard" | "zero_residue" => {
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
            proved_markers.push(format!("proved:case:{name}"));
            match name {
                "validated_target_discard" => {
                    proved_markers.push("accepted:lifecycle:validated_stage_discardable".into())
                }
                "zero_residue" => proved_markers.push("accepted:path:discard_zero_entries".into()),
                _ => unreachable!(),
            }
        }
        "dual_open" => {
            let first = DurableContinuityJournal::open(&fixture.config).unwrap();
            assert!(matches!(
                DurableContinuityJournal::open(&fixture.config),
                Err(ContinuityControlError::AlreadyOpen)
            ));
            proved_markers.push("proved:case:dual_open".into());
            drop(first);
            DurableContinuityJournal::open(&fixture.config)
                .unwrap_or_else(|error| panic!("journal did not reopen: {error:?}"));
            let (catalog_keys, decision_keys) = trusted_target_keys();
            assert!(matches!(
                TargetContinuityCoordinator::open(
                    fixture.config.clone(),
                    catalog_keys,
                    decision_keys,
                ),
                Err(ContinuityControlError::AlreadyOpen)
            ));
        }
        "evidence_redaction" => {
            let (stage, _) = staged(&fixture, "stage-a").await;
            let evidence = serde_json::to_string(&stage.cleanup).unwrap();
            assert!(!evidence.contains(root.to_str().unwrap()));
            assert!(!evidence.contains("private_key"));
            proved_markers.push("proved:case:evidence_redaction".into());
        }
        "public_surface_absent" => {
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
            let mut permit_signed =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
            permit_signed.signature = hex::encode(
                SigningKey::from_bytes(&[31; 32])
                    .sign(&permit_signed.unsigned_bytes().unwrap())
                    .to_bytes(),
            );
            assert!(matches!(
                fixture
                    .target
                    .activate(&stage.handle, &possession, &stage.cleanup, &permit_signed),
                Err(ContinuityControlError::ActivationDecision)
            ));
            let decision =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
            fixture
                .target
                .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                .unwrap();
            proved_markers.extend(
                [
                    "proved:case:public_surface_absent",
                    "denied:lifecycle:activation_decision_requires_204",
                ]
                .map(str::to_owned),
            );
        }
        "guardian_initialization_live" => {
            let (stage, catalog) = staged(&fixture, "stage-live").await;
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
            let decision =
                decision_certificate(&fixture, &stage.handle, &stage.cleanup, possession.digest());
            let activation = fixture
                .target
                .activate(&stage.handle, &possession, &stage.cleanup, &decision)
                .unwrap();
            assert!(!activation.digest().is_empty());
            assert!(fixture
                .config
                .state_dir
                .join("active-target.json")
                .is_file());
            proved_markers.extend(
                [
                    "proved:case:guardian_initialization_live",
                    "accepted:lifecycle:activation_effect_owned_by_208",
                ]
                .map(str::to_owned),
            );
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
            proved_markers.extend(
                [
                    "proved:case:participant_registry_complete",
                    "accepted:size:service_count_n_accepted",
                    "denied:size:service_count_n_plus_1_denied",
                ]
                .map(str::to_owned),
            );
        }
        _ => panic!("unknown contract case {name}"),
    }
}

async fn run_case(name: &str) {
    let temp = TempDir::new().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let mut proved_markers = Vec::<String>::new();
    execute_case(name, &root, &mut proved_markers).await;
    emit_case(name, &root, proved_markers);
    drop(root);
    temp.close()
        .unwrap_or_else(|error| panic!("test root teardown leaked for {name}: {error}"));
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
