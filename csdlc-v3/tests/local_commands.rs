use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::process::Command;

use csdlc_v3::commands::local::{
    authorize_bind, execute_operational_local_route, grants_operational_authority,
    inspect_local_lifecycle_state, local_route_command, local_route_status, plan_cards,
    prepare_local_workflow, required_local_commands, validate_contract, LocalPreparationRequest,
    OperationalLocalContext, PlanStatus, PromptRegistry, ScheduleReadinessInput,
    ShepherdRoutingInput, WorktreeRegistration, LOCAL_ROUTE_NAMES,
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
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: BTreeMap::new(),
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
fn contract_rejects_malformed_repository_identity() {
    let mut req = request();
    req.repository = "agent-logic/agent-design-language/issues/505".into();

    let findings = validate_contract(&req).expect_err("malformed repository blocks contract");
    assert!(findings
        .iter()
        .any(|finding| finding.code == "repository_invalid"));
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

fn run_git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .env("GIT_AUTHOR_NAME", "C-SDLC local test")
        .env("GIT_AUTHOR_EMAIL", "csdlc-local@example.invalid")
        .env("GIT_COMMITTER_NAME", "C-SDLC local test")
        .env("GIT_COMMITTER_EMAIL", "csdlc-local@example.invalid")
        .output()
        .expect("run fixture git command");
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout)
        .expect("git output is UTF-8")
        .trim()
        .to_owned()
}

fn operational_registry(root: &Path) -> PromptRegistry {
    let template_root = root.join("templates");
    let schema_root = root.join("schemas");
    fs::create_dir_all(&template_root).expect("template root");
    fs::create_dir_all(&schema_root).expect("schema root");
    let mut template_paths = BTreeMap::new();
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        let path = template_root.join(format!("{kind}.md"));
        fs::write(&path, format!("# {kind}\n{{{{title}}}}\n")).expect("template fixture");
        fs::write(
            schema_root.join(format!("{kind}.structure.json")),
            serde_json::to_vec(&serde_json::json!({"scaffold_lines": [format!("# {kind}")]}))
                .unwrap(),
        )
        .expect("structure schema fixture");
        template_paths.insert(kind.to_owned(), path.to_string_lossy().into_owned());
    }
    PromptRegistry {
        version: "1.0.3".into(),
        card_kinds: ["sip", "stp", "spp", "vpp", "srp", "sor"]
            .into_iter()
            .map(str::to_owned)
            .collect(),
        template_paths,
    }
}

fn operational_authority_fixture(
    name: &str,
    generation: &str,
) -> (PathBuf, String, OperationalLocalContext, PromptRegistry) {
    let fixture = fixture_dir(name);
    let repository_root = fixture.join("repo");
    fs::create_dir_all(&repository_root).expect("repository root");
    run_git(&repository_root, &["init", "--quiet"]);
    fs::write(repository_root.join("tracked"), b"fixture\n").expect("tracked fixture");
    run_git(&repository_root, &["add", "tracked"]);
    run_git(&repository_root, &["commit", "--quiet", "-m", "fixture"]);
    let approval_path = PathBuf::from("csdlc-v2/operator/generation-selector.json");
    let selector_path = repository_root.join("csdlc-v2/operator/generation-selector.json");
    fs::create_dir_all(selector_path.parent().expect("selector parent"))
        .expect("selector parent directory");
    let selector = if generation == "v3" {
        serde_json::json!({
            "schema": "csdlc.generation_selector.v2",
            "default_generation": "v3",
            "operational_authority": "csdlc-v3",
            "authority_issue": 505,
            "authority_pull_request": 591,
            "review_authority": "typed-v2-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "opted_in_issues": []
        })
    } else {
        serde_json::json!({
            "schema": "csdlc.generation_selector.v1",
            "default_generation": generation,
            "opted_in_issues": []
        })
    };
    let selector_bytes = serde_json::to_vec_pretty(&selector).expect("selector JSON");
    fs::write(&selector_path, &selector_bytes).expect("canonical selector");
    run_git(
        &repository_root,
        &["add", "csdlc-v2/operator/generation-selector.json"],
    );
    run_git(
        &repository_root,
        &["commit", "--quiet", "-m", "canonical selector"],
    );
    let exact_head = run_git(&repository_root, &["rev-parse", "HEAD"]);
    run_git(
        &repository_root,
        &["update-ref", "refs/remotes/origin/main", &exact_head],
    );
    let selector_digest = blake3::hash(&selector_bytes).to_hex().to_string();
    fs::create_dir_all(repository_root.join(".csdlc")).expect("state root");

    let context = OperationalLocalContext {
        repository_root: repository_root.clone(),
        state_root: repository_root.join(".csdlc"),
        allowed_worktree_parent: fixture.join("worktrees"),
        expected_authority_selector_digest: selector_digest.clone(),
        cutover_approval_path: approval_path,
        expected_cutover_approval_digest: selector_digest.clone(),
        expected_head_sha: exact_head,
        expected_lifecycle_digest: None,
    };
    fs::create_dir_all(&context.allowed_worktree_parent).expect("allowed worktree parent");
    let registry = operational_registry(&fixture);
    (repository_root, generation.to_owned(), context, registry)
}

