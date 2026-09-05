# Issue #695 design: five-minute agent partials with asynchronous S3 archive

## Cadence and isolation

Runtime v3 adds one bounded periodic coordinator outside agent execution tasks. The
cadence uses Tokio monotonic fixed-rate time with a default interval of 300 seconds,
a validated inclusive configuration range of 60 through 86,400 seconds, and the first
tick one full interval after successful Runtime readiness. Missed ticks use `Skip`:
the coordinator never bursts or tries to recreate missed cycles.

At each tick it freezes one ordered snapshot of canonical resident IDs. An agent
admitted after that point waits until the next cycle. An agent removed during capture
may finish the already-owned snapshot, but a tombstone at a later sequence prevents
that partial from resurrecting it. The coordinator owns at most one in-flight capture
per agent and a configurable global concurrency bound defaulting to four. It clones a
bounded immutable state view under the existing short-lived lock, then serializes and
hashes outside locks through a size-limited writer. Snapshot serialization performs no
network or provider I/O and has no arbitrary wall-clock timeout: its enforceable bound
is the 16 MiB output cap plus the Runtime's already-bounded resident state. A capture
that exceeds that cap fails deterministically. One failure cannot delay other agents,
conversations, readiness, shutdown, or the next fixed-rate cycle. A still-running
capture is skipped at the next tick and reported as overdue; it is never duplicated.

## Partial and restore authority

Each partial is a self-contained snapshot of one agent, not a delta. The versioned
schema binds stable `runtime_instance_id`, `polis_id`, canonical agent ID/name,
provider/model reference, per-agent snapshot sequence, global cadence sequence,
parent full-checkpoint generation and integrity digest, source
`runtime_incarnation_id`, creation time, payload checksum, and canonical digest.
Secrets, credentials, raw provider bodies, and infrastructure locations are excluded.
The stable Runtime instance, polis, agent, schema, and parent full-checkpoint lineage
must match on restore. The source incarnation is retained for audit but is expected to
differ after restart and is not a rejection condition.

Restore starts with the latest valid full Runtime checkpoint and selects the highest
valid self-contained partial per agent whose parent generation/integrity matches it.
Per-agent sequences must strictly increase; global cadence gaps are allowed because
unchanged, slow, removed, or coalesced agents need not produce every cycle. Duplicate
sequence with identical digest is idempotent; duplicate sequence with different bytes,
lower sequence, wrong identity/lineage, or invalid digest is rejected. Signed
tombstones use the same identity and sequencing rules and override older snapshots.

## Bounded local retention and S3 outage behavior

Atomic local writes use create-temp, fsync, rename, and parent-directory fsync. The
recoverable local store is independently bounded to 2 GiB and 8,192 files; the archive
spool is bounded to 512 MiB and 4,096 files; each partial is limited to 16 MiB. Normal
retention keeps up to the latest twelve locally completed partials per agent plus the
latest tombstone while the global caps permit it. It prunes oldest superseded partials
across agents by `(sequence, canonical agent ID)` before admitting a write and never
prunes the newest valid partial or newest tombstone for any agent. If those protected
newest records alone reach either local-store cap, the next write is refused as
`local_store_saturated`; disk use does not grow, the affected agent becomes `failed`,
and Runtime remains available. Because partials are self-contained, spool saturation coalesces pending
uploads per agent to the newest valid partial while retaining the latest local
recoverable state and tombstone. It never evicts the only current partial for an agent.
Coalescing increments a durable dropped-archive-interval count and changes archive
state to `degraded`; it does not claim continuous five-minute remote history. If the
newest-per-agent set itself exceeds either global bound, new archive enqueue is
refused and reported as `spool_saturated`, while local checkpointing and Runtime stay
available. Archived local predecessors may be pruned after successful receipt.

A separate worker uploads immutable objects using workload IAM, exponential backoff
from 5 seconds through 5 minutes with jitter, and deterministic idempotency. Object
keys are privacy-safe digests:
`v1/polis/<polis-digest>/runtime/<instance-digest>/agent/<agent-digest>/sequence/<20-digit>.json`.
Sequence objects are immutable. Each upload requests an S3-managed SHA-256 checksum,
and the returned checksum is matched before a durable archive receipt is committed.
The worker also updates a deterministic mutable per-agent `latest.json` pointer so
asynchronous recovery needs two bounded object reads rather than an archive scan.
S3 failure only changes archive freshness and backlog state.

## API and Observatory contract

Roster and agent-detail projections retain canonical `name`, `provider`, and `model`
and add nullable integer `last_snapshot_at_unix_millis`, nullable integer
`last_archive_at_unix_millis`, nullable integer `snapshot_sequence`, integer
`pending_archive_count`, `snapshot_state` (`never_snapshotted`, `current`, `overdue`,
or `failed`), and `archive_state` (`disabled`, `current`, `pending`, `degraded`, or
`spool_saturated`). `never_snapshotted` means no attempt has completed and the first
scheduled cycle is not yet due. `current` means the most recently due cycle completed
successfully. `overdue` means a due cycle is still running or was skipped because that
agent already had an in-flight capture. `failed` means the most recently completed
attempt failed, including `local_store_saturated`; the next successful cycle returns
it to `current`. Snapshot-state precedence is failed, overdue, current, then never.
Archive state is `disabled` when unconfigured, `current` when the newest local sequence
has a durable receipt and no backlog, `pending` when that sequence is queued without a
recorded failure, `degraded` after upload failure or coalescing, and `spool_saturated`
when enqueue was refused; a receipt for the newest sequence and an empty backlog
returns it to `current`. Archive-state precedence is spool_saturated, degraded,
pending, current, then disabled. Timestamps are authoritative successful-write/receipt times; null
means no success and zero is never synthesized. No bucket, prefix, object key, account,
credential, or error body is exposed. Observatory renders these exact fields per agent
and labels all null/degraded states without inferring success from elapsed time.

## Terraform and proof boundary

Terraform owns a dedicated private bucket with all four public-access-block flags,
BucketOwnerEnforced ownership, versioning, a customer-managed KMS key with annual
rotation, a TLS-only deny policy, and lifecycle rules that expire current partials
after 30 days, noncurrent versions after 7 days, and incomplete multipart uploads after
one day. The writer role receives only `s3:PutObject`, `s3:AbortMultipartUpload`, and
KMS encrypt/data-key permissions constrained to the exact archive prefix and encryption
context; the restore role separately receives prefix-scoped object read and KMS
decrypt. Bucket policy requires the declared KMS key and TLS. Repository work produces
plans and deterministic fixtures only; live AWS apply and permanent Wuji rollout remain
separate operator-authorized operations.

The acceptance manifest enumerates every AC row. Its validator consumes a required
exact-head result receipt, checks the receipt's Git SHA against `HEAD`, requires a
successful result for every named proof, and rejects each proof whose `test_count` plus
`assertion_count` is zero. Named
test targets cover cadence and roster mutation, slow-agent overlap, crash-atomic writes,
spool saturation/coalescing, outage/retry/idempotency, restore rejection and restart,
roster/detail API schema, Observatory states, and Terraform plan/policy assertions.
