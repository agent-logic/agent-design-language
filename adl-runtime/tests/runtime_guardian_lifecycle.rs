use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant},
};

use adl_runtime::guardian::{run_guardian, GuardianConfig, GuardianTerminalState};
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

struct TestRoot(tempfile::TempDir);

impl TestRoot {
    fn new(name: &str) -> Self {
        let parent = std::env::var_os("ADL_TEST_TMPDIR")
            .map(PathBuf::from)
            .unwrap_or_else(|| std::env::temp_dir().join("adl-5820-tests"));
        fs::create_dir_all(&parent).expect("Guardian lifecycle test root");
        Self(
            tempfile::Builder::new()
                .prefix(name)
                .tempdir_in(parent)
                .expect("Guardian lifecycle temporary directory"),
        )
    }

    fn path(&self, name: &str) -> PathBuf {
        self.0.path().join(name)
    }
}

fn compile_child(root: &TestRoot) -> PathBuf {
    let source = root.path("guardian-child.rs");
    let binary = root.path(&format!("guardian-child{}", std::env::consts::EXE_SUFFIX));
    fs::write(
        &source,
        r#"
use std::{
    io::{Read, Write},
    net::TcpStream,
    path::PathBuf,
    time::Duration,
};

fn main() {
    let state = PathBuf::from(std::env::args().nth(1).expect("state path"));
    let ready = PathBuf::from(std::env::args().nth(2).expect("ready path"));
    let mode = std::env::args().nth(3).expect("mode");
    if mode == "configuration-exit" {
        std::process::exit(64);
    }

    let generation = std::fs::read_to_string(&state)
        .ok()
        .and_then(|value| value.trim().parse::<u32>().ok())
        .unwrap_or(0)
        + 1;
    std::fs::write(&state, generation.to_string()).expect("durable generation");

    let address = std::env::var("ADL_RUNTIME_GUARDIAN_LEASE_ADDRESS").expect("lease address");
    let token = std::env::var("ADL_RUNTIME_GUARDIAN_LEASE_TOKEN").expect("lease token");
    let mut lease = (0..100)
        .find_map(|_| match TcpStream::connect(&address) {
            Ok(stream) => Some(stream),
            Err(_) => {
                std::thread::sleep(Duration::from_millis(5));
                None
            }
        })
        .expect("lease connection");
    lease.write_all(token.as_bytes()).expect("lease token write");
    let mut acknowledgement = [0_u8; 2];
    lease.read_exact(&mut acknowledgement).expect("lease acknowledgement");
    assert_eq!(&acknowledgement, b"ok");

    if generation == 1 {
        eprintln!("dependency_state=degraded reason=optional_provider_unavailable");
        std::process::exit(7);
    }

    std::fs::write(&ready, format!("generation={generation}"))
        .expect("readiness marker");
    let mut closed = [0_u8; 1];
    assert_eq!(lease.read(&mut closed).expect("lease close"), 0);
}
"#,
    )
    .expect("child source");
    let status = Command::new("rustc")
        .arg(&source)
        .arg("-o")
        .arg(&binary)
        .status()
        .expect("rustc must execute");
    assert!(status.success(), "portable child must compile");
    binary
}

fn guardian_config(program: &Path, args: Vec<String>) -> GuardianConfig {
    GuardianConfig {
        program: program.to_path_buf(),
        args,
        env: Vec::new(),
        restart_budget: 2,
        backoff_base_ms: 5,
        backoff_cap_ms: 20,
        healthy_window_ms: 5_000,
        child_shutdown_budget_ms: 500,
        shutdown_grace_ms: 1_000,
        lease_auth_timeout_ms: 500,
        lease_auth_attempts: 3,
        capture_max_bytes: 65_536,
        capture_drain_grace_ms: 250,
        configuration_exit_codes: vec![64],
    }
}

#[tokio::test]
async fn guardian_restarts_failed_child_preserves_state_and_shuts_down_cleanly() {
    let root = TestRoot::new("guardian-restart-");
    let child = compile_child(&root);
    let state = root.path("durable-generation");
    let ready = root.path("ready");
    let config = guardian_config(
        &child,
        vec![
            state.to_string_lossy().into_owned(),
            ready.to_string_lossy().into_owned(),
            "recover".to_owned(),
        ],
    );
    let shutdown = CancellationToken::new();
    let cancel = shutdown.clone();
    let guardian = tokio::spawn(run_guardian(config, shutdown));

    let deadline = Instant::now() + Duration::from_secs(5);
    while !ready.exists() && Instant::now() < deadline {
        sleep(Duration::from_millis(10)).await;
    }
    assert_eq!(fs::read_to_string(&ready).unwrap(), "generation=2");
    cancel.cancel();

    let outcome = guardian.await.unwrap().unwrap();
    assert_eq!(
        outcome.terminal_state,
        GuardianTerminalState::ShutdownCheckpointed
    );
    assert_eq!(outcome.attempts, 2);
    assert_eq!(outcome.restarts, 1);
    assert_eq!(fs::read_to_string(state).unwrap(), "2");
    assert!(outcome.attempts_detail[0]
        .stderr
        .contains("dependency_state=degraded"));
    assert!(outcome.attempts_detail[1].clean_checkpointed_shutdown);
    assert!(!outcome.attempts_detail[1].forced_shutdown);
}

#[tokio::test]
async fn guardian_never_restarts_a_configuration_exit() {
    let root = TestRoot::new("guardian-configuration-exit-");
    let child = compile_child(&root);
    let config = guardian_config(
        &child,
        vec![
            root.path("state").to_string_lossy().into_owned(),
            root.path("ready").to_string_lossy().into_owned(),
            "configuration-exit".to_owned(),
        ],
    );

    let outcome = run_guardian(config, CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(
        outcome.terminal_state,
        GuardianTerminalState::ConfigurationExit
    );
    assert_eq!(outcome.attempts, 1);
    assert_eq!(outcome.restarts, 0);
    assert_eq!(outcome.last_reason(), Some("configuration_exit"));
}

#[test]
fn guardian_policy_rejects_unbounded_or_inverted_limits() {
    let root = TestRoot::new("guardian-policy-");
    let child = compile_child(&root);
    let base = || guardian_config(&child, Vec::new());

    let mut zero_backoff = base();
    zero_backoff.backoff_base_ms = 0;
    assert!(zero_backoff.validate().is_err());

    let mut inverted_backoff = base();
    inverted_backoff.backoff_cap_ms = inverted_backoff.backoff_base_ms - 1;
    assert!(inverted_backoff.validate().is_err());

    let mut unbounded_capture = base();
    unbounded_capture.capture_max_bytes = 0;
    assert!(unbounded_capture.validate().is_err());

    let mut invalid_shutdown = base();
    invalid_shutdown.shutdown_grace_ms = invalid_shutdown.child_shutdown_budget_ms;
    assert!(invalid_shutdown.validate().is_err());

    let mut restart_budget = base();
    restart_budget.restart_budget = 10_001;
    assert!(restart_budget.validate().is_err());

    let mut lease_attempts = base();
    lease_attempts.lease_auth_attempts = 33;
    assert!(lease_attempts.validate().is_err());

    let mut capture_limit = base();
    capture_limit.capture_max_bytes = 1024 * 1024 * 1024 + 1;
    assert!(capture_limit.validate().is_err());

    for codes in [vec![], vec![0], vec![-1], vec![64, 64]] {
        let mut invalid_codes = base();
        invalid_codes.configuration_exit_codes = codes;
        assert!(invalid_codes.validate().is_err());
    }
}
