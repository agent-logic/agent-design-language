# WP-21 Next-Milestone Planning Review

Issue: #4648
Review issue: #5403
Status: reviewed with records finding; historical planning superseded
Remediation: shared records issue #5406

## Findings

### P1: The required WP-22 planning review changed scope and never reviewed WP-21

#4648 required its v0.92 planning inputs to be complete and ready for review.
PR #4901 changed 38 planning files across `docs/planning/`, v0.91.7, and v0.92.
The designated WP-22 issue #4649 later closed through PR #5142, but that PR's
scope was closed-sprint record reconciliation: its changed files are the sprint
review register, closed-sprint review, WP-02 packet, and tooling closeout packet.
It did not review the #4901 planning diff. The milestone checklist still leaves
`v0.92 next-milestone planning reviewed` unchecked at
`docs/milestones/v0.91.7/MILESTONE_CHECKLIST_v0.91.7.md:95`.

Impact: a broad next-milestone planning rewrite closed without the independent
planning review required by its own issue graph. At the time, downstream v0.92
activation documents could consume unreviewed assumptions.

Disposition: fixed for historical-consumption purposes by this review and by
the later #5383 v0.91.8 planning package, which now places a reviewed v0.91.8
bridge before v0.92. Do not mark the old checklist row complete; annotate it as
superseded when the canonical register path is available.

### P2: The merged planning review cannot reconstruct its lifecycle evidence

PR #4901 says its pre-PR subagent found three documentation-truth defects and
that all were fixed, but the only exact SRP/SOR references are ignored `.adl`
paths. Those records are not present in a clean checkout. PR #4901 has no
formal GitHub review decision.

Impact: reviewers cannot identify the original three findings, confirm their
exact dispositions, or compare the reviewed revision with the merged revision.

Disposition: open as part of the cross-cutting typed-v2 records-retention issue
also identified by the WP-12 and WP-13 reviews.

## Scope Coverage

- Reviewed #4648 issue contract and live closure state.
- Reviewed PR #4901 changed-file inventory, description, validation account,
  and absence of a formal GitHub review decision.
- Reviewed #4649 / PR #5142 scope against the WP-22 review obligation.
- Reviewed current v0.91.7 handoff and v0.91.8 milestone routing.
- No product code or dependency manifest changed in #4648.

## Current Planning Truth

The original direct v0.91.7-to-v0.92 planning route is no longer current.
`docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md:18` records #5383 and v0.91.8
as the bridge prerequisite. The v0.91.8 WBS assigns fresh WP-21/WP-21A/WP-22
issues #5362, #5355, and #5359 for planning, closeout planning, and independent
review. This review therefore does not promote or reactivate #4901's original
v0.92 candidate package.

## Validation And Limits

- Review was documentation, lifecycle, closeout, and planning-architecture
  focused.
- No tests were rerun because #4648 changed planning documents only.
- Historical ignored SRP/SOR content was unavailable and was not reconstructed
  from prose summaries.
- Both findings are review-discovered; no test-discovered defect is counted.

## Review Result

The original planning-review gate was missed. Current release risk is bounded
because #5383 and v0.91.8 supersede direct consumption, but durable lifecycle
review evidence remains an open cross-cutting records defect.
