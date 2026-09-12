//! PVF: required installed integration regression for #869 validator supervision.
//! Deterministic local Git/Cargo fixture; bounded CPU/filesystem/time and local loopback sockets, no external network.
//! Tests deliberately leave an owned descendant holding stdout/stderr and a local
//! loopback listener, then require timeout/cancellation to close those inherited handles.
#![cfg(unix)]
use serde_json::{json, Value};
use std::{
    fs,
    os::unix::process::CommandExt,
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
#[allow(dead_code)]
#[path = "support/intent_fixture.rs"]
mod intent_fixture;
use intent_fixture::{git, Fixture};

const CRATE_SOURCE: &str = r#"
#[cfg(test)]
mod tests {
    fn evidence()->std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap().join(".csdlc/evidence/505")
    }
    #[test]
    fn retained_output_descendant() {
        use std::process::{Command,Stdio};
        std::fs::create_dir_all(evidence()).unwrap();
        let child=Command::new(std::env::current_exe().unwrap())
            .args(["--exact","tests::sleeping_descendant","--ignored","--nocapture"])
            .stdin(Stdio::null()).stdout(Stdio::inherit()).stderr(Stdio::inherit()).spawn().unwrap();
        std::fs::write(evidence().join("timeout-child.pid"),child.id().to_string()).unwrap();
        // The test and Cargo may exit successfully; the descendant still owns
        // their inherited output handles. An immediate-child timeout is insufficient.
    }
    #[test]
    #[ignore]
    fn sleeping_descendant() {
        std::fs::create_dir_all(evidence()).unwrap();
        let socket=std::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST,0)).unwrap();
        std::fs::write(evidence().join("timeout-ready"),socket.local_addr().unwrap().port().to_string()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(30));
        drop(socket);
    }
}
"#;

fn prepare(timeout: u64, label: &str) -> (Fixture, PathBuf) {
    let mut fixture = Fixture::new(label);
    let plan = json!({"schema":"csdlc.v3.intent_plan.v1","slug":"installed-timeout",
        "cards":{"sip":{},"stp":{},"spp":{},"vpp":{},"srp":{},"sor":{}},
        "validators":[{"id":"owned-process-proof","program":"cargo","args":["test","--offline","--manifest-path","fixture-proof/Cargo.toml"],"success_marker":"test result: ok.","timeout_seconds":timeout}],
        "publication":{"base":"main","title":"Timeout fixture","body":"Closes #505","draft":true}});
    let plan_path = fixture.write_json("timeout-plan.json", &plan);
    let primary = fixture.root.clone();
    let prepared = fixture.run(
        &primary,
        &["prepare", "505", "--plan", plan_path.to_str().unwrap()],
    );
    assert!(prepared.status.success(), "prepare: {prepared:?}");
    let bound = fixture.run(&primary, &["bind", "505"]);
    assert!(bound.status.success(), "bind: {bound:?}");
    let linked = git(&primary, &["worktree", "list", "--porcelain"])
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
        .map(PathBuf::from)
        .find(|path| path != &primary)
        .unwrap();
    fs::write(linked.join("fixture-proof/src/lib.rs"), CRATE_SOURCE).unwrap();
    git(&linked, &["add", "fixture-proof/src/lib.rs"]);
    git(
        &linked,
        &[
            "commit",
            "--quiet",
            "-m",
            "tracked inherited-output fixture",
        ],
    );
    // Compilation is fixture setup, outside the production one-second timeout.
    let warm = Command::new("cargo")
        .current_dir(&linked)
        .args([
            "test",
            "--offline",
            "--manifest-path",
            "fixture-proof/Cargo.toml",
            "--no-run",
        ])
        .env("CARGO_TARGET_DIR", linked.join("target/intent-validation"))
        .output()
        .unwrap();
    assert!(warm.status.success(), "fixture warmup failed: {warm:?}");
    (fixture, linked)
}

fn listener_live(evidence: &Path) -> bool {
    let port = fs::read_to_string(evidence.join("timeout-ready"))
        .ok()
        .and_then(|text| text.parse::<u16>().ok());
    port.is_some_and(|port| {
        std::net::TcpStream::connect_timeout(
            &std::net::SocketAddr::from((std::net::Ipv4Addr::LOCALHOST, port)),
            Duration::from_millis(100),
        )
        .is_ok()
    })
}

unsafe extern "C" {
    fn kill(pid: i32, signal: i32) -> i32;
    fn getpgid(pid: i32) -> i32;
    fn getpgrp() -> i32;
}
struct OwnedInstalled {
    child: Child,
    descendant_pid: PathBuf,
}
impl Drop for OwnedInstalled {
    fn drop(&mut self) {
        // Failure cleanup is scoped to a PID written by this tracked fixture and
        // the exact installed process group created by this test, never a scan.
        if listener_live(self.descendant_pid.parent().unwrap()) {
            if let Ok(pid) = fs::read_to_string(&self.descendant_pid)
                .and_then(|text| text.trim().parse::<i32>().map_err(std::io::Error::other))
            {
                let group = unsafe { getpgid(pid) };
                if group > 0 && group != unsafe { getpgrp() } {
                    unsafe { kill(-group, 9) };
                }
            }
        }
        if self.child.try_wait().ok().flatten().is_none() {
            unsafe { kill(self.child.id() as i32, 15) };
            let deadline = Instant::now();
            while self.child.try_wait().ok().flatten().is_none()
                && deadline.elapsed() < Duration::from_secs(1)
            {
                thread::sleep(Duration::from_millis(10));
            }
            if self.child.try_wait().ok().flatten().is_none() {
                unsafe { kill(-(self.child.id() as i32), 9) };
                let _ = self.child.kill();
                let _ = self.child.try_wait();
            }
        }
    }
}

