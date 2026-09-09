# Issue 759 validation r6

- Recorded at: `2026-09-09T17:27:14Z`
- Validation source head: `4b9ef9d35241aeed575846a550fcd1a1a07a2c32`
- Branch: `codex/759-dynamic-agent-health-task-failures`
- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-759-dynamic-agent-health-task-failures`

## Hosted red classification

- PR: `https://github.com/agent-logic/agent-design-language/pull/779`
- Red run: `34381946883`
- Failed job: `adl-runtime-v3-fast` / `102568819659`
- Failed step: `Runtime v3 focused tests`
- Failed argv: `cargo test --manifest-path adl-runtime-kernel/Cargo.toml`
- Hosted failure: `adl-runtime-kernel/tests/agent_roster.rs:118`, `left: "beacon"`, `right: "shepherd"`
- Classification: current-main ancestry drift. GitHub tested synthetic merge `dbc941d20050afdbc597985da544cd0f10b88196`, merging PR head `2711d2ae7a405d7521b39827959ee993a3ec7599` into base `6fc5553797b0be6c715063076487042234ac1e28`; current main changed the test contract back to stable runtime id `shepherd` while branch production still emitted configured prefix `beacon`.
- Remediation: resynced to current `origin/main` `a09dcd40fca4a2714282eba05b114cd929eba826` and resolved `adl-runtime-kernel/src/control/feeds.rs` to use `resident_shepherd_runtime_id(index, config)`, preserving stable runtime id `shepherd` and canonical/display config projection.

## Current-head validation

All commands below ran from the bound #759 worktree at validation source head `4b9ef9d35241aeed575846a550fcd1a1a07a2c32`.

| Artifact | Command | Status | SHA-256 |
| --- | --- | --- | --- |
| `.csdlc/evidence/759/runtime-v3-fast-full-r6.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml` | `0` | `2c1cab6dd0010f2deaa282b4d7bd115301d00142b02fd1f98c2be26e6341b805` |
| `.csdlc/evidence/759/focused-dynamic-agent-health-r6.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml dynamic_agent_health_sweep_drains_after_task -- --nocapture` | `0` | `995ca5aefed9df16b0c6f952a24c1cc0703280c65f561d77ef04561117cc2950` |
| `.csdlc/evidence/759/focused-resident-shepherd-id-r6.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml resident_shepherd_construction_uses_configured_canonical_name_and_truthful_counts --test agent_roster -- --nocapture` | `0` | `8a82917bbde8af6e902d7e073fae807eeaff03965c14cdfff0a8ce338549c7be` |
| `.csdlc/evidence/759/strict-clippy-r6.log` | `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings` | `0` | `94fe537e33530b5ecdca38e01bbbc444bfb66916c39095aa6fe1b78fb394c636` |
| `.csdlc/evidence/759/html-observatory-proof-r6.log` | `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh` | `0` | `b015f1fb79ce8970a2dd8d1a31d7c308491b8a95cd32c4bed51c25b984aa6402` |
| `.csdlc/evidence/759/fmt-check-r6.log` | `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check` | `0` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `.csdlc/evidence/759/diff-check-r6.log` | `git diff --check HEAD` | `0` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

`fmt-check-r6.log` and `diff-check-r6.log` are zero-byte logs because the successful commands emitted no output.

## Diagnostic reproduction logs

| Artifact | Purpose | Status | SHA-256 |
| --- | --- | --- | --- |
| `.csdlc/evidence/759/reproduce-resident-shepherd-ci-red-r6.log` | Confirmed the focused resident-shepherd test passed on PR head before the current-main resync, proving the hosted red came from synthetic-merge/base drift rather than branch-head test bytes. | `0` | `b625dc393ee8b028789d7f7e321e2a3f5882a817e78e12366297f309e02496d2` |
| `.csdlc/evidence/759/reproduce-runtime-v3-fast-full-r6.log` | Confirmed the broad runtime command passed locally before the final base resync, supporting the synthetic-merge drift diagnosis. | `0` | `08df912cdb271fd679f77496b540f677d595c3ae8a468084738c28d36c5c05e0` |
