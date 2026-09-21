//! PVF: fixtures/codefriend/update-cycle/PVF.json. Lane: runtime. Role: deterministic update-cycle contract and activity
//! orchestration proof. Local Git/files only; no provider, deployment, source
//! mutation, publication, or rendered-diagram proof.
use adl::{
    codefriend::{
        activities::{
            run_with_executor, Activity, ActivityStatus, TestingGoal, TestingMode, UpdateCyclePlan,
            OUTPUT_SCHEMA, PLAN_SCHEMA, RESULT_SCHEMA,
        },
        evidence::{Admission, Retention},
        ingestion::{local, Scope},
        review::runner::{self, ExecutionOptions, LaneExecution},
    },
    provider_communication::ProviderInvocationFinalStatusV1,
};
use serde_json::json;
use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(args)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().into()
}

fn admission() -> (PathBuf, Admission) {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/codefriend-update-cycle-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
    let repo = root.join("repo");
    fs::create_dir_all(repo.join("src")).unwrap();
    git(&repo, &["init"]);
    git(
        &repo,
        &["remote", "add", "origin", "https://example.com/team/repo"],
    );
    fs::write(repo.join("src/lib.rs"), "pub fn answer() -> u32 { 42 }\n").unwrap();
    fs::write(repo.join("README.md"), "# Fixture\n").unwrap();
    git(&repo, &["add", "."]);
    git(
        &repo,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ],
    );
    let revision = git(&repo, &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &repo,
        "https://example.com/team/repo",
        &revision,
        Scope {
            analysis: vec!["src/lib.rs".into()],
            context: vec!["README.md".into()],
            max_files: 2,
            max_bytes: 8192,
            max_file_bytes: 4096,
        },
    )
    .unwrap();
    let admission = Admission::new(packet, Retention { seconds: 3600 }, 100).unwrap();
    (root, admission)
}

fn admission_with_prompt_omission() -> (PathBuf, Admission) {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("target/codefriend-update-cycle-prompt-boundary-tests")
        .join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
    let repo = root.join("repo");
    fs::create_dir_all(repo.join("src")).unwrap();
    git(&repo, &["init"]);
    git(
        &repo,
        &["remote", "add", "origin", "https://example.com/team/repo"],
    );
    fs::write(repo.join("src/large.rs"), "a".repeat(90 * 1024)).unwrap();
    fs::write(repo.join("src/omitted.rs"), "b".repeat(20 * 1024)).unwrap();
    git(&repo, &["add", "."]);
    git(
        &repo,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "fixture",
        ],
    );
    let revision = git(&repo, &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &repo,
        "https://example.com/team/repo",
        &revision,
        Scope {
            analysis: vec!["src/large.rs".into(), "src/omitted.rs".into()],
            context: vec![],
            max_files: 2,
            max_bytes: 128 * 1024,
            max_file_bytes: 100 * 1024,
        },
    )
    .unwrap();
    let admission = Admission::new(packet, Retention { seconds: 3600 }, 100).unwrap();
    (root, admission)
}

fn admission_with_privacy_omission() -> (PathBuf, Admission) {
    let (root, _) = admission();
    let repo = root.join("repo");
    fs::write(
        repo.join("private.txt"),
        "password = \"private-fixture-value\"",
    )
    .unwrap();
    git(&repo, &["add", "."]);
    git(
        &repo,
        &[
            "-c",
            "user.name=fixture",
            "-c",
            "user.email=fixture@example.com",
            "commit",
            "-m",
            "privacy fixture",
        ],
    );
    let revision = git(&repo, &["rev-parse", "HEAD"]);
    let packet = local::acquire(
        &repo,
        "https://example.com/team/repo",
        &revision,
        Scope {
            analysis: vec!["src/lib.rs".into()],
            context: vec!["private.txt".into()],
            max_files: 2,
            max_bytes: 8192,
            max_file_bytes: 4096,
        },
    )
    .unwrap();
    assert_eq!(packet.completeness, "partial");
    let admission = Admission::new(packet, Retention { seconds: 3600 }, 100).unwrap();
    (root, admission)
}

