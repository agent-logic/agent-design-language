# Structured Review Prompt

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

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

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- No live GCP or billing-export query was performed during final review; tracked redacted receipts are the live evidence boundary.

## Review Result

Revision: Some("git-blake3:5c2746a74c86fcb0bb08b661d818e473bed37f76:2b3cee529c23a21c0930fa1d3a74dc964fde75084479637f1a7549bf949eccab")

Reviewer: Some("/root/issue_670_release_truth_review")

Result: pass
