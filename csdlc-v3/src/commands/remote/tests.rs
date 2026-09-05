use super::{
    accepted_review, deliver, receipt, review_from_accepted_evidence, AcceptedPvfResult,
    AcceptedReviewEvidence, AuthoritySource, RemoteDeliveryInput, RemoteDeliveryRejectReason,
    VerifiableSubject, VerificationRejectReason, Verified,
};
use crate::publication::{
    classify_cleanup, cleanup_candidate_from_git_registration, derive_finish,
    execute_cleanup_removal, publish, CleanupCandidate, CleanupClassification, CleanupRejectReason,
    FinishClassification, FinishRejectReason, GitWorktreeRegistration, IssueReadback,
    PublicationMode, PublicationRejectReason, PublicationRequest, PullRequestReadback,
};
use crate::review::{
    authorize_publication, FindingDisposition, ReviewFinding, ReviewRejectReason, ReviewTarget,
};
use crate::REMOTE_DELIVERY_PREDECESSORS;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};

const ISSUE: u64 = 504;
const REVISION: &str = "974abc520454690f0b392162b9ced783e8584017";
const REPOSITORY: &str = "agent-logic/agent-design-language";
static NEXT_CLEANUP_ID: AtomicUsize = AtomicUsize::new(0);

fn pvf() -> AcceptedPvfResult {
    AcceptedPvfResult {
        issue: ISSUE,
        revision: REVISION.to_owned(),
        evidence_digest: "accepted-pvf-digest".to_owned(),
    }
}

fn publication(mode: PublicationMode, body: &str) -> PublicationRequest {
    PublicationRequest {
        repository: REPOSITORY.to_owned(),
        issue: ISSUE,
        pull_request: 586,
        mode,
        publisher: "worker-6-publisher".to_owned(),
        body: body.to_owned(),
        head_sha: REVISION.to_owned(),
    }
}

fn merged_pr(head_sha: &str) -> PullRequestReadback {
    PullRequestReadback {
        repository: REPOSITORY.to_owned(),
        number: 586,
        head_sha: head_sha.to_owned(),
        merged: true,
        closes_issue: Some(ISSUE),
        part_of_issue: None,
    }
}

fn merged_part_of_pr(head_sha: &str) -> PullRequestReadback {
    PullRequestReadback {
        repository: REPOSITORY.to_owned(),
        number: 586,
        head_sha: head_sha.to_owned(),
        merged: true,
        closes_issue: None,
        part_of_issue: Some(ISSUE),
    }
}

fn issue(open: bool) -> IssueReadback {
    IssueReadback {
        repository: REPOSITORY.to_owned(),
        issue: ISSUE,
        open,
    }
}

fn cleanup() -> CleanupCandidate {
    let id = NEXT_CLEANUP_ID.fetch_add(1, Ordering::SeqCst);
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "remote-cleanup-worktree-{}-{id}",
            std::process::id()
        ));
    if path.exists() {
        fs::remove_dir_all(&path).expect("remove prior cleanup fixture");
    }
    fs::create_dir_all(&path).expect("create cleanup fixture");
    cleanup_at(path)
}

fn cleanup_at(path: PathBuf) -> CleanupCandidate {
    let approved_worktree_parent = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let repository_root = approved_worktree_parent.join(format!(
        "remote-cleanup-repo-{}-{}",
        std::process::id(),
        NEXT_CLEANUP_ID.fetch_add(1, Ordering::SeqCst)
    ));
    let worktree_name = path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("worktree");
    let git_common_dir = repository_root
        .join(".git")
        .join("worktrees")
        .join(worktree_name);
    fs::create_dir_all(&git_common_dir).expect("create git common dir fixture");
    fs::write(
        path.join(".git"),
        format!("gitdir: {}\n", git_common_dir.display()),
    )
    .expect("write worktree gitdir pointer");
    fs::write(
        git_common_dir.join("gitdir"),
        format!("{}/.git\n", path.display()),
    )
    .expect("write common gitdir pointer");
    let registration = GitWorktreeRegistration {
        repository_root,
        worktree_path: path.clone(),
        git_common_dir,
    };
    cleanup_candidate_from_git_registration(
        &approved_worktree_parent,
        &path,
        registration,
        true,
        false,
        true,
        true,
        None,
        false,
        false,
    )
    .expect("registered cleanup identity")
}

