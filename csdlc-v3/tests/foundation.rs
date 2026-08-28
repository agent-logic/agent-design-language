use csdlc_v3::application::{FoundationState, FOUNDATION_PREDECESSORS, ISSUE_START_MINUTES_MAX};
use csdlc_v3::repository::RepositoryContext;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn state() -> FoundationState {
    let context = RepositoryContext::discover(repo_root()).expect("explicit repository context");
    FoundationState::load(&context).expect("foundation state loads")
}

#[test]
fn repository_context_is_explicit() {
    let root = repo_root();
    let context = RepositoryContext::discover(&root).expect("explicit repository context");
    assert_eq!(
        context.root(),
        root.canonicalize().expect("canonical repository root")
    );
    assert_eq!(
        context.relative_display(context.contract_path()),
        "docs/csdlc-v3/CONTRACT.md"
    );
    assert_eq!(
        context.relative_display(context.predecessor_coverage_path()),
        "docs/csdlc-v3/predecessor-coverage.json"
    );
    assert_eq!(
        context.relative_display(context.proportional_lifecycle_path()),
        "docs/csdlc-v3/proportional-lifecycle.json"
    );
}

#[test]
fn projection_replay_is_deterministic() {
    let first = state();
    let second = state();
    assert_eq!(first.projections(), second.projections());
    assert_eq!(first.to_machine_json(), second.to_machine_json());
    let keys = first
        .projections()
        .into_iter()
        .map(|projection| projection.key)
        .collect::<Vec<_>>();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
}

#[test]
fn retained_requirements_164_through_167_are_bound() {
    let state = state();
    assert_eq!(FOUNDATION_PREDECESSORS, [164, 165, 166, 167]);
    assert_eq!(state.foundation_predecessors(), [164, 165, 166, 167]);
    assert_eq!(state.operational_authority(), "csdlc-v2");
}

#[test]
fn issue_start_projection_preserves_three_minute_budget_without_authority_cutover() {
    let state = state();
    assert_eq!(ISSUE_START_MINUTES_MAX, 3);
    assert_eq!(state.issue_start_minutes_max(), 3);
    let json = state.to_machine_json();
    assert!(json.contains("\"read_only\":true"));
    assert!(json.contains("\"operational_authority\":\"csdlc-v2\""));
    assert!(json.contains("\"key\":\"issue_start_minutes_max\",\"value\":\"3\""));
}