#[test]
fn operational_context_rejects_legacy_boolean_and_forged_fields() {
    let (_, _, context, _) = operational_authority_fixture("forged-context", "v3");
    let mut value = serde_json::to_value(&context).expect("serialize operational context");
    value["cutover_approved"] = serde_json::Value::Bool(true);
    assert!(serde_json::from_value::<OperationalLocalContext>(value).is_err());

    let roundtrip = serde_json::from_slice::<OperationalLocalContext>(
        &serde_json::to_vec(&context).expect("serialize operational context"),
    )
    .expect("context is a serializable CLI adapter contract");
    assert_eq!(roundtrip, context);
}

#[test]
fn pre_cutover_v2_selector_denies_every_operational_local_route() {
    let (_, _, context, registry) = operational_authority_fixture("v2-denies-routes", "v2");
    let request = request();
    for route in LOCAL_ROUTE_NAMES {
        let findings = execute_operational_local_route(route, &request, &registry, &context)
            .expect_err("v2 selector must deny every operational local route");
        assert_eq!(findings[0].code, "canonical_v3_authority_inactive");
    }
    assert!(
        !context.state_root.join("issues").exists(),
        "authority denial must precede lifecycle state writes"
    );
}

#[test]
fn operational_local_authority_rejects_stale_selector_approval_and_head_digests() {
    let (_, _, context, registry) = operational_authority_fixture("stale-authority", "v3");
    let request = request();

    let mut stale_selector = context.clone();
    stale_selector.expected_authority_selector_digest = "0".repeat(64);
    let findings = execute_operational_local_route("issue", &request, &registry, &stale_selector)
        .expect_err("stale selector digest must fail closed");
    assert_eq!(
        findings[0].code,
        "canonical_authority_selector_digest_mismatch"
    );

    let mut stale_approval = context.clone();
    stale_approval.expected_cutover_approval_digest = "1".repeat(64);
    let findings = execute_operational_local_route("issue", &request, &registry, &stale_approval)
        .expect_err("stale approval digest must fail closed");
    assert_eq!(findings[0].code, "cutover_authority_mismatch");

    let mut stale_head = context;
    stale_head.expected_head_sha = "2".repeat(40);
    let findings = execute_operational_local_route("issue", &request, &registry, &stale_head)
        .expect_err("stale head must fail closed");
    assert_eq!(findings[0].code, "operational_exact_head_mismatch");
}

#[test]
fn operational_local_authority_rejects_stale_lifecycle_digest() {
    let (_, _, mut context, registry) = operational_authority_fixture("stale-lifecycle", "v3");
    write_lifecycle_state(&context.repository_root, 503, "ready", "observed-digest");
    let mut request = request();
    request.expected_lifecycle_digest = Some("stale-digest".into());
    context.expected_lifecycle_digest = request.expected_lifecycle_digest.clone();

    let findings = execute_operational_local_route("bind", &request, &registry, &context)
        .expect_err("stale lifecycle digest must fail closed");
    assert_eq!(findings[0].code, "stale_local_lifecycle_digest");
}