fn verified<T: VerifiableSubject>(value: T, source: AuthoritySource) -> Verified<T> {
    let subject_digest = value.subject_digest();
    Verified::new(value, receipt(source, &subject_digest)).expect("verified fixture")
}

fn delivery_input(mode: PublicationMode, body: &str, issue_open: bool) -> RemoteDeliveryInput {
    let pull_request = match mode {
        PublicationMode::Closing => merged_pr(REVISION),
        PublicationMode::PartOf => merged_part_of_pr(REVISION),
    };
    RemoteDeliveryInput::new(
        verified(pvf(), AuthoritySource::Pvf),
        verified(
            accepted_review(
                ISSUE,
                REVISION,
                "worker-6-implementation",
                "independent-reviewer",
                mode,
            ),
            AuthoritySource::Review,
        ),
        verified(publication(mode, body), AuthoritySource::PublicationIntent),
        verified(pull_request, AuthoritySource::GithubReadback),
        verified(issue(issue_open), AuthoritySource::GithubReadback),
        verified(cleanup(), AuthoritySource::WorktreeInspection),
    )
}

#[test]
fn v3e_denominator_is_exact() {
    assert_eq!(REMOTE_DELIVERY_PREDECESSORS, [174, 175, 176, 177, 178]);
    for issue in REMOTE_DELIVERY_PREDECESSORS {
        assert!(crate::is_v3e_remote_delivery_predecessor(issue));
    }
    assert!(!crate::is_v3e_remote_delivery_predecessor(173));
    assert!(!crate::is_v3e_remote_delivery_predecessor(179));
}

#[test]
fn accepted_pvf_result_reaches_safe_cleanup_preview() {
    let input = delivery_input(PublicationMode::Closing, "Closes #504", false);
    let cleanup_path = input.cleanup.value().candidate_path.clone();
    let result = deliver(input).expect("closing publication reaches cleanup");
    assert_eq!(
        result.finish,
        FinishClassification::TerminalClosedOut {
            pull_request: 586,
            issue: ISSUE,
            head_sha: REVISION.to_owned()
        }
    );
    assert_eq!(
        result.cleanup,
        Some(CleanupClassification::PreviewEligible { path: cleanup_path })
    );
}

#[test]
fn review_binds_exact_scope_revision_reviewer_and_findings() {
    let mut review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6-implementation",
        "independent-reviewer",
        PublicationMode::Closing,
    );
    let target = ReviewTarget {
        repository: REPOSITORY.to_owned(),
        issue: ISSUE,
        mode: PublicationMode::Closing,
    };
    assert!(authorize_publication(&review, REVISION, "worker-6-publisher", target.clone()).is_ok());

    assert_eq!(
        authorize_publication(
            &review,
            "different-head",
            "worker-6-publisher",
            target.clone()
        ),
        Err(ReviewRejectReason::StaleReview)
    );
    review.findings.push(ReviewFinding {
        id: "p1".to_owned(),
        disposition: FindingDisposition::Actionable,
    });
    assert_eq!(
        authorize_publication(&review, REVISION, "worker-6-publisher", target),
        Err(ReviewRejectReason::ActionableFinding)
    );
}

#[test]
fn same_principal_cannot_self_authorize_publication() {
    let review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6",
        "worker-6",
        PublicationMode::Closing,
    );
    let target = ReviewTarget {
        repository: REPOSITORY.to_owned(),
        issue: ISSUE,
        mode: PublicationMode::Closing,
    };
    assert_eq!(
        authorize_publication(&review, REVISION, "publisher", target),
        Err(ReviewRejectReason::SamePrincipal)
    );
}

#[test]
fn whitespace_variant_principal_cannot_self_authorize_publication() {
    let review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6",
        " worker-6 ",
        PublicationMode::Closing,
    );
    let target = ReviewTarget {
        repository: REPOSITORY.to_owned(),
        issue: ISSUE,
        mode: PublicationMode::Closing,
    };
    assert_eq!(
        authorize_publication(&review, REVISION, "publisher", target),
        Err(ReviewRejectReason::SamePrincipal)
    );
}

