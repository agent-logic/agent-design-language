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
    "id": "R2-P1-predecessor-gate-evidence-unlabeled",
    "severity": "p1",
    "summary": "Retained predecessor gate evidence does not label or prove terminal/reconciled/ancestral truth for #256, #340, #341, and legacy #5839 as required by the issue design.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "R2-P1-activation-ledger-not-structured",
    "severity": "p1",
    "summary": "The validator only checks that the activation ledger mentions WP-20 instead of parsing and comparing activation ledger ownership/status/revision against the matrix, coverage, and artifact index.",
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

- Review was read-only and bounded to exact commit 5b4992ab6d97c79f141c22369f8f1a2f9eef6f9a; reviewer ignored post-HEAD review-assignment metadata dirt.

## Review Result

Revision: Some("git-blake3:5b4992ab6d97c79f141c22369f8f1a2f9eef6f9a:667cac397d879c31afab79b9c38c0b56565cb02fc84cb6a01676485dbb8dbd41")

Reviewer: Some("fresh-session:codex-cli-issue-308-review-5b4992ab")

Result: changes_required
