use std::path::Path;
use std::process::Command;

use csdlc_v2::finish::{
    classify_recordless_closeout_target, RecordlessCloseoutMode, RecordlessCloseoutRequest,
    RecordlessCloseoutTarget,
};
use csdlc_v2::{ClosingPullRequestIdentity, IssueTerminalObservation, PrStatePacket};

const ISSUE: u64 = 204;
const PR: u64 = 247;

#[test]
fn recordless_closeout_classifies_no_projection_merged_issue_as_eligible() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let result = classify_recordless_closeout_target(
        temp.path(),
        &request(&head, &"b".repeat(40)),
        &target(&head, &"b".repeat(40)),
        &closed_issue(),
        &merged_packet(&head, &"b".repeat(40)),
        &[closing_candidate()],
    )
    .expect("classify");
    assert_eq!(result.classification, "recordless_terminal_eligible");
    assert!(!result.source_projection_at_pr_head);
    assert!(!result.local_projection_present);
    assert!(!result.existing_closeout_receipt_present);
    assert!(result.terminal.is_some());
}

#[test]
fn recordless_closeout_rejects_source_projection_at_pr_head() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/204")).expect("issue dir");
    std::fs::write(
        temp.path().join(".csdlc/issues/204/index.json"),
        r#"{"issue":204}"#,
    )
    .expect("projection");
    git(temp.path(), &["add", ".csdlc/issues/204/index.json"]);
    git(temp.path(), &["commit", "-q", "-m", "source projection"]);
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let result = classify_recordless_closeout_target(
        temp.path(),
        &request(&head, &"b".repeat(40)),
        &target(&head, &"b".repeat(40)),
        &closed_issue(),
        &merged_packet(&head, &"b".repeat(40)),
        &[closing_candidate()],
    )
    .expect("classify");
    assert_eq!(
        result.classification,
        "local_projection_present_use_normal_finish"
    );
    assert!(result.source_projection_at_pr_head);
    assert!(result.local_projection_present);
}

#[test]
fn recordless_closeout_rejects_conflicting_historical_publication() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let issue_path = temp.path().join(".csdlc/issues/204/index.json");
    std::fs::create_dir_all(issue_path.parent().expect("parent")).expect("issue dir");
    std::fs::write(
        &issue_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 204,
            "publication": {"pull_request": 999}
        }))
        .expect("json"),
    )
    .expect("projection");
    git(temp.path(), &["add", ".csdlc/issues/204/index.json"]);
    git(
        temp.path(),
        &["commit", "-q", "-m", "historical conflicting publication"],
    );
    git(temp.path(), &["rm", "-q", ".csdlc/issues/204/index.json"]);
    git(
        temp.path(),
        &["commit", "-q", "-m", "remove stale projection"],
    );
    let result = classify_recordless_closeout_target(
        temp.path(),
        &request(&head, &"b".repeat(40)),
        &target(&head, &"b".repeat(40)),
        &closed_issue(),
        &merged_packet(&head, &"b".repeat(40)),
        &[closing_candidate()],
    )
    .expect("classify");
    assert_eq!(result.classification, "conflicting_historical_publication");
}

#[test]
fn recordless_closeout_rejects_mismatched_live_pr_identity() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    let head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let result = classify_recordless_closeout_target(
        temp.path(),
        &request(&head, &"b".repeat(40)),
        &target(&head, &"b".repeat(40)),
        &closed_issue(),
        &merged_packet(&head, &"c".repeat(40)),
        &[closing_candidate()],
    )
    .expect("classify");
    assert_eq!(result.classification, "live_pr_identity_mismatch");
}

fn request(head: &str, merge: &str) -> RecordlessCloseoutRequest {
    RecordlessCloseoutRequest {
        schema: "csdlc.recordless_closeout_request.v1".into(),
        actor: "tester".into(),
        approved_reason: "already merged issue has no recoverable projection".into(),
        mode: RecordlessCloseoutMode::ClassifyOnly,
        targets: vec![target(head, merge)],
        token_file: None,
    }
}

fn target(head: &str, merge: &str) -> RecordlessCloseoutTarget {
    RecordlessCloseoutTarget {
        issue: ISSUE,
        issue_repository: "owner/repo".into(),
        pr_repository: "owner/repo".into(),
        pull_request: PR,
        expected_head_sha: head.into(),
        expected_merge_sha: merge.into(),
    }
}

fn closed_issue() -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: "closed".into(),
        labels: Vec::new(),
        observed_unix_seconds: 100,
    }
}

fn closing_candidate() -> ClosingPullRequestIdentity {
    ClosingPullRequestIdentity {
        repository: "owner/repo".into(),
        pull_request: PR,
        state: "MERGED".into(),
        merged: true,
        merged_at: Some("2026-08-18T00:00:00Z".into()),
    }
}

fn merged_packet(head: &str, merge: &str) -> PrStatePacket {
    PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "owner/repo".into(),
        pull_request: PR,
        linked_issue: Some(ISSUE),
        linkage_source: Some("github_closing_issues_references".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "unknown".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/204".into()),
        head_sha: head.into(),
        url: Some("https://github.com/owner/repo/pull/247".into()),
        body: Some("Closes #204".into()),
        merged: true,
        merge_commit_sha: Some(merge.into()),
        checks: Vec::new(),
        required_check_names: Vec::new(),
        classification: "merged".into(),
    }
}

fn init_repo(root: &Path) {
    git(root, &["init", "-q", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    std::fs::write(root.join("README.md"), "fixture\n").expect("readme");
    git(root, &["add", "README.md"]);
    git(root, &["commit", "-q", "-m", "fixture"]);
}

fn git_out(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn git(root: &Path, args: &[&str]) {
    assert!(
        Command::new("git")
            .args(args)
            .current_dir(root)
            .status()
            .expect("run git")
            .success(),
        "git {:?} failed",
        args
    );
}
