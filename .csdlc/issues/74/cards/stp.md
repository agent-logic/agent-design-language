# Structured Task Prompt

Template: 1.0.0

Issue: 74

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add exact legacy-claim topology proof and change production only if that proof exposes a remaining defect.

## Deliverables

- csdlc-v2/tests/gate2.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs

## Acceptance

1. AC-1: unrelated claim-bearing legacy record does not block bind
2. AC-2: no unrelated worktree record is rewritten or deleted
3. AC-3: a relevant malformed record fails closed
4. AC-4: same issue, branch, or canonical worktree collisions remain blocked
5. AC-5: focused Gate 2 regression proves the behavior through csdlc-bind

## Dependencies

- #61 / PR #70 relevance-first canonical topology scan

## Inputs

- agent-logic/agent-design-language#74
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate2.rs

## Non Goals

- Delete or rewrite stale records
- Restore claim compatibility in canonical IssueRecord
- Weaken relevant-record verification
- Refactor binding broadly
