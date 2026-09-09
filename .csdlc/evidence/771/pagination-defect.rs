
// PVF: deterministic local regression; synthetic transport; no credentials or network.
// Appended to commands/remote/tests.rs in an untracked scratch copy only.
#[test]
fn review771_comment_after_first_page_must_not_be_reposted() {
    let root = mutation_repo("pagination-regression", true);
    let head = mutation_head(&root);
    let mut request = mutation_request(&head, super::GithubMutation::IssueComment {
        body: "already posted synthetic comment".into(),
    });
    request.recovery = Some(super::GithubMutationRecovery::RetryAfterAuthenticatedAbsence);
    let operation_digest = super::github_mutation_operation_digest(&request);
    let marker = super::github_mutation_operation_marker(&operation_digest);
    let intent_path = persist_mutation_intent(&root, &request);
    // Model the remote collection, where the intended comment already exists at 101.
    let mut remote_comments = (1..=100).map(|id| serde_json::json!({
        "id": id, "body": format!("unrelated synthetic comment {id}")
    })).collect::<Vec<_>>();
    remote_comments.push(serde_json::json!({
        "id": 101, "body": format!("already posted synthetic comment\n\n{marker}")
    }));
    assert_eq!(remote_comments.len(), 101);
    let first_page = serde_json::Value::Array(remote_comments[..100].to_vec());
    // Production adapter requests only the first per_page=100 page. The fake
    // transport models that exact shape, then records an erroneous second POST.
    let mut process = SequencedProcessAdapter::new(vec![
        process_output(crate::adapters::ProcessStatus::Exit(0), first_page.clone()),
        process_output(crate::adapters::ProcessStatus::Exit(0), serde_json::json!({
            "id":102,"body":format!("already posted synthetic comment\n\n{marker}")
        })),
        process_output(crate::adapters::ProcessStatus::Exit(0), first_page),
    ]).requiring_intent(intent_path);
    let result = super::execute_github_mutation(&root, &request, &mut process);
    let writes = process.invocations.iter()
        .filter(|call| call.program == super::GITHUB_OPERATIONAL_ADAPTER).count();
    eprintln!("synthetic_remote_comments=101 existing_marker_comment_id=101 mutation_dispatches={writes}");
    eprintln!("result_code={}", result.as_ref().err().map(|f| f.code.as_str()).unwrap_or("success"));
    assert_eq!(writes, 0, "incomplete page cannot prove authenticated absence; existing comment must not be duplicated");
}
