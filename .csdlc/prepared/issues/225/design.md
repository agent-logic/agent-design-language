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
when the latest relevant review audit event is exactly `recover_review`, its
recorded transition returned the issue from `reviewed`, `published`, or
`merge_ready`, and review assignment, review, publication, readiness, and
terminal truth are absent. A transition-shaped record or stale recovery event
is insufficient provenance.

The second owns only `SIP.operator_constraints`. It is accepted only in
`initialized` or `ready` while branch and worktree are absent; migration,
execution, validation, review, publication, readiness, and terminal truth are
absent; and the authored design and diagram digests still match the record.
Authored-file drift fails closed instead of being absorbed by this operation.
It is a substantive pre-bind contract repair and therefore invalidates only
the existing design approval; no other projection field changes.

## Atomicity and audit

Both operations retain the existing generation/digest CAS, card ownership,
cross-card validation, renderer, AST validation, transaction, and interrupted
write recovery. Values, request actor, and request reason must be nonempty;
collection values may contain no empty element. Invalid input fails before any
durable mutation. Audit serialization records the complete previous and
replacement value, plus the validated request actor and reason.

No operation changes phase, branch, worktree, execution, validation, review,
publication, merge, terminal, or cleanup authority.

## Proof

Focused Gate 2 proof exercises initialized and ready SIP correction, design
invalidation, reapproval, stale CAS, wrong card, bound/later-phase rejection,
migration rejection, authored design/diagram drift rejection, empty values,
empty actor/reason rejection with zero mutation, atomic regeneration, and audit
old/new truth.

Focused Gate 5 proof exercises reviewed, published, and merge-ready recovery
followed by SPP summary correction; exact `recover_review` provenance; stale or
transition-only provenance rejection; and audit-only recovery from implemented
without a qualifying transition rejection. It also covers clean implemented and
unrecovered rejection, retained review/publication/readiness rejection, wrong
card, empty value, empty actor/reason rejection with zero mutation, stale CAS,
atomic regeneration, and audit old/new truth.

Formatting, strict library/editor Clippy, typed issue validation, diff hygiene,
and independent exact-head review complete the issue.

## Non-goals

- Generic implemented-phase replanning.
- Arbitrary initialized SIP mutation.
- Direct card, Markdown, or JSON patching.
- Binding or starting WP-20.
- Sprint membership or PR #224 planning changes.
