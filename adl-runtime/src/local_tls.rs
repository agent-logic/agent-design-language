use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    net::IpAddr,
    path::{Component, Path, PathBuf},
    sync::{Mutex, OnceLock},
};

use rcgen::{date_time_ymd, CertificateParams, ExtendedKeyUsagePurpose, KeyPair};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const LOCAL_TLS_BOOTSTRAP_SCHEMA: &str = "adl.runtime_v3.local_tls_bootstrap.v1";
pub const LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA: &str =
    "adl.runtime_v3.local_tls_bootstrap.outcome.v1";

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RuntimeTlsBootstrapConfig {
    pub schema: String,
    pub mode: RuntimeTlsBootstrapMode,
    #[serde(default)]
    pub state_root: Option<PathBuf>,
    #[serde(default)]
    pub tls_dir: Option<PathBuf>,
    pub certificate_chain_path: PathBuf,
    #[serde(default)]
    pub public_certificate_path: Option<PathBuf>,
    pub private_key_path: PathBuf,
    #[serde(default)]
    pub dns_names: Vec<String>,
    #[serde(default)]
    pub ip_addresses: Vec<IpAddr>,
    #[serde(default)]
    pub replace: bool,
}

impl RuntimeTlsBootstrapConfig {
    pub fn from_toml_str(text: &str) -> Result<Self, LocalTlsError> {
        let config: Self =
            toml::from_str(text).map_err(|error| LocalTlsError::Config(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn from_json_str(text: &str) -> Result<Self, LocalTlsError> {
        let config: Self =
            serde_json::from_str(text).map_err(|error| LocalTlsError::Config(error.to_string()))?;
        config.validate()?;
        Ok(config)
    }

    pub fn validate(&self) -> Result<(), LocalTlsError> {
        if self.schema != LOCAL_TLS_BOOTSTRAP_SCHEMA {
            return Err(LocalTlsError::UnsupportedSchema(self.schema.clone()));
        }
        match self.mode {
            RuntimeTlsBootstrapMode::ManagedExternal => {
                if self.replace {
                    return Err(LocalTlsError::Policy(
                        "managed_external TLS does not support local replacement".to_owned(),
                    ));
                }
                if self.certificate_chain_path.as_os_str().is_empty()
                    || self.private_key_path.as_os_str().is_empty()
                    || self.certificate_chain_path == self.private_key_path
                {
                    return Err(LocalTlsError::Policy(
                        "managed_external TLS requires distinct certificate and key paths"
                            .to_owned(),
                    ));
                }
                Ok(())
            }
            RuntimeTlsBootstrapMode::LocalSelfSigned => {
                let state_root = self
                    .state_root
                    .as_ref()
                    .ok_or_else(|| LocalTlsError::Policy("state_root is required".to_owned()))?;
                if !state_root.is_absolute() {
                    return Err(LocalTlsError::Policy(
                        "state_root must be an absolute configured path".to_owned(),
                    ));
                }
                let tls_dir = self
                    .tls_dir
                    .as_ref()
                    .ok_or_else(|| LocalTlsError::Policy("tls_dir is required".to_owned()))?;
                validate_relative_child("tls_dir", tls_dir)?;
                validate_relative_child("certificate_chain_path", &self.certificate_chain_path)?;
                validate_relative_child("private_key_path", &self.private_key_path)?;
                let public_certificate_path =
                    self.public_certificate_path.as_ref().ok_or_else(|| {
                        LocalTlsError::Policy("public_certificate_path is required".to_owned())
                    })?;
                validate_relative_child("public_certificate_path", public_certificate_path)?;
                if self.certificate_chain_path == self.private_key_path
                    || self.certificate_chain_path == *public_certificate_path
                    || self.private_key_path == *public_certificate_path
                {
                    return Err(LocalTlsError::Policy(
                        "local TLS certificate, public certificate, and key paths must be distinct"
                            .to_owned(),
                    ));
                }
                if self.dns_names.is_empty() && self.ip_addresses.is_empty() {
                    return Err(LocalTlsError::Policy(
                        "local_self_signed TLS requires at least one DNS or IP SAN".to_owned(),
                    ));
                }
                for name in &self.dns_names {
                    if name.trim().is_empty() || name.contains('/') || name.contains('\\') {
                        return Err(LocalTlsError::Policy(
                            "DNS SAN entries must be non-empty host names".to_owned(),
                        ));
                    }
                }
                Ok(())
            }
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeTlsBootstrapMode {
    ManagedExternal,
    LocalSelfSigned,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct RuntimeTlsBootstrapOutcome {
    pub schema: String,
    pub mode: RuntimeTlsBootstrapMode,
    pub certificate_chain_path: PathBuf,
    pub public_certificate_path: Option<PathBuf>,
    pub certificate_sha256: Option<String>,
    pub reused_existing_identity: bool,
    pub replaced_existing_identity: bool,
    pub event: RuntimeTlsBootstrapEvent,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RuntimeTlsBootstrapEvent {
    ManagedExternalPreserved,
    LocalCertificateCreated,
    LocalCertificateReused,
    LocalCertificateReplaced,
}

#[derive(Debug)]
pub enum LocalTlsError {
    UnsupportedSchema(String),
    Config(String),
    Policy(String),
    LockBusy,
    Io(String),
    Generate(String),
    Rustls(String),
}

impl std::fmt::Display for LocalTlsError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocalTlsError::UnsupportedSchema(schema) => {
                write!(
                    formatter,
                    "unsupported local TLS bootstrap schema: {schema}"
                )
            }
            LocalTlsError::Config(error) => {
                write!(formatter, "invalid local TLS bootstrap config: {error}")
            }
            LocalTlsError::Policy(error) => {
                write!(
                    formatter,
                    "local TLS policy rejected configuration: {error}"
                )
            }
            LocalTlsError::LockBusy => {
                write!(
                    formatter,
                    "local TLS bootstrap is already active for this state root"
                )
            }
            LocalTlsError::Io(error) => write!(formatter, "local TLS I/O failed: {error}"),
            LocalTlsError::Generate(error) => {
                write!(
                    formatter,
                    "local TLS certificate generation failed: {error}"
                )
            }
            LocalTlsError::Rustls(error) => {
                write!(
                    formatter,
                    "local TLS material failed rustls validation: {error}"
                )
            }
        }
    }
}

impl std::error::Error for LocalTlsError {}

struct LocalTlsPaths {
    tls_root: PathBuf,
    certificate_chain: PathBuf,
    public_certificate: PathBuf,
    private_key: PathBuf,
    lock_dir: PathBuf,
}

pub struct GeneratedTlsMaterial {
    pub certificate_pem: String,
    pub private_key_pem: String,
}

pub async fn bootstrap_runtime_tls(
    config: &RuntimeTlsBootstrapConfig,
) -> Result<RuntimeTlsBootstrapOutcome, LocalTlsError> {
    bootstrap_runtime_tls_with_generator(config, generate_local_material).await
}

pub async fn bootstrap_runtime_tls_with_generator<F>(
    config: &RuntimeTlsBootstrapConfig,
    generator: F,
) -> Result<RuntimeTlsBootstrapOutcome, LocalTlsError>
where
    F: FnOnce(&RuntimeTlsBootstrapConfig) -> Result<GeneratedTlsMaterial, LocalTlsError>,
{
    config.validate()?;
    match config.mode {
        RuntimeTlsBootstrapMode::ManagedExternal => {
            validate_rustls_pair(&config.certificate_chain_path, &config.private_key_path).await?;
            Ok(RuntimeTlsBootstrapOutcome {
                schema: LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA.to_owned(),
                mode: config.mode,
                certificate_chain_path: config.certificate_chain_path.clone(),
                public_certificate_path: config.public_certificate_path.clone(),
                certificate_sha256: sha256_file(&config.certificate_chain_path).ok(),
                reused_existing_identity: true,
                replaced_existing_identity: false,
                event: RuntimeTlsBootstrapEvent::ManagedExternalPreserved,
            })
        }
        RuntimeTlsBootstrapMode::LocalSelfSigned => {
            let paths = local_paths(config)?;
            fs::create_dir_all(&paths.tls_root)
                .map_err(|error| LocalTlsError::Io(error.to_string()))?;
            let _guard = LocalBootstrapGuard::acquire(&paths.lock_dir)?;
            let cert_exists = paths.certificate_chain.exists();
            let key_exists = paths.private_key.exists();
            if cert_exists != key_exists {
                return Err(LocalTlsError::Policy(
                    "local TLS certificate and key must be created or replaced together".to_owned(),
                ));
            }
            if cert_exists && !config.replace {
                validate_rustls_pair(&paths.certificate_chain, &paths.private_key).await?;
                ensure_public_certificate_copy(
                    &paths.certificate_chain,
                    &paths.public_certificate,
                )?;
                return Ok(local_outcome(
                    config.mode,
                    &paths,
                    true,
                    false,
                    RuntimeTlsBootstrapEvent::LocalCertificateReused,
                ));
            }
            if cert_exists {
                validate_rustls_pair(&paths.certificate_chain, &paths.private_key).await?;
            }
            let material = generator(config)?;
            write_candidate(&paths, &material).await?;
            Ok(local_outcome(
                config.mode,
                &paths,
                false,
                cert_exists,
                if cert_exists {
                    RuntimeTlsBootstrapEvent::LocalCertificateReplaced
                } else {
                    RuntimeTlsBootstrapEvent::LocalCertificateCreated
                },
            ))
        }
    }
}

fn local_outcome(
    mode: RuntimeTlsBootstrapMode,
    paths: &LocalTlsPaths,
    reused: bool,
    replaced: bool,
    event: RuntimeTlsBootstrapEvent,
) -> RuntimeTlsBootstrapOutcome {
    RuntimeTlsBootstrapOutcome {
        schema: LOCAL_TLS_BOOTSTRAP_OUTCOME_SCHEMA.to_owned(),
        mode,
        certificate_chain_path: paths.certificate_chain.clone(),
        public_certificate_path: Some(paths.public_certificate.clone()),
        certificate_sha256: sha256_file(&paths.certificate_chain).ok(),
        reused_existing_identity: reused,
        replaced_existing_identity: replaced,
        event,
    }
}

fn generate_local_material(
    config: &RuntimeTlsBootstrapConfig,
) -> Result<GeneratedTlsMaterial, LocalTlsError> {
    let mut names = config.dns_names.clone();
    names.extend(config.ip_addresses.iter().map(ToString::to_string));
    let mut params = CertificateParams::new(names)
        .map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    params.not_before = date_time_ymd(2026, 1, 1);
    params.not_after = date_time_ymd(2036, 1, 1);
    params.extended_key_usages = vec![ExtendedKeyUsagePurpose::ServerAuth];
    let key = KeyPair::generate().map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    let cert = params
        .self_signed(&key)
        .map_err(|error| LocalTlsError::Generate(error.to_string()))?;
    Ok(GeneratedTlsMaterial {
        certificate_pem: cert.pem(),
        private_key_pem: key.serialize_pem(),
    })
}

async fn write_candidate(
    paths: &LocalTlsPaths,
    material: &GeneratedTlsMaterial,
) -> Result<(), LocalTlsError> {
    let nonce = format!("{}.{}", std::process::id(), unique_suffix());
    let cert_tmp = paths.tls_root.join(format!("certificate.{nonce}.tmp"));
    let public_tmp = paths
        .tls_root
        .join(format!("public-certificate.{nonce}.tmp"));
    let key_tmp = paths.tls_root.join(format!("private-key.{nonce}.tmp"));
    write_file(
        &cert_tmp,
        material.certificate_pem.as_bytes(),
        FileMode::Public,
    )?;
    write_file(
        &public_tmp,
        material.certificate_pem.as_bytes(),
        FileMode::Public,
    )?;
    write_file(
        &key_tmp,
        material.private_key_pem.as_bytes(),
        FileMode::Private,
    )?;
    if let Err(error) = validate_rustls_pair(&cert_tmp, &key_tmp).await {
        let _ = fs::remove_file(&cert_tmp);
        let _ = fs::remove_file(&public_tmp);
        let _ = fs::remove_file(&key_tmp);
        return Err(error);
    }
    replace_file(&cert_tmp, &paths.certificate_chain)?;
    replace_file(&public_tmp, &paths.public_certificate)?;
    replace_file(&key_tmp, &paths.private_key)?;
    Ok(())
}

fn ensure_public_certificate_copy(source: &Path, target: &Path) -> Result<(), LocalTlsError> {
    if target.exists() {
        return Ok(());
    }
    let bytes = fs::read(source).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    write_file(target, &bytes, FileMode::Public)
}

async fn validate_rustls_pair(certificate: &Path, private_key: &Path) -> Result<(), LocalTlsError> {
    axum_server::tls_rustls::RustlsConfig::from_pem_file(certificate, private_key)
        .await
        .map(|_| ())
        .map_err(|error| LocalTlsError::Rustls(error.to_string()))
}

fn local_paths(config: &RuntimeTlsBootstrapConfig) -> Result<LocalTlsPaths, LocalTlsError> {
    let state_root = config
        .state_root
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("state_root is required".to_owned()))?;
    let tls_dir = config
        .tls_dir
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("tls_dir is required".to_owned()))?;
    let public_certificate = config
        .public_certificate_path
        .as_ref()
        .ok_or_else(|| LocalTlsError::Policy("public_certificate_path is required".to_owned()))?;
    let tls_root = state_root.join(tls_dir);
    Ok(LocalTlsPaths {
        lock_dir: tls_root.join(".bootstrap.lock"),
        certificate_chain: tls_root.join(&config.certificate_chain_path),
        public_certificate: tls_root.join(public_certificate),
        private_key: tls_root.join(&config.private_key_path),
        tls_root,
    })
}

fn validate_relative_child(field: &'static str, path: &Path) -> Result<(), LocalTlsError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(LocalTlsError::Policy(format!("{field} must be relative")));
    }
    if path.components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    }) {
        return Err(LocalTlsError::Policy(format!(
            "{field} must stay inside the configured state root"
        )));
    }
    Ok(())
}

