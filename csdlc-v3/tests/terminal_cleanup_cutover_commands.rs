use csdlc_v3::{
    adapters::{CommandInvocation, ProcessAdapter, ProcessOutput, ProcessStatus},
    commands::{
        proof::{
            classify_route, ProofManifest, ProofRouteRequest, ProofRouteStatus, ShadowCommandSpec,
            ShadowGeneration, ShadowNormalizationContract,
        },
        terminal::{
            prepare_terminal_cutover_with_github_observation,
            prepare_terminal_finish_with_github_observation, prepare_terminal_route,
            CleanupDecision, CleanupRouteRequest, CutoverDecisionRequest, CutoverOperation,
            DurableTerminalReceipt, FinishDecision, TerminalPublicationMode, TerminalRouteRequest,
            TerminalRouteStatus, TerminalStateWriteRequest,
        },
    },
};
use fs2::FileExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

fn base_request() -> TerminalRouteRequest {
    TerminalRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 630,
        pull_request: Some(641),
        expected_head_sha: Some("0123456789012345678901234567890123456789".into()),
        mode: Some(TerminalPublicationMode::Closing),
        public_adapter_receipt: None,
        terminal_state: None,
        cleanup: None,
        cutover: None,
        credential_names: Vec::new(),
    }
}

#[test]
fn public_finish_fails_closed_without_authenticated_adapter() {
    let request = base_request();
    let plan = prepare_terminal_route("finish", &request).expect("finish route");
    assert_eq!(plan.status, TerminalRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_adapter_required"));
    assert_eq!(
        plan.finish,
        Some(FinishDecision::CheckpointDenied {
            reason: "public finish route has no authenticated adapter receipt".into()
        })
    );
}

#[test]
fn part_of_publication_cannot_terminally_close() {
    let mut request = base_request();
    request.mode = Some(TerminalPublicationMode::PartOf);
    let plan = prepare_terminal_route("finish", &request).expect("finish plan");
    assert_eq!(
        plan.finish,
        Some(FinishDecision::CheckpointDenied {
            reason: "public finish route has no authenticated adapter receipt".into()
        })
    );
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "terminal_requires_closing_publication"));
}

#[test]
fn public_finish_requires_exact_closing_inputs_before_adapter_boundary() {
    let mut request = base_request();
    request.expected_head_sha = None;
    request.pull_request = None;
    let plan = prepare_terminal_route("finish", &request).expect("finish plan");
    assert_eq!(plan.status, TerminalRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_pull_request"));
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_exact_head"));
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_adapter_required"));
}

#[test]
fn public_finish_has_no_direct_verified_readback_constructor() {
    let terminal_source = include_str!("../src/commands/terminal.rs");
    assert!(terminal_source.contains("pub(crate) fn from_typed_adapter_receipt"));
    assert!(!terminal_source.contains("pub fn from_typed_adapter_receipt"));
}

#[test]
fn retained_unit_tests_cover_verified_readback_denials() {
    let terminal_source = include_str!("../src/commands/terminal.rs");
    assert!(terminal_source
        .contains("terminal_verified_readback_can_derive_closeout_inside_adapter_boundary"));
    assert!(terminal_source
        .contains("terminal_verified_readback_denies_stale_nonmerged_and_open_issue"));
    assert!(terminal_source.contains("\"head_mismatch\""));
    assert!(terminal_source.contains("\"pull_request_not_merged\""));
    assert!(terminal_source.contains("\"closing_issue_still_open\""));
}

#[test]
fn observed_finish_derives_terminal_closeout_from_github_pr_and_issue_readbacks() {
    let mut request = base_request();
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    let mut adapter = FakeGithubAdapter::new([
        github_pr_json(
            641,
            "0123456789012345678901234567890123456789",
            true,
            "Closes #630",
        ),
        github_issue_json(630, "closed"),
    ]);

    let plan =
        prepare_terminal_finish_with_github_observation(&request, &mut adapter).expect("finish");

    assert_eq!(plan.status, TerminalRouteStatus::Ready, "{plan:#?}");
    assert_eq!(
        plan.finish,
        Some(FinishDecision::TerminalClosedOut {
            pull_request: 641,
            issue: 630,
            head_sha: "0123456789012345678901234567890123456789".into()
        })
    );
    assert_eq!(
        adapter.invocations,
        vec![
            vec![
                "pull-request".to_owned(),
                "agent-logic/agent-design-language".to_owned(),
                "641".to_owned()
            ],
            vec![
                "issue".to_owned(),
                "agent-logic/agent-design-language".to_owned(),
                "630".to_owned()
            ],
        ]
    );
}

#[test]
fn observed_finish_denies_nonmerged_or_open_issue_readbacks() {
    let mut request = base_request();
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    let mut nonmerged = FakeGithubAdapter::new([
        github_pr_json(
            641,
            "0123456789012345678901234567890123456789",
            false,
            "Closes #630",
        ),
        github_issue_json(630, "closed"),
    ]);
    let nonmerged_plan =
        prepare_terminal_finish_with_github_observation(&request, &mut nonmerged).unwrap();
    assert_eq!(nonmerged_plan.status, TerminalRouteStatus::Blocked);
    assert!(nonmerged_plan
        .findings
        .iter()
        .any(|finding| finding.code == "pull_request_not_merged"));

    let mut open_issue = FakeGithubAdapter::new([
        github_pr_json(
            641,
            "0123456789012345678901234567890123456789",
            true,
            "Closes #630",
        ),
        github_issue_json(630, "open"),
    ]);
    let open_issue_plan =
        prepare_terminal_finish_with_github_observation(&request, &mut open_issue).unwrap();
    assert_eq!(open_issue_plan.status, TerminalRouteStatus::Blocked);
    assert!(open_issue_plan
        .findings
        .iter()
        .any(|finding| finding.code == "closing_issue_still_open"));
}

#[test]
fn cleanup_denies_nonexistent_parent_traversal_escape() {
    let fixture = fixture_root("cleanup_parent_traversal_escape");
    let approved = fixture.join("approved");
    let primary = fixture.join("primary");
    fs::create_dir_all(&fixture).expect("fixture root");
    fs::create_dir_all(&approved).expect("approved parent");
    init_repo(&primary);

    let plan = cleanup_plan(
        &approved,
        &primary,
        &approved.join("missing").join("..").join("escape"),
        false,
        None,
        None,
    );
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "path_not_normalized"));
}

#[test]
fn cleanup_denies_existing_relative_candidate_before_canonicalization() {
    let fixture = fixture_root("cleanup_existing_relative_candidate");
    let approved = fixture.join("approved");
    let primary = fixture.join("primary");
    let candidate = approved.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    fs::create_dir_all(&approved).expect("approved parent");
    init_repo(&primary);
    let head = git_stdout(&primary, &["rev-parse", "HEAD"]);
    let receipt = write_terminal_receipt(&primary, 630, 641, &head);
    git(&primary, &["worktree", "add", candidate.to_str().unwrap()]);

    let relative_candidate = candidate
        .strip_prefix(std::env::current_dir().expect("current dir"))
        .expect("candidate should be relative to repository cwd")
        .to_path_buf();
    let plan = cleanup_plan(
        &approved,
        &primary,
        &relative_candidate,
        false,
        Some(receipt.clone()),
        None,
    );
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "path_not_normalized"));

    let absolute = cleanup_plan(&approved, &primary, &candidate, false, Some(receipt), None);
    let eligible = prepare_terminal_route("clean", &absolute).expect("absolute clean plan");
    assert_eq!(eligible.status, TerminalRouteStatus::Ready);
    assert!(matches!(
        eligible.cleanup,
        Some(CleanupDecision::Live { .. }) | Some(CleanupDecision::Removable { .. })
    ));
}

