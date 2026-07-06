# CSM Runtime Observability API Proof (#4929)

This packet records the bounded v0.91.7 proof for the CSM-owned runtime
observability API.

## Implemented Surface

- `csm api serve --spec <agent-spec.yaml>` is available only through the
  standalone `csm` runtime binary.
- `adl csm api` rejects with an ownership error because `adl` remains tooling
  and control-plane support, not the runtime owner.
- The API binds to loopback by default and rejects non-loopback bind addresses
  until remote authorization is implemented.
- The implemented endpoints are `/status`, `/health`, `/ready`, `/metrics`,
  and `/events`.

## Endpoint Truth

`/status` reports:

- runtime owner and agent instance id
- retained daemon liveness from `daemon_status.json`
- current agent status from a read-only `status.json` snapshot; missing status
  returns a synthetic not-started snapshot without initializing state
- scheduler watcher, ChronoSense, AEE, and resilience-middleware integration
  states when present in daemon capabilities
- checkpoint freshness from retained daemon and continuity checkpoint artifacts
- continuity, replay-manifest, and safe-fail bundle artifact refs
- OTel status/log artifact refs when supplied with `--otel-status` and
  `--otel-log`
- retained operator event stream ref

`/health` and `/ready` classify degraded or not-ready states from missing
daemon artifacts, missing checkpoints, stale checkpoints, and failed/running
agent states instead of assuming success.

`/metrics` exposes bounded machine-readable gauges and states from the same
runtime artifacts.

`/events` returns a bounded redacted tail of retained operator events.

## Redaction And Bind Policy

Responses return artifact refs and byte counts rather than absolute host-private
paths. Response redaction rejects or redacts:

- `/Users/`, `/home/`, `/private/`, and `/var/folders/` paths
  even when embedded in larger event strings
- authorization headers and bearer token strings
- secret, token, authorization, and credential keyed fields
- AWS secret access key strings
- AWS ARN strings
- embedded or standalone 12-digit cloud account identifier strings

No remote/public API claim is made by this issue.

## Local Proof

Commands run from the bound #4929 worktree:

```sh
cargo fmt --manifest-path adl/Cargo.toml --all
cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_runtime_api_serves_status_health_ready_metrics_and_events -- --nocapture
```

Result:

- compile: `20.44s`
- test body: `1.32s`
- status: passed
- proof: the smoke starts `csm daemon`, writes retained observability and OTel
  status/log artifacts, starts `csm api serve`, and checks `/status`,
  `/health`, `/ready`, `/metrics`, and `/events`.

```sh
cargo test --manifest-path adl/Cargo.toml csm_runtime_api -- --nocapture
```

Result:

- initial compile: `24.62s`
- post-review-fix compile: `36.68s`
- unit tests: 3 passed
- smoke filter: 1 passed
- status: passed
- proof: missing-artifact readiness classification and event redaction,
  including embedded host-private path, secret token, AWS account id, and ARN
  cases; active runtime states produce explicit readiness blockers; missing
  runtime artifacts do not cause read-time state initialization.

```sh
cargo fmt --manifest-path adl/Cargo.toml --all --check
git diff --check
```

Result: passed.

## Bounded Review

Pre-PR subagent review found three issues:

- Embedded host paths, secrets, account ids, or ARN strings in larger retained
  event messages could leak or cause the API response assertion to fail.
- `/ready` could report `not_ready` for leased or running-cycle states without
  a blocking reason.
- `/status` originally used the mutating long-lived-agent status command, which
  could initialize runtime state during a read.

All three were fixed before publication and revalidated with
`cargo test --manifest-path adl/Cargo.toml csm_runtime_api -- --nocapture`.

## Build Platform Timing

Comparable benchmark command:

```sh
bash adl/tools/run_build_platform_benchmark.sh --platform wuji --cache-posture local_warm_target --out .adl/local-artifacts/build-platform/4929-wuji-summary.json --artifact-dir .adl/local-artifacts/build-platform/4929-wuji
```

Result:

- platform: `wuji`
- cache posture: `local_warm_target`
- build: `93s`
- test: `0.40s` real
- total: `93s`
- status: passed
- local artifact: `.adl/local-artifacts/build-platform/4929-wuji-summary.json`

Nessus, AWS Spot, and CodeBuild live benchmark rows require a committed and
pushed issue ref. They are intentionally pending until the #4929 branch has a
review-clean commit for those remote lanes to fetch.

## Non-Claims

- This does not implement a remotely exposed or unauthenticated API.
- This does not claim hosted telemetry backend readiness.
- This does not claim protobuf OTLP/gRPC or metrics export beyond current
  retained OTel status/log evidence.
- This does not replace retained file/log artifacts; it provides a local API
  view over them.
