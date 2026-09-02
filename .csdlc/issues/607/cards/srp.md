# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/607
.csdlc/evidence/607/local-validation-resume-b1ca07fb2.json
adl/tools/run_issue607_warm_polis.sh
adl/tools/test_issue607_warm_polis.sh

## Prompts

- Can normal launch reach any compiler package manager Git mutable download or model pull path?
- Can Terraform destroy or a trap delete the persistent warm volumes?
- Are timing denominators complete and comparable?
- Can stale or cross-AZ volume content activate?
- Are #605 SSH private-Ollama IAM and cleanup invariants preserved?

## Findings

[
  {
    "id": "607-resume-r4-f1-output-keys",
    "severity": "p1",
    "summary": "Resume reads launch instance output keys rather than preparation instance output keys and therefore rejects every non-terminal interrupted preparation.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r4-f2-proof-denominator",
    "severity": "p2",
    "summary": "Focused helper tests do not yet execute the integrated resume, controller propagation, checkpoint reconciliation, and terminal recovery-refusal paths claimed by retained proof.",
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

- Prepared artifacts remain retained and no compute is running while the integrated no-paid resume harness is corrected.

## Review Result

Revision: Some("git-blake3:08921defa2fe9f973da4b7ff090432e007084fd4:b52e63cc4dfc92522ed0c449170f607f31bb2d231085432b1887454bcc90beec")

Reviewer: Some("subagent:issue_607_resume_r4_review")

Result: changes_required
