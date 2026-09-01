# Validation Planning Prompt

Template: 1.0.0

Issue: 531

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/531/design.md

Diagram: .csdlc/prepared/issues/531/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint-3-closeout-static",
    "proof_role": "Prove the Sprint 3 closeout artifact names roster v4, all child issues, live dispositions, residual-risk boundaries, and no paid/cloud execution claim.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/sprint-3/validate-sprint-3-closeout.sh"
    ],
    "parallel_group": "sprint-3-closeout",
    "defer_reason": "Validator is produced with the sprint closeout artifact."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors in sprint closeout records and evidence.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "sprint-3-review-gate",
    "proof_role": "Require a fresh sprint-end review with no actionable findings before typed publication.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/sprint-3/validate-sprint-3-review-gate.sh"
    ],
    "parallel_group": "review",
    "defer_reason": "Validator is produced after sprint-end review truth is recorded."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash docs/milestones/v0.92.1/evidence/cloud/sprint-3/validate-sprint-3-closeout.sh`
- `git diff --check`
- `bash docs/milestones/v0.92.1/evidence/cloud/sprint-3/validate-sprint-3-review-gate.sh`

## Failure Semantics

Fail closed on stale roster truth, ambiguous child disposition, unsupported paid/cloud claim, dirty topology, or typed validation failure.

## Handoff

Retain typed evidence before convergence.
