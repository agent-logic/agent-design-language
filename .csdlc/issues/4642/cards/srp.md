# Structured Review Prompt

Template: 1.0.0

Issue: 4642

Repository: danielbaustin/agent-design-language

Card: srp

Status: pre_phase

## Scope

docs/milestones/v0.91.7/DEMO_MATRIX_v0.91.7.md
docs/milestones/v0.91.7/FEATURE_PROOF_COVERAGE_v0.91.7.md
docs/milestones/v0.91.7/review/V0917_WP15_DEMO_CONVERGENCE_4642.md
docs/milestones/v0.91.7/review/wp15_demo_convergence_4642/feature_proof_coverage_4642.json
.csdlc/issues/4642

## Prompts

- Does the issue stay within its WP scope?
- Are claims supported by retained or fresh evidence?
- Are skipped and unproven surfaces explicit?
- Are sibling WP and release/activation non-claims preserved?

## Findings

[
  {
    "id": "F-4642-1",
    "severity": "p2",
    "summary": "Markdown evidence paths in the feature coverage table used bare filenames instead of repo-resolvable paths.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:947c8c44a3322aa8f6133be6329d96bcb6be9df1:576978b0ba1cb1b7119315a0f46e90dc36adf9f6dc7d7b34b05f39c33920c577",
    "route": null
  },
  {
    "id": "F-4642-2",
    "severity": "p2",
    "summary": "WP-22 #4649 was listed as remaining next-gate work despite live issue truth showing it closed.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:947c8c44a3322aa8f6133be6329d96bcb6be9df1:576978b0ba1cb1b7119315a0f46e90dc36adf9f6dc7d7b34b05f39c33920c577",
    "route": null
  },
  {
    "id": "F-4642-3",
    "severity": "p3",
    "summary": "The JSON classification vocabulary omitted several classification values used by coverage rows.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:947c8c44a3322aa8f6133be6329d96bcb6be9df1:576978b0ba1cb1b7119315a0f46e90dc36adf9f6dc7d7b34b05f39c33920c577",
    "route": null
  },
  {
    "id": "F-4642-4",
    "severity": "p3",
    "summary": "The machine-readable WP-11 current issue truth omitted closed follow-ons #4912, #5096, and #5136 named by the Markdown audit.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:947c8c44a3322aa8f6133be6329d96bcb6be9df1:576978b0ba1cb1b7119315a0f46e90dc36adf9f6dc7d7b34b05f39c33920c577",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- none

## Review Result

Revision: Some("git-blake3:947c8c44a3322aa8f6133be6329d96bcb6be9df1:576978b0ba1cb1b7119315a0f46e90dc36adf9f6dc7d7b34b05f39c33920c577")

Reviewer: Some("subagent:019f745c-23d3-7820-8e91-084314fbc95f")

Result: pass
