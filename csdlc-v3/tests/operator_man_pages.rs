//! PVF: required deterministic small local tooling proof; no remote writes.
use csdlc_v3::commands::{
    local::{LocalPreparationRequest, LOCAL_ROUTE_NAMES},
    proof::{classify_route, ProofRouteRequest, ProofRouteStatus, PROOF_ROUTE_NAMES},
    remote::{
        prepare_remote_publication_route, GithubMutation, OperationalRemoteDispatchRequest,
        OperationalRemoteOperation, RemoteRouteRequest, RemoteRouteStatus, TypedReviewReceipt,
        REMOTE_PUBLICATION_ROUTE_NAMES,
    },
    sprint::SprintReadinessRequest,
    terminal::{
        prepare_terminal_route, TerminalRouteRequest, TerminalRouteStatus, TERMINAL_ROUTE_NAMES,
    },
};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .into()
}
fn manual() -> PathBuf {
    root().join("docs/csdlc-v3/man")
}
fn json_file(name: &str) -> Value {
    serde_json::from_slice(&fs::read(manual().join(name)).unwrap()).unwrap()
}
fn example(name: &str) -> Vec<u8> {
    fs::read(manual().join("examples").join(name)).unwrap()
}
fn flags(text: &str) -> BTreeSet<String> {
    text.split('"')
        .enumerate()
        .filter(|(i, s)| {
            i % 2 == 1
                && s.starts_with("--")
                && s.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        })
        .map(|(_, s)| s.to_owned())
        .collect()
}
fn parser_flags(source: &str, parser: &str) -> BTreeSet<String> {
    let body = source.split(&format!("impl {parser} {{")).nth(1).unwrap();
    let body = body
        .split("\n#[derive")
        .next()
        .unwrap()
        .split("\nimpl ")
        .next()
        .unwrap();
    let mut result = flags(body);
    result.extend(["--help".into(), "-h".into()]);
    result
}
fn function_flags(source: &str, function: &str) -> BTreeSet<String> {
    let body = source.split(&format!("fn {function}(")).nth(1).unwrap();
    let mut result = flags(body.split("\n}").next().unwrap());
    result.insert("-h".into());
    result
}
fn descriptor_intent_flags(command: &str) -> BTreeSet<String> {
    let descriptor =
        csdlc_v3::commands::contract::descriptor(command).expect("documented descriptor");
    let mut options: BTreeSet<String> = descriptor["input_variants"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|variant| variant["request_type"] == "IntentRequest")
        .flat_map(|variant| {
            variant["required_flags"]
                .as_array()
                .unwrap()
                .iter()
                .chain(variant["optional_flags"].as_array().unwrap())
        })
        .map(|flag| flag.as_str().unwrap().to_owned())
        .collect();
    if !options.is_empty() {
        options.extend(["--help".into(), "-h".into()]);
    }
    options
}

fn intent_parity(inventory: &Value, source: &str) -> Result<(), String> {
    let mut documented_union = BTreeSet::new();
    for row in inventory["commands"].as_array().unwrap() {
        let command = row["command"].as_str().unwrap();
        if command.contains(' ') {
            continue;
        }
        let required = descriptor_intent_flags(command);
        let documented: BTreeSet<String> = row["intent_options"]
            .as_array()
            .into_iter()
            .flatten()
            .map(|flag| flag.as_str().unwrap().to_owned())
            .collect();
        if required != documented {
            return Err(format!("{command} intent descriptor options differ"));
        }
        documented_union.extend(documented);
    }
    documented_union.remove("--help");
    documented_union.remove("-h");
    if documented_union != flags(source) {
        return Err("intent parser options differ".into());
    }
    Ok(())
}

