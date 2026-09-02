use std::collections::BTreeSet;
use std::process::Command;

use csdlc_v3::commands::local::{
    authorize_bind, grants_operational_authority, inspect_local_lifecycle_state,
    local_route_command, plan_cards, prepare_local_workflow, required_local_commands,
    validate_contract, LocalPreparationRequest, PlanStatus, PromptRegistry, WorktreeRegistration,
    LOCAL_ROUTE_NAMES,
};
use csdlc_v3::{is_v3d_local_preparation_predecessor, LOCAL_PREPARATION_PREDECESSORS};
use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn fixture_dir(name: &str) -> PathBuf {
    let dir = repo_root()
        .join("csdlc-v3/target/local-command-fixtures")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("fixture directory");
    dir
}

fn request() -> LocalPreparationRequest {
    LocalPreparationRequest {
        issue: 503,
        title: "[v0.92.1][V3-D] C-SDLC v3 local preparation workflow".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: "codex/503-v3-d-local-preparation-workflow-exec".into(),
        worktree: "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec".into(),
        registry_version: "1.0.3".into(),
        commands: required_local_commands().to_vec(),
    }
}

fn registry() -> PromptRegistry {
    PromptRegistry {
        version: "1.0.3".into(),
        card_kinds: ["sip", "stp", "spp", "vpp", "srp", "sor"]
            .into_iter()
            .map(str::to_string)
            .collect::<BTreeSet<_>>(),
    }
}

fn registrations() -> Vec<WorktreeRegistration> {
    vec![WorktreeRegistration {
        branch: "codex/503-v3-d-local-preparation-workflow-exec".into(),
        worktree: "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec".into(),
        primary: false,
    }]
}

#[test]
fn contract_commands_are_typed_and_non_authoritative() {
    let json = serde_json::to_vec(&request()).expect("request serializes");
    let request = LocalPreparationRequest::from_json(&json).expect("typed request parses");
    validate_contract(&request).expect("valid typed local command contract");
    for command in request.commands {
        assert!(!grants_operational_authority(command));
    }
    for route in LOCAL_ROUTE_NAMES {
        assert!(
            local_route_command(route).is_some(),
            "missing route {route}"
        );
    }
    assert_eq!(LOCAL_PREPARATION_PREDECESSORS, [171, 172, 173]);
    assert!(is_v3d_local_preparation_predecessor(171));
    assert!(is_v3d_local_preparation_predecessor(172));
    assert!(is_v3d_local_preparation_predecessor(173));
    assert!(!is_v3d_local_preparation_predecessor(170));
    assert!(!is_v3d_local_preparation_predecessor(174));
}

#[test]
fn topology_bind_requires_exact_registered_worktree() {
    let req = request();
    let authorized =
        authorize_bind(&req, &registrations()).expect("registered worktree authorizes bind");
    assert_eq!(authorized.issue, 503);
    assert_eq!(authorized.branch, req.branch);
    assert_eq!(authorized.worktree, req.worktree);

    let branch_only = [WorktreeRegistration {
        branch: req.branch,
        worktree: "/other/worktree".into(),
        primary: false,
    }];
    let findings = authorize_bind(&request(), &branch_only).expect_err("path mismatch blocks bind");
    assert_eq!(findings[0].status, PlanStatus::Blocked);
    assert_eq!(findings[0].code, "registered_topology_missing");
}

#[test]
fn card_roundtrip_uses_active_registry_denominator() {
    let bytes = fs::read(repo_root().join("docs/templates/prompts/current.json"))
        .expect("current prompt registry");
    let active = PromptRegistry::from_current_json(&bytes).expect("active registry parses");
    assert_eq!(active, registry());

    let plan = plan_cards(503, "1.0.3", &active).expect("complete active registry");
    assert_eq!(plan.registry_version, "1.0.3");
    assert_eq!(plan.card_kinds, ["sip", "stp", "spp", "vpp", "srp", "sor"]);

    let incomplete = PromptRegistry {
        version: "1.0.3".into(),
        card_kinds: ["sip", "stp"].into_iter().map(str::to_string).collect(),
    };
    let findings =
        plan_cards(503, "1.0.3", &incomplete).expect_err("missing card kinds block rendering");
    assert!(findings
        .iter()
        .all(|finding| finding.status == PlanStatus::Blocked));
    assert!(findings
        .iter()
        .any(|finding| finding.code == "card_kind_missing"));
}

#[test]
fn doctor_plan_preserves_distinct_outcome_states() {
    let plan = prepare_local_workflow(&request(), &registry(), &registrations())
        .expect("typed issue input reaches doctor-validated PVF plan");
    assert_eq!(plan.issue, 503);
    assert_eq!(plan.findings[0].status, PlanStatus::Ready);
    assert_eq!(plan.findings[0].code, "doctor_ready");
    assert!(plan.lifecycle_state.is_none());

    let statuses = [
        PlanStatus::Ready,
        PlanStatus::Blocked,
        PlanStatus::Failed,
        PlanStatus::Deferred,
        PlanStatus::Skipped,
        PlanStatus::Passed,
    ];
    assert_eq!(
        statuses.into_iter().collect::<BTreeSet<_>>().len(),
        statuses.len(),
        "doctor/PVF outcomes must not be conflated"
    );
}

