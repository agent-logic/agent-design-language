//! PVF: required csdlc owner contract; deterministic local Git/fake transport,
//! small CPU/filesystem; no live merge or external credentials.
use super::*;
use crate::commands::remote::{GithubMutation, MergeMethod, TypedReviewReceipt};
use serde_json::{json, Value};

fn request(root: &std::path::Path) -> super::super::GithubMutationRequest {
    let head = mutation_head(root);
    let receipt = TypedReviewReceipt {
        publication_linkage: Some(super::super::PublicationLinkage {
            repository: "agent-logic/agent-design-language".into(),
            issue: 505,
            mode: super::super::RemotePublicationMode::Closing,
        }),
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
        "body":"Closes #505", "closingIssuesReferences":{"nodes":[{"number":505,"url":"https://github.com/agent-logic/agent-design-language/issues/505","repository":{"nameWithOwner":request.repository}}],"pageInfo":{"hasNextPage":false}},
        "number":844,"url":"https://github.com/agent-logic/agent-design-language/pull/844", "headRefOid":request.expected_head_sha,
        "baseRefName":"main","baseRefOid":"1111111111111111111111111111111111111111","state":if merged {"MERGED"} else {"OPEN"},
        "merged":merged,"isDraft":false,"mergeable":"MERGEABLE","mergeStateStatus":"CLEAN","reviewDecision":null,
        "baseRef":{"branchProtectionRule":null},"reviewThreads":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "latestReviews":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "commits":{"nodes":[{"commit":{"oid":request.expected_head_sha,"statusCheckRollup":{"state":"SUCCESS","contexts":checks}}}]},
        "mergeCommit":if merged {json!({"oid":"2222222222222222222222222222222222222222","parents":{"nodes":[{"oid":"1111111111111111111111111111111111111111"},{"oid":request.expected_head_sha}],"pageInfo":{"hasNextPage":false}}})} else {Value::Null}
    }},"linkedRepository":{"nameWithOwner":request.repository,"issue":{"number":505,"url":"https://github.com/agent-logic/agent-design-language/issues/505","state":if merged {"CLOSED"} else {"OPEN"}}}}})
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

fn reviewed_linkage(
    root: &std::path::Path,
    r: &mut super::super::GithubMutationRequest,
    repository: &str,
    mode: super::super::RemotePublicationMode,
) {
    if let GithubMutation::PullRequestMerge {
        review_receipt_path,
        review_receipt_digest,
        ..
    } = &mut r.mutation
    {
        let mut receipt: TypedReviewReceipt =
            serde_json::from_slice(&std::fs::read(&*review_receipt_path).unwrap()).unwrap();
        receipt.publication_linkage = Some(super::super::PublicationLinkage {
            repository: repository.into(),
            issue: r.issue,
            mode,
        });
        *review_receipt_digest = super::super::typed_review_receipt_payload_digest(&receipt);
        std::fs::write(
            root.join(&*review_receipt_path),
            serde_json::to_vec(&receipt).unwrap(),
        )
        .unwrap();
    }
}

fn linkage_state(
    r: &super::super::GithubMutationRequest,
    repository: &str,
    mode: super::super::RemotePublicationMode,
    merged: bool,
) -> Value {
    let mut value = state(r, merged);
    let closing = mode == super::super::RemotePublicationMode::Closing;
    let issue = json!({"number":r.issue,"url":format!("https://github.com/{repository}/issues/{}",r.issue),"repository":{"nameWithOwner":repository}});
    let pr = &mut value["data"]["repository"]["pullRequest"];
    pr["body"] = json!(format!(
        "{} {repository}#{}",
        if closing { "Closes" } else { "Part of" },
        r.issue
    ));
    pr["closingIssuesReferences"]["nodes"] = if closing { json!([issue]) } else { json!([]) };
    value["data"]["linkedRepository"] = json!({"nameWithOwner":repository,"issue":{"number":r.issue,"url":format!("https://github.com/{repository}/issues/{}",r.issue),"state":if closing && merged {"CLOSED"} else {"OPEN"}}});
    value
}

