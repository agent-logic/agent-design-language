use std::collections::BTreeMap;
use std::process::Command;

use csdlc_v2::finish::{
    derive_historical_terminal, derive_terminal, diagnose_cached_terminal, envelope_matches_record,
    envelope_matches_record_in_repo, load_cached_terminal, retain_cached_terminal,
    select_historical_terminal, validate_finish_merge_authority, validate_historical_candidates,
    validate_historical_request, validate_publication_head_in_repo, CachedTerminalDiagnosticStatus,
};
use csdlc_v2::github::PrStatePacket;
use csdlc_v2::{
    ClosingPullRequestIdentity, DesignReview, FinishDisposition, FinishRequest,
    HistoricalFinishRequest, IssueRecord, IssueTerminalObservation, LifecyclePhase, MergeMethod,
    PublicationEvidence, PublicationLinkageMode, ReviewEvidence,
};

fn record(phase: LifecyclePhase, publication: Option<PublicationEvidence>) -> IssueRecord {
    IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 5778,
        repository: "owner/repo".into(),
        code_repository: None,
        initialization_digest: "initialization".into(),
        phase,
        generation: 8,
        digest: "canonical".into(),
        branch: None,
        worktree: None,
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

fn historical_request(disposition: FinishDisposition) -> HistoricalFinishRequest {
    let with_pr = disposition != FinishDisposition::ClosedNoPr;
    HistoricalFinishRequest {
        schema: "csdlc.historical_finish_request.v1".into(),
        issue: 5778,
        expected_generation: 8,
        expected_digest: "canonical".into(),
        actor: "operator".into(),
        issue_repository: "owner/repo".into(),
        disposition,
        pr_repository: with_pr.then(|| "canonical/repo".into()),
        pull_request: with_pr.then_some(9),
        expected_head_sha: with_pr.then(|| "a".repeat(40)),
        expected_merge_sha: (disposition == FinishDisposition::Merged).then(|| "b".repeat(40)),
        approved_reason: (disposition != FinishDisposition::Merged)
            .then(|| "approved historical disposition".into()),
        token_file: None,
    }
}

fn historical_packet(merged: bool) -> PrStatePacket {
    PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "canonical/repo".into(),
        pull_request: 9,
        linked_issue: Some(5778),
        linkage_source: Some("github_closing_issues_references".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "unknown".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5778".into()),
        head_sha: "a".repeat(40),
        url: Some("https://github.com/canonical/repo/pull/9".into()),
        body: Some("Closes owner/repo#5778".into()),
        merged,
        merge_commit_sha: merged.then(|| "b".repeat(40)),
        checks: Vec::new(),
        required_check_names: Vec::new(),
        classification: if merged { "merged" } else { "closed" }.into(),
    }
}

#[test]
fn historical_merged_finish_is_exact_and_does_not_invent_publication() {
    let record = record(LifecyclePhase::Implemented, None);
    let request = historical_request(FinishDisposition::Merged);
    let envelope = derive_historical_terminal(
        &record,
        &request,
        &issue("closed", false),
        Some(&historical_packet(true)),
    )
    .expect("historical merged terminal");
    assert_eq!(envelope.source, "live_github_historical_reconciliation");
    let expected_merge = "b".repeat(40);
    assert_eq!(envelope.merge_sha.as_deref(), Some(expected_merge.as_str()));
    assert!(envelope_matches_record(&envelope, &record).expect("canonical match"));
    assert!(record.review.is_none());
    assert!(record.publication.is_none());
}

#[test]
fn historical_request_fields_are_disposition_conditional() {
    let record = record(LifecyclePhase::Implemented, None);
    let mut merged = historical_request(FinishDisposition::Merged);
    merged.expected_merge_sha = None;
    assert!(validate_historical_request(&record, &merged).is_err());

    let mut closed = historical_request(FinishDisposition::ClosedUnmerged);
    closed.expected_merge_sha = Some("forbidden".into());
    assert!(validate_historical_request(&record, &closed).is_err());

    let mut no_pr = historical_request(FinishDisposition::ClosedNoPr);
    no_pr.pull_request = Some(9);
    assert!(validate_historical_request(&record, &no_pr).is_err());

    let mut malformed = historical_request(FinishDisposition::Merged);
    malformed.issue_repository = "owner/".into();
    assert!(validate_historical_request(&record, &malformed).is_err());
    let mut malformed = historical_request(FinishDisposition::Merged);
    malformed.expected_head_sha = Some("not-an-object-id".into());
    assert!(validate_historical_request(&record, &malformed).is_err());

    let mut value = serde_json::to_value(historical_request(FinishDisposition::Merged))
        .expect("historical request JSON");
    value["require_review"] = serde_json::json!(true);
    assert!(serde_json::from_value::<HistoricalFinishRequest>(value).is_err());
}

#[test]
fn historical_finish_requires_one_exact_closing_pr_candidate() {
    let request = historical_request(FinishDisposition::Merged);
    let expected = ClosingPullRequestIdentity {
        repository: "canonical/repo".into(),
        pull_request: 9,
        state: "MERGED".into(),
        merged: true,
        merged_at: None,
    };
    assert!(validate_historical_candidates(&request, std::slice::from_ref(&expected)).is_ok());
    assert!(validate_historical_candidates(&request, &[]).is_err());
    assert!(validate_historical_candidates(
        &request,
        &[
            expected.clone(),
            ClosingPullRequestIdentity {
                repository: "owner/repo".into(),
                pull_request: 5904,
                state: "CLOSED".into(),
                merged: false,
                merged_at: None,
            },
        ],
    )
    .is_ok());
    assert!(validate_historical_candidates(
        &request,
        &[
            ClosingPullRequestIdentity {
                merged_at: Some("2026-08-06T00:00:00Z".into()),
                ..expected.clone()
            },
            ClosingPullRequestIdentity {
                repository: "canonical/repo".into(),
                pull_request: 10,
                state: "MERGED".into(),
                merged: true,
                merged_at: Some("2026-08-06T01:00:00Z".into()),
            },
        ],
    )
    .is_err());

    let latest = ClosingPullRequestIdentity {
        repository: "canonical/repo".into(),
        pull_request: 9,
        state: "MERGED".into(),
        merged: true,
        merged_at: Some("2026-08-06T02:00:00+01:00".into()),
    };
    let earlier = ClosingPullRequestIdentity {
        repository: "canonical/repo".into(),
        pull_request: 8,
        state: "MERGED".into(),
        merged: true,
        merged_at: Some("2026-08-06T00:30:00Z".into()),
    };
    assert!(validate_historical_candidates(&request, &[earlier.clone(), latest.clone()]).is_ok());

    let mut wrong_request = request.clone();
    wrong_request.pull_request = Some(8);
    assert!(
        validate_historical_candidates(&wrong_request, &[earlier.clone(), latest.clone()]).is_err()
    );

    let mut missing = earlier.clone();
    missing.merged_at = None;
    assert!(validate_historical_candidates(&request, &[missing, latest.clone()]).is_err());

    let mut malformed = earlier.clone();
    malformed.merged_at = Some("not-rfc3339".into());
    assert!(validate_historical_candidates(&request, &[malformed, latest.clone()]).is_err());

    let tied = ClosingPullRequestIdentity {
        merged_at: Some("2026-08-06T01:00:00Z".into()),
        ..earlier
    };
    assert!(validate_historical_candidates(&request, &[tied, latest]).is_err());
}

#[test]
fn historical_finish_repeat_reuses_stable_terminal_authority() {
    let record = record(LifecyclePhase::Implemented, None);
    let request = historical_request(FinishDisposition::Merged);
    let first = derive_historical_terminal(
        &record,
        &request,
        &issue("closed", false),
        Some(&historical_packet(true)),
    )
    .expect("first observation");
    let mut later_issue = issue("closed", false);
    later_issue.observed_unix_seconds = 200;
    let later = derive_historical_terminal(
        &record,
        &request,
        &later_issue,
        Some(&historical_packet(true)),
    )
    .expect("later observation");
    assert_ne!(first.digest, later.digest);
    let (selected, already_terminal) =
        select_historical_terminal(Some(first.clone()), later).expect("select stable authority");
    assert!(already_terminal);
    assert_eq!(selected, first);
}

#[test]
fn historical_finish_rejects_open_or_mismatched_remote_identity() {
    let record = record(LifecyclePhase::Implemented, None);
    let request = historical_request(FinishDisposition::Merged);
    assert!(derive_historical_terminal(
        &record,
        &request,
        &issue("open", false),
        Some(&historical_packet(true)),
    )
    .is_err());

    let mut packet = historical_packet(true);
    packet.linked_issue = Some(99);
    assert!(
        derive_historical_terminal(&record, &request, &issue("closed", false), Some(&packet),)
            .is_err()
    );
}

#[test]
fn historical_non_merged_dispositions_require_explicit_authority() {
    let record = record(LifecyclePhase::Implemented, None);
    let closed = derive_historical_terminal(
        &record,
        &historical_request(FinishDisposition::ClosedUnmerged),
        &issue("closed", false),
        Some(&historical_packet(false)),
    )
    .expect("closed unmerged");
    assert_eq!(closed.disposition, FinishDisposition::ClosedUnmerged);
    assert!(closed.merge_sha.is_none());

    let no_pr_request = historical_request(FinishDisposition::ClosedNoPr);
    assert!(
        derive_historical_terminal(&record, &no_pr_request, &issue("closed", false), None,)
            .is_err()
    );
    let no_pr = derive_historical_terminal(&record, &no_pr_request, &issue("closed", true), None)
        .expect("approved no PR");
    assert_eq!(no_pr.disposition, FinishDisposition::ClosedNoPr);
}

#[test]
fn canonical_code_pr_derives_terminal_for_legacy_issue_authority() {
    let mut record = record(
        LifecyclePhase::Published,
        Some(PublicationEvidence {
            repository: "agent-logic/agent-design-language".into(),
            issue: 5778,
            pull_request: 9,
            url: "https://github.com/agent-logic/agent-design-language/pull/9".into(),
            base: "main".into(),
            head: "codex/5778".into(),
            revision: csdlc_v2::git::clean_commit_revision(&"a".repeat(40)),
            linkage_mode: Some(PublicationLinkageMode::Closing),
            draft: false,
            observed_state: "open".into(),
        }),
    );
    record.repository = "danielbaustin/agent-design-language".into();
    let mut request = no_pr_request();
    request.repository = record.repository.clone();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some("a".repeat(40));
    request.approved_no_pr_reason = None;
    let packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "agent-logic/agent-design-language".into(),
        pull_request: 9,
        linked_issue: Some(5778),
        linkage_source: Some("github_closing_issues_references".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "approved".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5778".into()),
        head_sha: "a".repeat(40),
        url: Some("https://github.com/agent-logic/agent-design-language/pull/9".into()),
        body: Some("Closes danielbaustin/agent-design-language#5778".into()),
        merged: true,
        merge_commit_sha: Some("b".repeat(40)),
        checks: vec![],
        required_check_names: vec![],
        classification: "merged".into(),
    };
    assert!(
        derive_terminal(&record, &request, &issue("open", false), Some(&packet))
            .expect("merged PR with open issue")
            .is_none()
    );
    let envelope = derive_terminal(&record, &request, &issue("closed", false), Some(&packet))
        .expect("derive split-authority terminal")
        .expect("terminal");
    assert_eq!(envelope.repository, record.repository);
    assert_eq!(envelope.pull_request, Some(9));
    assert_eq!(envelope.disposition, FinishDisposition::Merged);
    assert!(envelope_matches_record(&envelope, &record).expect("canonical match"));
}

#[test]
fn part_of_publication_cannot_authorize_finish() {
    let mut publication = PublicationEvidence {
        repository: "owner/repo".into(),
        issue: 5778,
        pull_request: 9,
        url: "https://example.test/pull/9".into(),
        base: "main".into(),
        head: "codex/5778".into(),
        revision: csdlc_v2::git::clean_commit_revision(&"a".repeat(40)),
        linkage_mode: Some(PublicationLinkageMode::PartOf),
        draft: false,
        observed_state: "open".into(),
    };
    let part_of_record = record(LifecyclePhase::Published, Some(publication.clone()));
    let mut request = no_pr_request();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some("a".repeat(40));
    request.approved_no_pr_reason = None;
    assert!(csdlc_v2::finish::validate_canonical_identity(&part_of_record, &request).is_err());

    publication.linkage_mode = Some(PublicationLinkageMode::Closing);
    let closing = record(LifecyclePhase::Published, Some(publication));
    assert!(csdlc_v2::finish::validate_canonical_identity(&closing, &request).is_ok());
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
fn finish_binary_validates_cached_terminal_without_remote_mutation() {
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
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/5778")).expect("issue dir");
    let index = temp.path().join(".csdlc/issues/5778/index.json");
    std::fs::write(
        &index,
        serde_json::to_vec_pretty(&record).expect("record JSON"),
    )
    .expect("record");
    let cache = retain_cached_terminal(temp.path(), &envelope).expect("retain");

    let validate = || {
        Command::new(env!("CARGO_BIN_EXE_csdlc-finish"))
            .args([
                "--root",
                &temp.path().to_string_lossy(),
                "--validate-cached-issue",
                "5778",
            ])
            .output()
            .expect("validate cached terminal")
    };
    let valid = validate();
    assert!(
        valid.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&valid.stdout),
        String::from_utf8_lossy(&valid.stderr)
    );
    assert!(String::from_utf8_lossy(&valid.stdout)
        .contains("\"schema\": \"csdlc.derived_terminal_validation.v1\""));

    let mut stale = record.clone();
    stale.generation += 1;
    std::fs::write(
        &index,
        serde_json::to_vec_pretty(&stale).expect("stale JSON"),
    )
    .expect("stale record");
    assert!(!validate().status.success());
    std::fs::write(
        &index,
        serde_json::to_vec_pretty(&record).expect("record JSON"),
    )
    .expect("restore record");

    let original_cache = std::fs::read(&cache).expect("cache");
    let mut malformed: serde_json::Value =
        serde_json::from_slice(&original_cache).expect("cache JSON");
    malformed["digest"] = serde_json::json!("wrong-digest");
    std::fs::write(
        &cache,
        serde_json::to_vec_pretty(&malformed).expect("malformed JSON"),
    )
    .expect("malformed cache");
    assert!(!validate().status.success());

    std::fs::remove_file(&cache).expect("remove cache");
    assert!(!validate().status.success());
}

#[test]
fn finish_diagnoses_stale_cached_terminal_without_mutating_receipts() {
    let record = record(LifecyclePhase::Reviewed, None);
    let mut terminal_record = record.clone();
    terminal_record.generation += 1;
    terminal_record.digest = "fresher-terminal-digest".repeat(2);
    let mut terminal_request = no_pr_request();
    terminal_request.expected_generation = terminal_record.generation;
    terminal_request.expected_digest = terminal_record.digest.clone();
    let envelope = derive_terminal(
        &terminal_record,
        &terminal_request,
        &issue("closed", true),
        None,
    )
    .expect("derive")
    .expect("terminal");
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/5778")).expect("issue dir");
    std::fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&record).expect("record JSON"),
    )
    .expect("record");
    let cache = retain_cached_terminal(temp.path(), &envelope).expect("retain terminal");
    let before = std::fs::read(&cache).expect("cache before");

    let report = diagnose_cached_terminal(temp.path(), 5778).expect("diagnose");
    assert_eq!(
        report.status,
        CachedTerminalDiagnosticStatus::StaleProjectionTerminalExists
    );
    assert!(!report.canonical_match);
    assert!(!report.immutable_receipt_overwrite_allowed);
    assert!(report.next_action.contains("worktree projection is stale"));
    assert_eq!(std::fs::read(&cache).expect("cache after"), before);

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc-finish"))
        .args([
            "--root",
            &temp.path().to_string_lossy(),
            "--diagnose-cached-issue",
            "5778",
        ])
        .output()
        .expect("diagnose cached terminal");
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(String::from_utf8_lossy(&output.stdout)
        .contains("\"status\": \"stale_projection_terminal_exists\""));
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