#[test]
fn local_preparation_cli_emits_machine_readable_non_authoritative_plan() {
    let dir = fixture_dir("success");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request()).expect("request json"),
    )
    .expect("write request fixture");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&registrations()).expect("registrations json"),
    )
    .expect("write registrations fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("local")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .output()
        .expect("run local preparation CLI");
    assert!(output.status.success(), "{output:?}");

    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable json");
    assert_eq!(value["schema"], "csdlc.v3.local_preparation.v1");
    assert_eq!(value["command"], "local");
    assert_eq!(value["read_only"], true);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["result"]["issue"], 503);
    assert_eq!(value["result"]["findings"][0]["code"], "doctor_ready");
    assert!(output.stderr.is_empty());
}

#[test]
fn implemented_local_routes_share_the_typed_non_authoritative_contract() {
    let dir = fixture_dir("routes");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request()).expect("request json"),
    )
    .expect("write request fixture");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&registrations()).expect("registrations json"),
    )
    .expect("write registrations fixture");

    for route in LOCAL_ROUTE_NAMES {
        let help = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .arg(route)
            .arg("--help")
            .output()
            .expect("run route help");
        assert!(help.status.success(), "{route} help failed: {help:?}");
        let help_stdout = String::from_utf8_lossy(&help.stdout);
        assert!(help_stdout.contains("status: implemented"));
        assert!(help_stdout.contains("#505 cutover"));

        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .arg(route)
            .arg("--request")
            .arg(&request_path)
            .arg("--registry")
            .arg(repo_root().join("docs/templates/prompts/current.json"))
            .arg("--registrations")
            .arg(&registrations_path)
            .output()
            .expect("run local route");
        assert!(output.status.success(), "{route} failed: {output:?}");
        let value: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("machine-readable json");
        assert_eq!(value["command"], route);
        assert_eq!(value["read_only"], true);
        assert_eq!(value["operational_authority"], false);
        assert_eq!(value["result"]["issue"], 503);
        assert!(output.stderr.is_empty());
    }
}

#[test]
fn missing_local_lifecycle_state_is_explicit_and_repairable() {
    let dir = fixture_dir("missing-state");
    let observation = inspect_local_lifecycle_state(&dir, 628);
    assert_eq!(observation.status, PlanStatus::Blocked);
    assert_eq!(observation.code, "missing_local_lifecycle_state");
    assert!(!observation.ready_to_execute);
    assert_eq!(
        observation.missing_cards,
        ["sip", "stp", "spp", "vpp", "srp", "sor"]
    );
    assert!(observation.message.contains("initialize or repair"));
}

#[test]
fn local_lifecycle_readiness_requires_supported_phase_and_all_cards() {
    let dir = fixture_dir("ready-state");
    let issue_root = dir.join(".csdlc/issues/628");
    fs::create_dir_all(issue_root.join("cards")).expect("issue cards dir");
    fs::write(issue_root.join("index.json"), br#"{"phase":"ready"}"#).expect("write index");
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        fs::write(
            issue_root.join("cards").join(format!("{card}.values.json")),
            b"{}",
        )
        .expect("write values");
        fs::write(
            issue_root.join("cards").join(format!("{card}.md")),
            format!("# {card}\n"),
        )
        .expect("write card");
    }

    let observation = inspect_local_lifecycle_state(&dir, 628);
    assert_eq!(observation.status, PlanStatus::Ready);
    assert_eq!(observation.code, "local_lifecycle_state_ready");
    assert!(observation.ready_to_execute);
    assert!(observation.message.contains("ready"));
}

#[test]
fn local_lifecycle_readiness_rejects_post_execution_phase() {
    let dir = fixture_dir("implemented-state");
    let issue_root = dir.join(".csdlc/issues/628");
    fs::create_dir_all(issue_root.join("cards")).expect("issue cards dir");
    fs::write(issue_root.join("index.json"), br#"{"phase":"implemented"}"#).expect("write index");
    for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        fs::write(
            issue_root.join("cards").join(format!("{card}.values.json")),
            b"{}",
        )
        .expect("write values");
        fs::write(
            issue_root.join("cards").join(format!("{card}.md")),
            format!("# {card}\n"),
        )
        .expect("write card");
    }

    let observation = inspect_local_lifecycle_state(&dir, 628);
    assert_eq!(observation.status, PlanStatus::Blocked);
    assert_eq!(observation.code, "unsupported_local_lifecycle_phase");
    assert!(!observation.ready_to_execute);
    assert!(observation.message.contains("implemented"));
}

#[test]
fn local_lifecycle_readiness_rejects_malformed_index() {
    let dir = fixture_dir("malformed-state");
    let issue_root = dir.join(".csdlc/issues/628");
    fs::create_dir_all(&issue_root).expect("issue dir");
    fs::write(issue_root.join("index.json"), b"{not-json").expect("write malformed index");

    let observation = inspect_local_lifecycle_state(&dir, 628);
    assert_eq!(observation.status, PlanStatus::Blocked);
    assert_eq!(observation.code, "invalid_local_lifecycle_state");
    assert!(!observation.ready_to_execute);
    assert!(observation.message.contains("not valid JSON"));
}

#[test]
fn local_preparation_cli_rejects_malformed_typed_request() {
    let dir = fixture_dir("malformed");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    fs::write(&request_path, b"{not-json").expect("write malformed request");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&registrations()).expect("registrations json"),
    )
    .expect("write registrations fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("local")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .output()
        .expect("run local preparation CLI");
    assert!(!output.status.success(), "{output:?}");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("typed_contract_invalid_json"));
    assert!(output.stdout.is_empty());
}
