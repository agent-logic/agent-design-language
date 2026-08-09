# Structured Task Prompt

Template: 1.0.0

Issue: 87

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remove the impossible ACIP minor upper-bound comparison while preserving inclusive-range semantics and prove the shared fix.

## Deliverables

- Clippy-clean ACIP version predicate
- adl-runtime/tests/acip_version_negotiation.rs
- Positive coverage for exact and wider compatible ranges
- Negative coverage for future-only and malformed ranges
- Both issue-named strict Clippy proofs

## Acceptance

1. AC-1: Preserve intended minimum and maximum major/minor compatibility semantics
2. AC-2: Focused positive and negative supported/unsupported offer coverage passes
3. AC-3: distributed_capability_advertisement strict Clippy passes
4. AC-4: distributed_resource_weather strict Clippy passes
5. AC-5: No Sprint child-owned implementation or test module changes

## Dependencies

- Current canonical Agent Logic main
- Sprint 4 child integration-test targets for final consumer proof

## Inputs

- AGENTS.md
- adl-runtime/src/acip.rs
- agent-logic/agent-design-language#87

## Non Goals

- Changing ACIP protocol constants
- Adding child capability-advertisement, resource-weather, or discovery behavior
- Refactoring unrelated runtime code
- AWS or broad workspace validation
