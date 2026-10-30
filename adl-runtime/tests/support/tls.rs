#![allow(dead_code)]

use tokio_rustls::rustls::{
    pki_types::{pem::PemObject, CertificateDer, PrivateKeyDer},
    RootCertStore,
};

const ROOT_CA: &[u8] = include_bytes!("tls-fixtures/root-ca.pem");
const INTERMEDIATE_CA: &[u8] = include_bytes!("tls-fixtures/intermediate-ca.pem");
const SERVER_CERT: &[u8] = include_bytes!("tls-fixtures/server-cert.pem");
const SERVER_KEY: &[u8] = include_bytes!("tls-fixtures/server-key.pem");
const WRONG_SAN_CERT: &[u8] = include_bytes!("tls-fixtures/wrong-san-cert.pem");
const WRONG_SAN_KEY: &[u8] = include_bytes!("tls-fixtures/wrong-san-key.pem");
const CLIENT_CERT: &[u8] = include_bytes!("tls-fixtures/client-cert.pem");
const CLIENT_KEY: &[u8] = include_bytes!("tls-fixtures/client-key.pem");
const WRONG_ROOT_CA: &[u8] = include_bytes!("tls-fixtures/wrong-root-ca.pem");
const WRONG_CLIENT_CERT: &[u8] = include_bytes!("tls-fixtures/wrong-client-cert.pem");
const WRONG_CLIENT_KEY: &[u8] = include_bytes!("tls-fixtures/wrong-client-key.pem");
const SELF_SIGNED_SERVER_CERT: &[u8] = include_bytes!("tls-fixtures/self-signed-server-cert.pem");
const SELF_SIGNED_SERVER_KEY: &[u8] = include_bytes!("tls-fixtures/self-signed-server-key.pem");

pub struct TestPki;

pub struct TestIdentity {
    pub certificate: CertificateDer<'static>,
    intermediate: Option<CertificateDer<'static>>,
    certificate_pem: &'static [u8],
    intermediate_pem: Option<&'static [u8]>,
    private_key_pem: &'static [u8],
}

impl TestPki {
    pub fn new(_name: &str) -> Self {
        Self
    }

    pub fn server(&self, _names: &[&str]) -> TestIdentity {
        identity(SERVER_CERT, Some(INTERMEDIATE_CA), SERVER_KEY)
    }

    pub fn client(&self, _names: &[&str]) -> TestIdentity {
        identity(CLIENT_CERT, None, CLIENT_KEY)
    }

    pub fn wrong_san_server(&self) -> TestIdentity {
        identity(WRONG_SAN_CERT, Some(INTERMEDIATE_CA), WRONG_SAN_KEY)
    }

    pub fn self_signed_server(&self) -> TestIdentity {
        identity(SELF_SIGNED_SERVER_CERT, None, SELF_SIGNED_SERVER_KEY)
    }

    pub fn wrong_client(&self) -> TestIdentity {
        identity(WRONG_CLIENT_CERT, None, WRONG_CLIENT_KEY)
    }

    pub fn roots(&self) -> RootCertStore {
        roots(ROOT_CA)
    }

    pub fn wrong_roots(&self) -> RootCertStore {
        roots(WRONG_ROOT_CA)
    }

    pub fn root_der(&self) -> CertificateDer<'static> {
        certificate(ROOT_CA)
    }

    pub fn root_pem(&self) -> &'static [u8] {
        ROOT_CA
    }

    pub fn wrong_root_pem(&self) -> &'static [u8] {
        WRONG_ROOT_CA
    }
}

impl TestIdentity {
    pub fn chain_der(&self) -> Vec<CertificateDer<'static>> {
        let mut chain = vec![self.certificate.clone()];
        if let Some(intermediate) = &self.intermediate {
            chain.push(intermediate.clone());
        }
        chain
    }

    pub fn leaf_only_der(&self) -> Vec<CertificateDer<'static>> {
        vec![self.certificate.clone()]
    }

    pub fn certificate_pem(&self) -> Vec<u8> {
        let mut chain = self.certificate_pem.to_vec();
        if let Some(intermediate) = self.intermediate_pem {
            chain.extend_from_slice(intermediate);
        }
        chain
    }

    pub fn leaf_only_pem(&self) -> Vec<u8> {
        self.certificate_pem.to_vec()
    }

    pub fn private_key_der(&self) -> PrivateKeyDer<'static> {
        PrivateKeyDer::from_pem_slice(self.private_key_pem).unwrap()
    }

    pub fn private_key_bytes(&self) -> Vec<u8> {
        self.private_key_der().secret_der().to_vec()
    }

    pub fn private_key_pem(&self) -> Vec<u8> {
        self.private_key_pem.to_vec()
    }
}

fn identity(
    certificate_pem: &'static [u8],
    intermediate_pem: Option<&'static [u8]>,
    private_key_pem: &'static [u8],
) -> TestIdentity {
    TestIdentity {
        certificate: certificate(certificate_pem),
        intermediate: intermediate_pem.map(certificate),
        certificate_pem,
        intermediate_pem,
        private_key_pem,
    }
}

fn certificate(pem: &'static [u8]) -> CertificateDer<'static> {
    CertificateDer::from_pem_slice(pem).unwrap()
}

fn roots(pem: &'static [u8]) -> RootCertStore {
    let mut roots = RootCertStore::empty();
    roots.add(certificate(pem)).unwrap();
    roots
}
