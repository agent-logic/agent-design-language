use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    path::{Path, PathBuf},
    process::{Command, ExitCode},
};

use adl_runtime::local_tls::{
    bootstrap_runtime_tls, certificate_fingerprint_sha256, current_local_certificate_sha256,
    reissue_runtime_tls_with_trust, LocalTlsError, LocalTlsTrustTransaction,
    RuntimeTlsBootstrapConfig, RuntimeTlsBootstrapMode,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const FAILURE_SCHEMA: &str = "adl.runtime_v3.local_tls_bootstrap.failure.v1";
const TRUST_OUTCOME_SCHEMA: &str = "adl.runtime_v3.local_tls_trust.outcome.v1";
const TRUST_RECEIPT_SCHEMA: &str = "adl.runtime_v3.local_tls_trust_receipt.v1";
const TRUST_RECEIPTS_DIR: &str = "trust-receipts";

#[tokio::main]
async fn main() -> ExitCode {
    let args = match Args::parse(std::env::args().skip(1).collect()) {
        Ok(args) => args,
        Err(error) => {
            return emit_failure("parse_args", 64, "usage", error, None);
        }
    };
    let text = match std::fs::read_to_string(&args.config) {
        Ok(text) => text,
        Err(error) => {
            return emit_failure(
                "read_config",
                66,
                "io",
                format!("failed reading local TLS bootstrap config: {error}"),
                Some(&args.config),
            );
        }
    };
    let config = if args
        .config
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("json")
    {
        RuntimeTlsBootstrapConfig::from_json_str(&text)
    } else {
        RuntimeTlsBootstrapConfig::from_toml_str(&text)
    };
    let config = match config {
        Ok(config) => config,
        Err(error) => {
            return emit_failure(
                "parse_config",
                64,
                local_tls_error_kind(&error),
                error.to_string(),
                Some(&args.config),
            );
        }
    };
    let result = execute(&args, &config).await;
    match result {
        Ok(outcome) => match serde_json::to_string_pretty(&outcome) {
            Ok(json) => {
                println!("{json}");
                ExitCode::SUCCESS
            }
            Err(error) => emit_failure(
                "encode_outcome",
                70,
                "encoding",
                format!("failed encoding local TLS bootstrap outcome: {error}"),
                Some(&args.config),
            ),
        },
        Err(error) => emit_failure(
            args.operation.stage(),
            75,
            local_tls_error_kind(&error),
            error.to_string(),
            Some(&args.config),
        ),
    }
}

async fn execute(
    args: &Args,
    config: &RuntimeTlsBootstrapConfig,
) -> Result<CliOutcome, LocalTlsError> {
    match args.operation {
        Operation::Bootstrap => bootstrap_runtime_tls(config)
            .await
            .map(CliOutcome::Bootstrap),
        Operation::TrustInstall => {
            require_consent(args)?;
            validate_supported_localhost_identity(config)?;
            let outcome = bootstrap_runtime_tls(config).await?;
            let certificate = outcome.public_certificate_path.as_ref().ok_or_else(|| {
                LocalTlsError::Policy("local TLS public certificate is required".to_owned())
            })?;
            let mut trust = MacOsTrustTransaction::new(config, args)?;
            let status = trust.install_current(certificate)?;
            Ok(CliOutcome::Trust(TrustOutcome::new(
                "trust_install",
                status,
                certificate,
            )?))
        }
        Operation::TrustVerify => {
            validate_supported_localhost_identity(config)?;
            let outcome = bootstrap_runtime_tls(config).await?;
            let certificate = outcome.public_certificate_path.as_ref().ok_or_else(|| {
                LocalTlsError::Policy("local TLS public certificate is required".to_owned())
            })?;
            let trust = MacOsTrustTransaction::new(config, args)?;
            trust.verify(certificate)?;
            Ok(CliOutcome::Trust(TrustOutcome::new(
                "trust_verify",
                "trusted",
                certificate,
            )?))
        }
        Operation::Reissue => {
            require_consent(args)?;
            validate_supported_localhost_identity(config)?;
            let old_sha = current_local_certificate_sha256(config)?.ok_or_else(|| {
                LocalTlsError::Policy(
                    "reissue requires an existing committed local identity".to_owned(),
                )
            })?;
            let mut trust = MacOsTrustTransaction::new(config, args)?;
            let outcome = reissue_runtime_tls_with_trust(config, &mut trust).await?;
            let cleanup_pending_certificate_sha256 = if outcome.manifest_durable {
                trust
                    .remove_old_digest_if_owned(&old_sha)
                    .err()
                    .map(|_| old_sha)
            } else {
                Some(old_sha)
            };
            let certificate = outcome.public_certificate_path.as_ref().ok_or_else(|| {
                LocalTlsError::Policy(
                    "reissued local TLS public certificate is required".to_owned(),
                )
            })?;
            Ok(CliOutcome::Trust(TrustOutcome::new_with_cleanup(
                "reissue",
                if cleanup_pending_certificate_sha256.is_some() {
                    "trusted_cleanup_pending"
                } else {
                    "trusted"
                },
                certificate,
                cleanup_pending_certificate_sha256,
            )?))
        }
        Operation::TrustRemove => {
            require_consent(args)?;
            let digest = normalize_sha256(&match args.certificate_sha256.as_ref() {
                Some(digest) => normalize_sha256(digest)?,
                None => current_local_certificate_sha256(config)?.ok_or_else(|| {
                    LocalTlsError::Policy("no committed local identity is available".to_owned())
                })?,
            })?;
            let trust = MacOsTrustTransaction::new(config, args)?;
            trust.remove_digest_if_owned(&digest)?;
            Ok(CliOutcome::Trust(TrustOutcome {
                schema: TRUST_OUTCOME_SCHEMA,
                operation: "trust_remove",
                status: "removed",
                certificate_sha256: digest,
                cleanup_pending_certificate_sha256: None,
            }))
        }
    }
}

#[derive(Serialize)]
#[serde(untagged)]
enum CliOutcome {
    Bootstrap(adl_runtime::local_tls::RuntimeTlsBootstrapOutcome),
    Trust(TrustOutcome),
}

#[derive(Serialize)]
struct TrustOutcome {
    schema: &'static str,
    operation: &'static str,
    status: &'static str,
    certificate_sha256: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    cleanup_pending_certificate_sha256: Option<String>,
}

impl TrustOutcome {
    fn new(
        operation: &'static str,
        status: &'static str,
        certificate: &Path,
    ) -> Result<Self, LocalTlsError> {
        Self::new_with_cleanup(operation, status, certificate, None)
    }

    fn new_with_cleanup(
        operation: &'static str,
        status: &'static str,
        certificate: &Path,
        cleanup_pending_certificate_sha256: Option<String>,
    ) -> Result<Self, LocalTlsError> {
        Ok(Self {
            schema: TRUST_OUTCOME_SCHEMA,
            operation,
            status,
            certificate_sha256: certificate_sha256(certificate)?,
            cleanup_pending_certificate_sha256,
        })
    }
}

#[derive(Clone, Copy)]
enum Operation {
    Bootstrap,
    TrustInstall,
    TrustVerify,
    Reissue,
    TrustRemove,
}

impl Operation {
    fn stage(self) -> &'static str {
        match self {
            Self::Bootstrap => "bootstrap",
            Self::TrustInstall => "trust_install",
            Self::TrustVerify => "trust_verify",
            Self::Reissue => "reissue",
            Self::TrustRemove => "trust_remove",
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
struct TrustReceipt {
    schema: String,
    platform: String,
    certificate_sha256: String,
    trust_store_sha256: String,
}

struct MacOsTrustTransaction {
    trust_store: PathBuf,
    receipt_root: PathBuf,
    installed_candidate: Option<String>,
}

impl MacOsTrustTransaction {
    fn new(config: &RuntimeTlsBootstrapConfig, args: &Args) -> Result<Self, LocalTlsError> {
        if std::env::consts::OS != "macos" {
            return Err(LocalTlsError::Trust(format!(
                "native {} trust is blocked: issue 5800 implements and proves the macOS user-keychain path only",
                std::env::consts::OS
            )));
        }
        let trust_store = args.trust_store.clone().ok_or_else(|| {
            LocalTlsError::Policy("--trust-store is required for host trust operations".to_owned())
        })?;
        if !trust_store.is_absolute() || !trust_store.is_file() {
            return Err(LocalTlsError::Policy(
                "--trust-store must be an absolute existing macOS user keychain".to_owned(),
            ));
        }
        let state_root = config.state_root.as_ref().ok_or_else(|| {
            LocalTlsError::Policy("state_root is required for host trust operations".to_owned())
        })?;
        let tls_dir = config.tls_dir.as_ref().ok_or_else(|| {
            LocalTlsError::Policy("tls_dir is required for host trust operations".to_owned())
        })?;
        Ok(Self {
            trust_store,
            receipt_root: state_root.join(tls_dir).join(TRUST_RECEIPTS_DIR),
            installed_candidate: None,
        })
    }

    fn install_current(&mut self, certificate: &Path) -> Result<&'static str, LocalTlsError> {
        if self.verify(certificate).is_ok() {
            let digest = certificate_sha256(certificate)?;
            return if self.authorize_removal(&digest).is_ok() {
                Ok("already_trusted_owned")
            } else {
                Ok("already_trusted_external")
            };
        }
        self.install_owned(certificate)?;
        Ok("trusted")
    }

    fn install_owned(&mut self, certificate: &Path) -> Result<(), LocalTlsError> {
        let digest = certificate_sha256(certificate)?;
        if self.keychain_contains_digest(&digest)? {
            self.authorize_removal(&digest).map_err(|_| {
                LocalTlsError::Trust(
                    "the candidate certificate already exists in the selected keychain without an issue-created receipt; refusing to modify unknown trust"
                        .to_owned(),
                )
            })?;
            if self.verify(certificate).is_ok() {
                self.installed_candidate = Some(digest);
                return Ok(());
            }
        } else {
            self.write_receipt(&digest)?;
        }
        if let Err(error) = run_security([
            "add-trusted-cert",
            "-r",
            "trustRoot",
            "-p",
            "ssl",
            "-k",
            path_text(&self.trust_store)?,
            path_text(certificate)?,
        ]) {
            if self
                .keychain_contains_digest(&digest)
                .is_ok_and(|present| !present)
            {
                let _ = self.remove_receipt(&digest);
            }
            return Err(error);
        }
        if let Err(error) = self.verify(certificate) {
            let rollback = self.delete_exact_digest(&digest);
            return match rollback {
                Ok(()) => {
                    self.remove_receipt(&digest)?;
                    Err(error)
                }
                Err(rollback_error) => Err(LocalTlsError::Trust(format!(
                    "candidate trust verification failed ({error}); exact trust rollback also failed ({rollback_error})"
                ))),
            };
        }
        self.installed_candidate = Some(digest);
        Ok(())
    }

    fn verify(&self, certificate: &Path) -> Result<(), LocalTlsError> {
        run_security([
            "verify-cert",
            "-c",
            path_text(certificate)?,
            "-p",
            "ssl",
            "-s",
            "localhost",
            "-k",
            path_text(&self.trust_store)?,
        ])
    }

    fn remove_digest_if_owned(&self, digest: &str) -> Result<(), LocalTlsError> {
        let digest = normalize_sha256(digest)?;
        let receipt_path = self.receipt_path(&digest);
        self.authorize_removal(&digest)?;
        if self.keychain_contains_digest(&digest)? {
            self.delete_exact_digest(&digest)?;
        }
        self.remove_receipt_path(&receipt_path)
    }

    fn authorize_removal(&self, digest: &str) -> Result<TrustReceipt, LocalTlsError> {
        let digest = normalize_sha256(digest)?;
        let bytes = fs::read(self.receipt_path(&digest)).map_err(|_| {
            LocalTlsError::Trust(
                "no issue-created trust receipt matches the requested certificate; refusing to delete unknown trust"
                    .to_owned(),
            )
        })?;
        let receipt: TrustReceipt = serde_json::from_slice(&bytes)
            .map_err(|error| LocalTlsError::Trust(format!("invalid trust receipt: {error}")))?;
        if receipt.schema != TRUST_RECEIPT_SCHEMA
            || receipt.platform != "macos"
            || receipt.certificate_sha256 != digest
            || receipt.trust_store_sha256 != path_sha256(&self.trust_store)
        {
            return Err(LocalTlsError::Trust(
                "trust receipt does not authorize removal from the selected keychain".to_owned(),
            ));
        }
        Ok(receipt)
    }

    fn remove_old_digest_if_owned(&self, digest: &str) -> Result<(), LocalTlsError> {
        let digest = normalize_sha256(digest)?;
        if self.receipt_path(&digest).is_file() {
            self.remove_digest_if_owned(&digest)
        } else {
            Ok(())
        }
    }

    fn delete_exact_digest(&self, digest: &str) -> Result<(), LocalTlsError> {
        run_security(delete_certificate_args(
            digest,
            path_text(&self.trust_store)?,
        ))?;
        if self.keychain_contains_digest(digest)? {
            return Err(LocalTlsError::Trust(
                "security did not remove the exact certificate and its trust settings".to_owned(),
            ));
        }
        Ok(())
    }

    fn keychain_contains_digest(&self, digest: &str) -> Result<bool, LocalTlsError> {
        let output = Command::new("/usr/bin/security")
            .args(["find-certificate", "-a", "-Z"])
            .arg(&self.trust_store)
            .output()
            .map_err(|error| LocalTlsError::Trust(format!("launch security failed: {error}")))?;
        if !output.status.success() {
            return Err(LocalTlsError::Trust(format!(
                "security find-certificate failed with status {}",
                output.status
            )));
        }
        let stdout = String::from_utf8_lossy(&output.stdout).to_ascii_uppercase();
        Ok(stdout.lines().any(|line| {
            line.trim()
                .strip_prefix("SHA-256 HASH: ")
                .is_some_and(|observed| observed.trim() == digest)
        }))
    }

    fn write_receipt(&self, digest: &str) -> Result<(), LocalTlsError> {
        fs::create_dir_all(&self.receipt_root)
            .map_err(|error| LocalTlsError::Io(error.to_string()))?;
        if self.receipt_path(digest).exists() {
            self.authorize_removal(digest)?;
            return sync_directory(&self.receipt_root);
        }
        let receipt = TrustReceipt {
            schema: TRUST_RECEIPT_SCHEMA.to_owned(),
            platform: "macos".to_owned(),
            certificate_sha256: digest.to_owned(),
            trust_store_sha256: path_sha256(&self.trust_store),
        };
        let target = self.receipt_path(digest);
        let temporary = self
            .receipt_root
            .join(format!(".{digest}.{}.json.tmp", std::process::id()));
        let mut options = OpenOptions::new();
        options.create_new(true).write(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options
            .open(&temporary)
            .map_err(|error| LocalTlsError::Io(error.to_string()))?;
        file.write_all(
            &serde_json::to_vec_pretty(&receipt)
                .map_err(|error| LocalTlsError::Io(error.to_string()))?,
        )
        .and_then(|()| file.sync_all())
        .map_err(|error| LocalTlsError::Io(error.to_string()))?;
        fs::rename(&temporary, &target).map_err(|error| {
            let _ = fs::remove_file(&temporary);
            LocalTlsError::Io(error.to_string())
        })?;
        sync_directory(&self.receipt_root)
    }

    fn remove_receipt(&self, digest: &str) -> Result<(), LocalTlsError> {
        self.remove_receipt_path(&self.receipt_path(digest))
    }

    fn remove_receipt_path(&self, path: &Path) -> Result<(), LocalTlsError> {
        match fs::remove_file(path) {
            Ok(()) => sync_directory(&self.receipt_root),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(error) => Err(LocalTlsError::Io(error.to_string())),
        }
    }

    fn receipt_path(&self, digest: &str) -> PathBuf {
        self.receipt_root.join(format!("{digest}.json"))
    }
}

fn sync_directory(path: &Path) -> Result<(), LocalTlsError> {
    fs::File::open(path)
        .and_then(|directory| directory.sync_all())
        .map_err(|error| LocalTlsError::Io(error.to_string()))
}

fn delete_certificate_args<'a>(digest: &'a str, trust_store: &'a str) -> [&'a str; 5] {
    ["delete-certificate", "-t", "-Z", digest, trust_store]
}

impl LocalTlsTrustTransaction for MacOsTrustTransaction {
    fn install_and_verify(&mut self, candidate_certificate: &Path) -> Result<(), LocalTlsError> {
        self.install_owned(candidate_certificate)
    }

    fn rollback_candidate(&mut self, _candidate_certificate: &Path) -> Result<(), LocalTlsError> {
        let digest = self.installed_candidate.take().ok_or_else(|| {
            LocalTlsError::Trust("candidate trust rollback had no owned receipt".to_owned())
        })?;
        self.remove_digest_if_owned(&digest)
    }
}

fn run_security<const N: usize>(args: [&str; N]) -> Result<(), LocalTlsError> {
    let output = Command::new("/usr/bin/security")
        .args(args)
        .output()
        .map_err(|error| LocalTlsError::Trust(format!("launch security failed: {error}")))?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let detail = stderr.trim();
        Err(LocalTlsError::Trust(format!(
            "security command failed with status {}{}",
            output.status,
            if detail.is_empty() {
                String::new()
            } else {
                format!(": {detail}")
            }
        )))
    }
}

fn validate_supported_localhost_identity(
    config: &RuntimeTlsBootstrapConfig,
) -> Result<(), LocalTlsError> {
    if config.mode != RuntimeTlsBootstrapMode::LocalSelfSigned {
        return Err(LocalTlsError::Policy(
            "host trust operations require local_self_signed TLS".to_owned(),
        ));
    }
    let dns = config
        .dns_names
        .iter()
        .map(|name| name.to_ascii_lowercase())
        .collect::<BTreeSet<_>>();
    let ips = config.ip_addresses.iter().copied().collect::<BTreeSet<_>>();
    if dns != BTreeSet::from(["localhost".to_owned()])
        || ips
            != BTreeSet::from([
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ])
    {
        return Err(LocalTlsError::Policy(
            "the supported browser-trusted identity requires exactly localhost, 127.0.0.1, and ::1 SANs"
                .to_owned(),
        ));
    }
    Ok(())
}

fn require_consent(args: &Args) -> Result<(), LocalTlsError> {
    if args.consent_host_trust {
        Ok(())
    } else {
        Err(LocalTlsError::Policy(
            "host trust mutation requires --consent-host-trust".to_owned(),
        ))
    }
}

fn certificate_sha256(path: &Path) -> Result<String, LocalTlsError> {
    certificate_fingerprint_sha256(path)
}

fn path_sha256(path: &Path) -> String {
    hex::encode_upper(Sha256::digest(
        path.as_os_str().to_string_lossy().as_bytes(),
    ))
}

fn normalize_sha256(digest: &str) -> Result<String, LocalTlsError> {
    let normalized = digest.trim().to_ascii_uppercase();
    if normalized.len() == 64 && normalized.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        Ok(normalized)
    } else {
        Err(LocalTlsError::Policy(
            "certificate SHA-256 must be exactly 64 hexadecimal characters".to_owned(),
        ))
    }
}

