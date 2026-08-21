# Structured Review Prompt

Template: 1.0.0

Issue: 308

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/evidence/308
.csdlc/issues/308
.csdlc/prepared/issues/308
.csdlc/locks/308.lock
adl/tools/validate_v092_demo_proof_coverage.py
adl/tools/test_v092_demo_proof_coverage.sh
docs/milestones/v0.92/DEMO_MATRIX_v0.92.md
docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md
docs/milestones/v0.92/V092_ACTIVATION_BRIDGE_LEDGER_v0.92.md
docs/milestones/v0.92/review/V092_DEMO_AEE_ARTIFACT_INDEX.md

## Prompts

- Do all accepted rows agree across matrix, coverage, activation, and artifact index at one exact revision?
- Does every accepted claim retain required positive and negative proof plus platform, credential, review, and non-claim truth?
- Does the validator reject every declared invalid class without manufacturing proof?
- Does the patch preserve WP-21/WP-21A and child ownership boundaries?
- Is the predecessor gate terminal, reconciled, and ancestral at the execution base?

## Findings

[
  {
    "id": "R1-P1-exact-revision-placeholder",
    "severity": "p1",
    "summary": "Accepted AEE-018 row uses current-issue-head instead of an immutable exact revision while the validator accepts any non-pending value.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R1-P1-review-state-premature",
    "severity": "p1",
    "summary": "Accepted AEE-018 row records pre-pr-review-required even though the exact-commit SRP is still pre_review with no reviewer or revision.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R1-P2-proof-validation-too-weak",
    "severity": "p2",
    "summary": "Accepted proof validation is path-existence-only and does not reject non-proof docs/scripts as retained run evidence or placeholder exact/review fields.",
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

Revision: Some("git-blake3:b9e8d45444d3d757950ec9b309ea531dcb06940b:e523b73e20a2fa02257dbbc0234ff7d94df5904740d41579da38bf1c2930f911")

Reviewer: Some("fresh-session:codex-cli-issue-308-review-b9e8d454")

Result: changes_required
