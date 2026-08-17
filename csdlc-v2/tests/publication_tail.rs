use std::path::Path;
use std::process::Command;

use std::collections::BTreeMap;

use csdlc_v2::finish::validate_publication_head_in_repo;
use csdlc_v2::publication::{
    commit_publication_metadata_tail, current_head_sha,
    governed_publication_metadata_followup_paths, persist_publication_intent,
    publication_intent_dir, reconcile_action, resume_recorded_publication_intent,
    PublicationAction,
};
use csdlc_v2::{
    DesignReview, FinishRequest, IssueRecord, LifecyclePhase, MergeMethod, PublicationEvidence,
    PublicationIntent, PublicationLinkageMode, PublicationRequest, RemotePullRequest,
    ReviewEvidence, Store,
};

#[test]
fn publication_intent_cache_does_not_leave_git_visible_tail_for_publish_actions() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());

    let linked = temp.path().join("linked-worktree");
    git(
        temp.path(),
        &[
            "worktree",
            "add",
            "--detach",
            linked.to_str().expect("linked path"),
            "HEAD",
        ],
    );

    for (issue, action, observed) in [
        (306, PublicationAction::Create, None),
        (
            307,
            PublicationAction::Update,
            Some(remote_pr(307, "stale title")),
        ),
        (308, PublicationAction::Noop, Some(remote_pr(308, "title"))),
    ] {
        let mut intent = intent(issue);
        intent.title = "title".into();
        assert_eq!(
            reconcile_action(&intent, observed.as_ref()).expect("publication action"),
            action
        );
        persist_publication_intent(&linked, &intent).expect("persist publication intent");

        assert!(
            !linked
                .join(format!(".csdlc/publication/{issue}.intent.json"))
                .exists(),
            "{action:?} must not create a git-visible publication tail"
        );
        assert!(
            publication_intent_dir(&linked)
                .expect("publication intent dir")
                .join(format!("{issue}.intent.json"))
                .exists(),
            "{action:?} must preserve restart evidence in git common-dir"
        );
        assert!(
            csdlc_v2::git::worktree_is_clean(&linked).expect("linked worktree clean"),
            "{action:?} intent persistence must leave finish-visible status clean"
        );
    }
}

#[test]
fn publication_intent_interrupt_windows_remain_exact_clean_for_finish() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());

    let head = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .expect("head")
        .stdout;
    let expected = csdlc_v2::git::clean_commit_revision(&head);

    persist_publication_intent(temp.path(), &intent(306)).expect("interrupted after intent");
    assert!(csdlc_v2::git::worktree_is_clean(temp.path()).expect("clean after intent"));
    assert_eq!(
        csdlc_v2::git::substantive_revision(temp.path(), &["README.md".into()])
            .expect("substantive revision after intent"),
        expected
    );

    git(
        temp.path(),
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
    persist_publication_intent(temp.path(), &intent(306)).expect("retry after pushed head");
    assert!(csdlc_v2::git::worktree_is_clean(temp.path()).expect("clean after retry"));
    assert_eq!(
        csdlc_v2::git::substantive_revision(temp.path(), &["README.md".into()])
            .expect("substantive revision after retry"),
        expected
    );
    assert!(publication_intent_dir(temp.path())
        .expect("publication intent dir")
        .join("306.intent.json")
        .exists());
}

