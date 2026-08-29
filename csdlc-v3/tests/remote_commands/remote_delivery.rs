use csdlc_v3::commands::remote::{
    accepted_review, deliver, AcceptedPvfResult, RemoteDeliveryInput, RemoteDeliveryRejectReason,
};
use csdlc_v3::publication::{
    classify_cleanup, derive_finish, publish, CleanupCandidate, CleanupClassification,
    CleanupRejectReason, FinishClassification, FinishRejectReason, IssueReadback, PublicationMode,
    PublicationRejectReason, PublicationRequest, PullRequestReadback,
};
use csdlc_v3::review::{
    authorize_publication, FindingDisposition, ReviewFinding, ReviewRejectReason, ReviewTarget,
};
use csdlc_v3::REMOTE_DELIVERY_PREDECESSORS;
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
    CleanupCandidate {
        preview: true,
        preview_receipt: false,
        committed_closed_out: true,
        terminal_receipt: true,
        registered_worktree: path.clone(),
        candidate_path: path,
        dirty: false,
        live: false,
    }
}

#[test]
fn v3e_denominator_is_exact() {
    assert_eq!(REMOTE_DELIVERY_PREDECESSORS, [174, 175, 176, 177, 178]);
    for issue in REMOTE_DELIVERY_PREDECESSORS {
        assert!(csdlc_v3::is_v3e_remote_delivery_predecessor(issue));
    }
    assert!(!csdlc_v3::is_v3e_remote_delivery_predecessor(173));
    assert!(!csdlc_v3::is_v3e_remote_delivery_predecessor(179));
}

#[test]
fn accepted_pvf_result_reaches_safe_cleanup_preview() {
    let cleanup_candidate = cleanup();
    let cleanup_path = cleanup_candidate.candidate_path.clone();
    let input = RemoteDeliveryInput {
        pvf: pvf(),
        review: accepted_review(
            ISSUE,
            REVISION,
            "worker-6-implementation",
            "independent-reviewer",
            PublicationMode::Closing,
        ),
        publication: publication(PublicationMode::Closing, "Closes #504"),
        pull_request: merged_pr(REVISION),
        issue: issue(false),
        cleanup: cleanup_candidate,
    };
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
        CleanupClassification::PreviewEligible { path: cleanup_path }
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
        derive_finish(&evidence, &merged_pr(REVISION), &issue(true)),
        FinishClassification::CheckpointCompleted {
            pull_request: 586,
            issue: ISSUE,
            invalidates_review_and_publication: true
        }
    );
    assert_eq!(
        derive_finish(&evidence, &merged_pr(REVISION), &issue(false)),
        FinishClassification::OperatorRequired {
            reason: FinishRejectReason::PartOfParentClosed
        }
    );
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
    assert_eq!(
        classify_cleanup(&remove),
        Ok(CleanupClassification::RemoveEligible { path: remove_path })
    );
    let mut attack = cleanup();
    let attack_path = attack.registered_worktree.with_extension("extra");
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
    unverified.registered_worktree = unverified.registered_worktree.join("missing-worktree");
    assert_eq!(
        classify_cleanup(&unverified),
        Err(CleanupRejectReason::PathMismatch)
    );
    let mut no_preview_receipt = cleanup();
    no_preview_receipt.preview = false;
    assert_eq!(
        classify_cleanup(&no_preview_receipt),
        Err(CleanupRejectReason::MissingPreviewReceipt)
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
    let mut input = RemoteDeliveryInput {
        pvf: pvf(),
        review: accepted_review(
            ISSUE,
            REVISION,
            "worker-6-implementation",
            "independent-reviewer",
            PublicationMode::Closing,
        ),
        publication: publication(PublicationMode::Closing, "Closes #504"),
        pull_request: merged_pr(REVISION),
        issue: issue(false),
        cleanup: cleanup(),
    };
    input.pvf.evidence_digest.clear();
    assert_eq!(
        deliver(input.clone()),
        Err(RemoteDeliveryRejectReason::PvfEvidenceMissing)
    );
    input.pvf.evidence_digest = "accepted-pvf-digest".to_owned();
    input.pvf.revision = "different".to_owned();
    assert_eq!(
        deliver(input),
        Err(RemoteDeliveryRejectReason::PvfRevisionMismatch)
    );
}
