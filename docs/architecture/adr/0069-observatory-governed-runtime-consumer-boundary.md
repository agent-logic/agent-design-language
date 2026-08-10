# ADR 0069: Observatory Governed Runtime Consumer Boundary

## Status

Status: **Deferred**

## Context

The Observatory has landed local proof surfaces, but WP-18A still owns the real
HTML and Unity consumer path against the governed Runtime API and WSS stream.

## Decision

Defer the durable consumer decision until both clients prove authenticated,
redacted, replay-aware consumption of real Runtime output without private-state
or control-authority leakage.

## Consequences

Existing Observatory demonstrations remain evidence inputs, not completion of
the v0.92 consumer architecture.

## Alternatives Considered

Static fixtures, screenshots, and URL-only checks were rejected as terminal
consumer proof.

## Source Evidence

- `docs/milestones/v0.92/features/OBSERVATORY_UNITY_CONSUMER_INTEGRATION_v0.92.md`

## Validation Evidence

- `adl-runtime-kernel/tests/observatory.rs`
- `.csdlc/evidence/5757/observatory-integrated-proof.log`

## Supersession Relationships

May refine ADR 0048 and ADR 0054 after WP-18A proof.

## Non-Claims

No real dual-client Runtime round trip, Unity completion, or Observatory control
authority is claimed.

## Approval Boundary

WP-18A landed executable proof and human review are required before Proposed.
