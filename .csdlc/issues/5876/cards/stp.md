# Structured Task Prompt

Template: 1.0.0

Issue: 5876

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement deterministic rollback and recovery for failed, interrupted, or ambiguous relocation.

## Deliverables

- adl-runtime/src/distributed/recovery.rs
- adl-runtime/tests/distributed_recovery.rs
- Integration issue #5878 owns production module registration
- Deterministic focused recovery tests covering every migration stage, restart, source and target loss, quorum loss, divergent durable histories, stale or malicious authority claims, and bounded-resource denial
- Exact-revision execution proof with machine-derived negative-case evidence and immutable source/evidence bindings
- Independent exact-head security and correctness review before publication with a bounded rollback path
- Fail-closed ambiguous recovery: neither the highest local epoch nor the last durable local owner may confer authority without majority or quorum committed proof

## Acceptance

1. Implement only the declared exclusive paths
2. Preserve Guardian, authentication, bounds, determinism, durability, and redaction invariants
3. Run the exact named test with nonzero test enforcement
4. Prove applicable stale, replay, malformed, unauthorized, failure, and recovery cases
5. Bind all evidence to the exact source revision and artifact digests
6. Complete independent review and child-owned typed closeout

## Dependencies

- #5909 PR #120 must merge and be ancestral before #5870 may execute
- #5870 must merge and be ancestral before #5873 and #5874 may execute
- #5873 and #5874 must both merge and be ancestral before #5875 may execute
- #5875 must merge and be ancestral before #5876 may bind or implement
- #5862 is the coordination umbrella and is not a substitute for child terminality
- #5821 is the terminal architecture gate

## Inputs

- docs/milestones/v0.92/features/DISTRIBUTED_GUARDIAN_POLIS_v0.92.md
- .csdlc/prepared/issues/5821/design.md
- adl-runtime/src/distributed/lease.rs
- adl-runtime/src/distributed/fencing.rs
- adl-runtime/src/distributed/snapshot_catalog.rs
- adl-runtime/src/distributed/migration.rs

## Non Goals

- Sibling WP-04 paths
- Runtime v2 fallback
- Custom cryptography or plaintext
- WP-14, consumer UI, or v0.93 work
- Self-attested completion
