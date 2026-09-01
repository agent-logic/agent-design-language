# Validation Planning Prompt

Template: 1.0.0

Issue: 608

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/608/design.md

Diagram: .csdlc/prepared/issues/608/diagram.mmd

## Selected Lanes

[
  {
    "lane": "vertex-provider-focused",
    "proof_role": "Prove endpoint derivation, request body rendering, trust policy, invalid config behavior, formatting, diff hygiene, and package compile through the issue-owned validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/608/validate-provider.sh"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "vertex-provider-live",
    "proof_role": "Prove the native provider against approved company GCP Vertex endpoints without endpoint overrides after the provider binary is built.",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      ".csdlc/evidence/608/live-vertex/run-live-provider-proof.sh"
    ],
    "parallel_group": "live",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/608/validate-provider.sh`
- `bash .csdlc/evidence/608/live-vertex/run-live-provider-proof.sh`

## Failure Semantics

Fail closed on credential exposure, endpoint trust ambiguity, missing live proof, or unsupported simultaneous thinking config.

## Handoff

Retain typed evidence before convergence.
