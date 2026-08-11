# ADR 0064: Adaptive Learning DAG Governance Boundary

## Status

Status: **Proposed**

## Context

WP-13A now supplies governed evaluation, bounded adaptation, graph mutation,
history integrity, rollback, and replay proof.

## Decision

Adaptive learning is a governed deterministic DAG transition. Every proposal
is bound to authority, evidence, evaluation, resource limits, and the current
graph head; accepted mutations append canonical history, while stale,
unauthorized, cyclic, over-budget, replayed, or rollback-invalid transitions
fail closed.

## Consequences

Runtime reasoning can adapt through an explicit reviewable mutation contract
without granting unrestricted self-modification.

## Alternatives Considered

Equating a repeated reasoning loop with governed adaptation was rejected.

## Source Evidence

- `docs/milestones/v0.92/features/ADAPTIVE_LEARNING_DAG_v0.92.md`
- `adl-runtime-kernel/src/adaptive_learning.rs`
- `adl-runtime-kernel/src/durable_state.rs`

## Validation Evidence

- `adl-runtime-kernel/tests/adaptive_learning.rs`
- `adl-runtime-kernel/tests/durable_state.rs`
- `.csdlc/evidence/5831/local-validation-manifest.json`
- `.csdlc/evidence/5831/native-validation-manifest.json`

## Supersession Relationships

Refines ADR 0008 and ADR 0009 while retaining their bounded-governance
constraints.

## Non-Claims

No unrestricted adaptive learning, autonomous self-modification, unbounded
recurrence, model training, recursive self-improvement, source rewriting, or
authority expansion outside the governed mutation contract is claimed.

## Approval Boundary

Human review must separately promote this candidate into `docs/adr/`.
