# CSM Backpressure Proof (#4921)

Status: `passed_local_proof_remote_pending_after_branch_publication`

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
| Nessus | not_run | not_run | not_run | 4s | failed_pre_push_git_ref_missing |
| AWS Spot | pending | pending | pending | pending | pending |
| CodeBuild | pending | pending | pending | pending | pending |

Nessus note: the first direct remote attempt ran before the branch was pushed,
so the remote runner could not resolve the #4921 git ref. Rerun Nessus, AWS
Spot, and CodeBuild after draft publication pushes the branch.

## Non-Claims

- This packet does not claim autoscaling.
- This packet does not claim production cloud orchestration.
- This packet does not claim a production capacity model.
- This packet does not claim a hosted telemetry backend.
