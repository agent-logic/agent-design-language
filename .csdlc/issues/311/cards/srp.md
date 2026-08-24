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
    "id": "311-API-R4-P1-ROW-CONTRACT",
    "severity": "p1",
    "summary": "Accepted-row contracts did not bind exact owner, reviewed source bytes, implementation paths, proof semantics, or prevent packet reuse across rows.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R4-P1-CHECK-AUTHORITY",
    "severity": "p1",
    "summary": "Required-check validation used only one ruleset, omitted pagination and integration identity strictness, and allowed older successes to mask newer failures.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R4-P1-ROOT-SUBSTITUTION",
    "severity": "p1",
    "summary": "QUALITY_GATE_ROOT permitted repository-authority substitution.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R4-P2-WP21A-IDENTITY",
    "severity": "p2",
    "summary": "The #310 prerequisite checked ancestry against the candidate rather than live main and matched worktree state by ambiguous substrings.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "311-API-R4-P2-NEGATIVE-SEAM",
    "severity": "p2",
    "summary": "Reopened-issue and wrong-check-app negatives exercised a synthetic production observation command instead of the accepted-row authority function.",
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

Revision: Some("git-blake3:54c9a3dd08bc943d51f5ff854bf75c4bf6084440:7ad7d42647bc55d5f9f14ad055cc9e8660bf6cbf517e63b6a2aa64e33dadfac6")

Reviewer: Some("fresh-session:de89f81c-82d1-415d-bc53-a6f9cd142297")

Result: changes_required
