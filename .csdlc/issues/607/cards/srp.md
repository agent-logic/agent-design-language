# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/607
.csdlc/evidence/607/local-validation-resume-b7b1ebd95.json
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
    "id": "607-resume-r6-f1-recovery-state",
    "severity": "p1",
    "summary": "Destructive recovery does not bind mutable Terraform state and live state-owned resources before destroy.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r6-f2-cost-concurrency",
    "severity": "p1",
    "summary": "Launch does not require valid aggregate preparation cost evidence and concurrent launches can race the shared ledger.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r6-f3-resume-campaign",
    "severity": "p2",
    "summary": "Resume terminal and incomplete paths do not bind the resource ledger campaign ID.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "607-resume-r6-f4-recovery-provenance",
    "severity": "p2",
    "summary": "Recovery campaign validation does not reconcile full stored authorization and consumed-marker provenance.",
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

- No compute is running while the final fail-closed state and cost guards are repaired.

## Review Result

Revision: Some("git-blake3:c389d85f35a9052399f91044ef1c81b980334d46:3405e6695ba3493eb548898f65bea6d6c11c3dd98937e198b8e8e4008bcc51f7")

Reviewer: Some("subagent:issue_607_resume_r6_review")

Result: changes_required
