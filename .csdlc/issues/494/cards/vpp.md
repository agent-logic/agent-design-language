# Validation Planning Prompt

Template: 1.0.0

Issue: 494

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/494/design.md

Diagram: .csdlc/prepared/issues/494/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-gcp-e-gpu-smoke-packet",
    "proof_role": "Proves #494 design packet readiness, #493 terminal dependency cache and ancestry, owned-path boundaries, paid-authorization/cost cap, exact input capture plan, GPU proof plan, telemetry plan, and cleanup-zero plan.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh",
      "--lane=prebind"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-review-readiness",
    "proof_role": "Proves #494 has an issue-owned executable validator and bounded review scope before design approval; this does not claim implementation exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh",
      "--lane=packet"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=prebind`
- `bash .csdlc/prepared/issues/494/validate-gcp-e-gpu-smoke.sh --lane=packet`

## Failure Semantics

Fail closed if paid authorization, quota/capacity, GPU inference, telemetry, cost ceiling, deadline, or zero-resource cleanup cannot be proven without credential disclosure.

## Handoff

Retain typed evidence before convergence.
