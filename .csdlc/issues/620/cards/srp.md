# Structured Review Prompt

Template: 1.0.0

Issue: 620

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92.2/**
.csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh

## Prompts

- Does the package include every canonical document and first-class feature surface?
- Does each planned issue describe one bounded unit with a concrete result?
- Is every relevant TBD source explicitly scheduled, deferred, preserved as reference or provenance, or flagged for operator decision?
- Does the plan avoid duplicating existing and completed issues?
- Does the package remain number-free and refrain from opening the milestone?

## Findings

[
  {
    "id": "620-R1",
    "severity": "p1",
    "summary": "MLX admission lacked a recorded operator decision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "620-R2",
    "severity": "p1",
    "summary": "TAIL-02 lost canonical documentation-review and external-review-handoff semantics.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "620-R3",
    "severity": "p1",
    "summary": "OPS-AWS did not reconcile completed issue #484 and risked duplicate inventory scope.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "620-R4",
    "severity": "p2",
    "summary": "Issue-combination language contradicted the one-result-per-issue contract.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The candidate is behind origin/main; live publication must recheck base mergeability and CI.

## Review Result

Revision: Some("git-blake3:2d912c9b27387af763b6e30e608b38abb55c9a1c:c36904ec879aa3087b0b2bf9d1a1ada4bc95b936f48a50ef06c9a96edb50a511")

Reviewer: Some("fresh-session:620-docs-2d912c9b2")

Result: changes_required
