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
    "id": "311-API-R5-P1-CUSTOM-MATRIX",
    "severity": "p1",
    "summary": "The public alternate-matrix path could emit a release pass without the #310 prerequisite or complete canonical packet.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R5-P1-PROOF-SEMANTICS",
    "severity": "p1",
    "summary": "Generic proof parsing did not bind behavior, commands, claims, revisions, and implementation paths to the exact row.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R5-P1-RULESET-MATCHING",
    "severity": "p1",
    "summary": "Ruleset wildcard applicability and authoritative check ordering were incomplete.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R5-P1-GIT-ENVIRONMENT",
    "severity": "p1",
    "summary": "Inherited Git and PATH environment could substitute repository authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R5-P2-CANDIDATE-BINDING",
    "severity": "p2",
    "summary": "The retained packet did not bind the evaluated candidate source commit/tree or reject product drift.",
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

Revision: Some("git-blake3:5a900f6100370e8f21d500cd7f39a2b73704fe69:c726a0e3b885c8935b0e100a16e15ee90a09cfb6c06ef358398c66f4fa83c637")

Reviewer: Some("fresh-session:9923e49d-ccc8-4078-996c-76968d15e736")

Result: changes_required
