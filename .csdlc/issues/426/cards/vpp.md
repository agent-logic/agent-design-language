# Validation Planning Prompt

Template: 1.0.0

Issue: 426

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/426/design.md

Diagram: .csdlc/prepared/issues/426/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csmctl-linux-lifecycle",
    "proof_role": "acceptance",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_csmctl_linux_backend.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_csmctl_linux_backend.sh`

## Failure Semantics

Fail closed on unsupported OS, ambiguous PID ownership, readiness failure, or review findings.

## Handoff

Retain typed evidence before convergence.