#[test]
fn publication_metadata_tail_is_committed_as_finish_ready_metadata_only_head() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn stable() {}\n").expect("source");
    git(temp.path(), &["add", "src/lib.rs"]);
    git(temp.path(), &["commit", "-q", "-m", "reviewed source"]);
    let reviewed_head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let reviewed_revision = csdlc_v2::git::substantive_revision(temp.path(), &["src".into()])
        .expect("reviewed revision");

    let mut historical = record(LifecyclePhase::Reviewed, None);
    historical.review = Some(ReviewEvidence {
        reviewer: "reviewer".into(),
        scope: vec!["src".into()],
        reviewed_revision: reviewed_revision.clone(),
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    });
    write_issue_record(temp.path(), &historical);
    git(temp.path(), &["add", ".csdlc/issues/306/index.json"]);
    git(temp.path(), &["commit", "-q", "-m", "review metadata"]);
    let published_head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    assert_ne!(reviewed_head, published_head);

    let mut published = historical;
    published.phase = LifecyclePhase::Published;
    published.publication = Some(PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 306,
        pull_request: 306,
        url: "https://github.com/agent-logic/agent-design-language/pull/306".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published_head),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    write_issue_record(temp.path(), &published);

    let metadata_head = commit_publication_metadata_tail(temp.path(), 306)
        .expect("commit metadata tail")
        .expect("metadata tail committed");
    assert_eq!(
        metadata_head.trim(),
        metadata_head,
        "metadata follow-up head must be normalized before strict remote equality"
    );
    assert!(csdlc_v2::git::worktree_is_clean(temp.path()).expect("worktree clean"));
    assert!(matches!(
        csdlc_v2::git::metadata_only_changed_paths(temp.path(), &published_head, &metadata_head),
        Ok(paths) if paths == vec![".csdlc/issues/306/index.json"]
    ));
    assert_eq!(
        governed_publication_metadata_followup_paths(
            temp.path(),
            306,
            &published_head,
            &metadata_head
        )
        .expect("governed publication metadata paths"),
        vec![".csdlc/issues/306/index.json"]
    );

    let mut request = finish_request();
    request.expected_head_sha = Some(metadata_head);
    validate_publication_head_in_repo(temp.path(), &published, &request)
        .expect("metadata-only publication tail remains finish-ready");
}

#[test]
fn publication_followup_rejects_unrelated_safe_metadata_paths() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());

    let published_head = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let mut published = record(LifecyclePhase::Published, None);
    published.publication = Some(PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 306,
        pull_request: 306,
        url: "https://github.com/agent-logic/agent-design-language/pull/306".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published_head),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    write_issue_record(temp.path(), &published);
    std::fs::create_dir_all(temp.path().join(".csdlc/evidence/999")).expect("evidence dir");
    std::fs::write(
        temp.path().join(".csdlc/evidence/999/unrelated.json"),
        "{\"unrelated\":true}\n",
    )
    .expect("unrelated evidence");
    git(
        temp.path(),
        &[
            "add",
            ".csdlc/issues/306/index.json",
            ".csdlc/evidence/999/unrelated.json",
        ],
    );
    git(temp.path(), &["commit", "-q", "-m", "mixed metadata tail"]);
    let metadata_head = git_out(temp.path(), &["rev-parse", "HEAD"]);

    assert!(matches!(
        csdlc_v2::git::metadata_only_changed_paths(temp.path(), &published_head, &metadata_head),
        Ok(paths)
            if paths
                == vec![
                    ".csdlc/evidence/999/unrelated.json",
                    ".csdlc/issues/306/index.json"
                ]
    ));
    let error = governed_publication_metadata_followup_paths(
        temp.path(),
        306,
        &published_head,
        &metadata_head,
    )
    .expect_err("unrelated safe metadata must not satisfy publication follow-up");
    assert_eq!(error.code, csdlc_v2::ErrorCode::ReconciliationRequired);
}

#[test]
fn publication_metadata_tail_rejects_pre_staged_non_governed_paths() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn changed() {}\n").expect("source");
    git(temp.path(), &["add", "src/lib.rs"]);

    let mut published = record(LifecyclePhase::Published, None);
    published.publication = Some(PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 306,
        pull_request: 306,
        url: "https://github.com/agent-logic/agent-design-language/pull/306".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        revision: csdlc_v2::git::clean_commit_revision(&git_out(
            temp.path(),
            &["rev-parse", "HEAD"],
        )),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    write_issue_record(temp.path(), &published);

    let before = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let error = commit_publication_metadata_tail(temp.path(), 306)
        .expect_err("pre-staged source must fail closed");
    assert_eq!(error.code, csdlc_v2::ErrorCode::UnsafeCheckout);
    assert_eq!(git_out(temp.path(), &["rev-parse", "HEAD"]), before);
    assert_eq!(
        git_out(temp.path(), &["diff", "--cached", "--name-only"]),
        "src/lib.rs"
    );
}

