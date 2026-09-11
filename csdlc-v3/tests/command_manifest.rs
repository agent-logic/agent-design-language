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
    assert!(stdout.contains("authenticated canonical selector"));
    assert!(stdout.contains("Missing or stale proof suspends authority"));
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
    assert!(stdout.contains("github-issue --request <path> [--observe-github] [--execute]"));
    assert!(stdout.contains("github-pr --request <path> [--observe-github] [--execute]"));
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
    assert_eq!(manifest["operational_authority"], true);
    assert_eq!(manifest["status"], "post_cutover_operational");
    // The full replacement denominator below is immutable pre-cutover evidence.
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
        assert_eq!(row["authority_status"], "authenticated_v3");
    }
    for command in IMPLEMENTED_CONSTRUCTION_COMMANDS {
        let row = command_row(commands, command);
        assert_eq!(row["implementation_status"], "implemented_construction");
        assert_eq!(row["authority_status"], "proof_only");
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
            matches!(row["authority_status"].as_str(), Some("read_only_helper")),
            "{command} must not grant mutation authority"
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
fn implemented_local_routes_expose_guarded_operational_help() {
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
fn implemented_remote_publication_routes_expose_guarded_operational_help() {
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
fn implemented_terminal_routes_expose_guarded_operational_help() {
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
        help_stdout.contains("C-SDLC v3 is operational after #505 / PR #591"),
        "{command} help should preserve authority boundary: {help_stdout}"
    );
}

#[test]
fn proof_routes_distinguish_operational_and_historical_execution() {
    for command in IMPLEMENTED_CONSTRUCTION_COMMANDS {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .unwrap();
        assert!(help.status.success());
        let expected = if matches!(*command, "shadow" | "soak") {
            "historical; execution disabled"
        } else {
            "operational; authenticated bound issue worktree required"
        };
        assert!(str::from_utf8(&help.stdout).unwrap().contains(expected));
        assert!(str::from_utf8(&help.stdout)
            .unwrap()
            .contains("C-SDLC v3 is operational after #505 / PR #591"));
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

// PVF: deterministic local contract; small CPU/Git/subprocess; release-gating
// for authority guidance. This checks current routing, not historical fixtures.
fn current_authority_text_is_consistent(text: &str) -> bool {
    let normalized = text
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let obsolete = [
        "v3 is not live authority before #505",
        "v2 remains live authority",
        "v2 remains the live lifecycle authority",
        "live route still remains typed v2",
        "#505 is the pending",
        "non-authoritative c-sdlc v3 contract boundary",
        "keep using v2",
    ];
    normalized.contains("authenticated")
        && normalized.contains("selector")
        && normalized.contains("receipt")
        && !obsolete.iter().any(|phrase| normalized.contains(phrase))
}

fn ordinary_lifecycle_guidance_is_unambiguous(text: &str) -> bool {
    let normalized = text.to_lowercase();
    let stale_current_claim = [
        "sole current c-sdlc operational authority is the independent rust binary set in `csdlc-v2/`",
        "canonical under gate 10d2",
        "resolve the sole active c-sdlc generation",
        "resolve the active generation with `.adl/bin/csdlc-v2/",
        "workflow commands should resolve through `.adl/bin/csdlc-v2/`",
    ]
    .iter()
    .any(|needle| normalized.contains(needle));
    if stale_current_claim {
        return false;
    }
    let names_v2_route = [
        ".adl/bin/csdlc-v2/",
        "csdlc-install resolve",
        "csdlc-doctor",
        "csdlc-issue",
        "csdlc-bind",
    ]
    .iter()
    .any(|needle| normalized.contains(needle));
    if !names_v2_route {
        return true;
    }
    let document_scoped_exception = normalized
        .lines()
        .take(12)
        .any(|line| line.contains("retained exception path only"));
    if document_scoped_exception {
        return true;
    }
    let lines = normalized.lines().collect::<Vec<_>>();
    lines.iter().enumerate().all(|(index, line)| {
        let names_route = [
            ".adl/bin/csdlc-v2/",
            "csdlc-install resolve",
            "csdlc-doctor",
            "csdlc-issue",
            "csdlc-bind",
        ]
        .iter()
        .any(|needle| line.contains(needle));
        if !names_route {
            return true;
        }
        let start = index.saturating_sub(8);
        let end = (index + 2).min(lines.len().saturating_sub(1));
        let context = lines[start..=end].join(" ");
        context.contains("explicitly authorized rollback")
            || context.contains("explicit operator-authorized rollback")
            || context.contains("bounded transition remediation")
            || context.contains("bounded transition-remediation")
            || context.contains("historical evidence")
    })
}

#[test]
fn current_surfaces_agree_with_authenticated_post_cutover_authority() {
    let root = repo_root();
    let authority = csdlc_v3::authority::canonical_v3_authority(&root)
        .expect("canonical selector and authenticated receipt must validate")
        .expect("this post-cutover repository requires native v3 authority");
    assert_eq!(authority.operational_authority, "csdlc-v3");
    for path in [
        "AGENTS.md",
        "csdlc-v3/AGENTS.md",
        "csdlc-v3/README.md",
        "docs/default_workflow.md",
        "docs/tooling/card-lifecycle.md",
        "docs/tooling/structured-prompt-contracts.md",
        "docs/csdlc-v3/CURRENT_AUTHORITY.md",
        "docs/templates/prompts/current.json",
        "docs/csdlc-v3/v3-command-manifest.json",
        "csdlc-v3/src/lib.rs",
        "csdlc-v3/src/commands/local/mod.rs",
        "csdlc-v3/src/commands/mod.rs",
        "csdlc-v3/src/commands/sprint.rs",
        "csdlc-v3/Cargo.toml",
    ] {
        let text = fs::read_to_string(root.join(path)).expect("current authority surface");
        assert!(
            current_authority_text_is_consistent(&text),
            "obsolete routing in {path}"
        );
    }
    for command in IMPLEMENTED_LOCAL_COMMANDS
        .iter()
        .chain(IMPLEMENTED_REMOTE_PUBLICATION_COMMANDS)
        .chain(IMPLEMENTED_TERMINAL_COMMANDS)
        .chain(IMPLEMENTED_CONSTRUCTION_COMMANDS)
    {
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args([command, "--help"])
            .output()
            .expect("actual CLI help");
        assert!(output.status.success());
        let help = str::from_utf8(&output.stdout).unwrap();
        assert!(
            current_authority_text_is_consistent(help),
            "obsolete {command} help"
        );
        assert!(help.contains("authenticated canonical selector"));
        assert!(help.contains("Missing or stale proof suspends authority"));
    }
}

#[test]
fn authority_fitness_rejects_obsolete_policy_help_and_automatic_v2_fallback() {
    let guarded =
        "C-SDLC v3 is operational; authenticated canonical selector and receipt are required.";
    assert!(current_authority_text_is_consistent(guarded));
    assert!(!current_authority_text_is_consistent(
        "C-SDLC v3 is operational unconditionally."
    ));
    for text in [
        "C-SDLC v2 remains the live lifecycle authority.",
        "authority: C-SDLC v3 is not live authority before #505 cutover.",
        "If proof is absent, keep using v2.",
        "V3-F/#505 is the pending tooling changeover decision.",
    ] {
        assert!(
            !current_authority_text_is_consistent(&format!("{guarded} {text}")),
            "missed contradiction despite valid guard: {text}"
        );
    }
    assert!(current_authority_text_is_consistent(
        "C-SDLC v3 is operational; authenticated canonical selector and receipt are required."
    ));
}

// PVF: review_docs; release-gating deterministic local contract; small CPU and
// filesystem profile; validates TPR-005 without network or cloud resources.
#[test]
fn active_boot_path_inventory_is_complete_source_backed_and_unambiguous() {
    let root = repo_root();
    let inventory_path = root.join("docs/tooling/ACTIVE_BOOT_PATHS.md");
    let inventory = fs::read_to_string(&inventory_path).expect("active boot-path inventory");
    assert_eq!(
        inventory
            .matches("ordinary_lifecycle_entrypoint: .adl/bin/native-v3/csdlc")
            .count(),
        1,
        "inventory must declare exactly one ordinary lifecycle entrypoint"
    );
    for required in [
        "csdlc-v3/operator/authority-selector.json",
        "csdlc-v3/src/main.rs",
        "adl/src/main.rs",
        "adl/src/bin/csm.rs",
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        "adl-runtime/src/bin/adl-runtime-guardian.rs",
        "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs",
        "retained rollback/transition-only source",
        "product/runtime control surface, not C-SDLC authority",
        "canonical stable install destination: `.adl/bin/csdlc`",
    ] {
        assert!(inventory.contains(required), "inventory missing {required}");
    }
    for source in [
        "csdlc-v3/operator/authority-selector.json",
        "csdlc-v3/src/main.rs",
        "adl/src/main.rs",
        "adl/src/bin/csm.rs",
        "adl/src/cli/csm_runtime_v3_cmd.rs",
        "adl-runtime/src/bin/adl-runtime-guardian.rs",
        "adl-runtime-kernel/src/bin/adl-runtime-kernel.rs",
    ] {
        assert!(
            root.join(source).exists(),
            "inventory source missing: {source}"
        );
    }
    for current_doc in [
        "docs/tooling/ACTIVE_BOOT_PATHS.md",
        "docs/tooling/OWNER_BINARY_INSTALLATION.md",
        "docs/tooling/ADL_PLATFORM_CLI_BINARY_TAXONOMY.md",
        "docs/tooling/README.md",
        "docs/tooling/structured-prompt-contracts.md",
        "docs/tooling/structured-prompt-validator-binary-resolution.md",
        "docs/tooling/card-lifecycle.md",
        "docs/tooling/C_SDLC_V2_ISSUE_CREATION_AND_BINDING_RUNBOOK.md",
        "adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md",
        "docs/tooling/editor/command_adapter.md",
        "docs/tooling/editor/current_skill_wiring_demo.md",
        "adl/tools/README.md",
        "adl/tools/editor_action.sh",
    ] {
        let text = fs::read_to_string(root.join(current_doc)).expect("current tooling doc");
        assert!(
            ordinary_lifecycle_guidance_is_unambiguous(&text),
            "stale ordinary v2 guidance in {current_doc}"
        );
    }
    assert_eq!(
        csdlc_v3::commands::proof::CANONICAL_INSTALL_DESTINATION,
        ".adl/bin/csdlc",
        "native install and cutover must share one stable destination"
    );
    let operational_guide =
        fs::read_to_string(root.join("adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md"))
            .expect("operational skills guide");
    assert!(
        operational_guide.contains(
            "csdlc issue --request <bootstrap-request.json> --registry docs/templates/prompts/current.json --registrations <registrations.json>"
        ),
        "documented native issue route must include its required registrations input"
    );
}

#[test]
fn active_boot_path_guard_rejects_stale_v2_current_guidance_but_allows_history() {
    assert!(!ordinary_lifecycle_guidance_is_unambiguous(
        "Resolve the sole active C-SDLC generation with csdlc-install resolve, then use .adl/bin/csdlc-v2/csdlc-doctor."
    ));
    assert!(!ordinary_lifecycle_guidance_is_unambiguous(
        "C-SDLC workflow lifecycle: canonical under Gate 10D2; run csdlc-bind."
    ));
    assert!(ordinary_lifecycle_guidance_is_unambiguous(
        "Historical evidence: csdlc-doctor was the source-time v2 command."
    ));
    assert!(ordinary_lifecycle_guidance_is_unambiguous(
        "The .adl/bin/csdlc-v2/ surface is retained only for explicitly authorized rollback or bounded transition remediation."
    ));
}