fn parity(inventory: &Value, source: &str) -> Result<(), String> {
    let mut expected: BTreeSet<String> = LOCAL_ROUTE_NAMES
        .iter()
        .chain(PROOF_ROUTE_NAMES.iter())
        .chain(REMOTE_PUBLICATION_ROUTE_NAMES.iter())
        .chain(TERMINAL_ROUTE_NAMES.iter())
        .chain(csdlc_v3::application::intent::INTENTS.iter())
        .map(|s| (*s).into())
        .collect();
    expected.extend(
        [
            "foundation",
            "local",
            "remote",
            "sprint",
            "release-preflight",
            "rollback",
            "github-issue create",
            "github-issue close",
        ]
        .map(str::to_owned),
    );
    let rows = inventory["commands"].as_array().unwrap();
    let actual: BTreeSet<_> = rows
        .iter()
        .map(|row| row["command"].as_str().unwrap().to_owned())
        .collect();
    if expected != actual || rows.len() != actual.len() {
        return Err("command denominator drift".into());
    }
    for row in rows {
        let command = row["command"].as_str().unwrap();
        let family = row["family"].as_str().unwrap();
        let mut required = match family {
            "intent" => descriptor_intent_flags(command),
            "local" => parser_flags(source, "LocalArgs"),
            "remote" => parser_flags(source, "RemoteArgs"),
            "terminal" => parser_flags(source, "TerminalArgs"),
            "proof" | "release" => parser_flags(source, "RequestOnlyArgs"),
            "simple-issue" => parser_flags(
                source,
                if command.ends_with("create") {
                    "SimpleIssueCreateArgs"
                } else {
                    "SimpleIssueCloseArgs"
                },
            ),
            _ => function_flags(
                source,
                match command {
                    "foundation" => "run_foundation",
                    "sprint" => "run_sprint",
                    "remote" => "run_remote_overview",
                    _ => unreachable!(),
                },
            ),
        };
        if command == "clean" {
            required.remove("--observe-github");
        }
        let documented: BTreeSet<_> = row["options"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_owned())
            .collect();
        if required != documented {
            return Err(format!(
                "{command} options differ: parsed={required:?} documented={documented:?}"
            ));
        }
    }
    Ok(())
}

#[test]
fn command_and_parser_option_inventory_is_complete() {
    let inventory = json_file("inventory.json");
    let source = fs::read_to_string(root().join("csdlc-v3/src/main.rs")).unwrap();
    parity(&inventory, &source).unwrap();
    let intent_source =
        fs::read_to_string(root().join("csdlc-v3/src/application/intent/mod.rs")).unwrap();
    intent_parity(&inventory, &intent_source).unwrap();
    assert_eq!(inventory["coverage"]["installed_root_commands"], 29);
    assert_eq!(
        inventory["coverage"]["frozen_sim03_predecessor_dispositions"],
        27
    );
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("--help")
        .output()
        .unwrap();
    assert!(output.status.success());
    let help = String::from_utf8(output.stdout).unwrap();
    let documented: BTreeSet<_> = inventory["commands"]
        .as_array()
        .unwrap()
        .iter()
        .map(|row| row["command"].as_str().unwrap())
        .collect();
    for line in help.lines().filter(|line| line.starts_with("  ")) {
        let command = line
            .split_whitespace()
            .take_while(|part| !part.starts_with("--") && !part.starts_with('<'))
            .collect::<Vec<_>>()
            .join(" ");
        assert!(
            documented.contains(command.as_str()),
            "undocumented help: {line}"
        );
    }
    let pages = json_file("manual.json");
    for row in inventory["commands"].as_array().unwrap() {
        let command = row["command"].as_str().unwrap();
        let page_name = row["page"]
            .as_str()
            .map(str::to_owned)
            .unwrap_or_else(|| format!("csdlc-{command}"));
        let page = pages["pages"]
            .as_array()
            .unwrap()
            .iter()
            .find(|p| p["name"] == page_name)
            .unwrap();
        let prose = serde_json::to_string(page).unwrap();
        for option in row["options"]
            .as_array()
            .unwrap()
            .iter()
            .chain(row["intent_options"].as_array().into_iter().flatten())
        {
            assert!(
                prose.contains(option.as_str().unwrap()),
                "{page_name} lacks {option}"
            );
        }
        for section in ["SYNOPSIS", "DESCRIPTION", "OPTIONS", "EXAMPLES"] {
            assert!(
                page["sections"][section].is_array(),
                "{page_name} lacks {section}"
            );
        }
        assert!(manual()
            .join("man1")
            .join(format!("{page_name}.1"))
            .is_file());
        if let Some(example) = row["example"].as_str() {
            assert!(manual().join("examples").join(example).is_file());
        }
        let mut argv: Vec<_> = command.split_whitespace().collect();
        argv.push("--help");
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .args(&argv)
            .output()
            .unwrap();
        assert!(help.status.success(), "{command} help failed");
    }
}

