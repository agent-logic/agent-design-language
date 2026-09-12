//! #876 PVF: deterministic production integration; bounded loopback/CPU/filesystem;
//! required provider-platform gate. No credentials or metered provider calls.
use super::*;
use std::io::{Read, Write};
use std::sync::{
    atomic::{AtomicBool, AtomicUsize, Ordering},
    mpsc, Arc,
};
use std::time::{Duration, Instant};

struct Endpoint {
    url: String,
    requests: mpsc::Receiver<serde_json::Value>,
    release: mpsc::Sender<()>,
    count: Arc<AtomicUsize>,
    stop: Arc<AtomicBool>,
    worker: Option<std::thread::JoinHandle<()>>,
}
impl Endpoint {
    fn start() -> Self {
        let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        listener.set_nonblocking(true).unwrap();
        let url = format!("http://{}/api/generate", listener.local_addr().unwrap());
        let (tx, requests) = mpsc::channel();
        let (release, rx) = mpsc::channel();
        let count = Arc::new(AtomicUsize::new(0));
        let stop = Arc::new(AtomicBool::new(false));
        let worker_count = count.clone();
        let worker_stop = stop.clone();
        let worker = std::thread::spawn(move || {
            while !worker_stop.load(Ordering::SeqCst) {
                let (mut socket, _) = match listener.accept() {
                    Ok(connection) => connection,
                    Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                        std::thread::sleep(Duration::from_millis(2));
                        continue;
                    }
                    Err(e) => panic!("accept: {e}"),
                };
                socket.set_nonblocking(false).unwrap();
                socket
                    .set_read_timeout(Some(Duration::from_secs(5)))
                    .unwrap();
                let mut bytes = Vec::new();
                let (body_start, length) = loop {
                    let mut chunk = [0; 2048];
                    let read = socket.read(&mut chunk).unwrap();
                    assert!(read > 0);
                    bytes.extend_from_slice(&chunk[..read]);
                    assert!(bytes.len() < 65536);
                    if let Some(end) = bytes.windows(4).position(|w| w == b"\r\n\r\n") {
                        let header = std::str::from_utf8(&bytes[..end]).unwrap();
                        assert!(header.starts_with("POST /api/generate "));
                        let length = header
                            .lines()
                            .find_map(|line| {
                                let (name, value) = line.split_once(':')?;
                                name.eq_ignore_ascii_case("content-length")
                                    .then(|| value.trim().parse::<usize>().unwrap())
                            })
                            .unwrap();
                        break (end + 4, length);
                    }
                };
                while bytes.len() < body_start + length {
                    let mut chunk = [0; 2048];
                    let read = socket.read(&mut chunk).unwrap();
                    assert!(read > 0);
                    bytes.extend_from_slice(&chunk[..read]);
                }
                let body: serde_json::Value =
                    serde_json::from_slice(&bytes[body_start..body_start + length]).unwrap();
                worker_count.fetch_add(1, Ordering::SeqCst);
                tx.send(body.clone()).unwrap();
                // Request arrival, not elapsed sleep, controls the reload boundary.
                if rx.recv_timeout(Duration::from_secs(10)).is_err() {
                    break;
                }
                let response = serde_json::json!({"response": body["model"]}).to_string();
                write!(socket, "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", response.len(), response).unwrap();
            }
        });
        Self {
            url,
            requests,
            release,
            count,
            stop,
            worker: Some(worker),
        }
    }
    fn request(&self, model: &str, temperature: f64) {
        let body = self
            .requests
            .recv_timeout(Duration::from_secs(5))
            .expect("real provider request");
        assert_eq!(body["model"], model);
        assert_eq!(body["options"]["temperature"], temperature);
        assert_eq!(body["stream"], false);
    }
}
impl Drop for Endpoint {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = self.release.send(());
        if let Some(worker) = self.worker.take() {
            let result = worker.join();
            if !std::thread::panicking() {
                result.unwrap();
            }
        }
    }
}
fn sidecar(endpoint: &str, profile: &str, temperature: f64) -> String {
    format!("schema: adl.provider_reload_sidecar.v1\nproviders:\n  p1: &definition\n    profile: {profile}\n    config:\n      endpoint: {endpoint}\n      temperature: {temperature}\n  p2: *definition\n")
}
fn dispatch(
    resolved: &AdlResolved,
    base: &std::path::Path,
    handle: &crate::provider::ProviderReloadHandle,
) -> std::thread::JoinHandle<String> {
    let (resolved, base, handle) = (resolved.clone(), base.to_path_buf(), handle.clone());
    std::thread::spawn(move || execute_reload_once_with_handle(&resolved, &base, &handle))
}

