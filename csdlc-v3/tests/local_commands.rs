use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::process::Command;

use csdlc_v3::commands::local::{
    authorize_bind, grants_operational_authority, inspect_local_lifecycle_state,
    local_route_command, local_route_status, plan_cards, prepare_local_workflow,
    required_local_commands, validate_contract, LocalPreparationRequest, PlanStatus,
    PromptRegistry, WorktreeRegistration, LOCAL_ROUTE_NAMES,
};
use csdlc_v3::{is_v3d_local_preparation_predecessor, LOCAL_PREPARATION_PREDECESSORS};
use std::fs;
use std::path::{Path, PathBuf};

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
        expected_lifecycle_digest: None,
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
        template_paths: [
            ("sip", "docs/templates/prompts/1.0.3/sip.md"),
            ("stp", "docs/templates/prompts/1.0.3/stp.md"),
            ("spp", "docs/templates/prompts/1.0.3/spp.md"),
            ("vpp", "docs/templates/prompts/1.0.3/vpp.md"),
            ("srp", "docs/templates/prompts/1.0.3/srp.md"),
            ("sor", "docs/templates/prompts/1.0.3/sor.md"),
        ]
        .into_iter()
        .map(|(kind, path)| (kind.to_owned(), path.to_owned()))
        .collect::<BTreeMap<_, _>>(),
    }
}

fn registrations() -> Vec<WorktreeRegistration> {
    vec![WorktreeRegistration {
        branch: "codex/503-v3-d-local-preparation-workflow-exec".into(),
        worktree: "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec".into(),
        primary: false,
    }]
}

fn write_lifecycle_state(root: &Path, issue: u64, phase: &str, digest: &str) {
    let issue_root = root.join(format!(".csdlc/issues/{issue}"));
    fs::create_dir_all(issue_root.join("cards")).expect("issue cards dir");
    fs::write(
        issue_root.join("index.json"),
        format!("{{\"phase\":\"{phase}\",\"generation\":7,\"digest\":\"{digest}\"}}"),
    )
    .expect("write index");
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

    let primary_checkout = [WorktreeRegistration {
        branch: request().branch,
        worktree: request().worktree,
        primary: true,
    }];
    let findings =
        authorize_bind(&request(), &primary_checkout).expect_err("primary checkout blocks bind");
    assert_eq!(findings[0].status, PlanStatus::Blocked);
    assert_eq!(findings[0].code, "primary_worktree_denied");
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
        template_paths: BTreeMap::new(),
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
    assert_eq!(value["operational_read_only"], true);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["writes_v3_state"], false);
    assert_eq!(value["result"]["issue"], 503);
    assert!(value["route_status"].is_null());
    assert!(value["route_result"].is_null());
    assert_eq!(value["result"]["findings"][0]["code"], "doctor_ready");
    let rendered = value["result"]["cards"]["rendered_cards"]
        .as_array()
        .expect("rendered cards");
    assert_eq!(rendered.len(), 6);
    for (idx, kind) in ["sip", "stp", "spp", "vpp", "srp", "sor"]
        .into_iter()
        .enumerate()
    {
        assert_eq!(rendered[idx]["kind"], kind);
        assert_eq!(
            rendered[idx]["template_ref"],
            format!("docs/templates/prompts/1.0.3/{kind}.md")
        );
        assert_eq!(
            rendered[idx]["rendered_ref"],
            format!(".csdlc/issues/503/cards/{kind}.md")
        );
        assert_eq!(
            rendered[idx]["render_manifest_digest"]
                .as_str()
                .expect("render manifest digest")
                .len(),
            64
        );
    }
    assert!(output.stderr.is_empty());
}

#[test]
fn implemented_local_routes_have_distinct_typed_non_authoritative_statuses() {
    let dir = fixture_dir("routes");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let ready_root = dir.join("ready-root");
    let bound_root = dir.join("bound-root");
    let state_root = dir.join("state-root");
    write_lifecycle_state(&ready_root, 503, "ready", "digest-ready");
    write_lifecycle_state(&bound_root, 503, "bound", "digest-bound");
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

        let mut output = Command::new(env!("CARGO_BIN_EXE_csdlc"));
        output
            .arg(route)
            .arg("--request")
            .arg(&request_path)
            .arg("--registry")
            .arg(repo_root().join("docs/templates/prompts/current.json"))
            .arg("--registrations")
            .arg(&registrations_path);
        match route {
            "issue" => {
                output.arg("--v3-state-root").arg(&state_root);
            }
            "shepherd" => {
                output.arg("--repo-root").arg(&bound_root);
            }
            _ => {
                output.arg("--repo-root").arg(&ready_root);
            }
        }
        let output = output.output().expect("run local route");
        assert!(output.status.success(), "{route} failed: {output:?}");
        let value: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("machine-readable json");
        assert_eq!(value["command"], route);
        assert_eq!(value["read_only"], route != "issue");
        assert_eq!(value["operational_read_only"], true);
        assert_eq!(value["operational_authority"], false);
        assert_eq!(value["writes_v3_state"], route == "issue");
        assert_eq!(value["result"]["issue"], 503);
        assert_eq!(value["route_status"]["route"], route);
        assert_eq!(value["route_status"]["issue_start_minutes_max"], 3);
        assert_eq!(value["route_result"]["issue"], 503);
        assert_eq!(
            value["route_result"]["kind"],
            expected_route_result_kind(route)
        );
        assert!(output.stderr.is_empty());
    }
}