#[test]
fn cleanup_uses_git_registration_and_preserves_distinct_outcomes() {
    let fixture = fixture_root("cleanup_distinct");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    let unregistered = fixture.join("unregistered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    let head = git_stdout(&primary, &["rev-parse", "HEAD"]);
    let receipt = write_terminal_receipt(&primary, 630, 641, &head);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);
    fs::create_dir_all(&unregistered).expect("unregistered dir");

    let dirty = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    fs::write(registered.join("dirty.txt"), "dirty\n").expect("dirty marker");
    assert!(matches!(
        prepare_terminal_route("clean", &dirty).unwrap().cleanup,
        Some(CleanupDecision::Dirty { .. })
    ));
    fs::remove_file(registered.join("dirty.txt")).expect("remove dirty marker");

    fs::create_dir_all(registered.join(".csdlc")).expect("marker parent");
    fs::write(registered.join(".csdlc/live-worktree.marker"), "live\n").expect("live marker");
    let live = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &live).unwrap().cleanup,
        Some(CleanupDecision::Live { .. })
    ));
    fs::remove_file(registered.join(".csdlc/live-worktree.marker")).expect("remove live marker");

    let preview = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    let preview_plan = prepare_terminal_route("clean", &preview).expect("preview");
    let receipt_digest = match preview_plan.cleanup {
        Some(CleanupDecision::Removable { receipt_digest, .. }) => receipt_digest,
        other => panic!("expected removable preview, got {other:?}"),
    };

    let wrong_receipt = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        true,
        Some(receipt.clone()),
        Some("wrong".into()),
    );
    assert!(prepare_terminal_route("clean", &wrong_receipt)
        .unwrap()
        .findings
        .iter()
        .any(|finding| finding.code == "preview_receipt_mismatch"));

    let remove = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        true,
        Some(receipt.clone()),
        Some(receipt_digest.clone()),
    );
    let denied_remove = prepare_terminal_route("clean", &remove).expect("denied remove");
    assert_eq!(denied_remove.status, TerminalRouteStatus::Blocked);
    assert!(denied_remove
        .findings
        .iter()
        .any(|finding| finding.code == "cleanup_removal_denied_pre_cutover"));
    assert!(matches!(
        denied_remove.cleanup,
        Some(CleanupDecision::RemovalDeniedPreCutover { .. })
    ));
    assert!(registered.exists());
    let post_denial_preview = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &post_denial_preview)
            .unwrap()
            .cleanup,
        Some(CleanupDecision::Removable {
            receipt_digest: digest,
            ..
        }) if digest == receipt_digest
    ));

    fs::remove_dir_all(&registered).expect("simulate external already-removed state");

    let already_removed = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &already_removed)
            .unwrap()
            .cleanup,
        Some(CleanupDecision::AlreadyRemoved { .. })
    ));

    let absent = cleanup_plan(
        &fixture,
        &primary,
        &fixture.join("never-created"),
        false,
        Some(receipt.clone()),
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &absent).unwrap().cleanup,
        Some(CleanupDecision::Absent { .. })
    ));

    let unregistered_plan = cleanup_plan(
        &fixture,
        &primary,
        &unregistered,
        false,
        Some(receipt),
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &unregistered_plan)
            .unwrap()
            .cleanup,
        Some(CleanupDecision::Unregistered { .. })
    ));
}

#[test]
fn cleanup_rejects_registered_worktree_at_stale_head() {
    let fixture = fixture_root("cleanup_stale_candidate_head");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    let stale_head = "0123456789012345678901234567890123456789";
    let receipt = write_terminal_receipt(&primary, 630, 641, stale_head);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);

    let plan = cleanup_plan(&fixture, &primary, &registered, false, Some(receipt), None);
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "candidate_head_mismatch"));
}

#[test]
fn cleanup_denies_symlink_escape_from_approved_parent() {
    let fixture = fixture_root("cleanup_symlink_escape");
    let approved = fixture.join("approved");
    let primary = fixture.join("primary");
    let outside = fixture.join("outside");
    let escape = approved.join("escape-link");
    fs::create_dir_all(&fixture).expect("fixture root");
    fs::create_dir_all(&approved).expect("approved parent");
    fs::create_dir_all(&outside).expect("outside target");
    init_repo(&primary);
    let receipt = write_terminal_receipt(
        &primary,
        630,
        641,
        "0123456789012345678901234567890123456789",
    );
    create_symlink(&outside, &escape);

    let plan = cleanup_plan(&approved, &primary, &escape, false, Some(receipt), None);
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "path_outside_approved_parent"));
}

#[test]
fn cleanup_denies_nonexistent_child_under_symlink_escape() {
    let fixture = fixture_root("cleanup_symlink_child_escape");
    let approved = fixture.join("approved");
    let primary = fixture.join("primary");
    let outside = fixture.join("outside");
    let escape = approved.join("escape-link");
    fs::create_dir_all(&fixture).expect("fixture root");
    fs::create_dir_all(&approved).expect("approved parent");
    fs::create_dir_all(&outside).expect("outside target");
    init_repo(&primary);
    let receipt = write_terminal_receipt(
        &primary,
        630,
        641,
        "0123456789012345678901234567890123456789",
    );
    create_symlink(&outside, &escape);

    let plan = cleanup_plan(
        &approved,
        &primary,
        &escape.join("missing"),
        false,
        Some(receipt),
        None,
    );
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "path_outside_approved_parent"));
}

#[test]
fn cutover_requires_operator_approval_rollback_and_fail_closed_undo() {
    let mut request = base_request();
    request.cutover = Some(CutoverDecisionRequest {
        operator: "worker-6".into(),
        approval: "approved".into(),
        selected_binary_provenance: "git:abc".into(),
        rollback_evidence: "".into(),
        undo_boundary: "manual undo".into(),
        operation: CutoverOperation::Apply,
        execute: false,
        repository_root: None,
        selected_binary_path: None,
        authority_selector_path: None,
        install_destination_path: None,
        rollback_receipt_path: None,
        readiness_evidence_path: None,
        readiness_evidence_digest: None,
    });
    let blocked = prepare_terminal_route("cutover", &request).expect("cutover plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_505_approval"));
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_rollback_evidence"));
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_fail_closed_undo"));

    request.cutover = Some(CutoverDecisionRequest {
        operator: "worker-6".into(),
        approval: "#505 operator-reviewed approval".into(),
        selected_binary_provenance: "git:0123456789012345678901234567890123456789".into(),
        rollback_evidence: "v2 rollback target verified".into(),
        undo_boundary: "fail-closed before any irreversible mutation".into(),
        operation: CutoverOperation::Apply,
        execute: false,
        repository_root: None,
        selected_binary_path: None,
        authority_selector_path: None,
        install_destination_path: None,
        rollback_receipt_path: None,
        readiness_evidence_path: None,
        readiness_evidence_digest: None,
    });
    let ready = prepare_terminal_route("cutover", &request).expect("cutover plan");
    assert_eq!(ready.status, TerminalRouteStatus::Ready);
    assert!(!ready.cutover.unwrap().executes_cutover);
}

