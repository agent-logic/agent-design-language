use std::{process::Command, str};

const REMAINING_REPLACEMENT_COMMANDS: &[&str] = &[
    "bind",
    "clean",
    "cutover",
    "doctor",
    "edit",
    "eligibility",
    "finish",
    "github",
    "github-issue",
    "github-pr",
    "install",
    "issue",
    "pr-state",
    "proof",
    "publish",
    "review",
    "schedule",
    "shepherd",
    "soak",
];

const PARTIAL_CONSTRUCTION_COMMANDS: &[&str] = &["shadow", "validate"];

#[test]
fn help_exposes_one_binary_command_surface() {
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("--help")
        .output()
        .expect("csdlc help should run");
    assert!(output.status.success());
    let stdout = str::from_utf8(&output.stdout).expect("help stdout should be utf8");
    assert!(stdout.contains("usage: csdlc <command>"));
    assert!(stdout.contains("foundation --repo-root <path>"));
    assert!(stdout.contains("local --request <path> --registry <path> --registrations <path>"));
    for command in REMAINING_REPLACEMENT_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --help")),
            "help should expose {command}"
        );
    }
    for command in PARTIAL_CONSTRUCTION_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --help")),
            "help should expose partial construction route {command}"
        );
    }
}

#[test]
fn fail_closed_routes_do_not_claim_live_authority() {
    for command in REMAINING_REPLACEMENT_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe reserved route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: fail_closed"),
            "{command} help should be truthful: {help_stdout}"
        );
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .arg(command)
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} should run: {error}"));
        assert!(
            !output.status.success(),
            "{command} should fail closed before implementation"
        );
        let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
        assert!(stderr.contains("fail_closed"), "{command} stderr: {stderr}");
        assert!(
            stderr.contains("C-SDLC v3 is not live authority before #505 cutover"),
            "{command} stderr should preserve authority boundary: {stderr}"
        );
        assert!(
            !stderr.contains("csdlc-v2") && !stderr.contains("gh "),
            "{command} must not advertise v2/raw-gh fallback: {stderr}"
        );
    }
}

#[test]
fn partial_routes_remain_non_authoritative() {
    for command in PARTIAL_CONSTRUCTION_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe partial construction route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: partial"),
            "{command} help should be truthful: {help_stdout}"
        );
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .arg(command)
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} should run: {error}"));
        assert!(
            !output.status.success(),
            "{command} should not become live authority in #627"
        );
        let stderr = str::from_utf8(&output.stderr).expect("stderr should be utf8");
        assert!(stderr.contains("partial"), "{command} stderr: {stderr}");
        assert!(
            stderr.contains("C-SDLC v3 is not live authority before #505 cutover"),
            "{command} stderr should preserve authority boundary: {stderr}"
        );
    }
}