#[test]
fn editable_provider_definitions_drive_real_dispatch_and_atomic_reload() {
    let old = Endpoint::start();
    let new = Endpoint::start();
    let base = unique_temp_path("adl-editable-provider-definitions");
    std::fs::create_dir_all(&base).unwrap();
    let path = base.join("providers.yaml");
    std::fs::write(&path, sidecar(&old.url, "ollama:phi4-mini", 0.0)).unwrap();
    let resolved = provider_reload_resolved("unused", 0);
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let owner = runtime
        .block_on(crate::provider::ProviderReloadOwner::start(
            path.clone(),
            resolved.doc.clone(),
            adl_runtime_kernel::config_reload::ConfigReloadOptions {
                poll_interval: Duration::from_millis(5),
                debounce: Duration::from_millis(10),
            },
        ))
        .expect("profile-only definitions must load through production owner");
    let mut handle = owner.handle();
    let old_snapshot = handle.current_snapshot();
    let old_request = dispatch(&resolved, &base.join("old"), &handle);
    old.request("phi4-mini", 0.0);
    let read_handle = handle.clone();
    let reading = Arc::new(AtomicBool::new(true));
    let reader_flag = reading.clone();
    let reader = std::thread::spawn(move || {
        let mut count = 0;
        while reader_flag.load(Ordering::SeqCst) {
            let snapshot = read_handle.current_snapshot();
            let p1 = &snapshot.document.providers["p1"];
            let p2 = &snapshot.document.providers["p2"];
            assert_eq!(p1.profile, p2.profile);
            assert_eq!(p1.default_model, p2.default_model);
            assert_eq!(p1.config["endpoint"], p2.config["endpoint"]);
            count += 1;
            std::thread::yield_now();
        }
        count
    });
    std::fs::write(&path, sidecar(&new.url, "ollama:qwen2.5-7b", 0.5)).unwrap();
    let changed = runtime
        .block_on(async { tokio::time::timeout(Duration::from_secs(5), handle.changed()).await })
        .unwrap()
        .unwrap();
    assert_eq!(changed.generation, 1);
    assert_eq!(
        old_snapshot.document.providers["p1"]
            .default_model
            .as_deref(),
        Some("phi4-mini")
    );
    let new_request = dispatch(&resolved, &base.join("new"), &handle);
    new.request("qwen2.5:7b", 0.5);
    new.release.send(()).unwrap();
    assert_eq!(new_request.join().unwrap(), "qwen2.5:7b");
    old.release.send(()).unwrap();
    assert_eq!(old_request.join().unwrap(), "phi4-mini");
    // Invalid expanded candidate must not partially promote the replacement map.
    let invalid = sidecar("123", "ollama:phi4-mini", 0.0);
    std::fs::write(&path, invalid).unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while handle.last_diagnostic().is_none() {
        assert!(Instant::now() < deadline);
        std::thread::sleep(Duration::from_millis(2));
    }
    assert_eq!(handle.current_snapshot().generation, 1);
    assert_eq!(handle.current_snapshot().digest, changed.digest);
    let retained = dispatch(&resolved, &base.join("retained"), &handle);
    new.request("qwen2.5:7b", 0.5);
    new.release.send(()).unwrap();
    assert_eq!(retained.join().unwrap(), "qwen2.5:7b");
    reading.store(false, Ordering::SeqCst);
    assert!(reader.join().unwrap() > 0);
    let outcome = runtime.block_on(owner.shutdown()).unwrap();
    assert_eq!(outcome.reloads_applied, 1);
    assert!(outcome.invalid_updates_rejected >= 1);
    assert_eq!(old.count.load(Ordering::SeqCst), 1);
    assert_eq!(
        new.count.load(Ordering::SeqCst),
        2,
        "reload introduces no probe requests"
    );
    std::fs::remove_dir_all(base).unwrap();
}