#[test]
fn publication_modes_require_exact_relation() {
    let review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6-implementation",
        "independent-reviewer",
        PublicationMode::Closing,
    );
    let authorization = authorize_publication(
        &review,
        REVISION,
        "worker-6-publisher",
        ReviewTarget {
            repository: REPOSITORY.to_owned(),
            issue: ISSUE,
            mode: PublicationMode::Closing,
        },
    )
    .expect("authorized");

    assert_eq!(
        publish(
            publication(PublicationMode::Closing, "Related #504"),
            &authorization
        ),
        Err(PublicationRejectReason::MissingClosingRelation)
    );
    assert_eq!(
        publish(
            publication(PublicationMode::Closing, "Closes #504\nPart-Of #504"),
            &authorization
        ),
        Err(PublicationRejectReason::ClosingModeHasNonClosingRelation)
    );
}

#[test]
fn part_of_checkpoint_does_not_close_parent_or_grant_terminal_authority() {
    let review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6-implementation",
        "independent-reviewer",
        PublicationMode::PartOf,
    );
    let authorization = authorize_publication(
        &review,
        REVISION,
        "worker-6-publisher",
        ReviewTarget {
            repository: REPOSITORY.to_owned(),
            issue: ISSUE,
            mode: PublicationMode::PartOf,
        },
    )
    .expect("part-of authorized");
    let evidence = publish(
        publication(PublicationMode::PartOf, "Part-Of #504"),
        &authorization,
    )
    .expect("part-of publication");
    assert_eq!(
        derive_finish(&evidence, &merged_part_of_pr(REVISION), &issue(true)),
        FinishClassification::CheckpointCompleted {
            pull_request: 586,
            issue: ISSUE,
            invalidates_review_and_publication: true
        }
    );
    assert_eq!(
        derive_finish(&evidence, &merged_part_of_pr(REVISION), &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::PartOfParentClosed
        }
    );
}

#[test]
fn part_of_checkpoint_completes_through_end_to_end_delivery_without_cleanup() {
    let result = deliver(delivery_input(
        PublicationMode::PartOf,
        "Part-Of #504",
        true,
    ))
    .expect("part-of publication completes as checkpoint");
    assert_eq!(
        result.finish,
        FinishClassification::CheckpointCompleted {
            pull_request: 586,
            issue: ISSUE,
            invalidates_review_and_publication: true
        }
    );
    assert_eq!(result.cleanup, None);
}

#[test]
fn finish_derives_terminal_truth_from_remote_readback_not_local_claims() {
    let review = accepted_review(
        ISSUE,
        REVISION,
        "worker-6-implementation",
        "independent-reviewer",
        PublicationMode::Closing,
    );
    let authorization = authorize_publication(
        &review,
        REVISION,
        "worker-6-publisher",
        ReviewTarget {
            repository: REPOSITORY.to_owned(),
            issue: ISSUE,
            mode: PublicationMode::Closing,
        },
    )
    .expect("authorized");
    let evidence = publish(
        publication(PublicationMode::Closing, "Closes #504"),
        &authorization,
    )
    .expect("closing publication");
    let mut unmerged = merged_pr(REVISION);
    unmerged.merged = false;
    assert_eq!(
        derive_finish(&evidence, &unmerged, &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::PullRequestNotMerged
        }
    );
    assert_eq!(
        derive_finish(&evidence, &merged_pr("stale-head"), &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::HeadMismatch
        }
    );
    assert_eq!(
        derive_finish(&evidence, &merged_pr(REVISION), &issue(true)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::ClosingParentStillOpen
        }
    );
    let mut unrelated = merged_pr(REVISION);
    unrelated.number = 999;
    assert_eq!(
        derive_finish(&evidence, &unrelated, &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::PullRequestMismatch
        }
    );
    let mut missing_linkage = merged_pr(REVISION);
    missing_linkage.closes_issue = None;
    assert_eq!(
        derive_finish(&evidence, &missing_linkage, &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::ClosingLinkageMissing
        }
    );
}

