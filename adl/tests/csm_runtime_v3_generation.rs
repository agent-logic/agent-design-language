//! PVF runtime service-manager lane: deterministic CLI contract checks with no
//! service mutation, network access, or paid inference. Behavioral sequencing
//! remains covered by the colocated `csm_runtime_v3_cmd` unit tests.

use std::process::Command;

fn csm() -> Command {
    Command::new(env!("CARGO_BIN_EXE_csm"))
}

#[test]
fn runtime_v3_service_help_exposes_the_managed_lifecycle() {
    let output = csm()
        .args(["runtime-v3", "help"])
        .output()
        .expect("run csm runtime-v3 help");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("help is UTF-8");
    assert!(stdout.contains("start|stop|status|reload"));
    assert!(stdout.contains("--init <absolute-runtime-init.toml>"));
}

#[test]
fn runtime_v3_start_rejects_reload_candidate_before_service_mutation() {
    let output = csm()
        .args([
            "runtime-v3",
            "start",
            "--init",
            "/nonexistent/runtime-init.toml",
            "--candidate",
            "/nonexistent/runtime-init.next.toml",
        ])
        .output()
        .expect("run csm runtime-v3 start preflight");
    assert!(!output.status.success());
    let stderr = String::from_utf8(output.stderr).expect("diagnostic is UTF-8");
    assert!(stderr.contains("--candidate is valid only with runtime-v3 reload"));
}
