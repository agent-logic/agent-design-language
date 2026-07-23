# Validation Planning Prompt

Template: 1.0.0

Issue: 5332

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5332/design.md

Diagram: .csdlc/prepared/issues/5332/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-ilpp-classifier-unit",
    "proof_role": "Prove complete-signature classification, semantic progress reset, readonly progression, and normal-start behavior without Unity",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-8",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh"
    ],
    "parallel_group": "unity-ilpp-static",
    "defer_reason": null
  },
  {
    "lane": "unity-ilpp-staged-reproduction",
    "proof_role": "Reproduce or clear the ILPP loop in one #4741-approved staged batch mode",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"
    ],
    "parallel_group": "unity-ilpp-live",
    "defer_reason": "Run only after #4741 provides exact safe staged-project ownership and the deterministic classifier lane passes."
  },
  {
    "lane": "unity-ilpp-diagnostic-matrix",
    "proof_role": "Retain one-variable environment comparisons that isolate the ILPP failure owner",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 1800,
    "budget_tokens": 8000,
    "argv": [
      "bash",
      "adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh"
    ],
    "parallel_group": "unity-ilpp-live",
    "defer_reason": "Execute only the cells still required after the baseline reproduction; do not run redundant Unity comparisons."
  },
  {
    "lane": "unity-ilpp-diff-hygiene",
    "proof_role": "Prove bounded text and shell hygiene",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "unity-ilpp-static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh`
- `bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh`
- `bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption.sh`
- `git diff --check`

## Failure Semantics

Fail closed on incomplete signature attribution, arbitrary total-runtime ceilings, multi-variable diagnosis, unsafe staging, secret-bearing host evidence, adjacent Unity scope, or unsupported root-cause and readiness claims.

## Handoff

Retain typed evidence before convergence.
