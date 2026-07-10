# Runtime AWS Heartbeat

This runbook covers the WP-08 runtime heartbeat publisher added for issue `#4684`.
It publishes long-lived-agent heartbeat status to CloudWatch Logs when live AWS
signal mode is explicitly enabled.

## CSM Binary Preparation

The live proof wrapper now resolves the standalone `csm` runtime owner binary
through `adl/tools/ensure_csm_binary.sh`. It writes
`csm_binary_availability.json` into the proof directory so the run records
whether it reused an existing trusted executable or restored the binary through
the repo-native Rust build path after warm-cache preparation.

You do not need to run a manual `cargo build` before the proof unless you are
deliberately pre-warming the repo target cache.

## Live Proof Command

Run the proof wrapper from an issue worktree or the main checkout:

```sh
AWS_PROFILE=agent-logic-admin ADL_AWS_PROFILE=agent-logic-admin \
  bash adl/tools/run_wp08_heartbeat_live_proof.sh \
    --out docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684 \
    --profile agent-logic-admin \
    --region us-west-2 \
    --run-id wp08-4684-live-$(date -u +%Y%m%dT%H%M%SZ) \
    --csm-bin adl/target/debug/csm
```

The wrapper creates or reuses the bounded log group
`/adl/v0917/wp08/4684/runtime-heartbeat`, sets a seven-day retention policy,
creates a run-specific stream, runs `csm daemon` with live heartbeat publication
enabled with run-scoped state and observability artifacts, retries CloudWatch
event retrieval for eventual consistency, and writes `live_heartbeat_summary.json`.

Validate the retained summary:

```sh
python3 adl/tools/validate_wp08_heartbeat_live_proof.py \
  docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/live_heartbeat_summary.json
```

Use `--cleanup` only for disposable test runs. Cleanup deletes only the
run-specific CloudWatch log stream; the bounded issue log group remains under
its seven-day retention policy.

## Runtime Environment

Live publication is opt-in. These variables must be set for a live heartbeat:

- `ADL_AWS_SIGNAL_MODE=live`
- `ADL_AWS_SIGNAL_APPROVED=true`
- `ADL_AWS_PROFILE=agent-logic-admin`
- `AWS_PROFILE=agent-logic-admin`
- `ADL_AWS_REGION=us-west-2`
- `ADL_AWS_HEARTBEAT_TARGET=cloudwatch_logs`
- `ADL_AWS_HEARTBEAT_LOG_GROUP=/adl/v0917/wp08/4684/runtime-heartbeat`
- `ADL_AWS_HEARTBEAT_LOG_STREAM=<existing stream name>`

Missing approval, region, profile, log group, log stream, or an unsupported
target causes the publisher to fail closed and emit an `aws_runtime_heartbeat`
failure event instead of attempting a live AWS write.

## Retained Proof

The canonical `#4684` proof is retained under:

`docs/milestones/v0.91.7/review/runtime/wp08_heartbeat_4684/`

The proof records the Agent Logic AWS profile, region, hashed account id,
CloudWatch group/stream, event count, selected heartbeat fields, and redaction
checks. It must not record AWS credentials or raw account ids. Each run writes
state under `state/<run-id>/` and observability to an `observability_<run-id>.log`
file so repeated proofs do not mix runtime state.
