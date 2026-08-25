# Issue 318 Design: v0.92 Next-Milestone Review Pass

Status: authored for independent design review of the bound WP-29 scope.

## Authority And Outcome

Issue #318 / WP-29 owns an independent, documentation-only review of the v0.92
terminal packet and the planned v0.92.1 and v0.92.2 handoffs. Its only opening
dependency is #317 / PR #474: independently reviewed, required checks green,
merged as `5b035094725d1872b48dda8692ef88f46487f37c`, and ancestral to `main`.
Typed finish, worktree cleanup, and administrative closeout are asynchronous
and do not gate this review.

This issue produces findings and dispositions and may correct only the four
canonical v0.92.1 planning-contract surfaces named under Owned Paths when the
review finds title variance, a bundled issue, or a missing concrete result. It
does not implement product work, create successor issues, activate v0.92.1,
v0.92.2, or v0.93, merge, release, tag, finish, clean, or close any issue.
Legacy issue #5851 is provenance only; canonical lifecycle and publication
authority remains issue #318.

## Independent Reconstruction

The review must not accept the #317 packet's denominator or conclusions by
self-attestation. It independently reconstructs the complete 13-row canonical
Sprint 6 universe from tracked wave and sprint authorities:

`#307`, `#308`, `#309`, `#310`, `#311`, `#312`, `#313`, `#314`, `#315`,
`#316`, `#317`, `#318`, and `#319`.

For each row it independently resolves the canonical issue, closing PR where
applicable, head and merge identity, classification, owner, and next action
from current GitHub and Git observations. The normalized observation artifact
is retained with the review result. Missing, duplicate, extra, ambiguous,
stale, unowned, or sliced rows fail closed.

## Review Model

1. Freeze #317's merged target, packet manifest, 13-row universe, action graph,
   raw observation envelope, and artifact digests as review inputs.
2. Independently derive the same 13-row denominator from canonical tracked
   authority and acquire fresh read-only GitHub and Git observations.
3. Compare the independent reconstruction against #317 and record every
   discrepancy with evidence, severity, owner, route, disposition, and exact
   revision identity. Substantive packet changes require fresh exact-head
   review.
4. Review v0.92.1 readiness and its handoff into v0.92.2, including merge-only
   dependencies, number-free milestone opening authority, operator-controlled
   external actions, Runtime-v4 rebaseline boundaries, and asynchronous
   finish/cleanup semantics.
   Audit every canonical v0.92.1 planning surface and require the release tail
   to use these exact individual issue titles, in this exact serial order:

   1. TAIL-01 `Quality gate`
   2. TAIL-02 `Documentation review and external-review handoff`
   3. TAIL-03 `Publication finalization`
   4. TAIL-04 `Internal review`
   5. TAIL-05 `External / third-party review`
   6. TAIL-06 `Review findings remediation`
   7. TAIL-07 `Next-milestone planning`
   8. TAIL-08 `Next-milestone closeout plan`
   9. TAIL-09 `Next milestone review pass`
   10. TAIL-10 `Release ceremony`

   Bundled phase descriptions, punctuation changes, reordered nodes, aliases,
   or title variance fail validation. Detailed scope belongs in each issue's
   deliverables and acceptance criteria, not in its title.
   For all 31 creation-owned v0.92.1 issues, require exactly one bounded
   objective, one primary deliverable, and one independently verifiable result
   that proves that issue alone. Reject phase umbrellas, multiple unrelated
   outcomes, execution gated by administrative closeout, and results that can
   be established only by another issue. In particular, TAIL-06 owns finding
   disposition/remediation only; a later quality recheck belongs to the
   appropriate quality gate. TAIL-10 owns the operator-authorized release
   ceremony only; validation, notes, tag, cleanup, and asynchronous terminal
   reconciliation are inputs or separately owned results rather than bundled
   TAIL-10 deliverables.
5. Review v0.92.2 readiness and successor handoff without selecting or
   activating v0.93. Candidate successor work remains planning input only.
6. Hand a clean exact-head result to #319 after #318's reviewed green merge.
   #319 separately requires #315's completed reviewed remediation merge.

## Owned Paths

- `docs/reviews/v0.92/next-milestone-review-318`
- `docs/milestones/v0.92/review/V092_NEXT_MILESTONE_REVIEW_318.md`
- `.csdlc/evidence/318`
- `.csdlc/prepared/issues/318/validate-readiness-review.rb`
- `.csdlc/prepared/issues/318/test-validate-readiness-review.rb`
- `docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml`
- `docs/milestones/v0.92.1/PLANNED_ISSUE_CATALOG_v0.92.1.md`
- `docs/milestones/v0.92.1/WBS_v0.92.1.md`
- `docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml`

The issue-local design, diagram, cards, and typed state are lifecycle surfaces.
All #317 evidence, other milestone plans, GitHub state, Git topology, sibling
issue records, and legacy #5851 artifacts are read-only inputs.

## Validation Contract

The validator must compare the exact 13-row issue denominator to live GitHub,
validate recorded merge objects and ancestry, and require one evidence-backed
disposition for every finding. It must reject denominator drift, v0.93
activation, issue creation, closeout-as-gate serialization, and author
self-attestation.
It must also inspect all canonical v0.92.1 planning surfaces that declare or
render issue identities and enforce exact equality with the ten-title ordered
contract above plus the one-objective/one-primary-deliverable/one-verifiable-
result rule. A variance is an actionable review finding; only the four named
planning surfaces may be corrected by #318, and every correction remains
visible in the finding disposition. The script supplies structural guards; the
independent exact-head review supplies the semantic single-unit classification.

Focused negative fixtures must each mutate exactly one accepted field and run
through the real validator. They prove exact rejection of title variance,
a missing primary deliverable, creation-denominator drift, a missing canonical
issue row, release-tail reordering, dependency miswiring, and v0.93 activation.

## Rollback And Stop Conditions

Withdraw the review decision and regenerate its retained observations if any
identity, digest, ancestry, denominator, or disposition cannot be reproduced.
Stop before publication for an unresolved actionable finding, incomplete
observation truth, denominator disagreement, non-exact negative result, authority
ambiguity, or any need to mutate a successor milestone. Rollback requires no
remote mutation.

## Non-Goals

- No product remediation already owned by #315 or earlier work packages.
- No merge, finish, cleanup, tag, release, ceremony, sprint closeout, or issue
  closure.
- No issue creation or activation of v0.92.1, v0.92.2, or v0.93.
- No approval based only on #317 packet existence or author self-attestation.
- No use of legacy #5851 artifacts as current execution authority.