#[test]
fn executable_cutover_cannot_bypass_request_findings() {
    let root = fixture_root("cutover-preview-gate");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let mut request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);
    let cutover = request.cutover.as_mut().unwrap();
    cutover.operator.clear();
    cutover.undo_boundary = "manual undo".into();

    let blocked = prepare_terminal_route("cutover", &request).expect("cutover plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_operator"));
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_fail_closed_undo"));
    assert!(!root.join(".adl/bin/csdlc").exists());
    let selector: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["generation"], "v3");
}

#[test]
fn executable_cutover_requires_authenticated_github_authority_before_mutation() {
    let root = fixture_root("cutover-authenticated-authority-gate");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);

    let blocked = prepare_terminal_route("cutover", &request).expect("blocked cutover plan");

    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_cutover_observation_required"));
    assert!(!root.join(".adl/bin/csdlc").exists());
    let selector: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["generation"], "v3");
}

#[test]
fn authenticated_cutover_rejects_unmerged_authority_pr() {
    let root = fixture_root("cutover-merged-authority-gate");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let mut request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);
    request.pull_request = Some(591);
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    let revision = request
        .cutover
        .as_ref()
        .unwrap()
        .selected_binary_provenance
        .trim_start_matches("git:");
    let mut adapter = FakeGithubAdapter::new([
        serde_json::json!({"number": 591, "merged": false, "head": {"sha": revision}}).to_string(),
        serde_json::json!({"number": 505, "state": "closed"}).to_string(),
    ]);

    let blocked = prepare_terminal_cutover_with_github_observation(&request, &mut adapter)
        .expect("blocked cutover plan");

    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "cutover_approval_not_exact"));
    assert!(!root.join(".adl/bin/csdlc").exists());
}

#[test]
fn cutover_and_rollback_share_one_mutation_lock() {
    let root = fixture_root("cutover-shared-mutation-lock");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    fs::create_dir_all(root.join(".git/csdlc-v3")).unwrap();
    let holder = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .open(root.join(".git/csdlc-v3/cutover-mutation.lock"))
        .unwrap();
    holder.lock_exclusive().unwrap();
    let request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);

    let blocked = execute_cutover_request(&request).expect("cutover plan");

    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "cutover_mutation_locked"));
    assert!(!root.join(".adl/bin/csdlc").exists());

    drop(holder);
    let retry = execute_cutover_request(&request).expect("released advisory lock permits retry");
    assert_eq!(retry.status, TerminalRouteStatus::Ready, "{retry:#?}");
}

#[test]
fn approved_cutover_atomically_installs_selector_and_rollback_receipt() {
    let root = fixture_root("approved_cutover_execution");
    fs::create_dir_all(root.join(".git")).expect("git marker");
    fs::create_dir_all(root.join("build")).expect("build directory");
    fs::write(root.join("build/csdlc"), b"v3-binary").expect("selected binary");
    let binary_digest = blake3::hash(b"v3-binary").to_hex().to_string();
    let readiness = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.authority_readiness.v1",
        "authority_issue": 505,
        "all_operational": true,
        "v3_only_canary_passed": true,
        "independent_exact_head_review_passed": true,
        "selected_binary_digest": binary_digest,
        "operational_routes": [
            "bind", "clean", "cutover", "doctor", "edit", "eligibility", "finish",
            "github", "github-issue", "github-pr", "install", "issue", "pr-state",
            "proof", "publish", "review", "schedule", "shadow", "shepherd", "soak",
            "validate"
        ]
    }))
    .unwrap();
    fs::write(root.join("build/readiness.json"), &readiness).expect("readiness evidence");
    let readiness_digest = blake3::hash(&readiness).to_hex().to_string();

    let mut request = base_request();
    request.issue = 505;
    request.cutover = Some(CutoverDecisionRequest {
        operator: "operator".into(),
        approval: "#505 operator-reviewed approval".into(),
        selected_binary_provenance: "git:0123456789012345678901234567890123456789".into(),
        rollback_evidence: "typed C-SDLC v2 rollback target verified".into(),
        undo_boundary: "fail-closed before irreversible mutation".into(),
        operation: CutoverOperation::Apply,
        execute: true,
        repository_root: Some(root.clone()),
        selected_binary_path: Some(PathBuf::from("build/csdlc")),
        authority_selector_path: Some(PathBuf::from(".csdlc/authority-selector.json")),
        install_destination_path: Some(PathBuf::from(".adl/bin/csdlc")),
        rollback_receipt_path: Some(PathBuf::from(".csdlc/evidence/505/cutover-receipt.json")),
        readiness_evidence_path: Some(PathBuf::from("build/readiness.json")),
        readiness_evidence_digest: Some(readiness_digest),
    });

    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);

    let plan = execute_cutover_request(&request).expect("cutover route");
    assert_eq!(plan.status, TerminalRouteStatus::Ready, "{plan:#?}");
    assert!(plan.operational_authority);
    assert!(plan.cutover.expect("cutover decision").executes_cutover);
    assert_eq!(
        fs::read(root.join(".adl/bin/csdlc")).unwrap(),
        fs::read(env!("CARGO_BIN_EXE_csdlc")).unwrap()
    );
    let selector: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["schema"], "csdlc.v3.authority_selector.v1");
    assert_eq!(selector["generation"], "v3");
    assert_eq!(selector["operational_authority"], "csdlc-v3");
    assert_eq!(selector["authority_issue"], 505);
    assert_eq!(selector["authority_pull_request"], 591);
    assert_eq!(selector["review_authority"], "typed-exact-head");
    assert_eq!(
        selector["approval_authority"],
        "merged-pr-591-closed-issue-505"
    );
    let receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join(".git/csdlc-v3/cutover-receipt.json")).unwrap())
            .unwrap();
    assert_eq!(receipt["phase"], "committed");
    assert!(receipt["prior_selector"].is_array());
    let retry = execute_cutover_request(&request).expect("idempotent retry");
    assert_eq!(retry.status, TerminalRouteStatus::Ready);
}

