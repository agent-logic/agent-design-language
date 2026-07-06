# CSM Backpressure Proof (#4921)

Status: `passed_local_proof_remote_timing_recorded`

This packet proves a bounded CSM overload and backpressure policy for #4921.
It exercises the standalone `csm` runtime owner binary, writes retained
machine-readable backpressure state, and projects queue/backpressure signals
through the runtime API `/metrics` endpoint.

## What This Proves

- CSM owns a bounded resource taxonomy for runtime loop, event export,
  checkpoint writes, snapshot/diff writes, DAG execution, provider calls,
  cloud hooks, and continuity serialization.
- Required state is not silently dropped under overload.
- Noncritical work can be deferred or shed with explicit observability.
- Retry exhaustion is represented as a bounded fail-closed case.
- Survival-threshold breach verifies the retained `safe_fail_bundle.json`
  before recording `safe_fail_serialize` as the action.
- Proof cases exercise runtime loop, event export, checkpoint writes,
  snapshot/diff writes, DAG execution, provider calls, cloud hooks, and
  continuity serialization.
- Runtime API metrics expose queue depth, lag, deferred count, shed count,
  retry budget, backpressure health, and safe-fail action.

## Evidence

| Artifact | Purpose |
| --- | --- |
| `agent.yaml` | Retained local CSM runtime spec. |
| `state/daemon_status.json` | CSM daemon fire-up status and runtime capability evidence. |
| `state/safe_fail_bundle.json` | Verified recoverable safe-fail serialization bundle required by the proof. |
| `state/csm_backpressure_state.json` | Runtime-local backpressure state consumed by the API. |
| `proof/backpressure_report.json` | Full taxonomy, policy matrix, negative cases, and non-claims. |
| `proof/csm_backpressure_state.json` | Proof-local copy of the runtime backpressure state. |
| `api_status.json` | `/status` sample including backpressure artifact presence. |
| `api_metrics.json` | `/metrics` sample with backpressure gauges and states. |
| `observability.log` | ADL observability events, including `backpressure_policy`. |

## Local Proof Commands

- `cargo fmt --manifest-path adl/Cargo.toml --all --check`
- `cargo check --manifest-path adl/Cargo.toml --locked --bin csm`
- `cargo test --manifest-path adl/Cargo.toml --locked --test cli_smoke csm_runtime_api_serves_status_health_ready_metrics_and_events -- --nocapture`
- `adl/target/debug/csm daemon --spec docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/agent.yaml --max-restarts 1 --checkpoint-interval-secs 1 --no-sleep --json`
- `adl/target/debug/csm backpressure prove --spec docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/agent.yaml --out docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/proof --profile soak2 --json`
- `adl/target/debug/csm api serve --spec docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/agent.yaml --bind 127.0.0.1:49210 --max-requests 2 --idle-timeout-ms 60000 --json`
- `curl -fsS http://127.0.0.1:49210/status -o docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/api_status.json`
- `curl -fsS http://127.0.0.1:49210/metrics -o docs/milestones/v0.91.7/review/runtime/csm_backpressure_4921/api_metrics.json`

## Observed Metrics

| Metric | Value |
| --- | ---: |
| `backpressure_queue_depth` | 12 |
| `backpressure_lag_ms` | 3100 |
| `backpressure_deferred_count` | 23 |
| `backpressure_shed_count` | 7 |
| `backpressure_retry_budget_remaining` | 0 |

States:

- `backpressure_health`: `capacity_degraded`
- `backpressure_safe_fail_action`: `safe_fail_serialize`
- `safe_fail_action.status`: `verified`
- `safe_fail_action.recoverability_class`: `recoverable_sleeping`

## Platform Timings

| Platform | Build | Test | Total | Wrapper | Status |
| --- | ---: | ---: | ---: | ---: | --- |
| wuji local warm target | 92s | 35s | 127s | 127s | passed |
| Nessus | 211s | 186s | 397s | 403s | passed_slow |
| AWS Spot | not_completed | not_completed | not_completed | interrupted_at_592s_then_wrapper_stalled | failed_interrupted |
| CodeBuild | 242s | 223s | 465s | 493s | passed_slow |
| GitHub `adl-ci` required check | in_progress_after_907s_then_success | in_progress_after_907s_then_success | waiter_timed_out_at_907s_before_later_success | repo-native `pr.sh finish --merge` waiter plus validation recheck | passed_after_long_test_step |
| wuji exact PR-fast lane replay | 134s | 4s | 138s | direct `run_pr_fast_test_lane.sh` replay | passed |

Nessus note: the first direct remote attempt ran before the branch was pushed,
so the remote runner could not resolve the #4921 git ref. The post-publication
rerun passed but was slower than wuji for this benchmark.

AWS Spot note: the post-publication Spot run launched and began validation, then
the instance was interrupted while the validation command was still running. The
local wrapper did not return a final failed summary after cleanup began, and
`resume-state.json` still showed no recorded attempts. This is a remote-build
tooling problem to fix before relying on Spot for sprint throughput.

CodeBuild note: the post-publication CodeBuild run succeeded, but the inner
benchmark was slower than wuji for this lane.

GitHub `adl-ci` note: after the clippy repair, PR #4971 commit
`869a1154e9f108a363a375bc7110b73f0194c8da` reached the Rust `test` step and
remained `IN_PROGRESS` until the repo-native `pr.sh finish --merge` waiter
timed out at 907s. The required check was pending, not failed; `adl-coverage`
was green and `adl-slow-proof` was skipped by policy. A later repo-native
validation recheck reported `adl-ci` completed successfully for the same
commit, so this is recorded as a long CI test-step latency finding rather than
a #4921 test failure.

Wuji replay note: the exact PR-fast lane selected by CI was replayed locally
with `bash adl/tools/run_pr_fast_test_lane.sh --base origin/main --head HEAD`.
It selected `test(csm_cmd) or test(csm_backpressure) or test(csm_runtime_api)
or binary_id(adl::cli_smoke) and test(/^agent::/)`, compiled in 2m14s, and ran
18 tests in 4.355s with 18 passed and 19665 skipped. This makes the long
GitHub check a CI-runner liveness/performance finding rather than a reproduced
#4921 test failure.

## Non-Claims

- This packet does not claim autoscaling.
- This packet does not claim production cloud orchestration.
- This packet does not claim a production capacity model.
- This packet does not claim a hosted telemetry backend.
