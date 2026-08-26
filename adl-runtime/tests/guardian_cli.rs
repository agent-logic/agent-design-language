use std::{
    collections::BTreeMap,
    fs,
    net::{SocketAddr, TcpListener},
    path::PathBuf,
    process::{Command, Output},
    sync::Arc,
    time::Duration,
};

use adl_runtime_kernel::{
    bootstrap_reasoning_services, build_mutual_tls_server_config,
    build_production_operation_executors_with_recorder, load_identity, load_trust_roots,
    serve_private_continuity_listener, sha256, AdapterPolicy, AuthorityMode, CanonicalIngress,
    CatalogSigningAuthority, ContinuityControlBounds, ContinuityControlInitConfig,
    ContinuityControlService, ContinuityControlTlsConfig, DurableContinuityJournal,
    IngressSnapshot, LiveContinuityRegistry, LiveOperationContinuity, OperationalAdapter,
    OperationalFactory, RuntimeInitConfig, RuntimeRecorder, SourceContinuityEffectPort,
    TargetContinuityCoordinator, TlsIdentityPaths, PRIVATE_ALPN, REQUIRED_OPERATIONAL_ADAPTERS,
};
use ed25519_dalek::SigningKey;
use tokio_util::sync::CancellationToken;

#[path = "support/tls.rs"]
mod tls;

fn guardian(args: &[String]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl-runtime-guardian"))
        .args(args)
        .output()
        .expect("guardian binary should execute")
}

fn legacy_args(kernel: &str, root: &std::path::Path) -> Vec<String> {
    let init = root.join("runtime-init.toml");
    fs::write(
        &init,
        format!(
            r#"
[binaries]
kernel_path = "{}"

[shutdown]
checkpoint_deadline_millis = 100
kernel_grace_millis = 100
api_drain_millis = 100
guardian_margin_millis = 100

[guardian]
restart_budget = 0
backoff_base_millis = 1
backoff_cap_millis = 1
healthy_window_millis = 100
lease_auth_timeout_millis = 100
lease_auth_attempts = 1
capture_max_bytes = 65536
capture_drain_grace_millis = 100
configuration_exit_codes = [64]
"#,
            kernel.replace('\\', "\\\\").replace('"', "\\\"")
        ),
    )
    .unwrap();
    vec!["--init".to_owned(), init.to_string_lossy().into_owned()]
}

struct ContinuityServer {
    runtime: tokio::runtime::Runtime,
    shutdown: CancellationToken,
    task: tokio::task::JoinHandle<()>,
}

impl Drop for ContinuityServer {
    fn drop(&mut self) {
        self.shutdown.cancel();
        self.runtime.block_on(async {
            let _ = tokio::time::timeout(Duration::from_secs(2), &mut self.task).await;
        });
    }
}

fn write_identity(
    root: &std::path::Path,
    name: &str,
    identity: &tls::TestIdentity,
    roots: &[u8],
) -> (PathBuf, PathBuf, PathBuf) {
    let certificate = root.join(format!("{name}.pem"));
    let private_key = root.join(format!("{name}.key"));
    let trust_roots = root.join(format!("{name}-roots.pem"));
    fs::write(&certificate, identity.certificate_pem()).unwrap();
    fs::write(&private_key, identity.private_key_pem()).unwrap();
    fs::write(&trust_roots, roots).unwrap();
    (certificate, private_key, trust_roots)
}

fn spki_sha256(certificate: &rustls::pki_types::CertificateDer<'_>) -> String {
    let (_, parsed) = x509_parser::parse_x509_certificate(certificate.as_ref()).unwrap();
    sha256(parsed.public_key().raw)
}

