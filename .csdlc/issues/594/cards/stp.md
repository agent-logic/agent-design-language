# Structured Task Prompt

Template: 1.0.0

Issue: 594

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

One bounded Runtime-to-Vector-to-S3 archival path and its Terraform, focused tests, and authorized live AWS proof.

## Deliverables

- Private encrypted versioned S3 archive with public access blocked and explicit retention
- Bounded Vector S3 buffering and delivery configuration
- Least-privilege identity-scoped publisher policy
- Focused local failure and configuration proof
- Separately authorized live AWS object delivery and retrieval receipt

## Acceptance

1. AC-1: Only runtime_v3_redacted records enter keys matching logs/env=<env>/polis=<polis>/runtime=<runtime>/year=<YYYY>/month=<MM>/day=<DD>/hour=<HH>/<uuid>.json.gz, flushed by 5 MiB or 60 seconds.
2. AC-2: The bucket blocks all public access, uses bucket-owner-enforced ownership and SSE-S3, enables versioning, retains current versions 30 days and noncurrent versions 7 days, and aborts incomplete multipart uploads after 1 day.
3. AC-3: Publisher IAM permits only GetBucketLocation on the exact bucket and PutObject plus multipart completion/abort operations on the exact environment, Polis, and Runtime prefix.
4. AC-4: The S3 sink disables startup health checks, uses an isolated 512 MiB disk buffer with drop-newest overflow, retries at most five times with backoff capped at 30 seconds, emits failure/drop telemetry, and cannot block Vector startup, Runtime readiness, master-log progress, or CloudWatch health.
5. AC-5: A separately authorized live proof verifies the expected business account, bucket controls and encrypted object metadata, retrieves one nonempty archived object into issue evidence, and inspects redaction without printing sensitive content.
6. AC-6: Lifecycle execution uses typed v2 authority unless and until explicit v3 cutover is recorded, and no live AWS action occurs without separate authorization.

## Dependencies

- Issue #589 is related but not an implementation dependency
- C-SDLC v3 exercise is gated by explicit operator-reviewed v3 cutover
- Live AWS proof is gated by separate paid-cloud authorization

## Inputs

- agent-logic/agent-design-language#594
- adl-runtime-kernel/src/observability/vector.rs
- adl-runtime-kernel/src/observability.rs
- infra/aws/csm-runtime-health

## Non Goals

- Replacing CloudWatch heartbeat alarms or SSM recovery
- Making archival synchronous or readiness-critical
- Archiving unredacted secrets, credentials, or raw sensitive payloads
- Using CloudFormation
- Performing cloud mutation during preparation
