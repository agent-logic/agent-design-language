# Structured Review Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/issues/607
adl/tools/issue607_validate_saved_plan.sh
adl/tools/run_issue607_warm_polis.sh
adl/tools/test_issue607_warm_polis.sh
docs/operations/cloud/aws/shepherd-gpu-proof/README.md
infra/aws/runtime/gpu-proof/warm-storage/preparation/gpu-user-data.sh.tftpl
infra/aws/runtime/gpu-proof/warm-storage/preparation/runtime-user-data.sh.tftpl

## Prompts

- Can normal launch reach any compiler package manager Git mutable download or model pull path?
- Can Terraform destroy or a trap delete the persistent warm volumes?
- Are timing denominators complete and comparable?
- Can stale or cross-AZ volume content activate?
- Are #605 SSH private-Ollama IAM and cleanup invariants preserved?

## Findings

[
  {
    "id": "I607-R5-F1",
    "severity": "p1",
    "summary": "Preparation publishes success before cloud-init and SSH host-key image hygiene completes.",
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

- Live paid AWS preparation and repeated warm-launch timing remain pending.

## Review Result

Revision: Some("git-blake3:f0aed7a20af45eec53a301a08c601da3815d9261:116917425bcc1e693831d8befad217d3e8648b42fc136e50831234c8b08562e9")

Reviewer: Some("subagent:issue_607_final_exact_review")

Result: changes_required
