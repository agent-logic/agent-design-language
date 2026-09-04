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
    "id": "670-R2-P1-PROJECT-COUPLING",
    "severity": "p1",
    "summary": "Paid launch applies before proving the Terraform project and approved preflight target match the exact authorized company project.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-R2-P1-CLEAN-INIT",
    "severity": "p1",
    "summary": "The declared validator fails in a clean checkout because it does not initialize required Terraform modules and providers.",
    "actionable": true,
    "in_scope": true,
    "disposition": "open",
    "fix_revision": null,
    "route": null
  },
  {
    "id": "670-R2-P1-EVIDENCE-OVERCLAIM",
    "severity": "p1",
    "summary": "SOR overstates repeat-apply attachment proof and complete residual cleanup, and records a non-executable combined gcloud command.",
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

- The reviewer did not rerun paid GCP actions or independently query current cloud inventory.
- The reviewer did not inspect or mutate GitHub or PR state.

## Review Result

Revision: Some("git-blake3:da2d97603b1a24017be42a87135e0e1468638583:ca3b682f0bfc6e0a5ae2f17d81d650a9daeaf15dfa54e03d5389f84a27826d5f")

Reviewer: Some("/root/issue_670_final_review")

Result: changes_required
