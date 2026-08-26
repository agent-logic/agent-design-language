# Validation Planning Prompt

Template: 1.0.0

Issue: 295

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/295/design.md

Diagram: .csdlc/prepared/issues/295/diagram.mmd

## Selected Lanes

[
  {
    "lane": "mechanical-compile-fallout-classifier",
    "proof_role": "Lane class cli_workflow: deterministic parser, mapping, receipt, negative, and end-to-end threshold integration proof; small resource; PR validation only and explicitly non-authoritative for release coverage.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/test_mechanical_coverage_fallout.sh"
    ],
    "parallel_group": "coverage-policy",
    "defer_reason": null
  },
  {
    "lane": "coverage-impact-regression",
    "proof_role": "Lane class cli_workflow: existing deterministic changed-source and authoritative-routing contract regression; small resource; preserves the 80 percent release gate and does not treat PR-fast evidence as authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_check_coverage_impact.sh"
    ],
    "parallel_group": "coverage-policy",
    "defer_reason": null
  },
  {
    "lane": "validation-selector-pvf",
    "proof_role": "Lane class cli_workflow: deterministic PVF selector inventory and route contract proof; small resource; records release gate status as non_authoritative_pr_validation.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_select_validation_lanes.sh"
    ],
    "parallel_group": "coverage-policy",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_mechanical_coverage_fallout.sh`
- `bash adl/tools/test_check_coverage_impact.sh`
- `bash adl/tools/test_select_validation_lanes.sh`

## Failure Semantics

Fail closed without classification on malformed or mixed diffs, unmapped tokens or files, incomplete hunk compile proof, incomplete owner behavioral proof, incomplete receipts, validation failure, or stale exact-head review; retain the ordinary 80 percent coverage gate.

## Handoff

Retain typed evidence before convergence.
