# Structured Review Prompt

Template: 1.0.0

Issue: 5835

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/issues/5835
.csdlc/prepared/issues/5835/design.md
.csdlc/prepared/issues/5835/diagram.mmd
.csdlc/evidence/5835
docs/milestones/v0.92/features/CROSS_POLIS_CONTINUITY_AND_MIGRATION_v0.92.md
docs/milestones/v0.92/design/CROSS_POLIS_CONTINUITY_TRANSFER_DESIGN_v0.92.md
docs/milestones/v0.92/NEXT_MILESTONE_HANDOFF_v0.92.md

## Prompts

- Does every movement-semantics row cite landed v0.92 evidence and preserve lineage?
- Can copied, ambiguous, or private state be misread as transferable continuity?
- Does any text absorb WP-04 infrastructure or v0.93 governance authority?
- Are the exact candidate paths and validation artifacts independently usable?

## Findings

[
  {
    "id": "P1-owned-path-boundary",
    "severity": "p1",
    "summary": "NEXT_MILESTONE_HANDOFF was modified although the approved design marks it read-only.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "P1-self-authorizing-transfer",
    "severity": "p1",
    "summary": "Caller-selected repository, revision, authority context, and redaction policy lack trusted anchors.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "P1-acip-remediation-authority",
    "severity": "p1",
    "summary": "The ACIP row and validator are not bound to issue #209 and its retained native remediation proof.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "P2-rollback-evidence",
    "severity": "p2",
    "summary": "Rollback is recorded completed without a retained rejected matrix or executable rollback proof.",
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

Revision: Some("git-blake3:9a438d9ca893e6afebb1d2505748feffe95f62df:7045903b3b5a34a7d387da7401cc76db717f35f57769f804e71323083e12419b")

Reviewer: Some("/root/sprint5_5835/review_5835_exact_head")

Result: changes_required
