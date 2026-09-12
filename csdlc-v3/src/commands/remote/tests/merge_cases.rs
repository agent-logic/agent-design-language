//! PVF: required csdlc owner contract; deterministic local Git/fake transport,
//! small CPU/filesystem; no live merge or external credentials.
use super::*;
use crate::commands::remote::{GithubMutation, MergeMethod, TypedReviewReceipt};
use serde_json::{json, Value};

fn request(root: &std::path::Path) -> super::super::GithubMutationRequest {
    let head = mutation_head(root);
    let receipt = TypedReviewReceipt {
        schema: "csdlc.v3.typed_review_receipt.v1".into(),
        repository: "agent-logic/agent-design-language".into(),
        issue: 505,
        implementer: "implementer".into(),
        reviewer: "independent".into(),
        reviewed_revision: head.clone(),
        expected_head_sha: head.clone(),
        evidence_digest: "review-evidence".into(),
    };
    let path = root.join(".csdlc/evidence/505/merge-review.json");
    super::super::persist_json_create_new(&path, &receipt).unwrap();
    let mut request = mutation_request(
        &head,
        GithubMutation::PullRequestMerge {
            base: "main".into(),
            method: MergeMethod::Merge,
            review_receipt_path: path.to_string_lossy().into(),
            review_receipt_digest: super::super::typed_review_receipt_payload_digest(&receipt),
        },
    );
    request.pull_request = Some(844);
    request.operator_approval = Some("fixture explicit target merge authorization".into());
    request
}
fn state(request: &super::super::GithubMutationRequest, merged: bool) -> Value {
    let checks = json!({"nodes":[{"__typename":"CheckRun","name":"ci","status":"COMPLETED","conclusion":"SUCCESS","isRequired":true,"checkSuite":{"app":{"databaseId":42}}}],"pageInfo":{"hasNextPage":false}});
    json!({"data":{"repository":{"nameWithOwner":request.repository,"mergeCommitAllowed":true,"pullRequest":{
        "number":844,"url":"https://github.com/agent-logic/agent-design-language/pull/844", "headRefOid":request.expected_head_sha,
        "baseRefName":"main","baseRefOid":"1111111111111111111111111111111111111111","state":if merged {"MERGED"} else {"OPEN"},
        "merged":merged,"isDraft":false,"mergeable":"MERGEABLE","mergeStateStatus":"CLEAN","reviewDecision":null,
        "baseRef":{"branchProtectionRule":null},"reviewThreads":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "latestReviews":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "commits":{"nodes":[{"commit":{"oid":request.expected_head_sha,"statusCheckRollup":{"state":"SUCCESS","contexts":checks}}}]},
        "mergeCommit":if merged {json!({"oid":"2222222222222222222222222222222222222222","parents":{"nodes":[{"oid":"1111111111111111111111111111111111111111"},{"oid":request.expected_head_sha}],"pageInfo":{"hasNextPage":false}}})} else {Value::Null}
    }}}})
}
fn rules() -> Value {
    json!([{"type":"required_status_checks","parameters":{"required_status_checks":[{"context":"ci","integration_id":42}]}}])
}
fn out(value: Value) -> crate::adapters::ProcessOutput {
    process_output(crate::adapters::ProcessStatus::Exit(0), value)
}
fn adapter(
    root: &std::path::Path,
    r: &super::super::GithubMutationRequest,
    outputs: Vec<crate::adapters::ProcessOutput>,
) -> SequencedProcessAdapter {
    let mut adapter = SequencedProcessAdapter::new(outputs);
    adapter.intent_path = Some(root.join(".git/csdlc-v3/remote/merges").join(format!(
        "{}.intent.json",
        super::super::github_mutation_operation_digest(r)
    )));
    adapter
}
fn put_count(p: &SequencedProcessAdapter) -> usize {
    p.invocations
        .iter()
        .filter(|i| i.program == super::super::GITHUB_OPERATIONAL_ADAPTER)
        .count()
}