fn expected_route_result_kind(route: &str) -> &'static str {
    match route {
        "issue" => "issue_initialization",
        "bind" => "bind_worktree",
        "edit" => "card_edit",
        "validate" => "validation_plan",
        "doctor" => "doctor",
        "schedule" => "schedule",
        "shepherd" => "shepherd",
        "eligibility" => "eligibility",
        _ => panic!("unexpected route {route}"),
    }
}

#[test]
fn local_route_status_codes_are_route_specific() {
    let dir = fixture_dir("status-codes");
    let ready_root = dir.join("ready");
    let bound_root = dir.join("bound");
    write_lifecycle_state(&ready_root, 503, "ready", "digest-ready");
    write_lifecycle_state(&bound_root, 503, "bound", "digest-bound");
    let missing = inspect_local_lifecycle_state(&dir, 503);
    let ready = inspect_local_lifecycle_state(&ready_root, 503);
    let bound = inspect_local_lifecycle_state(&bound_root, 503);
    let mut codes = BTreeSet::new();
    for route in LOCAL_ROUTE_NAMES {
        let observation = match route {
            "issue" => &missing,
            "shepherd" => &bound,
            _ => &ready,
        };
        let status = local_route_status(route, Some(observation)).expect("known local route");
        assert!(
            codes.insert(status.code.clone()),
            "route {route} reused status code {}",
            status.code
        );
        assert_eq!(status.issue_start_minutes_max, 3);
    }
    assert!(codes.contains("issue_preparation_ready"));
    assert!(codes.contains("bind_topology_authorized"));
    assert!(codes.contains("edit_plan_ready"));
    assert!(codes.contains("pvf_plan_ready"));
    assert!(codes.contains("doctor_ready"));
    assert!(codes.contains("schedule_plan_ready"));
    assert!(codes.contains("shepherd_plan_ready"));
    assert!(!codes.contains("lifecycle_observation_missing"));
}

#[test]
fn eligibility_route_consumes_lifecycle_observation() {
    let dir = fixture_dir("eligibility-state");
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
    let status =
        local_route_status("eligibility", Some(&observation)).expect("eligibility route status");
    assert_eq!(status.status, PlanStatus::Ready);
    assert_eq!(status.code, "ready_to_execute");
}

#[test]
fn issue_route_can_initialize_v3_local_state_and_eligibility_consumes_it() {
    let dir = fixture_dir("v3-local-state");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let state_root = dir.join("state");
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

    let issue_output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("issue")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--v3-state-root")
        .arg(&state_root)
        .output()
        .expect("run v3 issue initialization route");
    assert!(issue_output.status.success(), "{issue_output:?}");
    let issue_value: serde_json::Value =
        serde_json::from_slice(&issue_output.stdout).expect("issue route JSON");
    assert_eq!(issue_value["route_result"]["kind"], "issue_initialization");
    assert_eq!(issue_value["read_only"], false);
    assert_eq!(issue_value["operational_read_only"], true);
    assert_eq!(issue_value["operational_authority"], false);
    assert_eq!(issue_value["writes_v3_state"], true);
    assert_eq!(
        issue_value["route_result"]["initialized_state"]["code"],
        "local_lifecycle_state_ready"
    );

    let eligibility_output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("eligibility")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--v3-state-root")
        .arg(&state_root)
        .output()
        .expect("run v3 eligibility route");
    assert!(
        eligibility_output.status.success(),
        "{eligibility_output:?}"
    );
    let eligibility_value: serde_json::Value =
        serde_json::from_slice(&eligibility_output.stdout).expect("eligibility route JSON");
    assert_eq!(
        eligibility_value["route_status"]["code"],
        "ready_to_execute"
    );
    assert_eq!(eligibility_value["read_only"], true);
    assert_eq!(eligibility_value["operational_read_only"], true);
    assert_eq!(eligibility_value["writes_v3_state"], false);
    assert_eq!(eligibility_value["route_result"]["kind"], "eligibility");
    assert_eq!(eligibility_value["route_result"]["ready_to_execute"], true);
    assert_eq!(
        eligibility_value["route_result"]["lifecycle_state"]["cards_present"]
            .as_array()
            .expect("cards_present is an array")
            .len(),
        6
    );
}