#[derive(Clone, Copy)]
enum FileMode {
    Public,
    Private,
}

fn write_file(path: &Path, bytes: &[u8], mode: FileMode) -> Result<(), LocalTlsError> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    }
    let mut options = OpenOptions::new();
    options.create_new(true).write(true);
    #[cfg(unix)]
    {
        use std::os::unix::fs::OpenOptionsExt;
        options.mode(match mode {
            FileMode::Public => 0o644,
            FileMode::Private => 0o600,
        });
    }
    let mut file = options
        .open(path)
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    file.write_all(bytes)
        .and_then(|()| file.sync_all())
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
    Ok(())
}

fn replace_file(source: &Path, target: &Path) -> Result<(), LocalTlsError> {
    if target.exists() {
        fs::remove_file(target).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    }
    fs::rename(source, target).map_err(|error| LocalTlsError::Io(error.to_string()))
}

fn sha256_file(path: &Path) -> Result<String, LocalTlsError> {
    let bytes = fs::read(path).map_err(|error| LocalTlsError::Io(error.to_string()))?;
    Ok(hex::encode(Sha256::digest(bytes)))
}

fn unique_suffix() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos().to_string())
        .unwrap_or_else(|_| "0".to_owned())
}

struct LocalBootstrapGuard {
    path: PathBuf,
}

