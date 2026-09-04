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
infra/gcp/workloads/modules/two-node-ollama-runtime/variables.tf
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
    "id": "670-r4-p1-deadline-termination",
    "severity": "p1",
    "summary": "Deadline wrappers can block forever after SIGTERM and prevent mandatory cleanup.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-r4-p1-catalog-cleanup-create",
    "severity": "p1",
    "summary": "Preparation cleanup can run an unbounded create-capable catalog apply after the deadline.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-r4-p1-partial-launch-cleanup",
    "severity": "p1",
    "summary": "Launch cleanup reads state outputs before destroy and can exit before teardown or residual verification after partial apply.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-r4-p1-live-proof-scope",
    "severity": "p1",
    "summary": "Retained live v1 receipts do not prove the remediated v2 failure controls and the validator does not make that boundary explicit.",
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

- Publication remains blocked until all four failure-path findings are fixed and freshly reviewed.

## Review Result

Revision: Some("git-blake3:65e2da4774672ee80201cd44b0bc8c6471dc614e:1360589323d130e538a63ba88b20547ff4ae2feb3703dac0506850a50b9e9e77")

Reviewer: Some("/root/issue_670_terminal_review")

Result: changes_required
