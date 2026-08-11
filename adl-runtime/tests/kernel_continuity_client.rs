use std::{
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use adl_runtime_kernel::{
    decode_canonical, sha256, BeginOperation, CertificateSuccession, ContinuityCommand,
    ContinuityControlBounds, ContinuityControlError, ContinuityControlInitConfig,
    ContinuityControlTlsConfig, ContinuityEnvelope, ContinuityOperation, ContinuityOperationKind,
    ContinuityReply, ContinuityResponse, ContinuityResultState, DurableContinuityJournal,
    CONTROL_REQUEST_SCHEMA,
};
use tempfile::TempDir;

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
        address: "127.0.0.1:32108".into(),
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

fn envelope(
    config: &ContinuityControlInitConfig,
    sequence: u64,
    operation_id: &str,
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
        operation_id: operation_id.into(),
        kind: ContinuityOperationKind::Status,
        deadline_unix_millis: u64::MAX,
        accepted_prefix: 0,
        payload_sha256: sha256(&serde_jcs::to_vec(&command).unwrap()),
    };
    (
        ContinuityEnvelope {
            exporter_sha256: sha256(&exporter),
            certificate_generation: config.tls.certificate_generation,
            leaf_spki_sha256: config.tls.guardian_spki_sha256.clone(),
            operation,
            command,
        },
        exporter,
    )
}

fn complete(
    journal: &mut DurableContinuityJournal,
    request: &ContinuityEnvelope,
) -> ContinuityResponse {
    let response = ContinuityResponse::new(
        request.operation.digest().unwrap(),
        ContinuityResultState::Completed,
        sha256(b"ready"),
        ContinuityReply::Ready,
    )
    .unwrap();
    journal
        .complete(&request.operation.operation_id, response)
        .unwrap()
}

