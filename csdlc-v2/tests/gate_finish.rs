use std::collections::BTreeMap;
use std::process::Command;

use csdlc_v2::finish::{
    derive_terminal, envelope_matches_record, load_cached_terminal, retain_cached_terminal,
};
use csdlc_v2::{
    DesignReview, FinishDisposition, FinishRequest, IssueRecord, LifecyclePhase, MergeMethod,
    PublicationEvidence,
};

fn record(phase: LifecyclePhase, publication: Option<PublicationEvidence>) -> IssueRecord {
    IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 5778,
        repository: "owner/repo".into(),
        initialization_digest: "initialization".into(),
        phase,
        generation: 8,
        digest: "canonical".into(),
        claim: None,
        review_assignment: None,
        review: None,
        publication,
        readiness: None,
        terminal: None,
        migration: None,
        design_path: ".csdlc/prepared/issues/5778/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/5778/diagram.mmd".into(),
        design_review: DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "reviewed".into(),
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: Vec::new(),
    }
}

fn no_pr_request() -> FinishRequest {
    FinishRequest {
        schema: "csdlc.finish_request.v1".into(),
        issue: 5778,
        expected_generation: 8,
        expected_digest: "canonical".into(),
        claim_id: "released-claim".into(),
        actor: "operator".into(),
        repository: "owner/repo".into(),
        pull_request: None,
        base: None,
        head: None,
        expected_head_sha: None,
        merge_method: MergeMethod::Squash,
        required_checks: Vec::new(),
        require_review: false,
        approved_no_pr_reason: Some("approved administrative closure".into()),
        token_file: None,
    }
}

#[test]
fn closed_no_pr_terminal_cache_is_minimal_rebuildable_and_idempotent() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), "closed", None)
        .expect("derive")
        .expect("terminal");
    assert_eq!(envelope.disposition, FinishDisposition::ClosedNoPr);
    assert_eq!(
        envelope.approved_reason.as_deref(),
        Some("approved administrative closure")
    );

    let temp = tempfile::tempdir().expect("tempdir");
    let status = Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init");
    assert!(status.success());

    let first = retain_cached_terminal(temp.path(), &envelope).expect("retain");
    let second = retain_cached_terminal(temp.path(), &envelope).expect("idempotent retain");
    assert_eq!(first, second);
    let loaded = load_cached_terminal(temp.path(), 5778)
        .expect("load")
        .expect("cached terminal");
    assert_eq!(loaded, envelope);
    assert!(envelope_matches_record(&loaded, &record).expect("identity"));
    assert!(!first.starts_with(temp.path().join(".csdlc")));
}

#[test]
fn open_issue_without_pr_is_not_terminal() {
    let record = record(LifecyclePhase::Reviewed, None);
    assert!(derive_terminal(&record, &no_pr_request(), "open", None)
        .expect("derive")
        .is_none());
}

#[cfg(unix)]
#[test]
fn terminal_cache_rejects_symlinked_git_common_parent() {
    use std::os::unix::fs::symlink;

    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), "closed", None)
        .expect("derive")
        .expect("terminal");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let outside = tempfile::tempdir().expect("outside");
    symlink(outside.path(), temp.path().join(".git/csdlc-v2")).expect("symlink");

    let error = retain_cached_terminal(temp.path(), &envelope).expect_err("unsafe cache parent");
    assert_eq!(error.code, csdlc_v2::ErrorCode::UnsafeCheckout);
}

#[test]
fn concurrent_identical_finish_retention_converges() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), "closed", None)
        .expect("derive")
        .expect("terminal");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let root = std::sync::Arc::new(temp.path().to_path_buf());
    let envelope = std::sync::Arc::new(envelope);
    let barrier = std::sync::Arc::new(std::sync::Barrier::new(2));
    let workers = (0..2)
        .map(|_| {
            let root = root.clone();
            let envelope = envelope.clone();
            let barrier = barrier.clone();
            std::thread::spawn(move || {
                barrier.wait();
                retain_cached_terminal(&root, &envelope)
            })
        })
        .collect::<Vec<_>>();
    let retained = workers
        .into_iter()
        .map(|worker| worker.join().expect("worker").expect("retain"))
        .collect::<Vec<_>>();
    assert_eq!(retained[0], retained[1]);
}
