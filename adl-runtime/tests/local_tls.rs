use std::{
    fs,
    net::{IpAddr, Ipv4Addr},
    path::PathBuf,
    sync::{Arc, Barrier},
    thread,
};

use adl_runtime::local_tls::{
    bootstrap_runtime_tls, bootstrap_runtime_tls_with_generator, GeneratedTlsMaterial,
    LocalTlsError, RuntimeTlsBootstrapConfig, RuntimeTlsBootstrapEvent, RuntimeTlsBootstrapMode,
    LOCAL_TLS_BOOTSTRAP_SCHEMA,
};
use base64::Engine;
use tokio::net::TcpListener;
use tokio_rustls::rustls::{pki_types::CertificateDer, ClientConfig, RootCertStore};

fn local_config(root: PathBuf) -> RuntimeTlsBootstrapConfig {
    RuntimeTlsBootstrapConfig {
        schema: LOCAL_TLS_BOOTSTRAP_SCHEMA.to_owned(),
        mode: RuntimeTlsBootstrapMode::LocalSelfSigned,
        state_root: Some(root),
        tls_dir: Some(PathBuf::from("runtime-tls")),
        certificate_chain_path: PathBuf::from("runtime-local-chain.pem"),
        public_certificate_path: Some(PathBuf::from("runtime-local-public.pem")),
        private_key_path: PathBuf::from("runtime-local-key.pem"),
        dns_names: vec!["localhost".to_owned()],
        ip_addresses: vec![IpAddr::V4(Ipv4Addr::LOCALHOST)],
        replace: false,
    }
}

#[tokio::test]
async fn first_bootstrap_persists_rustls_accepted_material_with_restrictive_key() {
    let temp = tempfile::tempdir().unwrap();
    let config = local_config(temp.path().to_path_buf());
    let outcome = bootstrap_runtime_tls(&config).await.unwrap();
    assert_eq!(
        outcome.event,
        RuntimeTlsBootstrapEvent::LocalCertificateCreated
    );
    assert!(!outcome.reused_existing_identity);
    assert!(outcome.certificate_chain_path.exists());
    assert!(outcome.public_certificate_path.unwrap().exists());
    assert!(config
        .state_root
        .unwrap()
        .join("runtime-tls/runtime-local-key.pem")
        .exists());
    assert_runtime_tls_accepts_localhost(&outcome.certificate_chain_path, &key_path(temp.path()))
        .await;
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = fs::metadata(key_path(temp.path()))
            .unwrap()
            .permissions()
            .mode()
            & 0o777;
        assert_eq!(mode, 0o600);
    }
}

#[tokio::test]
async fn restart_reuses_same_certificate_identity() {
    let temp = tempfile::tempdir().unwrap();
    let config = local_config(temp.path().to_path_buf());
    let first = bootstrap_runtime_tls(&config).await.unwrap();
    let second = bootstrap_runtime_tls(&config).await.unwrap();
    assert_eq!(
        second.event,
        RuntimeTlsBootstrapEvent::LocalCertificateReused
    );
    assert!(second.reused_existing_identity);
    assert_eq!(first.certificate_sha256, second.certificate_sha256);
}

#[tokio::test]
async fn restart_repairs_stale_public_copy() {
    let temp = tempfile::tempdir().unwrap();
    let config = local_config(temp.path().to_path_buf());
    let first = bootstrap_runtime_tls(&config).await.unwrap();
    let public = first.public_certificate_path.as_ref().unwrap();
    fs::write(public, b"stale public certificate").unwrap();

    let second = bootstrap_runtime_tls(&config).await.unwrap();

    assert_eq!(
        second.event,
        RuntimeTlsBootstrapEvent::LocalCertificateReused
    );
    assert_eq!(
        fs::read(&second.certificate_chain_path).unwrap(),
        fs::read(second.public_certificate_path.unwrap()).unwrap()
    );
}

#[cfg(unix)]
#[tokio::test]
async fn restart_repairs_private_key_permission_drift() {
    use std::os::unix::fs::PermissionsExt;

    let temp = tempfile::tempdir().unwrap();
    let config = local_config(temp.path().to_path_buf());
    bootstrap_runtime_tls(&config).await.unwrap();
    let key = key_path(temp.path());
    fs::set_permissions(&key, fs::Permissions::from_mode(0o644)).unwrap();

    let outcome = bootstrap_runtime_tls(&config).await.unwrap();

    assert_eq!(
        outcome.event,
        RuntimeTlsBootstrapEvent::LocalCertificateReused
    );
    assert_eq!(
        fs::metadata(key).unwrap().permissions().mode() & 0o777,
        0o600
    );
}

#[tokio::test]
async fn configured_sans_are_validated_by_rustls() {
    let temp = tempfile::tempdir().unwrap();
    let config = local_config(temp.path().to_path_buf());
    let outcome = bootstrap_runtime_tls(&config).await.unwrap();
    assert_runtime_tls_accepts_localhost(&outcome.certificate_chain_path, &key_path(temp.path()))
        .await;
}

