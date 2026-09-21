//! PVF lane: runtime. Role: deterministic update-cycle contract and activity
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
    },
    provider_communication::ProviderInvocationFinalStatusV1,
};
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
                "schema":OUTPUT_SCHEMA,"artifacts":[{"path":path,"kind":kind,"content":content,"evidence_paths":["src/lib.rs"],"limitations":["Proposal only"]}],
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