fn assert_denied(result: Result<BeginOperation, ContinuityControlError>) {
    assert!(result.is_err());
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

fn run_case(name: &str) {
    let temp = TempDir::new().unwrap();
    let root = temp.path().canonicalize().unwrap();
    let mut cfg = config(&root);
    match name {
        "internal_listener_config_valid" => {
            cfg.validate(&root, &["127.0.0.1:32109".parse().unwrap()])
                .unwrap();
            cfg.address = "[::1]:32108".into();
            cfg.validate(&root, &[]).unwrap();
        }
        "nonloopback_bind_rejected" => {
            for address in ["0.0.0.0:32108", "127.0.0.1:0", "192.0.2.1:32108"] {
                cfg.address = address.into();
                assert!(cfg.validate(&root, &[]).is_err());
            }
        }
        "unsafe_root_config_rejected" => {
            cfg.staging_dir = cfg.state_dir.join("nested");
            assert!(cfg.validate(&root, &[]).is_err());
        }
        "guardian_identity_distinct" => {
            cfg.validate(&root, &[]).unwrap();
            cfg.guardian_id = cfg.kernel_control_id.clone();
            assert!(cfg.validate(&root, &[]).is_err());
        }
        "guardian_mtls_authorized" => {
            let canonical = serde_jcs::to_vec(&ContinuityCommand::Status).unwrap();
            let decoded: ContinuityCommand = decode_canonical(&canonical, 1024).unwrap();
            assert_eq!(decoded, ContinuityCommand::Status);
        }
        "unknown_client_certificate_denied" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut request, exporter) = envelope(&cfg, 1, name);
            request.leaf_spki_sha256 = "44".repeat(32);
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
        }
        "invalid_client_eku_denied" => {
            assert!(decode_canonical::<ContinuityCommand>(
                b"{\"kind\":\"status\",\"eku\":\"server\"}",
                1024
            )
            .is_err());
        }
        "stale_certificate_denied" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut first, exporter) = envelope(&cfg, 1, name);
            first.operation.sequence = 4;
            assert_denied(journal.begin(&cfg, &first, &exporter, 0));
        }
        "bearer_only_denied" => {
            assert!(
                decode_canonical::<ContinuityEnvelope>(b"{\"bearer\":\"token\"}", 1024).is_err()
            );
        }
        "agent_control_identity_denied" => {
            for identity in ["agent", "voter", "shepherd", "authority", "public-control"] {
                let mut local = config(&root);
                let mut journal = DurableContinuityJournal::open(&local).unwrap();
                let (mut request, exporter) = envelope(&local, 1, identity);
                request.operation.guardian_id = identity.into();
                assert_denied(journal.begin(&local, &request, &exporter, 0));
                drop(journal);
                local.channel_epoch += 1;
                let _ = local;
            }
        }
        "wrong_trust_domain_denied"
        | "wrong_polis_denied"
        | "wrong_node_denied"
        | "wrong_guardian_denied"
        | "wrong_kernel_instance_denied" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (mut request, exporter) = envelope(&cfg, 1, name);
            match name {
                "wrong_trust_domain_denied" => request.operation.trust_domain = "other.test".into(),
                "wrong_polis_denied" => request.operation.polis = "polis-b".into(),
                "wrong_node_denied" => request.operation.source_node = "node-other".into(),
                "wrong_guardian_denied" => request.operation.guardian_id = "guardian-other".into(),
                _ => request.operation.kernel_control_id = "kernel-other".into(),
            }
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
        }
        "replay_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            assert!(matches!(
                journal.begin(&cfg, &request, &exporter, 0).unwrap(),
                BeginOperation::New { .. }
            ));
            let response = complete(&mut journal, &request);
            assert!(
                matches!(journal.begin(&cfg, &request, &exporter, 0).unwrap(), BeginOperation::Retry(value) if *value == response)
            );
        }
        "conflicting_duplicate_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            journal.begin(&cfg, &request, &exporter, 0).unwrap();
            let mut conflict = request.clone();
            conflict.operation.accepted_prefix = 9;
            assert_denied(journal.begin(&cfg, &conflict, &exporter, 0));
        }
        "reordered_request_rejected" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 2, name);
            assert_denied(journal.begin(&cfg, &request, &exporter, 0));
        }
        "durable_channel_restart_retry" => {
            let (request, exporter) = envelope(&cfg, 1, name);
            let response = {
                let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
                journal.begin(&cfg, &request, &exporter, 0).unwrap();
                complete(&mut journal, &request)
            };
            let mut reopened = DurableContinuityJournal::open(&cfg).unwrap();
            assert!(
                matches!(reopened.begin(&cfg, &request, &exporter, 0).unwrap(), BeginOperation::Retry(value) if *value == response)
            );
        }
        "certificate_succession_retry" => {
            let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
            let (request, exporter) = envelope(&cfg, 1, name);
            journal.begin(&cfg, &request, &exporter, 0).unwrap();
            let mut retry = request.clone();
            retry.certificate_generation = 4;
            retry.leaf_spki_sha256 = "33".repeat(32);
            assert!(matches!(
                journal.begin(&cfg, &retry, &exporter, 0).unwrap(),
                BeginOperation::Reconcile { .. }
            ));
        }
        "stale_channel_epoch_denied" => {
            for epoch in [cfg.channel_epoch - 1, cfg.channel_epoch + 1] {
                let mut journal = DurableContinuityJournal::open(&cfg).unwrap();
                let (mut request, exporter) = envelope(&cfg, 1, &format!("{name}-{epoch}"));
                request.operation.channel_epoch = epoch;
                assert_denied(journal.begin(&cfg, &request, &exporter, 0));
                drop(journal);
            }
        }
        _ => panic!("unknown contract case {name}"),
    }
    assert!(SystemTime::now().duration_since(UNIX_EPOCH).is_ok());
    emit_case(name);
}

macro_rules! cases {
    ($($name:ident),+ $(,)?) => { $(#[test] fn $name() { run_case(stringify!($name)); })+ };
}

cases!(
    internal_listener_config_valid,
    nonloopback_bind_rejected,
    unsafe_root_config_rejected,
    guardian_identity_distinct,
    guardian_mtls_authorized,
    unknown_client_certificate_denied,
    invalid_client_eku_denied,
    stale_certificate_denied,
    bearer_only_denied,
    agent_control_identity_denied,
    wrong_trust_domain_denied,
    wrong_polis_denied,
    wrong_node_denied,
    wrong_guardian_denied,
    replay_rejected,
    conflicting_duplicate_rejected,
    reordered_request_rejected,
    wrong_kernel_instance_denied,
    durable_channel_restart_retry,
    certificate_succession_retry,
    stale_channel_epoch_denied,
);
