use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::process::Command;

use csdlc_v3::commands::local::{
    authorize_bind, grants_operational_authority, plan_cards, prepare_local_workflow,
    validate_contract, LocalCommand, LocalPreparationRequest, PlanStatus, PromptRegistry,
    WorktreeRegistration,
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

fn write_remote_delivery_evidence(dir: &Path, operation: &str) -> PathBuf {
    let repo = dir.join("repo");
    let worktree = dir.join("worktree");
    let common = repo.join(".git").join("worktrees").join("worktree");
    fs::create_dir_all(&common).expect("common gitdir");
    fs::create_dir_all(&worktree).expect("worktree");
    fs::write(
        worktree.join(".git"),
        format!("gitdir: {}\n", common.display()),
    )
    .expect("worktree git pointer");
    fs::write(
        common.join("gitdir"),
        format!("{}/.git\n", worktree.display()),
    )
    .expect("common git pointer");

    let revision = "974abc520454690f0b392162b9ced783e8584017";
    fs::write(
        dir.join("pvf.json"),
        serde_json::json!({
            "schema": "csdlc.v3.pvf_result.v1",
            "issue": 504,
            "revision": revision,
            "evidence_digest": "accepted-pvf-digest"
        })
        .to_string(),
    )
    .expect("pvf evidence");
    fs::write(
        dir.join("review.json"),
        serde_json::json!({
            "schema": "csdlc.v3.accepted_review.v1",
            "issue": 504,
            "reviewed_revision": revision,
            "scope_paths": [
                "csdlc-v3/src/commands/remote",
                "csdlc-v3/src/review",
                "csdlc-v3/src/publication",
                "csdlc-v3/tests/remote_commands"
            ],
            "implementer": "worker-6-implementation",
            "reviewer": "independent-reviewer",
            "findings": [{"id": "clean", "disposition": "resolved"}],
            "target": {
                "repository": "agent-logic/agent-design-language",
                "issue": 504,
                "mode": "closing"
            },
            "typed_review_evidence_digest": "typed-review-digest"
        })
        .to_string(),
    )
    .expect("review evidence");
    fs::write(
        dir.join("publication.json"),
        serde_json::json!({
            "schema": "csdlc.v3.publication_intent.v1",
            "repository": "agent-logic/agent-design-language",
            "issue": 504,
            "pull_request": 586,
            "mode": "closing",
            "publisher": "worker-6-publisher",
            "body": "Closes #504",
            "head_sha": revision
        })
        .to_string(),
    )
    .expect("publication evidence");
    fs::write(
        dir.join("pr-readback.json"),
        serde_json::json!({
            "schema": "csdlc.v3.pr_readback.v1",
            "repository": "agent-logic/agent-design-language",
            "number": 586,
            "head_sha": revision,
            "merged": true,
            "closes_issue": 504,
            "part_of_issue": null
        })
        .to_string(),
    )
    .expect("pr readback");
    fs::write(
        dir.join("issue-readback.json"),
        serde_json::json!({
            "schema": "csdlc.v3.issue_readback.v1",
            "repository": "agent-logic/agent-design-language",
            "issue": 504,
            "open": false
        })
        .to_string(),
    )
    .expect("issue readback");
    fs::write(
        dir.join("cleanup.json"),
        serde_json::json!({
            "schema": "csdlc.v3.cleanup_inspection.v1",
            "preview": true,
            "preview_receipt": false,
            "committed_closed_out": true,
            "terminal_receipt": true,
            "approved_worktree_parent": dir.display().to_string(),
            "registration": {
                "repository_root": repo.display().to_string(),
                "worktree_path": worktree.display().to_string(),
                "git_common_dir": common.display().to_string()
            },
            "candidate_path": worktree.display().to_string(),
            "preview_identity_digest": null,
            "dirty": false,
            "live": false
        })
        .to_string(),
    )
    .expect("cleanup inspection");

    let request_path = dir.join(format!("remote-request-{operation}.json"));
    fs::write(
        &request_path,
        serde_json::json!({
            "repository": "agent-logic/agent-design-language",
            "issue": 504,
            "pull_request": 586,
            "head_sha": revision,
            "mode": "closing",
            "operation": operation,
            "pvf_evidence_ref": format!("csdlc-v3/target/local-command-fixtures/{}/pvf.json", dir.file_name().unwrap().to_string_lossy()),
            "typed_review_ref": format!("csdlc-v3/target/local-command-fixtures/{}/review.json", dir.file_name().unwrap().to_string_lossy()),
            "publication_intent_ref": format!("csdlc-v3/target/local-command-fixtures/{}/publication.json", dir.file_name().unwrap().to_string_lossy()),
            "pr_readback_ref": format!("csdlc-v3/target/local-command-fixtures/{}/pr-readback.json", dir.file_name().unwrap().to_string_lossy()),
            "issue_readback_ref": format!("csdlc-v3/target/local-command-fixtures/{}/issue-readback.json", dir.file_name().unwrap().to_string_lossy()),
            "cleanup_inspection_ref": format!("csdlc-v3/target/local-command-fixtures/{}/cleanup.json", dir.file_name().unwrap().to_string_lossy())
        })
        .to_string(),
    )
    .expect("remote request");
    request_path
}

fn request() -> LocalPreparationRequest {
    LocalPreparationRequest {
        issue: 503,
        title: "[v0.92.1][V3-D] C-SDLC v3 local preparation workflow".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: "codex/503-v3-d-local-preparation-workflow-exec".into(),
        worktree: "adl-worktrees/adl-issue-503-v3-d-local-preparation-workflow-exec".into(),
        registry_version: "1.0.3".into(),
        commands: vec![
            LocalCommand::PrepareIssue,
            LocalCommand::BindWorktree,
            LocalCommand::PlanPvf,
            LocalCommand::Doctor,
        ],
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

#[test]
fn contract_commands_are_typed_and_non_authoritative() {
    let json = serde_json::to_vec(&request()).expect("request serializes");
    let request = LocalPreparationRequest::from_json(&json).expect("typed request parses");
    validate_contract(&request).expect("valid typed local command contract");
    for command in request.commands {
        assert!(!grants_operational_authority(command));
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
    assert_eq!(value["read_only"], true);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["result"]["issue"], 503);
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

#[test]
fn remote_delivery_cli_verifies_repo_local_bridge_evidence_refs() {
    let dir = fixture_dir("remote-bridge-success");
    let request_path = write_remote_delivery_evidence(&dir, "verify_bridge_evidence");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("remote")
        .arg("--repo-root")
        .arg(repo_root())
        .arg("--request")
        .arg(&request_path)
        .output()
        .expect("run remote delivery CLI");
    assert!(output.status.success(), "{output:?}");
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable json");
    assert_eq!(value["schema"], "csdlc.v3.remote_delivery.v1");
    assert_eq!(value["status"], "ready_for_typed_bridge");
    assert_eq!(value["trusted_authority"], false);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(
        value["blockers"][0],
        "v3 remote command is pre-cutover verification only; typed C-SDLC v2 remains operational authority until #505 explicitly switches authority"
    );
    assert_eq!(
        value["evidence_refs"]["pvf_evidence_ref"],
        "csdlc-v3/target/local-command-fixtures/remote-bridge-success/pvf.json"
    );
    assert_eq!(
        value["evidence_digest"]
            .as_str()
            .expect("evidence digest")
            .len(),
        64
    );
}

#[test]
fn remote_delivery_cli_derives_end_to_end_terminal_workflow_from_typed_evidence() {
    let dir = fixture_dir("remote-delivery-derived");
    let request_path = write_remote_delivery_evidence(&dir, "deliver");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("remote")
        .arg("--repo-root")
        .arg(repo_root())
        .arg("--request")
        .arg(&request_path)
        .output()
        .expect("run remote delivery CLI");
    assert!(output.status.success(), "{output:?}");
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable json");
    assert_eq!(value["schema"], "csdlc.v3.remote_delivery.v1");
    assert_eq!(value["status"], "delivery_derived");
    assert_eq!(value["trusted_authority"], false);
    assert_eq!(value["operational_authority"], false);
    assert_eq!(value["result"]["mutation_allowed"], false);
    assert!(value["result"]["finish"]
        .as_str()
        .expect("finish result")
        .contains("TerminalClosedOut"));
    assert!(value["result"]["cleanup"]
        .as_str()
        .expect("cleanup result")
        .contains("PreviewEligible"));
}

#[test]
fn remote_delivery_cli_rejects_caller_forged_authority_refs() {
    let dir = fixture_dir("remote-bridge-forged");
    let request_path = dir.join("remote-request.json");
    fs::write(
        &request_path,
        serde_json::json!({
            "repository": "agent-logic/agent-design-language",
            "issue": 504,
            "pull_request": 586,
            "head_sha": "974abc520454690f0b392162b9ced783e8584017",
            "mode": "closing",
            "operation": "deliver",
            "pvf_evidence_ref": "caller:accepted-pvf",
            "typed_review_ref": "caller:review",
            "publication_intent_ref": "caller:publication",
            "pr_readback_ref": "caller:pr-readback",
            "issue_readback_ref": "caller:issue-readback",
            "cleanup_inspection_ref": "caller:cleanup"
        })
        .to_string(),
    )
    .expect("write forged remote request");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("remote")
        .arg("--repo-root")
        .arg(repo_root())
        .arg("--request")
        .arg(&request_path)
        .output()
        .expect("run remote delivery CLI");
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CallerForgedAuthority"));
    assert!(output.stdout.is_empty());
}

#[test]
fn remote_delivery_cli_rejects_schema_less_evidence_refs() {
    let dir = fixture_dir("remote-bridge-schema-less");
    for name in [
        "pvf.json",
        "review.json",
        "publication.json",
        "pr-readback.json",
        "issue-readback.json",
        "cleanup.json",
    ] {
        fs::write(dir.join(name), br#"{"not_schema":"fixture"}"#).expect("write evidence fixture");
    }
    let request_path = dir.join("remote-request.json");
    fs::write(
        &request_path,
        serde_json::json!({
            "repository": "agent-logic/agent-design-language",
            "issue": 504,
            "pull_request": 586,
            "head_sha": "974abc520454690f0b392162b9ced783e8584017",
            "mode": "closing",
            "operation": "verify_bridge_evidence",
            "pvf_evidence_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/pvf.json",
            "typed_review_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/review.json",
            "publication_intent_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/publication.json",
            "pr_readback_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/pr-readback.json",
            "issue_readback_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/issue-readback.json",
            "cleanup_inspection_ref": "csdlc-v3/target/local-command-fixtures/remote-bridge-schema-less/cleanup.json"
        })
        .to_string(),
    )
    .expect("write remote request");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("remote")
        .arg("--repo-root")
        .arg(repo_root())
        .arg("--request")
        .arg(&request_path)
        .output()
        .expect("run remote delivery CLI");
    assert!(!output.status.success(), "{output:?}");
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("EvidenceRefSchemaMissing"));
    assert!(output.stdout.is_empty());
}
