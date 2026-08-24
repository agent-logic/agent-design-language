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
    "id": "311-R1-P1-ACCEPTED-AUTHORITY",
    "severity": "p1",
    "summary": "Accepted rows did not independently authenticate typed terminal, GitHub, review, check, implementation-path, and proof-artifact authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-R1-P1-NEGATIVE-DENOMINATOR",
    "severity": "p1",
    "summary": "The negative suite did not independently exercise every forged-evidence class promised by the design.",
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

Revision: Some("git-blake3:02f86001a1936bf2e333f58e843e49fd9028cdaf:337195baca0d96511375a3b5ff2d54578d48f7a0f50d6df85c16194b20b8f6b5")

Reviewer: Some("fresh-session:3c216425-3a6e-47e4-a2d8-f11d4e0495aa")

Result: changes_required
