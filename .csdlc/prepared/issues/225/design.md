# #225 Bounded Card Review Repair Design

## Purpose

Issue #225 removes two typed-editor dead ends exposed by PR #224 without
weakening normal lifecycle authority. The editor must be able to correct a
recovered Sprint SPP summary and an unbound child SIP constraint through named,
audited semantic operations. Direct values or Markdown edits remain forbidden.

## Operations

Add exactly two `SemanticOperation` variants:

- `correct_plan_summary_after_recovery { value }`
- `correct_operator_constraints_before_bind { values }`

The first owns only `SPP.plan_summary`. It is accepted only in `implemented`
when the latest transition is a typed review recovery from `reviewed`,
`published`, or `merge_ready`, and review assignment, review, publication,
readiness, and terminal truth are absent.

The second owns only `SIP.operator_constraints`. It is accepted only in
`initialized` or `ready` while branch and worktree are absent and review,
publication, readiness, and terminal truth are absent. It is a substantive
pre-bind contract repair and therefore invalidates any design approval exactly
like the initialized acceptance/plan-step repair added by #213.

## Atomicity and audit

Both operations retain the existing generation/digest CAS, card ownership,
cross-card validation, renderer, AST validation, transaction, and interrupted
write recovery. Values must be nonempty and contain no empty element. Audit
serialization records the complete previous and replacement value, plus the
request actor and reason already owned by the edit envelope.

No operation changes phase, branch, worktree, execution, validation, review,
publication, merge, terminal, or cleanup authority.

## Proof

Focused Gate 2 proof exercises initialized and ready SIP correction, design
invalidation, reapproval, stale CAS, wrong card, bound/later-phase rejection,
empty values, atomic regeneration, and audit old/new truth.

Focused Gate 5 proof exercises reviewed and published recovery followed by SPP
summary correction, clean implemented and unrecovered rejection, retained
review/publication/readiness rejection, wrong card, empty value, stale CAS,
atomic regeneration, and audit old/new truth.

Formatting, strict library/editor Clippy, typed issue validation, diff hygiene,
and independent exact-head review complete the issue.

## Non-goals

- Generic implemented-phase replanning.
- Arbitrary initialized SIP mutation.
- Direct card, Markdown, or JSON patching.
- Binding or starting WP-20.
- Sprint membership or PR #224 planning changes.

