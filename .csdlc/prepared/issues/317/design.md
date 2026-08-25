# Issue 317 Design: v0.92 Terminal Closeout Plan

Status: authored for independent design review before execution bind.

## Authority And Outcome

Issue #317 / WP-28A owns a documentation-only, non-mutating plan for the
remaining v0.92 release tail. Its predecessor gate is the reviewed, green merge
of #316 / PR #472 into `main`; typed finish and worktree cleanup are asynchronous
and never gate this issue or its successors.

The output is an exact issue universe and an acyclic action graph. Each row
separates live GitHub issue and PR state, reviewed head and merge ancestry,
required checks, typed lifecycle state where present, retained evidence,
release dependency, classification, owner, and next action. Missing typed
closeout receipts are recorded as asynchronous reconciliation work, not as a
reason to block an already reviewed and merged successor dependency.

## Execution Model

1. Derive the required v0.92 work-package universe from canonical milestone
   planning and release-tail documents. Reconcile legacy `#5847`-`#5852`
   provenance to canonical current issues `#314`-`#319` using an explicit
   one-to-one mapping retained in the issue evidence; emit only canonical issue
   rows, retain legacy IDs as provenance fields, and reject missing, duplicate,
   ambiguous, or unmapped identities.
2. Record exactly one row per issue with immutable PR/head/merge/check/review
   identity and a truthful classification.
3. Build an acyclic graph whose execution gates are reviewed green merges and
   ancestry. Model typed finish, cleanup, umbrella bookkeeping, and handoff
   reconciliation as asynchronous follow-up nodes.
4. Hand the reviewed plan to #318 / WP-29 after #317's reviewed green merge;
   do not execute #318, #319, tag, release, closeout, or activation here.
5. Publication may open or update a reviewed PR whose body contains
   `Closes #317`; publication itself neither merges the PR nor closes #317.
   Closure occurs only if the operator later authorizes and completes merge.

## Owned Paths

- `docs/milestones/v0.92/V092_TERMINAL_CLOSEOUT_PLAN_317.md`
- `.csdlc/evidence/317`
- `.csdlc/prepared/issues/317/validate-closeout-plan.rb`

All other repository, GitHub, lifecycle, release, and historical surfaces are
read-only inputs.

## Validation Contract

The validator must derive the denominator from canonical tracked authority and
the explicit canonical/legacy mapping,
reject missing, duplicate, extra, unknown, or unowned rows, verify Git object
and merge ancestry claims, and prove the action graph acyclic. Focused negative
fixtures must fail for stale heads, red checks, absent review, non-ancestral
merge, duplicate rows, unknown nodes, dependency cycles, and closeout-as-gate
serialization. Live GitHub acquisition is a nondeterministic observation lane
that must retain observation time, repository, issue and PR identities, exact
head and merge, checks, reviews, and source-response digests. Deterministic
snapshot validation consumes that retained envelope and rejects self-declared
or unbound evidence. Markdown/JSON hygiene and exact-scope diff checks are
required.

## Rollback

Delete the candidate plan/evidence and regenerate it from canonical tracked and
live read-only authority. No remote mutation is required to roll back #317.

## Non-Goals

- No merge, typed finish, cleanup, tag, release, issue close, or v0.93 activation.
- No rewriting historical records or treating GitHub closure alone as proof.
- No use of legacy #5850 artifacts as current lifecycle authority.
