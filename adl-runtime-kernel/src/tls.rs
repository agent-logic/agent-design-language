use std::{
    fmt,
    io::BufReader,
    path::{Path, PathBuf},
    sync::Arc,
};

use rustls::{
    client::{danger::ServerCertVerifier, WebPkiServerVerifier},
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime},
    server::WebPkiClientVerifier,
    ClientConfig, RootCertStore, ServerConfig,
};

pub const HTTP_ALPN_PROTOCOLS: &[&[u8]] = &[b"h2", b"http/1.1"];

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TlsIdentityPaths {
    pub certificate_chain_path: PathBuf,
    pub private_key_path: PathBuf,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TlsServerValidation {
    pub trust_roots_path: PathBuf,
    pub server_name: String,
}

#[derive(Debug)]
pub enum TlsConfigError {
    Read {
        path: PathBuf,
        source: std::io::Error,
    },
    InvalidCertificateChain(String),
    InvalidPrivateKey(String),
    InvalidTrustRoots(String),
    Configuration(String),
}

impl fmt::Display for TlsConfigError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Read { path, source } => {
                write!(formatter, "read TLS material {}: {source}", path.display())
            }
            Self::InvalidCertificateChain(message) => {
                write!(formatter, "invalid TLS certificate chain: {message}")
            }
            Self::InvalidPrivateKey(message) => {
                write!(formatter, "invalid TLS private key: {message}")
            }
            Self::InvalidTrustRoots(message) => {
                write!(formatter, "invalid TLS trust roots: {message}")
            }
            Self::Configuration(message) => {
                write!(formatter, "TLS configuration failed: {message}")
            }
        }
    }
}

impl std::error::Error for TlsConfigError {}

pub struct TlsIdentity {
    certificate_chain: Vec<CertificateDer<'static>>,
    private_key: PrivateKeyDer<'static>,
}

impl TlsIdentity {
    pub fn from_der(
        certificate_chain: Vec<CertificateDer<'static>>,
        private_key: PrivateKeyDer<'static>,
    ) -> Result<Self, TlsConfigError> {
        if certificate_chain.is_empty() {
            return Err(TlsConfigError::InvalidCertificateChain(
                "expected at least one certificate".to_owned(),
            ));
        }
        Ok(Self {
            certificate_chain,
            private_key,
        })
    }

    pub fn certificate_chain(&self) -> &[CertificateDer<'static>] {
        &self.certificate_chain
    }

    fn into_parts(self) -> (Vec<CertificateDer<'static>>, PrivateKeyDer<'static>) {
        (self.certificate_chain, self.private_key)
    }
}

pub async fn load_identity(paths: &TlsIdentityPaths) -> Result<TlsIdentity, TlsConfigError> {
    let certificate_bytes = read(&paths.certificate_chain_path).await?;
    let private_key_bytes = read(&paths.private_key_path).await?;
    parse_identity(&certificate_bytes, &private_key_bytes)
}

pub fn parse_identity(
    certificate_pem: &[u8],
    private_key_pem: &[u8],
) -> Result<TlsIdentity, TlsConfigError> {
    let mut certificate_reader = BufReader::new(certificate_pem);
    let certificate_chain = rustls_pemfile::certs(&mut certificate_reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| TlsConfigError::InvalidCertificateChain(error.to_string()))?;
    let mut private_key_reader = BufReader::new(private_key_pem);
    let private_key = rustls_pemfile::private_key(&mut private_key_reader)
        .map_err(|error| TlsConfigError::InvalidPrivateKey(error.to_string()))?
        .ok_or_else(|| TlsConfigError::InvalidPrivateKey("missing private key".to_owned()))?;
    TlsIdentity::from_der(certificate_chain, private_key)
}

pub async fn load_trust_roots(path: impl AsRef<Path>) -> Result<RootCertStore, TlsConfigError> {
    let bytes = read(path.as_ref()).await?;
    parse_trust_roots(&bytes)
}

