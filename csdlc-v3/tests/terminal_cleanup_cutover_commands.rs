use csdlc_v3::{
    adapters::{CommandInvocation, ProcessAdapter, ProcessOutput, ProcessStatus},
    commands::terminal::{
        prepare_terminal_finish_with_github_observation, prepare_terminal_route, CleanupDecision,
        CleanupRouteRequest, CutoverDecisionRequest, DurableTerminalReceipt, FinishDecision,
        TerminalPublicationMode, TerminalRouteRequest, TerminalRouteStatus,
    },
};
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn base_request() -> TerminalRouteRequest {
    TerminalRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 630,
        pull_request: Some(641),
        expected_head_sha: Some("0123456789012345678901234567890123456789".into()),
        mode: Some(TerminalPublicationMode::Closing),
        public_adapter_receipt: None,
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

    assert_eq!(plan.status, TerminalRouteStatus::Ready);
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
        execute: false,
        repository_root: None,
        selected_binary_path: None,
        authority_selector_path: None,
        install_destination_path: None,
        rollback_receipt_path: None,
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
        execute: false,
        repository_root: None,
        selected_binary_path: None,
        authority_selector_path: None,
        install_destination_path: None,
        rollback_receipt_path: None,
    });
    let ready = prepare_terminal_route("cutover", &request).expect("cutover plan");
    assert_eq!(ready.status, TerminalRouteStatus::Ready);
    assert!(!ready.cutover.unwrap().executes_cutover);
}

#[test]
fn approved_cutover_atomically_installs_selector_and_rollback_receipt() {
    let root = fixture_root("approved_cutover_execution");
    fs::create_dir_all(root.join(".git")).expect("git marker");
    fs::create_dir_all(root.join("build")).expect("build directory");
    fs::write(root.join("build/csdlc"), b"v3-binary").expect("selected binary");

    let mut request = base_request();
    request.issue = 505;
    request.cutover = Some(CutoverDecisionRequest {
        operator: "operator".into(),
        approval: "#505 operator-reviewed approval".into(),
        selected_binary_provenance: "git:0123456789012345678901234567890123456789".into(),
        rollback_evidence: "typed C-SDLC v2 rollback target verified".into(),
        undo_boundary: "fail-closed before irreversible mutation".into(),
        execute: true,
        repository_root: Some(root.clone()),
        selected_binary_path: Some(PathBuf::from("build/csdlc")),
        authority_selector_path: Some(PathBuf::from(".csdlc/authority-selector.json")),
        install_destination_path: Some(PathBuf::from(".adl/bin/csdlc")),
        rollback_receipt_path: Some(PathBuf::from(".csdlc/evidence/505/cutover-receipt.json")),
    });

    let plan = prepare_terminal_route("cutover", &request).expect("cutover route");
    assert_eq!(plan.status, TerminalRouteStatus::Ready);
    assert!(plan.operational_authority);
    assert!(plan.cutover.expect("cutover decision").executes_cutover);
    assert_eq!(fs::read(root.join(".adl/bin/csdlc")).unwrap(), b"v3-binary");
    let selector: serde_json::Value =
        serde_json::from_slice(&fs::read(root.join(".csdlc/authority-selector.json")).unwrap())
            .unwrap();
    assert_eq!(selector["default_generation"], "v3");
    assert_eq!(selector["rollback_generation"], "v2");
    let receipt: serde_json::Value = serde_json::from_slice(
        &fs::read(root.join(".csdlc/evidence/505/cutover-receipt.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(receipt["performed_mutation"], true);
    assert_eq!(receipt["rollback_generation"], "v2");
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
        output.status.success(),
        "clean command failed\nstdout:{}\nstderr:{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let value: serde_json::Value =
        serde_json::from_slice(&output.stdout).expect("machine-readable clean JSON");
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
    };
    let path = repository_root.join(relative_path);
    fs::create_dir_all(path.parent().expect("receipt parent")).expect("receipt directory");
    let bytes = serde_json::to_vec(&receipt).expect("serialize receipt");
    fs::write(&path, &bytes).expect("write receipt");
    let digest = blake3::hash(&bytes).to_hex().to_string();
    (relative_path.into(), digest)
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
