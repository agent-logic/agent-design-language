use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    net::{IpAddr, TcpListener},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::{Duration, Instant},
};

fn validate_loopback_ollama_origin(value: &str) -> Result<(), &'static str> {
    if value.is_empty() || value.trim() != value {
        return Err("origin must be nonempty and contain no surrounding whitespace");
    }
    let authority = value
        .strip_prefix("http://")
        .ok_or("origin must use plain HTTP on the local loopback boundary")?;
    if authority.contains(['/', '?', '#', '@', '\\']) {
        return Err("origin must contain only a loopback authority and explicit port");
    }
    let (host, port) = if let Some(bracketed) = authority.strip_prefix('[') {
        let (host, port) = bracketed
            .split_once("]:")
            .ok_or("bracketed loopback origin must include an explicit port")?;
        (host, port)
    } else {
        authority
            .rsplit_once(':')
            .ok_or("loopback origin must include an explicit port")?
    };
    let port = port
        .parse::<u16>()
        .map_err(|_| "loopback origin port is invalid")?;
    if port == 0 {
        return Err("loopback origin port must be nonzero");
    }
    let loopback = host.eq_ignore_ascii_case("localhost")
        || host
            .parse::<IpAddr>()
            .is_ok_and(|address| address.is_loopback());
    if !loopback {
        return Err("origin host must be localhost or a loopback IP address");
    }
    Ok(())
}

const OLLAMA_ATTESTED_RUNNER: &str = r#"#!/usr/bin/python3
import json, os, sys, urllib.request

request = json.loads(sys.stdin.readline())
host = os.environ["ADL_OLLAMA_HOST"].rstrip("/")

class RejectRedirects(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, request, file_pointer, code, message, headers, new_url):
        raise RuntimeError("Ollama endpoint redirects are denied")

opener = urllib.request.build_opener(RejectRedirects)

def get(path):
    with opener.open(host + path, timeout=10) as response:
        return json.load(response)

def post(path, payload):
    encoded = json.dumps(payload).encode("utf-8")
    command = urllib.request.Request(
        host + path,
        data=encoded,
        headers={"Content-Type": "application/json"},
        method="POST",
    )
    with opener.open(command, timeout=240) as response:
        return json.load(response)

models = get("/api/tags").get("models", [])
matching = [entry for entry in models if entry.get("name") == request["model_identity"]]
installed_digest = matching[0].get("digest", "") if len(matching) == 1 else ""
if installed_digest.removeprefix("sha256:") != request["model_artifact_sha256"]:
    raise RuntimeError("configured model identity or digest is not locally installed")

generated = post("/api/generate", {
    "model": request["model_identity"],
    "prompt": request["prompt"],
    "stream": False,
    "options": {"temperature": 0.0, "top_p": 0.1},
})
loaded = get("/api/ps").get("models", [])
resident = [entry for entry in loaded if entry.get("name") == request["model_identity"]]
if len(resident) != 1 or int(resident[0].get("size_vram", 0)) <= 0:
    raise RuntimeError("configured model is not resident on the local GPU backend")

response = {
    key: request[key]
    for key in [
        "correlation_id", "runtime_id", "nonce", "backend_identity",
        "model_identity", "model_artifact_sha256"
    ]
}
response["schema"] = "adl.runtime.shepherd_runner_response.v1"
response["response"] = generated["response"]
print(json.dumps(response, sort_keys=True))
"#;

#[test]
fn local_model_origin_accepts_only_structural_loopback_http_origins() {
    for accepted in [
        "http://localhost:11434",
        "http://LOCALHOST:11434",
        "http://127.0.0.1:11434",
        "http://127.255.0.1:11434",
        "http://[::1]:11434",
    ] {
        assert_eq!(
            validate_loopback_ollama_origin(accepted),
            Ok(()),
            "{accepted}"
        );
    }

    for rejected in [
        "https://localhost:11434",
        "http://localhost",
        "http://localhost:0",
        "http://localhost:70000",
        "http://localhost:11434/",
        "http://localhost:11434/api",
        "http://localhost:11434?target=evil.example",
        "http://localhost:11434#evil.example",
        "http://localhost@evil.example:11434",
        "http://localhost:11434@evil.example:80",
        "http://localhost.evil.example:11434",
        "http://127.0.0.1.evil.example:11434",
        "http://2130706433:11434",
        "http://%31%32%37.0.0.1:11434",
        "http://[::2]:11434",
        " http://127.0.0.1:11434",
        "http://127.0.0.1:11434\n",
    ] {
        assert!(
            validate_loopback_ollama_origin(rejected).is_err(),
            "unsafe origin accepted: {rejected:?}"
        );
    }
}

#[test]
fn local_model_runner_denies_redirects() {
    let redirect_target = TcpListener::bind("127.0.0.1:0").unwrap();
    redirect_target.set_nonblocking(true).unwrap();
    let target_address = redirect_target.local_addr().unwrap();
    let target_reached = Arc::new(AtomicBool::new(false));
    let target_reached_worker = Arc::clone(&target_reached);
    let target = thread::spawn(move || {
        let deadline = Instant::now() + Duration::from_secs(2);
        while Instant::now() < deadline {
            match redirect_target.accept() {
                Ok(_) => {
                    target_reached_worker.store(true, Ordering::SeqCst);
                    return;
                }
                Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                    thread::sleep(Duration::from_millis(10));
                }
                Err(error) => panic!("redirect target accept failed: {error}"),
            }
        }
    });

    let source = TcpListener::bind("127.0.0.1:0").unwrap();
    let source_address = source.local_addr().unwrap();
    let redirect = thread::spawn(move || {
        let (mut stream, _) = source.accept().unwrap();
        let mut request = [0_u8; 1024];
        let _ = stream.read(&mut request).unwrap();
        write!(
            stream,
            "HTTP/1.1 302 Found\r\nLocation: http://{target_address}/api/tags\r\nContent-Length: 0\r\nConnection: close\r\n\r\n"
        )
        .unwrap();
    });

    let mut child = Command::new("python3")
        .arg("-c")
        .arg(OLLAMA_ATTESTED_RUNNER)
        .env("ADL_OLLAMA_HOST", format!("http://{source_address}"))
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("python3 must be available for the attested local-model runner");
    child.stdin.take().unwrap().write_all(b"{}\n").unwrap();
    let output = child.wait_with_output().unwrap();
    redirect.join().unwrap();
    target.join().unwrap();

    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("Ollama endpoint redirects are denied")
    );
    assert!(!target_reached.load(Ordering::SeqCst));
}
