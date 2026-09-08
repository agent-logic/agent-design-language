# Structured Task Prompt

Template: 1.0.0

Issue: 695

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove bounded local five-minute agent partials, asynchronous S3 archival, restore validation, API projection, Observatory rendering, and Terraform planning without live deployment.

## Deliverables

- Typed bounded periodic coordinator covering Shepherd and dynamic residents
- Versioned integrity-bound partial schema and atomic local store
- Durable bounded asynchronous S3 upload spool and archive worker
- Validated full-checkpoint-plus-partials restore path
- Roster and detail API continuity fields including provider/model and snapshot/archive status
- Observatory per-agent backing-model and snapshot/archive rendering
- Private encrypted versioned S3 Terraform module and least-privilege IAM
- Accelerated deterministic and production-shaped isolated tests
- adl-runtime-kernel/src/agent_partial_checkpoint.rs
- demos/html-observatory/tests/agent_continuity.test.mjs
- infra/aws/runtime/agent-checkpoint-archive/validate.sh
- .csdlc/prepared/issues/695/validate-acceptance.sh

## Acceptance

1. AC-1: A monotonic fixed-rate coordinator defaults to 300 seconds, accepts only 60 through 86400 seconds, starts after one full interval, and skips rather than bursts missed ticks
2. AC-2: Each cycle freezes an ordered resident roster including Shepherd; admission/removal, one-in-flight-per-agent, global concurrency four, and bounded no-I/O serialization cannot overlap captures or block conversations, readiness, shutdown, or other agents
3. AC-3: Each atomic self-contained partial is at most 16 MiB, excludes secrets and infrastructure locations, and binds stable Runtime/polis/agent identity, provider/model, per-agent and cycle sequences, parent full-checkpoint lineage, source incarnation, time, checksum, and canonical digest
4. AC-4: The local store is capped at 2 GiB and 8192 files and the asynchronous idempotent S3 spool at 512 MiB and 4096 files; normal retention keeps at most twelve partials plus the latest tombstone per agent, globally prunes only superseded records, coalesces pending uploads during outage, refuses growth as local_store_saturated or spool_saturated when protected newest records fill a cap, and never gates Runtime
5. AC-5: Terraform enforces all four S3 public-access blocks, BucketOwnerEnforced ownership, versioning, TLS-only access, customer-managed annually rotated KMS encryption, 30/7/1-day lifecycle rules, and separate prefix-scoped writer and restore IAM
6. AC-6: Restore composes the latest full checkpoint with the highest valid later self-contained partial per agent, accepts a differing source incarnation, allows global cadence gaps, treats identical duplicates idempotently, honors later tombstones, and rejects corruption, conflicting duplicates, rollback, or stable identity/lineage mismatch
7. AC-7: Roster and agent-detail APIs expose canonical name, provider, model, nullable last local snapshot and S3 archive times, nullable snapshot sequence, pending archive count, and no storage details; snapshot state is never before first due cycle, current after the latest due success, overdue while a due capture runs or is skipped, failed after the latest completed failure, with precedence failed-overdue-current-never and success recovery
8. AC-8: Observatory renders the authoritative per-agent API fields; archive state is disabled when unconfigured, current only when newest is receipted with no backlog, pending while queued without failure, degraded after failure/coalescing, spool_saturated after enqueue refusal, with precedence saturated-degraded-pending-current-disabled and newest receipt recovery, never inferring success
9. AC-9: An exact-HEAD acceptance results receipt rejects missing, failed, duplicate, stale-head, and zero-test or zero-assertion proof and maps every AC to accelerated production-shaped tests for cadence, roster mutation, slow capture, crash atomicity, saturation/coalescing, outage/retry, restore/restart, API, Observatory, and Terraform policy
10. AC-10: Focused validation, exact-range hygiene, and independent exact-head review pass without live Runtime or AWS mutation

## Dependencies

- Current Runtime v3 continuity and agent lifecycle baseline
- Issue #594 log archive is a convention input only and not modified
- Live rollout depends on reviewed merge plus separate operator authorization

## Inputs

- agent-logic/agent-design-language#695
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/live_continuity.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- demos/html-observatory
- infra/aws/runtime/log-archive

## Non Goals

- No replacement of full checkpoints or migration bundles
- No Runtime log archive redesign
- No synchronous S3 call from agent execution
- No cloud-dependent readiness
- No provider, A2A, or conversation-history redesign
- No live Runtime restart, AWS apply, or paid validation
