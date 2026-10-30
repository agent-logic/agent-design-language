use std::{
    env,
    fs::OpenOptions,
    net::SocketAddr,
    path::{Component, Path, PathBuf},
};

#[cfg(unix)]
use std::os::fd::AsRawFd;

use axum::{
    body::Body,
    extract::{OriginalUri, State},
    http::{header, HeaderValue, StatusCode},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};
use axum_server::tls_rustls::RustlsConfig;
use tokio::{fs, runtime};

#[derive(Clone)]
struct AppState {
    root: PathBuf,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("adl-observatory-static error={error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let config = Config::parse(env::args().skip(1).collect())?;
    daemonize_if_requested(&config)?;
    let runtime = runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .map_err(|error| format!("tokio_runtime_failed: {error}"))?;
    runtime.block_on(serve_forever(config))
}

async fn serve_forever(config: Config) -> Result<(), String> {
    loop {
        serve(config.clone()).await?;
        eprintln!("adl-observatory-static event=server_returned_cleanly action=restart");
    }
}

async fn serve(config: Config) -> Result<(), String> {
    let tls = RustlsConfig::from_pem_file(&config.cert, &config.key)
        .await
        .map_err(|error| format!("load_tls_failed: {error}"))?;
    let app = Router::new()
        .fallback(get(serve_static))
        .with_state(AppState { root: config.root });

    axum_server::bind_rustls(config.addr, tls)
        .serve(app.into_make_service())
        .await
        .map_err(|error| format!("serve_failed: {error}"))
}

#[cfg(unix)]
fn daemonize_if_requested(config: &Config) -> Result<(), String> {
    if !config.daemon {
        return Ok(());
    }
    let pid_file = config
        .pid_file
        .as_ref()
        .ok_or("--daemon requires --pid-file")?;
    let log_file = config
        .log_file
        .as_ref()
        .ok_or("--daemon requires --log-file")?;

    unsafe {
        let pid = libc::fork();
        if pid < 0 {
            return Err("daemon fork failed".to_string());
        }
        if pid > 0 {
            std::process::exit(0);
        }
        if libc::setsid() < 0 {
            return Err("daemon setsid failed".to_string());
        }
    }

    let log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(log_file)
        .map_err(|error| format!("open log failed: {error}"))?;
    let null = OpenOptions::new()
        .read(true)
        .open("/dev/null")
        .map_err(|error| format!("open /dev/null failed: {error}"))?;
    unsafe {
        libc::dup2(null.as_raw_fd(), libc::STDIN_FILENO);
        libc::dup2(log.as_raw_fd(), libc::STDOUT_FILENO);
        libc::dup2(log.as_raw_fd(), libc::STDERR_FILENO);
    }
    std::fs::write(pid_file, format!("{}\n", std::process::id()))
        .map_err(|error| format!("write pid file failed: {error}"))?;
    Ok(())
}

#[cfg(not(unix))]
fn daemonize_if_requested(config: &Config) -> Result<(), String> {
    if config.daemon {
        return Err("--daemon is only supported on Unix".to_string());
    }
    Ok(())
}

async fn serve_static(State(state): State<AppState>, OriginalUri(uri): OriginalUri) -> Response {
    match resolve_path(&state.root, uri.path()) {
        Some(path) => match fs::read(&path).await {
            Ok(bytes) => {
                let mut response = Body::from(bytes).into_response();
                if let Some(content_type) = content_type(&path) {
                    response
                        .headers_mut()
                        .insert(header::CONTENT_TYPE, HeaderValue::from_static(content_type));
                }
                response
            }
            Err(_) => StatusCode::NOT_FOUND.into_response(),
        },
        None => StatusCode::FORBIDDEN.into_response(),
    }
}

fn resolve_path(root: &Path, request_path: &str) -> Option<PathBuf> {
    let mut path = root.to_path_buf();
    let request_path = request_path.trim_start_matches('/');
    if request_path.is_empty() {
        path.push("index.html");
        return Some(path);
    }

    for component in Path::new(request_path).components() {
        match component {
            Component::Normal(part) => path.push(part),
            Component::CurDir => {}
            Component::RootDir | Component::Prefix(_) | Component::ParentDir => return None,
        }
    }
    if path.is_dir() {
        path.push("index.html");
    }
    Some(path)
}

fn content_type(path: &Path) -> Option<&'static str> {
    match path.extension().and_then(|extension| extension.to_str()) {
        Some("css") => Some("text/css; charset=utf-8"),
        Some("html") => Some("text/html; charset=utf-8"),
        Some("js") | Some("mjs") => Some("text/javascript; charset=utf-8"),
        Some("json") => Some("application/json; charset=utf-8"),
        Some("svg") => Some("image/svg+xml"),
        Some("txt") | Some("md") => Some("text/plain; charset=utf-8"),
        _ => None,
    }
}

