use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{Read, Write},
    net::{SocketAddr, ToSocketAddrs},
    path::{Component, Path, PathBuf},
    process::{ExitCode, Stdio},
    sync::Arc,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use adl_runtime::guardian::{GuardianOutcome, GuardianTerminalState};
use adl_runtime::runtime_v3_soak::{
    build_evidence, build_runner_plan, BinaryIdentity, CleanupContract, CleanupOutcome,
    EvidenceClock, FaultContract, FaultKind, FaultRecord, PlatformIdentity, RunOwner, SoakBounds,
    SoakConfig, SoakEvidence, SoakSample, SoakStatus, SoakThresholds, SoakViolation,
    WorkloadContract, AGENT_LOGIC_ACCOUNT_PROFILE, SOAK_CONTRACT_SCHEMA,
};
use adl_runtime_kernel::control::{
    OBSERVATORY_FEED_SCHEMA, OBSERVATORY_WS_AUTH_SCHEMA, OBSERVATORY_WS_CONTROL_RESULT_SCHEMA,
    OBSERVATORY_WS_PATH,
};
use adl_runtime_kernel::verify_live_continuity_lineage;
use base64::Engine;
use ed25519_dalek::{SigningKey, VerifyingKey};
use fs2::FileExt;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
use tokio::process::{Child, ChildStderr, ChildStdout, Command};
use tokio_rustls::rustls::{
    pki_types::{CertificateDer, ServerName},
    ClientConfig, RootCertStore,
};

#[cfg(windows)]
use windows_sys::Win32::Foundation::{CloseHandle, HANDLE};
#[cfg(windows)]
use windows_sys::Win32::System::{
    Console::{GenerateConsoleCtrlEvent, CTRL_BREAK_EVENT},
    Threading::{
        OpenProcess, TerminateProcess, CREATE_NEW_PROCESS_GROUP, PROCESS_QUERY_LIMITED_INFORMATION,
        PROCESS_TERMINATE,
    },
};

const REPORT_SCHEMA: &str = "adl.runtime_v3.lifecycle_soak.v1";
const REQUIRED_CYCLES: u64 = 10_000;
const STRESS_RUNS: u64 = 100;
const STRESS_SECONDS: u64 = 10;
const ENDURANCE_RUNS: u64 = 10;
const ENDURANCE_SECONDS: u64 = 600;
const PLATFORM_PROOF_SCHEMA: &str = "adl.wp12.platform_proof.v1";
const SHORT_QUALIFICATION_CONNECTIONS: u64 = 50;
const SHORT_QUALIFICATION_SAMPLE_INTERVAL_SECONDS: u64 = 1;
const SHORT_QUALIFICATION_WEATHER_STALE_AFTER_MILLIS: u64 = 5;
const SHORT_QUALIFICATION_WEATHER_SAMPLE_MILLIS: u64 = 50;

#[tokio::main]
async fn main() -> ExitCode {
    let raw_args = std::env::args().skip(1).collect::<Vec<_>>();
    if raw_args.iter().any(|arg| arg == "--aggregate-platform") {
        return match AggregateArgs::parse(raw_args.into_iter()) {
            Ok(args) => aggregate_platform(&args),
            Err(error) => {
                eprintln!("{error}");
                ExitCode::from(64)
            }
        };
    }

    let args = match Args::parse(raw_args.into_iter()) {
        Ok(args) => args,
        Err(error) => {
            eprintln!("{error}");
            return ExitCode::from(64);
        }
    };
    if let Err(error) = prepare_state_root(&args.state_root) {
        eprintln!("{error}");
        return ExitCode::from(64);
    }

    let started = Instant::now();
    let kernel_sha256 = match file_sha256(&args.kernel) {
        Ok(digest) => digest,
        Err(error) => {
            eprintln!("failed hashing Runtime v3 kernel: {error}");
            return ExitCode::from(66);
        }
    };
    let fixture = match ProductionFixture::create(
        &args.state_root,
        &args.init_template,
        &args.kernel,
        &args.vector,
        args.suite,
        &args.revision,
    )
    .await
    {
        Ok(fixture) => fixture,
        Err(error) => {
            eprintln!("failed preparing production Runtime v3 launch: {error}");
            return ExitCode::from(66);
        }
    };
    let execution = {
        let _qualification_lock =
            match QualificationLock::acquire(&args.init_template, fixture.address) {
                Ok(lock) => lock,
                Err(error) => {
                    eprintln!("failed acquiring lifecycle qualification lock: {error}");
                    return ExitCode::from(75);
                }
            };
        match execute_suite(&args, &fixture, started).await {
            Ok(execution) => execution,
            Err(failure) => return fail(&args, &kernel_sha256, started, failure),
        }
    };

    let report = report(&args, &kernel_sha256, started, "pass", &execution, None);
    if let Err(error) = write_report(&args.report, &report) {
        eprintln!("failed writing lifecycle report: {error}");
        return ExitCode::from(66);
    }
    println!("{report}");
    ExitCode::SUCCESS
}

struct Args {
    guardian: PathBuf,
    kernel: PathBuf,
    vector: PathBuf,
    init_template: PathBuf,
    state_root: PathBuf,
    report: PathBuf,
    revision: String,
    suite: Suite,
    pre_restart_ready_file: Option<PathBuf>,
    pre_restart_ack_file: Option<PathBuf>,
}

struct ProductionFixture {
    address: SocketAddr,
    init: PathBuf,
    continuity_root: PathBuf,
    local_state_root: PathBuf,
    observability_root: PathBuf,
    master_log: PathBuf,
    log_audit: PathBuf,
    tls_connector: tokio_rustls::TlsConnector,
    tls_server_name: String,
    observatory_origin: String,
    continuity_verifying_key: VerifyingKey,
    observatory_token: String,
    readiness_timeout: Duration,
    readiness_poll: Duration,
    shutdown_wait: Duration,
}

#[derive(Debug)]
struct QualificationLock {
    file: File,
    path: PathBuf,
}

impl QualificationLock {
    fn acquire(init_template: &Path, address: SocketAddr) -> Result<Self, String> {
        let init_template = init_template.canonicalize().map_err(|error| {
            format!(
                "init template {} could not be canonicalized: {error}",
                init_template.display()
            )
        })?;
        let repository_root = repository_root_for_init_template(&init_template)?;
        let lock_dir = repository_root
            .join(".adl")
            .join("runtime-v3")
            .join("qualification");
        std::fs::create_dir_all(&lock_dir).map_err(|error| {
            format!(
                "could not create qualification lock directory {}: {error}",
                lock_dir.display()
            )
        })?;
        let address_key = address.to_string().replace([':', '[', ']'], "_");
        Self::acquire_at(&lock_dir.join(format!("api-{address_key}.lock")), address)
    }

    fn acquire_at(path: &Path, address: SocketAddr) -> Result<Self, String> {
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(path)
            .map_err(|error| {
                format!(
                    "could not open qualification lock {}: {error}",
                    path.display()
                )
            })?;
        file.try_lock_exclusive().map_err(|error| {
            format!(
                "another lifecycle qualification owns configured API address {address} \
                 (lock {}): {error}",
                path.display()
            )
        })?;
        file.set_len(0)
            .and_then(|()| write!(file, "pid={}\naddress={address}\n", std::process::id()))
            .and_then(|()| file.sync_data())
            .map_err(|error| {
                let _ = FileExt::unlock(&file);
                format!(
                    "could not record qualification lock owner in {}: {error}",
                    path.display()
                )
            })?;
        Ok(Self {
            file,
            path: path.to_path_buf(),
        })
    }
}

impl Drop for QualificationLock {
    fn drop(&mut self) {
        let _ = FileExt::unlock(&self.file);
        let _ = std::fs::remove_file(&self.path);
    }
}

impl ProductionFixture {
    async fn create(
        state_root: &Path,
        init_template: &Path,
        kernel: &Path,
        vector: &Path,
        suite: Suite,
        revision: &str,
    ) -> Result<Self, String> {
        let template_text = std::fs::read_to_string(init_template).map_err(|error| {
            format!(
                "could not read init template {}: {error}",
                init_template.display()
            )
        })?;
        let mut init_document = toml::from_str::<toml::Value>(&template_text).map_err(|error| {
            format!("invalid init template {}: {error}", init_template.display())
        })?;
        let configured_address = toml_string(&init_document, &["api", "address"])?;
        let readiness_timeout = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "readiness_timeout_millis"],
        )?);
        let readiness_poll = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "readiness_poll_millis"],
        )?);
        let shutdown_wait = Duration::from_millis(toml_u64(
            &init_document,
            &["qualification", "shutdown_wait_millis"],
        )?);
        let address = configured_address
            .to_socket_addrs()
            .map_err(|error| format!("invalid configured API address: {error}"))?
            .find(SocketAddr::is_ipv4)
            .or_else(|| configured_address.to_socket_addrs().ok()?.next())
            .ok_or_else(|| "configured API address did not resolve".to_owned())?;
        let state_root = state_root
            .canonicalize()
            .map_err(|error| format!("state root could not be canonicalized: {error}"))?;
        let tls_root = create_contained_state_dir(
            &state_root,
            &toml_string(&init_document, &["paths", "tls_dir"])?,
            "TLS state directory",
        )?;
        let continuity_root = create_contained_state_dir(
            &state_root,
            &toml_string(&init_document, &["paths", "continuity_dir"])?,
            "continuity state directory",
        )?;
        let credentials_root = create_contained_state_dir(
            &state_root,
            &toml_string(&init_document, &["paths", "credentials_dir"])?,
            "credentials state directory",
        )?;
        let observability_root = create_contained_state_dir(
            &state_root,
            &toml_string(&init_document, &["paths", "observability_dir"])?,
            "observability state directory",
        )?;
        let master_log = contained_relative_path(
            &observability_root,
            &toml_string(
                &init_document,
                &["observability_pipeline", "master_log_path"],
            )?,
            "master log path",
        )?;
        let log_audit = contained_relative_path(
            &observability_root,
            &toml_string(&init_document, &["observability_pipeline", "audit_path"])?,
            "log audit path",
        )?;

        let certificate = configured_tls_file(
            &init_document,
            "certificate_chain_path",
            "TLS certificate chain",
            false,
        )?;
        let private_key =
            configured_tls_file(&init_document, "private_key_path", "TLS private key", true)?;
        let trust_roots =
            configured_tls_file(&init_document, "trust_roots_path", "TLS trust roots", false)?;
        if certificate.same_identity(&private_key)
            || certificate.same_identity(&trust_roots)
            || private_key.same_identity(&trust_roots)
        {
            return Err(
                "configured TLS certificate chain, private key, and trust roots must be distinct"
                    .to_owned(),
            );
        }
        let tls_server_name =
            toml_string(&init_document, &["api", "tls", "server_name"])?.to_owned();
        let observatory_origins = init_document
            .get("observatory")
            .and_then(|value| value.get("allowed_origins"))
            .and_then(toml::Value::as_array)
            .ok_or_else(|| "init template is missing observatory.allowed_origins".to_owned())?;
        if observatory_origins.len() != 1 {
            return Err(
                "lifecycle qualification requires exactly one Observatory origin".to_owned(),
            );
        }
        let observatory_origin = observatory_origins[0]
            .as_str()
            .ok_or_else(|| "configured Observatory origin must be a string".to_owned())?
            .to_owned();
        let certificate_snapshot = tls_root.join("certificate-chain.pem");
        let private_key_snapshot = tls_root.join("private-key.pem");
        let trust_roots_snapshot = tls_root.join("trust-roots.pem");
        write_secret(&certificate_snapshot, &certificate.bytes)
            .map_err(|_| "could not snapshot configured TLS certificate chain".to_owned())?;
        write_secret(&private_key_snapshot, &private_key.bytes)
            .map_err(|_| "could not snapshot configured TLS private key".to_owned())?;
        write_secret(&trust_roots_snapshot, &trust_roots.bytes)
            .map_err(|_| "could not snapshot configured TLS trust roots".to_owned())?;
        for (field, file_name, label, is_private_key) in [
            (
                "server_certificate_chain_path",
                "continuity-server-certificate.pem",
                "continuity server certificate",
                false,
            ),
            (
                "server_private_key_path",
                "continuity-server-private-key.pem",
                "continuity server private key",
                true,
            ),
            (
                "server_trust_roots_path",
                "continuity-server-trust-roots.pem",
                "continuity server trust roots",
                false,
            ),
            (
                "guardian_certificate_chain_path",
                "continuity-guardian-certificate.pem",
                "continuity Guardian certificate",
                false,
            ),
            (
                "guardian_private_key_path",
                "continuity-guardian-private-key.pem",
                "continuity Guardian private key",
                true,
            ),
            (
                "guardian_trust_roots_path",
                "continuity-guardian-trust-roots.pem",
                "continuity Guardian trust roots",
                false,
            ),
        ] {
            let configured = configured_tls_file_with_path(
                &init_document,
                &["continuity_control", "tls", field],
                label,
                is_private_key,
                || {},
            )?;
            let snapshot = tls_root.join(file_name);
            write_secret(&snapshot, &configured.bytes)
                .map_err(|_| format!("could not snapshot configured {label}"))?;
            set_toml_string(
                &mut init_document,
                &["continuity_control", "tls", field],
                toml_path(&snapshot)?,
            )?;
        }
        for field in ["guardian_state_dir", "state_dir", "staging_dir"] {
            let path = PathBuf::from(toml_string(&init_document, &["continuity_control", field])?);
            create_contained_absolute_state_dir(
                &state_root,
                &path,
                "configured private continuity state",
            )?;
        }

        let control_key = SigningKey::from_bytes(&[17_u8; 32]);
        let operation_key = SigningKey::from_bytes(&[29_u8; 32]);
        let migration_decision_key = SigningKey::from_bytes(&[31_u8; 32]);
        let continuity_key = SigningKey::from_bytes(&[23_u8; 32]);
        let control_public_key = hex::encode(control_key.verifying_key().as_bytes());
        let operation_public_key = hex::encode(operation_key.verifying_key().as_bytes());
        let migration_decision_public_key =
            hex::encode(migration_decision_key.verifying_key().as_bytes());
        let continuity_signing_key = hex::encode([23_u8; 32]);
        let observatory_token = "wp12-observatory-token-000000000001".to_owned();
        let acip_write_token = "wp12-acip-write-token-0000000000001".to_owned();
        let control_public_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "control_public_key_path"],
        )?);
        let operation_public_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "operation_public_key_path"],
        )?);
        let migration_decision_public_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "migration_decision_public_key_path"],
        )?);
        let continuity_signing_key_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "continuity_signing_key_path"],
        )?);
        let observatory_token_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "observatory_token_path"],
        )?);
        let acip_write_token_path = credentials_root.join(toml_file_name(
            &init_document,
            &["credentials", "acip_write_token_path"],
        )?);
        let birth_witness_trust_path = credentials_root.join("birth-witness-trust.json");
        std::fs::write(&control_public_key_path, &control_public_key)
            .map_err(|error| error.to_string())?;
        std::fs::write(&operation_public_key_path, &operation_public_key)
            .map_err(|error| error.to_string())?;
        std::fs::write(
            &migration_decision_public_key_path,
            &migration_decision_public_key,
        )
        .map_err(|error| error.to_string())?;
        write_secret(
            &continuity_signing_key_path,
            continuity_signing_key.as_bytes(),
        )
        .map_err(|error| error.to_string())?;
        write_secret(&observatory_token_path, observatory_token.as_bytes())
            .map_err(|error| error.to_string())?;
        write_secret(&acip_write_token_path, acip_write_token.as_bytes())
            .map_err(|error| error.to_string())?;
        let authorities = ["identity_continuity", "memory_capability", "negative_case_guard", "handoff_consumer"]
            .into_iter()
            .enumerate()
            .map(|(index, role)| serde_json::json!({
                "witness_id": format!("witness-{}", index + 1),
                "role": role,
                "signing_key_id": format!("witness-key-{}", index + 1),
                "verifying_key": hex::encode(SigningKey::from_bytes(&[(index + 1) as u8; 32]).verifying_key().as_bytes()),
            }))
            .collect::<Vec<_>>();
        std::fs::write(
            &birth_witness_trust_path,
            serde_json::to_vec(&serde_json::json!({
                "schema": "adl.runtime.birth_witness_trust.v1",
                "authority_context": "runtime-v3-birth-witness-authority",
                "authorities": authorities,
            }))
            .map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;

        set_toml_string(&mut init_document, &["state_root"], toml_path(&state_root)?)?;
        set_toml_string(
            &mut init_document,
            &["binaries", "kernel_path"],
            toml_path(kernel)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["observability_pipeline", "vector_binary_path"],
            toml_path(vector)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["api", "tls", "certificate_chain_path"],
            toml_path(&certificate_snapshot)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["api", "tls", "private_key_path"],
            toml_path(&private_key_snapshot)?,
        )?;
        set_toml_string(
            &mut init_document,
            &["api", "tls", "trust_roots_path"],
            toml_path(&trust_roots_snapshot)?,
        )?;
        for (field, path) in [
            ("control_public_key_path", &control_public_key_path),
            ("operation_public_key_path", &operation_public_key_path),
            (
                "migration_decision_public_key_path",
                &migration_decision_public_key_path,
            ),
            ("continuity_signing_key_path", &continuity_signing_key_path),
            ("observatory_token_path", &observatory_token_path),
            ("acip_write_token_path", &acip_write_token_path),
            (
                "birth_witness_trust_manifest_path",
                &birth_witness_trust_path,
            ),
        ] {
            set_toml_string(
                &mut init_document,
                &["credentials", field],
                toml_path(path)?,
            )?;
        }
        for (field, value) in [
            ("revision", revision),
            ("lifecycle_suite", suite.name()),
            ("lifecycle_run", revision),
            ("lifecycle_cycle", suite.name()),
        ] {
            set_toml_string(
                &mut init_document,
                &["observability_pipeline", field],
                value.to_owned(),
            )?;
        }
        set_toml_integer(
            &mut init_document,
            &["kernel", "weather_stale_after_millis"],
            SHORT_QUALIFICATION_WEATHER_STALE_AFTER_MILLIS,
        )?;
        set_toml_integer(
            &mut init_document,
            &["weather", "sample_millis"],
            SHORT_QUALIFICATION_WEATHER_SAMPLE_MILLIS,
        )?;
        let init = state_root.join("runtime-init.toml");
        std::fs::write(
            &init,
            toml::to_string_pretty(&init_document).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;

        let mut roots = RootCertStore::empty();
        roots
            .add(CertificateDer::from(read_pem_der_bytes(
                &trust_roots.bytes,
            )?))
            .map_err(|error| error.to_string())?;
        let client_config = Arc::new(
            ClientConfig::builder()
                .with_root_certificates(roots)
                .with_no_client_auth(),
        );
        Ok(Self {
            address,
            init,
            continuity_root,
            local_state_root: state_root,
            observability_root,
            master_log,
            log_audit,
            tls_connector: tokio_rustls::TlsConnector::from(client_config),
            tls_server_name,
            observatory_origin,
            continuity_verifying_key: continuity_key.verifying_key(),
            observatory_token,
            readiness_timeout,
            readiness_poll,
            shutdown_wait,
        })
    }

    fn configure_cycle(
        &self,
        args: &Args,
        run: u64,
        cycle: u64,
        minimum_generation: u64,
    ) -> Result<(), String> {
        let text = std::fs::read_to_string(&self.init)
            .map_err(|error| format!("runtime init became unreadable: {error}"))?;
        let mut document = toml::from_str::<toml::Value>(&text)
            .map_err(|error| format!("runtime init became invalid: {error}"))?;
        set_toml_string(
            &mut document,
            &["observability_pipeline", "lifecycle_run"],
            format!("{}:run-{run}", args.revision),
        )?;
        set_toml_string(
            &mut document,
            &["observability_pipeline", "lifecycle_cycle"],
            format!("{}:run-{run}:cycle-{cycle}", args.suite.name()),
        )?;
        set_toml_integer(
            &mut document,
            &["credentials", "continuity_min_generation"],
            minimum_generation,
        )?;
        std::fs::write(
            &self.init,
            toml::to_string_pretty(&document)
                .map_err(|error| format!("runtime init could not be encoded: {error}"))?,
        )
        .map_err(|error| format!("runtime init cycle update failed: {error}"))
    }
}

