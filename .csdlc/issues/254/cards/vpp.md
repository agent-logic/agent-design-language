# Validation Planning Prompt

Template: 1.0.0

Issue: 254

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/254/design.md

Diagram: .csdlc/prepared/issues/254/diagram.mmd

## Selected Lanes

[
  {
    "lane": "ci-runtime-contracts",
    "proof_role": "Focused workflow contract proof for coverage topology and heavy-runner allocation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "adl/tools/test_ci_runtime_contracts.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "ci-path-policy-contract",
    "proof_role": "Focused path-policy contract proof for PR-fast/full-coverage behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      "adl/tools/test_ci_path_policy.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "workflow-policy",
    "proof_role": "Machine-readable workflow policy validation for required and optional job topology.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 300,
    "argv": [
      "ruby",
      "adl/tools/validate_ci_workflow_policy.rb"
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

- `bash adl/tools/test_ci_runtime_contracts.sh`
- `bash adl/tools/test_ci_path_policy.sh`
- `ruby adl/tools/validate_ci_workflow_policy.rb`

## Failure Semantics

Fail closed on missing summaries/provenance, aggregate Rust coverage invocation, heavy aggregate runner allocation, failed focused validation, stale review, or typed lifecycle ambiguity.

## Handoff

Retain typed evidence before convergence.