#[test]
fn generated_pages_match_reviewed_manual_source() {
    let output = Command::new("python3")
        .arg(manual().join("render.py"))
        .arg("--check")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn parity_rejects_new_command_or_option_without_manual_coverage() {
    let inventory = json_file("inventory.json");
    let source = fs::read_to_string(root().join("csdlc-v3/src/main.rs")).unwrap();
    let changed = source.replace(
        "impl LocalArgs {",
        "impl LocalArgs {\n// parser extension \"--new-option\"",
    );
    assert!(parity(&inventory, &changed)
        .unwrap_err()
        .contains("options differ"));
    let mut missing = inventory.clone();
    missing["commands"].as_array_mut().unwrap().remove(0);
    assert!(parity(&missing, &source)
        .unwrap_err()
        .contains("denominator"));
    let intent_source =
        fs::read_to_string(root().join("csdlc-v3/src/application/intent/mod.rs")).unwrap();
    let changed_intent = format!("{intent_source}\n// parser extension \"--new-intent-option\"\n");
    assert!(intent_parity(&inventory, &changed_intent)
        .unwrap_err()
        .contains("parser options differ"));
    let mut missing_intent = inventory.clone();
    missing_intent["commands"]
        .as_array_mut()
        .unwrap()
        .iter_mut()
        .find(|row| row["command"] == "status")
        .unwrap()["intent_options"]
        .as_array_mut()
        .unwrap()
        .pop();
    assert!(intent_parity(&missing_intent, &intent_source)
        .unwrap_err()
        .contains("descriptor options differ"));
    let mut invented = inventory;
    invented["commands"]
        .as_array_mut()
        .unwrap()
        .push(json!({"command":"invented"}));
    assert!(parity(&invented, &source)
        .unwrap_err()
        .contains("denominator"));
}

#[test]
fn request_field_inventory_matches_current_source_and_operator_reference() {
    let inventory = json_file("inventory.json");
    let pages = json_file("manual.json");
    let reference = pages["pages"]
        .as_array()
        .unwrap()
        .iter()
        .find(|p| p["name"] == "csdlc-requests")
        .unwrap();
    for item in inventory["request_types"].as_array().unwrap() {
        let source = fs::read_to_string(root().join(item["source"].as_str().unwrap())).unwrap();
        let rust_name = item["rust_name"].as_str().unwrap();
        let body = source
            .split(&format!("pub struct {rust_name} {{"))
            .nth(1)
            .unwrap()
            .split("\n}")
            .next()
            .unwrap();
        let actual: BTreeSet<_> = body
            .lines()
            .filter_map(|line| line.trim().strip_prefix("pub "))
            .map(|line| line.trim_end_matches(',').to_owned())
            .collect();
        let documented: BTreeSet<_> = item["fields"]
            .as_array()
            .unwrap()
            .iter()
            .map(|f| {
                format!(
                    "{}: {}",
                    f["name"].as_str().unwrap(),
                    f["type"].as_str().unwrap()
                )
            })
            .collect();
        assert_eq!(actual, documented, "{rust_name} request fields drifted");
        let section = &reference["sections"][item["name"].as_str().unwrap().to_uppercase()];
        let prose = serde_json::to_string(section).unwrap();
        for field in &documented {
            assert!(prose.contains(field), "{rust_name}: missing {field}");
        }
    }
}

#[test]
fn examples_parse_current_native_requests_and_preserve_guard_truth() {
    let local = LocalPreparationRequest::from_json(&example("local.json")).unwrap();
    assert_eq!(local.issue, 861);
    assert_eq!(local.commands.len(), 8);
    let remote: RemoteRouteRequest = serde_json::from_slice(&example("remote.json")).unwrap();
    let review = prepare_remote_publication_route("review", &remote).unwrap();
    assert_eq!(review.status, RemoteRouteStatus::Ready);
    let native_review = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .current_dir(root())
        .args(["review", "--request"])
        .arg(manual().join("examples/review.json"))
        .output()
        .unwrap();
    assert!(native_review.status.success());
    let report: Value = serde_json::from_slice(&native_review.stdout).unwrap();
    assert_eq!(report["result"]["status"], "ready");
    assert_eq!(report["operational_authority"], false);
    // A schema fixture must not impersonate authenticated publication evidence.
    let publication = prepare_remote_publication_route("publish", &remote).unwrap();
    assert_eq!(publication.status, RemoteRouteStatus::Blocked);
    assert!(publication
        .findings
        .iter()
        .any(|f| f.code == "authenticated_review_receipt_missing"));
    let _: TypedReviewReceipt = serde_json::from_slice(&example("typed-review.json")).unwrap();
    for name in ["mutation.json", "pr-create.json", "merge.json"] {
        let dispatch: OperationalRemoteDispatchRequest =
            serde_json::from_slice(&example(name)).unwrap();
        assert!(matches!(
            dispatch.operation,
            OperationalRemoteOperation::GithubMutation(_)
        ));
    }
    let merge: OperationalRemoteDispatchRequest =
        serde_json::from_slice(&example("merge.json")).unwrap();
    assert!(
        matches!(merge.operation, OperationalRemoteOperation::GithubMutation(r) if matches!(r.mutation, GithubMutation::PullRequestMerge { .. }))
    );
    let finish: TerminalRouteRequest = serde_json::from_slice(&example("finish.json")).unwrap();
    let mut observation = finish.clone();
    observation.terminal_state = None;
    let terminal = prepare_terminal_route("finish", &observation).unwrap();
    assert_eq!(terminal.status, TerminalRouteStatus::Blocked);
    assert!(terminal
        .findings
        .iter()
        .any(|f| f.code == "authenticated_adapter_required"));
    for name in [
        "finish-primary.json",
        "clean.json",
        "cutover.json",
        "rollback.json",
    ] {
        let _: TerminalRouteRequest = serde_json::from_slice(&example(name)).unwrap();
    }
    for name in ["proof.json", "install.json", "shadow.json", "soak.json"] {
        let request: ProofRouteRequest = serde_json::from_slice(&example(name)).unwrap();
        if name == "shadow.json" || name == "soak.json" {
            let report = classify_route(name.trim_end_matches(".json"), request, None);
            assert_eq!(report.status, ProofRouteStatus::Blocked);
            assert!(report
                .findings
                .iter()
                .any(|f| f.code == "historical_route_disabled"));
        }
    }
    let _: SprintReadinessRequest = serde_json::from_slice(&example("sprint.json")).unwrap();
    let _: csdlc_v3::commands::release::Request =
        serde_json::from_slice(&example("release.json")).unwrap();
}

#[test]
fn primary_cleanup_requires_primary_durable_receipt_location() {
    // Receipt bytes are fixtures: this proves path compatibility, not authentication.
    let fixture = root()
        .join("target")
        .join(format!("manual-cleanup-{}", std::process::id()));
    assert!(!fixture.exists());
    fs::create_dir_all(&fixture).unwrap();
    let primary = fixture.join("primary");
    let worktree = fixture.join("linked");
    fs::create_dir(&primary).unwrap();
    let git = |args: &[&str]| {
        let out = Command::new("git")
            .arg("-C")
            .arg(&primary)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        String::from_utf8(out.stdout).unwrap().trim().to_owned()
    };
    git(&["init", "-q", "-b", "main"]);
    git(&[
        "-c",
        "user.name=Fixture",
        "-c",
        "user.email=fixture@example.invalid",
        "commit",
        "--allow-empty",
        "-qm",
        "fixture",
    ]);
    git(&[
        "worktree",
        "add",
        "-qb",
        "codex/861-example",
        worktree.to_str().unwrap(),
    ]);
    let head = git(&["rev-parse", "HEAD"]);
    let linked = worktree.join(".csdlc/evidence/861/terminal-receipt.json");
    let primary_receipt = primary.join(".git/csdlc-v3/local/evidence/861/terminal-receipt.json");
    let receipt = serde_json::to_vec(&json!({"schema":"csdlc.v3.terminal_receipt.v1","repository":"agent-logic/agent-design-language","issue":861,"pull_request":1234,"head_sha":head,"disposition":"closed_out"})).unwrap();
    fs::create_dir_all(linked.parent().unwrap()).unwrap();
    fs::write(&linked, &receipt).unwrap();
    let mut request: TerminalRouteRequest = serde_json::from_slice(&example("clean.json")).unwrap();
    request.expected_head_sha = Some(head);
    let cleanup = request.cleanup.as_mut().unwrap();
    cleanup.approved_parent = fixture.clone();
    cleanup.repository_root = primary.clone();
    cleanup.candidate_path = worktree.clone();
    cleanup.terminal_receipt_path = Some(linked.display().to_string());
    cleanup.terminal_receipt_digest = Some(blake3::hash(&receipt).to_hex().to_string());
    let result = prepare_terminal_route("clean", &request).unwrap();
    assert_eq!(result.status, TerminalRouteStatus::Blocked);
    assert!(result
        .findings
        .iter()
        .any(|f| f.code == "receipt_path_escapes_repository"));
    // Stand in for a separate native primary finish receipt; never copy this in live operation.
    fs::create_dir_all(primary_receipt.parent().unwrap()).unwrap();
    fs::write(&primary_receipt, &receipt).unwrap();
    fs::remove_dir_all(worktree.join(".csdlc")).unwrap();
    request.cleanup.as_mut().unwrap().terminal_receipt_path =
        Some(primary_receipt.display().to_string());
    let result = prepare_terminal_route("clean", &request).unwrap();
    assert!(matches!(
        result.cleanup,
        Some(csdlc_v3::commands::terminal::CleanupDecision::Removable { .. })
    ));
    git(&["worktree", "remove", worktree.to_str().unwrap()]);
    fs::remove_dir_all(fixture).unwrap();
}
