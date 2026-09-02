# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/607
.csdlc/evidence/607/local-validation-resume-f59fcbf6a.json
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
    "id": "607-resume-f1-idempotence",
    "severity": "p1",
    "summary": "Interrupted completion can create duplicate retained sealed snapshots and an unreferenced temporary restore while understating cost.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-f2-identity",
    "severity": "p1",
    "summary": "Resume does not fully bind checkout, state, plans, artifacts, ledger, and consumed authorization to the original exact campaign.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-f3-api-errors",
    "severity": "p1",
    "summary": "Permanent AWS API errors are retried forever instead of failing while only healthy transitional resource states wait indefinitely.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-f4-image-window",
    "severity": "p2",
    "summary": "Preservation starts after both create-image calls, leaving an interruption window that destroys a partially created image and makes the consumed run non-resumable.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-f5-tests-proof",
    "severity": "p2",
    "summary": "Tests do not execute resume rejection, idempotence, permanent API failure, preservation, and cost accuracy, and retained SOR validation is stale.",
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

- The available prepared AMIs and completed root snapshots remain preserved while remediation and fresh review proceed.

## Review Result

Revision: Some("git-blake3:b5debc6abc77eef40361abde362ea0458ef52668:f22cd6960bde70585c3737a7879c8c775610002b350a75f6fb3b77b9d956c945")

Reviewer: Some("subagent:issue_607_resume_review")

Result: changes_required