#[test]
fn issue_route_rejects_expected_digest_before_writing_v3_state() {
    let dir = fixture_dir("issue-stale-digest-write-free");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let missing_state_root = dir.join("missing-state");
    let stale_state_root = dir.join("stale-state");
    let mut stale = request();
    stale.expected_lifecycle_digest = Some("expected-digest".into());
    fs::write(
        &request_path,
        serde_json::to_vec(&stale).expect("request json"),
    )
    .expect("write request fixture");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&registrations()).expect("registrations json"),
    )
    .expect("write registrations fixture");

    let missing_output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("issue")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--v3-state-root")
        .arg(&missing_state_root)
        .output()
        .expect("run v3 issue route with missing state");
    assert!(!missing_output.status.success(), "{missing_output:?}");
    assert!(
        String::from_utf8_lossy(&missing_output.stderr).contains("local_lifecycle_digest_missing")
    );
    assert!(!missing_state_root.join("issues/503").exists());

    let stale_issue_root = stale_state_root.join("issues/503");
    fs::create_dir_all(&stale_issue_root).expect("stale issue dir");
    fs::write(
        stale_issue_root.join("index.json"),
        br#"{"phase":"ready","generation":7,"digest":"actual-digest"}"#,
    )
    .expect("write stale v3 index");
    let stale_output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("issue")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--v3-state-root")
        .arg(&stale_state_root)
        .output()
        .expect("run v3 issue route with stale digest");
    assert!(!stale_output.status.success(), "{stale_output:?}");
    assert!(String::from_utf8_lossy(&stale_output.stderr).contains("stale_local_lifecycle_digest"));
    assert!(!stale_issue_root.join("cards").exists());
}

#[test]
fn issue_route_requires_expected_digest_before_overwriting_existing_v3_state() {
    let dir = fixture_dir("issue-existing-state-requires-digest");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let state_root = dir.join("state");
    let issue_root = state_root.join("issues/503");
    fs::create_dir_all(&issue_root).expect("existing issue dir");
    fs::write(
        issue_root.join("index.json"),
        br#"{"phase":"ready","generation":7,"digest":"actual-digest"}"#,
    )
    .expect("write existing v3 index");
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
        .arg("issue")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--v3-state-root")
        .arg(&state_root)
        .output()
        .expect("run v3 issue route with existing state and no digest");
    assert!(!output.status.success(), "{output:?}");
    assert!(String::from_utf8_lossy(&output.stderr).contains("v3_local_state_digest_required"));
    assert!(!issue_root.join("cards").exists());
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
fn local_routes_fail_closed_without_observed_lifecycle_state() {
    let dir = fixture_dir("route-missing-observation");
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
        .arg("bind")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .output()
        .expect("run local bind route");
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("lifecycle_observation_missing"));
}

#[test]
fn local_routes_reject_unsupported_transitions_from_observed_phase() {
    let dir = fixture_dir("route-unsupported-transition");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let ready_root = dir.join("ready-root");
    write_lifecycle_state(&ready_root, 503, "ready", "digest-ready");
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
        .arg("shepherd")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--repo-root")
        .arg(&ready_root)
        .output()
        .expect("run local shepherd route");
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("unsupported_local_route_transition"));
}

#[test]
fn local_routes_reject_stale_lifecycle_digest() {
    let dir = fixture_dir("route-stale-digest");
    let request_path = dir.join("request.json");
    let registrations_path = dir.join("registrations.json");
    let ready_root = dir.join("ready-root");
    write_lifecycle_state(&ready_root, 503, "ready", "actual-digest");
    let mut stale = request();
    stale.expected_lifecycle_digest = Some("stale-digest".into());
    fs::write(
        &request_path,
        serde_json::to_vec(&stale).expect("request json"),
    )
    .expect("write request fixture");
    fs::write(
        &registrations_path,
        serde_json::to_vec(&registrations()).expect("registrations json"),
    )
    .expect("write registrations fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("bind")
        .arg("--request")
        .arg(&request_path)
        .arg("--registry")
        .arg(repo_root().join("docs/templates/prompts/current.json"))
        .arg("--registrations")
        .arg(&registrations_path)
        .arg("--repo-root")
        .arg(&ready_root)
        .output()
        .expect("run local bind route");
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("stale_local_lifecycle_digest"));
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
