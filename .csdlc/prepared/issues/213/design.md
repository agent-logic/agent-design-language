# #213 Initialized-Phase Card Repair Design

## Purpose

Issue #213 closes one typed lifecycle gap exposed by #205. An unbound issue can
receive a design-review finding after bootstrap and must be able to repair its
acceptance contract and executable plan without hand-editing generated cards or
falsely binding before its dependencies permit execution.

This change extends only the existing semantic operations
`replace_acceptance_criteria` and `replace_plan_steps`. It does not add a raw
values patch surface, a rendered-Markdown mutation path, or any execution
authority.

## Phase and ownership contract

`csdlc-edit apply` accepts these exact additional combinations:

- phase `initialized` or `ready`, card `stp`, operation
  `replace_acceptance_criteria`; and
- phase `initialized` or `ready`, card `spp`, operation
  `replace_plan_steps`.

Existing card ownership and semantic validators remain authoritative. STP
criteria must be nonempty. SPP steps must have unique nonempty identifiers,
nonempty actions, valid pending/in-progress/completed status, and acceptance IDs
that exactly cover the current STP denominator under cross-card validation.
Stale generation or issue digest is rejected before mutation.

The change does not enable either operation in reviewed, published, merge-ready,
merged, or closed-out phases. Existing bound and implemented behavior remains
unchanged. It does not allow source changes, validation results, execution
records, review results, publication, or Git topology mutation while unbound.

## Review invalidation

Acceptance criteria and plan steps are substantive design inputs. A successful
initialized/ready repair therefore changes `design_review` to `pending` when it
was approved or changes-required. The issue stays in its current lifecycle
phase, retains its branch/worktree fields, and receives one new generation and
one append-only audit event. Doctor must report
`design_review_missing_or_stale` until `csdlc-edit approve-design` records a
fresh independent approval for the repaired package.

The approved design and diagram digests remain unchanged when only card values
change. Reapproval refreshes the canonical SPP/VPP design and diagram bindings
through the existing typed owner. No record deletion, initialization reset, or
audit truncation is permitted.

## Atomicity and preservation

The existing store transaction remains the write owner. Before commit it
verifies current projections, applies the semantic operation, runs cross-card
validation, advances one generation across all six card identities, appends one
audit event, renders values/Markdown/AST projections, and commits atomically.
Interrupted-write recovery and `fail_after_backup` behavior remain unchanged.

Untouched semantic card fields, repository and issue identity, initialization
digest, transition history, review/publication/terminal fields, and Git topology
must remain byte-equivalent after ignoring the expected generation, projection
digest, design-review, edited field, and appended-audit changes.

## Focused proof

The Gate 2 integration fixture proves both phases and both operations:

1. bootstrap and approve a complete issue;
2. apply initialized STP acceptance replacement and prove exact regenerated
   criteria, incremented generation, append-only audit, unchanged plan until its
   own operation, and pending design review;
3. prove stale generation/digest replay, malformed criteria, missing acceptance
   coverage, and wrong card/operation ownership fail without partial mutation;
4. apply initialized SPP plan-step replacement and prove exact regenerated
   steps, plan revision advancement, cross-card coverage, append-only audit, and
   pending review;
5. independently reapprove, advance to ready, and repeat both repairs there;
6. prove doctor blocks after each repair and passes after exact reapproval; and
7. retain the existing bound operation tests and rejection coverage for later
   phases.

The proving lanes are the complete `gate2` integration binary, strict Clippy
for all C-SDLC v2 targets, formatting, and diff hygiene. A fresh independent
exact-head review is required before publication.

## Non-goals

- No #205 card repair, binding, implementation, or publication in this issue.
- No generic JSON Patch, Markdown import, or arbitrary pre-bind mutation.
- No change to validation-lane, execution, review, publication, merge, terminal,
  or cleanup authority.
- No weakening of CAS, cross-card acceptance coverage, transaction recovery,
  design approval, Git topology, or dependency gates.

