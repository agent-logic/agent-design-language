# Issue #759 validation r15

Issue: #759
Branch: `codex/759-dynamic-agent-health-task-failures`
Worktree: `/Volumes/FastWork/adl-worktrees/adl-issue-759-dynamic-agent-health-task-failures`
Source SHA validated: `23a5b82e6a353eaf4963357f72b536939430cab5`
Current `origin/main` at validation start: `11a3fe88e0bc30e46a9fd3a7ff39f01807e3477f`
Status: PASS after current-main resync, reviewer P2 remediation, and timing-proof hardening.

## Review recovery

- Review `/root/review_759_a80db54` returned P2 because r12 truth claimed the SNTP startup timeout was hardened, but the code had changed the unrelated shutdown-grace wrapper timeout.
- Remediation: restored `shutdown_grace_aborts_non_cooperative_operation_executor` to its 1s outer timeout and moved the 3s outer timeout to the intended `unavailable_sntp_does_not_block_kernel_startup_or_erase_bootstrap_time` startup wait.
- Additional narrow remediation: widened only the live Vector S3 outage proof deadline from 20s to 45s after repeated broad-run evidence showed master-log progress succeeded while sink failure telemetry arrived too late for the previous test window. Vector retry/batch/production configuration semantics are unchanged.

## Hosted red classification

- PR #779 run `34381946883` failed at stale remote head `2711d2ae7a405d7521b39827959ee993a3ec7599` in `adl-runtime-v3-fast` job `102568819659`.
- Exact hosted failure: `adl-runtime-kernel/tests/agent_roster.rs:118`, expected stable resident shepherd runtime id `shepherd` but observed `beacon` on GitHub's synthetic merge.
- Classification: current-main ancestry drift after main restored the stable resident-shepherd runtime id contract. The branch was resynced through current `origin/main`; production now uses the current-main `resident_shepherd_runtime_id` helper while preserving configured canonical/display projection.

## Local diagnostic red classification

- `runtime-v3-fast-full-r8.log` failed `pinned_vector_s3_archive_outage_emits_sink_failure_while_master_log_progresses`; isolated rerun `observability-s3-outage-isolated-r8.log` passed. Classification: pre-existing scheduler/timing-sensitive Vector proof, not #759 source behavior.
- `runtime-v3-fast-full-r8-rerun.log` failed `unavailable_sntp_does_not_block_kernel_startup_or_erase_bootstrap_time`; isolated rerun `assembly-sntp-isolated-red-r8.log` passed. Classification: scheduler/timing-sensitive startup budget in a broad run; remediated by moving the 3s timeout to the actual SNTP startup test.
- `runtime-v3-fast-full-r10.log` failed `runtime_guardian::tests::clean_shutdown_terminates_descendant_spawned_by_successful_child`; isolated rerun `parity-b-guardian-clean-shutdown-isolated-r10.log` passed. Classification: pre-existing scheduler/timing-sensitive Guardian proof, not #759 source behavior.
- `runtime-v3-fast-full-r14.log` failed the same Vector S3 outage proof before the r15 Vector deadline fix. Classification: repeated scheduler/timing-sensitive proof-window failure; remediated by widening the test deadline only.

## Current clean proof

| Command | Status | Evidence | SHA-256 |
| --- | --- | --- | --- |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml` | PASS | `.csdlc/evidence/759/runtime-v3-fast-full-r15.log` | `d9a320ed5ba676c75a159a9e5f787ddbc6b5569d8fb17b0970d96ee5cdbcb2f1` |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml dynamic_agent_health_sweep_drains_after_task -- --nocapture` | PASS | `.csdlc/evidence/759/focused-dynamic-agent-health-r15.log` | `9fa11f3e5684d3aa03e0ba3a4508bae85663ee5532d9107677cf5611e1f09b81` |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml resident_shepherd_construction_uses_configured_canonical_name_and_truthful_counts --test agent_roster -- --nocapture` | PASS | `.csdlc/evidence/759/focused-resident-shepherd-id-r15.log` | `6301a58c83553770dc429c2e3ec9277bfbb2e5310777c284988ea0e65ed1e859` |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml unavailable_sntp_does_not_block_kernel_startup_or_erase_bootstrap_time --test assembly -- --nocapture` | PASS | `.csdlc/evidence/759/assembly-sntp-isolated-r15.log` | `c5183c7de849ca73aeca0fdc522c51fdce13854c6587e73548e49ae28f0f5921` |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml shutdown_grace_aborts_non_cooperative_operation_executor --test assembly -- --nocapture` | PASS | `.csdlc/evidence/759/assembly-shutdown-grace-isolated-r15.log` | `f3e492e6ec856e18d0b3fed5f232dd2155e7186dc4a7737dee5392a0ae31d79c` |
| `cargo test --manifest-path adl-runtime-kernel/Cargo.toml pinned_vector_s3_archive_outage_emits_sink_failure_while_master_log_progresses --test observability -- --nocapture` | PASS | `.csdlc/evidence/759/observability-s3-outage-isolated-r15.log` | `381208e38be897bb26a99d3e06fafdbd08b7a6ff107f58cc6a4ec3b350bf9083` |
| `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings` | PASS | `.csdlc/evidence/759/strict-clippy-r15.log` | `11d208621432c4d2f122eb7b232efea5b902d5794a303b860be68af4593c310a` |
| `bash adl/tools/test_v0917_html_observatory_integrated_proof.sh` | PASS | `.csdlc/evidence/759/html-observatory-proof-r15.log` | `e6333af7915d7b7e24cda4ec9362d1c0039a4fd56b89f98c07acd1b284e69587` |
| `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check` | PASS | `.csdlc/evidence/759/fmt-check-r15.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |
| `git diff --check HEAD` | PASS | `.csdlc/evidence/759/diff-check-r15.log` | `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855` |

## Artifact hygiene

The HTML Observatory proof regenerates shared localhost certificate evidence under `.csdlc/evidence/5789/shared-localhost-certificate/`. Those generated bytes were restored to the tracked baseline after the proof run; #759 does not carry unrelated #5789 evidence churn.
