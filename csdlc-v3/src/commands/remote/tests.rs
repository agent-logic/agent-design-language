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