#[test]
fn operational_local_cas_recomputes_card_bytes_before_mutation() {
    let (_, _, mut context, registry) = operational_authority_fixture("card-byte-tamper", "v3");
    let initialized = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect("initialize authoritative lifecycle state");
    fs::write(
        context.state_root.join("issues/503/cards/sip.md"),
        "tampered after persisted digest",
    )
    .expect("tamper retained card bytes");
    let mut guarded = request();
    guarded.expected_lifecycle_digest = initialized.digest.clone();
    context.expected_lifecycle_digest = initialized.digest;

    let findings = execute_operational_local_route("validate", &guarded, &registry, &context)
        .expect_err("card tampering must invalidate operational CAS");
    assert_eq!(findings[0].code, "stale_local_lifecycle_digest");
}

#[test]
fn operational_issue_lock_serializes_competing_initializers() {
    let (_, _, context, registry) = operational_authority_fixture("issue-lock-race", "v3");
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers = [0, 1].map(|_| {
        let context = context.clone();
        let registry = registry.clone();
        let barrier = barrier.clone();
        std::thread::spawn(move || {
            barrier.wait();
            execute_operational_local_route("issue", &request(), &registry, &context)
        })
    });
    let outcomes = workers.map(|worker| worker.join().expect("initializer thread"));
    assert_eq!(outcomes.iter().filter(|outcome| outcome.is_ok()).count(), 1);
    assert_eq!(
        outcomes.iter().filter(|outcome| outcome.is_err()).count(),
        1
    );
    assert!(context.state_root.join("issues/503/index.json").is_file());
}

#[test]
fn operational_initialization_failure_never_publishes_partial_issue_root() {
    let (repository_root, _, context, mut registry) =
        operational_authority_fixture("initialization-transaction", "v3");
    let unreadable_template = repository_root.join("template-directory");
    fs::create_dir(&unreadable_template).expect("template directory");
    registry.template_paths.insert(
        "stp".into(),
        unreadable_template.to_string_lossy().into_owned(),
    );

    let findings = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect_err("staged initialization must fail on unreadable template");
    assert!(findings
        .iter()
        .any(|finding| finding.code == "template_read_failed"));
    assert!(!context.state_root.join("issues/503").exists());
    let issue_parent = context.state_root.join("issues");
    assert!(fs::read_dir(issue_parent)
        .expect("issue parent")
        .all(|entry| !entry
            .unwrap()
            .file_name()
            .to_string_lossy()
            .contains(".init-")));
}

#[test]
fn canonical_v3_selector_and_exact_approval_authorize_isolated_issue_initialization() {
    let (_, _, context, registry) = operational_authority_fixture("v3-authorizes", "v3");
    let result = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect("canonical v3 authority should permit isolated local execution");
    assert!(result.mutated);
    assert_eq!(result.phase.as_deref(), Some("ready"));
    assert!(context.state_root.join("issues/503/index.json").is_file());
}

#[test]
fn operational_schedule_preserves_the_six_dimension_readiness_denominator() {
    let (_, _, mut context, registry) = operational_authority_fixture("schedule-semantics", "v3");
    let initialized = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect("initialize schedule fixture");
    let mut scheduled = request();
    scheduled.expected_lifecycle_digest = initialized.digest.clone();
    context.expected_lifecycle_digest = initialized.digest;
    scheduled.schedule_readiness = Some(ScheduleReadinessInput {
        phase_ready: true,
        cards_ready: true,
        design_ready: true,
        dependencies_ready: true,
        paths_clear: true,
        budget_available: true,
    });
    let ready = execute_operational_local_route("schedule", &scheduled, &registry, &context)
        .expect("ready schedule report");
    let report = ready.routing.expect("typed schedule report");
    assert_eq!(report.schema, "csdlc.scheduler.report.v1");
    assert_eq!(report.state, "ready");
    assert_eq!(report.eligible_operations, ["validate"]);

    scheduled
        .schedule_readiness
        .as_mut()
        .unwrap()
        .dependencies_ready = false;
    let blocked = execute_operational_local_route("schedule", &scheduled, &registry, &context)
        .expect("blocked schedule is a typed routing result");
    let report = blocked.routing.expect("typed blocked schedule report");
    assert_eq!(report.state, "blocked");
    assert_eq!(report.blockers, ["dependencies_ready"]);
    assert!(report.eligible_operations.is_empty());
}

