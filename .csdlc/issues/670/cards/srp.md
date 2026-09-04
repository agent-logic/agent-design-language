# Structured Review Prompt

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/prepared/issues/670
.csdlc/evidence/670/live
infra/gcp/workloads/modules/two-node-ollama-runtime/main.tf
infra/gcp/workloads/warm-polis

## Prompts

- Does every paid action target only the exact authorized company project and remain within the USD 20.00 ceiling?
- Are the two snapshots sealed, versioned, and sufficient for launch without downloads or builds?
- Does the live proof establish private Runtime-to-Ollama behavior, both resident models, and a real agent/tool path?
- Are all timing denominators measured from authoritative events rather than inferred?
- Does cleanup prove every issue-owned VM and disk is absent while exactly the two intended snapshots remain?

## Findings

[
  {
    "id": "670-r3-p1-absolute-budget-deadline",
    "severity": "p1",
    "summary": "The absolute eight-hour budget deadline is not enforced across blocking paid Terraform operations or all paid guests.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-r3-p1-cleanup-failure",
    "severity": "p1",
    "summary": "Failure cleanup suppresses Terraform destroy failures without durable cleanup-pending evidence or mandatory residual verification.",
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

- Publication remains blocked until absolute deadline enforcement and fail-closed cleanup evidence are implemented and freshly reviewed.

## Review Result

Revision: Some("git-blake3:2745e3b1d60c585081f9baff44375c46a05860c2:97e0e567af595fc0e31c2612d6fa9f4f899754f7d8d80f5409ff3ee3d89c02db")

Reviewer: Some("/root/issue_670_budget_review")

Result: changes_required