#[derive(Clone, Debug)]
struct Config {
    addr: SocketAddr,
    cert: PathBuf,
    daemon: bool,
    key: PathBuf,
    log_file: Option<PathBuf>,
    pid_file: Option<PathBuf>,
    root: PathBuf,
}

impl Config {
    fn parse(args: Vec<String>) -> Result<Self, String> {
        let mut host = String::from("127.0.0.1");
        let mut port = String::from("8765");
        let mut cert = None;
        let mut daemon = false;
        let mut key = None;
        let mut log_file = None;
        let mut pid_file = None;
        let mut root = None;

        let mut i = 0;
        while i < args.len() {
            match args[i].as_str() {
                "--host" => {
                    i += 1;
                    host = args.get(i).cloned().ok_or("missing --host value")?;
                }
                "--port" => {
                    i += 1;
                    port = args.get(i).cloned().ok_or("missing --port value")?;
                }
                "--cert" => {
                    i += 1;
                    cert = Some(PathBuf::from(args.get(i).ok_or("missing --cert value")?));
                }
                "--daemon" => daemon = true,
                "--key" => {
                    i += 1;
                    key = Some(PathBuf::from(args.get(i).ok_or("missing --key value")?));
                }
                "--log-file" => {
                    i += 1;
                    log_file = Some(PathBuf::from(
                        args.get(i).ok_or("missing --log-file value")?,
                    ));
                }
                "--pid-file" => {
                    i += 1;
                    pid_file = Some(PathBuf::from(
                        args.get(i).ok_or("missing --pid-file value")?,
                    ));
                }
                "--root" => {
                    i += 1;
                    root = Some(PathBuf::from(args.get(i).ok_or("missing --root value")?));
                }
                "--help" | "-h" => return Err(usage()),
                other => return Err(format!("unknown argument: {other}\n{}", usage())),
            }
            i += 1;
        }

        let addr = format!("{host}:{port}")
            .parse()
            .map_err(|error| format!("invalid address: {error}"))?;
        let cert = cert.ok_or("missing --cert")?;
        let key = key.ok_or("missing --key")?;
        let root = root.ok_or("missing --root")?;
        if !root.is_dir() {
            return Err(format!("root is not a directory: {}", root.display()));
        }
        Ok(Self {
            addr,
            cert,
            daemon,
            key,
            log_file,
            pid_file,
            root,
        })
    }
}

