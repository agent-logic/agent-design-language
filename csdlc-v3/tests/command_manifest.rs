use std::{process::Command, str};

const IMPLEMENTED_LOCAL_COMMANDS: &[&str] = &[
    "issue",
    "bind",
    "edit",
    "validate",
    "doctor",
    "schedule",
    "shepherd",
    "eligibility",
];

const IMPLEMENTED_REMOTE_PUBLICATION_COMMANDS: &[&str] = &[
    "github",
    "github-issue",
    "github-pr",
    "pr-state",
    "publish",
    "review",
];

const IMPLEMENTED_TERMINAL_COMMANDS: &[&str] = &["clean", "cutover", "finish"];

const IMPLEMENTED_CONSTRUCTION_COMMANDS: &[&str] = &["install", "proof", "shadow", "soak"];

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
    for command in IMPLEMENTED_LOCAL_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --request <path>")),
            "help should expose implemented local route {command}"
        );
    }
    for command in IMPLEMENTED_REMOTE_PUBLICATION_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --request <path>")),
            "help should expose implemented remote/publication route {command}"
        );
    }
    for command in IMPLEMENTED_TERMINAL_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --request <path>")),
            "help should expose implemented terminal route {command}"
        );
    }
    for command in IMPLEMENTED_CONSTRUCTION_COMMANDS {
        assert!(
            stdout.contains(&format!("{command} --request <path>")),
            "help should expose implemented construction route {command}"
        );
    }
}

#[test]
fn implemented_local_routes_expose_non_authoritative_help() {
    for command in IMPLEMENTED_LOCAL_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented local route"
        );
        assert_implemented_help(command, &help.stdout);
    }
}

#[test]
fn implemented_remote_publication_routes_expose_non_authoritative_help() {
    for command in IMPLEMENTED_REMOTE_PUBLICATION_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented remote route"
        );
        assert_implemented_help(command, &help.stdout);
    }
}

#[test]
fn implemented_terminal_routes_expose_non_authoritative_help() {
    for command in IMPLEMENTED_TERMINAL_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented terminal route"
        );
        assert_implemented_help(command, &help.stdout);
    }
}

fn assert_implemented_help(command: &str, stdout: &[u8]) {
    let help_stdout = str::from_utf8(stdout).expect("help stdout should be utf8");
    assert!(
        help_stdout.contains("status: implemented"),
        "{command} help should be truthful: {help_stdout}"
    );
    assert!(
        help_stdout.contains("C-SDLC v3 is not live authority before #505 cutover"),
        "{command} help should preserve authority boundary: {help_stdout}"
    );
}

#[test]
fn issue_631_routes_are_implemented_construction_not_live_authority() {
    for command in IMPLEMENTED_CONSTRUCTION_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap_or_else(|error| panic!("csdlc {command} --help should run: {error}"));
        assert!(
            help.status.success(),
            "{command} --help should describe implemented construction route"
        );
        let help_stdout = str::from_utf8(&help.stdout).expect("help stdout should be utf8");
        assert!(
            help_stdout.contains("status: implemented_construction"),
            "{command} help should be truthful: {help_stdout}"
        );
        assert!(
            help_stdout.contains("C-SDLC v3 is not live authority before #505 cutover"),
            "{command} help should preserve authority boundary: {help_stdout}"
        );
    }
}
