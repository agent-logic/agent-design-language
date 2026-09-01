# Validation Planning Prompt

Template: 1.0.0

Issue: 511

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/511/design.md

Diagram: .csdlc/prepared/issues/511/diagram.mmd

## Selected Lanes

[
  {
    "lane": "information-contract",
    "proof_role": "Verify every designed view has named fields, source, state behavior, and consumer responsibility.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/511/validate-obs-a-contract.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "state-matrix",
    "proof_role": "Verify empty degraded recovery and revoked states are explicitly covered.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/511/validate-obs-a-states.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "accessibility-plan",
    "proof_role": "Verify keyboard and screen-reader flows are specified for each view and state.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/511/validate-obs-a-accessibility.sh"
    ],
    "parallel_group": "docs",
    "defer_reason": null
  },
  {
    "lane": "runtime-field-census",
    "proof_role": "Verify each field is sourced from current Runtime artifacts or rejected.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/511/validate-obs-a-runtime-fields.sh"
    ],
    "parallel_group": "runtime-census",
    "defer_reason": null
  },
  {
    "lane": "v3-local-canary",
    "proof_role": "Run the single csdlc binary local preparation path as non-authoritative cutover evidence.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "run",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--bin",
      "csdlc",
      "--",
      "local",
      "--request",
      ".csdlc/prepared/issues/511/v3-local-request.json",
      "--registry",
      "docs/templates/prompts/current.json",
      "--registrations",
      ".csdlc/prepared/issues/511/v3-local-registrations.json"
    ],
    "parallel_group": "cutover-canary",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/511/validate-obs-a-contract.sh`
- `bash .csdlc/prepared/issues/511/validate-obs-a-states.sh`
- `bash .csdlc/prepared/issues/511/validate-obs-a-accessibility.sh`
- `bash .csdlc/prepared/issues/511/validate-obs-a-runtime-fields.sh`
- `cargo run --locked --manifest-path csdlc-v3/Cargo.toml --bin csdlc -- local --request .csdlc/prepared/issues/511/v3-local-request.json --registry docs/templates/prompts/current.json --registrations .csdlc/prepared/issues/511/v3-local-registrations.json`

## Failure Semantics

Fail closed on invented Runtime fields, incomplete accessibility denominator, production implementation drift, or v3 authority overclaim.

## Handoff

Retain typed evidence before convergence.
