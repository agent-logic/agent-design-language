use csdlc_v2::{
    prepare_ready_publication, public_schema_bundle, publication::validate_ready_remote,
    DesignReview, IssueRecord, LifecyclePhase, NonSubstantiveProof, PublicationEvidence,
    PublicationLinkageMode, ReadyPublicationRequest, RemotePullRequest, ReviewEvidence, Store,
};
use std::collections::BTreeMap;
use std::path::Path;
use std::process::Command;

fn request() -> ReadyPublicationRequest {
    ReadyPublicationRequest {
        schema: "csdlc.ready_publication_request.v1".into(),
        issue: 604,
        expected_generation: 7,
        expected_digest: "digest".into(),
        actor: "worker-6".into(),
        repository: "agent-logic/agent-design-language".into(),
        code_repository: None,
        pull_request: 610,
        expected_head_sha: "0123456789abcdef0123456789abcdef01234567".into(),
        remote: "origin".into(),
        token_file: None,
    }
}

fn governed_draft() -> PublicationEvidence {
    PublicationEvidence {
        repository: "agent-logic/agent-design-language".into(),
        issue: 604,
        pull_request: 610,
        url: "https://github.com/agent-logic/agent-design-language/pull/610".into(),
        base: "main".into(),
        head: "codex/604-v2-publish-ready-reconciliation-exec".into(),
        revision: csdlc_v2::git::clean_commit_revision("0123456789abcdef0123456789abcdef01234567"),
        linkage_mode: Some(PublicationLinkageMode::Closing),
        draft: true,
        observed_state: "open".into(),
    }
}

fn remote(draft: bool) -> RemotePullRequest {
    RemotePullRequest {
        number: 610,
        url: "https://github.com/agent-logic/agent-design-language/pull/610".into(),
        repository: "agent-logic/agent-design-language".into(),
        base: "main".into(),
        head: "codex/604-v2-publish-ready-reconciliation-exec".into(),
        title: "[C-SDLC v2] Restore governed draft-to-ready publication reconciliation".into(),
        body: "Closes #604".into(),
        linkage_mode: PublicationLinkageMode::Closing,
        draft,
        state: "open".into(),
        head_sha: "0123456789abcdef0123456789abcdef01234567".into(),
        linked_issue: Some(604),
        linkage_source: Some("github_closing_issues_references".into()),
    }
}

fn record(draft: bool, head_sha: &str) -> IssueRecord {
    let revision = csdlc_v2::git::clean_commit_revision(head_sha);
    IssueRecord {
        schema: "csdlc.issue.v2".into(),
        issue: 604,
        repository: "agent-logic/agent-design-language".into(),
        code_repository: None,
        initialization_digest: "initialization".into(),
        phase: LifecyclePhase::Published,
        generation: 7,
        digest: "digest".into(),
        branch: Some("codex/604-v2-publish-ready-reconciliation-exec".into()),
        worktree: None,
        review_assignment: None,
        review: Some(ReviewEvidence {
            reviewer: "reviewer".into(),
            scope: vec!["README.md".into()],
            reviewed_revision: revision.clone(),
            findings: vec![],
            residual_risks: vec![],
            completed: true,
            non_substantive_proof: Some(NonSubstantiveProof {
                policy: "metadata_only".into(),
                from_revision: revision.clone(),
                to_revision: revision.clone(),
                from_commit: head_sha.into(),
                to_commit: head_sha.into(),
                changed_paths: vec![".csdlc/issues/604/index.json".into()],
            }),
        }),
        publication: Some(PublicationEvidence {
            repository: "agent-logic/agent-design-language".into(),
            issue: 604,
            pull_request: 610,
            url: "https://github.com/agent-logic/agent-design-language/pull/610".into(),
            base: "main".into(),
            head: "codex/604-v2-publish-ready-reconciliation-exec".into(),
            revision,
            linkage_mode: Some(PublicationLinkageMode::Closing),
            draft,
            observed_state: "open".into(),
        }),
        readiness: None,
        terminal: None,
        migration: None,
        design_path: ".csdlc/prepared/issues/604/design.md".into(),
        diagram_path: ".csdlc/prepared/issues/604/diagram.mmd".into(),
        design_review: DesignReview::Approved {
            reviewer: "reviewer".into(),
            revision: "design".into(),
        },
        cards: BTreeMap::new(),
        transitions: Vec::new(),
        audit: Vec::new(),
    }
}