#[tokio::test]
async fn concurrent_bootstrap_excludes_second_writer() {
    let temp = tempfile::tempdir().unwrap();
    let root = temp.path().to_path_buf();
    let barrier = Arc::new(Barrier::new(2));
    let handles = (0..2)
        .map(|_| {
            let root = root.clone();
            let barrier = Arc::clone(&barrier);
            thread::spawn(move || {
                barrier.wait();
                let runtime = tokio::runtime::Runtime::new().unwrap();
                runtime.block_on(async move { bootstrap_runtime_tls(&local_config(root)).await })
            })
        })
        .collect::<Vec<_>>();
    let results = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect::<Vec<_>>();
    assert_eq!(results.iter().filter(|result| result.is_ok()).count(), 1);
    assert_eq!(results.iter().filter(|result| result.is_err()).count(), 1);
}

#[tokio::test]
async fn replacement_is_explicit_and_changes_identity() {
    let temp = tempfile::tempdir().unwrap();
    let mut config = local_config(temp.path().to_path_buf());
    let first = bootstrap_runtime_tls(&config).await.unwrap();
    config.replace = true;
    let second = bootstrap_runtime_tls(&config).await.unwrap();
    assert_eq!(
        second.event,
        RuntimeTlsBootstrapEvent::LocalCertificateReplaced
    );
    assert!(second.replaced_existing_identity);
    assert_ne!(first.certificate_sha256, second.certificate_sha256);
}

#[tokio::test]
async fn failed_replacement_preserves_last_valid_certificate() {
    let temp = tempfile::tempdir().unwrap();
    let mut config = local_config(temp.path().to_path_buf());
    let first = bootstrap_runtime_tls(&config).await.unwrap();
    config.replace = true;
    let failed = bootstrap_runtime_tls_with_generator(&config, |_| {
        Err::<GeneratedTlsMaterial, LocalTlsError>(LocalTlsError::Generate(
            "intentional replacement failure".to_owned(),
        ))
    })
    .await;
    assert!(failed.is_err());
    let after = bootstrap_runtime_tls(&local_config(temp.path().to_path_buf()))
        .await
        .unwrap();
    assert_eq!(first.certificate_sha256, after.certificate_sha256);
}

#[tokio::test]
async fn managed_external_mode_preserves_existing_material() {
    let temp = tempfile::tempdir().unwrap();
    let local = local_config(temp.path().join("local"));
    let created = bootstrap_runtime_tls(&local).await.unwrap();
    let cert_before = fs::read(&created.certificate_chain_path).unwrap();
    let key = local
        .state_root
        .unwrap()
        .join("runtime-tls/runtime-local-key.pem");
    let key_before = fs::read(&key).unwrap();
    let external = RuntimeTlsBootstrapConfig {
        schema: LOCAL_TLS_BOOTSTRAP_SCHEMA.to_owned(),
        mode: RuntimeTlsBootstrapMode::ManagedExternal,
        state_root: None,
        tls_dir: None,
        certificate_chain_path: created.certificate_chain_path.clone(),
        public_certificate_path: None,
        private_key_path: key.clone(),
        dns_names: vec![],
        ip_addresses: vec![],
        replace: false,
    };
    let outcome = bootstrap_runtime_tls(&external).await.unwrap();
    assert_eq!(
        outcome.event,
        RuntimeTlsBootstrapEvent::ManagedExternalPreserved
    );
    assert_eq!(
        cert_before,
        fs::read(&created.certificate_chain_path).unwrap()
    );
    assert_eq!(key_before, fs::read(&key).unwrap());
}

#[tokio::test]
async fn production_defaults_fail_closed_without_explicit_mode() {
    let text = format!(
        "schema = \"{LOCAL_TLS_BOOTSTRAP_SCHEMA}\"\ncertificate_chain_path = \"cert.pem\"\nprivate_key_path = \"key.pem\"\n"
    );
    assert!(RuntimeTlsBootstrapConfig::from_toml_str(&text).is_err());
}

fn key_path(root: &std::path::Path) -> PathBuf {
    root.join("runtime-tls/runtime-local-key.pem")
}

async fn assert_runtime_tls_accepts_localhost(
    cert_path: &std::path::Path,
    key_path: &std::path::Path,
) {
    let tls = axum_server::tls_rustls::RustlsConfig::from_pem_file(cert_path, key_path)
        .await
        .unwrap();
    let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).await.unwrap();
    let addr = listener.local_addr().unwrap();
    let std_listener = listener.into_std().unwrap();
    let handle = axum_server::Handle::new();
    let shutdown = handle.clone();
    let server = tokio::spawn(async move {
        let app = axum::Router::new().route("/", axum::routing::get(|| async { "ok" }));
        let _ = axum_server::from_tcp_rustls(std_listener, tls)
            .unwrap()
            .handle(handle)
            .serve(app.into_make_service())
            .await;
    });
    let mut roots = RootCertStore::empty();
    roots
        .add(CertificateDer::from(read_pem_der(cert_path)))
        .unwrap();
    let connector = tokio_rustls::TlsConnector::from(Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    ));
    let stream = tokio::net::TcpStream::connect(addr).await.unwrap();
    connector
        .connect("localhost".try_into().unwrap(), stream)
        .await
        .unwrap();
    shutdown.graceful_shutdown(None);
    server.await.unwrap();
}

fn read_pem_der(path: &std::path::Path) -> Vec<u8> {
    let text = fs::read_to_string(path).unwrap();
    let body = text
        .lines()
        .filter(|line| !line.starts_with("-----"))
        .collect::<String>();
    base64::engine::general_purpose::STANDARD
        .decode(body)
        .unwrap()
}