#[test]
fn merge_positive_binds_result_and_replays_without_second_mutation() {
    let root = mutation_repo("merge-positive", true);
    let r = request(&root);
    let before = state(&r, false);
    let after = state(&r, true);
    let mut p = adapter(
        &root,
        &r,
        vec![
            out(before.clone()),
            out(rules()),
            out(rules()),
            out(before),
            out(json!({"merged":true,"sha":"2222222222222222222222222222222222222222"})),
            out(after.clone()),
        ],
    );
    let result = super::super::execute_github_mutation(&root, &r, &mut p).unwrap();
    assert_eq!(put_count(&p), 1);
    assert!(!result.receipt.idempotent_replay);
    let merge = result.reconciliation.merge.unwrap();
    assert_eq!(merge.head_sha, r.expected_head_sha);
    assert_eq!(merge.base, "main");
    assert_eq!(
        merge.merge_commit,
        "2222222222222222222222222222222222222222"
    );
    let mut p = adapter(&root, &r, vec![out(after)]);
    let result = super::super::execute_github_mutation(&root, &r, &mut p).unwrap();
    assert!(result.receipt.idempotent_replay);
    assert_eq!(put_count(&p), 0);
    // SIM-01 observation must accept settled evidence, then fail closed for
    // missing/corrupt evidence without replaying this already completed merge.
    let observation = serde_json::from_value(json!({
        "repository": r.repository, "issue": r.issue, "pull_request": r.pull_request
    }))
    .unwrap();
    super::super::pending_mutation_finding(&root, &observation).unwrap();
    let digest = super::super::github_mutation_operation_digest(&r);
    let receipt_path = root.join(format!(".git/csdlc-v3/remote/mutations/{digest}.json"));
    let reconciliation_path = root.join(format!(
        ".git/csdlc-v3/remote/merges/{digest}.reconciliation.json"
    ));
    let receipt_bytes = fs::read(&receipt_path).unwrap();
    let reconciliation_bytes = fs::read(&reconciliation_path).unwrap();
    for case in ["wrong_pr", "missing", "corrupt", "changed_merge_identity"] {
        match case {
            "wrong_pr" => {
                let mut value: Value = serde_json::from_slice(&receipt_bytes).unwrap();
                value["pull_request"] = json!(845);
                fs::write(&receipt_path, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            "missing" => fs::remove_file(&reconciliation_path).unwrap(),
            "corrupt" => fs::write(&reconciliation_path, b"not-json").unwrap(),
            "changed_merge_identity" => {
                let mut value: Value = serde_json::from_slice(&reconciliation_bytes).unwrap();
                value["merge"]["merge_commit"] = json!("3333333333333333333333333333333333333333");
                fs::write(&reconciliation_path, serde_json::to_vec(&value).unwrap()).unwrap();
            }
            _ => unreachable!(),
        }
        let before = fs::read(&reconciliation_path).ok();
        assert!(
            super::super::pending_mutation_finding(&root, &observation).is_err(),
            "{case}"
        );
        assert_eq!(
            fs::read(&reconciliation_path).ok(),
            before,
            "observer repaired {case}"
        );
        fs::write(&receipt_path, &receipt_bytes).unwrap();
        fs::write(&reconciliation_path, &reconciliation_bytes).unwrap();
        super::super::pending_mutation_finding(&root, &observation).unwrap();
    }
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn merge_eligibility_negative_matrix_never_writes_intent_or_dispatches() {
    let cases = [
        "draft",
        "conflict",
        "stale_head",
        "wrong_base",
        "wrong_repo",
        "wrong_pr",
        "red",
        "missing_check",
        "wrong_app",
        "required_skipped",
        "partial_graphql",
        "thread_page",
        "check_page",
        "review_page",
        "unresolved_thread",
        "changes_requested",
        "review_required",
        "missing_decision",
        "missing_policy",
        "linear",
        "queue",
        "disabled_method",
        "unknown_rule",
        "rule_page",
        "wrong_commit",
    ];
    for case in cases {
        let root = mutation_repo(&format!("merge-{case}"), true);
        let r = request(&root);
        let mut v = state(&r, false);
        let mut policy = rules();
        let pr = &mut v["data"]["repository"]["pullRequest"];
        match case {
            "draft" => pr["isDraft"] = json!(true),
            "conflict" => pr["mergeable"] = json!("CONFLICTING"),
            "stale_head" => pr["headRefOid"] = json!("3333333333333333333333333333333333333333"),
            "wrong_base" => pr["baseRefName"] = json!("other"),
            "wrong_pr" => pr["number"] = json!(845),
            "wrong_repo" => v["data"]["repository"]["nameWithOwner"] = json!("other/repository"),
            "red" => {
                pr["commits"]["nodes"][0]["commit"]["statusCheckRollup"]["state"] = json!("FAILURE")
            }
            "missing_check" => {
                policy[0]["parameters"]["required_status_checks"][0]["context"] = json!("missing")
            }
            "wrong_app" => {
                policy[0]["parameters"]["required_status_checks"][0]["integration_id"] = json!(43)
            }
            "required_skipped" => {
                pr["commits"]["nodes"][0]["commit"]["statusCheckRollup"]["contexts"]["nodes"][0]
                    ["conclusion"] = json!("SKIPPED")
            }
            "partial_graphql" => v["errors"] = json!([{"message":"partial"}]),
            "thread_page" => pr["reviewThreads"]["pageInfo"]["hasNextPage"] = json!(true),
            "review_page" => pr["latestReviews"]["pageInfo"]["hasNextPage"] = json!(true),
            "check_page" => {
                pr["commits"]["nodes"][0]["commit"]["statusCheckRollup"]["contexts"]["pageInfo"]
                    ["hasNextPage"] = json!(true)
            }
            "unresolved_thread" => pr["reviewThreads"]["nodes"] = json!([{"isResolved":false}]),
            "changes_requested" => {
                pr["latestReviews"]["nodes"] = json!([{"state":"CHANGES_REQUESTED"}])
            }
            "review_required" => pr["reviewDecision"] = json!("REVIEW_REQUIRED"),
            "missing_decision" => {
                pr.as_object_mut().unwrap().remove("reviewDecision");
            }
            "missing_policy" => pr["baseRef"] = Value::Null,
            "linear" => {
                pr["baseRef"]["branchProtectionRule"] = json!({"requiresLinearHistory":true})
            }
            "queue" => policy = json!([{"type":"merge_queue"}]),
            "disabled_method" => v["data"]["repository"]["mergeCommitAllowed"] = json!(false),
            "unknown_rule" => policy = json!([{"type":"future_rule"}]),
            "rule_page" => policy = json!(vec![json!({"type":"deletion"}); 100]),
            "wrong_commit" => {
                pr["commits"]["nodes"][0]["commit"]["oid"] =
                    json!("3333333333333333333333333333333333333333")
            }
            _ => unreachable!(),
        }
        let mut p = adapter(&root, &r, vec![out(v), out(policy)]);
        assert!(
            super::super::execute_github_mutation(&root, &r, &mut p).is_err(),
            "{case}"
        );
        assert_eq!(put_count(&p), 0, "{case}");
        assert!(!p.intent_path.unwrap().exists(), "{case}");
        std::fs::remove_dir_all(root).unwrap();
    }
}
#[test]
fn merge_uncertain_transport_is_reconciliation_only() {
    let root = mutation_repo("merge-uncertain", true);
    let r = request(&root);
    let before = state(&r, false);
    let mut p = adapter(
        &root,
        &r,
        vec![
            out(before.clone()),
            out(rules()),
            out(rules()),
            out(before.clone()),
            process_output(crate::adapters::ProcessStatus::Exit(28), json!({})),
            out(before.clone()),
        ],
    );
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 1);
    let mut p = adapter(&root, &r, vec![out(before)]);
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 0);
    let mut p = adapter(&root, &r, vec![out(state(&r, true))]);
    assert!(
        super::super::execute_github_mutation(&root, &r, &mut p)
            .unwrap()
            .receipt
            .idempotent_replay
    );
    assert_eq!(put_count(&p), 0);
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn merge_already_merged_and_wrong_parent_or_result() {
    for bad in [false, true] {
        let root = mutation_repo(
            if bad {
                "merge-wrong-parent"
            } else {
                "merge-already"
            },
            true,
        );
        let r = request(&root);
        let mut v = state(&r, true);
        if bad {
            v["data"]["repository"]["pullRequest"]["mergeCommit"]["parents"]["nodes"][1]["oid"] =
                json!("3333333333333333333333333333333333333333");
        }
        let mut p = adapter(&root, &r, vec![out(v)]);
        let result = super::super::execute_github_mutation(&root, &r, &mut p);
        assert_eq!(result.is_err(), bad);
        assert_eq!(put_count(&p), 0);
        std::fs::remove_dir_all(root).unwrap();
    }
}
#[test]
fn merge_review_and_method_guards_before_network() {
    let root = mutation_repo("merge-review", true);
    let r = request(&root);
    for bad in ["review", "approval", "method", "recovery"] {
        let mut r = r.clone();
        match bad {
            "review" => {
                if let GithubMutation::PullRequestMerge {
                    review_receipt_digest,
                    ..
                } = &mut r.mutation
                {
                    *review_receipt_digest = "wrong".into()
                }
            }
            "approval" => r.operator_approval = None,
            "recovery" => {
                r.recovery =
                    Some(super::super::GithubMutationRecovery::RetryAfterAuthenticatedAbsence)
            }
            "method" => {
                let mut v = serde_json::to_value(&r).unwrap();
                v["mutation"]["method"] = json!("squash");
                assert!(serde_json::from_value::<super::super::GithubMutationRequest>(v).is_err());
                continue;
            }
            _ => unreachable!(),
        }
        let mut p = adapter(&root, &r, vec![]);
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert!(p.invocations.is_empty());
    }
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn merge_checks_base_again_and_rejects_postmerge_parent_drift() {
    for late in [false, true] {
        let root = mutation_repo(
            if late {
                "merge-late-base"
            } else {
                "merge-early-base"
            },
            true,
        );
        let r = request(&root);
        let before = state(&r, false);
        let mut changed = state(&r, late);
        if late {
            changed["data"]["repository"]["pullRequest"]["mergeCommit"]["parents"]["nodes"][0]
                ["oid"] = json!("3333333333333333333333333333333333333333");
        } else {
            changed["data"]["repository"]["pullRequest"]["baseRefOid"] =
                json!("3333333333333333333333333333333333333333");
        }
        let mut outputs = vec![
            out(before.clone()),
            out(rules()),
            out(rules()),
            out(if late { before } else { changed.clone() }),
        ];
        if late {
            outputs.extend([
                out(json!({"merged":true,"sha":"2222222222222222222222222222222222222222"})),
                out(changed),
            ]);
        }
        let mut p = adapter(&root, &r, outputs);
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert_eq!(put_count(&p), usize::from(late));
        std::fs::remove_dir_all(root).unwrap();
    }
}
#[test]
fn merge_lock_blocks_concurrent_dispatch() {
    use fs2::FileExt;
    let root = mutation_repo("merge-lock", true);
    let r = request(&root);
    let dir = root.join(".git/csdlc-v3/remote/merges");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!(
        "{}.lock",
        super::super::stable_digest(&[&r.repository, "844"])
    ));
    let lock = std::fs::File::create(path).unwrap();
    lock.lock_exclusive().unwrap();
    let mut p = adapter(&root, &r, vec![]);
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert!(p.invocations.is_empty());
    drop(lock);
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn merge_result_is_consumed_by_existing_finish_observation_without_mutation() {
    use crate::commands::terminal::{
        prepare_terminal_finish_with_github_observation, TerminalPublicationMode,
        TerminalRouteRequest, TerminalRouteStatus,
    };
    let root = mutation_repo("merge-finish", true);
    let r = request(&root);
    let mut p = adapter(&root, &r, vec![out(state(&r, true))]);
    let result = super::super::execute_github_mutation(&root, &r, &mut p).unwrap();
    let identity = result.reconciliation.merge.unwrap();
    let request = TerminalRouteRequest {
        repository: identity.repository.clone(),
        issue: r.issue,
        pull_request: Some(identity.pull_request),
        expected_head_sha: Some(identity.head_sha.clone()),
        mode: Some(TerminalPublicationMode::Closing),
        public_adapter_receipt: None,
        terminal_state: None,
        no_pr_closeout: None,
        cleanup: None,
        cutover: None,
        credential_names: vec!["GITHUB_TOKEN".into()],
    };
    let mut p = adapter(
        &root,
        &r,
        vec![
            out(
                json!({"number":identity.pull_request,"head":{"sha":identity.head_sha},"merged":true,"merge_commit_sha":identity.merge_commit,"body":"Closes #505"}),
            ),
            out(json!({"number":505,"state":"closed"})),
        ],
    );
    let plan = prepare_terminal_finish_with_github_observation(&request, &mut p).unwrap();
    assert_eq!(plan.status, TerminalRouteStatus::Ready);
    assert_eq!(put_count(&p), 0);
    assert_eq!(p.invocations.len(), 2);
    std::fs::remove_dir_all(root).unwrap();
}

#[test]
fn merge_response_disagreement_stays_blocked_on_replay() {
    let root = mutation_repo("merge-response-drift", true);
    let r = request(&root);
    let before = state(&r, false);
    let after = state(&r, true);
    let mut p = adapter(
        &root,
        &r,
        vec![
            out(before.clone()),
            out(rules()),
            out(rules()),
            out(before),
            out(json!({"merged":true,"sha":"3333333333333333333333333333333333333333"})),
            out(after.clone()),
        ],
    );
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 1);
    let mut p = adapter(&root, &r, vec![out(after)]);
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 0);
    std::fs::remove_dir_all(root).unwrap();
}
#[test]
fn merge_partial_response_reconciles_from_authenticated_state() {
    for truncated in [true, false] {
        let root = mutation_repo(
            if truncated {
                "merge-truncated"
            } else {
                "merge-malformed"
            },
            true,
        );
        let r = request(&root);
        let before = state(&r, false);
        let mut response = out(json!({}));
        response.stdout = "not-json".into();
        response.truncated = truncated;
        let mut p = adapter(
            &root,
            &r,
            vec![
                out(before.clone()),
                out(rules()),
                out(rules()),
                out(before),
                response,
                out(state(&r, true)),
            ],
        );
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_ok());
        assert_eq!(put_count(&p), 1);
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn merge_same_principal_and_malformed_review_policy_rejected() {
    for reviewer in ["implementer", "IMPLEMENTER", " implementer "] {
        let root = mutation_repo("merge-same-reviewer", true);
        let mut r = request(&root);
        if let GithubMutation::PullRequestMerge {
            review_receipt_path,
            review_receipt_digest,
            ..
        } = &mut r.mutation
        {
            let path = std::path::Path::new(review_receipt_path);
            let mut receipt: TypedReviewReceipt =
                serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
            receipt.reviewer = reviewer.into();
            std::fs::write(path, serde_json::to_vec(&receipt).unwrap()).unwrap();
            *review_receipt_digest = super::super::typed_review_receipt_payload_digest(&receipt);
        }
        let mut p = adapter(&root, &r, vec![]);
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert!(p.invocations.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
    for field in ["require_code_owner_review", "require_last_push_approval"] {
        for value in [None, Some(Value::Null), Some(json!("false"))] {
            let root = mutation_repo("merge-malformed-policy", true);
            let r = request(&root);
            let mut params = json!({"required_approving_review_count":0,"require_code_owner_review":false,"require_last_push_approval":false});
            if let Some(value) = value {
                params[field] = value;
            } else {
                params.as_object_mut().unwrap().remove(field);
            }
            let mut p = adapter(
                &root,
                &r,
                vec![
                    out(state(&r, false)),
                    out(json!([{"type":"pull_request","parameters":params}])),
                ],
            );
            assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
            assert_eq!(put_count(&p), 0);
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}
#[test]
fn merge_alternate_review_path_cannot_bypass_uncertain_target_guard() {
    let root = mutation_repo("merge-path-alias", true);
    let mut r = request(&root);
    let before = state(&r, false);
    let mut p = adapter(
        &root,
        &r,
        vec![
            out(before.clone()),
            out(rules()),
            out(rules()),
            out(before.clone()),
            process_output(crate::adapters::ProcessStatus::Exit(28), json!({})),
            out(before),
        ],
    );
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 1);
    if let GithubMutation::PullRequestMerge {
        review_receipt_path,
        ..
    } = &mut r.mutation
    {
        *review_receipt_path = ".csdlc/evidence/505/merge-review.json".into();
    }
    let mut p = adapter(&root, &r, vec![]);
    assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
    assert_eq!(put_count(&p), 0);
    assert!(p.invocations.is_empty());
    std::fs::remove_dir_all(root).unwrap();
}