#[test]
fn cleanup_is_separate_preview_first_and_path_exact() {
    let candidate = cleanup();
    let path = candidate.candidate_path.clone();
    assert_eq!(
        classify_cleanup(&candidate),
        Ok(CleanupClassification::PreviewEligible { path: path.clone() })
    );
    let mut remove = cleanup();
    let remove_path = remove.candidate_path.clone();
    remove.preview = false;
    remove.preview_receipt = true;
    remove.preview_identity_digest = Some(remove.registration_digest.clone());
    assert_eq!(
        classify_cleanup(&remove),
        Ok(CleanupClassification::RemoveEligible { path: remove_path })
    );
    let mut attack = cleanup();
    let attack_path = attack.registration.worktree_path.with_extension("extra");
    fs::create_dir_all(&attack_path).expect("create sibling attack fixture");
    attack.candidate_path = attack_path;
    assert_eq!(
        classify_cleanup(&attack),
        Err(CleanupRejectReason::PathMismatch)
    );
    let mut relative = cleanup();
    relative.candidate_path = PathBuf::from("adl-issue-504");
    assert_eq!(
        classify_cleanup(&relative),
        Err(CleanupRejectReason::NonCanonicalPath)
    );
    let mut parent_component = cleanup();
    parent_component.candidate_path =
        PathBuf::from("/Volumes/FastWork/adl-worktrees/../adl-worktrees/adl-issue-504");
    assert_eq!(
        classify_cleanup(&parent_component),
        Err(CleanupRejectReason::NonCanonicalPath)
    );
    let mut unverified = cleanup();
    unverified.registration.worktree_path = unverified
        .registration
        .worktree_path
        .join("missing-worktree");
    assert_eq!(
        classify_cleanup(&unverified),
        Err(CleanupRejectReason::UnregisteredWorktree)
    );
    let mut no_preview_receipt = cleanup();
    no_preview_receipt.preview = false;
    assert_eq!(
        classify_cleanup(&no_preview_receipt),
        Err(CleanupRejectReason::MissingPreviewReceipt)
    );
    no_preview_receipt.preview_receipt = true;
    no_preview_receipt.preview_identity_digest = Some("forged-preview".to_owned());
    assert_eq!(
        classify_cleanup(&no_preview_receipt),
        Err(CleanupRejectReason::PreviewReceiptMismatch)
    );
    let broad_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let broad = cleanup_candidate_from_git_registration(
        &broad_path.join("target"),
        &broad_path,
        GitWorktreeRegistration {
            repository_root: broad_path.clone(),
            worktree_path: broad_path.clone(),
            git_common_dir: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(".git"),
        },
        true,
        false,
        true,
        true,
        None,
        false,
        false,
    );
    assert_eq!(
        broad.and_then(|candidate| classify_cleanup(&candidate)),
        Err(CleanupRejectReason::ProtectedPath)
    );
}

#[test]
fn cleanup_removal_executes_and_distinguishes_removed_states() {
    let mut remove = cleanup();
    let remove_path = remove.candidate_path.clone();
    remove.preview = false;
    remove.preview_receipt = true;
    remove.preview_identity_digest = Some(remove.registration_digest.clone());
    assert_eq!(
        execute_cleanup_removal(&remove),
        Ok(CleanupClassification::Removed {
            path: remove_path.clone()
        })
    );
    assert!(!remove_path.exists());
    assert_eq!(
        execute_cleanup_removal(&remove),
        Ok(CleanupClassification::AlreadyRemoved { path: remove_path })
    );

    let mut unregistered = cleanup();
    let candidate_path = unregistered.candidate_path.clone();
    unregistered.preview = false;
    unregistered.preview_receipt = true;
    unregistered.preview_identity_digest = Some(unregistered.registration_digest.clone());
    unregistered.registration.worktree_path = candidate_path.with_extension("missing-registration");
    assert_eq!(
        execute_cleanup_removal(&unregistered),
        Err(CleanupRejectReason::UnregisteredWorktree)
    );
}

