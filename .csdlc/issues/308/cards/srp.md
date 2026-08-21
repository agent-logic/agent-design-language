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
    "id": "R3-P1-predecessor-gate-reconciled-evidence-missing",
    "severity": "p1",
    "summary": "The predecessor gate evidence and validator prove terminal/ancestor facts but not labeled reconciled state or retained terminal evidence artifacts for #256, #340, #341, and legacy #5839.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R3-P1-demo-matrix-owner-revision-command-underchecked",
    "severity": "p1",
    "summary": "The demo matrix lacks owner, exact revision, and command columns, so the validator cannot enforce the promised four-surface owner/status/revision/command join.",
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

- Review was read-only and bounded to exact commit bcf420ec823682f86a2c713ddf1575c98c094398; reviewer ignored post-HEAD review-assignment metadata dirt and did not rerun mutating negative-suite fixtures.

## Review Result

Revision: Some("git-blake3:bcf420ec823682f86a2c713ddf1575c98c094398:1df0b08259eae1f8506f9680ae7e34e76a8c1360be9f5ce7682c32dca4b75491")

Reviewer: Some("fresh-session:codex-cli-issue-308-review-bcf420ec")

Result: changes_required