pub fn parse_trust_roots(certificate_pem: &[u8]) -> Result<RootCertStore, TlsConfigError> {
    let mut reader = BufReader::new(certificate_pem);
    let certificates = rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|error| TlsConfigError::InvalidTrustRoots(error.to_string()))?;
    trust_roots_from_der(&certificates)
}

pub fn trust_roots_from_der(
    certificates: &[CertificateDer<'static>],
) -> Result<RootCertStore, TlsConfigError> {
    if certificates.is_empty() {
        return Err(TlsConfigError::InvalidTrustRoots(
            "expected at least one CA certificate".to_owned(),
        ));
    }
    let mut roots = RootCertStore::empty();
    for (index, certificate) in certificates.iter().enumerate() {
        validate_trust_anchor(certificate, index)?;
        roots
            .add(certificate.clone())
            .map_err(|error| TlsConfigError::InvalidTrustRoots(error.to_string()))?;
    }
    Ok(roots)
}

fn validate_trust_anchor(
    certificate: &CertificateDer<'_>,
    index: usize,
) -> Result<(), TlsConfigError> {
    let (remaining, parsed) =
        x509_parser::parse_x509_certificate(certificate.as_ref()).map_err(|error| {
            TlsConfigError::InvalidTrustRoots(format!(
                "certificate {} is not valid X.509: {error}",
                index + 1
            ))
        })?;
    if !remaining.is_empty() {
        return Err(TlsConfigError::InvalidTrustRoots(format!(
            "certificate {} contains trailing data",
            index + 1
        )));
    }

    let basic_constraints = parsed.basic_constraints().map_err(|error| {
        TlsConfigError::InvalidTrustRoots(format!(
            "certificate {} has invalid Basic Constraints: {error}",
            index + 1
        ))
    })?;
    if !basic_constraints.is_some_and(|extension| extension.value.ca) {
        return Err(TlsConfigError::InvalidTrustRoots(format!(
            "certificate {} is not a CA certificate",
            index + 1
        )));
    }

    let key_usage = parsed.key_usage().map_err(|error| {
        TlsConfigError::InvalidTrustRoots(format!(
            "certificate {} has invalid Key Usage: {error}",
            index + 1
        ))
    })?;
    if key_usage.is_some_and(|extension| !extension.value.key_cert_sign()) {
        return Err(TlsConfigError::InvalidTrustRoots(format!(
            "certificate {} Key Usage does not permit certificate signing",
            index + 1
        )));
    }

    Ok(())
}

pub async fn load_axum_server_tls(
    paths: &TlsIdentityPaths,
    validation: &TlsServerValidation,
) -> Result<axum_server::tls_rustls::RustlsConfig, TlsConfigError> {
    let identity = load_identity(paths).await?;
    let trust_roots = load_trust_roots(&validation.trust_roots_path).await?;
    verify_server_identity(&identity, trust_roots, &validation.server_name)?;
    let config = build_server_config(identity, HTTP_ALPN_PROTOCOLS)?;
    Ok(axum_server::tls_rustls::RustlsConfig::from_config(config))
}

pub fn verify_server_identity(
    identity: &TlsIdentity,
    trust_roots: RootCertStore,
    server_name: &str,
) -> Result<(), TlsConfigError> {
    verify_server_identity_at(identity, trust_roots, server_name, UnixTime::now())
}

fn verify_server_identity_at(
    identity: &TlsIdentity,
    trust_roots: RootCertStore,
    server_name: &str,
    now: UnixTime,
) -> Result<(), TlsConfigError> {
    let server_name = ServerName::try_from(server_name.to_owned())
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    let verifier =
        WebPkiServerVerifier::builder_with_provider(Arc::new(trust_roots), crypto_provider())
            .build()
            .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    verifier
        .verify_server_cert(
            &identity.certificate_chain[0],
            &identity.certificate_chain[1..],
            &server_name,
            &[],
            now,
        )
        .map_err(|error| TlsConfigError::InvalidCertificateChain(error.to_string()))?;
    Ok(())
}

pub fn build_server_config(
    identity: TlsIdentity,
    alpn_protocols: &[&[u8]],
) -> Result<Arc<ServerConfig>, TlsConfigError> {
    let (certificate_chain, private_key) = identity.into_parts();
    let mut config = ServerConfig::builder_with_provider(crypto_provider())
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?
        .with_no_client_auth()
        .with_single_cert(certificate_chain, private_key)
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    config.alpn_protocols = alpn_protocols
        .iter()
        .map(|protocol| protocol.to_vec())
        .collect();
    Ok(Arc::new(config))
}

pub fn build_mutual_tls_server_config(
    identity: TlsIdentity,
    client_roots: RootCertStore,
    alpn_protocols: &[&[u8]],
) -> Result<Arc<ServerConfig>, TlsConfigError> {
    if client_roots.is_empty() {
        return Err(TlsConfigError::InvalidTrustRoots(
            "client authentication requires CA roots".to_owned(),
        ));
    }
    let provider = crypto_provider();
    let verifier =
        WebPkiClientVerifier::builder_with_provider(Arc::new(client_roots), provider.clone())
            .build()
            .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    let (certificate_chain, private_key) = identity.into_parts();
    let mut config = ServerConfig::builder_with_provider(provider)
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?
        .with_client_cert_verifier(verifier)
        .with_single_cert(certificate_chain, private_key)
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    config.alpn_protocols = alpn_protocols
        .iter()
        .map(|protocol| protocol.to_vec())
        .collect();
    Ok(Arc::new(config))
}

pub fn build_mutual_tls_client_config(
    identity: TlsIdentity,
    server_roots: RootCertStore,
    alpn_protocols: &[&[u8]],
) -> Result<Arc<ClientConfig>, TlsConfigError> {
    if server_roots.is_empty() {
        return Err(TlsConfigError::InvalidTrustRoots(
            "server authentication requires CA roots".to_owned(),
        ));
    }
    let (certificate_chain, private_key) = identity.into_parts();
    let mut config = ClientConfig::builder_with_provider(crypto_provider())
        .with_protocol_versions(&[&rustls::version::TLS13])
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?
        .with_root_certificates(server_roots)
        .with_client_auth_cert(certificate_chain, private_key)
        .map_err(|error| TlsConfigError::Configuration(error.to_string()))?;
    config.alpn_protocols = alpn_protocols
        .iter()
        .map(|protocol| protocol.to_vec())
        .collect();
    Ok(Arc::new(config))
}

fn crypto_provider() -> Arc<rustls::crypto::CryptoProvider> {
    Arc::new(rustls::crypto::aws_lc_rs::default_provider())
}

async fn read(path: &Path) -> Result<Vec<u8>, TlsConfigError> {
    tokio::fs::read(path)
        .await
        .map_err(|source| TlsConfigError::Read {
            path: path.to_path_buf(),
            source,
        })
}

#[cfg(test)]
mod tests {
    use rcgen::{
        date_time_ymd, BasicConstraints, CertificateParams, CertifiedIssuer,
        ExtendedKeyUsagePurpose, IsCa, KeyPair, KeyUsagePurpose,
    };

    use std::time::Duration;

    use rustls::pki_types::UnixTime;

    use super::{parse_identity, parse_trust_roots, verify_server_identity_at};

    fn identity(
        not_before: (i32, u8, u8),
        not_after: (i32, u8, u8),
        usage: ExtendedKeyUsagePurpose,
    ) -> (super::TlsIdentity, rustls::RootCertStore) {
        let mut ca_params = CertificateParams::new(["ADL test root".to_owned()]).unwrap();
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        ca_params.not_before = date_time_ymd(2020, 1, 1);
        ca_params.not_after = date_time_ymd(2040, 1, 1);
        let ca = CertifiedIssuer::self_signed(ca_params, KeyPair::generate().unwrap()).unwrap();

        let key = KeyPair::generate().unwrap();
        let mut params = CertificateParams::new(["localhost".to_owned()]).unwrap();
        params.not_before = date_time_ymd(not_before.0, not_before.1, not_before.2);
        params.not_after = date_time_ymd(not_after.0, not_after.1, not_after.2);
        params.extended_key_usages = vec![usage];
        let leaf = params.signed_by(&key, &ca).unwrap();
        (
            parse_identity(leaf.pem().as_bytes(), key.serialize_pem().as_bytes()).unwrap(),
            parse_trust_roots(ca.pem().as_bytes()).unwrap(),
        )
    }

    #[test]
    fn server_identity_validation_rejects_time_and_usage_failures() {
        let verification_time = UnixTime::since_unix_epoch(Duration::from_secs(1_893_456_000));
        let (valid, roots) = identity(
            (2020, 1, 1),
            (2036, 1, 1),
            ExtendedKeyUsagePurpose::ServerAuth,
        );
        assert!(verify_server_identity_at(&valid, roots, "localhost", verification_time).is_ok());

        for (not_before, not_after, usage) in [
            (
                (2020, 1, 1),
                (2021, 1, 1),
                ExtendedKeyUsagePurpose::ServerAuth,
            ),
            (
                (2031, 1, 1),
                (2032, 1, 1),
                ExtendedKeyUsagePurpose::ServerAuth,
            ),
            (
                (2020, 1, 1),
                (2036, 1, 1),
                ExtendedKeyUsagePurpose::ClientAuth,
            ),
        ] {
            let (identity, roots) = identity(not_before, not_after, usage);
            assert!(
                verify_server_identity_at(&identity, roots, "localhost", verification_time)
                    .is_err()
            );
        }
    }

    #[test]
    fn trust_roots_reject_the_served_leaf_certificate() {
        let mut ca_params = CertificateParams::new(["ADL test root".to_owned()]).unwrap();
        ca_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        ca_params.key_usages = vec![KeyUsagePurpose::KeyCertSign];
        let ca = CertifiedIssuer::self_signed(ca_params, KeyPair::generate().unwrap()).unwrap();

        let leaf_key = KeyPair::generate().unwrap();
        let leaf = CertificateParams::new(["localhost".to_owned()])
            .unwrap()
            .signed_by(&leaf_key, &ca)
            .unwrap();

        parse_identity(leaf.pem().as_bytes(), leaf_key.serialize_pem().as_bytes()).unwrap();
        let error = parse_trust_roots(leaf.pem().as_bytes()).unwrap_err();
        assert!(matches!(error, super::TlsConfigError::InvalidTrustRoots(_)));
        assert!(error.to_string().contains("is not a CA certificate"));
    }

    #[test]
    fn trust_roots_accept_ca_certificates_and_enforce_ca_key_usage() {
        let mut valid_params = CertificateParams::new(["ADL valid root".to_owned()]).unwrap();
        valid_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        valid_params.key_usages = vec![KeyUsagePurpose::KeyCertSign];
        let valid =
            CertifiedIssuer::self_signed(valid_params, KeyPair::generate().unwrap()).unwrap();
        assert!(parse_trust_roots(valid.pem().as_bytes()).is_ok());

        let mut invalid_params = CertificateParams::new(["ADL invalid root".to_owned()]).unwrap();
        invalid_params.is_ca = IsCa::Ca(BasicConstraints::Constrained(0));
        invalid_params.key_usages = vec![KeyUsagePurpose::DigitalSignature];
        let invalid =
            CertifiedIssuer::self_signed(invalid_params, KeyPair::generate().unwrap()).unwrap();
        let error = parse_trust_roots(invalid.pem().as_bytes()).unwrap_err();
        assert!(error
            .to_string()
            .contains("does not permit certificate signing"));
    }
}
