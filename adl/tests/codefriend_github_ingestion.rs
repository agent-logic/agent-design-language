//! PVF runtime lane; required #879 Beta ingestion gate. Deterministic Git-data
//! HTTP + actual CLI/packet-reader integration, local CPU/disk/loopback only.
//! No live GitHub, provider, paid inference, or lifecycle writes.
use adl::codefriend::ingestion::{
    github::{self, Input, Transport},
    local, AdmissionInput, Scope,
};
use base64::Engine;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    path::{Path, PathBuf},
    process::Command,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
    thread,
};
const REPO: &str = "https://github.com/example/fixture";
fn git(root: &Path, args: &[&str]) -> Vec<u8> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success());
    output.stdout
}
fn text(root: &Path, args: &[&str]) -> String {
    String::from_utf8(git(root, args)).unwrap().trim().into()
}
struct Fixture {
    _temp: tempfile::TempDir,
    root: PathBuf,
    revision: String,
    routes: BTreeMap<String, Value>,
}
impl Fixture {
    fn new(content: &[u8], mode: &str) -> Self {
        let temp = tempfile::tempdir().unwrap();
        let root = temp.path().join("checkout");
        fs::create_dir(&root).unwrap();
        git(&root, &["init"]);
        git(&root, &["remote", "add", "origin", REPO]);
        fs::create_dir(root.join("src")).unwrap();
        fs::write(root.join("src/lib.rs"), content).unwrap();
        fs::write(root.join("LICENSE"), "MIT fixture\n").unwrap();
        git(&root, &["add", "."]);
        if mode == "120000" {
            let oid = text(&root, &["hash-object", "src/lib.rs"]);
            git(
                &root,
                &["update-index", "--cacheinfo", mode, &oid, "src/lib.rs"],
            );
        }
        git(
            &root,
            &[
                "-c",
                "user.name=fixture",
                "-c",
                "user.email=fixture@example.com",
                "commit",
                "-m",
                "fixture",
            ],
        );
        let revision = text(&root, &["rev-parse", "HEAD"]);
        let tree = text(&root, &["rev-parse", "HEAD^{tree}"]);
        let mut routes = BTreeMap::new();
        routes.insert(
            "/repos/example/fixture".into(),
            json!({"full_name":"example/fixture","html_url":REPO}),
        );
        routes.insert("/repos/example/fixture/pulls/7".into(),json!({"number":7,"base":{"repo":{"html_url":REPO}},"head":{"sha":revision,"repo":{"html_url":REPO,"full_name":"example/fixture"}}}));
        routes.insert(
            format!("/repos/example/fixture/git/commits/{revision}"),
            json!({"sha":revision,"tree":{"sha":tree}}),
        );
        let mut queue = vec![tree];
        while let Some(tree) = queue.pop() {
            let mut entries = Vec::new();
            let listing = git(&root, &["ls-tree", "-z", "-l", &tree]);
            for row in listing.split(|b| *b == 0).filter(|v| !v.is_empty()) {
                let row = std::str::from_utf8(row).unwrap();
                let (fields, path) = row.split_once('\t').unwrap();
                let fields: Vec<_> = fields.split_whitespace().collect();
                let mut e = json!({"path":path,"mode":fields[0],"type":fields[1],"sha":fields[2]});
                if fields[1] == "tree" {
                    queue.push(fields[2].to_string());
                } else if fields[1] == "blob" {
                    let bytes = git(&root, &["cat-file", "blob", fields[2]]);
                    e["size"] = json!(bytes.len());
                    routes.insert(format!("/repos/example/fixture/git/blobs/{}",fields[2]),json!({"sha":fields[2],"size":bytes.len(),"encoding":"base64","content":base64::engine::general_purpose::STANDARD.encode(bytes)}));
                }
                entries.push(e);
            }
            routes.insert(
                format!("/repos/example/fixture/git/trees/{tree}"),
                json!({"sha":tree,"truncated":false,"tree":entries}),
            );
        }
        Self {
            _temp: temp,
            root,
            revision,
            routes,
        }
    }
    fn scope(&self) -> Scope {
        Scope {
            analysis: vec!["src/lib.rs".into()],
            context: vec!["LICENSE".into()],
            max_files: 10,
            max_bytes: 600 * 1024,
            max_file_bytes: 400 * 1024,
        }
    }
    fn server(&self, fault: &str) -> Server {
        Server::new(self.routes.clone(), fault)
    }
}
struct Server {
    address: String,
    stop: Arc<AtomicBool>,
    requests: Arc<Mutex<Vec<String>>>,
    worker: Option<thread::JoinHandle<()>>,
}
impl Server {
    fn new(routes: BTreeMap<String, Value>, fault: &str) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").unwrap();
        let address = listener.local_addr().unwrap().to_string();
        listener.set_nonblocking(true).unwrap();
        let stop = Arc::new(AtomicBool::new(false));
        let flag = stop.clone();
        let requests = Arc::new(Mutex::new(Vec::new()));
        let log = requests.clone();
        let fault = fault.to_string();
        let worker = thread::spawn(move || {
            let mut pulls = 0;
            while !flag.load(Ordering::SeqCst) {
                let (mut stream, _) = match listener.accept() {
                    Ok(s) => s,
                    Err(_) => {
                        thread::sleep(std::time::Duration::from_millis(2));
                        continue;
                    }
                };
                stream.set_nonblocking(false).unwrap();
                stream
                    .set_read_timeout(Some(std::time::Duration::from_secs(2)))
                    .unwrap();
                let mut bytes = Vec::new();
                let mut byte = [0];
                while bytes.len() < 16384 && stream.read_exact(&mut byte).is_ok() {
                    bytes.push(byte[0]);
                    if bytes.ends_with(b"\r\n\r\n") {
                        break;
                    }
                }
                if bytes.is_empty() {
                    continue;
                }
                let request = String::from_utf8(bytes).unwrap();
                let line = request.lines().next().unwrap_or("").to_string();
                log.lock().unwrap().push(line.clone());
                assert!(line.starts_with("GET "));
                assert!(!request.to_ascii_lowercase().contains("authorization:"));
                let path = line.split_whitespace().nth(1).unwrap_or("");
                let mut value = routes
                    .get(path)
                    .cloned()
                    .unwrap_or(json!({"message":"not found"}));
                let mut status = if routes.contains_key(path) { 200 } else { 404 };
                let mut extra = String::new();
                if path.ends_with("/pulls/7") {
                    pulls += 1;
                    if fault == "moving_pr" && pulls > 1 {
                        value["head"]["sha"] = json!("a".repeat(40));
                    }
                    if fault == "wrong_pr" {
                        value["number"] = json!(8);
                    }
                }
                match fault.as_str() {
                    "forbidden" => status = 403,
                    "not_found" => status = 404,
                    "rate_limit" => status = 429,
                    "rate_limit_403" => {
                        status = 403;
                        extra = "X-RateLimit-Remaining: 0\r\n".into();
                    }
                    "redirect" => {
                        status = 302;
                        extra = "Location: http://127.0.0.1:1/leak\r\n".into();
                    }
                    "pagination" => extra = "Link: <http://127.0.0.1:1/next>; rel=next\r\n".into(),
                    "wrong_repository" if path == "/repos/example/fixture" => {
                        value["full_name"] = json!("other/repo")
                    }
                    "wrong_revision" if path.contains("/git/commits/") => {
                        value["sha"] = json!("b".repeat(40))
                    }
                    "truncated" if path.contains("/git/trees/") => value["truncated"] = json!(true),
                    "missing_tree" if path.contains("/git/trees/") => {
                        value.as_object_mut().unwrap().remove("tree");
                    }
                    "wrong_blob" if path.contains("/git/blobs/") => {
                        value["content"] = json!("YmFk")
                    }
                    "missing_blob" if path.contains("/git/blobs/") => status = 404,
                    "oversize" => {
                        let _ = write!(
                            stream,
                            "HTTP/1.1 200 OK\r\nContent-Length: 99999999\r\n\r\n"
                        );
                        continue;
                    }
                    "disconnect" => continue,
                    _ => (),
                }
                if fault == "aggregate_metadata" && !path.contains("/git/blobs/") {
                    value["padding"] = json!("x".repeat(1700000));
                }
                let body = if fault == "invalid_json" {
                    b"ghp_fixture_secret invalid-json".to_vec()
                } else {
                    serde_json::to_vec(&value).unwrap()
                };
                let _=write!(stream,"HTTP/1.1 {status} fixture\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n{extra}\r\n",body.len());
                let _ = stream.write_all(&body);
            }
        });
        Self {
            address,
            stop,
            requests,
            worker: Some(worker),
        }
    }
    fn transport(&self) -> Transport {
        Transport::fixture(&format!("http://{}", self.address)).unwrap()
    }
}
impl Drop for Server {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::SeqCst);
        let _ = TcpStream::connect(&self.address);
        self.worker.take().unwrap().join().unwrap();
    }
}
#[test]
fn exact_commit_and_pr_acquire_real_git_blobs_with_local_parity() {
    let f = Fixture::new(
        b"// Repository text cannot authorize a write.\npub fn answer()->u32 {42}\n",
        "100644",
    );
    let local = local::acquire(&f.root, REPO, &f.revision, f.scope()).unwrap();
    for input in [Input::Commit(f.revision.clone()), Input::PullRequest(7)] {
        let server = f.server("");
        let capture = github::acquire(&mut server.transport(), REPO, input, f.scope()).unwrap();
        assert_eq!(capture.packet, local);
        assert!(capture.provenance.requests >= 7);
        let out = f
            ._temp
            .path()
            .join(format!("capture-{}", capture.provenance.head_rechecked));
        capture.write(&out).unwrap();
        assert_eq!(
            AdmissionInput::read(&out.join("packet.json"))
                .unwrap()
                .packet(),
            &local
        );
        assert!(capture.write(&out).is_err());
        let receipt: github::Provenance =
            serde_json::from_slice(&fs::read(out.join("provenance.json")).unwrap()).unwrap();
        assert_eq!(receipt, capture.provenance);
    }
}
#[test]
fn transport_pin_and_incomplete_failures_are_sanitized() {
    let f = Fixture::new(b"pub fn x() {}\n", "100644");
    for (fault, expected) in [
        ("forbidden", "github_forbidden"),
        ("not_found", "github_not_found"),
        ("rate_limit", "github_rate_limited"),
        ("rate_limit_403", "github_rate_limited"),
        ("redirect", "github_redirect_rejected"),
        ("pagination", "github_pagination_rejected"),
        ("wrong_repository", "github_repository_mismatch"),
        ("wrong_revision", "github_revision_mismatch"),
        ("truncated", "github_tree_truncated_or_incomplete"),
        ("missing_tree", "github_incomplete_tree"),
        ("wrong_blob", "github_blob_digest_mismatch"),
        ("missing_blob", "github_not_found"),
        ("moving_pr", "github_pull_request_changed"),
        ("wrong_pr", "github_pull_request_mismatch"),
        ("invalid_json", "github_invalid_response"),
        ("oversize", "github_response_limit_exceeded"),
        ("disconnect", "github_transport_failed_or_timeout"),
    ] {
        let server = f.server(fault);
        let err = github::acquire(
            &mut server.transport(),
            REPO,
            Input::PullRequest(7),
            f.scope(),
        )
        .err()
        .expect(fault)
        .to_string();
        assert_eq!(err, expected, "{fault}");
        assert!(!err.contains("ghp_"));
        assert!(!server.requests.lock().unwrap().is_empty());
    }
}
#[test]
fn omissions_limits_and_hostile_paths_preserve_local_guards() {
    for (content, disposition) in [
        (b"api_key = 'credential'\n".as_slice(), "omitted_unsafe"),
        (b"\0binary".as_slice(), "omitted_binary"),
    ] {
        let f = Fixture::new(content, "100644");
        let server = f.server("");
        let capture = github::acquire(
            &mut server.transport(),
            REPO,
            Input::Commit(f.revision.clone()),
            f.scope(),
        )
        .unwrap();
        assert_eq!(
            capture.packet,
            local::acquire(&f.root, REPO, &f.revision, f.scope()).unwrap()
        );
        assert_eq!(capture.packet.objects[1].disposition, disposition);
        assert_eq!(capture.packet.completeness, "partial");
        assert!(capture.packet.objects[1].content.is_none());
    }
    let f = Fixture::new(b"pub fn x() {}\n", "100644");
    let server = f.server("");
    let mut scope = f.scope();
    scope.context.push("missing".into());
    let capture = github::acquire(
        &mut server.transport(),
        REPO,
        Input::Commit(f.revision.clone()),
        scope,
    )
    .unwrap();
    assert_eq!(capture.packet.completeness, "partial");
    for path in [
        "../escape.rs",
        "/etc/passwd",
        "src/.git/config",
        "src/ghp_secret",
    ] {
        let mut scope = f.scope();
        scope.analysis = vec![path.into()];
        assert!(github::acquire(
            &mut server.transport(),
            REPO,
            Input::Commit(f.revision.clone()),
            scope
        )
        .is_err());
    }
    let mut scope = f.scope();
    scope.max_file_bytes = 1;
    assert_eq!(
        github::acquire(
            &mut server.transport(),
            REPO,
            Input::Commit(f.revision.clone()),
            scope
        )
        .err()
        .unwrap()
        .to_string(),
        "byte_limit_exceeded"
    );
    let symlink = Fixture::new(b"outside", "120000");
    let server = symlink.server("");
    assert_eq!(
        github::acquire(
            &mut server.transport(),
            REPO,
            Input::Commit(symlink.revision.clone()),
            symlink.scope()
        )
        .err()
        .unwrap()
        .to_string(),
        "symlink_input_rejected"
    );
}
#[test]
fn invalid_identity_and_fixture_endpoints_fail_before_transport() {
    for endpoint in [
        "http://example.com:80",
        "http://localhost:80",
        "https://127.0.0.1:80",
        "http://token@127.0.0.1:80",
        "http://127.0.0.1:80/path",
        "http://127.0.0.1:80?token=secret",
    ] {
        assert!(Transport::fixture(endpoint).is_err());
    }
    assert!(Transport::github(Some("bad\r\ncredential")).is_err());
    let f = Fixture::new(b"safe", "100644");
    let server = f.server("");
    for repo in [
        "https://user:ghp_secret@github.com/owner/repo",
        "https://github.com/owner/repo/extra",
        "https://other.example/owner/repo",
    ] {
        assert!(github::acquire(
            &mut server.transport(),
            repo,
            Input::Commit(f.revision.clone()),
            f.scope()
        )
        .is_err());
    }
    for revision in ["main".to_string(), "a".repeat(64), "A".repeat(40)] {
        assert!(github::acquire(
            &mut server.transport(),
            REPO,
            Input::Commit(revision),
            f.scope()
        )
        .is_err());
    }
    assert!(server.requests.lock().unwrap().is_empty());
}
#[test]
fn installed_command_executes_transport_and_reads_packet_without_credentials() {
    let f = Fixture::new(b"pub fn answer() -> u32 {42}\n", "100644");
    let server = f.server("");
    let scope = f._temp.path().join("scope.json");
    fs::write(&scope, serde_json::to_vec(&f.scope()).unwrap()).unwrap();
    let binary = std::env::var_os("ADL_CODEFRIEND_TEST_BINARY")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_adl")));
    for (kind, value) in [("--revision", f.revision.as_str()), ("--pr", "7")] {
        let out = f
            ._temp
            .path()
            .join(if kind == "--pr" { "pr" } else { "commit" });
        let result = Command::new(&binary)
            .args([
                "codefriend",
                "ingest",
                "github",
                "--repository",
                REPO,
                kind,
                value,
                "--scope",
            ])
            .arg(&scope)
            .arg("--out")
            .arg(&out)
            .args(["--fixture-api", &format!("http://{}", server.address)])
            .env("GITHUB_TOKEN", "ghp_fixture_must_not_be_sent")
            .env("ADL_GITHUB_TOKEN_FILE", "/nonexistent/fixture-credential")
            .env("ADL_OBSERVABILITY_OTEL", "0")
            .output()
            .unwrap();
        assert!(
            result.status.success(),
            "{}",
            String::from_utf8_lossy(&result.stderr)
        );
        let summary: Value = serde_json::from_slice(&result.stdout).unwrap();
        assert!(summary["provenance"]["requests"].as_u64().unwrap() > 0);
        assert!(!String::from_utf8_lossy(&result.stdout).contains("ghp_"));
        assert!(!String::from_utf8_lossy(&result.stderr).contains("ghp_"));
        let read = Command::new(&binary)
            .args(["codefriend", "packet", "read", "--input"])
            .arg(out.join("packet.json"))
            .output()
            .unwrap();
        assert!(read.status.success());
        let packet = AdmissionInput::read(&out.join("packet.json")).unwrap();
        assert_eq!(
            packet.packet(),
            &local::acquire(&f.root, REPO, &f.revision, f.scope()).unwrap()
        );
    }
}