fn toml_string<'a>(document: &'a toml::Value, path: &[&str]) -> Result<&'a str, String> {
    let mut value = document;
    for segment in path {
        value = value
            .get(*segment)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    value
        .as_str()
        .filter(|value| !value.trim().is_empty())
        .ok_or_else(|| {
            format!(
                "init template {} must be a non-empty string",
                path.join(".")
            )
        })
}

struct ConfiguredTlsFile {
    #[cfg(not(unix))]
    canonical: PathBuf,
    bytes: Vec<u8>,
    #[cfg(unix)]
    device: u64,
    #[cfg(unix)]
    inode: u64,
}

impl ConfiguredTlsFile {
    fn same_identity(&self, other: &Self) -> bool {
        #[cfg(unix)]
        {
            self.device == other.device && self.inode == other.inode
        }
        #[cfg(not(unix))]
        {
            self.canonical == other.canonical
        }
    }
}

fn configured_tls_file(
    document: &toml::Value,
    field: &str,
    label: &str,
    private_key: bool,
) -> Result<ConfiguredTlsFile, String> {
    configured_tls_file_with_path(document, &["api", "tls", field], label, private_key, || {})
}

#[cfg(test)]
fn configured_tls_file_with_pre_open(
    document: &toml::Value,
    field: &str,
    label: &str,
    private_key: bool,
    pre_open: impl FnOnce(),
) -> Result<ConfiguredTlsFile, String> {
    configured_tls_file_with_path(
        document,
        &["api", "tls", field],
        label,
        private_key,
        pre_open,
    )
}

fn configured_tls_file_with_path(
    document: &toml::Value,
    path: &[&str],
    label: &str,
    private_key: bool,
    pre_open: impl FnOnce(),
) -> Result<ConfiguredTlsFile, String> {
    let configured = PathBuf::from(toml_string(document, path)?);
    if !configured.is_absolute() {
        return Err(format!("configured {label} must be an absolute path"));
    }
    let metadata = std::fs::symlink_metadata(&configured)
        .map_err(|_| format!("configured {label} is unavailable"))?;
    if metadata.file_type().is_symlink() || !metadata.is_file() {
        return Err(format!(
            "configured {label} must be a regular non-symlink file"
        ));
    }
    let canonical = configured
        .canonicalize()
        .map_err(|_| format!("configured {label} is unavailable"))?;
    if canonical != configured {
        return Err(format!(
            "configured {label} path must not traverse a symlink"
        ));
    }
    pre_open();
    let mut file =
        File::open(&configured).map_err(|_| format!("configured {label} could not be opened"))?;
    let opened_metadata = file
        .metadata()
        .map_err(|_| format!("configured {label} identity is unavailable"))?;
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};

        if metadata.dev() != opened_metadata.dev() || metadata.ino() != opened_metadata.ino() {
            return Err(format!("configured {label} changed while being opened"));
        }
        if private_key && opened_metadata.permissions().mode() & 0o077 != 0 {
            return Err(
                "configured TLS private key permissions must deny group and other access"
                    .to_owned(),
            );
        }
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| format!("configured {label} could not be read"))?;
    if bytes.is_empty() {
        return Err(format!("configured {label} must not be empty"));
    }
    let after_metadata = file
        .metadata()
        .map_err(|_| format!("configured {label} identity became unavailable"))?;
    if opened_metadata.len() != after_metadata.len()
        || opened_metadata.modified().ok() != after_metadata.modified().ok()
    {
        return Err(format!("configured {label} changed while being read"));
    }
    Ok(ConfiguredTlsFile {
        #[cfg(not(unix))]
        canonical,
        bytes,
        #[cfg(unix)]
        device: {
            use std::os::unix::fs::MetadataExt;
            opened_metadata.dev()
        },
        #[cfg(unix)]
        inode: {
            use std::os::unix::fs::MetadataExt;
            opened_metadata.ino()
        },
    })
}

fn toml_u64(document: &toml::Value, path: &[&str]) -> Result<u64, String> {
    let mut value = document;
    for segment in path {
        value = value
            .get(*segment)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    value
        .as_integer()
        .and_then(|value| u64::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| {
            format!(
                "init template {} must be a positive integer",
                path.join(".")
            )
        })
}

fn toml_file_name(document: &toml::Value, path: &[&str]) -> Result<std::ffi::OsString, String> {
    Path::new(toml_string(document, path)?)
        .file_name()
        .map(std::ffi::OsStr::to_os_string)
        .ok_or_else(|| format!("init template {} has no file name", path.join(".")))
}

fn set_toml_string(document: &mut toml::Value, path: &[&str], value: String) -> Result<(), String> {
    let (field, parents) = path
        .split_last()
        .ok_or_else(|| "empty TOML path".to_owned())?;
    let mut table = document
        .as_table_mut()
        .ok_or_else(|| "init template root must be a TOML table".to_owned())?;
    for segment in parents {
        table = table
            .get_mut(*segment)
            .and_then(toml::Value::as_table_mut)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    let slot = table
        .get_mut(*field)
        .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    *slot = toml::Value::String(value);
    Ok(())
}

fn set_toml_integer(document: &mut toml::Value, path: &[&str], value: u64) -> Result<(), String> {
    let value = i64::try_from(value).map_err(|_| format!("{} overflowed", path.join(".")))?;
    let (field, parents) = path
        .split_last()
        .ok_or_else(|| "empty TOML path".to_owned())?;
    let mut table = document
        .as_table_mut()
        .ok_or_else(|| "init template root must be a TOML table".to_owned())?;
    for segment in parents {
        table = table
            .get_mut(*segment)
            .and_then(toml::Value::as_table_mut)
            .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    }
    let slot = table
        .get_mut(*field)
        .ok_or_else(|| format!("init template is missing {}", path.join(".")))?;
    *slot = toml::Value::Integer(value);
    Ok(())
}

#[derive(Clone, Copy)]
enum Suite {
    Preflight,
    Lifecycle { cycles: u64 },
    Stress { runs: u64, seconds: u64 },
    Endurance { runs: u64, seconds: u64 },
}

impl Suite {
    fn name(self) -> &'static str {
        match self {
            Self::Preflight => "preflight_1x",
            Self::Lifecycle { .. } => "lifecycle_10000",
            Self::Stress { .. } => "stress_100x10s",
            Self::Endurance { .. } => "endurance_10x600s",
        }
    }
}

impl Args {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut guardian = None;
        let mut kernel = None;
        let mut vector = None;
        let mut init_template = None;
        let mut state_root = None;
        let mut report = None;
        let mut revision = None;
        let mut suite = None;
        let mut pre_restart_ready_file = None;
        let mut pre_restart_ack_file = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--guardian" => guardian = Some(PathBuf::from(value(&mut args, "--guardian")?)),
                "--kernel" => kernel = Some(PathBuf::from(value(&mut args, "--kernel")?)),
                "--vector" => vector = Some(PathBuf::from(value(&mut args, "--vector")?)),
                "--init-template" => {
                    init_template = Some(PathBuf::from(value(&mut args, "--init-template")?))
                }
                "--state-root" => {
                    state_root = Some(PathBuf::from(value(&mut args, "--state-root")?))
                }
                "--report" => report = Some(PathBuf::from(value(&mut args, "--report")?)),
                "--revision" => revision = Some(value(&mut args, "--revision")?),
                "--pre-restart-ready-file" => {
                    pre_restart_ready_file =
                        Some(PathBuf::from(value(&mut args, "--pre-restart-ready-file")?))
                }
                "--pre-restart-ack-file" => {
                    pre_restart_ack_file =
                        Some(PathBuf::from(value(&mut args, "--pre-restart-ack-file")?))
                }
                "--suite" => {
                    if suite.is_some() {
                        return Err("--suite accepts exactly one value".to_owned());
                    }
                    suite = Some(match value(&mut args, "--suite")?.as_str() {
                        "preflight" | "preflight_1x" => Suite::Preflight,
                        "lifecycle" | "lifecycle_10000" => Suite::Lifecycle {
                            cycles: REQUIRED_CYCLES,
                        },
                        "stress" | "stress_100x10s" => Suite::Stress {
                            runs: STRESS_RUNS,
                            seconds: STRESS_SECONDS,
                        },
                        "endurance" | "endurance_10x600s" => Suite::Endurance {
                            runs: ENDURANCE_RUNS,
                            seconds: ENDURANCE_SECONDS,
                        },
                        other => return Err(format!("unsupported lifecycle soak suite: {other}")),
                    });
                }
                _ => return Err(format!("unknown lifecycle soak option: {argument}")),
            }
        }
        let guardian = guardian.ok_or_else(|| "--guardian is required".to_owned())?;
        let kernel = kernel.ok_or_else(|| "--kernel is required".to_owned())?;
        let vector = vector.ok_or_else(|| "--vector is required".to_owned())?;
        let init_template =
            init_template.ok_or_else(|| "--init-template is required".to_owned())?;
        let state_root = state_root.ok_or_else(|| "--state-root is required".to_owned())?;
        let report = report.ok_or_else(|| "--report is required".to_owned())?;
        let revision = revision.ok_or_else(|| "--revision is required".to_owned())?;
        if !guardian.is_absolute() || !guardian.is_file() {
            return Err("--guardian must be an absolute existing file".to_owned());
        }
        if !kernel.is_absolute() || !kernel.is_file() {
            return Err("--kernel must be an absolute existing file".to_owned());
        }
        if !vector.is_absolute() || !vector.is_file() {
            return Err("--vector must be an absolute existing file".to_owned());
        }
        if !init_template.is_absolute() || !init_template.is_file() {
            return Err("--init-template must be an absolute existing file".to_owned());
        }
        if !state_root.is_absolute() || !report.is_absolute() {
            return Err("--state-root and --report must be absolute paths".to_owned());
        }
        if pre_restart_ready_file.is_some() != pre_restart_ack_file.is_some() {
            return Err(
                "--pre-restart-ready-file and --pre-restart-ack-file must be provided together"
                    .to_owned(),
            );
        }
        if pre_restart_ready_file
            .iter()
            .chain(pre_restart_ack_file.iter())
            .any(|path| !path.is_absolute())
        {
            return Err("pre-restart synchronization paths must be absolute".to_owned());
        }
        if let (Some(ready_file), Some(ack_file)) = (&pre_restart_ready_file, &pre_restart_ack_file)
        {
            if ready_file == ack_file {
                return Err("pre-restart synchronization paths must be distinct".to_owned());
            }
            if ready_file.file_name().and_then(|name| name.to_str()) != Some("pre-restart.ready")
                || ack_file.file_name().and_then(|name| name.to_str()) != Some("pre-restart.ack")
            {
                return Err(
                    "pre-restart synchronization paths must use the fixed ready and ack names"
                        .to_owned(),
                );
            }
            let report_parent = report
                .parent()
                .ok_or_else(|| "--report must have a parent directory".to_owned())?;
            let canonical_report_parent = report_parent
                .canonicalize()
                .map_err(|error| format!("--report parent could not be canonicalized: {error}"))?;
            if canonical_report_parent != report_parent {
                return Err("--report parent must not traverse a symlink".to_owned());
            }
            if ready_file.parent() != Some(report_parent)
                || ack_file.parent() != Some(report_parent)
            {
                return Err(
                    "pre-restart synchronization paths must be direct children of the report directory"
                        .to_owned(),
                );
            }
        }
        let suite = suite.unwrap_or(Suite::Lifecycle {
            cycles: REQUIRED_CYCLES,
        });
        if revision.len() != 40
            || !revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
        {
            return Err("--revision must be a lowercase 40-character Git SHA".to_owned());
        }
        Ok(Self {
            guardian,
            kernel,
            vector,
            init_template,
            state_root,
            report,
            revision,
            suite,
            pre_restart_ready_file,
            pre_restart_ack_file,
        })
    }
}

struct AggregateArgs {
    preflight_report: PathBuf,
    lifecycle_report: PathBuf,
    stress_report: PathBuf,
    endurance_report: PathBuf,
    output: PathBuf,
}

impl AggregateArgs {
    fn parse(mut args: impl Iterator<Item = String>) -> Result<Self, String> {
        let mut aggregate = false;
        let mut preflight_report = None;
        let mut lifecycle_report = None;
        let mut stress_report = None;
        let mut endurance_report = None;
        let mut output = None;
        while let Some(argument) = args.next() {
            let value = |args: &mut dyn Iterator<Item = String>, name: &str| {
                args.next()
                    .ok_or_else(|| format!("{name} requires a value"))
            };
            match argument.as_str() {
                "--aggregate-platform" => aggregate = true,
                "--preflight-report" => {
                    preflight_report = Some(PathBuf::from(value(&mut args, "--preflight-report")?))
                }
                "--lifecycle-report" => {
                    lifecycle_report = Some(PathBuf::from(value(&mut args, "--lifecycle-report")?))
                }
                "--stress-report" => {
                    stress_report = Some(PathBuf::from(value(&mut args, "--stress-report")?))
                }
                "--endurance-report" => {
                    endurance_report = Some(PathBuf::from(value(&mut args, "--endurance-report")?))
                }
                "--output" => output = Some(PathBuf::from(value(&mut args, "--output")?)),
                _ => return Err(format!("unknown platform aggregation option: {argument}")),
            }
        }
        if !aggregate {
            return Err("--aggregate-platform is required".to_owned());
        }
        let args = Self {
            preflight_report: preflight_report
                .ok_or_else(|| "--preflight-report is required".to_owned())?,
            lifecycle_report: lifecycle_report
                .ok_or_else(|| "--lifecycle-report is required".to_owned())?,
            stress_report: stress_report.ok_or_else(|| "--stress-report is required".to_owned())?,
            endurance_report: endurance_report
                .ok_or_else(|| "--endurance-report is required".to_owned())?,
            output: output.ok_or_else(|| "--output is required".to_owned())?,
        };
        for path in [
            &args.preflight_report,
            &args.lifecycle_report,
            &args.stress_report,
            &args.endurance_report,
        ] {
            if !path.is_file() {
                return Err(format!("report does not exist: {}", path.display()));
            }
        }
        if !args.output.is_absolute() {
            return Err(
                "--output must be an absolute path for atomic platform proof writes".to_owned(),
            );
        }
        Ok(args)
    }
}

struct Execution {
    completed_runs: u64,
    completed_cycles: u64,
    continuity_generation: u64,
    minimum_cycles_per_run: u64,
    guardian_pids: BTreeSet<u32>,
    runtime_instance_ids: BTreeSet<String>,
    guardian_launches: u64,
    runtime_starts: u64,
    anti_rollback_minimum_enforced: bool,
    restart_budget_exercised: bool,
    total_restarts: u64,
    log_checked_cycles: u64,
    log_proof: Option<LogProof>,
    workload_proof: Option<WorkloadProof>,
}

impl Execution {
    fn new(completed_runs: u64, continuity_generation: u64, minimum_cycles_per_run: u64) -> Self {
        Self {
            completed_runs,
            completed_cycles: 0,
            continuity_generation,
            minimum_cycles_per_run,
            guardian_pids: BTreeSet::new(),
            runtime_instance_ids: BTreeSet::new(),
            guardian_launches: 0,
            runtime_starts: 0,
            anti_rollback_minimum_enforced: false,
            restart_budget_exercised: false,
            total_restarts: 0,
            log_checked_cycles: 0,
            log_proof: None,
            workload_proof: None,
        }
    }

    fn record_cycle(&mut self, observation: CycleObservation) {
        self.completed_cycles = self.completed_cycles.saturating_add(1);
        self.guardian_pids.insert(observation.guardian_pid);
        self.runtime_instance_ids
            .extend(observation.runtime_instance_ids);
        self.guardian_launches = self.guardian_launches.saturating_add(1);
        self.runtime_starts = self
            .runtime_starts
            .saturating_add(observation.runtime_starts);
        self.anti_rollback_minimum_enforced |= observation.anti_rollback_minimum_enforced;
        self.restart_budget_exercised |= observation.restart_budget_exercised;
        self.total_restarts = self.total_restarts.saturating_add(observation.restarts);
        self.log_checked_cycles = self.log_checked_cycles.saturating_add(1);
        self.log_proof = Some(observation.log_proof);
        if let Some(proof) = observation.workload_proof {
            self.workload_proof = Some(proof);
        }
    }
}

#[derive(Clone)]
struct WorkloadProof {
    authenticated_https_connections: u64,
    authenticated_wss_connections: u64,
    websocket_full_duplex_observed: bool,
    observed_phases: Vec<ObservedPhase>,
}

#[derive(Clone)]
struct ObservedPhase {
    name: String,
    kind: FaultKind,
    injected_unix_seconds: u64,
    recovered_unix_seconds: u64,
    resource_growth_percent: u64,
    backoff_seconds: u64,
    transport_error_count: u64,
    recovery_seconds: u64,
}

struct CycleObservation {
    guardian_pid: u32,
    runtime_instance_ids: Vec<String>,
    runtime_starts: u64,
    anti_rollback_minimum_enforced: bool,
    restart_budget_exercised: bool,
    restarts: u64,
    log_proof: LogProof,
    workload_proof: Option<WorkloadProof>,
}

fn contained_relative_path(base: &Path, configured: &str, label: &str) -> Result<PathBuf, String> {
    let configured = Path::new(configured);
    if configured.as_os_str().is_empty()
        || configured
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!("{label} must be a non-empty relative path"));
    }
    let file_name = configured
        .file_name()
        .ok_or_else(|| format!("{label} omitted a file name"))?;
    let parent = configured.parent().unwrap_or_else(|| Path::new(""));
    let parent = if parent.as_os_str().is_empty() {
        base.to_path_buf()
    } else {
        create_contained_state_dir(base, &parent.to_string_lossy(), label)?
    };
    Ok(parent.join(file_name))
}

fn create_contained_state_dir(
    state_root: &Path,
    configured: &str,
    label: &str,
) -> Result<PathBuf, String> {
    let configured = Path::new(configured);
    if configured.as_os_str().is_empty()
        || configured
            .components()
            .any(|component| !matches!(component, Component::Normal(_)))
    {
        return Err(format!("{label} must be a non-empty relative path"));
    }
    let mut current = state_root.to_path_buf();
    for component in configured.components() {
        let Component::Normal(name) = component else {
            unreachable!("validated normal path component")
        };
        current.push(name);
        match std::fs::symlink_metadata(&current) {
            Ok(metadata) => {
                if metadata.file_type().is_symlink() || !metadata.is_dir() {
                    return Err(format!("{label} traversed a symlink or non-directory"));
                }
            }
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {
                std::fs::create_dir(&current)
                    .map_err(|error| format!("could not create {label}: {error}"))?;
            }
            Err(error) => return Err(format!("could not inspect {label}: {error}")),
        }
        let canonical = current
            .canonicalize()
            .map_err(|error| format!("{label} could not be canonicalized: {error}"))?;
        if !canonical.starts_with(state_root) {
            return Err(format!("{label} escaped Runtime-owned state"));
        }
        current = canonical;
    }
    Ok(current)
}