#[test]
fn operational_shepherd_preserves_v2_state_priority_and_operations() {
    let (_, _, mut context, registry) = operational_authority_fixture("shepherd-semantics", "v3");
    let initialized = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect("initialize shepherd fixture");
    let mut shepherded = request();
    shepherded.expected_lifecycle_digest = initialized.digest.clone();
    context.expected_lifecycle_digest = initialized.digest;
    shepherded.shepherd_routing = Some(ShepherdRoutingInput {
        validation: Some("passed".into()),
        dependency_wait: true,
        retryable_failure: true,
        repair_needed: true,
        operator_decision_needed: true,
    });
    let operator = execute_operational_local_route("shepherd", &shepherded, &registry, &context)
        .expect("operator-required shepherd report");
    let report = operator.routing.expect("typed shepherd report");
    assert_eq!(report.schema, "csdlc.shepherd.report.v1");
    assert_eq!(report.state, "operator_required");
    assert_eq!(report.eligible_operations, ["operator_decision"]);

    shepherded.shepherd_routing = Some(ShepherdRoutingInput {
        validation: Some("failed".into()),
        dependency_wait: false,
        retryable_failure: false,
        repair_needed: true,
        operator_decision_needed: false,
    });
    let repair = execute_operational_local_route("shepherd", &shepherded, &registry, &context)
        .expect("repair-required shepherd report");
    let report = repair.routing.expect("typed repair shepherd report");
    assert_eq!(report.state, "repair_required");
    assert_eq!(report.eligible_operations, ["repair"]);

    shepherded.shepherd_routing = Some(ShepherdRoutingInput {
        validation: Some("failed".into()),
        dependency_wait: false,
        retryable_failure: true,
        repair_needed: false,
        operator_decision_needed: false,
    });
    let retryable = execute_operational_local_route("shepherd", &shepherded, &registry, &context)
        .expect("retryable shepherd report");
    let report = retryable.routing.expect("typed retryable shepherd report");
    assert_eq!(report.state, "retryable");
    assert_eq!(report.eligible_operations, ["retry"]);

    shepherded.shepherd_routing = Some(ShepherdRoutingInput {
        validation: None,
        dependency_wait: false,
        retryable_failure: false,
        repair_needed: false,
        operator_decision_needed: false,
    });
    let waiting = execute_operational_local_route("shepherd", &shepherded, &registry, &context)
        .expect("waiting shepherd report");
    let report = waiting.routing.expect("typed waiting shepherd report");
    assert_eq!(report.state, "waiting");
    assert!(report.eligible_operations.is_empty());

    shepherded.shepherd_routing = Some(ShepherdRoutingInput {
        validation: Some("passed".into()),
        dependency_wait: false,
        retryable_failure: false,
        repair_needed: false,
        operator_decision_needed: false,
    });
    let ready = execute_operational_local_route("shepherd", &shepherded, &registry, &context)
        .expect("ready shepherd report");
    let report = ready.routing.expect("typed ready shepherd report");
    assert_eq!(report.state, "ready");
    assert_eq!(report.eligible_operations, ["schedule"]);
}

#[cfg(unix)]
#[test]
fn operational_local_authority_rejects_state_root_symlink_escape() {
    let (repository_root, _, mut context, registry) =
        operational_authority_fixture("state-root-symlink-escape", "v3");
    let outside = repository_root.parent().unwrap().join("outside-state");
    fs::create_dir_all(&outside).expect("outside state directory");
    let state_link = repository_root.join(".csdlc/escaped-state");
    std::os::unix::fs::symlink(&outside, &state_link).expect("state symlink");
    context.state_root = state_link;

    let findings = execute_operational_local_route("issue", &request(), &registry, &context)
        .expect_err("state root symlink escape must fail closed");
    assert_eq!(findings[0].code, "state_root_outside_repository");
}