#[test]
fn published_finish_accepts_matching_git_topology() {
    let temp = tempfile::tempdir().expect("tempdir");
    assert!(Command::new("git")
        .args(["init", "-q", "-b", "codex/5778"])
        .current_dir(temp.path())
        .status()
        .expect("git init")
        .success());
    let mut record = record(
        LifecyclePhase::Published,
        Some(PublicationEvidence {
            repository: "owner/repo".into(),
            issue: 5778,
            pull_request: 9,
            url: "https://example.test/pull/9".into(),
            base: "main".into(),
            head: "codex/5778".into(),
            revision: csdlc_v2::git::clean_commit_revision("abc"),
            linkage_mode: Some(PublicationLinkageMode::Closing),
            draft: false,
            observed_state: "open".into(),
        }),
    );
    record.branch = Some("codex/5778".into());
    record.worktree = Some(temp.path().to_string_lossy().into_owned());
    let mut request = no_pr_request();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some("abc".into());
    request.approved_no_pr_reason = None;
    assert!(validate_finish_merge_authority(temp.path(), &record, &request, 100).is_ok());

    record.branch = Some("different-branch".into());
    let error = validate_finish_merge_authority(temp.path(), &record, &request, 100)
        .expect_err("branch mismatch");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
}

#[test]
fn derived_terminal_accepts_publication_metadata_only_head_and_rejects_substantive_drift() {
    let temp = tempfile::tempdir().expect("tempdir");
    let git = |args: &[&str]| {
        let output = Command::new("git")
            .args(args)
            .current_dir(temp.path())
            .output()
            .expect("git");
        assert!(output.status.success(), "git {:?}", args);
        String::from_utf8(output.stdout).unwrap().trim().to_owned()
    };
    git(&["init", "-q", "-b", "codex/5778"]);
    git(&["config", "user.email", "test@example.test"]);
    git(&["config", "user.name", "Test"]);
    std::fs::create_dir_all(temp.path().join("src")).unwrap();
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn stable() {}\n").unwrap();
    std::fs::write(temp.path().join("outside-review.txt"), "substantive\n").unwrap();
    git(&["add", "src/lib.rs", "outside-review.txt"]);
    git(&["commit", "-qm", "source"]);
    let source = git(&["rev-parse", "HEAD"]);
    let reviewed = csdlc_v2::git::substantive_revision(temp.path(), &["src".into()]).unwrap();
    assert_eq!(reviewed, csdlc_v2::git::clean_commit_revision(&source));

    let historical = record(LifecyclePhase::Reviewed, None);
    let completed_review = ReviewEvidence {
        reviewer: "reviewer".into(),
        scope: vec!["src".into()],
        reviewed_revision: reviewed,
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    };
    std::fs::create_dir_all(temp.path().join(".csdlc/issues/5778")).unwrap();
    std::fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&historical).unwrap(),
    )
    .unwrap();
    git(&["add", ".csdlc/issues/5778/index.json"]);
    git(&[
        "commit",
        "-qm",
        "publication anchor without review metadata",
    ]);
    let published = git(&["rev-parse", "HEAD"]);

    let mut record = historical;
    record.review = Some(completed_review);
    record.phase = LifecyclePhase::Published;
    record.publication = Some(PublicationEvidence {
        repository: "owner/repo".into(),
        issue: 5778,
        pull_request: 9,
        url: "https://example.test/pull/9".into(),
        base: "main".into(),
        head: "codex/5778".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    std::fs::write(
        temp.path().join(".csdlc/issues/5778/index.json"),
        serde_json::to_vec_pretty(&record).unwrap(),
    )
    .unwrap();
    git(&["add", ".csdlc/issues/5778/index.json"]);
    git(&["commit", "-qm", "publication metadata"]);
    let current = git(&["rev-parse", "HEAD"]);

    let mut request = no_pr_request();
    request.pull_request = Some(9);
    request.base = Some("main".into());
    request.head = Some("codex/5778".into());
    request.expected_head_sha = Some(current.clone());
    request.approved_no_pr_reason = None;
    validate_publication_head_in_repo(temp.path(), &record, &request)
        .expect("metadata-only forward head");

    let merged_packet = PrStatePacket {
        schema: "csdlc.github_pr_state.v1".into(),
        repository: "owner/repo".into(),
        pull_request: 9,
        linked_issue: Some(5778),
        linkage_source: Some("github".into()),
        state: "closed".into(),
        draft: false,
        merge_state: "unknown".into(),
        review_decision: "approved".into(),
        base_ref: Some("main".into()),
        head_ref: Some("codex/5778".into()),
        head_sha: current.clone(),
        url: Some("https://example.test/pull/9".into()),
        body: Some("Closes #5778".into()),
        merged: true,
        merge_commit_sha: Some("1111111111111111111111111111111111111111".into()),
        checks: vec![],
        required_check_names: vec![],
        classification: "merged".into(),
    };
    let merged = derive_terminal(
        &record,
        &request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 100,
        },
        Some(&merged_packet),
    )
    .expect("derive merged terminal")
    .expect("merged terminal");
    assert!(!envelope_matches_record(&merged, &record).expect("exact-only compatibility"));
    assert!(
        envelope_matches_record_in_repo(temp.path(), &merged, &record)
            .expect("repository-grounded metadata-only terminal")
    );

    git(&["checkout", "-qb", "rename-drift", &current]);
    std::fs::create_dir_all(temp.path().join(".csdlc/moved")).unwrap();
    git(&[
        "mv",
        "outside-review.txt",
        ".csdlc/moved/outside-review.txt",
    ]);
    git(&["commit", "-qm", "move substantive source into metadata"]);
    let rename_head = git(&["rev-parse", "HEAD"]);
    let mut rename_request = request.clone();
    rename_request.expected_head_sha = Some(rename_head);
    let rename_error = validate_publication_head_in_repo(temp.path(), &record, &rename_request)
        .expect_err("renamed substantive source must not become metadata-only drift");
    assert_eq!(
        rename_error.code,
        csdlc_v2::ErrorCode::ReconciliationRequired
    );
    let mut rename_packet = merged_packet.clone();
    rename_packet.head_sha = rename_request.expected_head_sha.clone().unwrap();
    let rename_terminal = derive_terminal(
        &record,
        &rename_request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 102,
        },
        Some(&rename_packet),
    )
    .expect("derive renamed terminal")
    .expect("renamed terminal");
    assert!(!envelope_matches_record(&rename_terminal, &record).expect("renamed terminal drift"));
    assert!(
        !envelope_matches_record_in_repo(temp.path(), &rename_terminal, &record)
            .expect("repository-grounded renamed terminal drift")
    );
    git(&["checkout", "-q", "codex/5778"]);

    git(&["checkout", "-qb", "non-ancestor", &source]);
    std::fs::create_dir_all(temp.path().join(".csdlc/evidence/5778")).unwrap();
    std::fs::write(
        temp.path().join(".csdlc/evidence/5778/non-ancestor.json"),
        "{}\n",
    )
    .unwrap();
    git(&["add", ".csdlc/evidence/5778/non-ancestor.json"]);
    git(&["commit", "-qm", "non-ancestor metadata"]);
    let non_ancestor_head = git(&["rev-parse", "HEAD"]);
    let mut non_ancestor_request = request.clone();
    non_ancestor_request.expected_head_sha = Some(non_ancestor_head.clone());
    let mut non_ancestor_packet = merged_packet.clone();
    non_ancestor_packet.head_sha = non_ancestor_head;
    let non_ancestor_terminal = derive_terminal(
        &record,
        &non_ancestor_request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 103,
        },
        Some(&non_ancestor_packet),
    )
    .expect("derive non-ancestor terminal")
    .expect("non-ancestor terminal");
    assert!(
        !envelope_matches_record_in_repo(temp.path(), &non_ancestor_terminal, &record)
            .expect("metadata-only non-ancestor must fail closed")
    );
    git(&["checkout", "-q", "codex/5778"]);

    let missing_head = "f".repeat(40);
    let mut missing_request = request.clone();
    missing_request.expected_head_sha = Some(missing_head.clone());
    let mut missing_packet = merged_packet.clone();
    missing_packet.head_sha = missing_head;
    let missing_terminal = derive_terminal(
        &record,
        &missing_request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 104,
        },
        Some(&missing_packet),
    )
    .expect("derive missing-commit terminal")
    .expect("missing-commit terminal");
    assert!(
        !envelope_matches_record_in_repo(temp.path(), &missing_terminal, &record)
            .expect("missing terminal head must fail closed")
    );

    let mut exact = record.clone();
    exact.publication.as_mut().unwrap().revision = csdlc_v2::git::clean_commit_revision(&current);
    std::fs::write(temp.path().join("src/lib.rs"), "dirty\n").unwrap();
    assert!(validate_publication_head_in_repo(temp.path(), &exact, &request).is_err());
    git(&["checkout", "--", "src/lib.rs"]);

    let mut wrong_local_request = request.clone();
    wrong_local_request.expected_head_sha = Some(published.clone());
    assert!(validate_publication_head_in_repo(temp.path(), &record, &wrong_local_request).is_err());

    let mut malformed_publication = record.clone();
    malformed_publication.publication.as_mut().unwrap().revision =
        format!("git-blake3:{published}:garbage");
    assert!(
        validate_publication_head_in_repo(temp.path(), &malformed_publication, &request).is_err()
    );
    assert!(!envelope_matches_record(&merged, &malformed_publication)
        .expect("malformed publication identity"));
    assert!(
        !envelope_matches_record_in_repo(temp.path(), &merged, &malformed_publication)
            .expect("repository-grounded malformed publication identity")
    );

    let mut changed_scope = record.clone();
    changed_scope.review.as_mut().unwrap().scope = vec!["src/lib.rs".into()];
    assert!(validate_publication_head_in_repo(temp.path(), &changed_scope, &request).is_err());

    let mut malformed_review = record.clone();
    let reviewed_commit = malformed_review
        .review
        .as_ref()
        .unwrap()
        .reviewed_revision
        .split(':')
        .nth(1)
        .unwrap()
        .to_owned();
    malformed_review.review.as_mut().unwrap().reviewed_revision =
        format!("git-blake3:{reviewed_commit}:garbage");
    assert!(validate_publication_head_in_repo(temp.path(), &malformed_review, &request).is_err());

    std::fs::write(temp.path().join("src/lib.rs"), "pub fn changed() {}\n").unwrap();
    git(&["add", "src/lib.rs"]);
    git(&["commit", "-qm", "substantive drift"]);
    request.expected_head_sha = Some(git(&["rev-parse", "HEAD"]));
    let error = validate_publication_head_in_repo(temp.path(), &record, &request)
        .expect_err("substantive forward head must fail closed");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);

    let mut substantive_packet = merged_packet;
    substantive_packet.head_sha = request.expected_head_sha.clone().unwrap();
    let substantive = derive_terminal(
        &record,
        &request,
        &IssueTerminalObservation {
            state: "closed".into(),
            labels: vec![],
            observed_unix_seconds: 101,
        },
        Some(&substantive_packet),
    )
    .expect("derive substantive terminal")
    .expect("substantive terminal");
    assert!(!envelope_matches_record(&substantive, &record).expect("substantive terminal drift"));
    assert!(
        !envelope_matches_record_in_repo(temp.path(), &substantive, &record)
            .expect("repository-grounded substantive terminal drift")
    );
}