fn path_text(path: &Path) -> Result<&str, LocalTlsError> {
    path.to_str()
        .ok_or_else(|| LocalTlsError::Policy("host trust path must be UTF-8".to_owned()))
}

#[derive(Serialize)]
struct BootstrapFailure {
    schema: &'static str,
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    config_path: Option<String>,
}

fn emit_failure(
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: impl Into<String>,
    config_path: Option<&PathBuf>,
) -> ExitCode {
    match failure_json(stage, exit_code, error_kind, message, config_path) {
        Ok(json) => eprintln!("{json}"),
        Err(error) => eprintln!(
            "{{\"schema\":\"{FAILURE_SCHEMA}\",\"stage\":\"encode_failure\",\"exit_code\":70,\"error_kind\":\"encoding\",\"message\":\"failed encoding local TLS bootstrap failure: {error}\"}}"
        ),
    }
    ExitCode::from(exit_code)
}

fn failure_json(
    stage: &'static str,
    exit_code: u8,
    error_kind: &'static str,
    message: impl Into<String>,
    config_path: Option<&PathBuf>,
) -> Result<String, serde_json::Error> {
    serde_json::to_string_pretty(&BootstrapFailure {
        schema: FAILURE_SCHEMA,
        stage,
        exit_code,
        error_kind,
        message: message.into(),
        config_path: config_path.map(|path| path.to_string_lossy().into_owned()),
    })
}

