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

## v0.92.1 Disposition

Status remains **Deferred**. The accepted consumer/authority separation in
[ADR 0054](../../adr/0054-runtime-v3-guardian-owned-kernel-and-api-boundary.md)
and observability boundary in
[ADR 0048](../../adr/0048-runtime-observability-and-otel-boundary.md) continue to
apply. This update does not promote the dual-client completion decision.

The [milestone decisions](../../milestones/v0.92.1/DECISIONS_v0.92.1.md) require
authentic Runtime data and prohibit invented, mocked, or status-only authority
from release proof. The [OBS-B record](../../../.csdlc/issues/512/cards/sor.md)
reports HTML polis isolation, origin/provenance display, conversation/A2A
activity, recovery, accessibility, and security tests. Those are concrete
implementation inputs; HTML evidence alone does not discharge this record's
real HTML-and-Unity, authenticated, redacted, replay-aware consumption gate.

The [release-tail report](../../milestones/v0.92.1/evidence/integration/gap_analysis_report.md)
also retains review/proof gaps. Its recorded BLOCKED decision cannot be
converted into Observatory acceptance by writing an ADR. This curation did not
perform a new live dual-client proof or adjudicate those findings.

Promotion requires the original WP-18A proof and human review, with exact
revision, both client identities, authentic runtime readback, redaction,
replay/recovery behavior, and explicit absence of presentation-owned execution
authority. The original alternatives, non-claims, and approval boundary remain.
