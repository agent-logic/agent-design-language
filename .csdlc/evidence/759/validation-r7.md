# Issue 759 validation r7

- Recorded at: `2026-09-09T17:31:55Z`
- Validation source head: `8a28f65a142ae701cbae4cdc70aeba4b1acb7321`
- Merged base: `origin/main` `38d5a360553f2b9a311ffff949c3d74b1c8a090d`
- Branch: `codex/759-dynamic-agent-health-task-failures`
- Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-759-dynamic-agent-health-task-failures`

## Current-head validation

All commands below ran from the bound #759 worktree at validation source head `8a28f65a142ae701cbae4cdc70aeba4b1acb7321`.

| Artifact | Command | Status | SHA-256 |
| --- | --- | --- | --- |
| `.csdlc/evidence/759/runtime-v3-fast-full-r7.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml` | `0` | `e6ad7163e54eb04be947a2ed4c66a97590731026ff5b520ca7fa9c9420b79f98` |
| `.csdlc/evidence/759/focused-dynamic-agent-health-r7.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml dynamic_agent_health_sweep_drains_after_task -- --nocapture` | `0` | `a68616425d9f9036289c271f83e81820b727ab3f483fa99e73b86bb82a54f8dc` |
| `.csdlc/evidence/759/focused-resident-shepherd-id-r7.log` | `cargo test --manifest-path adl-runtime-kernel/Cargo.toml resident_shepherd_construction_uses_configured_canonical_name_and_truthful_counts --test agent_roster -- --nocapture` | `0` | `8a37e0ec6235efc169c250bfa28b7121a4116d291aa9ca889049a5ed57cddef3` |
| `.csdlc/evidence/759/strict-clippy-r7.log` | `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings` | `0` | `f5fa0719f4e6c65d274a7b66e68866dacd902ab35e71da8c91627dd6d998ab0b` |
| `.csdlc/evidence/759/html-observatory-proof-r7.log` | `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh` | `0` | `9c635bbcd172b2283e4ec6137b4902e9b6f892b3832923cc687110c44ffecbc9` |
| `.csdlc/evidence/759/fmt-check-r7.log` | `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check` | `0` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `.csdlc/evidence/759/diff-check-r7.log` | `git diff --check HEAD` | `0` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

`fmt-check-r7.log` and `diff-check-r7.log` are zero-byte logs because the successful commands emitted no output.

## Hosted red closure

PR #779 run `34381946883` failed on an older synthetic merge in `adl-runtime-v3-fast`, asserting `left: "beacon"` and `right: "shepherd"` in `resident_shepherd_construction_uses_configured_canonical_name_and_truthful_counts`. The branch has now been resynced twice to current main, resolves the resident shepherd id contract to `resident_shepherd_runtime_id(index, config)`, and passes the exact hosted failing command locally at `8a28f65a142ae701cbae4cdc70aeba4b1acb7321`.