#[test]
fn publication_metadata_tail_rejects_unstaged_non_governed_issue_paths_without_staging_them() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());

    let mut published = record(LifecyclePhase::Published, None);
    published.publication = Some(PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 306,
        pull_request: 306,
        url: "https://github.com/agent-logic/agent-design-language/pull/306".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        revision: csdlc_v2::git::clean_commit_revision(&git_out(
            temp.path(),
            &["rev-parse", "HEAD"],
        )),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    write_issue_record(temp.path(), &published);
    let non_governed = temp.path().join(".csdlc/issues/306/debug.txt");
    std::fs::write(&non_governed, "scratch must not be staged\n").expect("scratch file");

    let before = git_out(temp.path(), &["rev-parse", "HEAD"]);
    let error = commit_publication_metadata_tail(temp.path(), 306)
        .expect_err("unstaged non-governed issue path must fail closed");
    assert_eq!(error.code, csdlc_v2::ErrorCode::UnsafeCheckout);
    assert_eq!(git_out(temp.path(), &["rev-parse", "HEAD"]), before);
    assert_eq!(
        git_out(temp.path(), &["diff", "--cached", "--name-only"]),
        ""
    );
    assert!(git_out(
        temp.path(),
        &["status", "--porcelain", "--untracked-files=all"]
    )
    .lines()
    .any(|line| line == "?? .csdlc/issues/306/debug.txt"));
}

#[test]
fn interrupted_after_record_publication_retry_can_commit_metadata_tail() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    std::fs::create_dir_all(temp.path().join("src")).expect("src");
    std::fs::write(temp.path().join("src/lib.rs"), "pub fn stable() {}\n").expect("source");
    git(temp.path(), &["add", "src/lib.rs"]);
    git(temp.path(), &["commit", "-q", "-m", "reviewed source"]);

    let mut reviewed = record(LifecyclePhase::Reviewed, None);
    reviewed.digest = "pre-publication-digest".into();
    reviewed.review = Some(ReviewEvidence {
        reviewer: "reviewer".into(),
        scope: vec!["src".into()],
        reviewed_revision: csdlc_v2::git::substantive_revision(temp.path(), &["src".into()])
            .expect("reviewed revision"),
        findings: vec![],
        residual_risks: vec![],
        completed: true,
        non_substantive_proof: None,
    });
    write_issue_record(temp.path(), &reviewed);
    git(temp.path(), &["add", ".csdlc/issues/306/index.json"]);
    git(
        temp.path(),
        &["commit", "-q", "-m", "published reviewed head"],
    );
    let published_head = git_out(temp.path(), &["rev-parse", "HEAD"]);

    let mut recorded = reviewed.clone();
    recorded.phase = LifecyclePhase::Published;
    recorded.generation += 1;
    recorded.digest = "post-publication-digest".into();
    recorded.publication = Some(PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 306,
        pull_request: 306,
        url: "https://github.com/agent-logic/agent-design-language/pull/306".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        revision: csdlc_v2::git::clean_commit_revision(&published_head),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: false,
        observed_state: "open".into(),
    });
    write_issue_record(temp.path(), &recorded);

    let original_request = publication_request(reviewed.generation, &reviewed.digest);
    let resumed = resume_recorded_publication_intent(&Store::new(temp.path()), &original_request)
        .expect("resume lookup")
        .expect("recorded publication can resume");
    assert_eq!(resumed.commit_sha, published_head);

    let metadata_head = commit_publication_metadata_tail(temp.path(), 306)
        .expect("commit metadata")
        .expect("metadata commit");
    assert_ne!(metadata_head, published_head);
    assert!(csdlc_v2::git::worktree_is_clean(temp.path()).expect("worktree clean"));

    let mut request = finish_request();
    request.expected_generation = recorded.generation;
    request.expected_digest = recorded.digest.clone();
    request.expected_head_sha = Some(metadata_head);
    validate_publication_head_in_repo(temp.path(), &recorded, &request)
        .expect("resumed metadata tail is finish-ready");
}

