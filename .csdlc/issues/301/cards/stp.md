# Structured Task Prompt

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue 301 only; github.rs and focused owner tests, with no store, card, recovery, or coverage changes.

## Deliverables

- Durable title-only operation provenance
- Exact reconciliation semantics
- Focused regression tests

## Acceptance

1. AC-1: Title-only update preserves prior body content while durably recording the operation key
2. AC-2: Immediate readback verifies title, body preservation, and operation provenance before reconciled=true
3. AC-3: Same-key retry is idempotent and conflicting reuse fails closed
4. AC-4: Body-bearing updates retain compatibility
5. AC-5: Focused tests and strict Clippy pass
6. AC-6: Fresh exact-head review has no unresolved finding

## Dependencies

- Independent of #291, #295, and #297/#298

## Inputs

- csdlc-v2/src/github.rs
- csdlc-v2/tests/gate_github_actions.rs

## Non Goals

- Lifecycle redesign
- Bulk issue title repair
- Publication, merge, or closeout
