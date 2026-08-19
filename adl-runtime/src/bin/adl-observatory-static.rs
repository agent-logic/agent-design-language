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

#[derive(Clone)]
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
