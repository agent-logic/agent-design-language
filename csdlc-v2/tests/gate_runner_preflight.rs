use std::{
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    process::Command,
    sync::{Arc, Mutex},
    thread,
};

use serde_json::{json, Value};

#[test]
fn cli_paginates_api_and_emits_redacted_fail_closed_packet() {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind loopback");
    let address = listener.local_addr().expect("loopback address");
    let requests = Arc::new(Mutex::new(Vec::new()));
    let observed = Arc::clone(&requests);
    let server = thread::spawn(move || {
        for _ in 0..5 {
            let (mut stream, _) = listener.accept().expect("accept request");
            let target = read_target(&mut stream);
            observed.lock().unwrap().push(target.clone());
            let body = response_for(&target);
            write_json(&mut stream, &body);
        }
    });

    let temp = tempfile::tempdir().expect("temp directory");
    let token_path = temp.path().join("token");
    fs::write(&token_path, "super-secret-token\n").expect("write token");
    let request_path = temp.path().join("request.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&json!({
            "repository": "agent-logic/agent-design-language",
            "organization": "agent-logic",
            "runner_group_id": 3,
            "expected_label": "adl-ubuntu-24.04-16core",
            "workflow_path": ".github/workflows/ci.yaml",
            "queue_timeout_seconds": 300,
            "token_file": token_path
        }))
        .unwrap(),
    )
    .expect("write request");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc-github"))
        .args(["runner-preflight", "--request"])
        .arg(&request_path)
        .env(
            "CSDLC_V2_TEST_GITHUB_API_BASE",
            format!("http://{address}/"),
        )
        .output()
        .expect("run preflight");
    server.join().expect("join server");

    assert_eq!(output.status.code(), Some(2));
    assert!(output.stderr.is_empty());
    let stdout = String::from_utf8(output.stdout).expect("UTF-8 stdout");
    assert!(!stdout.contains("super-secret-token"));
    assert!(!stdout.contains(token_path.to_string_lossy().as_ref()));
    let packet: Value = serde_json::from_str(&stdout).expect("diagnostic JSON");
    assert_eq!(packet["capacity"], "ready");
    assert_eq!(packet["policy"], "eligible");
    assert_eq!(
        packet["classification"],
        "configuration_eligible_dispatch_unproven"
    );
    assert_eq!(packet["repository_selected"], true);

    let requests = requests.lock().unwrap();
    assert!(requests
        .iter()
        .any(|target| target.ends_with("hosted-runners?per_page=100&page=2")));
    assert!(requests
        .iter()
        .any(|target| target.ends_with("repositories?per_page=100&page=2")));
}

fn read_target(stream: &mut TcpStream) -> String {
    let mut bytes = Vec::new();
    let mut buffer = [0_u8; 1024];
    loop {
        let count = stream.read(&mut buffer).expect("read request");
        bytes.extend_from_slice(&buffer[..count]);
        if bytes.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }
    let request = String::from_utf8(bytes).expect("UTF-8 request");
    request
        .lines()
        .next()
        .and_then(|line| line.split_whitespace().nth(1))
        .expect("request target")
        .to_owned()
}

fn response_for(target: &str) -> Value {
    if target.ends_with("hosted-runners?per_page=100&page=1") {
        return json!({"total_count": 2, "runners": [{
            "name": "other", "status": "Ready", "maximum_runners": 1,
            "runner_group_id": 3
        }]});
    }
    if target.ends_with("hosted-runners?per_page=100&page=2") {
        return json!({"total_count": 2, "runners": [{
            "name": "adl-ubuntu-24.04-16core", "status": "Ready",
            "maximum_runners": 10, "runner_group_id": 3
        }]});
    }
    if target.ends_with("runner-groups/3") {
        return json!({
            "id": 3, "name": "adl-build-experiment", "visibility": "selected",
            "restricted_to_workflows": false, "selected_workflows": []
        });
    }
    if target.ends_with("repositories?per_page=100&page=1") {
        return json!({"total_count": 2, "repositories": [{"full_name": "agent-logic/other"}]});
    }
    if target.ends_with("repositories?per_page=100&page=2") {
        return json!({"total_count": 2, "repositories": [{
            "full_name": "agent-logic/agent-design-language"
        }]});
    }
    panic!("unexpected request target: {target}");
}

fn write_json(stream: &mut TcpStream, body: &Value) {
    let body = serde_json::to_vec(body).expect("serialize response");
    write!(
        stream,
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n",
        body.len()
    )
    .expect("write headers");
    stream.write_all(&body).expect("write body");
}
