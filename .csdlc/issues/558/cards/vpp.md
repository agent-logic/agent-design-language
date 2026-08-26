# Validation Planning Prompt

Template: 1.0.0

Issue: 558

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/558/design.md

Diagram: .csdlc/prepared/issues/558/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-learner-replication",
    "proof_role": "Prove the exact governed learner replication test passes after behavior-preserving stabilization.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1200,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/558/validate-focused-proof.sh"
    ],
    "parallel_group": "558-serial-01",
    "defer_reason": null
  },
  {
    "lane": "lifecycle-evidence",
    "proof_role": "Verify issue-local lifecycle evidence exists when review, publication, and finish evidence become available.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/558/validate-lifecycle-evidence.sh"
    ],
    "parallel_group": "558-lifecycle",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/558/validate-focused-proof.sh`
- `bash .csdlc/prepared/issues/558/validate-lifecycle-evidence.sh`

## Failure Semantics

Fail closed on semantic Runtime changes, missing focused proof, missing exact-head API review, stale head, red required checks, or lifecycle topology drift.

## Handoff

Retain typed evidence before convergence.
