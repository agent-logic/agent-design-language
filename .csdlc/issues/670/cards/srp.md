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
    "id": "670-r5-p1-provider-orphan",
    "severity": "p1",
    "summary": "Deadline enforcement can orphan a TERM-resistant provider child after its Terraform parent exits.",
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

- Publication remains blocked until paid operations execute in a fully terminated process group and the provider-child case is tested.

## Review Result

Revision: Some("git-blake3:5046c2f6398644282da73292dba46416c3b5ba6c:454084a471a4d7444a5d2049ca93ad1fdbe5e69e1f11efdb2707449b860a8d7a")

Reviewer: Some("/root/issue_670_release_review")

Result: changes_required