#[test]
fn approved_cutover_refuses_existing_receipt_before_mutation() {
    let root = fixture_root("cutover_existing_receipt");
    fs::create_dir_all(root.join(".git")).expect("git marker");
    fs::create_dir_all(root.join("build")).expect("build directory");
    fs::write(root.join("build/csdlc"), b"new-v3-binary").expect("selected binary");
    let binary_digest = blake3::hash(b"new-v3-binary").to_hex().to_string();
    let readiness = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.authority_readiness.v1",
        "authority_issue": 505,
        "all_operational": true,
        "v3_only_canary_passed": true,
        "independent_exact_head_review_passed": true,
        "selected_binary_digest": binary_digest,
        "operational_routes": [
            "bind", "clean", "cutover", "doctor", "edit", "eligibility", "finish",
            "github", "github-issue", "github-pr", "install", "issue", "pr-state",
            "proof", "publish", "review", "schedule", "shadow", "shepherd", "soak",
            "validate"
        ]
    }))
    .unwrap();
    fs::write(root.join("build/readiness.json"), &readiness).expect("readiness evidence");
    let readiness_digest = blake3::hash(&readiness).to_hex().to_string();
    fs::create_dir_all(root.join(".adl/bin")).expect("install parent");
    fs::write(root.join(".adl/bin/csdlc"), b"prior-binary").expect("prior binary");
    fs::create_dir_all(root.join(".csdlc")).expect("selector parent");
    fs::write(
        root.join(".csdlc/authority-selector.json"),
        b"prior-selector",
    )
    .expect("prior selector");
    fs::create_dir_all(root.join(".git/csdlc-v3")).expect("receipt parent");
    fs::write(
        root.join(".git/csdlc-v3/cutover-receipt.json"),
        b"prior-receipt",
    )
    .expect("prior receipt");

    let mut request = base_request();
    request.issue = 505;
    request.cutover = Some(CutoverDecisionRequest {
        operator: "operator".into(),
        approval: "#505 operator-reviewed approval".into(),
        selected_binary_provenance: "git:0123456789012345678901234567890123456789".into(),
        rollback_evidence: "typed C-SDLC v2 rollback target verified".into(),
        undo_boundary: "fail-closed before irreversible mutation".into(),
        operation: CutoverOperation::Apply,
        execute: true,
        repository_root: Some(root.clone()),
        selected_binary_path: Some(PathBuf::from("build/csdlc")),
        authority_selector_path: Some(PathBuf::from(".csdlc/authority-selector.json")),
        install_destination_path: Some(PathBuf::from(".adl/bin/csdlc")),
        rollback_receipt_path: Some(PathBuf::from(".csdlc/evidence/505/cutover-receipt.json")),
        readiness_evidence_path: Some(PathBuf::from("build/readiness.json")),
        readiness_evidence_digest: Some(readiness_digest),
    });

    let readiness_digest = write_cutover_fixture(&root, b"new-v3-binary");
    fs::create_dir_all(root.join(".git/csdlc-v3")).expect("durable receipt parent");
    fs::write(
        root.join(".git/csdlc-v3/cutover-receipt.json"),
        b"prior-receipt",
    )
    .expect("restore invalid receipt");
    let request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);

    let blocked = execute_cutover_request(&request).expect("cutover route");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "cutover_receipt_invalid"));
    assert_eq!(
        fs::read(root.join(".adl/bin/csdlc")).expect("prior binary retained"),
        b"prior-binary"
    );
    assert_eq!(
        fs::read(root.join(".csdlc/authority-selector.json")).expect("prior selector retained"),
        b"prior-selector"
    );
    assert_eq!(
        fs::read(root.join(".git/csdlc-v3/cutover-receipt.json")).expect("prior receipt retained"),
        b"prior-receipt"
    );
}

#[test]
fn cutover_rejects_intermediate_output_parent_symlink_escape() {
    let root = fixture_root("cutover_output_parent_escape");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let outside = root.parent().unwrap().join("outside-cutover-output");
    fs::create_dir_all(outside.join("bin")).expect("outside output parent");
    if root.join(".adl").exists() {
        fs::remove_dir_all(root.join(".adl")).expect("replace tracked policy with escape symlink");
    }
    create_symlink(&outside, &root.join(".adl"));
    let request = cutover_request(&root, readiness_digest, CutoverOperation::Apply);

    let blocked = execute_cutover_request(&request).expect("cutover plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "cutover_path_escapes_repository"));
    assert!(!outside.join("bin/csdlc").exists());
    let selector: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join("csdlc-v3/operator/authority-selector.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(selector["generation"], "v3");
}

fn cleanup_plan(
    approved_parent: &Path,
    repository_root: &Path,
    candidate_path: &Path,
    remove: bool,
    terminal_receipt: Option<(String, String)>,
    preview_receipt_digest: Option<String>,
) -> TerminalRouteRequest {
    let mut request = base_request();
    let (terminal_receipt_path, terminal_receipt_digest) =
        terminal_receipt.unwrap_or_else(|| ("".into(), "".into()));
    if !terminal_receipt_path.is_empty() {
        let receipt_bytes =
            fs::read(repository_root.join(&terminal_receipt_path)).expect("read terminal receipt");
        let receipt: DurableTerminalReceipt =
            serde_json::from_slice(&receipt_bytes).expect("parse terminal receipt");
        request.expected_head_sha = Some(receipt.head_sha);
    }
    request.cleanup = Some(CleanupRouteRequest {
        approved_parent: approved_parent.to_path_buf(),
        repository_root: repository_root.to_path_buf(),
        candidate_path: candidate_path.to_path_buf(),
        remove,
        terminal_receipt: true,
        terminal_receipt_path: (!terminal_receipt_path.is_empty()).then_some(terminal_receipt_path),
        terminal_receipt_digest: (!terminal_receipt_digest.is_empty())
            .then_some(terminal_receipt_digest),
        preview_receipt_digest,
    });
    request
}

#[test]
fn cleanup_rejects_caller_boolean_without_durable_terminal_receipt() {
    let fixture = fixture_root("cleanup_missing_terminal_receipt");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);

    let plan = cleanup_plan(&fixture, &primary, &registered, false, None, None);
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "missing_terminal_receipt"));
}

#[test]
fn cleanup_rejects_disposable_terminal_receipt_path() {
    let fixture = fixture_root("cleanup_disposable_terminal_receipt");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);
    let receipt = write_terminal_receipt_at(
        &primary,
        630,
        641,
        "0123456789012345678901234567890123456789",
        "target/disposable-terminal-receipt.json",
    );

    let plan = cleanup_plan(&fixture, &primary, &registered, false, Some(receipt), None);
    let blocked = prepare_terminal_route("clean", &plan).expect("clean plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(blocked
        .findings
        .iter()
        .any(|finding| finding.code == "receipt_path_not_durable"));
}

#[test]
fn clean_cli_reports_requested_but_unperformed_mutation_before_cutover() {
    let fixture = fixture_root("cleanup_cli_read_only_report");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);

    let request = cleanup_plan(&fixture, &primary, &registered, true, None, None);
    let request_path = fixture.join("clean-request.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request).expect("serialize clean request"),
    )
    .expect("write clean request");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .args(["clean", "--request"])
        .arg(&request_path)
        .output()
        .expect("run clean command");
    assert!(
        !output.status.success(),
        "blocked clean must return nonzero"
    );
    let stderr = String::from_utf8_lossy(&output.stderr);
    let value: serde_json::Value =
        serde_json::from_str(stderr.strip_prefix("csdlc: ").unwrap_or(&stderr).trim())
            .expect("machine-readable clean JSON on stderr");
    assert_eq!(value["read_only"], true);
    assert_eq!(value["requested_mutation"], true);
    assert_eq!(value["performed_mutation"], false);
    assert_eq!(value["result"]["status"], "blocked");
    assert!(registered.exists());
}

