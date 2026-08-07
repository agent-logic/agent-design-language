# Structured Task Prompt

Template: 1.0.0

Issue: 5905

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and execute only the closed-issue terminal reconciliation path.

## Deliverables

- Typed historical reconciliation request and result
- Focused Rust tests
- Validated terminal envelope for #5800
- Terminal reconciliation census for all closed v0.92 issues

## Acceptance

1. After the implementation PR merges, #5800 is reconciled first against canonical PR #9 and exact merge SHA 7dfb791ad2fc1ecbc1e3b3651815b1d37bfa060f, and its cached terminal envelope validates before any other issue is processed
2. Every issue in the frozen closed-v0.92 inventory has exactly one truthful merged, closed_unmerged, or closed_no_pr disposition
3. Merged reconciliation requires exact issue repository, PR repository, issue, PR, head SHA, merge SHA, and GitHub closing linkage
4. Closed_unmerged requires an exact closed unmerged PR and explicit operator-approved reason; closed_no_pr requires the canonical approval label, no PR, and explicit operator-approved reason
5. Open, mismatched, multiply attributable, ambiguously migrated, contradictory-field, or unsupported remote state fails closed
6. Routine csdlc-finish review and publication gates are unchanged
7. Historical results use distinct live_github_historical_reconciliation provenance and never assert missing review or publication history
8. Focused Rust tests cover success, idempotency, disposition rules, the identity mismatch matrix, ambiguous linkage, provenance, and routine-gate non-regression
9. No generated state is hand-edited and post-merge envelopes remain Git-common authority rather than tracked main mutations

## Dependencies

- Live closed issue and PR state
- Existing csdlc.derived_terminal.v1 schema
- Current shared GitHub token resolver

## Inputs

- AGENTS.md
- csdlc-v2/src/finish.rs
- csdlc-v2/src/store.rs
- csdlc-v2/tests/gate_finish.rs

## Non Goals

- Reopening product implementation
- Restoring csdlc-import or v1 wrappers
- Weakening current review-before-publication
- Unrelated worktree cleanup
