use std::{fs, path::PathBuf, process::Command, str};

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
const IMPLEMENTED_HELPER_COMMANDS: &[&str] = &["remote", "sprint"];
const IMPLEMENTED_STATUSES: &[&str] = &[
    "implemented",
    "implemented_construction",
    "implemented_pre_cutover_bridge",
];

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
    assert!(stdout.contains("remote --help"));
    assert!(stdout.contains("sprint --repo-root <path> --request <path>"));
}

#[test]
fn tracked_command_denominators_match_cli_surface_and_cutover_boundary() {
    let root = repo_root();
    let manifest: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/csdlc-v3/v3-command-manifest.json"))
            .expect("command manifest"),
    )
    .expect("command manifest json");
    let denominator: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(root.join("docs/csdlc-v3/full-replacement-denominator.json"))
            .expect("full replacement denominator"),
    )
    .expect("full replacement denominator json");

    assert_eq!(manifest["one_binary"], "csdlc");
    assert_eq!(manifest["operational_authority"], false);
    assert_eq!(denominator["cutover_ready"], false);
    assert_eq!(
        denominator["status"],
        "pre_cutover_implemented_pending_authority_evidence"
    );
    assert_eq!(manifest["denominator"]["v2_entrypoints"], 21);
    assert_eq!(manifest["denominator"]["current_v3_commands"], 25);
    assert_eq!(manifest["denominator"]["implemented_commands"], 25);
    assert_eq!(manifest["denominator"]["partial_commands"], 0);
    assert_eq!(manifest["denominator"]["fail_closed_commands"], 0);
    assert_eq!(
        manifest["denominator"]["implemented_replacement_routes"],
        21
    );
    assert_eq!(manifest["denominator"]["partial_replacement_routes"], 0);
    assert_eq!(manifest["denominator"]["fail_closed_replacement_routes"], 0);
    assert_eq!(manifest["denominator"]["remaining_replacement_routes"], 0);

    let commands = manifest["commands"].as_array().expect("manifest commands");
    assert_eq!(commands.len(), 25);
    assert_eq!(implemented_status_count(commands), 25);
    assert_eq!(status_count(commands, "partial"), 0);
    assert_eq!(status_count(commands, "fail_closed"), 0);

    for command in IMPLEMENTED_LOCAL_COMMANDS
        .iter()
        .chain(IMPLEMENTED_REMOTE_PUBLICATION_COMMANDS)
        .chain(IMPLEMENTED_TERMINAL_COMMANDS)
    {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented");
        assert_eq!(row["authority_status"], "not_live");
    }
    for command in IMPLEMENTED_CONSTRUCTION_COMMANDS {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented_construction");
        assert_eq!(row["authority_status"], "not_live");
    }
    for command in IMPLEMENTED_HELPER_COMMANDS {
        let row = command_row(commands, command);
        assert!(
            row["implementation_status"]
                .as_str()
                .is_some_and(|status| IMPLEMENTED_STATUSES.contains(&status)),
            "{command} should be implemented before cutover"
        );
        assert!(
            matches!(
                row["authority_status"].as_str(),
                Some("not_live" | "not_live_helper" | "read_only_construction")
            ),
            "{command} should not be live authority before #505"
        );
        assert!(row["replaces"].as_array().expect("replaces").is_empty());
    }

    let current = denominator["current_v3_commands"]
        .as_array()
        .expect("current v3 commands")
        .iter()
        .map(|value| value.as_str().expect("command").to_owned())
        .collect::<Vec<_>>();
    for command in commands {
        let name = command["command"].as_str().expect("command name");
        assert!(
            current.iter().any(|current| current == name),
            "denominator should include manifest command {name}"
        );
    }

    let replacements = denominator["required_v2_entrypoints"]
        .as_array()
        .expect("required v2 entrypoints");
    assert_eq!(replacements.len(), 21);
    assert_eq!(implemented_status_count_for_replacements(replacements), 21);
    assert_eq!(
        replacements
            .iter()
            .filter(|row| row["replacement_status"] == "fail_closed")
            .count(),
        0
    );
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

fn status_count(commands: &[serde_json::Value], status: &str) -> usize {
    commands
        .iter()
        .filter(|command| command["implementation_status"] == status)
        .count()
}

fn implemented_status_count(commands: &[serde_json::Value]) -> usize {
    commands
        .iter()
        .filter(|command| {
            command["implementation_status"]
                .as_str()
                .is_some_and(|status| IMPLEMENTED_STATUSES.contains(&status))
        })
        .count()
}

fn implemented_status_count_for_replacements(replacements: &[serde_json::Value]) -> usize {
    replacements
        .iter()
        .filter(|row| {
            row["replacement_status"]
                .as_str()
                .is_some_and(|status| IMPLEMENTED_STATUSES.contains(&status))
        })
        .count()
}

fn command_row<'a>(commands: &'a [serde_json::Value], command: &str) -> &'a serde_json::Value {
    commands
        .iter()
        .find(|row| row["command"].as_str() == Some(command))
        .unwrap_or_else(|| panic!("missing manifest row for {command}"))
}

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root")
        .to_path_buf()
}