fn usage() -> String {
    "usage: adl-observatory-static [--daemon --pid-file PID --log-file LOG] --host 127.0.0.1 --port 8765 --cert cert.pem --key key.pem --root demos/html-observatory".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use axum::http::Uri;
    use tempfile::TempDir;

    fn temp_root() -> TempDir {
        tempfile::tempdir().expect("temp root")
    }

    #[test]
    fn adl_observatory_static_config_requires_tls_and_root() {
        let error = Config::parse(vec![]).expect_err("missing cert must fail");
        assert_eq!(error, "missing --cert");

        let root = temp_root();
        let config = Config::parse(vec![
            "--host".to_owned(),
            "0.0.0.0".to_owned(),
            "--port".to_owned(),
            "20997".to_owned(),
            "--cert".to_owned(),
            "/cert/localhost.pem".to_owned(),
            "--key".to_owned(),
            "/cert/localhost-key.pem".to_owned(),
            "--root".to_owned(),
            root.path().display().to_string(),
            "--daemon".to_owned(),
            "--pid-file".to_owned(),
            "/tmp/csm-observatory.pid".to_owned(),
            "--log-file".to_owned(),
            "/tmp/csm-observatory.log".to_owned(),
        ])
        .expect("valid config");

        assert_eq!(config.addr, "0.0.0.0:20997".parse().expect("addr"));
        assert_eq!(config.cert, PathBuf::from("/cert/localhost.pem"));
        assert_eq!(config.key, PathBuf::from("/cert/localhost-key.pem"));
        assert!(config.daemon);
        assert_eq!(
            config.pid_file.as_deref(),
            Some(Path::new("/tmp/csm-observatory.pid"))
        );
        assert_eq!(
            config.log_file.as_deref(),
            Some(Path::new("/tmp/csm-observatory.log"))
        );
        assert_eq!(config.root, root.path());
    }

    #[test]
    fn adl_observatory_static_config_rejects_bad_arguments() {
        let missing_value =
            Config::parse(vec!["--cert".to_owned()]).expect_err("missing cert value must fail");
        assert_eq!(missing_value, "missing --cert value");

        let unknown =
            Config::parse(vec!["--surprise".to_owned()]).expect_err("unknown option must fail");
        assert!(unknown.contains("unknown argument: --surprise"));
        assert!(unknown.contains("usage: adl-observatory-static"));

        let nonexistent_root = Config::parse(vec![
            "--cert".to_owned(),
            "cert.pem".to_owned(),
            "--key".to_owned(),
            "key.pem".to_owned(),
            "--root".to_owned(),
            "/definitely/not/a/csm/root".to_owned(),
        ])
        .expect_err("non-directory root must fail");
        assert!(nonexistent_root.contains("root is not a directory"));

        let invalid_addr = Config::parse(vec![
            "--host".to_owned(),
            "not a host".to_owned(),
            "--port".to_owned(),
            "nope".to_owned(),
            "--cert".to_owned(),
            "cert.pem".to_owned(),
            "--key".to_owned(),
            "key.pem".to_owned(),
            "--root".to_owned(),
            ".".to_owned(),
        ])
        .expect_err("invalid address must fail");
        assert!(invalid_addr.contains("invalid address"));

        let missing_key = Config::parse(vec![
            "--cert".to_owned(),
            "cert.pem".to_owned(),
            "--root".to_owned(),
            ".".to_owned(),
        ])
        .expect_err("missing key must fail");
        assert_eq!(missing_key, "missing --key");

        let missing_root = Config::parse(vec![
            "--cert".to_owned(),
            "cert.pem".to_owned(),
            "--key".to_owned(),
            "key.pem".to_owned(),
        ])
        .expect_err("missing root must fail");
        assert_eq!(missing_root, "missing --root");
    }

    #[test]
    fn adl_observatory_static_usage_and_foreground_daemon_path_are_safe() {
        let root = temp_root();
        let config = Config::parse(vec![
            "--cert".to_owned(),
            "cert.pem".to_owned(),
            "--key".to_owned(),
            "key.pem".to_owned(),
            "--root".to_owned(),
            root.path().display().to_string(),
        ])
        .expect("foreground config");

        assert_eq!(config.addr, "127.0.0.1:8765".parse().expect("default addr"));
        assert!(!config.daemon);
        assert!(config.pid_file.is_none());
        assert!(config.log_file.is_none());
        daemonize_if_requested(&config).expect("foreground path must not daemonize");

        let help = Config::parse(vec!["--help".to_owned()]).expect_err("help exits via usage");
        assert_eq!(help, usage());
        assert!(help.contains("--cert cert.pem --key key.pem --root"));
    }

    #[test]
    fn adl_observatory_static_path_resolution_serves_index_and_blocks_escape() {
        let root = temp_root();
        let nested = root.path().join("assets");
        std::fs::create_dir_all(&nested).expect("nested dir");

        assert_eq!(
            resolve_path(root.path(), "/"),
            Some(root.path().join("index.html"))
        );
        assert_eq!(
            resolve_path(root.path(), "/assets/"),
            Some(nested.join("index.html"))
        );
        assert_eq!(
            resolve_path(root.path(), "/assets/app.js"),
            Some(nested.join("app.js"))
        );
        assert!(resolve_path(root.path(), "/../secret").is_none());
        assert!(resolve_path(root.path(), "/assets/../../secret").is_none());
    }

    #[test]
    fn adl_observatory_static_content_types_are_browser_safe() {
        assert_eq!(
            content_type(Path::new("index.html")),
            Some("text/html; charset=utf-8")
        );
        assert_eq!(
            content_type(Path::new("app.js")),
            Some("text/javascript; charset=utf-8")
        );
        assert_eq!(
            content_type(Path::new("style.css")),
            Some("text/css; charset=utf-8")
        );
        assert_eq!(
            content_type(Path::new("manifest.json")),
            Some("application/json; charset=utf-8")
        );
        assert_eq!(content_type(Path::new("icon.svg")), Some("image/svg+xml"));
        assert_eq!(
            content_type(Path::new("README.md")),
            Some("text/plain; charset=utf-8")
        );
        assert_eq!(content_type(Path::new("binary.wasm")), None);
    }

    #[tokio::test]
    async fn adl_observatory_static_serves_files_and_status_codes() {
        let root = temp_root();
        std::fs::write(root.path().join("index.html"), "<h1>observatory</h1>").expect("index");
        std::fs::write(root.path().join("app.js"), "console.log('ok');").expect("app");

        let state = AppState {
            root: root.path().to_path_buf(),
        };
        let index = serve_static(
            State(state.clone()),
            OriginalUri(Uri::from_static("https://localhost:20997/")),
        )
        .await;
        assert_eq!(index.status(), StatusCode::OK);
        assert_eq!(
            index.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/html; charset=utf-8"))
        );
        let bytes = to_bytes(index.into_body(), 1024).await.expect("index body");
        assert_eq!(&bytes[..], b"<h1>observatory</h1>");

        let script = serve_static(
            State(state.clone()),
            OriginalUri(Uri::from_static("https://localhost:20997/app.js")),
        )
        .await;
        assert_eq!(script.status(), StatusCode::OK);
        assert_eq!(
            script.headers().get(header::CONTENT_TYPE),
            Some(&HeaderValue::from_static("text/javascript; charset=utf-8"))
        );

        let missing = serve_static(
            State(state.clone()),
            OriginalUri(Uri::from_static("https://localhost:20997/missing.html")),
        )
        .await;
        assert_eq!(missing.status(), StatusCode::NOT_FOUND);

        let forbidden = serve_static(
            State(state),
            OriginalUri(Uri::from_static("https://localhost:20997/../secret")),
        )
        .await;
        assert_eq!(forbidden.status(), StatusCode::FORBIDDEN);
    }

    #[tokio::test]
    async fn adl_observatory_static_reports_tls_load_errors_before_binding() {
        let root = temp_root();
        let config = Config {
            addr: "127.0.0.1:0".parse().expect("addr"),
            cert: root.path().join("missing-cert.pem"),
            daemon: false,
            key: root.path().join("missing-key.pem"),
            log_file: None,
            pid_file: None,
            root: root.path().to_path_buf(),
        };

        let error = serve(config)
            .await
            .expect_err("missing TLS files must fail");
        assert!(error.contains("load_tls_failed"));
    }
}
