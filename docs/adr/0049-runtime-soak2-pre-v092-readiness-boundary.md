# ADR 0049: Runtime Soak #2 Pre-v0.92 Readiness Boundary

- Status: Accepted
- Date: 2026-07-06
- Accepted in: v0.91.7
- Related issues: #4681, #4682, #4683, #4843, #4989
- Related ADRs: ADR 0011, ADR 0012, ADR 0038
- Source evidence:
  - `docs/milestones/v0.91.7/RUNTIME_SOAK_2_EXECUTION_PACKET_v0.91.7.md`
  - `docs/milestones/v0.91.7/review/runtime/SOAK2_FEATURE_LIST_MATRIX_4843.md`
  - `docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md`

## Context

v0.92 cannot rely on isolated component proofs alone. The runtime needs a
minimal integrated path that consumes scheduler, provider, AEE/ObsMem,
observability, AWS/signal, security, and Observatory evidence where applicable.

## Decision

ADL should use Runtime Soak #2 as the pre-v0.92 integration gate. Feature rows
must be classified as integrated proven, ready for soak, blocked before soak,
deferred, or routed to a later soak with evidence.

## Consequences

- Component completion no longer substitutes for integrated runtime proof.
- Blockers stay visible before v0.92 activation.
- Soak evidence becomes a release-tail artifact, not an optional demo.

## Alternatives Considered

### Open v0.92 from component proofs alone

Rejected. It would preserve the "pieces on the floor" failure mode.

## Validation Notes

Review the Soak #2 execution packet and feature-list matrix before claiming
runtime coherence.

## Non-Claims

- This ADR does not make blocked-before-soak rows complete.
- This ADR does not make a smoke test equivalent to a soak.