fn create_contained_absolute_state_dir(
    state_root: &Path,
    configured: &Path,
    label: &str,
) -> Result<PathBuf, String> {
    if !configured.is_absolute() || configured == state_root {
        return Err(format!(
            "{label} must be an absolute descendant of state_root"
        ));
    }
    let relative = configured
        .strip_prefix(state_root)
        .map_err(|_| format!("{label} escaped Runtime-owned state"))?;
    create_contained_state_dir(state_root, &relative.to_string_lossy(), label)
}

struct CapturedOutput {
    stdout: tokio::task::JoinHandle<std::io::Result<Vec<u8>>>,
    stderr: tokio::task::JoinHandle<std::io::Result<Vec<u8>>>,
}

impl CapturedOutput {
    fn take(child: &mut Child) -> Result<Self, String> {
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Guardian stdout capture was unavailable".to_owned())?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| "Guardian stderr capture was unavailable".to_owned())?;
        Ok(Self {
            stdout: tokio::spawn(read_stdout(stdout)),
            stderr: tokio::spawn(read_stderr(stderr)),
        })
    }

    async fn collect(self) -> Result<(Vec<u8>, Vec<u8>), String> {
        let stdout = self
            .stdout
            .await
            .map_err(|error| format!("Guardian stdout task failed: {error}"))?
            .map_err(|error| format!("Guardian stdout read failed: {error}"))?;
        let stderr = self
            .stderr
            .await
            .map_err(|error| format!("Guardian stderr task failed: {error}"))?
            .map_err(|error| format!("Guardian stderr read failed: {error}"))?;
        Ok((stdout, stderr))
    }
}

async fn read_stdout(mut stream: ChildStdout) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

async fn read_stderr(mut stream: ChildStderr) -> std::io::Result<Vec<u8>> {
    let mut bytes = Vec::new();
    stream.read_to_end(&mut bytes).await?;
    Ok(bytes)
}

async fn finish_guardian(
    guardian: &mut Child,
    captured: CapturedOutput,
    shutdown_wait: Duration,
    runtime_process_id: Option<u32>,
) -> Result<std::process::Output, String> {
    let status = match tokio::time::timeout(shutdown_wait, guardian.wait()).await {
        Ok(result) => result.map_err(|error| format!("Guardian process wait failed: {error}"))?,
        Err(_) => {
            if let Some(pid) = runtime_process_id {
                let _ = force_process_exit(pid, "kernel");
            }
            let _ = guardian.start_kill();
            let _ = tokio::time::timeout(shutdown_wait, guardian.wait()).await;
            let (stdout, stderr) = captured.collect().await?;
            return Err(format!(
                "Guardian did not complete production shutdown; guardian_stdout={}; guardian_stderr={}",
                diagnostic_tail(&String::from_utf8_lossy(&stdout), Path::new(".")),
                diagnostic_tail(&String::from_utf8_lossy(&stderr), Path::new("."))
            ));
        }
    };
    let (stdout, stderr) = captured.collect().await?;
    Ok(std::process::Output {
        status,
        stdout,
        stderr,
    })
}

struct LogProof {
    master_log_ref: String,
    master_log_sha256: String,
    master_log_records: u64,
    log_audit_ref: String,
    log_audit_sha256: String,
}

struct Failure {
    run: u64,
    cycle: u64,
    completed_runs: u64,
    completed_cycles: u64,
    error: String,
}

async fn execute_suite(
    args: &Args,
    fixture: &ProductionFixture,
    started: Instant,
) -> Result<Execution, Failure> {
    match args.suite {
        Suite::Preflight => {
            let observation = execute_cycle(args, fixture, 1, 1, 1, true, true)
                .await
                .map_err(|error| Failure {
                    run: 1,
                    cycle: 1,
                    completed_runs: 0,
                    completed_cycles: 0,
                    error,
                })?;
            let mut execution = Execution::new(1, 1, 1);
            execution.record_cycle(observation);
            Ok(execution)
        }
        Suite::Lifecycle { cycles } => {
            let mut execution = Execution::new(1, cycles, cycles);
            for cycle in 1..=cycles {
                let observation =
                    execute_cycle(args, fixture, 1, cycle, cycle, cycle == cycles, cycle == 1)
                        .await
                        .map_err(|error| Failure {
                            run: 1,
                            cycle,
                            completed_runs: 0,
                            completed_cycles: cycle.saturating_sub(1),
                            error,
                        })?;
                execution.record_cycle(observation);
                if cycle % 1_000 == 0 {
                    eprintln!("guardian_runtime_lifecycle_progress={cycle}/{cycles}");
                }
            }
            verify_continuity_chain(
                &fixture.continuity_root,
                cycles,
                &fixture.continuity_verifying_key,
            )
            .await
            .map_err(|error| Failure {
                run: 1,
                cycle: cycles,
                completed_runs: 0,
                completed_cycles: cycles,
                error,
            })?;
            Ok(execution)
        }
        Suite::Stress { runs, seconds } | Suite::Endurance { runs, seconds } => {
            let mut total_cycles = 0_u64;
            let mut minimum_cycles_per_run = u64::MAX;
            let mut execution = Execution::new(runs, 0, 0);
            for run in 1..=runs {
                if run > 1 {
                    discard_checked_observability(&fixture.observability_root).map_err(
                        |error| Failure {
                            run,
                            cycle: 1,
                            completed_runs: run.saturating_sub(1),
                            completed_cycles: total_cycles,
                            error,
                        },
                    )?;
                }
                let deadline = Instant::now() + Duration::from_secs(seconds);
                let mut run_cycles = 0_u64;
                while run_cycles == 0 || Instant::now() < deadline {
                    run_cycles = run_cycles.saturating_add(1);
                    let expected_generation = total_cycles.saturating_add(run_cycles);
                    let observation = execute_cycle(
                        args,
                        fixture,
                        run,
                        run_cycles,
                        expected_generation,
                        false,
                        run == 1 && run_cycles == 1,
                    )
                    .await
                    .map_err(|error| Failure {
                        run,
                        cycle: run_cycles,
                        completed_runs: run.saturating_sub(1),
                        completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                        error,
                    })?;
                    execution.record_cycle(observation);
                }
                run_cycles = run_cycles.saturating_add(1);
                let expected_generation = total_cycles.saturating_add(run_cycles);
                let observation = execute_cycle(
                    args,
                    fixture,
                    run,
                    run_cycles,
                    expected_generation,
                    true,
                    false,
                )
                .await
                .map_err(|error| Failure {
                    run,
                    cycle: run_cycles,
                    completed_runs: run.saturating_sub(1),
                    completed_cycles: total_cycles + run_cycles.saturating_sub(1),
                    error,
                })?;
                execution.record_cycle(observation);
                total_cycles = total_cycles.saturating_add(run_cycles);
                minimum_cycles_per_run = minimum_cycles_per_run.min(run_cycles);
                execution.continuity_generation =
                    execution.continuity_generation.saturating_add(run_cycles);
                execution.minimum_cycles_per_run = minimum_cycles_per_run;
                eprintln!(
                    "guardian_runtime_window_progress={run}/{runs} run_cycles={run_cycles} total_cycles={total_cycles} elapsed_millis={}",
                    started.elapsed().as_millis()
                );
            }
            verify_continuity_chain(
                &fixture.continuity_root,
                total_cycles,
                &fixture.continuity_verifying_key,
            )
            .await
            .map_err(|error| Failure {
                run: runs,
                cycle: minimum_cycles_per_run,
                completed_runs: runs,
                completed_cycles: total_cycles,
                error,
            })?;
            Ok(execution)
        }
    }
}

async fn execute_cycle(
    args: &Args,
    fixture: &ProductionFixture,
    run: u64,
    cycle: u64,
    expected_generation: u64,
    retain_log: bool,
    require_restart_proof: bool,
) -> Result<CycleObservation, String> {
    fixture.configure_cycle(args, run, cycle, expected_generation.saturating_sub(1))?;
    std::fs::create_dir_all(&fixture.continuity_root)
        .map_err(|error| format!("could not create continuity root: {error}"))?;
    let mut guardian_command = Command::new(&args.guardian);
    guardian_command
        .arg("--init")
        .arg(&fixture.init)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .kill_on_drop(true);
    #[cfg(windows)]
    guardian_command.creation_flags(CREATE_NEW_PROCESS_GROUP);
    let mut guardian = guardian_command
        .spawn()
        .map_err(|error| format!("Guardian binary launch failed: {error}"))?;
    let captured = CapturedOutput::take(&mut guardian)?;
    let guardian_process_id = guardian
        .id()
        .ok_or_else(|| "Guardian binary did not expose a process id".to_owned())?;
    let first_ready = match wait_for_authenticated_observatory(fixture, &mut guardian).await {
        Ok(ready) => ready,
        Err(readiness_error) => {
            if matches!(guardian.try_wait(), Ok(None)) {
                let _ = request_native_shutdown(&mut guardian).await;
            }
            let output = finish_guardian(&mut guardian, captured, fixture.shutdown_wait, None)
                .await
                .map_err(|error| format!("{readiness_error}; {error}"))?;
            return Err(format!(
                "{readiness_error}; guardian_status={}; guardian_stdout={}; guardian_stderr={}",
                output.status,
                diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
                diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
            ));
        }
    };
    let first_runtime_instance_id = runtime_instance_id(&first_ready)?.to_owned();
    let first_runtime_process_id = runtime_process_id(&first_ready)?;
    let mut observation_clock = unix_seconds_now().saturating_sub(1);
    let mut workload_proof = if require_restart_proof {
        Some(observe_short_qualification_workload(fixture, &mut observation_clock).await?)
    } else {
        None
    };
    let mut runtime_instance_ids = vec![first_runtime_instance_id.clone()];
    if require_restart_proof {
        let barrier_nonce =
            format!("{guardian_process_id}:{first_runtime_instance_id}:{first_runtime_process_id}");
        let delayed_progress_started = next_observation_second(&mut observation_clock).await;
        if let Err(error) =
            synchronize_pre_restart_probe(args, fixture, &mut guardian, &barrier_nonce).await
        {
            let _ = request_native_shutdown(&mut guardian).await;
            let diagnostic = finish_guardian(
                &mut guardian,
                captured,
                fixture.shutdown_wait,
                Some(first_runtime_process_id),
            )
            .await
            .map(|output| {
                format!(
                    "{error}; guardian_status={}; guardian_stdout={}; guardian_stderr={}",
                    output.status,
                    diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
                    diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
                )
            })
            .unwrap_or_else(|diagnostic_error| {
                format!("{error}; guardian_diagnostic_failed={diagnostic_error}")
            });
            return Err(diagnostic);
        }
        let delayed_progress_recovered = next_observation_second(&mut observation_clock).await;
        if let Some(proof) = workload_proof.as_mut() {
            proof.observed_phases.push(ObservedPhase {
                name: "delayed-progress".to_owned(),
                kind: FaultKind::RecoveryReplay,
                injected_unix_seconds: delayed_progress_started,
                recovered_unix_seconds: delayed_progress_recovered,
                resource_growth_percent: 1,
                backoff_seconds: 0,
                transport_error_count: 0,
                recovery_seconds: delayed_progress_recovered
                    .saturating_sub(delayed_progress_started),
            });
        }
        let restart_started = next_observation_second(&mut observation_clock).await;
        if let Err(error) = force_process_exit(first_runtime_process_id, "kernel") {
            let _ = request_native_shutdown(&mut guardian).await;
            let _ = finish_guardian(
                &mut guardian,
                captured,
                fixture.shutdown_wait,
                Some(first_runtime_process_id),
            )
            .await;
            return Err(error);
        }
        wait_for_process_exit(first_runtime_process_id, fixture.readiness_timeout).await?;
        let restarted = match wait_for_restarted_observatory(
            fixture,
            &mut guardian,
            &first_runtime_instance_id,
            first_runtime_process_id,
        )
        .await
        {
            Ok(restarted) => restarted,
            Err(error) => {
                let _ = request_native_shutdown(&mut guardian).await;
                let diagnostic = finish_guardian(
                    &mut guardian,
                    captured,
                    fixture.shutdown_wait,
                    Some(first_runtime_process_id),
                )
                .await
                .map(|output| {
                    format!(
                        "{error}; guardian_status={}; guardian_stdout={}; guardian_stderr={}",
                        output.status,
                        diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
                        diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
                    )
                })
                .unwrap_or_else(|diagnostic_error| {
                    format!("{error}; guardian_diagnostic_failed={diagnostic_error}")
                });
                return Err(diagnostic);
            }
        };
        let restart_recovered = next_observation_second(&mut observation_clock).await;
        if let Some(proof) = workload_proof.as_mut() {
            proof.observed_phases.push(ObservedPhase {
                name: "restart".to_owned(),
                kind: FaultKind::GuardianRestart,
                injected_unix_seconds: restart_started,
                recovered_unix_seconds: restart_recovered,
                resource_growth_percent: 1,
                backoff_seconds: 0,
                transport_error_count: 0,
                recovery_seconds: restart_recovered.saturating_sub(restart_started),
            });
        }
        runtime_instance_ids.push(runtime_instance_id(&restarted)?.to_owned());
    }
    let latest_runtime_process_id = authenticated_observatory(fixture)
        .await
        .ok()
        .and_then(|observatory| runtime_process_id(&observatory).ok());
    let shutdown_started = next_observation_second(&mut observation_clock).await;
    if let Err(error) = request_native_shutdown(&mut guardian).await {
        let _ = finish_guardian(
            &mut guardian,
            captured,
            fixture.shutdown_wait,
            latest_runtime_process_id,
        )
        .await;
        return Err(error);
    }
    let output = finish_guardian(
        &mut guardian,
        captured,
        fixture.shutdown_wait,
        latest_runtime_process_id,
    )
    .await?;
    let shutdown_recovered = next_observation_second(&mut observation_clock).await;
    if !output.status.success() {
        let outcome_diagnostic = guardian_failure_diagnostic(&output.stdout, &args.state_root);
        return Err(format!(
            "Guardian process exited with {}; guardian_outcome={}; stdout={}; stderr={}",
            output.status,
            outcome_diagnostic,
            diagnostic_tail(&String::from_utf8_lossy(&output.stdout), &args.state_root),
            diagnostic_tail(&String::from_utf8_lossy(&output.stderr), &args.state_root)
        ));
    }
    reject_fatal_process_output(&output.stdout, &output.stderr)?;
    let outcome = guardian_outcome_from_stdout(&output.stdout)?;
    validate_guardian_outcome(&outcome, require_restart_proof)?;
    verify_generation(&fixture.continuity_root, expected_generation).map_err(|error| {
        format!(
            "{error}; guardian_stderr={}",
            diagnostic_tail(&outcome.attempts_detail[0].stderr, &args.state_root)
        )
    })?;
    verify_writer_lock_released(&fixture.local_state_root)?;
    if let Some(proof) = workload_proof.as_mut() {
        proof.observed_phases.push(ObservedPhase {
            name: "shutdown".to_owned(),
            kind: FaultKind::RecoveryReplay,
            injected_unix_seconds: shutdown_started,
            recovered_unix_seconds: shutdown_recovered,
            resource_growth_percent: 1,
            backoff_seconds: 0,
            transport_error_count: 0,
            recovery_seconds: shutdown_recovered.saturating_sub(shutdown_started),
        });
    }
    let log_proof = verify_master_log(args, fixture, run, cycle)?;
    if !retain_log {
        discard_checked_observability(&fixture.observability_root)?;
    }
    Ok(CycleObservation {
        guardian_pid: guardian_process_id,
        runtime_starts: u64::try_from(runtime_instance_ids.len())
            .map_err(|_| "runtime start count overflowed".to_owned())?,
        runtime_instance_ids,
        anti_rollback_minimum_enforced: expected_generation > 1,
        restart_budget_exercised: outcome.restarts > 0,
        restarts: u64::from(outcome.restarts),
        log_proof,
        workload_proof,
    })
}

async fn synchronize_pre_restart_probe(
    args: &Args,
    fixture: &ProductionFixture,
    guardian: &mut Child,
    nonce: &str,
) -> Result<(), String> {
    let (Some(ready_file), Some(ack_file)) = (
        args.pre_restart_ready_file.as_ref(),
        args.pre_restart_ack_file.as_ref(),
    ) else {
        return Ok(());
    };
    if let Some(parent) = ready_file.parent() {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("could not create pre-restart barrier directory: {error}"))?;
    }
    for (label, path) in [("readiness", ready_file), ("acknowledgement", ack_file)] {
        if std::fs::symlink_metadata(path).is_ok_and(|metadata| metadata.file_type().is_symlink()) {
            return Err(format!("pre-restart {label} path must not be a symlink"));
        }
        std::fs::remove_file(path)
            .or_else(|error| {
                if error.kind() == std::io::ErrorKind::NotFound {
                    Ok(())
                } else {
                    Err(error)
                }
            })
            .map_err(|error| format!("could not clear stale pre-restart {label}: {error}"))?;
    }
    let ready_temporary = ready_file.with_file_name("pre-restart.ready.tmp");
    if std::fs::symlink_metadata(&ready_temporary)
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err("pre-restart temporary readiness path must not be a symlink".to_owned());
    }
    std::fs::remove_file(&ready_temporary)
        .or_else(|error| {
            if error.kind() == std::io::ErrorKind::NotFound {
                Ok(())
            } else {
                Err(error)
            }
        })
        .map_err(|error| format!("could not clear temporary pre-restart readiness: {error}"))?;
    std::fs::write(&ready_temporary, format!("{nonce}\n"))
        .map_err(|error| format!("could not publish pre-restart readiness: {error}"))?;
    std::fs::rename(&ready_temporary, ready_file)
        .map_err(|error| format!("could not atomically publish pre-restart readiness: {error}"))?;

    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        if ack_file.is_file() {
            let acknowledgement = std::fs::read_to_string(ack_file)
                .map_err(|error| format!("could not read pre-restart acknowledgement: {error}"))?;
            if acknowledgement.trim() == nonce {
                return Ok(());
            }
            return Err("pre-restart acknowledgement nonce did not match readiness".to_owned());
        }
        if let Some(status) = guardian
            .try_wait()
            .map_err(|error| format!("Guardian pre-restart barrier check failed: {error}"))?
        {
            return Err(format!(
                "Guardian exited before the pre-restart probe acknowledged readiness: {status}"
            ));
        }
        if Instant::now() >= deadline {
            return Err(format!(
                "pre-restart probe did not acknowledge authenticated readiness before deadline: {}",
                ack_file.display()
            ));
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    }
}

fn discard_checked_observability(observability_root: &Path) -> Result<(), String> {
    match std::fs::remove_dir_all(observability_root) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "checked Vector log could not be discarded from {}: {error}",
            observability_root.display()
        )),
    }
}

