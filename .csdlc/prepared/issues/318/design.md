# Issue 318 Design: v0.92 Next-Milestone Review Pass

Status: authored for independent design review before execution bind.

## Authority And Outcome

Issue #318 / WP-29 owns an independent, documentation-only review of the v0.92
terminal packet and the planned v0.92.1 and v0.92.2 handoffs. Its only opening
dependency is #317 / PR #474: independently reviewed, required checks green,
merged as `5b035094725d1872b48dda8692ef88f46487f37c`, and ancestral to `main`.
Typed finish, worktree cleanup, and administrative closeout are asynchronous
and do not gate this review.

This issue produces findings and dispositions. It does not implement a finding,
create successor issues, activate v0.92.1, v0.92.2, or v0.93, merge, release,
tag, finish, clean, or close any issue. Legacy issue #5851 is provenance only;
canonical lifecycle and publication authority remains issue #318.

## Independent Reconstruction

The review must not accept the #317 packet's denominator or conclusions by
self-attestation. It independently reconstructs the complete 13-row canonical
Sprint 6 universe from tracked wave and sprint authorities:

`#307`, `#308`, `#309`, `#310`, `#311`, `#312`, `#313`, `#314`, `#315`,
`#316`, `#317`, `#318`, and `#319`.

For each row it independently resolves the canonical issue, legacy provenance,
closing PR where applicable, base/head/merge identity, required-check and
review truth, typed phase/receipt availability, branch/worktree topology,
release dependency, classification, owner, and next action. It retains the raw
GitHub response bytes and recomputes their SHA-256 digests. Missing, duplicate,
extra, ambiguous, stale, unowned, sliced, or self-declared rows fail closed.

## Review Model

1. Freeze #317's merged target, packet manifest, 13-row universe, action graph,
   raw observation envelope, and artifact digests as review inputs.
2. Independently derive the same 13-row denominator from canonical tracked
   authority and acquire fresh read-only GitHub, typed lifecycle, and Git
   topology observations with retained raw provenance.
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
   For every creation-owned v0.92.1 issue, require exactly one bounded
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

The issue-local design, diagram, cards, and typed state are lifecycle surfaces.
All #317 evidence, milestone plans, GitHub state, Git topology, sibling issue
records, and legacy #5851 artifacts are read-only inputs.

## Validation Contract

The deterministic validator must consume retained raw observations, recompute
each raw-response digest before parsing, independently derive and compare the
13-row denominator, validate issue/PR/head/merge/check/review identity and
ancestry, prove dependency graphs acyclic, and require one evidence-backed
disposition for every finding. It must reject v0.93 activation, issue creation,
closeout-as-gate serialization, and author self-attestation.
It must also inspect all canonical v0.92.1 planning surfaces that declare or
render issue identities and enforce exact equality with the ten-title ordered
contract above plus the one-objective/one-primary-deliverable/one-verifiable-
result rule. Because milestone planning is read-only here, any variance is an
actionable review finding routed to its planning owner; #318 does not silently
normalize the source documents.

Focused negative fixtures must each mutate exactly one accepted field and run
through the real comparison or handoff classifier. Required cases include a
missing or duplicate row, stale or non-ancestral SHA, red or absent checks,
missing exact-head review, arbitrary/self-bound digest, active or dirty claimed
clean worktree, absent required receipt, partial release identity, duplicate
retry/mutation, dependency cycle, missing owner/disposition, premature
closeout, and v0.93 activation. Every case must return its exact expected
blocker; a generic failure is insufficient.

## Rollback And Stop Conditions

Withdraw the review decision and regenerate its retained observations if any
identity, digest, ancestry, denominator, or disposition cannot be reproduced.
Stop before publication for an unresolved actionable finding, incomplete raw
provenance, denominator disagreement, non-exact negative result, authority
ambiguity, or any need to mutate a successor milestone. Rollback requires no
remote mutation.

## Non-Goals

- No product remediation already owned by #315 or earlier work packages.
- No merge, finish, cleanup, tag, release, ceremony, sprint closeout, or issue
  closure.
- No issue creation or activation of v0.92.1, v0.92.2, or v0.93.
- No approval based only on #317 packet existence or author self-attestation.
- No use of legacy #5851 artifacts as current execution authority.
