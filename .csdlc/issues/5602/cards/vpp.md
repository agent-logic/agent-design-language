# Validation Planning Prompt

Template: 1.0.0

Issue: 5602

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5602/design.md

Diagram: .csdlc/prepared/issues/5602/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authoritative-coverage-contract",
    "proof_role": "Prove profile-only partition collection and unchanged combined reporting",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_run_authoritative_coverage_lane.sh"
    ],
    "parallel_group": "tooling-contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_run_authoritative_coverage_lane.sh`

## Failure Semantics

Fail closed if any test selector, threshold, explicit report, or non-PR failure behavior is weakened.

## Handoff

Retain typed evidence before convergence.
