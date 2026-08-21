# Issue #449 design: governed Adaptive Learning resident integration

Status: initialized design packet for review.

## Scope

Issue #449 wires the existing governed Adaptive Learning DAG into a production Runtime-owned resident execution path. The integration must use an actual resident cycle entrypoint, not a fixture, benchmark, or demo binary.

## Production boundary

The resident cycle should construct a governed adaptive-learning invocation only after Runtime-owned inputs are available:

- resident identity and continuity evidence;
- governed cognitive profile and capability envelope from production handles;
- loop outcome, evaluation, and proposal for the current resident cycle;
- policy and MutationGate authority;
- cancellation token;
- prior durable adaptive history.

The adaptive-learning executor may remain the central authority for decision logic. The production resident path owns gathering and binding the inputs, enforcing fail-closed preconditions, and persisting terminal evidence.

## Dependency boundary

The issue may prepare design, local tests, and non-overlapping integration scaffolding in parallel with sibling Runtime work. It must not claim capability/profile production handles until the sibling issue has delivered and proven them. If those handles are unavailable at implementation time, #449 must stop at the exact dependency gate rather than inventing fixture authority.

#446 is related but distinct: #446 owns ACC-governed tool execution and must not be merged with adaptive-learning mutation authority.

## Proposed implementation slices

1. Add a Runtime-owned resident adaptive-learning adapter that maps production resident-cycle evidence into the existing `execute_governed_adaptive_learning` API.
2. Add precondition validation for identity, continuity, cognitive profile, capability envelope, bounded proposal, policy, prior history, and cancellation state.
3. Route accepted adaptations through `MutationGate` only, retaining canonical before/after graph and state digests plus evaluation/policy bindings.
4. Persist terminal evidence for accepted, rejected, unauthorized, malformed, stale, invalid-profile, invalid-capability, cancelled, and unbounded proposals.
5. Restore adaptive-learning history on restart/rehydration and fail closed on tamper, rollback, gaps, or lineage mismatch.
6. Add production-path integration proof that drives an actual resident cycle through accepted adaptation, rejected adaptation, restart, and deterministic continuation.
7. Update v0.92 feature/evidence documents only after the production-path proof passes.

## Non-goals

- ACC tool execution or governed adapter dispatch; that belongs to #446.
- Arbitrary self-modification or hidden reward channels.
- Birthday/demo restoration, Runtime v4 redesign, or fixture-only proof.
- Provider-content leakage in observability/evidence.

## Review questions

- Does the design preserve MutationGate as the only mutation authority?
- Are capability/profile handles treated as dependency-gated production inputs rather than fixture values?
- Does the proof plan exercise an actual resident cycle and restart rather than only unit tests?
- Are #446 tool-actuation concerns kept outside #449?