#[test]
fn aggregate_metadata_is_bounded_across_individually_valid_responses() {
    let f = Fixture::new(b"safe", "100644");
    let server = f.server("aggregate_metadata");
    let error = github::acquire(
        &mut server.transport(),
        REPO,
        Input::PullRequest(7),
        f.scope(),
    )
    .err()
    .unwrap();
    assert_eq!(error.to_string(), "github_capture_byte_limit_exceeded");
    assert!(server.requests.lock().unwrap().len() >= 4);
}

#[test]
fn fork_pull_request_records_requested_and_actual_repository_separately() {
    let mut f = Fixture::new(b"safe", "100644");
    let fork = "https://github.com/fork/fixture";
    let originals = f.routes.clone();
    for (path, value) in originals {
        if path.contains("/git/") {
            f.routes
                .insert(path.replace("example/fixture", "fork/fixture"), value);
        }
    }
    f.routes.insert(
        "/repos/fork/fixture".into(),
        json!({"html_url":fork,"full_name":"fork/fixture"}),
    );
    f.routes.get_mut("/repos/example/fixture/pulls/7").unwrap()["head"]["repo"] =
        json!({"html_url":fork,"full_name":"fork/fixture"});
    git(&f.root, &["remote", "set-url", "origin", fork]);
    let server = f.server("");
    let mut capture = github::acquire(
        &mut server.transport(),
        REPO,
        Input::PullRequest(7),
        f.scope(),
    )
    .unwrap();
    assert_eq!(
        capture.packet,
        local::acquire(&f.root, fork, &f.revision, f.scope()).unwrap()
    );
    assert_eq!(capture.provenance.requested_repository, REPO);
    assert_eq!(capture.provenance.source_repository, fork);
    capture.provenance.resolved_commit = "a".repeat(40);
    assert!(capture.write(&f._temp.path().join("invalid")).is_err());
    assert!(!f._temp.path().join("invalid").exists());
}

