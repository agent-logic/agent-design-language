# Validation Planning Prompt

Template: 1.0.0

Issue: 226

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/226/design.md

Diagram: .csdlc/prepared/issues/226/diagram.mmd

## Selected Lanes

[
  {
    "lane": "selector-contract",
    "proof_role": "Prove narrow path mapping and retained unknown-path failure.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "ci-path-policy-contract",
    "proof_role": "Prove focused PR routing avoids slow and authoritative coverage escalation.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash adl/tools/test_select_validation_lanes.sh`
- `bash adl/tools/test_ci_path_policy.sh`

## Failure Semantics

Fail closed on any unmapped path or unexpected slow/full-coverage selection.

## Handoff

Retain typed evidence before convergence.
