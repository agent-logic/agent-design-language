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

1. AC-1: Redacted logs arrive on a documented bounded cadence under environment, Polis, Runtime instance, and UTC date prefixes
2. AC-2: The bucket blocks public access, encrypts and versions objects, and applies declared retention lifecycle controls
3. AC-3: Publisher IAM is restricted to required bucket metadata and exact-prefix object writes
4. AC-4: S3 denial or outage leaves Runtime readiness and operation intact while buffering, retry, and failure telemetry remain bounded
5. AC-5: A separately authorized live proof retrieves and validates one archived object without exposing sensitive content
6. AC-6: Lifecycle execution uses typed v2 authority unless and until explicit v3 cutover is recorded

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
