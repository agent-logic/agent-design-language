# Post-finalize remediation validation

Validated the exact current worktree after binding finish to the canonical
per-issue authority lock and preserving decisive exact-head review state across
later comment-only reviews.

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --lib --test gate_finish --test gate7_lifecycle --test gate10a --test gate10b --test gate_github_actions`
  passed: 168 tests, 0 failed after the independent-review remediation.
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`
  passed with no warnings.

No network credentials or AWS resources were used.

After PR publication, current `main` advanced through #5781 and exposed a
stable-rustfmt failure in the synthetic merge tree. The separately tracked
#5783 repair was folded into this already-open integration path after typed
review recovery and claim-scope amendment:

- `cd adl && cargo +stable fmt --all -- --check` passed on the exact merged tree.
- `cargo test --manifest-path ../adl-runtime/Cargo.toml runtime_api_contract_advertises_only_served_routes --locked`
  passed: 1 focused test, 0 failed.

The remediation requires a fixed GitHub approval label for no-PR closure,
re-observes PR and issue state after merge, binds cache authority to the exact
canonical record, bounds mutable terminal freshness, serializes the full finish
attempt under the canonical issue authority lock, and reduces review state to
each reviewer's latest decisive exact-head review.
