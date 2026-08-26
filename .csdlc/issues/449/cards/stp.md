# Structured Task Prompt

Template: 1.0.0

Issue: 449

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare, review, bind, implement, validate, review, publish, and finish only the #449 governed Adaptive Learning resident integration. Stop rather than inventing production capability/profile handles if dependency gates are not satisfied.

## Deliverables

- adl-runtime-kernel/src/adaptive_learning.rs
- adl-runtime-kernel/tests/adaptive_learning.rs
- Runtime-owned resident adaptive-learning integration path
- Precondition/fail-closed validation for all required production inputs
- MutationGate-only accepted adaptation application with canonical digest/evidence retention
- Rejected/cancelled/invalid proposal terminal evidence without mutation
- Restart/rehydration deterministic continuation proof
- Focused actual-resident-cycle integration proof
- Truthful v0.92 feature/evidence updates after proof

## Acceptance

1. AC1: A Runtime-owned production resident path invokes execute_governed_adaptive_learning; no fixture, benchmark, or demo binary serves as the production entrypoint.
2. AC2: The call binds exact resident identity, verified continuity, governed cognitive profile, capability envelope, loop outcome, evaluation, proposal, policy, mutation grant, cancellation token, and prior durable history.
3. AC3: Accepted adaptations mutate graph/state only through MutationGate, retaining canonical before/after graph and state digests, evaluation bindings, decision, policy, and history.
4. AC4: Rejected, unauthorized, malformed, stale, invalid-profile, invalid-capability, cancelled, and unbounded proposals make no mutation and retain terminal evidence.
5. AC5: Restart/rehydration restores the latest valid adaptive history and continues deterministically without replaying or duplicating an accepted mutation; tamper, rollback, gaps, and lineage mismatch fail closed.
6. AC6: Focused production-path integration proof executes an actual resident cycle through one accepted adaptation, one rejected adaptation, durable restart, and deterministic continuation; unit tests alone are insufficient.
7. AC7: Observability/evidence distinguishes proposed, accepted, rejected, applied, restored, and cancelled outcomes without leaking private profile or provider content.
8. AC8: Canonical feature and v0.92 evidence truth is updated from library-only to production-integrated only after exact proof passes.

## Dependencies

- Sibling capability/profile production-handle issue must deliver verified Runtime handles before #449 can claim full AC2 production binding.
- #446 is related and must remain separate; #449 must coordinate without combining mutation authority and ACC tool-actuation authority.
- Existing MutationGate, adaptive-learning history, continuity, and cancellation contracts.

## Inputs

- GitHub issue #449 body and acceptance criteria
- adl-runtime-kernel/src/adaptive_learning.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/src/adaptive_learning.rs
- adl-runtime-kernel/tests/adaptive_learning.rs
- existing MutationGate, history, continuity, and cancellation contracts
- canonical v0.92 feature/evidence documents

## Non Goals

- Arbitrary self-modification
- Hidden reward channel
- ACC tool execution or governed adapter dispatch owned by #446
- Birthday composition
- Demo restoration
- Runtime v4 redesign
- Fixture-only proof
- Provider/private profile content leakage