#[test]
fn already_removed_cleanup_still_requires_terminal_and_preview_gates() {
    let id = NEXT_CLEANUP_ID.fetch_add(1, Ordering::SeqCst);
    let missing_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "remote-cleanup-missing-worktree-{}-{id}",
            std::process::id()
        ));
    if missing_path.exists() {
        fs::remove_dir_all(&missing_path).expect("remove stale missing fixture");
    }
    let mut candidate = CleanupCandidate {
        preview: false,
        preview_receipt: true,
        committed_closed_out: true,
        terminal_receipt: true,
        approved_worktree_parent: PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target"),
        registration: GitWorktreeRegistration {
            repository_root: PathBuf::from(env!("CARGO_MANIFEST_DIR")),
            worktree_path: missing_path.clone(),
            git_common_dir: missing_path.join(".git"),
        },
        candidate_path: missing_path,
        registration_digest: "missing-registration-digest".to_owned(),
        preview_identity_digest: Some("missing-registration-digest".to_owned()),
        dirty: false,
        live: false,
    };

    candidate.committed_closed_out = false;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::NotTerminal)
    );
    candidate.committed_closed_out = true;
    candidate.terminal_receipt = false;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::MissingReceipt)
    );
    candidate.terminal_receipt = true;
    candidate.preview_receipt = false;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::MissingPreviewReceipt)
    );
    candidate.preview_receipt = true;
    candidate.dirty = true;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::DirtyWorktree)
    );
    candidate.dirty = false;
    candidate.live = true;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::LiveWorktree)
    );
    candidate.live = false;
    candidate.preview = true;
    assert_eq!(
        execute_cleanup_removal(&candidate),
        Err(CleanupRejectReason::UnregisteredWorktree)
    );
}

#[test]
fn cleanup_requires_committed_closed_out_state_and_receipt() {
    let mut candidate = cleanup();
    candidate.committed_closed_out = false;
    assert_eq!(
        classify_cleanup(&candidate),
        Err(CleanupRejectReason::NotTerminal)
    );
    candidate.committed_closed_out = true;
    candidate.terminal_receipt = false;
    assert_eq!(
        classify_cleanup(&candidate),
        Err(CleanupRejectReason::MissingReceipt)
    );
}

#[test]
fn remote_workflow_refuses_missing_pvf_and_revision_mismatch() {
    let mut input = RemoteDeliveryInput::new(
        verified(pvf(), AuthoritySource::Pvf),
        verified(
            accepted_review(
                ISSUE,
                REVISION,
                "worker-6-implementation",
                "independent-reviewer",
                PublicationMode::Closing,
            ),
            AuthoritySource::Review,
        ),
        verified(
            publication(PublicationMode::Closing, "Closes #504"),
            AuthoritySource::PublicationIntent,
        ),
        verified(merged_pr(REVISION), AuthoritySource::GithubReadback),
        verified(issue(false), AuthoritySource::GithubReadback),
        verified(cleanup(), AuthoritySource::WorktreeInspection),
    );
    input.pvf = verified(
        AcceptedPvfResult {
            evidence_digest: String::new(),
            ..pvf()
        },
        AuthoritySource::Pvf,
    );
    assert_eq!(
        deliver(input.clone()),
        Err(RemoteDeliveryRejectReason::PvfEvidenceMissing)
    );
    input.pvf = verified(
        AcceptedPvfResult {
            revision: "different".to_owned(),
            ..pvf()
        },
        AuthoritySource::Pvf,
    );
    assert_eq!(
        deliver(input),
        Err(RemoteDeliveryRejectReason::PvfRevisionMismatch)
    );
}

#[test]
fn remote_delivery_rejects_unverified_or_wrong_source_observations() {
    assert_eq!(
        Verified::new(
            pvf(),
            super::AuthorityReceipt {
                source: AuthoritySource::Pvf,
                digest: String::new(),
                subject_digest: pvf().subject_digest(),
            },
        ),
        Err(VerificationRejectReason::MissingReceipt)
    );

    let mut input = delivery_input(PublicationMode::Closing, "Closes #504", false);
    input.pull_request = verified(merged_pr(REVISION), AuthoritySource::PublicationIntent);
    assert_eq!(
        deliver(input),
        Err(RemoteDeliveryRejectReason::Verification(
            VerificationRejectReason::WrongSource
        ))
    );
}

#[test]
fn verified_receipt_must_match_the_exact_subject_value() {
    let original = pvf();
    let receipt = receipt(AuthoritySource::Pvf, &original.subject_digest());
    let forged = AcceptedPvfResult {
        revision: "attacker-revision".to_owned(),
        ..original
    };
    assert_eq!(
        Verified::new(forged, receipt),
        Err(VerificationRejectReason::MissingReceipt)
    );
}

