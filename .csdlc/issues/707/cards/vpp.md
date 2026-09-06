# Validation Planning Prompt

Template: 1.0.0

Issue: 707

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/707/design.md

Diagram: .csdlc/prepared/issues/707/diagram.mmd

## Selected Lanes

[
  {
    "lane": "config-generation-cross-binary",
    "proof_role": "Prove deterministic install-generation handling and fail-closed artifact mismatch behavior; extend this retained target with cross-binary config identity coverage during implementation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/test_runtime_v3_generation_install.sh"
    ],
    "parallel_group": "runtime-generation",
    "defer_reason": null
  },
  {
    "lane": "format-diff",
    "proof_role": "Reject formatting and exact-range whitespace defects before review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "runtime-generation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_runtime_v3_generation_install.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on any identity disagreement, receipt or executable mismatch, competing listener, unhealthy rollout, ambiguous A2A recipient, or unresolved review finding; roll back to the retained known-good generation on live failure.

## Handoff

Retain typed evidence before convergence.
