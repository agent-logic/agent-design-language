use csdlc_v3::commands::terminal::{
    derive_finish_from_verified, prepare_terminal_route, CleanupDecision, CleanupRouteRequest,
    CutoverDecisionRequest, FinishDecision, TerminalPublicationMode, TerminalRouteRequest,
    TerminalRouteStatus, VerifiedTerminalReadback,
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
fn sealed_typed_readback_can_derive_terminal_closeout() {
    let request = base_request();
    let readback = VerifiedTerminalReadback::from_typed_adapter_receipt(
        "csdlc-github-pr-state",
        request.repository.clone(),
        request.issue,
        request.pull_request.unwrap(),
        request.expected_head_sha.clone().unwrap(),
        TerminalPublicationMode::Closing,
        true,
        false,
        Some(request.issue),
        None,
    )
    .expect("typed receipt");
    let decision = derive_finish_from_verified(&request, readback).expect("finish decision");
    assert_eq!(
        decision,
        FinishDecision::TerminalClosedOut {
            pull_request: 641,
            issue: 630,
            head_sha: "0123456789012345678901234567890123456789".into()
        }
    );
}

#[test]
fn part_of_publication_cannot_terminally_close() {
    let mut request = base_request();
    request.mode = Some(TerminalPublicationMode::PartOf);
    let readback = VerifiedTerminalReadback::from_typed_adapter_receipt(
        "csdlc-github-pr-state",
        request.repository.clone(),
        request.issue,
        request.pull_request.unwrap(),
        request.expected_head_sha.clone().unwrap(),
        TerminalPublicationMode::PartOf,
        true,
        true,
        None,
        Some(request.issue),
    )
    .expect("typed receipt");
    let decision = derive_finish_from_verified(&request, readback).expect("finish decision");
    assert_eq!(
        decision,
        FinishDecision::CheckpointDenied {
            reason: "part_of publications cannot close a terminal issue".into()
        }
    );
}

#[test]
fn cleanup_uses_git_registration_and_preserves_distinct_outcomes() {
    let fixture = fixture_root("cleanup_distinct");
    let primary = fixture.join("primary");
    let registered = fixture.join("registered");
    let unregistered = fixture.join("unregistered");
    fs::create_dir_all(&fixture).expect("fixture root");
    init_repo(&primary);
    git(&primary, &["worktree", "add", registered.to_str().unwrap()]);
    fs::create_dir_all(&unregistered).expect("unregistered dir");

    let dirty = cleanup_plan(&fixture, &primary, &registered, false, None);
    fs::write(registered.join("dirty.txt"), "dirty\n").expect("dirty marker");
    assert!(matches!(
        prepare_terminal_route("clean", &dirty).unwrap().cleanup,
        Some(CleanupDecision::Dirty { .. })
    ));
    fs::remove_file(registered.join("dirty.txt")).expect("remove dirty marker");

    fs::create_dir_all(registered.join(".csdlc")).expect("marker parent");
    fs::write(registered.join(".csdlc/live-worktree.marker"), "live\n").expect("live marker");
    let live = cleanup_plan(&fixture, &primary, &registered, false, None);
    assert!(matches!(
        prepare_terminal_route("clean", &live).unwrap().cleanup,
        Some(CleanupDecision::Live { .. })
    ));
    fs::remove_file(registered.join(".csdlc/live-worktree.marker")).expect("remove live marker");

    let preview = cleanup_plan(&fixture, &primary, &registered, false, None);
    let preview_plan = prepare_terminal_route("clean", &preview).expect("preview");
    let receipt_digest = match preview_plan.cleanup {
        Some(CleanupDecision::Removable { receipt_digest, .. }) => receipt_digest,
        other => panic!("expected removable preview, got {other:?}"),
    };

    let wrong_receipt = cleanup_plan(&fixture, &primary, &registered, true, Some("wrong".into()));
    assert!(prepare_terminal_route("clean", &wrong_receipt)
        .unwrap()
        .findings
        .iter()
        .any(|finding| finding.code == "preview_receipt_mismatch"));

    let remove = cleanup_plan(&fixture, &primary, &registered, true, Some(receipt_digest));
    assert!(matches!(
        prepare_terminal_route("clean", &remove).unwrap().cleanup,
        Some(CleanupDecision::Removed { .. })
    ));
    assert!(!registered.exists());

    let already_removed = cleanup_plan(&fixture, &primary, &registered, false, None);
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
        None,
    );
    assert!(matches!(
        prepare_terminal_route("clean", &absent).unwrap().cleanup,
        Some(CleanupDecision::Absent { .. })
    ));

    let unregistered_plan = cleanup_plan(&fixture, &primary, &unregistered, false, None);
    assert!(matches!(
        prepare_terminal_route("clean", &unregistered_plan)
            .unwrap()
            .cleanup,
        Some(CleanupDecision::Unregistered { .. })
    ));
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
    });
    let ready = prepare_terminal_route("cutover", &request).expect("cutover plan");
    assert_eq!(ready.status, TerminalRouteStatus::Ready);
    assert_eq!(ready.cutover.unwrap().executes_cutover, false);
}

fn cleanup_plan(
    approved_parent: &Path,
    repository_root: &Path,
    candidate_path: &Path,
    remove: bool,
    preview_receipt_digest: Option<String>,
) -> TerminalRouteRequest {
    let mut request = base_request();
    request.cleanup = Some(CleanupRouteRequest {
        approved_parent: approved_parent.to_path_buf(),
        repository_root: repository_root.to_path_buf(),
        candidate_path: candidate_path.to_path_buf(),
        remove,
        terminal_receipt: true,
        preview_receipt_digest,
    });
    request
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
