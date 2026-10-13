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
