use csdlc_v2::{
    append_marker, execute_github_action, marker_line, GithubAction, GithubActionRequest,
};
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    mpsc, Arc, Mutex,
};
use std::thread;
use std::time::Duration;

fn base_request(action: GithubAction) -> GithubActionRequest {
    GithubActionRequest {
        repository: "owner/repo".into(),
        action,
        operation_key: Some("issue-5655.test-key".into()),
        token_file: None,
        issue: None,
        pull_request: None,
        title: None,
        body: None,
        labels: Vec::new(),
        assignees: Vec::new(),
        milestone: None,
        state: None,
        comment_body: None,
        required_checks: Vec::new(),
        require_review: false,
        linked_issue: None,
    }
}

#[test]
fn operation_marker_is_stable_and_idempotent() {
    let marker = marker_line("issue-5655.test-key");
    assert_eq!(
        marker,
        "<!-- csdlc-github-operation:issue-5655.test-key -->"
    );
    let once = append_marker("body", "issue-5655.test-key");
    let twice = append_marker(&once, "issue-5655.test-key");
    assert_eq!(once, twice);
    assert!(once.contains(&marker));
}

#[tokio::test]
async fn issue_create_requires_valid_operation_key() {
    let mut request = base_request(GithubAction::IssueCreate);
    request.title = Some("Title".into());
    request.body = Some("Body".into());
    request.operation_key = None;
    let error = execute_github_action(&request)
        .await
        .expect_err("missing key");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);

    request.operation_key = Some("../bad".into());
    let error = execute_github_action(&request).await.expect_err("bad key");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
}

#[tokio::test]
async fn issue_create_requires_title_and_body_before_token_resolution() {
    let request = base_request(GithubAction::IssueCreate);
    let error = execute_github_action(&request)
        .await
        .expect_err("missing title/body");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("title and body"));
}

#[tokio::test]
async fn issue_update_rejects_invalid_state_before_token_resolution() {
    let mut request = base_request(GithubAction::IssueUpdate);
    request.issue = Some(42);
    request.state = Some("done".into());
    let error = execute_github_action(&request)
        .await
        .expect_err("invalid state");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("state"));
}

#[tokio::test]
async fn pr_state_requires_pull_request_before_token_resolution() {
    let request = base_request(GithubAction::PrState);
    let error = execute_github_action(&request)
        .await
        .expect_err("missing pull request");
    assert_eq!(error.code, csdlc_v2::ErrorCode::InvalidInput);
    assert!(error.message.contains("pull_request"));
}

#[tokio::test]
async fn issue_create_and_comment_reconcile_by_marker_with_exact_readback() {
    let server = LocalGithub::start();
    let previous_base = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE");
    std::env::set_var("CSDLC_V2_TEST_GITHUB_API_BASE", server.uri());
    let mut token = tempfile::NamedTempFile::new().expect("token file");
    writeln!(token, "fake-token").expect("write token");

    let mut create = base_request(GithubAction::IssueCreate);
    create.token_file = Some(token.path().to_string_lossy().into_owned());
    create.title = Some("Repo-native GitHub action surface".into());
    create.body = Some("Create issue body".into());
    create.labels = vec!["area:tools".into(), "version:v0.91.8".into()];
    let first = execute_github_action(&create).await.expect("create");
    let second = execute_github_action(&create).await.expect("reconcile");
    assert_eq!(first.issue.as_ref().unwrap().number, 77);
    assert_eq!(second.issue.as_ref().unwrap().number, 77);
    assert!(second.issue.as_ref().unwrap().marker_present);
    assert_eq!(server.count("POST", "/repos/owner/repo/issues"), 1);

    let mut comment = base_request(GithubAction::IssueComment);
    comment.token_file = create.token_file.clone();
    comment.issue = Some(77);
    comment.comment_body = Some("Retained closeout note".into());
    let first_comment = execute_github_action(&comment).await.expect("comment");
    let second_comment = execute_github_action(&comment)
        .await
        .expect("comment reconcile");
    assert_eq!(first_comment.comment_id, Some(9001));
    assert_eq!(second_comment.comment_id, Some(9001));
    assert_eq!(
        server.count("POST", "/repos/owner/repo/issues/77/comments"),
        1
    );
    server.assert_clean();

    match previous_base {
        Some(value) => std::env::set_var("CSDLC_V2_TEST_GITHUB_API_BASE", value),
        None => std::env::remove_var("CSDLC_V2_TEST_GITHUB_API_BASE"),
    }
}

#[derive(Default)]
struct LocalGithubState {
    issue: Option<Value>,
    comment: Option<Value>,
}

struct LocalGithub {
    address: SocketAddr,
    requests: Arc<Mutex<Vec<(String, String)>>>,
    failures: Arc<Mutex<Vec<String>>>,
    stop: Arc<AtomicBool>,
    thread: Option<thread::JoinHandle<()>>,
}

