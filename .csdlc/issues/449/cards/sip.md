# Structured Intent Prompt

Template: 1.0.0

Issue: 449

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Integrate the existing governed Adaptive Learning DAG into actual Runtime resident execution without using fixture, benchmark, or demo entrypoints as production proof.

## Required Outcome

A Runtime-owned production resident path invokes execute_governed_adaptive_learning with exact resident identity, continuity, profile, capability, proposal, policy, MutationGate, cancellation, and durable-history bindings; accepted adaptations mutate only through MutationGate and restart/rehydration continues deterministically.

## Scope

- Runtime-owned resident adaptive-learning invocation path
- Production binding of resident identity, verified continuity, governed cognitive profile, capability envelope, loop outcome, evaluation, proposal, policy, MutationGate, cancellation token, and prior durable history
- Accepted/rejected/cancelled/fail-closed adaptive-learning evidence and durable history
- Restart/rehydration of latest valid adaptive history with deterministic continuation
- Focused production-path integration proof through actual resident cycle
- Canonical v0.92 feature/evidence truth updates only after exact proof passes

## Authority

- MutationGate remains the only graph/state mutation authority for accepted adaptations
- Resident production path owns input binding, precondition validation, terminal evidence, and restart integration
- Existing governed adaptive-learning executor remains decision-policy authority unless bounded API changes are required
- Capability/profile handles are dependency-gated production inputs and must not be fabricated
- #446 ACC tool-actuation authority is out of scope and must not be merged into #449

## Assumptions

- Issue #449 body is authoritative for scope and acceptance criteria.
- Existing adaptive-learning executor and durable history primitives are available.
- Production capability/profile handles remain dependency-gated and must not be fabricated.

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use issue-bound FastWork worktree before source implementation
- Preserve #446 staging and other worktrees
- Do not absorb #446, #341, #343, #84, #122, #251, birthday composition, Runtime v4 redesign, or demo-restoration scope
- Use standard runners only for CI unless explicitly reauthorized
- No design-only completion; #449 requires code and proof