#[test]
fn accepted_review_requires_typed_evidence_digest_and_scope() {
    let evidence = AcceptedReviewEvidence {
        issue: ISSUE,
        reviewed_revision: REVISION.to_owned(),
        scope_paths: vec!["csdlc-v3/src/commands/remote".to_owned()],
        implementer: "worker-6-implementation".to_owned(),
        reviewer: "independent-reviewer".to_owned(),
        findings: vec![],
        target: ReviewTarget {
            repository: REPOSITORY.to_owned(),
            issue: ISSUE,
            mode: PublicationMode::Closing,
        },
        typed_review_evidence_digest: String::new(),
    };
    assert_eq!(
        review_from_accepted_evidence(evidence),
        Err(VerificationRejectReason::MissingReceipt)
    );
}

#[test]
fn cleanup_constructor_rejects_self_made_git_directory_registration() {
    let id = NEXT_CLEANUP_ID.fetch_add(1, Ordering::SeqCst);
    let approved_worktree_parent = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    let path = approved_worktree_parent.join(format!(
        "remote-cleanup-forged-worktree-{}-{id}",
        std::process::id()
    ));
    let repository_root = approved_worktree_parent.join(format!(
        "remote-cleanup-forged-repo-{}-{id}",
        std::process::id()
    ));
    let self_made_git_dir = path.join(".git");
    fs::create_dir_all(&self_made_git_dir).expect("create self-made git dir fixture");
    fs::create_dir_all(repository_root.join(".git").join("worktrees"))
        .expect("create registration parent fixture");

    assert_eq!(
        cleanup_candidate_from_git_registration(
            &approved_worktree_parent,
            &path,
            GitWorktreeRegistration {
                repository_root,
                worktree_path: path.clone(),
                git_common_dir: self_made_git_dir,
            },
            true,
            false,
            true,
            true,
            None,
            false,
            false,
        ),
        Err(CleanupRejectReason::MissingRegistrationReceipt)
    );
}

#[derive(Debug)]
struct SequencedProcessAdapter {
    outputs: std::collections::VecDeque<crate::adapters::ProcessOutput>,
    invocations: Vec<crate::adapters::CommandInvocation>,
    intent_path: Option<PathBuf>,
}

impl SequencedProcessAdapter {
    fn new(outputs: Vec<crate::adapters::ProcessOutput>) -> Self {
        Self {
            outputs: outputs.into(),
            invocations: Vec::new(),
            intent_path: None,
        }
    }

    fn requiring_intent(mut self, intent_path: PathBuf) -> Self {
        self.intent_path = Some(intent_path);
        self
    }
}

impl crate::adapters::ProcessAdapter for SequencedProcessAdapter {
    fn run(
        &mut self,
        invocation: crate::adapters::CommandInvocation,
    ) -> crate::adapters::ProcessOutput {
        if invocation.program == super::GITHUB_OPERATIONAL_ADAPTER {
            assert!(
                self.intent_path.as_ref().is_some_and(|path| path.exists()),
                "durable intent must exist before external mutation"
            );
        }
        self.invocations.push(invocation);
        self.outputs.pop_front().expect("scripted process output")
    }
}

fn process_output(
    status: crate::adapters::ProcessStatus,
    value: serde_json::Value,
) -> crate::adapters::ProcessOutput {
    crate::adapters::ProcessOutput {
        status,
        stdout: value.to_string(),
        stderr: String::new(),
        truncated: false,
    }
}

fn mutation_repo(name: &str, exact_review_sha: &str, active: bool) -> PathBuf {
    let id = NEXT_CLEANUP_ID.fetch_add(1, Ordering::SeqCst);
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("target")
        .join(format!(
            "remote-mutation-{name}-{}-{id}",
            std::process::id()
        ));
    if root.exists() {
        fs::remove_dir_all(&root).expect("remove stale mutation fixture");
    }
    fs::create_dir_all(root.join(".git")).expect("git control fixture");
    let selector_path = root.join(super::CANONICAL_AUTHORITY_SELECTOR_PATH);
    fs::create_dir_all(selector_path.parent().expect("selector parent")).expect("selector parent");
    let selector = if active {
        serde_json::json!({
            "schema": "csdlc.generation_selector.v2",
            "default_generation": "v3",
            "operational_authority": "csdlc-v3",
            "authority_issue": 505,
            "exact_review_sha": exact_review_sha,
            "readiness_evidence_digest": "readiness-digest",
            "approval_evidence_digest": "approval-digest"
        })
    } else {
        serde_json::json!({
            "schema": "csdlc.generation_selector.v1",
            "default_generation": "v2",
            "opted_in_issues": []
        })
    };
    fs::write(
        selector_path,
        serde_json::to_vec_pretty(&selector).expect("selector JSON"),
    )
    .expect("write selector");
    root
}