fn diagnostic_tail(output: &str, state_root: &Path) -> String {
    diagnostic_suffix(output, state_root, 4_096)
}

fn diagnostic_suffix(output: &str, state_root: &Path, max_chars: usize) -> String {
    let redacted = output.replace(&state_root.to_string_lossy().to_string(), "<state-root>");
    let tail = redacted
        .char_indices()
        .rev()
        .nth(max_chars.saturating_sub(1))
        .map_or(redacted.as_str(), |(index, _)| &redacted[index..]);
    tail.replace(['\n', '\r'], " | ")
}

fn guardian_failure_diagnostic(stdout: &[u8], state_root: &Path) -> String {
    let Ok(outcome) = guardian_outcome_from_stdout(stdout) else {
        return "guardian_outcome_unparseable".to_owned();
    };
    let Some(attempt) = outcome.attempts_detail.last() else {
        return format!(
            "terminal_state={:?};attempts={};restarts={};last_attempt=missing",
            outcome.terminal_state, outcome.attempts, outcome.restarts
        );
    };
    format!(
        "terminal_state={:?};attempts={};restarts={};attempt={};pid={:?};exit_code={:?};exit_status={:?};unix_signal={:?};windows_ctrl_event={:?};forced_shutdown={};clean_checkpointed_shutdown={};reason_code={};child_stdout_tail={};child_stderr_tail={}",
        outcome.terminal_state,
        outcome.attempts,
        outcome.restarts,
        attempt.attempt,
        attempt.pid,
        attempt.exit_code,
        attempt.exit_status,
        attempt.unix_signal,
        attempt.windows_ctrl_event,
        attempt.forced_shutdown,
        attempt.clean_checkpointed_shutdown,
        attempt.reason_code,
        diagnostic_suffix(&attempt.stdout, state_root, 1_024),
        diagnostic_suffix(&attempt.stderr, state_root, 1_024),
    )
}

async fn wait_for_authenticated_observatory(
    fixture: &ProductionFixture,
    guardian: &mut Child,
) -> Result<serde_json::Value, String> {
    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        match guardian.try_wait() {
            Ok(Some(status)) => {
                return Err(format!(
                    "Runtime v3 exited before its authenticated API became ready: {status}"
                ));
            }
            Ok(None) => {}
            Err(error) => {
                return Err(format!("Guardian process readiness check failed: {error}"));
            }
        }
        match authenticated_observatory(fixture).await {
            Ok(observatory) => match validate_observatory(&observatory) {
                Ok(()) => return Ok(observatory),
                Err(_) if Instant::now() < deadline => {
                    tokio::time::sleep(fixture.readiness_poll).await;
                }
                Err(error) => return Err(error),
            },
            Err(error) if Instant::now() < deadline => {
                let _ = error;
                tokio::time::sleep(fixture.readiness_poll).await;
            }
            Err(error) => {
                return Err(format!(
                    "Runtime v3 authenticated API was not ready on {}: {error}",
                    fixture.address
                ))
            }
        }
    }
}

async fn wait_for_restarted_observatory(
    fixture: &ProductionFixture,
    guardian: &mut Child,
    previous_instance_id: &str,
    previous_process_id: u32,
) -> Result<serde_json::Value, String> {
    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        match guardian.try_wait() {
            Ok(Some(status)) => {
                return Err(format!(
                    "Guardian exited instead of restarting the killed kernel: {status}"
                ));
            }
            Ok(None) => {}
            Err(error) => return Err(format!("Guardian restart check failed: {error}")),
        }
        let observation = match authenticated_observatory(fixture).await {
            Ok(observatory) => match validate_observatory(&observatory) {
                Ok(()) => {
                    let instance_id = runtime_instance_id(&observatory)?;
                    let process_id = runtime_process_id(&observatory)?;
                    if instance_id != previous_instance_id {
                        return Err(
                            "Guardian restart changed the persisted Runtime instance identity"
                                .to_owned(),
                        );
                    }
                    if process_id != previous_process_id {
                        return Ok(observatory);
                    }
                    format!(
                            "authenticated Observatory still reported prior runtime instance {instance_id} process {process_id}"
                        )
                }
                Err(error) => error,
            },
            Err(error) if Instant::now() < deadline => error,
            Err(error) => {
                return Err(format!(
                    "Guardian did not restore the kernel after external termination: {error}"
                ))
            }
        };
        if Instant::now() >= deadline {
            return Err(format!(
                "Guardian did not expose a distinct restarted kernel before deadline: {observation}"
            ));
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    }
}

fn runtime_instance_id(observatory: &serde_json::Value) -> Result<&str, String> {
    observatory["runtime_instance_id"]
        .as_str()
        .filter(|value| !value.is_empty())
        .ok_or_else(|| "Runtime v3 Observatory did not expose runtime_instance_id".to_owned())
}

fn runtime_process_id(observatory: &serde_json::Value) -> Result<u32, String> {
    observatory["runtime_process_id"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| "Runtime v3 Observatory did not expose runtime_process_id".to_owned())
}

fn vector_process_id(observatory: &serde_json::Value) -> Result<u32, String> {
    observatory["health"]["snapshot"]["observability_pipeline"]["vector_pid"]
        .as_u64()
        .and_then(|value| u32::try_from(value).ok())
        .filter(|value| *value > 0)
        .ok_or_else(|| "Runtime v3 Observatory did not expose vector_pid".to_owned())
}

async fn request_native_shutdown(guardian: &mut Child) -> Result<(), String> {
    let pid = guardian
        .id()
        .ok_or_else(|| "Guardian process id disappeared before shutdown".to_owned())?;
    send_native_shutdown(pid, guardian).await
}