impl LocalBootstrapGuard {
    fn acquire(path: &Path) -> Result<Self, LocalTlsError> {
        let canonical_key = path.to_path_buf();
        let locks = in_process_locks();
        {
            let mut active = locks
                .lock()
                .map_err(|_| LocalTlsError::Policy("local TLS lock poisoned".to_owned()))?;
            if !active.insert(canonical_key.clone()) {
                return Err(LocalTlsError::LockBusy);
            }
        }
        if let Err(error) = fs::create_dir(path) {
            let mut active = locks
                .lock()
                .map_err(|_| LocalTlsError::Policy("local TLS lock poisoned".to_owned()))?;
            active.remove(&canonical_key);
            if error.kind() == std::io::ErrorKind::AlreadyExists {
                return Err(LocalTlsError::LockBusy);
            }
            return Err(LocalTlsError::Io(error.to_string()));
        }
        Ok(Self {
            path: canonical_key,
        })
    }
}

impl Drop for LocalBootstrapGuard {
    fn drop(&mut self) {
        let _ = fs::remove_dir(&self.path);
        if let Ok(mut active) = in_process_locks().lock() {
            active.remove(&self.path);
        }
    }
}

fn in_process_locks() -> &'static Mutex<BTreeSet<PathBuf>> {
    static LOCKS: OnceLock<Mutex<BTreeSet<PathBuf>>> = OnceLock::new();
    LOCKS.get_or_init(|| Mutex::new(BTreeSet::new()))
}