fn write_record(root: &Path, record: &IssueRecord) {
    std::fs::create_dir_all(root.join(".csdlc/issues/604")).expect("issue dir");
    std::fs::write(
        root.join(".csdlc/issues/604/index.json"),
        serde_json::to_vec_pretty(record).expect("record"),
    )
    .expect("write record");
}

fn init_repo(root: &Path) -> String {
    git(
        root,
        &[
            "init",
            "-q",
            "-b",
            "codex/604-v2-publish-ready-reconciliation-exec",
        ],
    );
    git(root, &["config", "user.email", "test@example.invalid"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    std::fs::write(root.join("README.md"), "fixture\n").expect("readme");
    git(root, &["add", "README.md"]);
    git(root, &["commit", "-q", "-m", "fixture"]);
    git_out(root, &["rev-parse", "HEAD"])
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

fn git_out(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(root)
        .output()
        .expect("run git");
    assert!(output.status.success(), "git {:?} failed", args);
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

#[test]
fn ready_preflight_requires_exact_draft_pr_identity() {
    validate_ready_remote(&governed_draft(), &request(), &remote(true), true)
        .expect("exact draft identity is accepted before ready mutation");
}

#[test]
fn ready_post_readback_requires_exact_non_draft_pr_identity() {
    validate_ready_remote(&governed_draft(), &request(), &remote(false), false)
        .expect("exact non-draft identity is accepted after ready mutation");
}

#[test]
fn ready_readback_rejects_identity_drift_and_wrong_draft_state() {
    let governed = governed_draft();
    let request = request();
    let mut wrong_head = remote(true);
    wrong_head.head_sha = "1111111111111111111111111111111111111111".into();
    assert!(validate_ready_remote(&governed, &request, &wrong_head, true).is_err());

    let mut wrong_pr = remote(true);
    wrong_pr.number = 611;
    assert!(validate_ready_remote(&governed, &request, &wrong_pr, true).is_err());

    let mut closed = remote(false);
    closed.state = "closed".into();
    assert!(validate_ready_remote(&governed, &request, &closed, false).is_err());

    let mut linkage_drift = remote(false);
    linkage_drift.body = "Related #604".into();
    assert!(validate_ready_remote(&governed, &request, &linkage_drift, false).is_err());

    let mut missing_closing_relation = remote(false);
    missing_closing_relation.linked_issue = None;
    missing_closing_relation.linkage_source = None;
    assert!(validate_ready_remote(&governed, &request, &missing_closing_relation, false).is_err());

    assert!(validate_ready_remote(&governed, &request, &remote(false), true).is_err());
    assert!(validate_ready_remote(&governed, &request, &remote(true), false).is_err());
}

#[test]
fn ready_request_contracts_are_in_public_schema_bundle() {
    let schemas = public_schema_bundle();
    assert!(schemas.get("ready_publication_request").is_some());
    assert!(schemas
        .get("ready_publication_reconciliation_request")
        .is_some());
}

#[test]
fn ready_preparation_accepts_exact_published_draft_record() {
    let temp = tempfile::tempdir().expect("temp repo");
    let head = init_repo(temp.path());
    let mut request = request();
    request.expected_head_sha = head.clone();
    write_record(temp.path(), &record(true, &head));

    let governed = prepare_ready_publication(&Store::new(temp.path()), &request)
        .expect("exact published draft can be marked ready");
    assert!(governed.draft);
    assert_eq!(governed.pull_request, request.pull_request);
    assert_eq!(
        governed.revision,
        csdlc_v2::git::clean_commit_revision(&head)
    );
}

#[test]
fn ready_preparation_rejects_stale_cas_and_non_draft_publication() {
    let temp = tempfile::tempdir().expect("temp repo");
    let head = init_repo(temp.path());
    let mut request = request();
    request.expected_head_sha = head.clone();
    write_record(temp.path(), &record(true, &head));

    let mut stale = request.clone();
    stale.expected_digest = "stale".into();
    assert!(prepare_ready_publication(&Store::new(temp.path()), &stale).is_err());

    write_record(temp.path(), &record(false, &head));
    assert!(prepare_ready_publication(&Store::new(temp.path()), &request).is_err());
}
