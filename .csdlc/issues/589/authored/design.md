# Issue 589 design: reliable Wuji startup and reload

## Decision

Ordinary Runtime v3 startup uses the Guardian's existing authenticated child
lease as the sole startup-liveness relationship. The Guardian starts the
kernel, authenticates the lease, observes kernel lifetime, forwards shutdown,
and applies restart policy. It does not establish a second private continuity
client before declaring the child healthy.

Distributed checkpoint transfer and migration continuity remain explicit
operation-time capabilities. They must not be a prerequisite for starting a
single-node Polis or serving its API.

## Operator surface

CSM owns the host-service lifecycle for Runtime v3:

- `start` validates the selected init file, reconciles only provably stale
  startup metadata, starts the Guardian, and waits for HTTPS readiness.
- `stop` asks the Guardian for graceful checkpointed shutdown.
- `status` reports host-service, Guardian, kernel, API, and configuration
  identity in one result.
- `reload` validates a candidate init file before restart, retains the last
  known-good configuration, and rolls back if readiness is not reached.

The command must be idempotent: starting an already healthy Runtime succeeds,
and restarting after an interrupted attempt converges without manual journal
or lock editing.

## State and safety

Retained Polis state, signed checkpoints, credentials, and runtime identity are
not reset. A lock with a live, matching owner remains a hard failure. Automatic
reconciliation is allowed only when CSM has a durable start-transaction record
that names the lock path and exact owner PID, the permission-safe exact-PID
probe proves that PID dead, and an atomic quarantine rename followed by a
second absence/ownership check succeeds. A legacy ownerless lock without that
correlated start record remains ambiguous and fails closed with one recovery
command; missing `owner.json` alone is never evidence of staleness.

The separate port-20998 Guardian continuity handshake is removed from ordinary
startup. This does not authorize migration, target activation, checkpoint
transfer, or distributed membership changes; those retain their governed
operation-time authority.

## Failure behavior

- Invalid candidate configuration fails before touching the running service.
- Failed reload restores the last known-good config and service.
- Ambiguous writer ownership fails closed with one actionable diagnostic.
- A healthy kernel is not killed merely because an unrelated continuity
  transport is absent or stale.

## Proof

Focused tests cover cold start, idempotent start, interrupted startup recovery,
live-writer rejection, candidate validation, failed-reload rollback, Guardian
restart, and stable HTTPS readiness on port 20997. Operational proof checks
Wuji locally and through the existing AWS-facing routes without broad cloud
changes.

## AWS remote recovery

The Runtime regularly emits a health heartbeat through the existing
Vector/CloudWatch EMF path. The metric identifies the Polis and Runtime instance
and reports readiness plus the active configuration generation; it contains no
credentials or sensitive state. CloudWatch alarms on an unhealthy value or
missing heartbeat data. A low-frequency Synthetics check of Wuji's public
`/v1/health` separately covers the end-to-end edge path.

An alarm routes through EventBridge and invokes a bounded SSM Run Command
against the uniquely identified managed Wuji host. That command calls the same
CSM `status` and governed `start` or `reload` operation used locally; it does not
manipulate processes, locks, or state directly. The alarm, trigger, and terminal
recovery result are emitted through SNS. Repeated delivery is idempotent, and
ambiguous account, host identity, or live-writer ownership fails closed without
resetting Polis state. Alarm recovery does not disable continued CloudWatch
observation, so persistent failure remains visible rather than creating a
silent restart loop.
