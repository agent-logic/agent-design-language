# #213 Initialized-Phase Card Repair Design

## Purpose and literal #205 sequence

Issue #213 closes the typed lifecycle gap exposed by #205. An unbound issue can
receive a design-review finding after bootstrap and must repair its acceptance
contract and executable plan without hand-editing generated cards or falsely
binding before dependencies permit execution.

The required sequence is literal, not illustrative:

1. #205 is bootstrapped at generation 0 with design and diagram bindings `D0`;
2. an allowed initialized planning-collection edit advances it to generation 1;
3. review adds the protected all-47 manifest to design/diagram truth, producing
   current bindings `D1` while the SPP/VPP projections still contain `D0`;
4. one initialized STP acceptance repair and one initialized SPP plan-step
   repair must each be reachable without bind; and
5. the successful repair transaction must atomically refresh both SPP and VPP
   to current `D1`, invalidate design approval, regenerate all projections, and
   preserve the issue/audit prefix.

The first successful pre-bind repair after design or diagram drift owns the
binding refresh. It verifies the old recorded references are the canonical
issue-owned paths, reads those exact regular files beneath the repository,
computes current digests, writes the current ref/digest pair to both SPP and
VPP in the same transaction, and only then performs cross-card validation and
commit. There is no intermediate committed state containing repaired semantics
with stale bindings or refreshed bindings with unrepaired semantics.

## Exact phase, card, and operation contract

`csdlc-edit apply` accepts only these additional combinations:

- phase `initialized` or `ready`, card `stp`, operation
  `replace_acceptance_criteria`; and
- phase `initialized` or `ready`, card `spp`, operation
  `replace_plan_steps`.

Pre-bind STP criteria must use the exact ordinal vocabulary `AC-1` through
`AC-N`, encoded as a nonempty ordered array whose entry at ordinal `n` begins
with `AC-n:`. IDs may not be missing, duplicated, reordered, skipped, malformed,
or embedded only in prose. This issue preserves the existing denominator: a
pre-bind acceptance repair may change criterion text but not `N`.

Every replacement pre-bind SPP step must have a unique nonempty identifier, a
nonempty action, status exactly `pending`, and a nonempty set of valid ordinal
acceptance IDs. The union of all step acceptance IDs must equal the current STP
ordinal denominator. `in_progress` and `completed` remain valid in their
existing execution-phase operations, but are rejected by these newly admitted
initialized/ready repairs.

Existing semantic validators, exact card ownership, and cross-card validation
remain authoritative. Stale generation or issue digest is rejected before any
read or mutation. The change does not admit either operation in reviewed,
published, merge-ready, merged, or closed-out phases and grants no source,
validation, execution, review, publication, merge, terminal, or Git-topology
authority while unbound.

## Binding refresh and drift rejection

The pre-bind refresh is not a generic path repair. It accepts only the exact
`record.design_path` and `record.diagram_path` established at bootstrap and
requires both SPP and VPP to name those same paths before mutation. It rejects:

- absolute, traversal, empty, noncanonical, symlinked, replaced, missing,
  non-regular, or outside-repository design/diagram paths;
- SPP/VPP reference disagreement or a reference different from the record;
- issue, repository, slug, version, schema, template, or generation identity
  drift across any card;
- a changed initialization digest, transition history, branch/worktree
  topology, review/publication/readiness/terminal state, or audit prefix; and
- a digest read that races inode or size replacement.

The store already validates canonical repository-relative authored paths at
bootstrap. The repair reuses the same safe authored-artifact reader rather than
plain path joining. It snapshots file identity and bytes before digesting and
fails closed on any replacement or ambiguity.

## Review invalidation and ready-phase guard

Acceptance criteria and plan steps are substantive design inputs. Any
successful initialized or ready repair sets `design_review` to `pending`, keeps
the lifecycle phase unchanged, preserves branch/worktree topology exactly, and
appends one generation-scoped audit event. Doctor reports
`design_review_missing_or_stale` until `csdlc-edit approve-design` records a
fresh independent approval over the current design digest.

The ready-phase route is additionally guarded: it is admitted only when the
record was already `ready`, unbound (`branch == null` and `worktree == null`),
has no review assignment/result, publication, readiness, migration, or terminal
evidence, and passes the exact generation/digest CAS. A pending design review
created by an earlier accepted repair does not grant authority and does not
change phase. A second complementary repair is permitted under a fresh CAS so
the literal AC-then-step sequence can complete before one independent
reapproval. Any bound topology or later lifecycle evidence fails closed.

At initialized, the same no-topology/no-later-evidence guard applies. Existing
bound `replace_acceptance_criteria` and `replace_plan_steps` authorization is
retained byte-for-byte. Existing implemented SPP plan-step repair behavior is
retained byte-for-byte. #213 adds no implemented STP acceptance route and does
not change any execution-phase status semantics.

## Atomicity and byte-preservation proof

The existing store transaction remains the sole write owner. The focused test
takes before snapshots of every issue file and the design/diagram bytes, applies
one operation, then takes after snapshots. It proves:

- exact byte equality for design, diagram, initialization digest, transition
  history, topology, later-phase evidence, and every untouched semantic field;
- the audit file equals the complete before bytes plus exactly one newline-
  terminated event;
- all six card identity generations advance by exactly one and all generated
  values/Markdown/AST projection digests match the committed index;
- only the selected semantic field, SPP/VPP binding pair when stale, plan
  revision for plan-step replacement, design-review state, generation,
  projections, index digest, and appended audit event may differ; and
- `fail_after_backup`, malformed input, stale CAS, drift, or validation failure
  leaves every before snapshot byte-identical after typed recovery.

The audit event records the semantic operation and the binding-refresh fact,
including old and new design/diagram digests, without copying design contents.

## Focused proof

The Gate 2 integration fixture proves both phases and both operations:

1. bootstrap and approve a complete issue, then perform the same prior
   initialized planning edit used by #205;
2. change the exact design and diagram files and prove the next initialized AC
   repair atomically refreshes bindings, repairs exact ordinal criteria,
   invalidates approval, and preserves before/after bytes as declared;
3. apply a fresh-CAS complementary initialized step repair with every step
   pending and exact ordinal coverage, then independently reapprove;
4. advance to ready and repeat both repairs, including pending reapproval,
   unchanged phase/topology/audit prefix, and one final reapproval;
5. reject stale generation and digest, malformed/reordered/renumbered ACs,
   changed denominator, non-pending pre-bind steps, duplicate/missing/extra AC
   coverage, wrong card ownership, path/reference/identity drift, bound topology,
   later evidence, unsupported phase, and interrupted writes without partial
   mutation; and
6. run explicit compatibility fixtures proving the pre-existing bound STP and
   SPP operations and implemented SPP plan-step operation retain their former
   results, status vocabulary, audit shape, and phase behavior.

The proving lanes are the complete `gate2` integration binary, strict Clippy
for all C-SDLC v2 targets, formatting, and a committed base-to-source diff check
using `git diff --check origin/main...HEAD`. The diff lane covers the complete
source range rather than only uncommitted work and rejects whitespace plus
blank-line-at-EOF defects. Fresh independent exact-head review is required
before publication.

## Non-goals

- No #205 card repair, binding, implementation, or publication in this issue.
- No generic JSON Patch, Markdown import, arbitrary pre-bind mutation, or
  acceptance-denominator change.
- No change to validation-lane, execution, review, publication, merge, terminal,
  or cleanup authority.
- No weakening of CAS, safe authored-path reads, cross-card coverage,
  transaction recovery, design approval, Git topology, or dependency gates.