#[test]
fn cleanup_rejects_stale_or_mismatched_terminal_receipts() {
    let fixture = fixture_root("cleanup_stale_terminal_receipt");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);

    let mut stale_digest = write_terminal_receipt(
        &primary,
        630,
        641,
        "0123456789012345678901234567890123456789",
    );
    stale_digest.1 = "wrong".into();
    let digest_plan = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(stale_digest),
        None,
    );
    let digest_blocked = prepare_terminal_route("clean", &digest_plan).expect("clean plan");
    assert_eq!(digest_blocked.status, TerminalRouteStatus::Blocked);
    assert!(digest_blocked
        .findings
        .iter()
        .any(|finding| finding.code == "terminal_receipt_digest_mismatch"));

    let mismatched = write_terminal_receipt(
        &primary,
        999,
        641,
        "0123456789012345678901234567890123456789",
    );
    let mismatch_plan = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(mismatched),
        None,
    );
    let mismatch_blocked = prepare_terminal_route("clean", &mismatch_plan).expect("clean plan");
    assert_eq!(mismatch_blocked.status, TerminalRouteStatus::Blocked);
    assert!(mismatch_blocked
        .findings
        .iter()
        .any(|finding| finding.code == "terminal_receipt_mismatch"));
}

fn write_terminal_receipt(
    repository_root: &Path,
    issue: u64,
    pull_request: u64,
    head_sha: &str,
) -> (String, String) {
    write_terminal_receipt_at(
        repository_root,
        issue,
        pull_request,
        head_sha,
        ".csdlc/evidence/630/terminal-receipt.json",
    )
}

fn write_terminal_receipt_at(
    repository_root: &Path,
    issue: u64,
    pull_request: u64,
    head_sha: &str,
    relative_path: &str,
) -> (String, String) {
    let receipt = DurableTerminalReceipt {
        schema: "csdlc.v3.terminal_receipt.v1".into(),
        repository: "agent-logic/agent-design-language".into(),
        issue,
        pull_request,
        head_sha: head_sha.into(),
        disposition: "closed_out".into(),
        state_digest: None,
    };
    let path = repository_root.join(relative_path);
    fs::create_dir_all(path.parent().expect("receipt parent")).expect("receipt directory");
    let bytes = serde_json::to_vec(&receipt).expect("serialize receipt");
    fs::write(&path, &bytes).expect("write receipt");
    let digest = blake3::hash(&bytes).to_hex().to_string();
    (relative_path.into(), digest)
}

#[test]
fn post_cutover_finish_persists_typed_state_and_receipt_idempotently() {
    let root = fixture_root("post_cutover_finish");
    init_repo(&root);
    write_generation_selector(&root, "v3");
    let head = git_stdout(&root, &["rev-parse", "HEAD"]);
    let mut request = base_request();
    request.expected_head_sha = Some(head.clone());
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    request.terminal_state = Some(TerminalStateWriteRequest {
        repository_root: root.clone(),
        state_path: PathBuf::from(".csdlc/v3/issues/630/terminal.json"),
        receipt_path: PathBuf::from(".csdlc/evidence/630/terminal-receipt.json"),
        expected_state_digest: None,
    });
    for _ in 0..2 {
        let mut adapter = FakeGithubAdapter::new([
            github_pr_json(641, &head, true, "Closes #630"),
            github_issue_json(630, "closed"),
        ]);
        let plan = prepare_terminal_finish_with_github_observation(&request, &mut adapter)
            .expect("post-cutover finish");
        assert_eq!(plan.status, TerminalRouteStatus::Ready);
        assert!(plan.operational_authority);
    }
    let state: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join(".csdlc/v3/issues/630/terminal.json")).unwrap())
            .unwrap();
    assert_eq!(state["schema"], "csdlc.v3.terminal_state.v1");
    assert_eq!(state["disposition"], "closed_out");
    let receipt: DurableTerminalReceipt = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/630/terminal-receipt.json")).unwrap(),
    )
    .unwrap();
    assert!(receipt.state_digest.is_some());
}

#[test]
fn pre_cutover_finish_denies_terminal_persistence() {
    let root = fixture_root("pre_cutover_finish");
    init_repo(&root);
    write_generation_selector(&root, "v2");
    let mut request = base_request();
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    request.terminal_state = Some(TerminalStateWriteRequest {
        repository_root: root,
        state_path: PathBuf::from(".csdlc/v3/issues/630/terminal.json"),
        receipt_path: PathBuf::from(".csdlc/evidence/630/terminal-receipt.json"),
        expected_state_digest: None,
    });
    let mut adapter = FakeGithubAdapter::new([
        github_pr_json(
            641,
            "0123456789012345678901234567890123456789",
            true,
            "Closes #630",
        ),
        github_issue_json(630, "closed"),
    ]);
    let plan = prepare_terminal_finish_with_github_observation(&request, &mut adapter).unwrap();
    assert_eq!(plan.status, TerminalRouteStatus::Blocked);
    assert!(!plan.operational_authority);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "terminal_persistence_denied_pre_cutover"));
}

#[test]
fn post_cutover_cleanup_removes_exact_clean_registered_terminal_worktree() {
    let fixture = fixture_root("cleanup_post_cutover_remove");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    write_generation_selector(&primary, "v3");
    let head = git_stdout(&primary, &["rev-parse", "HEAD"]);
    let receipt = write_terminal_receipt(&primary, 630, 641, &head);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);
    let preview = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        false,
        Some(receipt.clone()),
        None,
    );
    let preview_digest = match prepare_terminal_route("clean", &preview).unwrap().cleanup {
        Some(CleanupDecision::Removable { receipt_digest, .. }) => receipt_digest,
        other => panic!("expected removable preview, got {other:?}"),
    };
    let remove = cleanup_plan(
        &fixture,
        &primary,
        &registered,
        true,
        Some(receipt),
        Some(preview_digest.clone()),
    );
    let removed = prepare_terminal_route("clean", &remove).expect("post-cutover remove");
    assert_eq!(removed.status, TerminalRouteStatus::Ready);
    assert!(matches!(
        removed.cleanup,
        Some(CleanupDecision::Removed { receipt_digest, .. }) if receipt_digest == preview_digest
    ));
    assert!(!registered.exists());
}

#[test]
fn cutover_recovers_interrupted_boundaries_and_rollback_is_idempotent() {
    let root = fixture_root("cutover_interrupted_recovery");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let apply = cutover_request(&root, readiness_digest.clone(), CutoverOperation::Apply);
    execute_cutover_request(&apply).expect("initial apply");
    let receipt_path = root.join(".git/csdlc-v3/cutover-receipt.json");
    let selector_path = root.join("csdlc-v3/operator/authority-selector.json");
    let mut receipt: serde_json::Value =
        serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    let prior_selector = receipt["prior_selector"]
        .as_array()
        .unwrap()
        .iter()
        .map(|value| value.as_u64().unwrap() as u8)
        .collect::<Vec<_>>();

    receipt["phase"] = "prepared".into();
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    fs::write(&selector_path, &prior_selector).unwrap();
    fs::remove_file(root.join(".adl/bin/csdlc")).unwrap();
    execute_cutover_request(&apply).expect("recover after journal");

    receipt = serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["phase"] = "binary_installed".into();
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    fs::write(&selector_path, &prior_selector).unwrap();
    execute_cutover_request(&apply).expect("recover after binary");

    receipt = serde_json::from_slice(&fs::read(&receipt_path).unwrap()).unwrap();
    receipt["phase"] = "binary_installed".into();
    fs::write(&receipt_path, serde_json::to_vec_pretty(&receipt).unwrap()).unwrap();
    execute_cutover_request(&apply).expect("recover after selector");

    fs::write(&selector_path, &prior_selector).unwrap();
    publish_v2_rollback(&root);
    let rollback = cutover_request(&root, readiness_digest, CutoverOperation::Rollback);
    let rolled_back = prepare_terminal_route("cutover", &rollback).expect("interrupted rollback");
    assert_eq!(rolled_back.status, TerminalRouteStatus::Ready);
    let rolled_back_selector: serde_json::Value =
        serde_json::from_slice(&fs::read(&selector_path).unwrap()).unwrap();
    assert_eq!(rolled_back_selector["generation"], "rollback");
    assert!(!root.join(".adl/bin/csdlc").exists());
    let retry = prepare_terminal_route("cutover", &rollback).expect("idempotent rollback");
    assert_eq!(retry.status, TerminalRouteStatus::Ready, "{retry:#?}");
}