fn mutation_request(mutation: super::GithubMutation) -> super::GithubMutationRequest {
    super::GithubMutationRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: None,
        cutover_issue: None,
        operator_approval: Some("caller-forged operator approval for #505".into()),
        expected_head_sha: REVISION.into(),
        credential_names: vec!["GITHUB_TOKEN".into()],
        mutation,
    }
}

#[test]
fn mutation_intent_precedes_dispatch_and_uncertain_comment_reconciles() {
    let root = mutation_repo("comment", REVISION, true);
    let request = mutation_request(super::GithubMutation::IssueComment {
        body: "durable comment".into(),
    });
    let operation_digest = super::github_mutation_operation_digest(&request);
    let marker = super::github_mutation_operation_marker(&operation_digest);
    let intent_path =
        super::github_mutation_intent_path(&root, &operation_digest).expect("intent path");
    let mut process = SequencedProcessAdapter::new(vec![
        process_output(
            crate::adapters::ProcessStatus::Cancelled,
            serde_json::json!({}),
        ),
        process_output(
            crate::adapters::ProcessStatus::Exit(0),
            serde_json::json!([{
                "id": 991,
                "body": format!("durable comment\n\n{marker}")
            }]),
        ),
    ])
    .requiring_intent(intent_path.clone());

    let result = super::execute_github_mutation(&root, &request, &mut process)
        .expect("uncertain response reconciles by authenticated marker readback");
    assert!(intent_path.exists());
    assert_eq!(process.invocations.len(), 2);
    assert_eq!(
        process.invocations[0].program,
        super::GITHUB_OPERATIONAL_ADAPTER
    );
    assert_eq!(
        process.invocations[1].program,
        super::GITHUB_READ_ONLY_ADAPTER
    );
    assert_eq!(
        result.receipt.response_digest,
        Some(super::stable_digest(&["{}"]))
    );
    assert_eq!(result.reconciliation.remote_object_id, Some(991));
    assert!(!result.receipt.idempotent_replay);
    assert!(
        super::github_mutation_receipt_path(&root, &operation_digest)
            .expect("receipt path")
            .exists()
    );
}

#[test]
fn restart_reconciles_pr_create_without_replaying_mutation() {
    let root = mutation_repo("pr-create", REVISION, true);
    let request = mutation_request(super::GithubMutation::PullRequestCreate {
        base: "main".into(),
        head: "codex/505".into(),
        title: "Issue 505".into(),
        body: "Closes #505".into(),
        draft: false,
    });
    let operation_digest = super::github_mutation_operation_digest(&request);
    let marker = super::github_mutation_operation_marker(&operation_digest);
    let intent_path =
        super::github_mutation_intent_path(&root, &operation_digest).expect("intent path");
    let mut first = SequencedProcessAdapter::new(vec![
        process_output(
            crate::adapters::ProcessStatus::TimedOut,
            serde_json::json!({}),
        ),
        process_output(
            crate::adapters::ProcessStatus::TimedOut,
            serde_json::json!({}),
        ),
    ])
    .requiring_intent(intent_path.clone());
    let finding = super::execute_github_mutation(&root, &request, &mut first)
        .expect_err("unavailable readback leaves durable uncertain intent");
    assert_eq!(finding.code, "github_mutation_reconciliation_pending");
    assert!(intent_path.exists());
    assert!(
        !super::github_mutation_receipt_path(&root, &operation_digest)
            .expect("receipt path")
            .exists()
    );

    let mut restart = SequencedProcessAdapter::new(vec![process_output(
        crate::adapters::ProcessStatus::Exit(0),
        serde_json::json!([{
            "number": 591,
            "head": {"sha": REVISION, "ref": "codex/505"},
            "base": {"ref": "main"},
            "title": "Issue 505",
            "body": format!("Closes #505\n\n{marker}"),
            "draft": false
        }]),
    )]);
    let result = super::execute_github_mutation(&root, &request, &mut restart)
        .expect("restart reconciles exact PR without mutation replay");
    assert_eq!(restart.invocations.len(), 1);
    assert_eq!(
        restart.invocations[0].program,
        super::GITHUB_READ_ONLY_ADAPTER
    );
    assert_eq!(result.receipt.pull_request, Some(591));
    assert!(result.receipt.idempotent_replay);
    assert_eq!(result.receipt.response_digest, None);
}

