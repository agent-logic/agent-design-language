# Validation Planning Prompt

Template: 1.0.0

Issue: 490

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/490/design.md

Diagram: .csdlc/prepared/issues/490/diagram.mmd

## Selected Lanes

[
  {
    "lane": "hierarchy-readback",
    "proof_role": "Run read-only gcloud identity, project, and billing decision readbacks.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/gcp-a/run-readonly-decision-readbacks.sh"
    ],
    "parallel_group": "gcp-readonly",
    "defer_reason": null
  },
  {
    "lane": "decision-denominator",
    "proof_role": "Validate decision register, redaction, no-mutation posture, and quota/cost language.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/490/validate-gcp-a-decision.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "typed-review-publication",
    "proof_role": "Prove current typed issue integrity before exact-head review and closing publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "490"
    ],
    "parallel_group": "lifecycle",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash docs/milestones/v0.92.1/evidence/cloud/gcp-a/run-readonly-decision-readbacks.sh`
- `bash .csdlc/prepared/issues/490/validate-gcp-a-decision.sh`
- `.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 490`

## Failure Semantics

Fail closed on identity or billing ambiguity, mutation requirement, missing cost ceiling, credential exposure, scope drift, stale review, or red CI.

## Handoff

Retain typed evidence before convergence.