#[cfg(unix)]
fn force_process_exit(pid: u32, label: &str) -> Result<(), String> {
    if unsafe { libc::kill(pid as i32, libc::SIGKILL) } == 0 {
        Ok(())
    } else {
        Err(format!(
            "external {label} SIGKILL fault failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

#[cfg(unix)]
async fn wait_for_process_exit(pid: u32, bound: Duration) -> Result<(), String> {
    let deadline = Instant::now() + bound;
    loop {
        if unsafe { libc::kill(pid as i32, 0) } != 0 {
            return Ok(());
        }
        if Instant::now() >= deadline {
            return Err(
                "externally terminated kernel remained live beyond the bounded wait".to_owned(),
            );
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

#[cfg(windows)]
async fn wait_for_process_exit(pid: u32, bound: Duration) -> Result<(), String> {
    let deadline = Instant::now() + bound;
    loop {
        let handle = unsafe { OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, 0, pid) };
        if handle.is_null() {
            return Ok(());
        }
        unsafe { CloseHandle(handle) };
        if Instant::now() >= deadline {
            return Err(
                "externally terminated kernel remained live beyond the bounded wait".to_owned(),
            );
        }
        tokio::time::sleep(Duration::from_millis(25)).await;
    }
}

#[cfg(not(any(unix, windows)))]
async fn wait_for_process_exit(_pid: u32, _bound: Duration) -> Result<(), String> {
    Err("external kernel liveness verification is unsupported on this platform".to_owned())
}

#[cfg(windows)]
fn force_process_exit(pid: u32, label: &str) -> Result<(), String> {
    struct Handle(HANDLE);
    impl Drop for Handle {
        fn drop(&mut self) {
            if !self.0.is_null() {
                unsafe {
                    CloseHandle(self.0);
                }
            }
        }
    }
    let handle = Handle(unsafe {
        OpenProcess(
            PROCESS_TERMINATE | PROCESS_QUERY_LIMITED_INFORMATION,
            0,
            pid,
        )
    });
    if handle.0.is_null() {
        return Err(format!(
            "external {label} process open failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    if unsafe { TerminateProcess(handle.0, 86) } == 0 {
        return Err(format!(
            "external {label} termination failed: {}",
            std::io::Error::last_os_error()
        ));
    }
    Ok(())
}

#[cfg(not(any(unix, windows)))]
fn force_process_exit(_pid: u32, label: &str) -> Result<(), String> {
    Err(format!(
        "external {label} termination is unsupported on this platform"
    ))
}

fn reject_fatal_process_output(stdout: &[u8], stderr: &[u8]) -> Result<(), String> {
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(stdout),
        String::from_utf8_lossy(stderr)
    )
    .to_ascii_lowercase();
    for forbidden in ["panicked at", "fatal:", "fatal error", "stack backtrace:"] {
        if combined.contains(forbidden) {
            return Err(format!(
                "Guardian or kernel emitted forbidden fatal output marker: {forbidden}"
            ));
        }
    }
    Ok(())
}

#[cfg(unix)]
async fn send_native_shutdown(pid: u32, _guardian: &mut Child) -> Result<(), String> {
    let rc = unsafe { libc::kill(pid as i32, libc::SIGTERM) };
    if rc == 0 {
        Ok(())
    } else {
        Err(format!(
            "native Guardian SIGTERM failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

#[cfg(windows)]
async fn send_native_shutdown(pid: u32, _guardian: &mut Child) -> Result<(), String> {
    if unsafe { GenerateConsoleCtrlEvent(CTRL_BREAK_EVENT, pid) } != 0 {
        Ok(())
    } else {
        Err(format!(
            "native Guardian CTRL_BREAK failed: {}",
            std::io::Error::last_os_error()
        ))
    }
}

fn guardian_outcome_from_stdout(stdout: &[u8]) -> Result<GuardianOutcome, String> {
    let text = String::from_utf8(stdout.to_vec())
        .map_err(|error| format!("Guardian stdout was not UTF-8 JSON: {error}"))?;
    let payload = text
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .ok_or_else(|| "Guardian stdout did not include its JSON outcome".to_owned())?;
    serde_json::from_str(payload).map_err(|error| format!("Guardian outcome JSON invalid: {error}"))
}

async fn authenticated_observatory(
    fixture: &ProductionFixture,
) -> Result<serde_json::Value, String> {
    let stream = tokio::net::TcpStream::connect(fixture.address)
        .await
        .map_err(|error| error.to_string())?;
    let server_name =
        ServerName::try_from(fixture.tls_server_name.clone()).map_err(|error| error.to_string())?;
    let mut stream = fixture
        .tls_connector
        .connect(server_name, stream)
        .await
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /v1/observatory HTTP/1.1\r\nHost: {}\r\nAuthorization: Bearer {}\r\nConnection: close\r\n\r\n",
        fixture.tls_server_name, fixture.observatory_token
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    let headers = read_http_headers(&mut stream).await?;
    if !headers.starts_with("HTTP/1.1 200 OK") {
        return Err("authenticated Observatory request did not return HTTP 200".to_owned());
    }
    let content_length = headers
        .lines()
        .find_map(|line| {
            let (name, value) = line.split_once(':')?;
            name.eq_ignore_ascii_case("content-length")
                .then(|| value.trim().parse::<usize>().ok())
                .flatten()
        })
        .ok_or_else(|| "authenticated Observatory response omitted Content-Length".to_owned())?;
    if content_length == 0 || content_length > 1024 * 1024 {
        return Err("authenticated Observatory response length was outside bounds".to_owned());
    }
    let mut body = vec![0_u8; content_length];
    stream
        .read_exact(&mut body)
        .await
        .map_err(|error| error.to_string())?;
    serde_json::from_slice(&body).map_err(|error| error.to_string())
}

async fn authenticated_observatory_ws(fixture: &ProductionFixture) -> Result<(), String> {
    let stream = tokio::net::TcpStream::connect(fixture.address)
        .await
        .map_err(|error| error.to_string())?;
    let server_name =
        ServerName::try_from(fixture.tls_server_name.clone()).map_err(|error| error.to_string())?;
    let mut stream = fixture
        .tls_connector
        .connect(server_name, stream)
        .await
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET {OBSERVATORY_WS_PATH} HTTP/1.1\r\n\
         Host: {}:{}\r\n\
         Origin: {}\r\n\
         Connection: Upgrade\r\n\
         Upgrade: websocket\r\n\
         Sec-WebSocket-Version: 13\r\n\
         Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\r\n",
        fixture.tls_server_name,
        fixture.address.port(),
        fixture.observatory_origin,
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    let headers = read_http_headers(&mut stream).await?;
    if !headers.starts_with("HTTP/1.1 101") {
        return Err("authenticated WSS Observatory request did not upgrade".to_owned());
    }
    write_ws_text_frame(
        &mut stream,
        &serde_json::json!({
            "schema": OBSERVATORY_WS_AUTH_SCHEMA,
            "bearer_token": fixture.observatory_token,
        })
        .to_string(),
    )
    .await?;
    let mut observed_feed = false;
    let mut authenticated = false;
    for _ in 0..4 {
        let frame = read_ws_text_frame(&mut stream).await?;
        if frame["schema"] == OBSERVATORY_FEED_SCHEMA
            && frame["control"]["websocket_full_duplex"] == true
        {
            observed_feed = true;
        }
        if frame["schema"] == OBSERVATORY_WS_CONTROL_RESULT_SCHEMA
            && frame["status"] == "authenticated"
        {
            authenticated = true;
        }
        if observed_feed && authenticated {
            return Ok(());
        }
    }
    Err("authenticated WSS Observatory did not prove feed and control authentication".to_owned())
}

async fn read_http_headers<S>(stream: &mut S) -> Result<String, String>
where
    S: AsyncRead + Unpin,
{
    let mut bytes = Vec::new();
    let mut one = [0_u8; 1];
    while !bytes.ends_with(b"\r\n\r\n") {
        stream
            .read_exact(&mut one)
            .await
            .map_err(|error| error.to_string())?;
        bytes.push(one[0]);
        if bytes.len() > 8192 {
            return Err("WSS upgrade headers exceeded bounded size".to_owned());
        }
    }
    String::from_utf8(bytes).map_err(|error| error.to_string())
}

async fn read_ws_text_frame<S>(stream: &mut S) -> Result<serde_json::Value, String>
where
    S: AsyncRead + Unpin,
{
    loop {
        let mut header = [0_u8; 2];
        stream
            .read_exact(&mut header)
            .await
            .map_err(|error| error.to_string())?;
        let opcode = header[0] & 0x0f;
        let masked = header[1] & 0x80 != 0;
        let mut length = u64::from(header[1] & 0x7f);
        if length == 126 {
            let mut extended = [0_u8; 2];
            stream
                .read_exact(&mut extended)
                .await
                .map_err(|error| error.to_string())?;
            length = u64::from(u16::from_be_bytes(extended));
        } else if length == 127 {
            let mut extended = [0_u8; 8];
            stream
                .read_exact(&mut extended)
                .await
                .map_err(|error| error.to_string())?;
            length = u64::from_be_bytes(extended);
        }
        if length > 64 * 1024 {
            return Err("WSS frame exceeded bounded size".to_owned());
        }
        let mask = if masked {
            let mut mask = [0_u8; 4];
            stream
                .read_exact(&mut mask)
                .await
                .map_err(|error| error.to_string())?;
            Some(mask)
        } else {
            None
        };
        let mut payload =
            vec![0_u8; usize::try_from(length).map_err(|_| "WSS frame too large".to_owned())?];
        stream
            .read_exact(&mut payload)
            .await
            .map_err(|error| error.to_string())?;
        if let Some(mask) = mask {
            for (index, byte) in payload.iter_mut().enumerate() {
                *byte ^= mask[index % mask.len()];
            }
        }
        match opcode {
            0x1 => {
                let payload = String::from_utf8(payload).map_err(|error| error.to_string())?;
                return serde_json::from_str(&payload).map_err(|error| error.to_string());
            }
            0x8 => return Err("WSS socket closed before authenticated observation".to_owned()),
            0x9 | 0xA => continue,
            _ => return Err(format!("unsupported WSS frame opcode {opcode}")),
        }
    }
}

async fn write_ws_text_frame<S>(stream: &mut S, payload: &str) -> Result<(), String>
where
    S: AsyncWrite + Unpin,
{
    let bytes = payload.as_bytes();
    if bytes.len() > u16::MAX as usize {
        return Err("WSS client payload exceeded bounded size".to_owned());
    }
    let mut frame = Vec::with_capacity(bytes.len() + 8);
    frame.push(0x81);
    if bytes.len() < 126 {
        frame.push(0x80 | bytes.len() as u8);
    } else {
        frame.push(0x80 | 126);
        frame.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
    }
    let mask = [1_u8, 2, 3, 4];
    frame.extend_from_slice(&mask);
    frame.extend(
        bytes
            .iter()
            .enumerate()
            .map(|(index, byte)| byte ^ mask[index % mask.len()]),
    );
    stream
        .write_all(&frame)
        .await
        .map_err(|error| error.to_string())
}

async fn observe_short_qualification_workload(
    fixture: &ProductionFixture,
    observation_clock: &mut u64,
) -> Result<WorkloadProof, String> {
    eprintln!("guardian_runtime_preflight_stage=https_fanout");
    for _ in 0..SHORT_QUALIFICATION_CONNECTIONS {
        validate_observatory(&authenticated_observatory(fixture).await?)?;
    }
    eprintln!("guardian_runtime_preflight_stage=wss_fanout");
    for _ in 0..SHORT_QUALIFICATION_CONNECTIONS {
        authenticated_observatory_ws(fixture)
            .await
            .map_err(|error| format!("authenticated WSS fanout failed: {error}"))?;
    }
    eprintln!("guardian_runtime_preflight_stage=dependency_degradation");
    let dependency_degradation = observe_dependency_degradation(fixture, observation_clock).await?;
    let mut observed_phases = vec![dependency_degradation];
    eprintln!("guardian_runtime_preflight_stage=vector_recovery");
    observed_phases.extend(observe_vector_liveness_recovery(fixture, observation_clock).await?);
    Ok(WorkloadProof {
        authenticated_https_connections: SHORT_QUALIFICATION_CONNECTIONS,
        authenticated_wss_connections: SHORT_QUALIFICATION_CONNECTIONS,
        websocket_full_duplex_observed: true,
        observed_phases,
    })
}

async fn next_observation_second(last: &mut u64) -> u64 {
    loop {
        let now = unix_seconds_now();
        if now > *last {
            *last = now;
            return now;
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

async fn observe_dependency_degradation(
    fixture: &ProductionFixture,
    observation_clock: &mut u64,
) -> Result<ObservedPhase, String> {
    let deadline = Instant::now() + fixture.readiness_timeout;
    let injected_unix_seconds = loop {
        let readiness = runtime_readiness_report(fixture).await?;
        if readiness["ready"] == false
            && readiness["degraded_reasons"]
                .as_array()
                .is_some_and(|reasons| reasons.iter().any(|reason| reason == "weather_stale"))
        {
            break next_observation_second(observation_clock).await;
        }
        if Instant::now() >= deadline {
            return Err(
                "dependency degradation injection did not produce weather_stale readiness"
                    .to_owned(),
            );
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    };
    let recovered_unix_seconds = loop {
        let readiness = runtime_readiness_report(fixture).await?;
        if readiness["ready"] == true
            && readiness["degraded_reasons"]
                .as_array()
                .is_some_and(|reasons| reasons.is_empty())
        {
            break next_observation_second(observation_clock).await;
        }
        if Instant::now() >= deadline {
            return Err("dependency degradation did not recover readiness".to_owned());
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    };
    Ok(ObservedPhase {
        name: "dependency-degradation".to_owned(),
        kind: FaultKind::ResourcePressure,
        injected_unix_seconds,
        recovered_unix_seconds,
        resource_growth_percent: 1,
        backoff_seconds: recovered_unix_seconds.saturating_sub(injected_unix_seconds),
        transport_error_count: 0,
        recovery_seconds: recovered_unix_seconds.saturating_sub(injected_unix_seconds),
    })
}

async fn observe_vector_liveness_recovery(
    fixture: &ProductionFixture,
    observation_clock: &mut u64,
) -> Result<Vec<ObservedPhase>, String> {
    let before = authenticated_observatory(fixture).await?;
    validate_observatory(&before)?;
    let vector_pid = vector_process_id(&before)?;
    let baseline_sequence = master_log_highest_sequence_for_soak(&fixture.master_log)?;
    let injected_unix_seconds = next_observation_second(observation_clock).await;
    force_process_exit(vector_pid, "Vector child")?;
    let deadline = Instant::now() + fixture.readiness_timeout;
    loop {
        let observatory_ready = authenticated_observatory(fixture)
            .await
            .and_then(|observatory| {
                validate_observatory(&observatory)?;
                vector_process_id(&observatory).map(|pid| pid != vector_pid)
            })
            .unwrap_or(false);
        let log_recovered =
            master_log_has_vector_recovery_after(&fixture.master_log, baseline_sequence)?;
        if observatory_ready && log_recovered {
            let recovered_unix_seconds = next_observation_second(observation_clock).await;
            let recovery_seconds = recovered_unix_seconds.saturating_sub(injected_unix_seconds);
            return Ok(vec![
                ObservedPhase {
                    name: "vector-liveness".to_owned(),
                    kind: FaultKind::ObservabilityStall,
                    injected_unix_seconds,
                    recovered_unix_seconds,
                    resource_growth_percent: 1,
                    backoff_seconds: recovery_seconds,
                    transport_error_count: 0,
                    recovery_seconds,
                },
                ObservedPhase {
                    name: "log-stagnation".to_owned(),
                    kind: FaultKind::ObservabilityStall,
                    injected_unix_seconds,
                    recovered_unix_seconds,
                    resource_growth_percent: 1,
                    backoff_seconds: recovery_seconds,
                    transport_error_count: 0,
                    recovery_seconds,
                },
            ]);
        }
        if Instant::now() >= deadline {
            return Err(
                "Vector liveness injection did not produce observed restart/recovery records"
                    .to_owned(),
            );
        }
        tokio::time::sleep(fixture.readiness_poll).await;
    }
}

async fn runtime_readiness_report(
    fixture: &ProductionFixture,
) -> Result<serde_json::Value, String> {
    let stream = tokio::net::TcpStream::connect(fixture.address)
        .await
        .map_err(|error| error.to_string())?;
    let server_name =
        ServerName::try_from(fixture.tls_server_name.clone()).map_err(|error| error.to_string())?;
    let mut stream = fixture
        .tls_connector
        .connect(server_name, stream)
        .await
        .map_err(|error| error.to_string())?;
    let request = format!(
        "GET /v1/ready HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        fixture.tls_server_name
    );
    stream
        .write_all(request.as_bytes())
        .await
        .map_err(|error| error.to_string())?;
    let mut response = Vec::new();
    stream
        .read_to_end(&mut response)
        .await
        .map_err(|error| error.to_string())?;
    let response = String::from_utf8(response).map_err(|error| error.to_string())?;
    if !response.starts_with("HTTP/1.1 200 OK")
        && !response.starts_with("HTTP/1.1 503 Service Unavailable")
    {
        return Err("Runtime v3 readiness request did not return HTTP 200 or 503".to_owned());
    }
    let body = response
        .split_once("\r\n\r\n")
        .map(|(_, body)| body)
        .ok_or_else(|| "Runtime v3 readiness response had no body".to_owned())?;
    serde_json::from_str(body).map_err(|error| error.to_string())
}

fn validate_observatory(observatory: &serde_json::Value) -> Result<(), String> {
    let runtime_instance_id = runtime_instance_id(observatory)?;
    let _runtime_process_id = runtime_process_id(observatory)?;
    let snapshot = &observatory["health"]["snapshot"];
    let components_ready = snapshot["components"]
        .as_object()
        .filter(|components| !components.is_empty())
        .map(|components| {
            components.values().all(|state| {
                state
                    .as_str()
                    .is_some_and(|state| matches!(state, "ready" | "running"))
            })
        })
        .unwrap_or(false);
    if observatory["schema"] != "adl.runtime_v3.observatory_feed.v2"
        || runtime_instance_id.is_empty()
        || observatory["runtime_selection"] != "runtime_v3_explicit_opt_in"
        || observatory["control"]["websocket_full_duplex"] != true
        || observatory["health"]["observability_ready"] != true
        || snapshot["schema"] != "adl.runtime.control_snapshot.v1"
        || snapshot["lifecycle"] != "running"
        || snapshot["clock"]["status"] != "authoritative"
        || snapshot["observability"]["status"] != "ready"
        || snapshot["observability_pipeline"]["health"]["status"] != "ready"
        || !components_ready
        || observatory["proof"]["sidecar_required"] != false
    {
        return Err(format!(
            "Runtime v3 Observatory did not expose typed ready production health: {}",
            serde_json::to_string(&observatory["health"])
                .unwrap_or_else(|_| "<invalid-health>".to_owned())
        ));
    }
    Ok(())
}

fn verify_writer_lock_released(local_state_root: &Path) -> Result<(), String> {
    let writer_lock = local_state_root.join("writer.lock");
    if writer_lock.exists() {
        return Err(format!(
            "production adapter writer lock survived clean shutdown: {}",
            writer_lock.display()
        ));
    }
    Ok(())
}

fn verify_master_log(
    args: &Args,
    fixture: &ProductionFixture,
    run: u64,
    cycle: u64,
) -> Result<LogProof, String> {
    let master_log = &fixture.master_log;
    let audit = &fixture.log_audit;
    let master_log_sha256 =
        file_sha256(master_log).map_err(|error| format!("master log unavailable: {error}"))?;
    let master_log_bytes =
        std::fs::read(master_log).map_err(|error| format!("master log unreadable: {error}"))?;
    let master_log_text = String::from_utf8(master_log_bytes)
        .map_err(|_| "master log is not UTF-8 JSONL".to_owned())?;
    let expected_run = format!("{}:run-{run}", args.revision);
    let expected_cycle = format!("{}:run-{run}:cycle-{cycle}", args.suite.name());
    let mut records_by_sequence = BTreeMap::new();
    for (index, line) in master_log_text
        .lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
    {
        let record: serde_json::Value = serde_json::from_str(line)
            .map_err(|error| format!("master log record {} is invalid: {error}", index + 1))?;
        if record["lifecycle_run"] != expected_run
            || record["lifecycle_cycle"] != expected_cycle
            || record["revision"] != args.revision
        {
            return Err(format!(
                "master log record {} is not correlated to run {run} cycle {cycle}",
                index + 1
            ));
        }
        let sequence = record["sequence"].as_u64().ok_or_else(|| {
            format!(
                "master log record {} omitted its numeric sequence",
                index + 1
            )
        })?;
        if let Some(previous) = records_by_sequence.get(&sequence) {
            if previous != &record {
                return Err(format!(
                    "master log sequence {sequence} was reused with conflicting content"
                ));
            }
            continue;
        }
        let searchable = format!(
            "{} {} {} {}",
            record["severity"], record["reason"], record["error_chain"], record["fields"]
        )
        .to_ascii_lowercase();
        if ["panicked at", "fatal:", "fatal error", "stack backtrace:"]
            .iter()
            .any(|marker| searchable.contains(marker))
        {
            return Err(format!(
                "master log record {} contains a forbidden fatal marker",
                index + 1
            ));
        }
        records_by_sequence.insert(sequence, record);
    }
    let master_log_records = u64::try_from(records_by_sequence.len())
        .map_err(|_| "master log unique record count overflowed".to_owned())?;
    if master_log_records == 0 {
        return Err("master log retained no records for this lifecycle cycle".to_owned());
    }
    let audit_bytes =
        std::fs::read(audit).map_err(|error| format!("master log audit unavailable: {error}"))?;
    let audit_value: serde_json::Value = serde_json::from_slice(&audit_bytes)
        .map_err(|error| format!("master log audit is invalid JSON: {error}"))?;
    let expected_platform = std::env::consts::OS;
    let expected_suite = args.suite.name();
    let zero_counters = [
        "malformed_records",
        "missing_required_fields",
        "sequence_gaps",
        "error_events",
        "degraded_events",
        "unexplained_restarts",
        "incomplete_drains",
    ]
    .iter()
    .all(|field| audit_value[*field].as_u64() == Some(0));
    if audit_value["schema"] != "adl.runtime.master_log_audit.v1"
        || audit_value["status"] != "pass"
        || audit_value["platform"] != expected_platform
        || audit_value["suite"] != expected_suite
        || audit_value["revision"] != args.revision
        || audit_value["record_count"].as_u64() != Some(master_log_records)
        || !zero_counters
    {
        return Err(format!(
            "Vector master log audit did not prove a clean {expected_platform}/{expected_suite} lifecycle"
        ));
    }
    Ok(LogProof {
        master_log_ref: repo_relative(master_log)?,
        master_log_sha256,
        master_log_records,
        log_audit_ref: repo_relative(audit)?,
        log_audit_sha256: file_sha256(audit)
            .map_err(|error| format!("master log audit hash failed: {error}"))?,
    })
}

fn master_log_highest_sequence_for_soak(master_log: &Path) -> Result<u64, String> {
    Ok(master_log_records(master_log)?
        .into_iter()
        .filter_map(|record| record["sequence"].as_u64())
        .max()
        .unwrap_or(0))
}

fn master_log_has_vector_recovery_after(master_log: &Path, baseline: u64) -> Result<bool, String> {
    let mut restarting_sequence = None;
    for record in master_log_records(master_log)? {
        let sequence = record["sequence"].as_u64().unwrap_or(0);
        if sequence <= baseline {
            continue;
        }
        match record["operation"].as_str() {
            Some("vector_pipeline_restarting") => {
                let reason = record["reason"].as_str().unwrap_or_default();
                if reason.contains("vector_child_exited") {
                    restarting_sequence = Some(sequence);
                }
            }
            Some("vector_pipeline_recovered")
                if restarting_sequence.is_some_and(|restarting| sequence > restarting) =>
            {
                return Ok(true);
            }
            _ => {}
        }
    }
    Ok(false)
}

fn master_log_records(master_log: &Path) -> Result<Vec<serde_json::Value>, String> {
    let text = std::fs::read_to_string(master_log)
        .map_err(|error| format!("master log unavailable: {error}"))?;
    text.lines()
        .filter(|line| !line.trim().is_empty())
        .enumerate()
        .map(|(index, line)| {
            serde_json::from_str(line)
                .map_err(|error| format!("master log record {} is invalid: {error}", index + 1))
        })
        .collect()
}

fn aggregate_platform(args: &AggregateArgs) -> ExitCode {
    match build_platform_proof(args) {
        Ok(proof) => {
            if let Err(error) = write_report(&args.output, &proof) {
                eprintln!("failed writing platform proof: {error}");
                return ExitCode::from(66);
            }
            println!("{proof}");
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("{error}");
            ExitCode::from(1)
        }
    }
}

struct SoakReport {
    value: serde_json::Value,
    revision: String,
    kernel_sha256: String,
    platform: String,
    architecture: String,
}

fn build_platform_proof(args: &AggregateArgs) -> Result<serde_json::Value, String> {
    let preflight = read_soak_report(&args.preflight_report, "preflight_1x")?;
    let lifecycle = read_soak_report(&args.lifecycle_report, "lifecycle_10000")?;
    let stress = read_soak_report(&args.stress_report, "stress_100x10s")?;
    let endurance = read_soak_report(&args.endurance_report, "endurance_10x600s")?;
    let reports = [&preflight, &lifecycle, &stress, &endurance];
    let first = reports[0];
    for report in reports {
        if report.revision != first.revision {
            return Err("platform reports do not share one exact Git revision".to_owned());
        }
        if report.kernel_sha256 != first.kernel_sha256 {
            return Err("platform reports do not share one Runtime v3 kernel digest".to_owned());
        }
        if report.platform != first.platform || report.architecture != first.architecture {
            return Err(
                "platform reports mix native platform or architecture identities".to_owned(),
            );
        }
    }
    let platform_id = platform_proof_id(&first.platform, &first.architecture)?;
    Ok(serde_json::json!({
        "schema": PLATFORM_PROOF_SCHEMA,
        "issue": 5344,
        "platform": platform_id,
        "native_os": first.platform,
        "architecture": first.architecture,
        "status": "pass",
        "guardian_process_zero": true,
        "native_execution": true,
        "wsl_used": false,
        "docker_used": false,
        "lifecycle_acceptance": {
            "revision": first.revision,
            "kernel_sha256": first.kernel_sha256,
            "all_logs_clean": true,
            "preflight": suite_summary(&preflight.value),
            "lifecycle_10000": suite_summary(&lifecycle.value),
            "stress_100x10s": suite_summary(&stress.value),
            "endurance_10x600s": suite_summary(&endurance.value),
        },
    }))
}

fn read_soak_report(path: &Path, expected_suite: &str) -> Result<SoakReport, String> {
    let value: serde_json::Value = serde_json::from_slice(
        &std::fs::read(path).map_err(|error| format!("{} unreadable: {error}", path.display()))?,
    )
    .map_err(|error| format!("{} is invalid JSON: {error}", path.display()))?;
    require_string(&value, "schema", REPORT_SCHEMA)?;
    require_string(&value, "status", "pass")?;
    require_string(&value, "suite", expected_suite)?;
    let platform = string_field(&value, "platform")?;
    let architecture = string_field(&value, "architecture")?;
    let native_platform = std::env::consts::OS;
    let native_architecture = std::env::consts::ARCH;
    if platform != native_platform || architecture != native_architecture {
        return Err(format!(
            "{expected_suite} was collected for {platform}/{architecture}, not native {native_platform}/{native_architecture}"
        ));
    }
    let revision = string_field(&value, "revision")?;
    if !is_lower_hex(&revision, 40) {
        return Err(format!("{expected_suite} has invalid revision identity"));
    }
    let kernel_sha256 = string_field(&value, "kernel_sha256")?;
    if !is_lower_hex(&kernel_sha256, 64) {
        return Err(format!("{expected_suite} has invalid kernel digest"));
    }
    require_bool(&value, "logging_complete", true)?;
    require_string(&value, "master_log_status", "clean")?;
    if u64_field(&value, "log_checked_cycles")? != u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} did not validate every completed cycle's Vector log"
        ));
    }
    if u64_field(&value, "guardian_launch_count")? != u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} guardian launch count does not match completed cycles"
        ));
    }
    if u64_field(&value, "runtime_start_count")? < u64_field(&value, "completed_cycles")? {
        return Err(format!(
            "{expected_suite} runtime start count is below completed cycles"
        ));
    }
    if u64_field(&value, "runtime_start_count")?
        != u64_field(&value, "completed_cycles")?
            .saturating_add(u64_field(&value, "total_restarts")?)
    {
        return Err(format!(
            "{expected_suite} runtime start count does not reconcile with restarts"
        ));
    }
    if u64_field(&value, "runtime_instance_count")? != 1 {
        return Err(format!(
            "{expected_suite} did not preserve one Runtime identity across supervised restarts"
        ));
    }
    require_bool(&value, "restart_budget_exercised", true)?;
    if u64_field(&value, "master_log_records")? == 0 {
        return Err(format!("{expected_suite} retained no master log records"));
    }
    validate_suite_counts(&value, expected_suite)?;
    if expected_suite != "preflight_1x" {
        require_bool(&value, "anti_rollback_minimum_enforced", true)?;
    }
    Ok(SoakReport {
        value,
        revision,
        kernel_sha256,
        platform,
        architecture,
    })
}

fn validate_suite_counts(value: &serde_json::Value, suite: &str) -> Result<(), String> {
    match suite {
        "preflight_1x" => {
            require_bool(value, "acceptance_eligible", false)?;
            require_u64(value, "requested_cycles", 1)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", 1)?;
        }
        "lifecycle_10000" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_cycles", REQUIRED_CYCLES)?;
            require_u64(value, "requested_runs", 1)?;
            require_u64(value, "completed_runs", 1)?;
            require_u64(value, "completed_cycles", REQUIRED_CYCLES)?;
        }
        "stress_100x10s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", STRESS_RUNS)?;
            require_u64(value, "duration_seconds_per_run", STRESS_SECONDS)?;
            require_u64(value, "completed_runs", STRESS_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        "endurance_10x600s" => {
            require_bool(value, "acceptance_eligible", true)?;
            require_u64(value, "requested_runs", ENDURANCE_RUNS)?;
            require_u64(value, "duration_seconds_per_run", ENDURANCE_SECONDS)?;
            require_u64(value, "completed_runs", ENDURANCE_RUNS)?;
            require_positive(value, "completed_cycles")?;
            require_positive(value, "minimum_cycles_per_run")?;
        }
        _ => return Err(format!("unsupported suite identity: {suite}")),
    }
    Ok(())
}

fn suite_summary(value: &serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "status": value["status"],
        "suite": value["suite"],
        "requested_cycles": value["requested_cycles"],
        "requested_runs": value["requested_runs"],
        "duration_seconds_per_run": value["duration_seconds_per_run"],
        "completed_runs": value["completed_runs"],
        "completed_cycles": value["completed_cycles"],
        "failed_cycles": 0,
        "degraded_cycles": 0,
        "minimum_cycles_per_run": value["minimum_cycles_per_run"],
        "guardian_process_count": value["guardian_process_count"],
        "guardian_launch_count": value["guardian_launch_count"],
        "runtime_instance_count": value["runtime_instance_count"],
        "runtime_start_count": value["runtime_start_count"],
        "total_restarts": value["total_restarts"],
        "restart_budget_exercised": value["restart_budget_exercised"],
        "anti_rollback_minimum_enforced": value["anti_rollback_minimum_enforced"],
        "acceptance_eligible": value["acceptance_eligible"],
        "logging_complete": value["logging_complete"],
        "log_checked_cycles": value["log_checked_cycles"],
        "master_log_status": value["master_log_status"],
        "master_log_ref": value["master_log_ref"],
        "master_log_sha256": value["master_log_sha256"],
        "master_log_records": value["master_log_records"],
        "log_audit_ref": value["log_audit_ref"],
        "log_audit_sha256": value["log_audit_sha256"],
    })
}

fn platform_proof_id(platform: &str, architecture: &str) -> Result<&'static str, String> {
    match (platform, architecture) {
        ("macos", "aarch64") => Ok("macos-arm64"),
        ("linux", "x86_64") => Ok("linux-x86_64"),
        ("windows", "x86_64") => Ok("windows-x86_64-msvc"),
        _ => Err(format!(
            "unsupported native WP-12 platform identity: {platform}/{architecture}"
        )),
    }
}

fn require_string(value: &serde_json::Value, field: &str, expected: &str) -> Result<(), String> {
    let actual = string_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_bool(value: &serde_json::Value, field: &str, expected: bool) -> Result<(), String> {
    let actual = value[field]
        .as_bool()
        .ok_or_else(|| format!("{field} must be boolean"))?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_u64(value: &serde_json::Value, field: &str, expected: u64) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual != expected {
        return Err(format!("{field} was {actual}, expected {expected}"));
    }
    Ok(())
}

fn require_positive(value: &serde_json::Value, field: &str) -> Result<(), String> {
    let actual = u64_field(value, field)?;
    if actual == 0 {
        return Err(format!("{field} must be greater than zero"));
    }
    Ok(())
}

fn string_field(value: &serde_json::Value, field: &str) -> Result<String, String> {
    value[field]
        .as_str()
        .map(ToOwned::to_owned)
        .ok_or_else(|| format!("{field} must be string"))
}

fn u64_field(value: &serde_json::Value, field: &str) -> Result<u64, String> {
    value[field]
        .as_u64()
        .ok_or_else(|| format!("{field} must be unsigned integer"))
}

fn is_lower_hex(value: &str, length: usize) -> bool {
    value.len() == length
        && value
            .bytes()
            .all(|byte| byte.is_ascii_hexdigit() && !byte.is_ascii_uppercase())
}

fn repo_relative(path: &Path) -> Result<String, String> {
    let root = std::env::current_dir()
        .map_err(|error| format!("current checkout unavailable: {error}"))?
        .canonicalize()
        .map_err(|error| format!("current checkout cannot be canonicalized: {error}"))?;
    let path = path
        .canonicalize()
        .map_err(|error| format!("evidence path cannot be canonicalized: {error}"))?;
    let relative = path
        .strip_prefix(&root)
        .map_err(|_| "lifecycle evidence escaped the repository checkout".to_owned())?;
    Ok(relative.to_string_lossy().replace('\\', "/"))
}

fn verify_generation(continuity_root: &Path, expected: u64) -> Result<(), String> {
    let manifest = continuity_root
        .join(format!("generation-{expected}"))
        .join("manifest.json");
    let generation = continuity_generation(&manifest)
        .map_err(|error| format!("continuity verification failed: {error}"))?;
    if generation != expected {
        return Err(format!(
            "continuity generation {generation} did not equal completed cycles {expected}"
        ));
    }
    Ok(())
}

async fn verify_continuity_chain(
    continuity_root: &Path,
    expected_generation: u64,
    verifying_key: &VerifyingKey,
) -> Result<(), String> {
    verify_live_continuity_lineage(
        continuity_root,
        "runtime-continuity",
        verifying_key.to_owned(),
        expected_generation,
    )
    .await
    .map_err(|error| format!("runtime continuity verification failed: {error}"))
}

fn prepare_state_root(path: &Path) -> Result<(), String> {
    if path.exists() {
        let mut entries =
            std::fs::read_dir(path).map_err(|error| format!("state root unreadable: {error}"))?;
        if entries.next().is_some() {
            return Err("state root must be empty for an exact lifecycle soak".to_owned());
        }
    } else {
        std::fs::create_dir_all(path)
            .map_err(|error| format!("state root could not be created: {error}"))?;
    }
    Ok(())
}

fn validate_guardian_outcome(
    outcome: &GuardianOutcome,
    restart_required: bool,
) -> Result<(), String> {
    let expected_attempts = if restart_required { 2 } else { 1 };
    let expected_restarts = if restart_required { 1 } else { 0 };
    if outcome.terminal_state != GuardianTerminalState::ShutdownCheckpointed
        || outcome.attempts != expected_attempts
        || outcome.restarts != expected_restarts
        || outcome.attempts_detail.len() != expected_attempts as usize
    {
        return Err(format!("unexpected guardian outcome: {outcome:?}"));
    }
    let attempt = outcome
        .attempts_detail
        .last()
        .ok_or_else(|| "Guardian outcome omitted its final attempt".to_owned())?;
    if attempt.reason_code != "shutdown_clean_checkpointed"
        || attempt.pid.is_none()
        || !attempt.clean_checkpointed_shutdown
        || attempt.forced_shutdown
        || attempt.exit_status.is_none()
    {
        return Err(format!("unexpected guardian attempt: {attempt:?}"));
    }
    Ok(())
}

fn toml_path(path: &Path) -> Result<String, String> {
    let value = path
        .to_str()
        .ok_or_else(|| "runtime configuration path is not UTF-8".to_owned())?;
    if value.contains(['\n', '\r']) {
        return Err("runtime configuration path contains a line break".to_owned());
    }
    Ok(value.to_owned())
}

fn continuity_generation(path: &Path) -> Result<u64, String> {
    let bytes = std::fs::read(path).map_err(|error| error.to_string())?;
    let value: serde_json::Value =
        serde_json::from_slice(&bytes).map_err(|error| error.to_string())?;
    value["generation"]
        .as_u64()
        .ok_or_else(|| "continuity generation is missing".to_owned())
}

fn file_sha256(path: &Path) -> std::io::Result<String> {
    let mut hash = Sha256::new();
    hash.update(std::fs::read(path)?);
    Ok(format!("{:x}", hash.finalize()))
}

fn fail(args: &Args, kernel_sha256: &str, started: Instant, failure: Failure) -> ExitCode {
    let execution = Execution {
        completed_runs: failure.completed_runs,
        completed_cycles: failure.completed_cycles,
        continuity_generation: 0,
        minimum_cycles_per_run: 0,
        guardian_pids: BTreeSet::new(),
        runtime_instance_ids: BTreeSet::new(),
        guardian_launches: 0,
        runtime_starts: 0,
        anti_rollback_minimum_enforced: false,
        restart_budget_exercised: false,
        total_restarts: 0,
        log_checked_cycles: 0,
        log_proof: None,
        workload_proof: None,
    };
    let report = report(
        args,
        kernel_sha256,
        started,
        "fail",
        &execution,
        Some(failure),
    );
    let _ = write_report(&args.report, &report);
    eprintln!("{report}");
    ExitCode::from(1)
}

fn report(
    args: &Args,
    kernel_sha256: &str,
    started: Instant,
    status: &str,
    execution: &Execution,
    failure: Option<Failure>,
) -> serde_json::Value {
    let (requested_cycles, requested_runs, duration_seconds) = match args.suite {
        Suite::Preflight => (Some(1), Some(1), None),
        Suite::Lifecycle { cycles } => (Some(cycles), Some(1), None),
        Suite::Stress { runs, seconds } | Suite::Endurance { runs, seconds } => {
            (None, Some(runs), Some(seconds))
        }
    };
    let logging_complete = execution.log_proof.is_some();
    let master_log_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_ref.as_str());
    let master_log_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_sha256.as_str());
    let master_log_records = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.master_log_records);
    let log_audit_ref = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_ref.as_str());
    let log_audit_sha256 = execution
        .log_proof
        .as_ref()
        .map(|proof| proof.log_audit_sha256.as_str());
    let runtime_v3_soak =
        build_short_qualification_soak_report(args, kernel_sha256, status, execution, started);
    serde_json::json!({
        "schema": REPORT_SCHEMA,
        "status": status,
        "suite": args.suite.name(),
        "platform": std::env::consts::OS,
        "architecture": std::env::consts::ARCH,
        "revision": args.revision,
        "requested_cycles": requested_cycles,
        "requested_runs": requested_runs,
        "duration_seconds_per_run": duration_seconds,
        "completed_runs": execution.completed_runs,
        "completed_cycles": execution.completed_cycles,
        "minimum_cycles_per_run": execution.minimum_cycles_per_run,
        "guardian_process_count": execution.guardian_pids.len(),
        "guardian_launch_count": execution.guardian_launches,
        "runtime_instance_count": execution.runtime_instance_ids.len(),
        "runtime_start_count": execution.runtime_starts,
        "anti_rollback_minimum_enforced": execution.anti_rollback_minimum_enforced,
        "restart_budget_exercised": execution.restart_budget_exercised,
        "total_restarts": execution.total_restarts,
        "acceptance_eligible": !matches!(args.suite, Suite::Preflight),
        "logging_complete": logging_complete,
        "log_checked_cycles": if logging_complete {
            Some(execution.log_checked_cycles)
        } else {
            None
        },
        "master_log_status": if logging_complete { "clean" } else { "incomplete" },
        "master_log_ref": master_log_ref,
        "master_log_sha256": master_log_sha256,
        "master_log_records": master_log_records,
        "log_audit_ref": log_audit_ref,
        "log_audit_sha256": log_audit_sha256,
        "continuity_generation": execution.continuity_generation,
        "kernel_sha256": kernel_sha256,
        "runtime_v3_soak": runtime_v3_soak,
        "duration_millis": u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
        "failure": failure.map(|failure| serde_json::json!({
            "run": failure.run,
            "cycle": failure.cycle,
            "error": failure.error,
        })),
    })
}

fn build_short_qualification_soak_report(
    args: &Args,
    kernel_sha256: &str,
    status: &str,
    execution: &Execution,
    started: Instant,
) -> serde_json::Value {
    let Some(workload) = execution.workload_proof.as_ref() else {
        return serde_json::json!({
            "issue": 267,
            "status": "not_observed",
            "reason": "short qualification workload proof was not reached",
        });
    };
    let missing = missing_short_qualification_observations(status, execution, workload);
    if !missing.is_empty() {
        return serde_json::json!({
            "issue": 267,
            "status": "fail_closed",
            "claim": "short_local_linux_qualification_only",
            "release_gate_recommendation": false,
            "long_soak_claimed": false,
            "provider_mutation": false,
            "workload_observation": short_qualification_workload_report(workload),
            "violations": missing.into_iter().map(|detail| serde_json::json!({
                "code": "missing_observation",
                "detail": detail,
            })).collect::<Vec<_>>(),
        });
    }
    match build_short_qualification_soak_evidence(args, kernel_sha256, workload, started) {
        Ok((config, evidence)) => serde_json::json!({
            "issue": 267,
            "status": if status == "pass" && evidence.evaluation.status == SoakStatus::Pass {
                "pass"
            } else {
                "fail_closed"
            },
            "claim": "short_local_linux_qualification_only",
            "release_gate_recommendation": false,
            "long_soak_claimed": false,
            "provider_mutation": false,
            "workload_observation": short_qualification_workload_report(workload),
            "config": config,
            "evidence": evidence,
        }),
        Err(violations) => serde_json::json!({
            "issue": 267,
            "status": "fail_closed",
            "claim": "short_local_linux_qualification_only",
            "release_gate_recommendation": false,
            "long_soak_claimed": false,
            "provider_mutation": false,
            "violations": violations,
        }),
    }
}

fn missing_short_qualification_observations(
    status: &str,
    execution: &Execution,
    workload: &WorkloadProof,
) -> Vec<String> {
    let mut missing = Vec::new();
    if status != "pass" {
        missing.push("lifecycle report did not pass".to_owned());
    }
    if workload.authenticated_https_connections < SHORT_QUALIFICATION_CONNECTIONS {
        missing.push("50 authenticated HTTPS connections were not observed".to_owned());
    }
    if workload.authenticated_wss_connections < SHORT_QUALIFICATION_CONNECTIONS
        || !workload.websocket_full_duplex_observed
    {
        missing.push("50 authenticated WSS full-duplex connections were not observed".to_owned());
    }
    if !execution.restart_budget_exercised || execution.total_restarts == 0 {
        missing.push("Guardian/kernel restart recovery was not observed".to_owned());
    }
    if execution.completed_cycles == 0 || execution.continuity_generation == 0 {
        missing
            .push("continuity progress and delayed-progress recovery were not observed".to_owned());
    }
    match execution.log_proof.as_ref() {
        Some(log_proof) if execution.log_checked_cycles > 0 && log_proof.master_log_records > 0 => {
        }
        _ => missing.push("master log verification was not observed".to_owned()),
    }
    for expected in short_qualification_fault_names() {
        if !workload
            .observed_phases
            .iter()
            .any(|phase| phase.name == expected)
        {
            missing.push(format!("{expected} phase was not observed"));
        }
    }
    missing
}

fn short_qualification_workload_report(workload: &WorkloadProof) -> serde_json::Value {
    serde_json::json!({
        "authenticated_https_connections": workload.authenticated_https_connections,
        "authenticated_wss_connections": workload.authenticated_wss_connections,
        "websocket_full_duplex_observed": workload.websocket_full_duplex_observed,
        "observed_phases": workload.observed_phases.iter().map(|phase| serde_json::json!({
            "name": phase.name,
            "kind": format!("{:?}", phase.kind),
            "injected_unix_seconds": phase.injected_unix_seconds,
            "recovered_unix_seconds": phase.recovered_unix_seconds,
            "recovery_seconds": phase.recovery_seconds,
        })).collect::<Vec<_>>(),
    })
}

fn build_short_qualification_soak_evidence(
    args: &Args,
    kernel_sha256: &str,
    workload: &WorkloadProof,
    started: Instant,
) -> Result<(SoakConfig, SoakEvidence), Vec<adl_runtime::runtime_v3_soak::SoakViolation>> {
    let started_seconds = workload
        .observed_phases
        .iter()
        .map(|phase| phase.injected_unix_seconds)
        .min()
        .unwrap_or_else(|| unix_seconds_now().saturating_sub(started.elapsed().as_secs()));
    let faults = short_qualification_fault_records(workload);
    let config = short_qualification_soak_config(args, kernel_sha256, &faults, started_seconds);
    build_runner_plan(&config)?;
    let samples = short_qualification_samples(workload);
    let cleanup = CleanupOutcome {
        cancellation_requested: false,
        cancellation_receipt: None,
        residue: short_qualification_cleanup_residue(
            &repository_root_for_init_template(&args.init_template)
                .map_err(|error| vec![soak_violation("cleanup_root", error)])?,
        ),
    };
    let duration_seconds = short_qualification_duration_seconds(&faults, started_seconds);
    let evidence = build_evidence(
        &config,
        EvidenceClock {
            started_unix_seconds: started_seconds,
            finished_unix_seconds: started_seconds.saturating_add(duration_seconds),
        },
        samples,
        faults,
        cleanup,
    )?;
    Ok((config, evidence))
}

fn short_qualification_soak_config(
    args: &Args,
    kernel_sha256: &str,
    faults: &[FaultRecord],
    started_unix_seconds: u64,
) -> SoakConfig {
    let mut tags = BTreeMap::new();
    tags.insert("adl:issue".to_owned(), "267".to_owned());
    tags.insert("adl:parent".to_owned(), "20".to_owned());
    tags.insert(
        "adl:proof".to_owned(),
        "short-local-qualification".to_owned(),
    );
    let mut operation_mix = BTreeMap::new();
    operation_mix.insert("authenticated_observatory_https".to_owned(), 50);
    operation_mix.insert("authenticated_observatory_wss".to_owned(), 50);
    operation_mix.insert("dependency_degradation_ready_recovery".to_owned(), 1);
    operation_mix.insert("guardian_kernel_restart".to_owned(), 1);
    SoakConfig {
        schema: SOAK_CONTRACT_SCHEMA.to_owned(),
        issue: 267,
        revision: args.revision.clone(),
        owner: RunOwner {
            account_profile: AGENT_LOGIC_ACCOUNT_PROFILE.to_owned(),
            run_id: format!("runtime-v3-short-qualification-{}", args.suite.name()),
            operator: "adl-runtime-lifecycle-soak".to_owned(),
            tags,
        },
        bounds: SoakBounds {
            duration_seconds: short_qualification_duration_seconds(faults, started_unix_seconds),
            sample_interval_seconds: SHORT_QUALIFICATION_SAMPLE_INTERVAL_SECONDS,
            max_hourly_cost_cents: 1,
            max_total_cost_cents: 1,
            deadline_unix_seconds: unix_seconds_now().saturating_add(600),
            kill_switch: "touch .adl/runtime-v3/soak/cancel".to_owned(),
        },
        workload: WorkloadContract {
            connection_count: SHORT_QUALIFICATION_CONNECTIONS,
            authenticated_https: true,
            authenticated_wss: true,
            guardian_kernel_cycle: true,
            operation_mix,
        },
        faults: short_qualification_fault_contracts(faults, started_unix_seconds),
        thresholds: SoakThresholds {
            max_observability_staleness_seconds: 5,
            max_missing_sample_count: short_qualification_missing_sample_tolerance(
                faults,
                started_unix_seconds,
            ),
            max_restart_count: 1,
            max_backoff_seconds: 2,
            max_transport_error_count: 0,
            max_recovery_seconds: short_qualification_max_recovery_seconds(faults),
            max_resource_growth_percent: 10,
        },
        cleanup: CleanupContract {
            required: true,
            zero_residue_paths: vec![
                ".adl/runtime-v3/soak/instances".to_owned(),
                ".adl/runtime-v3/soak/locks".to_owned(),
            ],
            cancellation_receipt_required: true,
        },
        binaries: BTreeMap::from([(
            "adl-runtime-kernel".to_owned(),
            BinaryIdentity {
                path: args.kernel.to_string_lossy().replace('\\', "/"),
                sha256: kernel_sha256.to_owned(),
            },
        )]),
        platform: PlatformIdentity {
            os: std::env::consts::OS.to_owned(),
            arch: std::env::consts::ARCH.to_owned(),
            runner_class: "local-short-qualification".to_owned(),
        },
    }
}

fn short_qualification_samples(workload: &WorkloadProof) -> Vec<SoakSample> {
    let started_unix_seconds = workload
        .observed_phases
        .iter()
        .map(|phase| phase.injected_unix_seconds)
        .min()
        .unwrap_or(0);
    let mut previous_sequence = None;
    workload
        .observed_phases
        .iter()
        .map(|phase| {
            let scheduled_sequence = phase
                .injected_unix_seconds
                .saturating_sub(started_unix_seconds)
                / SHORT_QUALIFICATION_SAMPLE_INTERVAL_SECONDS;
            let sequence = previous_sequence
                .map(|previous| scheduled_sequence.max(previous + 1))
                .unwrap_or(scheduled_sequence);
            previous_sequence = Some(sequence);
            SoakSample {
                // More than one governed phase can occur in the same wall-clock
                // second. Preserve the real schedule when possible and advance
                // colliding observations to the next unique sample slot. The
                // phase record below retains its actual injection timestamp.
                sequence,
                observed_unix_seconds: started_unix_seconds.saturating_add(
                    sequence.saturating_mul(SHORT_QUALIFICATION_SAMPLE_INTERVAL_SECONDS),
                ),
                observability_cursor_unix_seconds: Some(phase.injected_unix_seconds),
                resource_growth_percent: phase.resource_growth_percent,
                restart_count: (phase.name == "restart") as u64,
                backoff_seconds: phase.backoff_seconds,
                transport_error_count: phase.transport_error_count,
                recovery_seconds: Some(phase.recovery_seconds),
            }
        })
        .collect()
}

fn short_qualification_fault_contracts(
    records: &[FaultRecord],
    started_unix_seconds: u64,
) -> Vec<FaultContract> {
    records
        .iter()
        .map(|record| FaultContract {
            name: record.name.clone(),
            kind: match record.name.as_str() {
                "restart" => FaultKind::GuardianRestart,
                "dependency-degradation" => FaultKind::ResourcePressure,
                "vector-liveness" | "log-stagnation" => FaultKind::ObservabilityStall,
                "delayed-progress" | "shutdown" => FaultKind::RecoveryReplay,
                _ => FaultKind::RecoveryReplay,
            },
            inject_after_seconds: record
                .injected_unix_seconds
                .saturating_sub(started_unix_seconds),
            expected_recovery_seconds: record
                .recovered_unix_seconds
                .unwrap_or(record.injected_unix_seconds)
                .saturating_sub(record.injected_unix_seconds),
        })
        .collect()
}

fn short_qualification_fault_names() -> Vec<String> {
    vec![
        "restart".to_owned(),
        "delayed-progress".to_owned(),
        "dependency-degradation".to_owned(),
        "vector-liveness".to_owned(),
        "log-stagnation".to_owned(),
        "shutdown".to_owned(),
    ]
}

fn short_qualification_fault_records(workload: &WorkloadProof) -> Vec<FaultRecord> {
    workload
        .observed_phases
        .iter()
        .map(|phase| FaultRecord {
            name: phase.name.clone(),
            injected_unix_seconds: phase.injected_unix_seconds,
            recovered_unix_seconds: Some(phase.recovered_unix_seconds),
            notes: "observed by production lifecycle runner".to_owned(),
        })
        .collect()
}

fn short_qualification_duration_seconds(faults: &[FaultRecord], started_unix_seconds: u64) -> u64 {
    faults
        .iter()
        .filter_map(|fault| fault.recovered_unix_seconds)
        .map(|recovered| recovered.saturating_sub(started_unix_seconds))
        .max()
        .unwrap_or(0)
        .max(1)
}

fn short_qualification_max_recovery_seconds(faults: &[FaultRecord]) -> u64 {
    faults
        .iter()
        .map(|fault| {
            fault
                .recovered_unix_seconds
                .unwrap_or(fault.injected_unix_seconds)
                .saturating_sub(fault.injected_unix_seconds)
        })
        .max()
        .unwrap_or(0)
}

fn short_qualification_missing_sample_tolerance(
    faults: &[FaultRecord],
    started_unix_seconds: u64,
) -> u64 {
    short_qualification_duration_seconds(faults, started_unix_seconds)
        .saturating_sub(faults.len() as u64)
}

fn repository_root_for_init_template(init_template: &Path) -> Result<PathBuf, String> {
    let init_template = init_template.canonicalize().map_err(|error| {
        format!(
            "init template {} could not be canonicalized: {error}",
            init_template.display()
        )
    })?;
    init_template
        .ancestors()
        .find(|candidate| candidate.join(".git").exists())
        .map(Path::to_path_buf)
        .ok_or_else(|| {
            format!(
                "init template {} is not inside a Git worktree",
                init_template.display()
            )
        })
}

fn short_qualification_cleanup_residue(repository_root: &Path) -> BTreeMap<String, u64> {
    [
        ".adl/runtime-v3/qualification",
        ".adl/runtime-v3/soak/instances",
        ".adl/runtime-v3/soak/locks",
    ]
    .into_iter()
    .map(|path| {
        (
            path.to_owned(),
            residue_count(&repository_root.join(path.replace('/', std::path::MAIN_SEPARATOR_STR))),
        )
    })
    .collect()
}

fn soak_violation(code: impl Into<String>, detail: impl Into<String>) -> SoakViolation {
    SoakViolation {
        code: code.into(),
        detail: detail.into(),
    }
}

fn residue_count(path: &Path) -> u64 {
    match std::fs::read_dir(path) {
        Ok(entries) => entries.count().try_into().unwrap_or(u64::MAX),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => 0,
        Err(_) => u64::MAX,
    }
}

fn unix_seconds_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

fn write_report(path: &Path, report: &serde_json::Value) -> std::io::Result<()> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let temporary = path.with_extension("tmp");
    std::fs::write(&temporary, serde_json::to_vec_pretty(report)?)?;
    std::fs::rename(temporary, path)
}