fn live_registry(root: &std::path::Path) -> LiveContinuityRegistry {
    let recorder = RuntimeRecorder::new(16);
    fs::create_dir_all(root).unwrap();
    let reasoning = bootstrap_reasoning_services(recorder.clone()).unwrap();
    let ingress = CanonicalIngress::new(8, recorder.clone(), BTreeMap::new());
    ingress.restore(IngressSnapshot {
        accepted_through: 0,
        completed: BTreeMap::new(),
    });
    let executors =
        build_production_operation_executors_with_recorder(root, recorder.clone()).unwrap();
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
                .unwrap(),
            );
            (
                kind.service_name().to_owned(),
                OperationalFactory::new(adapter, Vec::new()),
            )
        })
        .collect();
    LiveContinuityRegistry::from_production_handles(
        ingress,
        recorder,
        reasoning,
        root.to_path_buf(),
        LiveOperationContinuity::from_factories(factories).unwrap(),
        5,
    )
    .unwrap()
}

fn complete_args(kernel: &str, root: &std::path::Path) -> (Vec<String>, ContinuityServer) {
    let state_root = root.join("state");
    let tls_root = state_root.join("tls");
    let credentials_root = state_root.join("credentials");
    fs::create_dir_all(&tls_root).unwrap();
    fs::create_dir_all(&credentials_root).unwrap();

    let pki = tls::TestPki::new("guardian-cli");
    let server_identity = pki.server(&["localhost"]);
    let guardian_identity = pki.client(&["guardian-logical"]);
    let (api_certificate, api_private_key, api_roots) =
        write_identity(&tls_root, "api-server", &server_identity, pki.root_pem());
    let (server_certificate, server_private_key, server_roots) = write_identity(
        &tls_root,
        "continuity-server",
        &server_identity,
        pki.root_pem(),
    );
    let (guardian_certificate, guardian_private_key, guardian_roots) = write_identity(
        &tls_root,
        "continuity-guardian",
        &guardian_identity,
        pki.root_pem(),
    );

    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let continuity_address = listener.local_addr().unwrap();
    let api_address: SocketAddr = TcpListener::bind("127.0.0.1:0")
        .and_then(|probe| probe.local_addr())
        .unwrap();
    assert_ne!(api_address, continuity_address);

    let guardian_state = state_root.join("guardian-continuity");
    let continuity_state = state_root.join("kernel-continuity");
    let staging = state_root.join("continuity-staging");
    for path in [&guardian_state, &continuity_state, &staging] {
        fs::create_dir_all(path).unwrap();
    }
    let continuity = ContinuityControlInitConfig {
        address: continuity_address.to_string(),
        guardian_state_dir: guardian_state,
        state_dir: continuity_state,
        staging_dir: staging,
        trust_domain: "agent-logic.test".to_owned(),
        polis: "polis-a".to_owned(),
        source_node: "node-source".to_owned(),
        target_node: "node-target".to_owned(),
        guardian_id: "guardian-logical".to_owned(),
        kernel_control_id: "kernel-control".to_owned(),
        channel_epoch: 1,
        tls: ContinuityControlTlsConfig {
            server_certificate_chain_path: server_certificate.clone(),
            server_private_key_path: server_private_key.clone(),
            server_trust_roots_path: server_roots.clone(),
            server_name: "localhost".to_owned(),
            guardian_certificate_chain_path: guardian_certificate,
            guardian_private_key_path: guardian_private_key,
            guardian_trust_roots_path: guardian_roots,
            guardian_spki_sha256: spki_sha256(&guardian_identity.certificate),
            server_spki_sha256: spki_sha256(&server_identity.certificate),
            certificate_generation: 1,
            successor: None,
        },
        bounds: ContinuityControlBounds {
            max_frame_bytes: 65_536,
            max_blob_bytes: 65_536,
            max_total_bytes: 524_288,
            max_services: 5,
            max_journal_entries: 64,
            max_open_handles: 8,
        },
    };

    let control_key = credentials_root.join("control-public-key.hex");
    let operation_key = credentials_root.join("operation-public-key.hex");
    let migration_key = credentials_root.join("migration-decision-public-key.hex");
    let continuity_key = credentials_root.join("continuity-signing-key.hex");
    let observatory_token = credentials_root.join("observatory-token.txt");
    let acip_token = credentials_root.join("acip-write-token.txt");
    let birth_witness_trust = credentials_root.join("birth-witness-trust.json");
    for (path, value) in [
        (&control_key, "11".repeat(32)),
        (&operation_key, "22".repeat(32)),
        (&migration_key, "33".repeat(32)),
        (&continuity_key, "44".repeat(32)),
        (&observatory_token, "guardian-observatory-token".to_owned()),
        (&acip_token, "guardian-acip-token".to_owned()),
    ] {
        fs::write(path, value).unwrap();
    }
    let authorities = ["identity_continuity", "memory_capability", "negative_case_guard", "handoff_consumer"]
        .into_iter()
        .enumerate()
        .map(|(index, role)| serde_json::json!({
            "witness_id": format!("witness-{}", index + 1),
            "role": role,
            "signing_key_id": format!("witness-key-{}", index + 1),
            "verifying_key": hex::encode(ed25519_dalek::SigningKey::from_bytes(&[u8::try_from(index + 1).unwrap(); 32]).verifying_key().as_bytes()),
        }))
        .collect::<Vec<_>>();
    fs::write(
        &birth_witness_trust,
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.runtime.birth_witness_trust.v1",
            "authority_context": "runtime-v3-birth-witness-authority",
            "authorities": authorities,
        }))
        .unwrap(),
    )
    .unwrap();

    let mut init: RuntimeInitConfig =
        toml::from_str(include_str!("../../infra/runtime-v3/runtime-init.toml")).unwrap();
    init.state_root = state_root.clone();
    init.binaries.kernel_path = PathBuf::from(kernel);
    init.api.address = api_address.to_string();
    init.api.public_base_url = format!("https://localhost:{}", api_address.port());
    init.api.tls.certificate_chain_path = api_certificate;
    init.api.tls.private_key_path = api_private_key;
    init.api.tls.trust_roots_path = api_roots;
    init.api.tls.server_name = "localhost".to_owned();
    init.continuity_control = Some(continuity.clone());
    init.credentials.control_public_key_path = control_key;
    init.credentials.operation_public_key_path = operation_key;
    init.credentials.migration_decision_public_key_path = migration_key;
    init.credentials.continuity_signing_key_path = continuity_key;
    init.credentials.observatory_token_path = observatory_token;
    init.credentials.acip_write_token_path = acip_token;
    init.rebase_birth_witness_trust_manifest();
    init.observability_pipeline.vector_binary_path = PathBuf::from(kernel);
    init.shutdown.checkpoint_deadline_millis = 100;
    init.shutdown.kernel_grace_millis = 100;
    init.shutdown.api_drain_millis = 100;
    init.shutdown.guardian_margin_millis = 100;
    init.guardian.restart_budget = 0;
    init.guardian.backoff_base_millis = 1;
    init.guardian.backoff_cap_millis = 1;
    init.guardian.healthy_window_millis = 100;
    init.guardian.lease_auth_timeout_millis = 1_000;
    init.guardian.lease_auth_attempts = 1;
    init.guardian.capture_drain_grace_millis = 100;
    init.validate().unwrap();

    let catalog_authority =
        CatalogSigningAuthority::from_secret("continuity-key", 1, &[23; 32]).unwrap();
    let catalog_key = catalog_authority.verifying_key();
    let source = Arc::new(
        SourceContinuityEffectPort::open(
            state_root.join("private-exports"),
            live_registry(&state_root.join("operations")),
            catalog_authority,
            continuity.bounds.clone(),
            continuity.channel_epoch,
        )
        .unwrap(),
    );
    let target = Arc::new(
        TargetContinuityCoordinator::open(
            continuity.clone(),
            BTreeMap::from([(("continuity-key".to_owned(), 1), catalog_key)]),
            BTreeMap::from([(
                ("runtime-migration-decisions".to_owned(), 1),
                SigningKey::from_bytes(&[29; 32]).verifying_key(),
            )]),
        )
        .unwrap(),
    );
    let service = Arc::new(ContinuityControlService::new(
        continuity.clone(),
        DurableContinuityJournal::open(&continuity).unwrap(),
        source,
        target,
    ));
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let (server_tls, listener) = runtime.block_on(async {
        let identity = load_identity(&TlsIdentityPaths {
            certificate_chain_path: server_certificate,
            private_key_path: server_private_key,
        })
        .await
        .unwrap();
        let roots = load_trust_roots(&server_roots).await.unwrap();
        let tls = build_mutual_tls_server_config(identity, roots, PRIVATE_ALPN).unwrap();
        let listener = tokio::net::TcpListener::from_std(listener).unwrap();
        (tls, listener)
    });
    let shutdown = CancellationToken::new();
    let task_shutdown = shutdown.clone();
    let task = runtime.spawn(async move {
        serve_private_continuity_listener(listener, server_tls, service, task_shutdown)
            .await
            .unwrap();
    });

    let init_path = root.join("runtime-init.toml");
    fs::write(&init_path, toml::to_string(&init).unwrap()).unwrap();
    (
        vec![
            "--init".to_owned(),
            init_path.to_string_lossy().into_owned(),
        ],
        ContinuityServer {
            runtime,
            shutdown,
            task,
        },
    )
}

