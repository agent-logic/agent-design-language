//! Operational remote routing after canonical authority admission.

use std::path::Path;

use crate::adapters::ProcessAdapter;

use super::authority::verify_canonical_v3_authority;
use super::model::*;
use super::mutation::execute_github_mutation;
use super::publication::{
    load_remote_route_receipts, prepare_remote_publication_route_with_receipts,
};
use super::support::remote_finding;

#[cfg(unix)]
pub fn dispatch_operational_remote(
    repo_root: &Path,
    dispatch: &OperationalRemoteDispatchRequest,
    process: &mut impl ProcessAdapter,
) -> Result<OperationalRemoteDispatchResult, RemoteRouteFinding> {
    let authority = verify_canonical_v3_authority(
        repo_root,
        Some(&dispatch.expected_lifecycle_digest),
        &dispatch.exact_review_sha,
    )?;
    let outcome = match &dispatch.operation {
        OperationalRemoteOperation::Review(request) => {
            verify_transition_revision("review", request, &dispatch.exact_review_sha)?;
            let receipts = load_remote_route_receipts(repo_root, request)?;
            OperationalRemoteOutcome::Review(prepare_remote_publication_route_with_receipts(
                "review", request, &receipts,
            )?)
        }
        OperationalRemoteOperation::Publish(request) => {
            verify_transition_revision("publish", request, &dispatch.exact_review_sha)?;
            let receipts = load_remote_route_receipts(repo_root, request)?;
            OperationalRemoteOutcome::Publish(prepare_remote_publication_route_with_receipts(
                "publish", request, &receipts,
            )?)
        }
        OperationalRemoteOperation::GithubMutation(request) => {
            if request.expected_head_sha != dispatch.exact_review_sha {
                return Err(remote_finding(
                    "operational_remote_exact_review_mismatch",
                    "GitHub mutation must bind the canonical exact review SHA",
                ));
            }
            let result = execute_github_mutation(repo_root, request, process)?;
            OperationalRemoteOutcome::GithubMutation(Box::new(OperationalGithubMutationResult {
                performed_mutation: result.performed_mutation,
                receipt: result.receipt,
                reconciliation: result.reconciliation,
            }))
        }
    };
    Ok(OperationalRemoteDispatchResult {
        schema: "csdlc.v3.operational_remote_dispatch.v1".into(),
        authority,
        outcome,
    })
}

#[cfg(not(unix))]
pub fn dispatch_operational_remote(
    _repo_root: &Path,
    _dispatch: &OperationalRemoteDispatchRequest,
    _process: &mut impl ProcessAdapter,
) -> Result<OperationalRemoteDispatchResult, RemoteRouteFinding> {
    Err(remote_finding(
        "operational_remote_unsupported_platform",
        "native v3 operational remote mutation is supported only on Unix platforms",
    ))
}

fn verify_transition_revision(
    route: &str,
    request: &RemoteRouteRequest,
    exact_review_sha: &str,
) -> Result<(), RemoteRouteFinding> {
    if exact_review_sha.trim().is_empty()
        || request.review_revision.as_deref() != Some(exact_review_sha)
        || request.expected_head_sha.as_deref() != Some(exact_review_sha)
        || (route == "publish" && request.head_sha.as_deref() != Some(exact_review_sha))
    {
        return Err(remote_finding(
            "operational_remote_exact_review_mismatch",
            "operational review and publication transitions require the canonical exact review SHA",
        ));
    }
    Ok(())
}