#[test]
fn clean_resume_uses_normalized_current_head_for_metadata_convergence() {
    let temp = tempfile::tempdir().expect("temp repo");
    init_repo(temp.path());
    let raw_head = csdlc_v2::git::run(temp.path(), &["rev-parse", "HEAD"])
        .expect("raw head")
        .stdout;
    let normalized = current_head_sha(temp.path()).expect("normalized head");
    assert_eq!(normalized, raw_head.trim());
    assert_eq!(
        normalized.trim(),
        normalized,
        "clean resume fallback must be safe for strict remote SHA equality"
    );
}

fn intent(issue: u64) -> PublicationIntent {
    PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue,
        repository: "agent-logic/agent-design-language".into(),
        issue_repository: "agent-logic/agent-design-language".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        title: "[v0.92][C-SDLC][defect] Prevent publication metadata tail from blocking exact-clean finish".into(),
        body: format!("Closes #{issue}"),
        linkage_mode: PublicationLinkageMode::Closing,
        draft: false,
        revision: "git-blake3:fixture:fixture".into(),
        commit_sha: "fixture".into(),
    }
}

fn remote_pr(issue: u64, title: &str) -> RemotePullRequest {
    RemotePullRequest {
        number: issue,
        url: format!("https://github.com/agent-logic/agent-design-language/pull/{issue}"),
        repository: "agent-logic/agent-design-language".into(),
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        title: title.into(),
        body: format!("Closes #{issue}"),
        linkage_mode: PublicationLinkageMode::Closing,
        draft: false,
        state: "open".into(),
        head_sha: "fixture".into(),
    }
}

fn record(phase: LifecyclePhase, publication: Option<PublicationEvidence>) -> IssueRecord {
    IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 306,
        repository: "agent-logic/agent-design-language".into(),
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
        design_path: ".csdlc/prepared/issues/306/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/306/diagram.mmd".into(),
        design_review: DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "reviewed".into(),
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: Vec::new(),
    }
}

fn write_issue_record(root: &Path, record: &IssueRecord) {
    std::fs::create_dir_all(root.join(".csdlc/issues/306")).expect("issue dir");
    std::fs::write(
        root.join(".csdlc/issues/306/index.json"),
        serde_json::to_vec_pretty(record).expect("record JSON"),
    )
    .expect("write record");
}

fn finish_request() -> FinishRequest {
    FinishRequest {
        schema: "csdlc.finish_request.v1".into(),
        issue: 306,
        expected_generation: 8,
        expected_digest: "canonical".into(),
        actor: "operator".into(),
        repository: "agent-logic/agent-design-language".into(),
        pull_request: Some(306),
        base: Some("main".into()),
        head: Some("codex/306-publication-tail-exact-clean-finish".into()),
        expected_head_sha: None,
        merge_method: MergeMethod::Squash,
        required_checks: Vec::new(),
        require_review: false,
        approved_no_pr_reason: None,
        token_file: None,
    }
}

fn publication_request(expected_generation: u64, expected_digest: &str) -> PublicationRequest {
    PublicationRequest {
        schema: "csdlc.publication_request.v1".into(),
        issue: 306,
        expected_generation,
        expected_digest: expected_digest.into(),
        actor: "publisher".into(),
        repository: "agent-logic/agent-design-language".into(),
        code_repository: None,
        base: "main".into(),
        head: "codex/306-publication-tail-exact-clean-finish".into(),
        title: "[v0.92][C-SDLC][defect] Prevent publication metadata tail from blocking exact-clean finish".into(),
        body: "Closes #306".into(),
        linkage_mode: PublicationLinkageMode::Closing,
        draft: false,
        remote: "origin".into(),
        token_file: None,
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
