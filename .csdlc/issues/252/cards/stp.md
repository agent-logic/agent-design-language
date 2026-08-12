# Structured Task Prompt

Template: 1.0.0

Issue: 252

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Diagnose, implement, repeatedly prove, independently review, and publish the smallest Guardian spawn-flake correction; stop before merge.

## Deliverables

- Deterministic Guardian child executable resolution or fixture construction
- Regression proof for both hosted failures
- Required Runtime local lane and strict Clippy evidence
- Fresh exact-head review and ready PR

## Acceptance

1. AC-1: Both observed regressions pass repeatedly without dependence on caller cwd or test order.
2. AC-2: Genuine missing child programs still produce SpawnFailed.
3. AC-3: The required Runtime local lane passes.
4. AC-4: Strict all-target Clippy passes.
5. AC-5: Fresh exact-head review has no actionable findings.

## Dependencies

- PR #243 hosted Runtime attempt-1 and attempt-2 failure evidence
- Current origin/main Guardian implementation

## Inputs

- adl-runtime/src/guardian.rs
- adl-runtime-kernel/tests/parity_b_live_kernel.rs
- .github/workflows/ci.yml
- AGENTS.md

## Non Goals

- Birth-witness behavior changes
- Broad Guardian redesign
- Optional or paid CI
- Merge or cleanup
