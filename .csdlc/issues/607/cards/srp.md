# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/607
.csdlc/evidence/607/local-validation-resume-5fbd22933.json
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
    "id": "607-resume-r3-f1-partial-images",
    "severity": "p1",
    "summary": "Resume rejects a valid interrupted preparation with only one of two prepared AMIs instead of creating only the missing node image.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r3-f2-controller-generation",
    "severity": "p1",
    "summary": "Launch cannot use an older authorized prepared generation from a newer repaired controller HEAD.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r3-f3-terminal-checkpoint",
    "severity": "p1",
    "summary": "Preparation result and ledger completion are not committed as a reconcilable terminal checkpoint.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r3-f4-proof-claims",
    "severity": "p2",
    "summary": "SOR and evidence overstate interruption safety without regressions for partial images, controller-generation skew, and terminal checkpoint recovery.",
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

- Prepared AMIs, root snapshots, and warm volumes remain retained with no compute running while restart-safety is repaired.

## Review Result

Revision: Some("git-blake3:dc35154bb6bf109460f8d3286d4da6c7776efd87:55d40878b6c0a104c2f7578e5d74e64d6a6d5f25c9798a4bba354d3265929ef8")

Reviewer: Some("subagent:issue_607_resume_r3_review")

Result: changes_required