fn plan(activities: Vec<Activity>, testing: Option<TestingGoal>) -> UpdateCyclePlan {
    UpdateCyclePlan {
        schema: PLAN_SCHEMA.into(),
        repository: "https://example.com/team/repo".into(),
        activities,
        testing,
    }
}

#[test]
fn activity_selection_is_canonical_independent_and_testing_goal_is_conditional() {
    let (root, admission) = admission();
    plan(
        vec![Activity::Documentation, Activity::Tests],
        Some(TestingGoal {
            mode: TestingMode::Target,
            target: Some(82),
        }),
    )
    .validate(&admission)
    .unwrap();
    assert!(plan(
        vec![Activity::Tests, Activity::Documentation],
        Some(TestingGoal {
            mode: TestingMode::Target,
            target: Some(82)
        })
    )
    .validate(&admission)
    .is_err());
    assert!(plan(
        vec![Activity::Documentation],
        Some(TestingGoal {
            mode: TestingMode::CloseGaps,
            target: None
        })
    )
    .validate(&admission)
    .is_err());
    assert!(plan(
        vec![Activity::Tests],
        Some(TestingGoal {
            mode: TestingMode::Target,
            target: None
        })
    )
    .validate(&admission)
    .is_err());
    assert!(plan(
        vec![Activity::Tests],
        Some(TestingGoal {
            mode: TestingMode::ChangedCode,
            target: Some(80)
        })
    )
    .validate(&admission)
    .is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn selected_activities_only_produce_bound_proposals_without_coverage_claims() {
    let (root, admission) = admission();
    let mut called = Vec::new();
    let result=run_with_executor(
        plan(vec![Activity::Documentation,Activity::Diagrams,Activity::Tests],Some(TestingGoal{mode:TestingMode::Target,target:Some(80)})),
        admission,"cycle-1".into(),"provider:fixture:model-v1".into(),None,
        |activity,prompt,manifest| {
            called.push(activity); assert!(prompt.contains("UNTRUSTED_SOURCE=")); assert_eq!(manifest.activity,activity);
            let (path,kind,content)=match activity {
                Activity::Documentation=>("docs/guide.md","documentation","# Guide\nGrounded behavior."),
                Activity::Diagrams=>("docs/system.mmd","mermaid_diagram","flowchart LR\nA --> B"),
                Activity::Tests=>("tests/answer.rs","test","assert_eq!(answer(), 42);"),
                Activity::Review=>unreachable!(),
            };
            Ok(adl::codefriend::activities::ProviderOutput{final_status:ProviderInvocationFinalStatusV1::Ok,output_text:Some(serde_json::json!({
                "schema":OUTPUT_SCHEMA,"artifacts":[{"path":path,"kind":kind,"disposition":"create","content":content,"evidence_paths":["src/lib.rs"],"unsupported_claims":[],"limitations":["Proposal only"],"render_manifest":if activity == Activity::Diagrams { Some(json!({"schema":"codefriend.mermaid_render_manifest.v1","source_path":path,"output_path":"docs/system.svg","format":"svg","renderer":"mmdc"})) } else { None }}],
                "gaps":[{"category":activity.id(),"title":"Bounded gap","rationale":"The admitted source leaves this behavior undocumented.","evidence_paths":["src/lib.rs"],"limitations":["Scoped evidence only"]}],
                "measured_coverage_percent":null
            }).to_string())})
        }
    ).unwrap();
    assert_eq!(
        called,
        vec![Activity::Documentation, Activity::Diagrams, Activity::Tests]
    );
    assert_eq!(result.schema, RESULT_SCHEMA);
    assert!(result.failures.is_empty());
    assert!(result.review.is_none());
    assert!(result
        .activities
        .iter()
        .all(|item| item.status == ActivityStatus::Complete && item.output_digest.is_some()));
    assert_eq!(
        result.activities[2]
            .input_manifest
            .testing
            .as_ref()
            .unwrap()
            .target,
        Some(80)
    );
    assert!(result.activities[..2]
        .iter()
        .all(|item| item.input_manifest.testing.is_none()));
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn generated_artifacts_and_gaps_require_admitted_source_evidence() {
    let (root, admission) = admission();
    for empty_artifact in [true, false] {
        let result = run_with_executor(
            plan(vec![Activity::Documentation], None),
            admission.clone(),
            "cycle-evidence".into(),
            "provider:fixture:model-v1".into(),
            None,
            |_, _, _| {
                Ok(adl::codefriend::activities::ProviderOutput {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some(
                        serde_json::json!({
                            "schema":OUTPUT_SCHEMA,
                            "artifacts":[{"path":"docs/guide.md","kind":"documentation","disposition":"update","content":"# Guide","evidence_paths":if empty_artifact { vec![] } else { vec!["src/lib.rs"] },"unsupported_claims":[],"limitations":[],"render_manifest":null}],
                            "gaps":[{"category":"documentation","title":"Gap","rationale":"Bounded gap","evidence_paths":if empty_artifact { vec!["src/lib.rs"] } else { vec![] },"limitations":[]}],
                            "measured_coverage_percent":null
                        })
                        .to_string(),
                    ),
                })
            },
        )
        .unwrap();
        assert_eq!(
            result.completion,
            adl::codefriend::evidence::contracts::Completion::Failed
        );
        assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn prompt_subset_limits_citations_but_preserves_full_path_disposition_truth() {
    let (root, omitted_admission) = admission_with_prompt_omission();
    assert!(omitted_admission
        .evidence
        .iter()
        .any(|item| item.path == "src/omitted.rs"));
    let result = run_with_executor(
        plan(vec![Activity::Documentation], None),
        omitted_admission,
        "cycle-prompt-boundary".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, manifest| {
            assert!(!manifest
                .evidence
                .iter()
                .any(|item| item.path == "src/omitted.rs"));
            Ok(adl::codefriend::activities::ProviderOutput {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(
                    json!({
                        "schema":OUTPUT_SCHEMA,
                        "artifacts":[{"path":"docs/guide.md","kind":"documentation","disposition":"create","content":"# Guide","evidence_paths":["src/omitted.rs"],"unsupported_claims":[],"limitations":[],"render_manifest":null}],
                        "gaps":[],
                        "measured_coverage_percent":null
                    })
                    .to_string(),
                ),
            })
        },
    )
    .unwrap();
    assert_eq!(
        result.completion,
        adl::codefriend::evidence::contracts::Completion::Failed
    );
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    fs::remove_dir_all(root).unwrap();

    let (root, existing_admission) = admission_with_prompt_omission();
    let result = run_with_executor(
        plan(vec![Activity::Documentation], None),
        existing_admission,
        "cycle-doc-prompt-omitted-existing".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, manifest| {
            assert!(!manifest.evidence.iter().any(|item| item.path == "src/omitted.rs"));
            assert!(manifest
                .known_repository_paths
                .iter()
                .any(|path| path == "src/omitted.rs"));
            Ok(adl::codefriend::activities::ProviderOutput {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(json!({
                    "schema":OUTPUT_SCHEMA,
                    "artifacts":[{"path":"src/omitted.rs","kind":"documentation","disposition":"create","content":"# Existing","evidence_paths":["src/large.rs"],"unsupported_claims":[],"limitations":["proposal only"],"render_manifest":null}],
                    "gaps":[],
                    "measured_coverage_percent":null
                }).to_string()),
            })
        },
    )
    .unwrap();
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    fs::remove_dir_all(root).unwrap();

    for (limitations, expected) in [
        (vec!["proposal only"], ActivityStatus::Failed),
        (
            vec!["path existence outside admitted packet is unverified"],
            ActivityStatus::Complete,
        ),
    ] {
        let (root, admission) = admission();
        let result = run_with_executor(
            plan(vec![Activity::Documentation], None),
            admission,
            "cycle-doc-unknown-update".into(),
            "provider:fixture:model-v1".into(),
            None,
            |_, _, _| {
                Ok(adl::codefriend::activities::ProviderOutput {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some(json!({
                        "schema":OUTPUT_SCHEMA,
                        "artifacts":[{"path":"docs/unknown.md","kind":"documentation","disposition":"update","content":"# Update","evidence_paths":["src/lib.rs"],"unsupported_claims":[],"limitations":limitations,"render_manifest":null}],
                        "gaps":[],
                        "measured_coverage_percent":null
                    }).to_string()),
                })
            },
        )
        .unwrap();
        assert_eq!(result.activities[0].status, expected);
        fs::remove_dir_all(root).unwrap();
    }
}

#[test]
fn diagrams_require_mermaid_syntax_and_an_exact_render_manifest() {
    let (root, admission) = admission();
    for (content, render_manifest) in [
        (
            "hello",
            Some(
                json!({"schema":"codefriend.mermaid_render_manifest.v1","source_path":"docs/system.mmd","output_path":"docs/system.svg","format":"svg","renderer":"mmdc"}),
            ),
        ),
        (
            "flowchart garbage",
            Some(
                json!({"schema":"codefriend.mermaid_render_manifest.v1","source_path":"docs/system.mmd","output_path":"docs/system.svg","format":"svg","renderer":"mmdc"}),
            ),
        ),
        ("flowchart LR\nA-->B", None),
        (
            "flowchart LR\nA-->B",
            Some(
                json!({"schema":"codefriend.mermaid_render_manifest.v1","source_path":"docs/system.mmd","output_path":"docs/wrong.svg","format":"svg","renderer":"mmdc"}),
            ),
        ),
    ] {
        let result = run_with_executor(
            plan(vec![Activity::Diagrams], None),
            admission.clone(),
            "cycle-diagram-contract".into(),
            "provider:fixture:model-v1".into(),
            None,
            |_, _, _| {
                Ok(adl::codefriend::activities::ProviderOutput {
                    final_status: ProviderInvocationFinalStatusV1::Ok,
                    output_text: Some(
                        json!({
                            "schema":OUTPUT_SCHEMA,
                            "artifacts":[{"path":"docs/system.mmd","kind":"mermaid_diagram","disposition":"create","content":content,"evidence_paths":["src/lib.rs"],"unsupported_claims":[],"limitations":[],"render_manifest":render_manifest}],
                            "gaps":[],
                            "measured_coverage_percent":null
                        })
                        .to_string(),
                    ),
                })
            },
        )
        .unwrap();
        assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    }
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn documentation_proposals_require_create_or_update_classification() {
    let (root, missing_admission) = admission();
    let result = run_with_executor(
        plan(vec![Activity::Documentation], None),
        missing_admission,
        "cycle-doc-disposition".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, _| {
            Ok(adl::codefriend::activities::ProviderOutput {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(json!({
                    "schema":OUTPUT_SCHEMA,
                    "artifacts":[{"path":"docs/guide.md","kind":"documentation","content":"# Guide","evidence_paths":["src/lib.rs"],"unsupported_claims":[],"limitations":[],"render_manifest":null}],
                    "gaps":[],
                    "measured_coverage_percent":null
                }).to_string()),
            })
        },
    )
    .unwrap();
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    fs::remove_dir_all(root).unwrap();

    let (root, admission) = admission();
    let result = run_with_executor(
        plan(vec![Activity::Documentation], None),
        admission,
        "cycle-doc-contradictory-disposition".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, _| {
            Ok(adl::codefriend::activities::ProviderOutput {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(json!({
                    "schema":OUTPUT_SCHEMA,
                    "artifacts":[{"path":"src/lib.rs","kind":"documentation","disposition":"create","content":"# Replacement","evidence_paths":["src/lib.rs"],"unsupported_claims":[],"limitations":["proposal only"],"render_manifest":null}],
                    "gaps":[],
                    "measured_coverage_percent":null
                }).to_string()),
            })
        },
    )
    .unwrap();
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn aggregate_revalidates_embedded_review_content_beyond_its_digest() {
    let (root, admission) = admission();
    let route = "provider:fixture:model-v1";
    let mut review = runner::run_with_executor(
        ExecutionOptions {
            out: root.join("review"),
            run_id: "cycle-review".into(),
            cancel_file: None,
        },
        admission.clone(),
        route.into(),
        |_, _, _| {
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"findings\":[]}".into()),
            })
        },
    )
    .unwrap();
    let resolved_route = "provider:fixture:resolved-model-v2";
    review.rebind_provider_route(resolved_route).unwrap();
    let mut result = run_with_executor(
        plan(vec![Activity::Review], None),
        admission,
        "cycle-review".into(),
        resolved_route.into(),
        Some(review),
        |_, _, _| unreachable!(),
    )
    .unwrap();
    result.review.as_mut().unwrap().lane_results[0].provider_route = "forged:route".into();
    result.activities[0].review_result_digest =
        Some(adl::codefriend::evidence::hash(result.review.as_ref().unwrap()).unwrap());
    assert!(result.validate(resolved_route).is_err());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn review_activity_accepts_successful_partial_review_with_explicit_coverage() {
    let (root, admission) = admission_with_privacy_omission();
    let route = "provider:fixture:model-v1";
    let mut review = runner::run_with_executor(
        ExecutionOptions {
            out: root.join("partial-review"),
            run_id: "cycle-partial-review".into(),
            cancel_file: None,
        },
        admission.clone(),
        route.into(),
        |_, prompt, _| {
            assert!(!prompt.contains("private-fixture-value"));
            Ok(LaneExecution {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"findings\":[]}".into()),
            })
        },
    )
    .unwrap();
    assert_eq!(
        review.completion,
        adl::codefriend::evidence::contracts::Completion::Incomplete
    );
    let resolved_route = "provider:fixture:resolved-model-v2";
    review.rebind_provider_route(resolved_route).unwrap();
    let result = run_with_executor(
        plan(vec![Activity::Review], None),
        admission,
        "cycle-partial-review".into(),
        resolved_route.into(),
        Some(review),
        |_, _, _| unreachable!(),
    )
    .unwrap();
    assert_eq!(result.activities[0].status, ActivityStatus::Complete);
    assert!(result.failures.is_empty());
    result.validate(resolved_route).unwrap();
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn invalid_or_failed_activity_output_is_explicit_and_does_not_dispatch_unselected_work() {
    let (root, admission) = admission();
    let mut calls = 0;
    let result = run_with_executor(
        plan(vec![Activity::Documentation], None),
        admission,
        "cycle-2".into(),
        "provider:fixture:model-v1".into(),
        None,
        |activity, _, _| {
            calls += 1;
            assert_eq!(activity, Activity::Documentation);
            Ok(adl::codefriend::activities::ProviderOutput {
                final_status: ProviderInvocationFinalStatusV1::Ok,
                output_text: Some("{\"schema\":\"wrong\"}".into()),
            })
        },
    )
    .unwrap();
    assert_eq!(calls, 1);
    assert_eq!(result.activities.len(), 1);
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    assert_eq!(
        result.activities[0].failure.as_deref(),
        Some("activity_execution_failed")
    );
    assert_eq!(result.failures, vec!["documentation_failed"]);
    assert!(result.activities[0].output.is_none());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn uncertain_provider_effect_is_not_downgraded_to_an_activity_failure() {
    let (root, admission) = admission();
    let error = run_with_executor(
        plan(vec![Activity::Documentation], None),
        admission,
        "cycle-uncertain".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, _| Err(adl::provider_adapter::CodeFriendProviderInterrupted.into()),
    )
    .unwrap_err();
    assert!(error.is::<adl::provider_adapter::CodeFriendProviderInterrupted>());
    fs::remove_dir_all(root).unwrap();
}

#[test]
fn review_selection_cannot_claim_completion_without_the_bound_review_result() {
    let (root, admission) = admission();
    let mut called = false;
    let result = run_with_executor(
        plan(vec![Activity::Review], None),
        admission,
        "cycle-3".into(),
        "provider:fixture:model-v1".into(),
        None,
        |_, _, _| {
            called = true;
            unreachable!()
        },
    )
    .unwrap();
    assert!(!called);
    assert_eq!(result.activities[0].status, ActivityStatus::Failed);
    assert_eq!(result.failures, vec!["review_failed"]);
    fs::remove_dir_all(root).unwrap();
}