#[test]
fn rollback_fails_closed_on_stale_selector_digest() {
    let root = fixture_root("rollback_stale_selector");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let apply = cutover_request(&root, readiness_digest.clone(), CutoverOperation::Apply);
    execute_cutover_request(&apply).expect("apply");
    write_generation_selector(&root, "v2");
    let rollback = cutover_request(&root, readiness_digest, CutoverOperation::Rollback);
    let blocked = prepare_terminal_route("cutover", &rollback).expect("rollback plan");
    assert_eq!(blocked.status, TerminalRouteStatus::Blocked);
    assert!(
        blocked
            .findings
            .iter()
            .any(|finding| finding.code == "rollback_requires_selector_revert"),
        "{blocked:#?}"
    );
    assert!(root.join(".adl/bin/csdlc").exists());
}

#[test]
fn tracked_selector_revert_rolls_back_from_fresh_worktree() {
    let root = fixture_root("rollback_fresh_worktree");
    let readiness_digest = write_cutover_fixture(&root, b"v3-binary");
    let apply = cutover_request(&root, readiness_digest.clone(), CutoverOperation::Apply);
    execute_cutover_request(&apply).expect("apply cutover");
    publish_v2_rollback(&root);

    let fresh = root.parent().unwrap().join(format!(
        "{}-fresh",
        root.file_name().unwrap().to_string_lossy()
    ));
    git(&root, &["worktree", "add", fresh.to_str().unwrap()]);
    fs::create_dir_all(fresh.join(".adl/bin")).unwrap();
    fs::copy(root.join(".adl/bin/csdlc"), fresh.join(".adl/bin/csdlc")).unwrap();

    let mut rollback = cutover_request(&root, readiness_digest, CutoverOperation::Rollback);
    rollback.cutover.as_mut().unwrap().repository_root = Some(fresh.clone());
    let plan = prepare_terminal_route("cutover", &rollback).expect("fresh worktree rollback");
    assert_eq!(plan.status, TerminalRouteStatus::Ready, "{plan:#?}");
    assert!(!fresh.join(".adl/bin/csdlc").exists());
    assert!(root.join(".git/csdlc-v3/cutover-receipt.json").exists());
}

fn native_review_fixture(root: &Path) -> (Vec<u8>, String, String) {
    let revision = git_output(root, &["rev-parse", "HEAD"]);
    let reviewed_revision = format!(
        "git-blake3:{revision}:{}",
        blake3::hash(revision.as_bytes()).to_hex()
    );
    let reviewer = "fresh-session:11111111-2222-4333-8444-555555555555";
    let mut record = serde_json::json!({
        "schema": "csdlc.v3.native_review_receipt.v1",
        "issue": 505,
        "repository": "agent-logic/agent-design-language",
        "phase": "reviewed",
        "generation": 1,
        "digest": "",
        "review_assignment": {
            "reviewer": reviewer,
            "assigned_by": "operator",
            "revision": reviewed_revision,
            "scope": ["src"]
        },
        "review": {
            "reviewer": reviewer,
            "scope": ["src"],
            "reviewed_revision": reviewed_revision,
            "findings": [],
            "completed": true
        }
    });
    let digest = blake3::hash(&serde_json::to_vec(&record).unwrap())
        .to_hex()
        .to_string();
    record["digest"] = serde_json::Value::String(digest.clone());
    (serde_json::to_vec(&record).unwrap(), digest, revision)
}

fn git_output(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn write_cutover_fixture(root: &Path, _binary_marker: &[u8]) -> String {
    if root.join(".git").exists() {
        fs::remove_dir_all(root.join(".git")).expect("replace git marker");
    }
    fs::create_dir_all(root).expect("fixture root");
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    fs::create_dir_all(root.join("fixture-worktrees")).expect("fixture worktree parent");
    fs::create_dir_all(root.join(".adl")).expect("fixture adl parent");
    fs::write(
        root.join(".adl/worktree-policy.json"),
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.worktree_policy.v1",
            "required_parent": root.join("fixture-worktrees")
        }))
        .unwrap(),
    )
    .expect("fixture worktree policy");
    fs::write(root.join("fixture-base"), "native v3 authority\n").unwrap();
    git(root, &["add", "fixture-base", ".adl/worktree-policy.json"]);
    git(root, &["commit", "-m", "authority fixture"]);
    fs::create_dir_all(root.join("build")).expect("build directory");
    fs::copy(env!("CARGO_BIN_EXE_csdlc"), root.join("build/csdlc"))
        .expect("selected real v3 binary");
    #[cfg(unix)]
    {
        let mut permissions = fs::metadata(root.join("build/csdlc"))
            .expect("selected binary metadata")
            .permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(root.join("build/csdlc"), permissions)
            .expect("selected binary executable");
    }
    let selected_binary = fs::read(root.join("build/csdlc")).expect("selected binary bytes");
    let selected_binary_digest = blake3::hash(&selected_binary).to_hex().to_string();
    write_generation_selector(root, "v2");
    let (review, review_record_digest, revision) = native_review_fixture(root);
    fs::create_dir_all(root.join(".csdlc/evidence/505")).expect("native review parent");
    fs::write(
        root.join(".csdlc/evidence/505/native-authority-review.json"),
        &review,
    )
    .expect("native review record");
    fs::create_dir_all(root.join(".csdlc/issues/505")).expect("review state parent");
    fs::write(root.join(".csdlc/issues/505/index.json"), &review).expect("native review state");
    let review_ref = serde_json::json!({
        "path": ".csdlc/evidence/505/native-authority-review.json",
        "digest": blake3::hash(&review).to_hex().to_string(),
        "revision": revision
    });
    let canary_ref =
        write_proof_command_fixture(root, "v3-only-canary", "v3-only-canary", &revision);
    let rollback_ref =
        write_proof_command_fixture(root, "rollback-readiness", "rollback-readiness", &revision);
    let rollback_digest = rollback_ref["digest"].as_str().unwrap().to_owned();
    let approval = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.cutover_approval.v1",
        "authority_issue": 505,
        "repository": "agent-logic/agent-design-language",
        "decision": "approved",
        "exact_head": revision,
        "selected_binary_digest": selected_binary_digest,
        "selector_metadata_digest": "pre-cutover-selector",
        "rollback_evidence_digest": rollback_digest,
        "review_record_digest": review_record_digest,
        "reviewer_github_login": "reviewer",
        "review_comment_id": 1001,
        "approval_comment_id": 1002,
        "approved_by": "operator"
    }))
    .unwrap();
    fs::create_dir_all(root.join(".csdlc/evidence/505")).expect("approval parent");
    fs::write(
        root.join(".csdlc/evidence/505/cutover-approval.json"),
        &approval,
    )
    .expect("approval evidence");
    let readiness = serde_json::to_vec(&serde_json::json!({
        "schema": "csdlc.v3.authority_readiness.v1",
        "authority_issue": 505,
        "selected_binary_digest": selected_binary_digest,
        "selected_revision": revision,
        "canary_proof": canary_ref,
        "review_proof": review_ref,
        "rollback_proof": rollback_ref
    }))
    .unwrap();
    fs::write(
        root.join(".csdlc/evidence/505/authority-readiness.json"),
        &readiness,
    )
    .expect("readiness evidence");
    write_generation_selector(root, "v3");
    blake3::hash(&readiness).to_hex().to_string()
}