impl LocalGithub {
    fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind mock");
        listener.set_nonblocking(true).expect("nonblocking mock");
        let address = listener.local_addr().expect("mock address");
        let state = Arc::new(Mutex::new(LocalGithubState::default()));
        let requests = Arc::new(Mutex::new(Vec::new()));
        let failures = Arc::new(Mutex::new(Vec::new()));
        let stop = Arc::new(AtomicBool::new(false));
        let thread_state = Arc::clone(&state);
        let thread_requests = Arc::clone(&requests);
        let thread_failures = Arc::clone(&failures);
        let thread_stop = Arc::clone(&stop);
        let (started_tx, started_rx) = mpsc::sync_channel(0);
        let thread = thread::spawn(move || {
            let _ = started_tx.send(());
            while !thread_stop.load(Ordering::SeqCst) {
                match listener.accept() {
                    Ok((mut stream, _)) => {
                        if let Some(request) = read_request(&mut stream) {
                            thread_requests
                                .lock()
                                .unwrap()
                                .push((request.method.clone(), request.path_only()));
                            let response = respond(&thread_state, request);
                            write_response(&mut stream, response);
                        } else {
                            thread_failures
                                .lock()
                                .unwrap()
                                .push("mock server received unreadable request".into());
                        }
                    }
                    Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                        thread::sleep(Duration::from_millis(2));
                    }
                    Err(error) => {
                        thread_failures
                            .lock()
                            .unwrap()
                            .push(format!("mock listener failed: {error}"));
                        break;
                    }
                }
            }
        });
        started_rx
            .recv_timeout(Duration::from_secs(2))
            .expect("mock listener did not start");
        Self {
            address,
            requests,
            failures,
            stop,
            thread: Some(thread),
        }
    }

    fn uri(&self) -> String {
        format!("http://{}/", self.address)
    }

    fn count(&self, method: &str, path: &str) -> usize {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .filter(|(m, p)| m == method && p == path)
            .count()
    }

    fn assert_clean(&self) {
        let failures = self.failures.lock().unwrap();
        assert!(failures.is_empty(), "{failures:?}");
    }
}

impl Drop for LocalGithub {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(self.address);
        if let Some(thread) = self.thread.take() {
            let _ = thread.join();
        }
    }
}

struct MockRequest {
    method: String,
    target: String,
    body: String,
}

impl MockRequest {
    fn path_only(&self) -> String {
        self.target
            .split_once('?')
            .map(|(path, _)| path)
            .unwrap_or(&self.target)
            .to_owned()
    }
}

fn read_request(stream: &mut TcpStream) -> Option<MockRequest> {
    let mut buffer = Vec::new();
    let mut byte = [0_u8; 1];
    while stream.read_exact(&mut byte).is_ok() {
        buffer.push(byte[0]);
        if buffer.ends_with(b"\r\n\r\n") {
            break;
        }
    }
    let headers = String::from_utf8(buffer).ok()?;
    let mut lines = headers.lines();
    let first = lines.next()?;
    let mut parts = first.split_whitespace();
    let method = parts.next()?.to_owned();
    let target = parts.next()?.to_owned();
    let content_length = headers
        .lines()
        .find_map(|line| line.strip_prefix("content-length: "))
        .or_else(|| {
            headers
                .lines()
                .find_map(|line| line.strip_prefix("Content-Length: "))
        })
        .and_then(|value| value.trim().parse::<usize>().ok())
        .unwrap_or(0);
    let mut body = vec![0_u8; content_length];
    if content_length > 0 {
        stream.read_exact(&mut body).ok()?;
    }
    Some(MockRequest {
        method,
        target,
        body: String::from_utf8(body).ok()?,
    })
}

fn respond(state: &Arc<Mutex<LocalGithubState>>, request: MockRequest) -> Value {
    let marker = marker_line("issue-5655.test-key");
    let mut state = state.lock().unwrap();
    match (request.method.as_str(), request.path_only().as_str()) {
        ("GET", "/search/issues") => json!({
            "total_count": state.issue.as_ref().map_or(0, |_| 1),
            "items": state.issue.as_ref().map(|issue| vec![issue.clone()]).unwrap_or_default()
        }),
        ("POST", "/repos/owner/repo/issues") => {
            let payload: Value = serde_json::from_str(&request.body).expect("issue payload");
            let issue = json!({
                "number": 77,
                "title": payload["title"],
                "body": payload["body"],
                "state": "open",
                "labels": [{"name":"area:tools"}, {"name":"version:v0.91.8"}],
                "assignees": [],
                "milestone": null
            });
            assert!(issue["body"].as_str().unwrap().contains(&marker));
            state.issue = Some(issue.clone());
            issue
        }
        ("GET", "/repos/owner/repo/issues/77") => state.issue.clone().expect("issue exists"),
        ("GET", "/repos/owner/repo/issues/77/comments") => Value::Array(
            state
                .comment
                .as_ref()
                .map(|v| vec![v.clone()])
                .unwrap_or_default(),
        ),
        ("POST", "/repos/owner/repo/issues/77/comments") => {
            let payload: Value = serde_json::from_str(&request.body).expect("comment payload");
            let comment = json!({"id": 9001, "body": payload["body"]});
            assert!(comment["body"].as_str().unwrap().contains(&marker));
            state.comment = Some(comment.clone());
            comment
        }
        _ => panic!(
            "unexpected mock request: {} {}",
            request.method, request.target
        ),
    }
}

fn write_response(stream: &mut TcpStream, body: Value) {
    let body = body.to_string();
    let response = format!(
        "HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: close\r\n\r\n{}",
        body.len(),
        body
    );
    stream
        .write_all(response.as_bytes())
        .expect("write response");
}