#[test]
fn reconciliation_matches_issue_edit_pr_update_and_ready_exact_state() {
    let issue_edit = mutation_request(super::GithubMutation::IssueEdit {
        title: Some("updated issue".into()),
        body: Some("updated body".into()),
    });
    let issue_digest = super::github_mutation_operation_digest(&issue_edit);
    let issue_marker = super::github_mutation_operation_marker(&issue_digest);
    assert_eq!(
        super::match_reconciled_mutation(
            &issue_edit,
            &issue_marker,
            &serde_json::json!({
                "number": 505,
                "title": "updated issue",
                "body": format!("updated body\n\n{issue_marker}")
            })
        ),
        Ok((None, None))
    );

    let mut update = mutation_request(super::GithubMutation::PullRequestUpdate {
        title: Some("updated PR".into()),
        body: None,
    });
    update.pull_request = Some(591);
    let update_digest = super::github_mutation_operation_digest(&update);
    let update_marker = super::github_mutation_operation_marker(&update_digest);
    assert_eq!(
        super::match_reconciled_mutation(
            &update,
            &update_marker,
            &serde_json::json!({
                "number": 591,
                "head": {"sha": REVISION},
                "title": "updated PR",
                "body": "unchanged",
                "draft": true
            })
        ),
        Ok((Some(591), Some(591)))
    );

    update.mutation = super::GithubMutation::PullRequestReady;
    assert_eq!(
        super::match_reconciled_mutation(
            &update,
            &super::github_mutation_operation_marker(&super::github_mutation_operation_digest(
                &update
            )),
            &serde_json::json!({
                "number": 591,
                "head": {"sha": REVISION},
                "draft": false
            })
        ),
        Ok((Some(591), Some(591)))
    );
}

#[test]
fn operational_dispatcher_fails_pre_cutover_and_serializes_review_result() {
    let blocked_root = mutation_repo("dispatcher-blocked", REVISION, false);
    let route_request = super::RemoteRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        pull_request: Some(591),
        actor: Some("worker-8".into()),
        implementer: Some("worker-8".into()),
        reviewer: Some("independent-reviewer".into()),
        review_revision: Some(REVISION.into()),
        expected_head_sha: Some(REVISION.into()),
        head_sha: Some(REVISION.into()),
        mode: Some(super::RemotePublicationMode::Closing),
        title: Some("Issue 505".into()),
        body: Some("Closes #505".into()),
        review_present: true,
        typed_review_receipt_path: None,
        typed_review_receipt_digest: None,
        readback_source: None,
        readback_receipt_path: None,
        readback_receipt_digest: None,
        adapter_receipt_path: None,
        adapter_receipt_digest: None,
        closes_issue: Some(505),
        closing_issues: vec![505],
        part_of_issue: None,
        credential_names: vec!["GITHUB_TOKEN".into()],
    };
    let blocked = super::OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: super::canonical_authority_selector_digest(&blocked_root)
            .expect("selector digest"),
        exact_review_sha: REVISION.into(),
        operation: super::OperationalRemoteOperation::Review(route_request.clone()),
    };
    let mut no_process = SequencedProcessAdapter::new(vec![]);
    assert_eq!(
        super::dispatch_operational_remote(&blocked_root, &blocked, &mut no_process)
            .expect_err("v2 selector fails closed")
            .code,
        "canonical_v3_authority_inactive"
    );

    let active_root = mutation_repo("dispatcher-active", REVISION, true);
    let dispatch = super::OperationalRemoteDispatchRequest {
        expected_lifecycle_digest: super::canonical_authority_selector_digest(&active_root)
            .expect("selector digest"),
        exact_review_sha: REVISION.into(),
        operation: super::OperationalRemoteOperation::Review(route_request),
    };
    let result = super::dispatch_operational_remote(&active_root, &dispatch, &mut no_process)
        .expect("active canonical selector dispatches review");
    let value = serde_json::to_value(result).expect("typed dispatcher result serializes");
    assert_eq!(value["schema"], "csdlc.v3.operational_remote_dispatch.v1");
    assert_eq!(value["authority"]["authority_issue"], 505);
    assert_eq!(value["outcome"]["kind"], "review");
}
