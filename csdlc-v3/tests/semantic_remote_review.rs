//! PVF: deterministic local CPU-only contract tests; required #870 qualification input.
//! No transport, credentials or lifecycle effects. These tests prove native variant
//! scope and rejection only, not installed remote integration or authentication.
use csdlc_v3::commands::remote::{
    intent::{semantic_mutation_target, SemanticMutationTarget},
    GithubMutationRequest,
};
use csdlc_v3::lifecycle::semantic::SemanticCommand;
use serde_json::{json, Value};

fn request(issue: u64, pr: Option<u64>, mutation: Value) -> GithubMutationRequest {
    serde_json::from_value(json!({
        "repository":"example/repo", "issue":issue,"pull_request":pr,
        "expected_head_sha":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        "credential_names":["GITHUB_TOKEN"],"mutation":mutation
    }))
    .unwrap()
}

#[test]
fn all_native_variants_have_exact_semantic_scope_without_terminal_inference() {
    use SemanticCommand::*;
    let cases = [
        (
            0,
            None,
            json!({"action":"issue_create","title":"new","body":"body"}),
            SemanticMutationTarget::RepositoryCreation,
        ),
        (
            870,
            None,
            json!({"action":"issue_comment","body":"comment"}),
            SemanticMutationTarget::Issue(RecordIssueMutation),
        ),
        (
            870,
            None,
            json!({"action":"issue_edit","body":"new body"}),
            SemanticMutationTarget::Issue(RecordIssueMutation),
        ),
        (
            870,
            None,
            json!({"action":"issue_close","rationale":"duplicate","current_body":"body","disposition":"duplicate","duplicate_of":871}),
            SemanticMutationTarget::Issue(RecordIssueMutation),
        ),
        (
            870,
            None,
            json!({"action":"pull_request_create","base":"main","head":"codex/870-test","title":"change","body":"Closes #870","draft":true}),
            SemanticMutationTarget::Issue(Publish),
        ),
        (
            870,
            Some(966),
            json!({"action":"pull_request_update","body":"Closes #870"}),
            SemanticMutationTarget::Issue(Publish),
        ),
        (
            870,
            Some(966),
            json!({"action":"pull_request_ready"}),
            SemanticMutationTarget::Issue(MarkMergeReady),
        ),
        (
            870,
            Some(966),
            json!({"action":"pull_request_merge","base":"main","method":"merge","review_receipt_path":"receipt.json","review_receipt_digest":"digest"}),
            SemanticMutationTarget::Issue(RecordMerge),
        ),
    ];
    assert_eq!(cases.len(), 8);
    for (issue, pr, mutation, expected) in cases {
        assert_eq!(
            semantic_mutation_target(&request(issue, pr, mutation)).unwrap(),
            expected
        );
    }
}

#[test]
fn creation_cannot_borrow_calling_issue_and_existing_mutations_cannot_use_zero() {
    for (issue, pr, mutation) in [
        (
            870,
            None,
            json!({"action":"issue_create","title":"new","body":"body"}),
        ),
        (
            0,
            Some(966),
            json!({"action":"issue_create","title":"new","body":"body"}),
        ),
        (0, None, json!({"action":"issue_comment","body":"comment"})),
        (0, Some(966), json!({"action":"pull_request_ready"})),
        (870, None, json!({"action":"pull_request_ready"})),
    ] {
        assert!(semantic_mutation_target(&request(issue, pr, mutation)).is_err());
    }
}
