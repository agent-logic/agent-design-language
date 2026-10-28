# CSM Liveness 4980 Live Proof

Issue: #4980
Captured: 2026-07-07T03:22:00Z
Bind: 127.0.0.1:19997
Runtime owner: csm

## Result

- `csm daemon` was started from the rebuilt #4980 `csm` binary without `--no-sleep` and without public restart or kill controls.
- `csm api serve` was started from the rebuilt #4980 `csm` binary on canonical port `127.0.0.1:19997`.
- `/ready` returned `ready` with no blocking reasons.
- `/metrics` returned `service_mode: permanent`, `restart_policy: always`, `restart_count: 0`, `completed_cycle_count: 601`, and `operator_event_count_observed: 2404` at capture time.
- A scan of the retained API responses and touched runtime surfaces found no deprecated cap-control vocabulary.

## Retained Response Snapshots

- `api/ready.json`
- `api/status.json`
- `api/metrics.json`
- `api/events.json`

## Local Runtime Artifacts

The live runtime also emitted append-oriented state, checkpoint, safe-fail, observability, and OTel artifacts under this directory. Those artifacts are intentionally retained locally for operator inspection; only the compact proof packet is intended for PR review.
