# Post-Merge Closeout Watcher Proof (#4713)

## Summary

`#4713` removes the Rust `pr finish` skip path that emitted `rust_closeout_no_background_watcher` when no post-merge closeout shell helper was configured.

The default Rust path now creates a durable post-merge closeout watcher packet and launches the configured Codex closeout watcher command. The explicit compatibility hook `ADL_POST_MERGE_CLOSEOUT_CMD` remains supported for operators who intentionally provide a helper command.

## Changed Surfaces

- `adl/src/cli/pr_cmd/github.rs`
  - keeps `ADL_POST_MERGE_CLOSEOUT_DISABLE=1` as the explicit opt-out
  - keeps non-empty `ADL_POST_MERGE_CLOSEOUT_CMD` as a compatibility command override
  - treats missing or blank `ADL_POST_MERGE_CLOSEOUT_CMD` as the Rust-owned default path
  - writes watcher artifacts under `.adl/logs/post-merge-closeout/issue-<issue>/`
  - launches `codex exec` by default, or `ADL_POST_MERGE_CLOSEOUT_CODEX_CMD` when set
- `adl/src/cli/pr_cmd/github/tests/helpers.rs`
  - proves disabled, helper, failure, and default fallback behavior
- `adl/src/cli/tests/pr_cmd_inline/finish/publication/closeout.rs`
  - proves the default watcher packet and durable log behavior
- `docs/tooling/ISSUE_LIFECYCLE_SHEPHERD_CONTRACT.md`
  - documents post-merge closeout watcher packets as retained wait-state evidence

## Durable Artifact Shape

The Rust default path writes:

```text
.adl/logs/post-merge-closeout/issue-<issue>/input.yaml
.adl/logs/post-merge-closeout/issue-<issue>/prompt.md
.adl/logs/post-merge-closeout/issue-<issue>/codex.log
.adl/logs/post-merge-closeout/issue-<issue>/last_message.md
.adl/logs/post-merge-closeout/issue-<issue>/pid
```

The input packet uses `post_merge_closeout_watcher.v1` and records issue, PR, branch, repo, and closeout policy.

## Validation

Passed locally in the issue worktree:

```bash
ADL_RUST_WARM_CACHE_SOURCE_TARGET=/Users/daniel/git/agent-design-language/adl/target \
ADL_RUST_WARM_CACHE_DEST_TARGET=/Users/daniel/git/agent-design-language/.worktrees/adl-wp-4713/adl/target \
ADL_RUST_WARM_CACHE_MANIFEST_PATH=/Users/daniel/git/agent-design-language/.worktrees/adl-wp-4713/adl/Cargo.toml \
bash adl/tools/rust_validation_warm_cache.sh

cargo test --manifest-path adl/Cargo.toml --bin adl attach_post_merge_closeout -- --nocapture
cargo test --manifest-path adl/Cargo.toml --bin adl helper_attach_commands_cover_disabled_success_failure_and_fallback_paths -- --nocapture
cargo fmt --manifest-path adl/Cargo.toml --all --check
git diff --check
```

## Non-Claims

- This does not merge PRs automatically.
- This does not close issues without the normal closeout path.
- This does not make watcher output authoritative over human review or merge authority.
- This does not remove the explicit helper-command compatibility hook.

## Pre-PR Review Finding And Disposition

A bounded subagent review found a P1 risk in the initial implementation: the default Codex watcher command inherited GitHub token environment variables and wrote raw child stdout/stderr to durable `codex.log`.

Disposition: fixed before PR. The default Rust-owned watcher path now launches through a no-secret helper command that removes `GITHUB_TOKEN`, `GH_TOKEN`, `ADL_GITHUB_TOKEN_FILE`, `ADL_GITHUB_TOKEN_KEYCHAIN_SERVICE`, and `ADL_GITHUB_TOKEN_KEYCHAIN_ACCOUNT` from the child environment. Focused tests set fake token values and prove they are not retained in the durable watcher log. Explicit non-empty `ADL_POST_MERGE_CLOSEOUT_CMD` compatibility overrides still receive the GitHub context because they are operator-provided helper commands and keep the previous failure redaction behavior.
