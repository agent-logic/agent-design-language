use std::collections::BTreeMap;
use std::process::Command;

use csdlc_v2::finish::{
    derive_terminal, envelope_matches_record, envelope_releases_claim, load_cached_terminal,
    retain_cached_terminal,
};
use csdlc_v2::{
    DesignReview, FinishDisposition, FinishRequest, IssueRecord, IssueTerminalObservation,
    LifecyclePhase, MergeMethod, PublicationEvidence,
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

fn issue(state: &str, approved: bool) -> IssueTerminalObservation {
    IssueTerminalObservation {
        state: state.into(),
        labels: approved
            .then(|| csdlc_v2::finish::NO_PR_APPROVAL_LABEL.into())
            .into_iter()
            .collect(),
        observed_unix_seconds: 100,
    }
}

#[test]
fn closed_no_pr_terminal_cache_is_minimal_rebuildable_and_idempotent() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
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
    assert!(
        derive_terminal(&record, &no_pr_request(), &issue("open", false), None)
            .expect("derive")
            .is_none()
    );
}

#[test]
fn closed_no_pr_requires_canonical_github_approval_label() {
    let record = record(LifecyclePhase::Reviewed, None);
    let error = derive_terminal(&record, &no_pr_request(), &issue("closed", false), None)
        .expect_err("missing approval label");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
}

#[test]
fn mutable_terminal_cache_expires_and_is_bound_to_exact_record() {
    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    assert!(envelope_releases_claim(&envelope, &record, 400).expect("fresh"));
    assert!(!envelope_releases_claim(&envelope, &record, 401).expect("expired"));
    let mut changed = record.clone();
    changed.generation += 1;
    assert!(!envelope_releases_claim(&envelope, &changed, 100).expect("record drift"));
}

#[cfg(unix)]
#[test]
fn terminal_cache_rejects_symlinked_git_common_parent() {
    use std::os::unix::fs::symlink;

    let record = record(LifecyclePhase::Reviewed, None);
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
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
    let envelope = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
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

#[test]
fn mutable_terminal_cache_is_replaceable_by_a_fresher_live_observation() {
    let record = record(LifecyclePhase::Reviewed, None);
    let first = derive_terminal(&record, &no_pr_request(), &issue("closed", true), None)
        .expect("derive")
        .expect("terminal");
    let mut later_issue = issue("closed", true);
    later_issue.observed_unix_seconds = 200;
    let later = derive_terminal(&record, &no_pr_request(), &later_issue, None)
        .expect("derive later")
        .expect("terminal later");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    retain_cached_terminal(temp.path(), &first).expect("retain first");
    retain_cached_terminal(temp.path(), &later).expect("replace mutable cache");
    assert_eq!(
        load_cached_terminal(temp.path(), 5778)
            .expect("load")
            .expect("terminal"),
        later
    );
}

#[test]
fn finish_uses_the_canonical_issue_authority_lock() {
    use fs2::FileExt;

    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    std::fs::create_dir_all(temp.path().join(".csdlc/locks")).expect("lock dir");
    let store = csdlc_v2::Store::new(temp.path());
    let authority = store
        .authority_projection_lock(5778)
        .expect("canonical authority lock");
    let contender = std::fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(temp.path().join(".csdlc/locks/5778.lock"))
        .expect("contender");
    assert!(contender.try_lock_exclusive().is_err());
    FileExt::unlock(&authority).expect("canonical authority unlock");
    drop(authority);
    contender
        .try_lock_exclusive()
        .expect("canonical lock released");
}
