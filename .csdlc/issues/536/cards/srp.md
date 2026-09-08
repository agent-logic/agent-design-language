# Structured Review Prompt

Template: 1.0.0

Issue: 536

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.92.1/evidence/sprint-8/SPRINT_8_CLOSEOUT_READINESS.md
.csdlc/prepared/issues/536/validate-sprint-closeout.rb

## Prompts

- Does the packet preserve exact child ownership and dependency truth?
- Are the podcast and Observatory lanes genuinely independent where marked parallel?
- Are #251 and provider-submission operator gates fail-closed?
- Can the umbrella close only after every current member has truthful terminal state or an explicit accepted disposition?

## Findings

[
  {
    "id": "P1-closeout-validator-loose-substrings",
    "severity": "p1",
    "summary": "The closeout validator used loose substring checks instead of enforcing exact child-to-review, PR, checks, merge, and no-PR mappings.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": null,
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:7d37f253d206e41bfebbacbcca25f6b3a4b59907:3cc0a3751c5f291cb7830a89d39108706db5b3a49253abe2acb90ecdf1075a54")

Reviewer: Some("codex:/root/review_536_closeout")

Result: changes_required