fn write_secret(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    let mut options = std::fs::OpenOptions::new();
    options.write(true).create_new(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(0o600);
    }
    let mut file = options.open(path)?;
    std::io::Write::write_all(&mut file, bytes)?;
    file.sync_all()
}

fn read_pem_der_bytes(bytes: &[u8]) -> Result<Vec<u8>, String> {
    let text = std::str::from_utf8(bytes).map_err(|_| "configured PEM is not UTF-8".to_owned())?;
    let mut in_certificate = false;
    let mut body = String::new();
    for line in text.lines() {
        match line.trim() {
            "-----BEGIN CERTIFICATE-----" => {
                in_certificate = true;
                body.clear();
            }
            "-----END CERTIFICATE-----" if in_certificate => {
                return base64::engine::general_purpose::STANDARD
                    .decode(body.as_bytes())
                    .map_err(|_| "configured PEM certificate body is invalid".to_owned());
            }
            _ if in_certificate => body.push_str(line.trim()),
            _ => {}
        }
    }
    Err("configured PEM did not contain a certificate block".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn qualification_lock_serializes_the_configured_api_address() {
        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let lock_path = directory.path().join("api.lock");
        let address = "127.0.0.1:20997".parse().expect("test address");

        let first =
            QualificationLock::acquire_at(&lock_path, address).expect("first qualification lock");
        let contention = QualificationLock::acquire_at(&lock_path, address)
            .expect_err("second qualification must be rejected");
        assert!(contention.contains("another lifecycle qualification owns"));

        drop(first);
        QualificationLock::acquire_at(&lock_path, address)
            .expect("qualification lock should release with its owner");
    }

    #[test]
    fn checked_observability_is_discarded_between_timed_runs() {
        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let observability_root = directory.path().join("observability");
        std::fs::create_dir_all(&observability_root).expect("observability root");
        std::fs::write(observability_root.join("master.log.jsonl"), b"checked")
            .expect("checked log");

        discard_checked_observability(&observability_root)
            .expect("retained prior-run log should be discarded");
        assert!(!observability_root.exists());
        discard_checked_observability(&observability_root)
            .expect("already absent observability root is idempotent");
    }

    #[test]
    fn toml_path_preserves_windows_paths_through_serializer_round_trip() {
        let original = PathBuf::from(r#"C:\adl-wp-5344\state\quoted"name"#);
        let document = toml::Value::Table(toml::map::Map::from_iter([(
            "path".to_owned(),
            toml::Value::String(toml_path(&original).expect("portable path")),
        )]));
        let serialized = toml::to_string(&document).expect("serialize path");
        let parsed = toml::from_str::<toml::Value>(&serialized).expect("parse path");
        assert_eq!(
            parsed.get("path").and_then(toml::Value::as_str),
            original.to_str()
        );
    }

    #[tokio::test]
    async fn init_fixture_uses_config_owned_tls() {
        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let executable = std::env::current_exe().expect("current executable");
        let canonical_init_template = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("infra")
            .join("runtime-v3")
            .join("runtime-init.toml");
        let fixtures = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("tests")
            .join("support")
            .join("tls-fixtures");
        let certificate_chain = directory.path().join("server-cert.pem");
        let private_key = directory.path().join("server-key.pem");
        let trust_roots = directory.path().join("root-ca.pem");
        let guardian_certificate = directory.path().join("client-cert.pem");
        let guardian_private_key = directory.path().join("client-key.pem");
        let guardian_trust_roots = directory.path().join("guardian-root-ca.pem");
        std::fs::copy(fixtures.join("server-cert.pem"), &certificate_chain)
            .expect("copy certificate");
        std::fs::copy(fixtures.join("server-key.pem"), &private_key).expect("copy private key");
        std::fs::copy(fixtures.join("root-ca.pem"), &trust_roots).expect("copy trust roots");
        std::fs::copy(fixtures.join("client-cert.pem"), &guardian_certificate)
            .expect("copy Guardian certificate");
        std::fs::copy(fixtures.join("client-key.pem"), &guardian_private_key)
            .expect("copy Guardian private key");
        std::fs::copy(fixtures.join("root-ca.pem"), &guardian_trust_roots)
            .expect("copy Guardian trust roots");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&private_key, std::fs::Permissions::from_mode(0o600))
                .expect("protect private key");
            std::fs::set_permissions(
                &guardian_private_key,
                std::fs::Permissions::from_mode(0o600),
            )
            .expect("protect Guardian private key");
        }
        let mut document = toml::from_str::<toml::Value>(
            &std::fs::read_to_string(canonical_init_template).expect("canonical init"),
        )
        .expect("parse canonical init");
        set_toml_string(
            &mut document,
            &["api", "tls", "certificate_chain_path"],
            toml_path(&certificate_chain).unwrap(),
        )
        .unwrap();
        set_toml_string(
            &mut document,
            &["api", "tls", "private_key_path"],
            toml_path(&private_key).unwrap(),
        )
        .unwrap();
        set_toml_string(
            &mut document,
            &["api", "tls", "trust_roots_path"],
            toml_path(&trust_roots).unwrap(),
        )
        .unwrap();
        let init_template = directory.path().join("runtime-init.toml");
        let mut rendered = toml::to_string_pretty(&document).unwrap();
        rendered.push_str(&format!(
            r#"
[continuity_control]
address = "127.0.0.1:20998"
guardian_state_dir = {guardian_state:?}
state_dir = {kernel_state:?}
staging_dir = {staging:?}
trust_domain = "agent-logic.lifecycle"
polis = "lifecycle-polis"
source_node = "lifecycle-source"
target_node = "lifecycle-target"
guardian_id = "lifecycle-guardian"
kernel_control_id = "lifecycle-kernel-control"
channel_epoch = 1

[continuity_control.tls]
server_certificate_chain_path = {certificate_chain:?}
server_private_key_path = {private_key:?}
server_trust_roots_path = {trust_roots:?}
server_name = "localhost"
guardian_certificate_chain_path = {guardian_certificate:?}
guardian_private_key_path = {guardian_private_key:?}
guardian_trust_roots_path = {guardian_trust_roots:?}
guardian_spki_sha256 = "{digest}"
server_spki_sha256 = "{digest}"
certificate_generation = 1

[continuity_control.bounds]
max_frame_bytes = 65536
max_blob_bytes = 65536
max_total_bytes = 524288
max_services = 5
max_journal_entries = 64
max_open_handles = 8
"#,
            guardian_state = directory.path().join("guardian-continuity"),
            kernel_state = directory.path().join("kernel-continuity"),
            staging = directory.path().join("continuity-staging"),
            digest = "00".repeat(32),
        ));
        std::fs::write(&init_template, rendered).expect("write config-owned TLS init");

        let fixture = ProductionFixture::create(
            directory.path(),
            &init_template,
            &executable,
            &executable,
            Suite::Preflight,
            "0123456789abcdef0123456789abcdef01234567",
        )
        .await
        .expect("fixture should use externally provisioned TLS material");

        let init = std::fs::read_to_string(&fixture.init).expect("runtime init");
        let parsed = toml::from_str::<toml::Value>(&init).expect("runtime init toml");
        let certificate = PathBuf::from(
            parsed["api"]["tls"]["certificate_chain_path"]
                .as_str()
                .expect("certificate path"),
        );
        let private_key = PathBuf::from(
            parsed["api"]["tls"]["private_key_path"]
                .as_str()
                .expect("private key path"),
        );
        assert_eq!(
            certificate,
            directory.path().join("tls/certificate-chain.pem")
        );
        assert_eq!(private_key, directory.path().join("tls/private-key.pem"));
        assert_eq!(
            std::fs::read(certificate).unwrap(),
            std::fs::read(certificate_chain).unwrap()
        );
        assert_eq!(
            std::fs::read(private_key).unwrap(),
            std::fs::read(directory.path().join("server-key.pem")).unwrap()
        );
    }

    fn arguments(mode: &[&str]) -> Vec<String> {
        let root = std::env::current_dir().expect("current directory");
        let executable = std::env::current_exe()
            .expect("current executable")
            .to_string_lossy()
            .into_owned();
        let init_template = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("..")
            .join("infra")
            .join("runtime-v3")
            .join("runtime-init.toml");
        let mut values = vec![
            "--guardian".to_owned(),
            executable.clone(),
            "--kernel".to_owned(),
            executable.clone(),
            "--vector".to_owned(),
            executable,
            "--init-template".to_owned(),
            init_template.to_string_lossy().into_owned(),
            "--state-root".to_owned(),
            root.join("state").to_string_lossy().into_owned(),
            "--report".to_owned(),
            root.join("report.json").to_string_lossy().into_owned(),
            "--revision".to_owned(),
            "0123456789abcdef0123456789abcdef01234567".to_owned(),
        ];
        values.extend(mode.iter().map(|value| (*value).to_owned()));
        values
    }

    #[test]
    fn rejects_removed_tls_command_inputs() {
        for option in [
            "--tls-certificate-chain",
            "--tls-private-key",
            "--tls-trust-roots",
        ] {
            let mut values = arguments(&[]);
            values.extend([option.to_owned(), "/sensitive/tls/path".to_owned()]);
            let error = Args::parse(values.into_iter())
                .err()
                .expect("TLS argv must be rejected");
            assert_eq!(error, format!("unknown lifecycle soak option: {option}"));
        }
    }

    #[cfg(unix)]
    #[test]
    fn configured_tls_files_fail_closed_on_permissions_and_symlinks() {
        use std::os::unix::fs::{symlink, PermissionsExt};

        let current_dir = std::env::current_dir().expect("current directory");
        let directory = tempfile::tempdir_in(current_dir).expect("repo-local temporary directory");
        let key = directory.path().join("private-key.pem");
        std::fs::write(&key, b"private key fixture").expect("write key");
        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o644))
            .expect("set permissive mode");
        let document = toml::from_str::<toml::Value>(&format!(
            "[api.tls]\nprivate_key_path = {:?}\n",
            key.to_string_lossy()
        ))
        .expect("parse config");
        let error = configured_tls_file(&document, "private_key_path", "TLS private key", true)
            .err()
            .expect("permissive key must fail");
        assert_eq!(
            error,
            "configured TLS private key permissions must deny group and other access"
        );

        std::fs::set_permissions(&key, std::fs::Permissions::from_mode(0o600))
            .expect("protect key");
        let linked_key = directory.path().join("linked-private-key.pem");
        symlink(&key, &linked_key).expect("create symlink");
        let linked_document = toml::from_str::<toml::Value>(&format!(
            "[api.tls]\nprivate_key_path = {:?}\n",
            linked_key.to_string_lossy()
        ))
        .expect("parse linked config");
        let error = configured_tls_file(
            &linked_document,
            "private_key_path",
            "TLS private key",
            true,
        )
        .err()
        .expect("symlinked key must fail");
        assert_eq!(
            error,
            "configured TLS private key must be a regular non-symlink file"
        );

        let swappable = directory.path().join("swappable.pem");
        let replacement = directory.path().join("replacement.pem");
        std::fs::write(&swappable, b"first identity").expect("write first identity");
        std::fs::write(&replacement, b"replacement identity").expect("write replacement");
        std::fs::set_permissions(&swappable, std::fs::Permissions::from_mode(0o600))
            .expect("protect first identity");
        std::fs::set_permissions(&replacement, std::fs::Permissions::from_mode(0o600))
            .expect("protect replacement identity");
        let swap_document = toml::from_str::<toml::Value>(&format!(
            "[api.tls]\nprivate_key_path = {:?}\n",
            swappable.to_string_lossy()
        ))
        .expect("parse swap config");
        let error = configured_tls_file_with_pre_open(
            &swap_document,
            "private_key_path",
            "TLS private key",
            true,
            || std::fs::rename(&replacement, &swappable).expect("swap path identity"),
        )
        .err()
        .expect("identity substitution must fail");
        assert_eq!(
            error,
            "configured TLS private key changed while being opened"
        );
    }

    #[test]
    fn accepts_only_the_three_exact_acceptance_suites() {
        let lifecycle = Args::parse(arguments(&["--suite", "lifecycle_10000"]).into_iter())
            .expect("10k lifecycle suite");
        assert!(matches!(
            lifecycle.suite,
            Suite::Lifecycle {
                cycles: REQUIRED_CYCLES
            }
        ));

        let stress = Args::parse(arguments(&["--suite", "stress_100x10s"]).into_iter())
            .expect("100x10s stress suite");
        assert!(matches!(
            stress.suite,
            Suite::Stress {
                runs: STRESS_RUNS,
                seconds: STRESS_SECONDS
            }
        ));

        let endurance = Args::parse(arguments(&["--suite", "endurance_10x600s"]).into_iter())
            .expect("10x600s endurance suite");
        assert!(matches!(
            endurance.suite,
            Suite::Endurance {
                runs: ENDURANCE_RUNS,
                seconds: ENDURANCE_SECONDS
            }
        ));
    }

    #[test]
    fn pre_restart_probe_barrier_requires_paired_absolute_paths() {
        let root = std::env::current_dir().expect("current directory");
        let ready = root.join("pre-restart.ready");
        let ack = root.join("pre-restart.ack");
        let parsed = Args::parse(
            arguments(&[
                "--pre-restart-ready-file",
                ready.to_str().expect("ready path"),
                "--pre-restart-ack-file",
                ack.to_str().expect("ack path"),
            ])
            .into_iter(),
        )
        .expect("paired absolute barrier paths");
        assert_eq!(
            parsed.pre_restart_ready_file.as_deref(),
            Some(ready.as_path())
        );
        assert_eq!(parsed.pre_restart_ack_file.as_deref(), Some(ack.as_path()));

        let error = match Args::parse(
            arguments(&[
                "--pre-restart-ready-file",
                ready.to_str().expect("ready path"),
            ])
            .into_iter(),
        ) {
            Ok(_) => panic!("unpaired barrier path must fail closed"),
            Err(error) => error,
        };
        assert!(error.contains("must be provided together"));

        let alias_error = match Args::parse(
            arguments(&[
                "--pre-restart-ready-file",
                ready.to_str().expect("ready path"),
                "--pre-restart-ack-file",
                ready.to_str().expect("ready path"),
            ])
            .into_iter(),
        ) {
            Ok(_) => panic!("aliased barrier paths must fail closed"),
            Err(error) => error,
        };
        assert!(alias_error.contains("must be distinct"));

        let case_variant = root.join("PRE-RESTART.READY");
        let case_error = match Args::parse(
            arguments(&[
                "--pre-restart-ready-file",
                case_variant.to_str().expect("case-variant path"),
                "--pre-restart-ack-file",
                ack.to_str().expect("ack path"),
            ])
            .into_iter(),
        ) {
            Ok(_) => panic!("case-variant marker name must fail closed"),
            Err(error) => error,
        };
        assert!(case_error.contains("fixed ready and ack names"));
    }

    #[test]
    fn preflight_is_real_but_never_acceptance_eligible() {
        let preflight = Args::parse(arguments(&["--suite", "preflight_1x"]).into_iter())
            .expect("one-cycle preflight");
        assert!(matches!(preflight.suite, Suite::Preflight));
        let execution = Execution {
            completed_runs: 1,
            completed_cycles: 1,
            continuity_generation: 1,
            minimum_cycles_per_run: 1,
            guardian_pids: BTreeSet::from([1234]),
            runtime_instance_ids: BTreeSet::from(["runtime-test-instance".to_owned()]),
            guardian_launches: 1,
            runtime_starts: 1,
            anti_rollback_minimum_enforced: false,
            restart_budget_exercised: false,
            total_restarts: 0,
            log_checked_cycles: 1,
            log_proof: Some(LogProof {
                master_log_ref: ".csdlc/evidence/5344/work/master.jsonl".to_owned(),
                master_log_sha256: "b".repeat(64),
                master_log_records: 2,
                log_audit_ref: ".csdlc/evidence/5344/work/audit.json".to_owned(),
                log_audit_sha256: "c".repeat(64),
            }),
            workload_proof: None,
        };
        let value = report(
            &preflight,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );
        assert_eq!(value["acceptance_eligible"], false);
        assert_eq!(value["logging_complete"], true);
        assert_eq!(value["master_log_status"], "clean");
        assert_eq!(value["master_log_records"], 2);
        assert_eq!(value["runtime_v3_soak"]["status"], "not_observed");
    }

    #[test]
    fn short_qualification_report_binds_production_workload_to_soak_evidence() {
        let args = Args::parse(arguments(&["--suite", "preflight_1x"]).into_iter())
            .expect("one-cycle preflight");
        let mut execution = Execution::new(1, 1, 1);
        execution.completed_cycles = 1;
        execution.restart_budget_exercised = true;
        execution.total_restarts = 1;
        execution.log_checked_cycles = 1;
        execution.log_proof = Some(LogProof {
            master_log_ref: ".csdlc/evidence/267/work/master.jsonl".to_owned(),
            master_log_sha256: "b".repeat(64),
            master_log_records: 2,
            log_audit_ref: ".csdlc/evidence/267/work/audit.json".to_owned(),
            log_audit_sha256: "c".repeat(64),
        });
        execution.workload_proof = Some(WorkloadProof {
            authenticated_https_connections: SHORT_QUALIFICATION_CONNECTIONS,
            authenticated_wss_connections: SHORT_QUALIFICATION_CONNECTIONS,
            websocket_full_duplex_observed: true,
            observed_phases: test_observed_phases(),
        });

        let value = report(
            &args,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );

        assert_eq!(value["runtime_v3_soak"]["issue"], 267);
        assert_eq!(value["runtime_v3_soak"]["status"], "pass");
        assert_eq!(
            value["runtime_v3_soak"]["claim"],
            "short_local_linux_qualification_only"
        );
        assert_eq!(
            value["runtime_v3_soak"]["release_gate_recommendation"],
            false
        );
        assert_eq!(value["runtime_v3_soak"]["long_soak_claimed"], false);
        assert_eq!(value["runtime_v3_soak"]["provider_mutation"], false);
        assert_eq!(
            value["runtime_v3_soak"]["workload_observation"]["authenticated_https_connections"],
            50
        );
        assert_eq!(
            value["runtime_v3_soak"]["workload_observation"]["authenticated_wss_connections"],
            50
        );
        assert_eq!(
            value["runtime_v3_soak"]["config"]["schema"],
            SOAK_CONTRACT_SCHEMA
        );
        assert_eq!(value["runtime_v3_soak"]["config"]["issue"], 267);
        assert_eq!(
            value["runtime_v3_soak"]["evidence"]["evaluation"]["status"],
            "pass"
        );
        assert_eq!(
            value["runtime_v3_soak"]["evidence"]["faults"]
                .as_array()
                .expect("fault records")
                .len(),
            short_qualification_fault_names().len()
        );
        let config_faults = value["runtime_v3_soak"]["config"]["faults"]
            .as_array()
            .expect("config fault contracts");
        for name in ["vector-liveness", "log-stagnation"] {
            let fault = config_faults
                .iter()
                .find(|fault| fault["name"] == name)
                .expect("observability fault contract");
            assert_eq!(fault["kind"], "observability_stall");
        }
    }

    #[test]
    fn short_qualification_fails_closed_without_dependency_degradation_receipt() {
        let args = Args::parse(arguments(&["--suite", "preflight_1x"]).into_iter())
            .expect("one-cycle preflight");
        let mut execution = Execution::new(1, 1, 1);
        execution.completed_cycles = 1;
        execution.restart_budget_exercised = true;
        execution.total_restarts = 1;
        execution.log_checked_cycles = 1;
        execution.log_proof = Some(LogProof {
            master_log_ref: ".csdlc/evidence/373/work/master.jsonl".to_owned(),
            master_log_sha256: "b".repeat(64),
            master_log_records: 2,
            log_audit_ref: ".csdlc/evidence/373/work/audit.json".to_owned(),
            log_audit_sha256: "c".repeat(64),
        });
        let observed_phases = test_observed_phases()
            .into_iter()
            .filter(|phase| phase.name != "dependency-degradation")
            .collect();
        execution.workload_proof = Some(WorkloadProof {
            authenticated_https_connections: SHORT_QUALIFICATION_CONNECTIONS,
            authenticated_wss_connections: SHORT_QUALIFICATION_CONNECTIONS,
            websocket_full_duplex_observed: true,
            observed_phases,
        });

        let value = report(
            &args,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );

        assert_eq!(value["runtime_v3_soak"]["status"], "fail_closed");
        assert!(value["runtime_v3_soak"]["violations"]
            .as_array()
            .expect("violations")
            .iter()
            .any(
                |violation| violation["detail"] == "dependency-degradation phase was not observed"
            ));
    }

    #[test]
    fn short_qualification_evidence_fails_closed_when_fanout_is_under_counted() {
        let args = Args::parse(arguments(&["--suite", "preflight_1x"]).into_iter())
            .expect("one-cycle preflight");
        let mut execution = Execution::new(1, 1, 1);
        execution.completed_cycles = 1;
        execution.restart_budget_exercised = true;
        execution.total_restarts = 1;
        execution.log_checked_cycles = 1;
        execution.log_proof = Some(LogProof {
            master_log_ref: ".csdlc/evidence/267/work/master.jsonl".to_owned(),
            master_log_sha256: "b".repeat(64),
            master_log_records: 2,
            log_audit_ref: ".csdlc/evidence/267/work/audit.json".to_owned(),
            log_audit_sha256: "c".repeat(64),
        });
        execution.workload_proof = Some(WorkloadProof {
            authenticated_https_connections: SHORT_QUALIFICATION_CONNECTIONS - 1,
            authenticated_wss_connections: SHORT_QUALIFICATION_CONNECTIONS,
            websocket_full_duplex_observed: true,
            observed_phases: test_observed_phases(),
        });

        let value = report(
            &args,
            &"a".repeat(64),
            Instant::now(),
            "pass",
            &execution,
            None,
        );

        assert_eq!(value["runtime_v3_soak"]["status"], "fail_closed");
        assert!(value["runtime_v3_soak"]["violations"]
            .as_array()
            .expect("violations")
            .iter()
            .any(|violation| violation["code"] == "missing_observation"));
    }

    #[test]
    fn short_qualification_samples_keep_identity_when_phases_share_a_second() {
        let mut phases = test_observed_phases();
        phases[2].injected_unix_seconds = phases[1].injected_unix_seconds;
        let workload = WorkloadProof {
            authenticated_https_connections: SHORT_QUALIFICATION_CONNECTIONS,
            authenticated_wss_connections: SHORT_QUALIFICATION_CONNECTIONS,
            websocket_full_duplex_observed: true,
            observed_phases: phases,
        };

        let samples = short_qualification_samples(&workload);
        assert_eq!(samples.len(), short_qualification_fault_names().len());
        assert_eq!(samples[1].sequence, 2);
        assert_eq!(samples[2].sequence, 3);
        assert_eq!(
            samples[1].observability_cursor_unix_seconds,
            samples[2].observability_cursor_unix_seconds
        );
        assert_eq!(
            samples[2].observed_unix_seconds,
            samples[1].observed_unix_seconds + SHORT_QUALIFICATION_SAMPLE_INTERVAL_SECONDS
        );
    }

    #[test]
    fn runtime_state_directories_reject_absolute_and_traversal_escape() {
        let directory = tempfile::tempdir().expect("state root");
        let root = directory.path().canonicalize().expect("canonical root");

        assert!(create_contained_state_dir(&root, "/tmp/external", "TLS state").is_err());
        assert!(create_contained_state_dir(&root, "../external", "TLS state").is_err());
        assert!(create_contained_state_dir(&root, "tls/../../external", "TLS state").is_err());
        assert!(create_contained_absolute_state_dir(
            &root,
            &root.join("continuity/../external"),
            "private continuity state"
        )
        .is_err());
    }

    #[cfg(unix)]
    #[test]
    fn runtime_state_directories_reject_symlink_escape() {
        use std::os::unix::fs::symlink;

        let directory = tempfile::tempdir().expect("state root");
        let external = tempfile::tempdir().expect("external root");
        let root = directory.path().canonicalize().expect("canonical root");
        symlink(external.path(), root.join("tls")).expect("escape symlink");

        assert!(create_contained_state_dir(&root, "tls/snapshots", "TLS state").is_err());
        assert!(create_contained_absolute_state_dir(
            &root,
            &root.join("tls/continuity"),
            "private continuity state"
        )
        .is_err());
        assert!(!external.path().join("snapshots").exists());
    }

    fn test_observed_phases() -> Vec<ObservedPhase> {
        short_qualification_fault_names()
            .into_iter()
            .enumerate()
            .map(|(index, name)| {
                let injected_unix_seconds = 1_700_000_000 + (index as u64 * 2);
                let recovered_unix_seconds = injected_unix_seconds + 1;
                ObservedPhase {
                    kind: match name.as_str() {
                        "restart" => FaultKind::GuardianRestart,
                        "dependency-degradation" => FaultKind::ResourcePressure,
                        "vector-liveness" | "log-stagnation" => FaultKind::ObservabilityStall,
                        _ => FaultKind::RecoveryReplay,
                    },
                    name,
                    injected_unix_seconds,
                    recovered_unix_seconds,
                    resource_growth_percent: 1,
                    backoff_seconds: 0,
                    transport_error_count: 0,
                    recovery_seconds: recovered_unix_seconds.saturating_sub(injected_unix_seconds),
                }
            })
            .collect()
    }

    #[test]
    fn nonzero_guardian_diagnostic_preserves_child_exit_cause() {
        let root = std::env::current_dir().expect("current directory");
        let outcome = GuardianOutcome {
            schema: "adl.runtime_v3.guardian.v1".to_owned(),
            terminal_state: GuardianTerminalState::ShutdownForced,
            attempts: 1,
            restarts: 0,
            attempts_detail: vec![adl_runtime::guardian::GuardianAttempt {
                attempt: 1,
                pid: Some(42),
                exit_code: Some(70),
                exit_status: Some("exit code: 70".to_owned()),
                unix_signal: None,
                windows_ctrl_event: Some(1),
                forced_shutdown: false,
                clean_checkpointed_shutdown: false,
                stdout: format!("stopped {}", root.display()),
                stderr: "runtime shutdown failed: component".to_owned(),
                reason_code: "shutdown_child_failed".to_owned(),
            }],
        };
        let stdout = serde_json::to_vec(&outcome).expect("Guardian outcome");

        let diagnostic = guardian_failure_diagnostic(&stdout, &root);

        assert!(diagnostic.contains("terminal_state=ShutdownForced"));
        assert!(diagnostic.contains("exit_code=Some(70)"));
        assert!(diagnostic.contains("windows_ctrl_event=Some(1)"));
        assert!(diagnostic.contains("reason_code=shutdown_child_failed"));
        assert!(diagnostic.contains("runtime shutdown failed: component"));
        assert!(!diagnostic.contains(&root.to_string_lossy().to_string()));
    }

    #[test]
    fn rejects_partial_or_mixed_acceptance_suites() {
        for mode in [
            vec!["--suite", "lifecycle_9999"],
            vec!["--suite", "stress_100x9s"],
            vec!["--suite", "endurance_9x600s"],
            vec!["--suite", "lifecycle_10000", "--suite", "stress_100x10s"],
            vec!["--preflight", "--suite", "lifecycle_10000"],
        ] {
            assert!(
                Args::parse(arguments(&mode).into_iter()).is_err(),
                "unexpectedly accepted {mode:?}"
            );
        }
    }

    #[test]
    fn aggregates_four_exact_lifecycle_reports_with_compact_clean_logs() {
        let root = std::env::current_dir().expect("current directory");
        let temp = tempfile::tempdir_in(&root).expect("repo-local temp evidence");
        let revision = "0123456789abcdef0123456789abcdef01234567";
        let kernel_sha256 = &"a".repeat(64);
        let preflight = write_sample_report(
            &root,
            temp.path(),
            "preflight_1x",
            revision,
            kernel_sha256,
            1,
        );
        let lifecycle = write_sample_report(
            &root,
            temp.path(),
            "lifecycle_10000",
            revision,
            kernel_sha256,
            REQUIRED_CYCLES,
        );
        let stress = write_sample_report(
            &root,
            temp.path(),
            "stress_100x10s",
            revision,
            kernel_sha256,
            42,
        );
        let endurance = write_sample_report(
            &root,
            temp.path(),
            "endurance_10x600s",
            revision,
            kernel_sha256,
            24,
        );
        let output = temp.path().join("platform-proof.json");
        let args = AggregateArgs {
            preflight_report: preflight,
            lifecycle_report: lifecycle,
            stress_report: stress,
            endurance_report: endurance,
            output: output.clone(),
        };

        let proof = build_platform_proof(&args).expect("platform proof");
        write_report(&output, &proof).expect("atomic proof write");
        let written: serde_json::Value =
            serde_json::from_slice(&std::fs::read(output).expect("written proof"))
                .expect("proof JSON");

        assert_eq!(written["schema"], PLATFORM_PROOF_SCHEMA);
        assert_eq!(written["status"], "pass");
        assert_eq!(written["guardian_process_zero"], true);
        assert_eq!(written["native_execution"], true);
        assert_eq!(written["wsl_used"], false);
        assert_eq!(written["docker_used"], false);
        assert_eq!(written["lifecycle_acceptance"]["all_logs_clean"], true);
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["completed_cycles"],
            REQUIRED_CYCLES
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["failed_cycles"],
            0
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["degraded_cycles"],
            0
        );
        assert_eq!(
            written["lifecycle_acceptance"]["lifecycle_10000"]["master_log_records"],
            3
        );
        assert_eq!(
            written["lifecycle_acceptance"]["stress_100x10s"]["master_log_records"],
            3
        );
    }

    #[test]
    fn vector_recovery_receipt_requires_restart_and_recovered_master_log_records_after_baseline() {
        let root = std::env::current_dir().expect("current directory");
        let temp = tempfile::tempdir_in(&root).expect("repo-local temp evidence");
        let log = temp.path().join("master.log.jsonl");
        std::fs::write(
            &log,
            concat!(
                "{\"sequence\":1,\"operation\":\"vector_pipeline_restarting\",\"reason\":\"vector_child_exited\"}\n",
                "{\"sequence\":2,\"operation\":\"vector_pipeline_recovered\"}\n",
                "{\"sequence\":3,\"operation\":\"vector_pipeline_restarting\",\"reason\":\"operator_restart\"}\n",
            ),
        )
        .expect("baseline master log");

        assert_eq!(master_log_highest_sequence_for_soak(&log).unwrap(), 3);
        assert!(!master_log_has_vector_recovery_after(&log, 3).unwrap());

        let mut text = std::fs::read_to_string(&log).expect("master log text");
        text.push_str(concat!(
            "{\"sequence\":4,\"operation\":\"vector_pipeline_restarting\",\"reason\":\"vector_child_exited\"}\n",
            "{\"sequence\":5,\"operation\":\"vector_pipeline_recovered\"}\n",
        ));
        std::fs::write(&log, text).expect("extended master log");

        assert!(master_log_has_vector_recovery_after(&log, 3).unwrap());

        let reversed = temp.path().join("reversed-master.log.jsonl");
        std::fs::write(
            &reversed,
            concat!(
                "{\"sequence\":1,\"operation\":\"vector_pipeline_recovered\"}\n",
                "{\"sequence\":2,\"operation\":\"vector_pipeline_restarting\",\"reason\":\"vector_child_exited\"}\n",
            ),
        )
        .expect("reversed master log");
        assert!(!master_log_has_vector_recovery_after(&reversed, 0).unwrap());
    }

    fn write_sample_report(
        root: &Path,
        temp: &Path,
        suite: &str,
        revision: &str,
        kernel_sha256: &str,
        completed_cycles: u64,
    ) -> PathBuf {
        let suite_dir = temp.join(suite);
        std::fs::create_dir_all(&suite_dir).expect("suite dir");
        let log = suite_dir.join("master.log.jsonl");
        std::fs::write(
            &log,
            b"{\"sequence\":1,\"level\":\"info\"}\n{\"sequence\":2,\"level\":\"info\"}\n{\"sequence\":3,\"level\":\"info\"}\n",
        )
        .expect("master log");
        let log_sha256 = file_sha256(&log).expect("log sha");
        let audit = suite_dir.join("master-log-audit.json");
        let audit_value = serde_json::json!({
            "schema": "adl.runtime.master_log_audit.v1",
            "status": "pass",
            "platform": std::env::consts::OS,
            "suite": suite,
            "revision": revision,
            "master_log_sha256": log_sha256,
            "record_count": 3,
            "malformed_records": 0,
            "missing_required_fields": 0,
            "sequence_gaps": 0,
            "error_events": 0,
            "degraded_events": 0,
            "unexplained_restarts": 0,
            "incomplete_drains": 0,
        });
        std::fs::write(
            &audit,
            serde_json::to_vec_pretty(&audit_value).expect("audit bytes"),
        )
        .expect("audit");
        let audit_sha256 = file_sha256(&audit).expect("audit sha");
        let (requested_runs, requested_cycles, duration_seconds) = match suite {
            "preflight_1x" => (1, Some(1), None),
            "lifecycle_10000" => (1, Some(REQUIRED_CYCLES), None),
            "stress_100x10s" => (STRESS_RUNS, None, Some(STRESS_SECONDS)),
            "endurance_10x600s" => (ENDURANCE_RUNS, None, Some(ENDURANCE_SECONDS)),
            _ => panic!("unsupported sample suite"),
        };
        let report = serde_json::json!({
            "schema": REPORT_SCHEMA,
            "status": "pass",
            "suite": suite,
            "platform": std::env::consts::OS,
            "architecture": std::env::consts::ARCH,
            "revision": revision,
            "requested_cycles": requested_cycles,
            "requested_runs": requested_runs,
            "duration_seconds_per_run": duration_seconds,
            "completed_runs": requested_runs,
            "completed_cycles": completed_cycles,
            "failed_cycles": 0,
            "degraded_cycles": 0,
            "minimum_cycles_per_run": completed_cycles.max(1),
            "guardian_process_count": 1,
            "guardian_launch_count": completed_cycles,
            "runtime_instance_count": 1,
            "runtime_start_count": completed_cycles + 1,
            "anti_rollback_minimum_enforced": suite != "preflight_1x",
            "restart_budget_exercised": true,
            "total_restarts": 1,
            "acceptance_eligible": suite != "preflight_1x",
            "logging_complete": true,
            "log_checked_cycles": completed_cycles,
            "master_log_status": "clean",
            "master_log_ref": rel(root, &log),
            "master_log_sha256": log_sha256,
            "master_log_records": 3,
            "log_audit_ref": rel(root, &audit),
            "log_audit_sha256": audit_sha256,
            "continuity_generation": completed_cycles,
            "kernel_sha256": kernel_sha256,
            "duration_millis": 1,
            "failure": null,
        });
        let report_path = suite_dir.join("report.json");
        std::fs::write(
            &report_path,
            serde_json::to_vec_pretty(&report).expect("report bytes"),
        )
        .expect("report");
        report_path
    }

    fn rel(root: &Path, path: &Path) -> String {
        path.strip_prefix(root)
            .expect("repo-relative test path")
            .to_string_lossy()
            .replace('\\', "/")
    }
}
