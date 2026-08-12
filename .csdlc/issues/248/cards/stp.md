# Structured Task Prompt

Template: 1.0.0

Issue: 248

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Correct only process-backend output-limit versus timeout precedence and coupled cleanup proof.

## Deliverables

- Deterministic process-backend precedence rule
- repeated focused regression proof
- required Runtime validation evidence

## Acceptance

1. AC-1: Oversized file output observably present at deadline arbitration reports output_limit independent of scheduler order.
2. AC-2: Ordinary hanging execution reports timeout.
3. AC-3: Output-limit and timeout terminal paths leave no output artifacts and terminate owned process trees.
4. AC-4: Existing timeout, output-limit, cancellation, process-tree, and cleanup semantics remain green.
5. AC-5: Repeated focused pressure, required Runtime lane, strict Clippy, Observatory proof, and fresh exact-head review pass.

## Dependencies

- Required CI failure run 31563230539 job 94009803658
- Blocks #244/PR #247 until merged and ancestral

## Inputs

- adl-runtime-kernel/src/parity.rs
- adl-runtime-kernel/tests/parity.rs
- adl-runtime-kernel/src/bin/adl-runtime-shadow-fixture.rs
- .github/workflows/ci.yml

## Non Goals

- Changing #244 cleanup hooks or PR #247
- Changing #112 authority
- Widening timing constants
- Unrelated Runtime refactoring
- Optional or paid CI