fn test_root(name: &str) -> tempfile::TempDir {
    let parent = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join(".csdlc")
        .join("evidence")
        .join("5344")
        .join("work")
        .join("guardian-cli-tests");
    fs::create_dir_all(&parent).unwrap();
    tempfile::Builder::new()
        .prefix(name)
        .tempdir_in(fs::canonicalize(parent).unwrap())
        .unwrap()
}

fn portable_success_child(root: &std::path::Path) -> PathBuf {
    let source = root.join("success_child.rs");
    let executable = root.join(format!("success_child{}", std::env::consts::EXE_SUFFIX));
    fs::write(&source, "fn main() {}\n").unwrap();
    let status = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&executable)
        .status()
        .expect("the Rust test toolchain should resolve rustc");
    assert!(status.success(), "portable child compilation failed");
    executable
}

#[test]
fn guardian_cli_reports_successful_portable_child_as_json() {
    let continuity = test_root("success");
    let child = portable_success_child(continuity.path());
    let (args, _server) = complete_args(child.to_str().unwrap(), continuity.path());
    let output = guardian(&args);

    assert!(
        output.status.success(),
        "stderr={}",
        String::from_utf8_lossy(&output.stderr)
    );
    let payload: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(payload["schema"], "adl.runtime_v3.external_guardian.v2");
    assert_eq!(payload["terminal_state"], "exited_successfully");
    assert_eq!(payload["attempts"], 1);
}

#[test]
fn guardian_cli_rejects_missing_kernel_before_launch() {
    let continuity = test_root("spawn-failure");
    let output = guardian(&legacy_args(
        "/definitely/missing/adl-runtime-kernel",
        continuity.path(),
    ));

    assert_eq!(output.status.code(), Some(64));
    assert!(String::from_utf8_lossy(&output.stderr)
        .contains("binaries.kernel_path must be an absolute existing file"));
    assert!(output.stdout.is_empty());
}

#[test]
fn guardian_cli_rejects_incomplete_unknown_and_invalid_numeric_arguments() {
    for args in [
        Vec::new(),
        vec!["--unknown".to_owned(), "value".to_owned()],
        vec!["--init".to_owned(), "relative.toml".to_owned()],
    ] {
        let output = guardian(&args);
        assert_eq!(output.status.code(), Some(64));
        assert!(!output.stderr.is_empty());
        assert!(output.stdout.is_empty());
    }
}
