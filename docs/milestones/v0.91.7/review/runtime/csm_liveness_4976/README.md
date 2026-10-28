# CSM Operational Liveness Test (#4976)

Issue `#4976` is an intermediate WP-07 operational test while final runtime
coherence gate `#4906` remains deferred behind owner work in WP-08 and WP-12.

## Current Status

As of the retained check at `2026-07-06T17:22:35Z`, the full CSM operational
run is live.

The full run is under `full/` and enables:

- standalone CSM daemon from the repo owner binary
  `adl/target/debug/csm`;
- live AWS CloudWatch heartbeat publication using the Agent Logic
  `agent-logic-admin` profile;
- local retained `ADL_OBSERVABILITY_LOG`, `ADL_OTEL_LOG`, and
  `ADL_OTEL_STATUS` files;
- OTLP HTTP JSON export to a bounded loopback collector;
- CSM API server with `/status`, `/health`, `/ready`, `/metrics`, and
  `/events` response artifacts;
- continuity checkpoints, cycle ledger, safe-fail bundle, and safe-fail
  artifact snapshots.

The earlier local-only and AWS-only daemon attempts were stopped with
`adl agent stop` after the full run superseded them. Their post-stop process and
agent status snapshots are retained under `published/stops/`.

## Live Processes

Permission-safe `adl process status --pid-file ... --json` checks reported
`live_pid` for:

- `full/logs/csm.pid`: full CSM daemon.
- `full/collector/collector.pid`: loopback OTLP collector.
- `full/api/csm-api.pid`: CSM API server.

These checks use the repo-owned process-status helper and do not rely on broad
host process scans. Compact process-status snapshots are retained under
`published/process/`.

## AWS Signal Evidence

The full run writes live CloudWatch heartbeat events to:

- log group: `/adl/v0917/wp07/4976/runtime-full`
- log stream: recorded in `full/logs/cloudwatch_stream.txt`
- retained fetch: `full/logs/cloudwatch_recent_events.redacted.json`

The latest retained CloudWatch fetch showed `200` recent heartbeat events, with
the latest event at heartbeat sequence `543` for runtime
`csm-liveness-4976-full`. The exported messages are operations-safe and do not
record credentials or raw AWS account identifiers.

The publishable CloudWatch sample is
`published/aws/cloudwatch_recent_events.redacted.json`.

## OTel Evidence

The full run writes local OTel monitor status to
`full/logs/otel_status.json`. The retained status showed:

- `event_count`: `1271`
- exporter endpoint configured: `true`
- exporter status: `success`
- HTTP status: `200`
- last event: `csm.child_exit`
- trace id: `agent.csm-liveness-4976-full.daemon`

The loopback collector status snapshot in `published/otel/collector_status.json`
showed `received_request_count: 1271`, matching the OTel status count at the same
check.

## API Evidence

The CSM API server started on a loopback address recorded in the local live
artifact `full/api/csm_api_stdout.jsonl`. The following endpoint responses were
retained in publishable form:

- `published/api/status.json`
- `published/api/health.json`
- `published/api/ready.json`
- `published/api/metrics.json`
- `published/api/events.json`

This run used an explicit ephemeral loopback bind and observed
`127.0.0.1:57053`. Follow-on `#4980` owns the canonical CSM API port and bind
contract; this packet does not claim that `57053` is the stable operator port.

## Runtime State Evidence

The full run retains:

- `published/state/cycle_ledger_tail.jsonl`
- `published/state/continuity_checkpoint.json`
- `published/state/daemon_status.json`
- `published/state/safe_fail_bundle_snapshot.json`

The live append-only state remains local under `full/state/` while the run is
active and is ignored for PR publication.

The latest retained cycle-ledger sample showed successful cycles through
`cycle-000139`.

## Adjacent Owner Work

This packet does not replace final `#4906` runtime-coherence proof. The final
gate still needs to consume or disposition adjacent owner work, including open
WP-08 AWS/Polis issues:

- `#4635` WP-08 runtime AWS and signal operations umbrella.
- `#4685` ACIP to SNS integration.
- `#4686` AWS signal integration in full.
- `#4687` local Polis SSM operations.
- `#4688` S3 ObsMem community-memory archive policy.
- `#4913` durable storage for Polis state.
- `#4915` CloudFront and control-plane hooks.

WP-12 security/protocol rows also remain separate owner work for the final
`#4906` gate.

## Non-Claims

This packet does not claim:

- final v0.92 runtime coherence;
- completion of WP-08 AWS/Polis owner issues;
- completion of WP-12 security/protocol owner issues;
- production-host service-manager permanence;
- resilience to host resource exhaustion or missing-binary conditions.

Follow-on `#4977` tracks the CSM owner-binary availability guard so runtime work
does not depend on incidental `target/` cache state.

Follow-on `#4979` tracks splitting runtime control-plane commands away from the
`adl` tooling binary into a CSM-owned control-plane surface.