fn local_tls_error_kind(error: &LocalTlsError) -> &'static str {
    match error {
        LocalTlsError::UnsupportedSchema(_) => "unsupported_schema",
        LocalTlsError::Config(_) => "config",
        LocalTlsError::Policy(_) => "policy",
        LocalTlsError::LockBusy => "lock_busy",
        LocalTlsError::Io(_) => "io",
        LocalTlsError::Generate(_) => "generate",
        LocalTlsError::Rustls(_) => "rustls",
        LocalTlsError::Trust(_) => "trust",
    }
}

struct Args {
    config: PathBuf,
    operation: Operation,
    consent_host_trust: bool,
    trust_store: Option<PathBuf>,
    certificate_sha256: Option<String>,
}

impl Args {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut config = None;
        let mut operation = Operation::Bootstrap;
        let mut consent_host_trust = false;
        let mut trust_store = None;
        let mut certificate_sha256 = None;
        let mut iter = args.into_iter();
        while let Some(arg) = iter.next() {
            match arg.as_str() {
                "--config" => {
                    let value = iter
                        .next()
                        .ok_or_else(|| "--config requires a path".to_owned())?;
                    config = Some(PathBuf::from(value));
                }
                "--operation" => {
                    operation = match iter
                        .next()
                        .ok_or_else(|| "--operation requires a value".to_owned())?
                        .as_str()
                    {
                        "bootstrap" => Operation::Bootstrap,
                        "trust-install" => Operation::TrustInstall,
                        "trust-verify" => Operation::TrustVerify,
                        "reissue" => Operation::Reissue,
                        "trust-remove" => Operation::TrustRemove,
                        value => return Err(format!("unsupported --operation: {value}")),
                    };
                }
                "--consent-host-trust" => consent_host_trust = true,
                "--trust-store" => {
                    trust_store = Some(PathBuf::from(
                        iter.next()
                            .ok_or_else(|| "--trust-store requires a path".to_owned())?,
                    ));
                }
                "--certificate-sha256" => {
                    certificate_sha256 = Some(
                        iter.next()
                            .ok_or_else(|| "--certificate-sha256 requires a digest".to_owned())?,
                    );
                }
                "--help" | "-h" => {
                    return Err(
                        "Usage: adl-runtime-local-tls-bootstrap --config <config.toml|config.json> [--operation bootstrap|trust-install|trust-verify|reissue|trust-remove] [--consent-host-trust] [--trust-store <absolute-user-keychain>] [--certificate-sha256 <digest>]".to_owned(),
                    );
                }
                _ => return Err(format!("unknown argument: {arg}")),
            }
        }
        Ok(Self {
            config: config.ok_or_else(|| "--config is required".to_owned())?,
            operation,
            consent_host_trust,
            trust_store,
            certificate_sha256,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DIGEST: &str = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA";

    #[test]
    fn failure_payload_is_machine_readable_json() {
        let json = failure_json(
            "bootstrap",
            75,
            "policy",
            "local TLS rejected test",
            Some(&PathBuf::from("config.toml")),
        )
        .unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed["schema"], FAILURE_SCHEMA);
        assert_eq!(parsed["stage"], "bootstrap");
        assert_eq!(parsed["exit_code"], 75);
        assert_eq!(parsed["error_kind"], "policy");
        assert_eq!(parsed["config_path"], "config.toml");
    }

    #[test]
    fn trust_operations_parse_and_report_distinct_stages() {
        for (name, stage) in [
            ("bootstrap", "bootstrap"),
            ("trust-install", "trust_install"),
            ("trust-verify", "trust_verify"),
            ("reissue", "reissue"),
            ("trust-remove", "trust_remove"),
        ] {
            let args = Args::parse(vec![
                "--config".to_owned(),
                "config.toml".to_owned(),
                "--operation".to_owned(),
                name.to_owned(),
            ])
            .unwrap();
            assert_eq!(args.operation.stage(), stage);
        }
    }

    #[test]
    fn host_trust_mutation_requires_explicit_consent() {
        let denied = Args::parse(vec![
            "--config".to_owned(),
            "config.toml".to_owned(),
            "--operation".to_owned(),
            "trust-install".to_owned(),
        ])
        .unwrap();
        assert!(require_consent(&denied).is_err());

        let allowed = Args::parse(vec![
            "--config".to_owned(),
            "config.toml".to_owned(),
            "--operation".to_owned(),
            "trust-install".to_owned(),
            "--consent-host-trust".to_owned(),
        ])
        .unwrap();
        assert!(require_consent(&allowed).is_ok());
    }

    #[test]
    fn trusted_identity_requires_exact_localhost_sans() {
        let temp = tempfile::tempdir().unwrap();
        let config = localhost_config(temp.path().to_path_buf());
        assert!(validate_supported_localhost_identity(&config).is_ok());

        let mut missing_ipv6 = config.clone();
        missing_ipv6.ip_addresses.pop();
        assert!(validate_supported_localhost_identity(&missing_ipv6).is_err());

        let mut widened = config;
        widened.dns_names.push("runtime.local".to_owned());
        assert!(validate_supported_localhost_identity(&widened).is_err());
    }

    #[test]
    fn sha256_normalization_rejects_non_exact_digests() {
        assert_eq!(
            normalize_sha256(&DIGEST.to_ascii_lowercase()).unwrap(),
            DIGEST
        );
        assert!(normalize_sha256("AA").is_err());
        assert!(normalize_sha256(&"G".repeat(64)).is_err());
    }

    #[test]
    fn deletion_targets_exact_fingerprint_and_removes_trust_settings() {
        assert_eq!(
            delete_certificate_args(DIGEST, "/Volumes/FastWork/test.keychain-db"),
            [
                "delete-certificate",
                "-t",
                "-Z",
                DIGEST,
                "/Volumes/FastWork/test.keychain-db",
            ]
        );
    }

    #[tokio::test]
    async fn committed_reissue_can_report_retryable_old_trust_cleanup() {
        let temp = tempfile::tempdir().unwrap();
        let outcome = bootstrap_runtime_tls(&localhost_config(temp.path().to_path_buf()))
            .await
            .unwrap();
        let certificate = outcome.public_certificate_path.unwrap();
        let outcome = TrustOutcome::new_with_cleanup(
            "reissue",
            "trusted_cleanup_pending",
            &certificate,
            Some(DIGEST.to_owned()),
        )
        .unwrap();
        let value = serde_json::to_value(outcome).unwrap();
        assert_eq!(value["status"], "trusted_cleanup_pending");
        assert_eq!(value["cleanup_pending_certificate_sha256"], DIGEST);
    }

    #[test]
    fn receipt_authorization_rejects_absent_and_malformed_receipts() {
        let temp = tempfile::tempdir().unwrap();
        let transaction = fake_transaction(temp.path());
        assert!(transaction.authorize_removal(DIGEST).is_err());

        fs::create_dir_all(&transaction.receipt_root).unwrap();
        fs::write(transaction.receipt_path(DIGEST), b"not json").unwrap();
        assert!(transaction.authorize_removal(DIGEST).is_err());
    }

    #[test]
    fn receipt_authorization_rejects_wrong_certificate_or_keychain() {
        let temp = tempfile::tempdir().unwrap();
        let transaction = fake_transaction(temp.path());
        fs::create_dir_all(&transaction.receipt_root).unwrap();
        write_test_receipt(
            &transaction,
            DIGEST,
            &"B".repeat(64),
            &path_sha256(&transaction.trust_store),
        );
        assert!(transaction.authorize_removal(DIGEST).is_err());

        write_test_receipt(&transaction, DIGEST, DIGEST, &"C".repeat(64));
        assert!(transaction.authorize_removal(DIGEST).is_err());
    }

    #[test]
    fn receipt_authorization_accepts_exact_owned_identity_without_keychain_mutation() {
        let temp = tempfile::tempdir().unwrap();
        let transaction = fake_transaction(temp.path());
        transaction.write_receipt(DIGEST).unwrap();

        let receipt = transaction.authorize_removal(DIGEST).unwrap();

        assert_eq!(receipt.certificate_sha256, DIGEST);
        assert_eq!(
            receipt.trust_store_sha256,
            path_sha256(&transaction.trust_store)
        );
    }

    #[test]
    fn ownership_receipt_is_idempotent_and_leaves_no_temporary_file() {
        let temp = tempfile::tempdir().unwrap();
        let transaction = fake_transaction(temp.path());

        transaction.write_receipt(DIGEST).unwrap();
        transaction.write_receipt(DIGEST).unwrap();

        assert!(transaction.receipt_path(DIGEST).is_file());
        assert_eq!(fs::read_dir(&transaction.receipt_root).unwrap().count(), 1);
        transaction.authorize_removal(DIGEST).unwrap();
    }

    fn localhost_config(state_root: PathBuf) -> RuntimeTlsBootstrapConfig {
        RuntimeTlsBootstrapConfig {
            schema: adl_runtime::local_tls::LOCAL_TLS_BOOTSTRAP_SCHEMA.to_owned(),
            mode: RuntimeTlsBootstrapMode::LocalSelfSigned,
            state_root: Some(state_root),
            tls_dir: Some(PathBuf::from("runtime-tls")),
            certificate_chain_path: PathBuf::from("localhost-chain.pem"),
            public_certificate_path: Some(PathBuf::from("localhost-public.pem")),
            private_key_path: PathBuf::from("localhost-key.pem"),
            dns_names: vec!["localhost".to_owned()],
            ip_addresses: vec![
                IpAddr::V4(Ipv4Addr::LOCALHOST),
                IpAddr::V6(Ipv6Addr::LOCALHOST),
            ],
            replace: false,
        }
    }

    fn fake_transaction(root: &Path) -> MacOsTrustTransaction {
        let trust_store = root.join("login.keychain-db");
        fs::write(&trust_store, b"test keychain placeholder").unwrap();
        MacOsTrustTransaction {
            trust_store,
            receipt_root: root.join(TRUST_RECEIPTS_DIR),
            installed_candidate: None,
        }
    }

    fn write_test_receipt(
        transaction: &MacOsTrustTransaction,
        path_digest: &str,
        certificate_digest: &str,
        trust_store_digest: &str,
    ) {
        let receipt = TrustReceipt {
            schema: TRUST_RECEIPT_SCHEMA.to_owned(),
            platform: "macos".to_owned(),
            certificate_sha256: certificate_digest.to_owned(),
            trust_store_sha256: trust_store_digest.to_owned(),
        };
        fs::write(
            transaction.receipt_path(path_digest),
            serde_json::to_vec_pretty(&receipt).unwrap(),
        )
        .unwrap();
    }
}
