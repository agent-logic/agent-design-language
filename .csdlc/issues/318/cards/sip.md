# Structured Intent Prompt

Template: 1.0.0

Issue: 318

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Independently review the complete v0.92 terminal packet and the v0.92.1/v0.92.2 handoff without activating a successor milestone.

## Required Outcome

Produce an evidence-bound exact-head review that independently reconstructs all 13 canonical Sprint 6 rows, resolves every finding, and truthfully assesses v0.92.1 and v0.92.2 readiness.

## Scope

- docs/reviews/v0.92/next-milestone-review-318
- docs/milestones/v0.92/review/V092_NEXT_MILESTONE_REVIEW_318.md
- .csdlc/evidence/318
- .csdlc/prepared/issues/318/validate-readiness-review.rb

## Authority

- Canonical #318 is the sole WP-29 lifecycle and publication authority; legacy #5851 is provenance only.
- Reviewed green merge ancestry gates execution; typed finish, cleanup, and closeout are asynchronous and non-gating.
- The issue reviews v0.92.1 and v0.92.2 planning but creates no issues and activates no successor milestone.

## Assumptions

- none

## Operator Constraints

- Never write tracked changes on main.
- Bind only after independent design approval.
- Retain raw observations and independently recompute their digests.
- Do not merge, release, tag, finish, clean, close, or activate v0.93.
