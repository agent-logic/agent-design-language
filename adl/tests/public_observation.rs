//! PVF local_contract: retained path and occupancy observation compatibility.
//! Deterministic isolated fixtures; no lifecycle admission, provider or network.
use adl::control_plane::*;
use adl::session_ledger::*;
use chrono::{DateTime, Utc};
use std::{fs, path::Path};

fn now() -> DateTime<Utc> {
    DateTime::parse_from_rfc3339("2026-06-22T00:00:00Z")
        .unwrap()
        .with_timezone(&Utc)
}

fn input(issue: u64) -> ClaimInput {
    ClaimInput {
        session_id: "fixture-session".into(),
        owner: "fixture".into(),
        resource: ResourceRef {
            kind: "csdlc_issue".into(),
            id: issue.to_string(),
        },
        purpose: "isolated observation contract".into(),
        mode: ClaimMode::Active,
        lifecycle_phase: None,
        policy_ref: None,
        github: GithubRef {
            issue: Some(issue),
            pull_request: None,
            repository: None,
            last_state: None,
        },
        branch: Some(format!("codex/{issue}-example")),
        worktree_path: Some(format!(".worktrees/adl-wp-{issue}")),
        do_not_touch_paths: vec![],
        blockers: vec![],
        ttl_secs: DEFAULT_TTL_SECS,
    }
}

// PVF local_contract: pure path compatibility, deterministic and bounded;
// no Git, filesystem mutation, lifecycle authority or provider execution.
#[test]
fn complete_card_bundle_preserves_names_across_distinct_checkout_roots() {
    let issue = IssueRef::new(7, "release", "  Portable / Contracts  ").unwrap();
    assert_eq!(issue.issue_number(), 7);
    assert_eq!(issue.scope(), "release");
    assert_eq!(issue.slug(), "portable-contracts");
    let primary = Path::new("/primary");
    let worktree = Path::new("/execution");
    let primary_cards = [
        issue.task_bundle_input_path(primary),
        issue.task_bundle_stp_path(primary),
        issue.task_bundle_plan_path(primary),
        issue.task_bundle_validation_plan_path(primary),
        issue.task_bundle_review_policy_path(primary),
        issue.task_bundle_output_path(primary),
    ];
    let execution_cards = [
        issue.worktree_task_bundle_input_path(worktree),
        issue.worktree_task_bundle_stp_path(worktree),
        issue.worktree_task_bundle_plan_path(worktree),
        issue.worktree_task_bundle_validation_plan_path(worktree),
        issue.worktree_task_bundle_review_policy_path(worktree),
        issue.worktree_task_bundle_output_path(worktree),
    ];
    for ((primary_card, execution_card), filename) in primary_cards
        .iter()
        .zip(&execution_cards)
        .zip(["sip.md", "stp.md", "spp.md", "vpp.md", "srp.md", "sor.md"])
    {
        let relative =
            Path::new(".adl/release/tasks/issue-0007__portable-contracts").join(filename);
        assert_eq!(primary_card.strip_prefix(primary).unwrap(), relative);
        assert_eq!(execution_card.strip_prefix(worktree).unwrap(), relative);
        assert!(!execution_card.starts_with(primary));
    }
    assert_eq!(
        issue.legacy_issue_prompt_path(primary),
        Path::new("/primary/.adl/issues/release/bodies/issue-7-portable-contracts.md")
    );
    assert_eq!(
        card_validation_plan_path(Path::new("/cards"), 7),
        Path::new("/cards/7/vpp_7.md")
    );
}

#[test]
fn path_resolution_refuses_empty_identity_and_preserves_unknown_topology() {
    assert!(IssueRef::new(7, " ", "valid").is_err());
    assert!(IssueRef::new(7, "release", " / ").is_err());
    let current = Path::new("/checkout");
    assert_eq!(resolve_primary_checkout_root(current, None), current);
    assert_eq!(
        resolve_primary_checkout_root(current, Some(Path::new(".git"))),
        current
    );
    assert_eq!(
        resolve_primary_checkout_root(current, Some(Path::new("metadata/foreign"))),
        current
    );
}

// PVF local_contract: deterministic persisted occupancy observations in an
// isolated temporary directory; no lifecycle admission or external effects.
#[test]
fn persisted_assessment_preserves_watching_and_released_history_without_writes() {
    let directory = tempfile::tempdir().unwrap();
    let root = directory.path();
    let path = default_ledger_path(root);
    let mut ledger = SessionLedger::empty(now());
    let mut watching = input(4419);
    watching.mode = ClaimMode::Watching;
    let claim = ledger.claim(watching, now()).unwrap();
    save_ledger(&path, &ledger).unwrap();
    let observed_before = fs::read(&path).unwrap();
    let observer = ["other-session".to_string()];
    let worktree = root.join(".worktrees/adl-wp-4419");
    let assessment = load_target_claim_assessment(
        root,
        4419,
        "codex/4419-example",
        &worktree,
        &observer,
        now(),
    )
    .unwrap();
    assert_eq!(assessment.status, "WARN");
    assert_eq!(assessment.block_kind, "session_nonblocking_live_claim");
    assert_eq!(assessment.relevant_claims.len(), 1);
    assert_eq!(
        assessment.relevant_claims[0].classification,
        ClaimClassification::Watching
    );
    assert_eq!(fs::read(&path).unwrap(), observed_before);
    assert!(!path.with_file_name("ledger.json.lock").exists());

    ledger
        .release(&claim.claim_id, Some("handoff".into()), now())
        .unwrap();
    save_ledger(&path, &ledger).unwrap();
    let released_before = fs::read(&path).unwrap();
    let released = load_target_claim_assessment(
        root,
        4419,
        "codex/4419-example",
        &worktree,
        &observer,
        now(),
    )
    .unwrap();
    assert_eq!(released.status, "PASS");
    assert_eq!(released.block_kind, "session_released_history");
    assert_eq!(released.relevant_claims[0].claim_id, claim.claim_id);
    assert_eq!(
        released.relevant_claims[0].classification,
        ClaimClassification::Released
    );
    assert_eq!(fs::read(&path).unwrap(), released_before);
    assert!(!path.with_file_name("ledger.json.lock").exists());
}