#[test]
fn merge_linkage_positive_modes_and_qualified_split_repository_replay() {
    for repository in ["agent-logic/agent-design-language", "agent-logic/planning"] {
        for mode in [
            super::super::RemotePublicationMode::Closing,
            super::super::RemotePublicationMode::PartOf,
        ] {
            let root = mutation_repo("merge-linkage-positive", true);
            let mut r = request(&root);
            reviewed_linkage(&root, &mut r, repository, mode);
            let before = linkage_state(&r, repository, mode, false);
            let after = linkage_state(&r, repository, mode, true);
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
            let identity = result.reconciliation.merge.unwrap();
            assert_eq!(identity.publication_linkage.repository, repository);
            assert_eq!(identity.publication_linkage.mode, mode);
            assert_eq!(
                identity.issue_state,
                if mode == super::super::RemotePublicationMode::Closing {
                    "CLOSED"
                } else {
                    "OPEN"
                }
            );
            let mut p = adapter(&root, &r, vec![out(after)]);
            assert!(
                super::super::execute_github_mutation(&root, &r, &mut p)
                    .unwrap()
                    .receipt
                    .idempotent_replay
            );
            assert_eq!(put_count(&p), 0);
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}

#[test]
fn merge_linkage_negative_matrix_rejects_before_intent_and_dispatch() {
    for case in [
        "missing",
        "mixed",
        "wrong_target",
        "wrong_repository",
        "duplicate",
        "closing_removed",
        "part_of_to_closing",
        "split_unqualified",
        "manual_other_close",
        "link_page",
        "missing_graph",
        "wrong_graph",
        "missing_issue",
        "wrong_issue_repo",
        "closed_parent",
    ] {
        let root = mutation_repo("merge-linkage-negative", true);
        let mut r = request(&root);
        let mode = if case == "part_of_to_closing" {
            super::super::RemotePublicationMode::PartOf
        } else {
            super::super::RemotePublicationMode::Closing
        };
        let repo = if case == "split_unqualified" {
            "agent-logic/planning"
        } else {
            "agent-logic/agent-design-language"
        };
        reviewed_linkage(&root, &mut r, repo, mode);
        let mut before = linkage_state(&r, repo, mode, false);
        let pr = &mut before["data"]["repository"]["pullRequest"];
        match case {
            "missing" => {
                pr.as_object_mut().unwrap().remove("body");
            }
            "mixed" => pr["body"] = json!("Closes #505\nPart of #505"),
            "wrong_target" => pr["body"] = json!("Closes #506"),
            "wrong_repository" => pr["body"] = json!("Closes other/repo#505"),
            "duplicate" => pr["body"] = json!("Closes #505\nCloses #505"),
            "closing_removed" => pr["body"] = json!("Related #505"),
            "part_of_to_closing" | "split_unqualified" => pr["body"] = json!("Closes #505"),
            "manual_other_close" => pr["closingIssuesReferences"]["nodes"]
                .as_array_mut()
                .unwrap()
                .push(json!({"number":506})),
            "link_page" => pr["closingIssuesReferences"]["pageInfo"]["hasNextPage"] = json!(true),
            "missing_graph" => {
                pr.as_object_mut()
                    .unwrap()
                    .remove("closingIssuesReferences");
            }
            "wrong_graph" => {
                pr["closingIssuesReferences"]["nodes"][0]["repository"]["nameWithOwner"] =
                    json!("other/repo")
            }
            "missing_issue" => before["data"]["linkedRepository"]["issue"] = Value::Null,
            "wrong_issue_repo" => {
                before["data"]["linkedRepository"]["nameWithOwner"] = json!("other/repo")
            }
            "closed_parent" => {
                before["data"]["linkedRepository"]["issue"]["state"] = json!("CLOSED")
            }
            _ => unreachable!(),
        }
        let mut p = adapter(&root, &r, vec![out(before), out(rules())]);
        assert!(
            super::super::execute_github_mutation(&root, &r, &mut p).is_err(),
            "{case}"
        );
        assert_eq!(put_count(&p), 0, "{case}");
        assert!(!p.intent_path.as_ref().unwrap().exists(), "{case}");
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn merge_linkage_same_head_drift_before_dispatch_and_after_uncertain_result() {
    for mode in [
        super::super::RemotePublicationMode::PartOf,
        super::super::RemotePublicationMode::Closing,
    ] {
        let root = mutation_repo("merge-linkage-drift", true);
        let mut r = request(&root);
        let repo = "agent-logic/agent-design-language";
        reviewed_linkage(&root, &mut r, repo, mode);
        let before = linkage_state(&r, repo, mode, false);
        let mut drift = before.clone();
        drift["data"]["repository"]["pullRequest"]["body"] =
            json!(if mode == super::super::RemotePublicationMode::PartOf {
                "Closes #505"
            } else {
                "Part of #505"
            });
        let mut p = adapter(
            &root,
            &r,
            vec![out(before.clone()), out(rules()), out(rules()), out(drift)],
        );
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert_eq!(put_count(&p), 0);
        assert!(p.intent_path.as_ref().unwrap().exists());
        // A retained intent is reconciliation-only even after pre-dispatch rejection.
        let mut p = adapter(&root, &r, vec![out(before)]);
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert_eq!(put_count(&p), 0);
        std::fs::remove_dir_all(root).unwrap();

        let root = mutation_repo("merge-linkage-poststate", true);
        let mut r = request(&root);
        reviewed_linkage(&root, &mut r, repo, mode);
        let before = linkage_state(&r, repo, mode, false);
        let mut after = linkage_state(&r, repo, mode, true);
        after["data"]["linkedRepository"]["issue"]["state"] =
            json!(if mode == super::super::RemotePublicationMode::PartOf {
                "CLOSED"
            } else {
                "OPEN"
            });
        let mut p = adapter(
            &root,
            &r,
            vec![
                out(before.clone()),
                out(rules()),
                out(rules()),
                out(before),
                process_output(crate::adapters::ProcessStatus::TimedOut, json!({})),
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
}

#[test]
fn merge_linkage_review_digest_prevents_missing_or_changed_review_linkage() {
    for change in ["missing", "mode", "repository", "issue"] {
        let root = mutation_repo("merge-linkage-review", true);
        let r = request(&root);
        if let GithubMutation::PullRequestMerge {
            review_receipt_path,
            ..
        } = &r.mutation
        {
            let mut receipt: TypedReviewReceipt =
                serde_json::from_slice(&std::fs::read(review_receipt_path).unwrap()).unwrap();
            match change {
                "missing" => receipt.publication_linkage = None,
                "mode" => {
                    receipt.publication_linkage.as_mut().unwrap().mode =
                        super::super::RemotePublicationMode::PartOf
                }
                "repository" => {
                    receipt.publication_linkage.as_mut().unwrap().repository = "other/repo".into()
                }
                "issue" => receipt.publication_linkage.as_mut().unwrap().issue += 1,
                _ => unreachable!(),
            }
            std::fs::write(review_receipt_path, serde_json::to_vec(&receipt).unwrap()).unwrap();
        }
        let mut p = adapter(&root, &r, vec![]);
        assert!(super::super::execute_github_mutation(&root, &r, &mut p).is_err());
        assert!(p.invocations.is_empty());
        std::fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn merge_linkage_url_part_of_directives_reject_before_intent_and_dispatch() {
    for mode in [
        super::super::RemotePublicationMode::PartOf,
        super::super::RemotePublicationMode::Closing,
    ] {
        for directive in ["Part of", "Part-of"] {
            for reference in [
                "https://github.com/other/repo/issues/506",
                "<https://github.com/other/repo/issues/506>",
                "[parent](https://github.com/other/repo/issues/506)",
                "[other parent](https://github.com/other/repo/issues/506)",
            ] {
                for canonical_present in [false, true] {
                    let root = mutation_repo("merge-url-part-of", true);
                    let mut r = request(&root);
                    let repository = "agent-logic/agent-design-language";
                    reviewed_linkage(&root, &mut r, repository, mode);
                    let mut before = linkage_state(&r, repository, mode, false);
                    let body = &mut before["data"]["repository"]["pullRequest"]["body"];
                    let extra = format!("{directive} {reference}");
                    *body = if canonical_present {
                        json!(format!("{}\n{extra}", body.as_str().unwrap()))
                    } else {
                        json!(extra)
                    };
                    let mut after = linkage_state(&r, repository, mode, true);
                    after["data"]["repository"]["pullRequest"]["body"] = body.clone();
                    let mut p = adapter(
                        &root,
                        &r,
                        vec![
                            out(before.clone()),
                            out(rules()),
                            out(rules()),
                            out(before),
                            out(
                                json!({"merged":true,"sha":"2222222222222222222222222222222222222222"}),
                            ),
                            out(after),
                        ],
                    );
                    assert!(
                        super::super::execute_github_mutation(&root, &r, &mut p).is_err(),
                        "{mode:?} {directive} {reference} canonical={canonical_present}"
                    );
                    assert_eq!(put_count(&p), 0);
                    assert!(!p.intent_path.as_ref().unwrap().exists());
                    std::fs::remove_dir_all(root).unwrap();
                }
            }
        }
    }
}
