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
    "id": "311-API-R3-P1-ROW-BINDING",
    "severity": "p1",
    "summary": "Accepted evidence was reusable across unrelated feature and critical-path rows without proving the reviewed contract or owner binding.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R3-P1-TYPED-AUTHORITY",
    "severity": "p1",
    "summary": "The accepted path did not sufficiently bind the canonical typed index, review assignment, reviewer, revision, and SOR authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R3-P1-GITHUB-AUTHORITY",
    "severity": "p1",
    "summary": "Live GitHub validation omitted issue closure and required-check GitHub App identity.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R3-P1-PACKET-ATOMICITY",
    "severity": "p1",
    "summary": "The complete packet did not fully bind lane uniqueness, log digests, exact denominator identity, blocker reasons, and the WP-21A source revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R3-P2-WP21A-PREREQUISITE",
    "severity": "p2",
    "summary": "Execution evidence had not consumed #310 terminal reconciliation and cleanup as the approved hard prerequisite.",
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

Revision: Some("git-blake3:ce99e1736bb2ccd2979e3ba38a7b4983f1f198e6:2dc74e203ec42fb7c1420d6fbd9d044ce9a28bedfad17394cec6668593c04d98")

Reviewer: Some("fresh-session:89119b99-dc15-475a-8908-3740b9c649fe")

Result: changes_required
