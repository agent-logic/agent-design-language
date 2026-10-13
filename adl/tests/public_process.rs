//! PVF local_contract: retained public process-status CLI regressions.
//! Deterministic exact-PID/temporary-file/loopback fixtures, bounded local
//! resources, required public consumer gate; no broad process scan or provider.
use std::process::{Command, Output};

mod helpers;
use helpers::unique_test_temp_dir;

#[path = "cli_smoke/process_status.rs"]
mod process_status;

fn run_adl(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_adl"))
        .args(args)
        .output()
        .expect("run public ADL process command")
}

#[test]
fn standalone_process_entrypoint_preserves_success_and_refusal_channels() {
    let binary = env!("CARGO_BIN_EXE_adl-process");
    let help = Command::new(binary).arg("--help").output().unwrap();
    assert!(help.status.success());
    assert!(!help.stdout.is_empty());
    assert!(help.stderr.is_empty());

    let refused = Command::new(binary)
        .arg("unknown-operation")
        .output()
        .unwrap();
    assert_eq!(refused.status.code(), Some(1));
    assert!(refused.stdout.is_empty());
    assert!(String::from_utf8(refused.stderr)
        .unwrap()
        .contains("unknown process command 'unknown-operation'"));
}
