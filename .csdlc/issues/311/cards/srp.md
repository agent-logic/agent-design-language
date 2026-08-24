# Structured Review Prompt

Template: 1.0.0

Issue: 311

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/311
.csdlc/prepared/issues/311
.csdlc/evidence/311
docs/milestones/v0.92/QUALITY_GATE_v0.92.md
docs/milestones/v0.92/WP_EXECUTION_READINESS_v0.92.md
docs/reviews/v0.92/quality-gate-311

## Prompts

- Does the matrix enumerate every indexed v0.92 feature and required supporting critical path exactly once?
- Does each accepted row independently prove repository identity, implementation, review, merge, ancestry, validation, negative, integration, platform, and typed-terminal truth?
- Do the validator and negative suite reject every prohibited evidence class without accepting self-attested JSON?
- Does any unresolved blocker incorrectly unlock WP-23, WP-25, or a release claim?
- Are all changed paths within #311 ownership and all dependency work read-only?

## Findings

[
  {
    "id": "311-API-R6-P1-CANDIDATE-REBIND",
    "severity": "p1",
    "summary": "Candidate-source authority was packet-selected rather than pinned to the approved commit and tree.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R6-P1-CHECK-TIE",
    "severity": "p1",
    "summary": "Same-timestamp authorized check runs with equal conclusions were not rejected as ambiguous.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R6-P1-TYPED-SUBPROCESS-ENV",
    "severity": "p1",
    "summary": "Typed terminal authority subprocesses inherited Git and PATH substitution state.",
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

- none

## Review Result

Revision: Some("git-blake3:b67e7529306c909f32b8545170ad306fcc4f888d:2c2d21bd30250725522b108c8e236bf0914f78fbe677e219319ee2b69be538d6")

Reviewer: Some("fresh-session:da06b704-9a43-4b67-8935-8fcf1eba0eec")

Result: changes_required