fn write_proof_command_fixture(
    root: &Path,
    manifest_id: &str,
    lane: &str,
    revision: &str,
) -> serde_json::Value {
    let source_ref = format!(".csdlc/evidence/505/readiness-source/{manifest_id}.json");
    let source = serde_json::to_vec(&serde_json::json!({
        "manifest_id": manifest_id,
        "revision": revision,
        "result": "pass"
    }))
    .unwrap();
    fs::create_dir_all(root.join(".csdlc/evidence/505/readiness-source"))
        .expect("proof source parent");
    fs::write(root.join(&source_ref), &source).expect("proof source");
    let digest = blake3::hash(&source).to_hex().to_string();
    let registry_ref = ".csdlc/evidence/505/readiness-source/doctor-current-registry.json";
    fs::write(
        root.join(registry_ref),
        serde_json::to_vec(&serde_json::json!({
            "schema": "adl.csdlc.prompt_template_registry.v1",
            "csdlc_prompt_template_set": "1.0.4",
            "semver": "1.0.4",
            "status": "active",
            "templates": {
                "sip": {"path": "fixture/templates/sip.md"},
                "stp": {"path": "fixture/templates/stp.md"},
                "spp": {"path": "fixture/templates/spp.md"},
                "vpp": {"path": "fixture/templates/vpp.md"},
                "srp": {"path": "fixture/templates/srp.md"},
                "sor": {"path": "fixture/templates/sor.md"}
            }
        }))
        .unwrap(),
    )
    .expect("fixture prompt registry");
    let request_ref =
        format!(".csdlc/evidence/505/readiness-source/{manifest_id}-doctor-request.json");
    fs::write(
        root.join(&request_ref),
        serde_json::to_vec(&serde_json::json!({
            "issue": 505,
            "title": "C-SDLC v3 authority transition",
            "repository": "agent-logic/agent-design-language",
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": "fixture-worktrees/issue-505",
            "registry_version": "1.0.4",
            "commands": ["prepare_issue", "bind_worktree", "edit_cards", "plan_pvf", "doctor", "schedule", "shepherd", "eligibility"],
            "card_updates": {}
        }))
        .unwrap(),
    )
    .expect("proof command request");
    let registrations_ref = ".csdlc/evidence/505/readiness-source/doctor-registrations.json";
    fs::write(
        root.join(registrations_ref),
        serde_json::to_vec(&serde_json::json!([{
            "branch": "codex/505-v3-f-authority-transition-decision-exec",
            "worktree": "fixture-worktrees/issue-505",
            "primary": false
        }]))
        .unwrap(),
    )
    .expect("proof command registrations");
    let normalization = ShadowNormalizationContract::DoctorIssuePhaseV1;
    let argv = vec![
        "doctor".into(),
        "--request".into(),
        request_ref.clone(),
        "--registry".into(),
        registry_ref.into(),
        "--registrations".into(),
        registrations_ref.into(),
        "--repo-root".into(),
        ".".into(),
    ];
    let command = ShadowCommandSpec {
        generation: ShadowGeneration::V3,
        binary_ref: "build/csdlc".into(),
        argv,
        request_ref,
        timeout_millis: 10_000,
        side_effect_boundary_refs: vec![source_ref.clone()],
        provider_side_effects: false,
    };
    let report = classify_route(
        "proof",
        ProofRouteRequest {
            issue: 505,
            repository: "agent-logic/agent-design-language".into(),
            cutover_issue: Some(505),
            operator_approval: None,
            evidence_root: Some(root.to_string_lossy().into_owned()),
            proof: Some(ProofManifest {
                manifest_id: manifest_id.into(),
                lane: lane.into(),
                deterministic: true,
                evidence_ref: source_ref,
                evidence_digest: digest.clone(),
                observed_digest: digest,
                stale: false,
                normalization,
                command,
            }),
            shadow: None,
            soak: None,
            install: None,
        },
        Some(root),
    );
    assert_eq!(report.status, ProofRouteStatus::Ready, "{report:#?}");
    let path = report.evidence_refs.first().expect("proof receipt path");
    let bytes = fs::read(root.join(path)).expect("proof receipt");
    serde_json::json!({
        "path": path,
        "digest": blake3::hash(&bytes).to_hex().to_string(),
        "revision": revision
    })
}

fn cutover_request(
    root: &Path,
    readiness_digest: String,
    operation: CutoverOperation,
) -> TerminalRouteRequest {
    let readiness: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/505/authority-readiness.json")).unwrap(),
    )
    .unwrap();
    let revision = readiness["selected_revision"].as_str().unwrap();
    let mut request = base_request();
    request.issue = 505;
    request.cutover = Some(CutoverDecisionRequest {
        operator: "operator".into(),
        approval: ".csdlc/evidence/505/cutover-approval.json".into(),
        selected_binary_provenance: format!("git:{revision}"),
        rollback_evidence: ".csdlc/evidence/505/v3-proof/rollback-readiness.json".into(),
        undo_boundary: "fail-closed before irreversible mutation".into(),
        operation,
        execute: true,
        repository_root: Some(root.to_path_buf()),
        selected_binary_path: Some(PathBuf::from("build/csdlc")),
        authority_selector_path: Some(PathBuf::from("csdlc-v3/operator/authority-selector.json")),
        install_destination_path: Some(PathBuf::from(".adl/bin/csdlc")),
        rollback_receipt_path: Some(PathBuf::from(".csdlc/evidence/505/cutover-receipt.json")),
        readiness_evidence_path: Some(PathBuf::from(
            ".csdlc/evidence/505/authority-readiness.json",
        )),
        readiness_evidence_digest: Some(readiness_digest),
    });
    request
}

fn execute_cutover_request(
    request: &TerminalRouteRequest,
) -> Result<
    csdlc_v3::commands::terminal::TerminalRoutePlan,
    csdlc_v3::commands::terminal::TerminalFinding,
