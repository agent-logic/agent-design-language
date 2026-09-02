use std::path::{Path, PathBuf};

#[allow(dead_code)]
pub fn write_for_state(
    directory: &Path,
    address: std::net::SocketAddr,
    state_root: &Path,
) -> PathBuf {
    write_with_certificate_for_state(directory, address, state_root).0
}

pub fn write_with_certificate_for_state(
    directory: &Path,
    address: std::net::SocketAddr,
    state_root: &Path,
) -> (PathBuf, Vec<u8>) {
    write_with_certificate_for_state_and_ingress_capacity(directory, address, state_root, 64)
}

pub fn write_with_certificate_for_state_and_ingress_capacity(
    directory: &Path,
    address: std::net::SocketAddr,
    state_root: &Path,
    canonical_ingress_capacity: usize,
) -> (PathBuf, Vec<u8>) {
    use rcgen::{
        date_time_ymd, BasicConstraints, CertificateParams, CertifiedIssuer,
        ExtendedKeyUsagePurpose, IsCa, KeyPair,
    };

    let mut ca_params = CertificateParams::new(["adl-runtime-v3-test-ca".to_owned()]).unwrap();
    ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
    ca_params.not_before = date_time_ymd(2026, 1, 1);
    ca_params.not_after = date_time_ymd(2036, 1, 1);
    let ca_key = KeyPair::generate().unwrap();
    let ca = CertifiedIssuer::self_signed(ca_params, ca_key).unwrap();
    let leaf_key = KeyPair::generate().unwrap();
    let mut leaf_params = CertificateParams::new([
        "localhost".to_owned(),
        "127.0.0.1".to_owned(),
        "::1".to_owned(),
    ])
    .unwrap();
    leaf_params.not_before = date_time_ymd(2026, 1, 1);
    leaf_params.not_after = date_time_ymd(2036, 1, 1);
    let leaf = leaf_params.signed_by(&leaf_key, &ca).unwrap();
    let tls_root = state_root.join("tls");
    std::fs::create_dir_all(&tls_root).unwrap();
    let certificate = tls_root.join("localhost-cert.pem");
    let private_key = tls_root.join("localhost-key.pem");
    let trust_roots = tls_root.join("test-root-ca.pem");
    std::fs::write(&certificate, format!("{}{}", leaf.pem(), ca.pem())).unwrap();
    std::fs::write(&private_key, leaf_key.serialize_pem()).unwrap();
    std::fs::write(&trust_roots, ca.pem()).unwrap();
    let continuity_server_key = KeyPair::generate().unwrap();
    let mut continuity_server_params = CertificateParams::new(["localhost".to_owned()]).unwrap();
    continuity_server_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    continuity_server_params.not_before = date_time_ymd(2026, 1, 1);
    continuity_server_params.not_after = date_time_ymd(2036, 1, 1);
    let continuity_server = continuity_server_params
        .signed_by(&continuity_server_key, &ca)
        .unwrap();
    let continuity_guardian_key = KeyPair::generate().unwrap();
    let mut continuity_guardian_params =
        CertificateParams::new(["guardian-logical".to_owned()]).unwrap();
    continuity_guardian_params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ClientAuth];
    continuity_guardian_params.not_before = date_time_ymd(2026, 1, 1);
    continuity_guardian_params.not_after = date_time_ymd(2036, 1, 1);
    let continuity_guardian = continuity_guardian_params
        .signed_by(&continuity_guardian_key, &ca)
        .unwrap();
    let continuity_server_certificate = tls_root.join("continuity-server.pem");
    let continuity_server_private_key = tls_root.join("continuity-server.key");
    let continuity_server_roots = tls_root.join("continuity-server-ca.pem");
    let continuity_guardian_certificate = tls_root.join("continuity-guardian.pem");
    let continuity_guardian_private_key = tls_root.join("continuity-guardian.key");
    let continuity_guardian_roots = tls_root.join("continuity-guardian-ca.pem");
    std::fs::write(
        &continuity_server_certificate,
        format!("{}{}", continuity_server.pem(), ca.pem()),
    )
    .unwrap();
    std::fs::write(
        &continuity_server_private_key,
        continuity_server_key.serialize_pem(),
    )
    .unwrap();
    std::fs::write(&continuity_server_roots, ca.pem()).unwrap();
    std::fs::write(
        &continuity_guardian_certificate,
        format!("{}{}", continuity_guardian.pem(), ca.pem()),
    )
    .unwrap();
    std::fs::write(
        &continuity_guardian_private_key,
        continuity_guardian_key.serialize_pem(),
    )
    .unwrap();
    std::fs::write(&continuity_guardian_roots, ca.pem()).unwrap();
    let (_, continuity_server_x509) =
        x509_parser::parse_x509_certificate(continuity_server.der().as_ref()).unwrap();
    let continuity_server_spki_sha256 =
        adl_runtime_kernel::sha256(continuity_server_x509.public_key().raw);
    let (_, continuity_guardian_x509) =
        x509_parser::parse_x509_certificate(continuity_guardian.der().as_ref()).unwrap();
    let continuity_guardian_spki_sha256 =
        adl_runtime_kernel::sha256(continuity_guardian_x509.public_key().raw);
    let continuity_address = loop {
        let probe = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let candidate = probe.local_addr().unwrap();
        drop(probe);
        if candidate != address {
            break candidate;
        }
    };
    let continuity_guardian_state = state_root.join("guardian-continuity");
    let continuity_state = state_root.join("kernel-continuity-control");
    let continuity_staging = state_root.join("continuity-staging");
    for path in [
        &continuity_guardian_state,
        &continuity_state,
        &continuity_staging,
    ] {
        std::fs::create_dir_all(path).unwrap();
    }
    let credentials_root = state_root.join("credentials");
    std::fs::create_dir_all(&credentials_root).unwrap();
    let control_public_key = credentials_root.join("control-public-key.hex");
    let operation_public_key = credentials_root.join("operation-public-key.hex");
    let migration_decision_public_key = credentials_root.join("migration-decision-public-key.hex");
    let continuity_signing_key = credentials_root.join("continuity-signing-key.hex");
    let observatory_token = credentials_root.join("observatory-token.txt");
    let acip_write_token = credentials_root.join("acip-write-token.txt");
    let birth_witness_trust = credentials_root.join("birth-witness-trust.json");
    std::fs::write(
        &control_public_key,
        hex::encode(
            ed25519_dalek::SigningKey::from_bytes(&[17_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(
        &migration_decision_public_key,
        hex::encode(
            ed25519_dalek::SigningKey::from_bytes(&[31_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(
        &operation_public_key,
        hex::encode(
            ed25519_dalek::SigningKey::from_bytes(&[29_u8; 32])
                .verifying_key()
                .as_bytes(),
        ),
    )
    .unwrap();
    std::fs::write(&continuity_signing_key, hex::encode([23_u8; 32])).unwrap();
    std::fs::write(&observatory_token, "guardian-observatory-token-00000001").unwrap();
    std::fs::write(&acip_write_token, "guardian-acip-write-token-000000001").unwrap();
    let authorities = [
        "identity_continuity",
        "memory_capability",
        "negative_case_guard",
        "handoff_consumer",
    ]
    .into_iter()
    .enumerate()
    .map(|(index, role)| {
        let seed = u8::try_from(index + 1).unwrap();
        serde_json::json!({
            "witness_id": format!("witness-{}", index + 1),
            "role": role,
            "signing_key_id": format!("witness-key-{}", index + 1),
            "verifying_key": hex::encode(
                ed25519_dalek::SigningKey::from_bytes(&[seed; 32])
                    .verifying_key()
                    .as_bytes()
            ),
        })
    })
    .collect::<Vec<_>>();
    std::fs::write(
        &birth_witness_trust,
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.runtime.birth_witness_trust.v1",
            "authority_context": "runtime-v3-birth-witness-authority",
            "authorities": authorities,
        }))
        .unwrap(),
    )
    .unwrap();
    let vector = repo_vector_binary();
    let kernel = std::env::current_exe().unwrap();
    let init = directory.join("runtime-init.toml");
    std::fs::write(
        &init,
        format!(
            r#"schema = "adl.runtime_v3.init.v1"
state_root = "{}"
[binaries]
kernel_path = "{}"
[paths]
continuity_dir = "continuity"
tls_dir = "tls"
credentials_dir = "credentials"
observability_dir = "observability"
[kernel]
recorder_capacity = 32
control_history_capacity = 64
checkpoint_channel_capacity = 4
canonical_ingress_capacity = {}
component_readiness_timeout_millis = 5000
observability_poll_millis = 50
weather_stale_after_millis = 75
guardian_lease_connect_millis = 30000
guardian_lease_auth_millis = 30000
trusted_time_sample_timeout_millis = 3000
trusted_time_max_offset_millis = 5000
trusted_time_max_round_trip_millis = 2000
trusted_time_retry_millis = 1000
trusted_time_refresh_millis = 60000
[api]
address = "{}"
public_base_url = "https://localhost:{}"
bind_attempts = 20
bind_retry_millis = 100
websocket_auth_timeout_millis = 5000
websocket_refresh_millis = 1000
websocket_max_frame_bytes = 65536
[api.tls]
certificate_chain_path = "{}"
private_key_path = "{}"
trust_roots_path = "{}"
server_name = "localhost"
[continuity_control]
address = "{}"
guardian_state_dir = "{}"
state_dir = "{}"
staging_dir = "{}"
trust_domain = "agent-logic.test"
polis = "polis-a"
source_node = "node-source"
target_node = "node-target"
guardian_id = "guardian-logical"
kernel_control_id = "kernel-control"
channel_epoch = 1
[continuity_control.tls]
server_certificate_chain_path = "{}"
server_private_key_path = "{}"
server_trust_roots_path = "{}"
server_name = "localhost"
guardian_certificate_chain_path = "{}"
guardian_private_key_path = "{}"
guardian_trust_roots_path = "{}"
guardian_spki_sha256 = "{}"
server_spki_sha256 = "{}"
certificate_generation = 1
[continuity_control.bounds]
max_frame_bytes = 65536
max_blob_bytes = 65536
max_total_bytes = 524288
max_services = 5
max_journal_entries = 64
max_open_handles = 8
[credentials]
control_public_key_path = "{}"
control_key_id = "operator"
control_principal = "operator"
operation_public_key_path = "{}"
operation_key_id = "runtime-operations"
migration_decision_public_key_path = "{}"
migration_decision_key_id = "runtime-migration-decisions"
migration_decision_key_generation = 1
continuity_signing_key_path = "{}"
continuity_key_id = "runtime-continuity"
observatory_token_path = "{}"
acip_write_token_path = "{}"
birth_witness_trust_manifest_path = "{}"
continuity_min_generation = 0
sntp_server = "time.cloudflare.com"
[shutdown]
checkpoint_deadline_millis = 5000
kernel_grace_millis = 10000
api_drain_millis = 3000
guardian_margin_millis = 500
[guardian]
restart_budget = 3
backoff_base_millis = 100
backoff_cap_millis = 5000
healthy_window_millis = 60000
lease_auth_timeout_millis = 5000
lease_auth_attempts = 3
capture_max_bytes = 65536
capture_drain_grace_millis = 2000
configuration_exit_codes = [64]
[qualification]
readiness_timeout_millis = 10000
readiness_poll_millis = 10
shutdown_wait_millis = 50000
[polis]
id = "polis-test"
display_name = "Test Polis"
public_domain = "localhost"
observatory_public_origin = "https://localhost:8765"

[resident_shepherd]
name = "beacon.axioma"
display_name = "Beacon"
office = "resident shepherd"
provider = "ollama"
model = "qwen3:8b"
endpoint = "http://127.0.0.1:11434"
[observatory]
allowed_origins = ["https://localhost:8765"]
[observability_pipeline]
vector_binary_path = "{}"
service_name = "adl-runtime-v3"
revision = "test-revision"
guardian_id = "guardian-process-0"
lifecycle_suite = "runtime"
lifecycle_run = "runtime-run"
lifecycle_cycle = "runtime-cycle"
trace_filter = "adl_runtime_kernel=info,adl_runtime=info"
otlp_timeout_millis = 5000
vector_startup_attempts = 3
vector_startup_backoff_millis = 100
vector_shutdown_limit_millis = 3000
drain_timeout_millis = 5000
vector_config_path = "config/runtime-v3-vector.json"
ingress_spool_path = "spool/runtime-v3.current.jsonl"
master_log_path = "durable/master.log.jsonl"
audit_path = "durable/master-log-audit.json"
sequence_checkpoint_path = "durable/sequence.json"
vector_data_dir = "vector-data"
spool_max_bytes = 8388608
spool_retained_files = 4
[weather]
sample_millis = 25
history_capacity = 60
disk_warning_free_bytes = 5368709120
disk_stop_free_bytes = 2147483648
disk_recover_free_bytes = 8589934592
memory_warning_used_basis_points = 8500
memory_stop_used_basis_points = 9500
memory_recover_used_basis_points = 7500
cpu_warning_basis_points = 9000
cpu_stop_basis_points = 9800
cpu_recover_basis_points = 8000
checkpoint_deadline_millis = 750
snapshot_concurrency = 4
"#,
            toml_path(state_root),
            toml_path(&kernel),
            canonical_ingress_capacity,
            address,
            address.port(),
            toml_path(&certificate),
            toml_path(&private_key),
            toml_path(&trust_roots),
            continuity_address,
            toml_path(&continuity_guardian_state),
            toml_path(&continuity_state),
            toml_path(&continuity_staging),
            toml_path(&continuity_server_certificate),
            toml_path(&continuity_server_private_key),
            toml_path(&continuity_server_roots),
            toml_path(&continuity_guardian_certificate),
            toml_path(&continuity_guardian_private_key),
            toml_path(&continuity_guardian_roots),
            continuity_guardian_spki_sha256,
            continuity_server_spki_sha256,
            toml_path(&control_public_key),
            toml_path(&operation_public_key),
            toml_path(&migration_decision_public_key),
            toml_path(&continuity_signing_key),
            toml_path(&observatory_token),
            toml_path(&acip_write_token),
            toml_path(&birth_witness_trust),
            toml_path(&vector),
        ),
    )
    .unwrap();
    (init, ca.der().to_vec())
}

pub fn toml_path(path: &Path) -> String {
    let value = path.to_string_lossy();
    assert!(!value.contains(['\n', '\r']));
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn repo_vector_binary() -> PathBuf {
    if let Ok(path) = std::env::var("ADL_RUNTIME_TEST_VECTOR_BINARY") {
        return PathBuf::from(path);
    }
    let repo_root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    let worktree_vector = repo_root.join(".adl/bin/vector");
    if worktree_vector.is_file() {
        return worktree_vector;
    }
    let output = std::process::Command::new("git")
        .current_dir(repo_root)
        .args(["rev-parse", "--path-format=absolute", "--git-common-dir"])
        .output();
    if let Ok(output) = output {
        if output.status.success() {
            let git_common = String::from_utf8(output.stdout).unwrap();
            let primary_vector = PathBuf::from(git_common.trim())
                .parent()
                .unwrap()
                .join(".adl/bin/vector");
            if primary_vector.is_file() {
                return primary_vector;
            }
        }
    }
    worktree_vector
}
