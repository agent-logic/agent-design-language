# v0.91.7 WP-03 Closeout Settled-State Proof (#4950)

Status: implemented

## Scope

Issue `#4950` fixes the lifecycle watcher/shepherd false-positive state where a
closed/completed issue with already-validated closeout truth could continue to
report as `merged_needs_closeout` or `closeout_needed`.

## Implementation Summary

- `pr closeout` now writes a durable local closeout-complete marker after the
  existing canonical closed-issue truth validation succeeds.
- `pr watch` treats closed/completed issues as `settled` when either:
  - the closeout-complete marker is current, points at valid canonical SOR
    truth, and for linked-PR issues the canonical SOR names the current merged
    PR with a marker `validated_at` timestamp at or after the issue's current
    GitHub `closedAt` value; or
  - legacy canonical closeout truth is already valid for an issue closed before
    durable markers existed and, for linked-PR issues, names the current merged
    PR through an explicit `pr_url` / PR URL field whose value matches the
    current linked PR URL.
- For linked-PR issues closed after the marker rollout cutoff, markerless
  closeout truth must carry a terminal SOR timestamp at or after the current
  GitHub `closedAt` value. This keeps old issues such as `#4630` settled
  without letting future reopen/reclose cycles inherit stale closeout truth.
- `pr shepherd` maps `settled` to inactive lifecycle state:
  - `active: false`
  - `owner_skill: none`
  - `next_skill: none`
  - `closeout_required: false`

## Validation

Focused validation was run from the `#4950` worktree using the warm Rust target
cache already present in the issue worktree.

Commands:

```bash
ADL_RUST_WARM_CACHE_SOURCE_TARGET=<primary-checkout>/adl/target \
ADL_RUST_WARM_CACHE_DEST_TARGET=<issue-worktree>/adl/target \
ADL_RUST_WARM_CACHE_MANIFEST_PATH=<issue-worktree>/adl/Cargo.toml \
bash adl/tools/rust_validation_warm_cache.sh

cargo fmt --manifest-path adl/Cargo.toml --all -- --check

cargo test --manifest-path adl/Cargo.toml \
  issue_watch_routes_validated_closeout_to_settled -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  closeout_closed_completed_issue_bundle_records_prune_result_on_canonical_output -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_accepts_legacy_canonical_truth_without_marker -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_ignores_corrupt_marker_when_canonical_truth_is_valid -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_matches_current_linked_pr -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_rejects_stale_linked_pr_context -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_rejects_incidental_pull_reference_without_pr_url -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_rejects_wrong_repo_pr_url_with_same_number -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_rejects_same_pr_stale_reclose_epoch -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  validated_closeout_state_rejects_markerless_post_rollout_stale_reclose_epoch -- --nocapture

cargo test --manifest-path adl/Cargo.toml \
  versioned_bootstrap_bundle_from_issue_prompt_includes_valid_six_cards_without_template_residue -- --nocapture

ADL_GITHUB_TOKEN_FILE=<approved-token-file> \
  adl/target/debug/adl pr watch 4630 --version v0.91.7 --json

ADL_GITHUB_TOKEN_FILE=<approved-token-file> \
  adl/target/debug/adl pr shepherd 4630 --version v0.91.7 --json

git diff --check
```

Results:

- Rust target warm-cache reuse: PASS.
- Rust formatting check: PASS.
- Watcher settled-state regression: PASS.
- Lifecycle closeout marker regression: PASS.
- Legacy canonical closeout without marker regression: PASS.
- Corrupt-marker canonical fallback regression: PASS.
- Linked-PR closeout binding regression: PASS.
- Stale linked-PR closeout rejection regression: PASS.
- Incidental `/pull/<n>` reference rejection regression: PASS.
- Same-number wrong-repo `pr_url` rejection regression: PASS.
- Same-PR stale reclose epoch rejection regression: PASS.
- Markerless post-rollout stale reclose epoch rejection regression: PASS.
- Versioned bootstrap SOR template-residue regression: PASS.
- Live `#4630` watch after rebuild: PASS, `classification: settled`,
  `tail_owner: none`, `next_skill: none`.
- Live `#4630` shepherd after rebuild: PASS, `active: false`,
  `state: settled`, `closeout_required: false`.
- Diff whitespace check: PASS.

## Residual Notes

The focused `cargo test` filter still invokes many binary test harnesses because
the package has many binary targets. That is validation topology cost, not a
failure of the `#4950` behavior.

The first finish attempt exposed a stale bootstrap-card assertion that treated
current `Start Time` / `End Time` template fields as residue. That assertion was
corrected to reject unresolved `<start_time>` / `<end_time>` placeholders
instead, preserving the intended no-template-residue invariant.

Subagent review found that marker-only validation could mask a reopened issue
that later closed through a different PR. The implementation now requires
linked-PR closeout truth to match the current merged PR URL/number before watch
or shepherd can report `settled`.

Subagent re-review then found the same issue could reclose through the same PR,
and that generic `/pull/<n>` text matching was too weak. The implementation now
records marker `validated_at` for new closeouts and requires that marker
timestamp to be at or after the current GitHub issue `closedAt` value for
linked-PR closeout settlement. Markerless legacy closeout records are accepted
only for pre-rollout close epochs with explicit `pr_url` / PR URL closeout
facts that exactly match the current linked PR URL, or for post-rollout records
whose terminal SOR timestamp covers the current `closedAt` epoch.