> {
    let mut authenticated = request.clone();
    authenticated.pull_request = Some(591);
    authenticated.credential_names = vec!["GITHUB_TOKEN".into()];
    let revision = request
        .cutover
        .as_ref()
        .unwrap()
        .selected_binary_provenance
        .trim_start_matches("git:");
    let mut adapter = FakeGithubAdapter::new([
        serde_json::json!({"number": 591, "merged": true, "head": {"sha": revision}}).to_string(),
        serde_json::json!({"number": 505, "state": "closed"}).to_string(),
    ]);
    prepare_terminal_cutover_with_github_observation(&authenticated, &mut adapter)
}

fn write_generation_selector(repository_root: &Path, generation: &str) {
    let path = repository_root.join("csdlc-v3/operator/authority-selector.json");
    fs::create_dir_all(path.parent().unwrap()).expect("selector parent");
    let selector = if generation == "v3" {
        let reviewed_head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        git(
            repository_root,
            &["commit", "--allow-empty", "-m", "V3-F cutover (#591)"],
        );
        let merge_commit = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        let terminal = serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.v3.terminal_receipt.v1", "repository":"agent-logic/agent-design-language",
            "issue":505, "pull_request":591, "head_sha":reviewed_head,
            "disposition":"closed_out", "state_digest":"fixture"
        })).unwrap();
        fs::create_dir_all(repository_root.join(".csdlc/evidence/505")).unwrap();
        fs::write(
            repository_root.join(".csdlc/evidence/505/terminal-receipt.json"),
            &terminal,
        )
        .unwrap();
        let observation = serde_json::to_vec_pretty(&serde_json::json!({
            "schema":"csdlc.github_pr_state.v1", "repository":"agent-logic/agent-design-language",
            "pull_request":591, "base_ref":"main", "head_sha":reviewed_head,
            "merge_commit_sha":merge_commit, "state":"closed", "merged":true,
            "linked_issue":505, "linkage_source":"github_closing_issues_references",
            "observation_authority":"typed-csdlc-github-pr-authenticated-readback"
        }))
        .unwrap();
        fs::write(
            repository_root.join("csdlc-v3/operator/native-authority-pr-observation.json"),
            &observation,
        )
        .unwrap();
        let receipt = serde_json::to_vec_pretty(&serde_json::json!({
            "schema": "csdlc.v3.native_authority_receipt.v1", "authority_issue": 505,
            "authority_pull_request": 591, "reviewed_head": reviewed_head,
            "merge_commit": merge_commit, "pr_observation_path":"csdlc-v3/operator/native-authority-pr-observation.json",
            "pr_observation_digest":blake3::hash(&observation).to_hex().to_string(), "terminal_receipt_path": ".csdlc/evidence/505/terminal-receipt.json",
            "terminal_receipt_digest": blake3::hash(&terminal).to_hex().to_string(), "source_selector_schema": "csdlc.generation_selector.v2",
            "source_selector_digest": format!("sha256:{}", "3".repeat(64)),
            "operational_authority": "csdlc-v3", "review_authority": "typed-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "remote_reconciliation": "canonical-terminal-receipt-and-git-objects"
        })).unwrap();
        fs::write(
            repository_root.join("csdlc-v3/operator/native-authority-receipt.json"),
            &receipt,
        )
        .unwrap();
        serde_json::json!({
            "schema": "csdlc.v3.authority_selector.v1",
            "generation": "v3",
            "operational_authority": "csdlc-v3",
            "authority_issue": 505,
            "authority_pull_request": 591,
            "review_authority": "typed-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "receipt_path": "csdlc-v3/operator/native-authority-receipt.json",
            "receipt_digest": blake3::hash(&receipt).to_hex().to_string()
        })
    } else {
        serde_json::json!({
            "schema": "csdlc.v3.authority_selector.v1",
            "generation": "rollback",
            "operational_authority": "suspended",
            "authority_issue": 505,
            "authority_pull_request": 591
        })
    };
    fs::write(path, serde_json::to_vec_pretty(&selector).unwrap()).expect("generation selector");
    if generation == "v3" {
        git(
            repository_root,
            &[
                "add",
                "csdlc-v3/operator/authority-selector.json",
                "csdlc-v3/operator/native-authority-receipt.json",
                "csdlc-v3/operator/native-authority-pr-observation.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        git(
            repository_root,
            &["commit", "-m", "activate fixture selector"],
        );
        let head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
        git(
            repository_root,
            &["update-ref", "refs/remotes/origin/main", &head],
        );
    }
}

fn publish_v2_rollback(repository_root: &Path) {
    write_generation_selector(repository_root, "v2");
    git(
        repository_root,
        &["add", "csdlc-v3/operator/authority-selector.json"],
    );
    git(
        repository_root,
        &["commit", "-m", "revert tracked selector to v2"],
    );
    let head = git_stdout(repository_root, &["rev-parse", "HEAD"]);
    git(
        repository_root,
        &["update-ref", "refs/remotes/origin/main", &head],
    );
}

struct FakeGithubAdapter {
    outputs: Vec<String>,
    invocations: Vec<Vec<String>>,
}

impl FakeGithubAdapter {
    fn new(outputs: impl IntoIterator<Item = String>) -> Self {
        let mut outputs = outputs.into_iter().collect::<Vec<_>>();
        outputs.reverse();
        Self {
            outputs,
            invocations: Vec::new(),
        }
    }
}

impl ProcessAdapter for FakeGithubAdapter {
    fn run(&mut self, invocation: CommandInvocation) -> ProcessOutput {
        self.invocations.push(invocation.argv().to_vec());
        ProcessOutput {
            status: ProcessStatus::Exit(0),
            stdout: self.outputs.pop().expect("fake GitHub output"),
            stderr: String::new(),
            truncated: false,
        }
    }
}

fn github_pr_json(number: u64, head_sha: &str, merged: bool, body: &str) -> String {
    serde_json::json!({
        "number": number,
        "head": { "sha": head_sha },
        "merged": merged,
        "body": body
    })
    .to_string()
}

fn github_issue_json(number: u64, state: &str) -> String {
    serde_json::json!({
        "number": number,
        "state": state
    })
    .to_string()
}

fn fixture_root(name: &str) -> PathBuf {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join("issue-630-tests")
        .join(format!("{}-{}", name, std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root).expect("clean stale fixture");
    }
    root
}

fn init_repo(path: &Path) {
    fs::create_dir_all(path).expect("repo dir");
    git(path, &["init", "-b", "main"]);
    git(path, &["config", "user.email", "test@example.com"]);
    git(path, &["config", "user.name", "C-SDLC Test"]);
    fs::write(path.join("README.md"), "fixture\n").expect("readme");
    git(path, &["add", "README.md"]);
    git(path, &["commit", "-m", "fixture"]);
}

fn git(path: &Path, args: &[&str]) {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:{}\nstderr:{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(path: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(path)
        .args(args)
        .output()
        .expect("run git");
    assert!(
        output.status.success(),
        "git {:?} failed\nstdout:{}\nstderr:{}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8_lossy(&output.stdout).trim().to_owned()
}

#[cfg(unix)]
fn create_symlink(target: &Path, link: &Path) {
    std::os::unix::fs::symlink(target, link).expect("symlink");
}

#[cfg(windows)]
fn create_symlink(target: &Path, link: &Path) {
    std::os::windows::fs::symlink_dir(target, link).expect("symlink");
}
