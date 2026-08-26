# Structured Task Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Reconcile only the contradictory continuity binding and prove composition and fail-closed substitution; do not redesign Runtime or absorb WP-18.

## Deliverables

- One shared verified continuity contract for downstream capability/cognition
- Focused real signed composition test
- Substitution and authority/privacy regression tests
- Exact review and required CI proof

## Acceptance

1. AC-1: Capability and cognitive validation bind the same verified BirthdayContinuityRecord.
2. AC-2: Real signed identity, continuity, capability, and governed cognition compose successfully.
3. AC-3: Substituted record, head, identity root, and identity-record digest each fail with typed rejection.
4. AC-4: Existing invalid-authority and privacy negatives remain green.
5. AC-5: No public trust constructor or caller-nominated authority input is added.
6. AC-6: Fresh exact-head independent review and the one required CI lane pass before merge.

## Dependencies

- Issue #5836 failing integrated composition
- Existing Birthday identity and continuity authority contracts
- Existing capability and cognitive authority contracts

## Inputs

- adl-runtime-kernel/src/birthday.rs
- adl-runtime-kernel/src/birthday_identity.rs
- adl-runtime-kernel/src/birthday_continuity.rs
- adl-runtime-kernel/src/capability_envelope.rs
- adl-runtime-kernel/src/cognitive_profile.rs
- adl-runtime-kernel/tests

## Non Goals

- Runtime server, transport, storage, or scheduler redesign
- Fixture or cached packet positive proof
- Public authority constructors
- Implementation or publication of issue #5836
