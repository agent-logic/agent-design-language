# Issue #659 Design: Configurable Runtime v3 Convergence

## Decision

Replace fixed service-control waits in `csm_runtime_v3_cmd.rs` with one explicit `service_convergence` policy loaded through `RuntimeInitConfig`. The policy has four independently validated millisecond fields:

| Field | Default | Valid range | Meaning |
|---|---:|---:|---|
| `stop_timeout_millis` | 300,000 (5 min) | 1,000–3,600,000 | Guardian-owned Runtime process reaches stopped state. |
| `unload_timeout_millis` | 300,000 (5 min) | 1,000–3,600,000 | Service manager reports the unit unloaded. |
| `listener_timeout_millis` | 300,000 (5 min) | 1,000–3,600,000 | The configured Runtime TCP listener accepts a connection. |
| `readiness_timeout_millis` | 900,000 (15 min) | 1,000–3,600,000 | The owned Runtime passes the authenticated full `/v1/ready` probe. |

The entire object is serde-defaulted so existing Runtime init files receive the generous defaults without migration. Invalid values fail before any service mutation. Launchd or systemd remains the sole process owner throughout start and reload.

## Behavior

- Parse and validate convergence policy before any service mutation.
- Treat listener-open and Runtime-ready as separate gates. A successful TCP connection proves only that the listener is open; it does not prove authenticated `/v1/ready`, model availability, observability readiness, or service-manager ownership.
- Start waits first for `listener`, then for `readiness`. Stop and interrupted-transaction recovery wait for `stop`; service removal waits for `unload`.
- Report the exact unfinished stage (`stop`, `unload`, `listener`, or `readiness`) and configured deadline when convergence expires.
- Preserve the last-known recoverable service state and existing rollback path.
- Keep provider/model execution and general API request timeout semantics outside this issue.
- Exercise slow success using deterministic controllable test probes; do not sleep for production-scale durations in tests.

## Validation

Focused unit tests cover default/backward-compatible parsing, every bound, invalid configuration before mutation, slow success within a configured deadline, true expiry with exact-stage diagnostics, rollback/recovery, and removal of fixed 15-second operational waits. The CLI tests prove listener-open and full readiness independently. Rust formatting, focused strict Clippy, diff hygiene, and fresh exact-head review are required before publication. The live Runtime is not restarted.