#[test]
fn installed_transport_failure_and_compatibility_logs_never_capture_credentials() {
    let f = Fixture::new(b"safe", "100644");
    let server = f.server("invalid_json");
    let scope = f._temp.path().join("scope.json");
    fs::write(&scope, serde_json::to_vec(&f.scope()).unwrap()).unwrap();
    let log = f._temp.path().join("events.log");
    let out = f._temp.path().join("failed-capture");
    let binary = std::env::var_os("ADL_CODEFRIEND_TEST_BINARY")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(env!("CARGO_BIN_EXE_adl")));
    let result = Command::new(binary)
        .args([
            "codefriend",
            "ingest",
            "github",
            "--repository",
            REPO,
            "--revision",
            &f.revision,
            "--scope",
        ])
        .arg(&scope)
        .arg("--out")
        .arg(&out)
        .args(["--fixture-api", &format!("http://{}", server.address)])
        .env("GITHUB_TOKEN", "ghp_fixture_must_not_be_sent")
        .env("ADL_OBSERVABILITY_LOG", &log)
        .env("ADL_OBSERVABILITY_STDERR", "0")
        .output()
        .unwrap();
    assert!(!result.status.success());
    assert!(result.stdout.is_empty());
    assert!(!out.exists());
    let stderr = String::from_utf8_lossy(&result.stderr);
    assert!(stderr.contains("github_invalid_response"));
    assert!(!stderr.contains("ghp_"));
    let events = fs::read_to_string(log).unwrap();
    assert!(events.contains("adl_event"));
    assert!(!events.contains("ghp_"));
    assert!(!events.contains(&server.address));
}
