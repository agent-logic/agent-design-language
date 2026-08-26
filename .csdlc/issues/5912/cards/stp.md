# Structured Task Prompt

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement only the production provisioning, invocation, emission, and focused proof missing from #5833.

## Deliverables

- Runtime-owned opaque policy service
- validated receipt emission boundary
- production-path integration test

## Acceptance

1. AC1: A non-test Runtime owner can provision trusted policy without exposing policy internals.
2. AC2: The production path builds and validates a witness packet before emitting its receipt.
3. AC3: An external integration test proves real production-path use and exact receipt emission.
4. AC4: Existing #5833 security, privacy, canonicalization, rejection, and non-authority behavior remains unchanged.
5. AC5: Focused tests and strict Clippy pass at the reviewed exact revision.

## Dependencies

- Closed issue #5833 and merged PR agent-logic/agent-design-language#198

## Inputs

- adl-runtime-kernel/src/birth_witness.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/birth_witness.rs

## Non Goals

- Changing the witness algorithm or receipt schema
- Broad Runtime orchestration or Birthday demo work
- Reopening or modifying #5833
- Granting birth, citizenship, governance, legal, or launch authority