fn exercise(timeout: u64, cancel: bool) {
    let (fixture, linked) = prepare(
        timeout,
        if cancel {
            "cancel-descendant"
        } else {
            "timeout-descendant"
        },
    );
    let evidence = linked.join(".csdlc/evidence/505");
    let stdout = fixture
        .root
        .join(".git/installed-candidate/timeout.stdout.json");
    let stderr = fixture
        .root
        .join(".git/installed-candidate/timeout.stderr.txt");
    let started = Instant::now();
    let child = Command::new(&fixture.binary)
        .current_dir(&linked)
        .args(["proof", "505"])
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env(
            "PATH",
            format!(
                "{}:{}",
                fixture
                    .root
                    .join(".git/installed-candidate/fake-bin")
                    .display(),
                std::env::var("PATH").unwrap()
            ),
        )
        .env(
            "ADL_GITHUB_TOKEN_FILE",
            fixture.root.join(".git/installed-candidate/token"),
        )
        .env("GITHUB_TOKEN", "SIM03_SYNTHETIC_TOKEN")
        .env_remove("GH_TOKEN")
        .env_remove("CARGO_TARGET_DIR")
        .env_remove("CARGO_BUILD_TARGET_DIR")
        .stdin(Stdio::null())
        .stdout(fs::File::create(&stdout).unwrap())
        .stderr(fs::File::create(&stderr).unwrap())
        .process_group(0)
        .spawn()
        .unwrap();
    let mut owned = OwnedInstalled {
        child,
        descendant_pid: evidence.join("timeout-child.pid"),
    };
    if cancel {
        while !evidence.join("timeout-ready").exists() && started.elapsed() < Duration::from_secs(8)
        {
            assert!(
                owned.child.try_wait().unwrap().is_none(),
                "proof exited before cancellation fixture became live: {}",
                fs::read_to_string(&stdout).unwrap()
            );
            thread::sleep(Duration::from_millis(10));
        }
        assert!(
            evidence.join("timeout-ready").exists(),
            "owned descendant never became live"
        );
        assert!(
            listener_live(&evidence),
            "descendant did not retain live socket before cancellation"
        );
        assert_eq!(
            unsafe { kill(owned.child.id() as i32, 15) },
            0,
            "targeted SIGTERM failed"
        );
    }
    let status = loop {
        if let Some(status) = owned.child.try_wait().unwrap() {
            break status;
        }
        assert!(
            started.elapsed() < Duration::from_secs(8),
            "installed proof hung beyond its bounded supervision window"
        );
        thread::sleep(Duration::from_millis(10));
    };
    let wall = started.elapsed();
    assert!(!status.success(), "timed-out/cancelled proof must fail");
    assert!(
        evidence.join("timeout-ready").exists(),
        "test must exercise a real live descendant, not compilation timeout"
    );
    assert!(
        !listener_live(&evidence),
        "owned descendant retained its live socket after supervisor returned"
    );
    let result: Value = serde_json::from_slice(&fs::read(&stdout).unwrap()).unwrap();
    let receipt: Value =
        serde_json::from_slice(&fs::read(evidence.join("intent-proof.json")).unwrap()).unwrap();
    assert_eq!(receipt["status"], "failed");
    assert_eq!(result["envelope"]["effects"]["outcome"], "performed");
    let validator = &receipt["validators"][0];
    assert_eq!(validator["timeout_seconds"], timeout);
    assert_eq!(validator["timed_out"], !cancel);
    assert_eq!(validator["cancelled"], cancel);
    assert_eq!(
        validator["cleanup_complete"], true,
        "output handles must have reached EOF and child must be reaped: {validator}"
    );
    assert_eq!(validator["passed"], false);
    let elapsed = validator["elapsed_ms"].as_u64().unwrap();
    assert!(
        elapsed < 5_000 && u128::from(elapsed) <= wall.as_millis(),
        "receipt must report actual bounded supervisor time: receipt={elapsed}, wall={wall:?}"
    );
    if !cancel {
        assert!(
            elapsed >= 1_000,
            "timeout cannot be reported before its deadline"
        );
    }
    assert!(!fs::read_to_string(&stdout)
        .unwrap()
        .contains("SIM03_SYNTHETIC_TOKEN"));
    assert!(!fs::read_to_string(&stderr)
        .unwrap()
        .contains("SIM03_SYNTHETIC_TOKEN"));
    let source = Path::new(env!("CARGO_MANIFEST_DIR"));
    let report = source.join("target/sim03-intent-corpus").join(if cancel {
        "installed-cancellation.json"
    } else {
        "installed-timeout.json"
    });
    fs::create_dir_all(report.parent().unwrap()).unwrap();
    fs::write(report,serde_json::to_vec_pretty(&json!({"schema":"csdlc.v3.installed_timeout_proof.v1","issue":869,
        "installed_binary_blake3":blake3::hash(&fs::read(&fixture.binary).unwrap()).to_hex().to_string(),
        "case":if cancel{"scoped_sigterm_cancellation"}else{"exited_cargo_descendant_holds_outputs"},
        "wall_elapsed_ms":wall.as_millis(),"validator":validator,"live_socket_after":false})).unwrap()).unwrap();
}

#[test]
fn installed_timeout_terminates_descendant_retaining_output_after_cargo_exits() {
    exercise(1, false);
}
#[test]
fn installed_sigterm_cancels_owned_group_and_records_distinct_failed_proof() {
    exercise(30, true);
}
