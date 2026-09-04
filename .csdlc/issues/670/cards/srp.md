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
    "id": "670-R3-P1-AUTHORITY-BYPASS",
    "severity": "p1",
    "summary": "Caller-overridable project and preflight paths plus an unchecked snapshot-catalog var-file can bypass exact project, region, zone, and budget authority.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-R3-P1-BUDGET-ENFORCEMENT",
    "severity": "p1",
    "summary": "Caller-overridable pricing inputs and unlimited paid observation can allow the qualification to exceed the authorized USD 20 ceiling.",
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

- No paid GCP action or independent live inventory query was rerun by the reviewer.
- Validator execution may refresh ignored Terraform provider artifacts.

## Review Result

Revision: Some("git-blake3:70d8a1a2a4d19a6396851d213c32068ac8fa8efb:0f29ffe3dfbc842f79bc961ba55489aadbda7c0ca14ec8641af51f36411802b3")

Reviewer: Some("/root/issue_670_remediation_review")

Result: changes_required
