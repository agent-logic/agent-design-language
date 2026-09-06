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
    "proof_role": "Prove deterministic identity and fail-closed mismatch handling across production binary manifests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
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
    "lane": "runtime-focused",
    "proof_role": "Prove touched Runtime packages compile and focused tests pass.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/run_owner_validation_lane.sh",
      "runtime"
    ],
    "parallel_group": "runtime-generation",
    "defer_reason": null
  },
  {
    "lane": "live-wuji-a2a",
    "proof_role": "Prove owned readiness and a distinct Beacon-to-Ember delivery after generation install.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/707/validate-live-wuji-a2a.sh"
    ],
    "parallel_group": "runtime-generation-serial",
    "defer_reason": "Created during implementation and run only after local proof and review."
  },
  {
    "lane": "format-diff",
    "proof_role": "Reject formatting and range-diff hygiene defects.",
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
- `bash adl/tools/run_owner_validation_lane.sh runtime`
- `bash .csdlc/prepared/issues/707/validate-live-wuji-a2a.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on any identity disagreement, receipt or executable mismatch, competing listener, unhealthy rollout, ambiguous A2A recipient, or unresolved review finding; roll back to the retained known-good generation on live failure.

## Handoff

Retain typed evidence before convergence.
