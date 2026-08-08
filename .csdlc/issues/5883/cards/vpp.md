# Validation Planning Prompt

Template: 1.0.0

Issue: 5883

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5883/design.md

Diagram: .csdlc/prepared/issues/5883/diagram.mmd

## Selected Lanes

[
  {
    "lane": "create-contract",
    "proof_role": "Prove real csdlc-issue create validation, atomicity, idempotence, and conflict behavior.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2400,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "installed-inventory",
    "proof_role": "Prove csdlc-init is absent from declared and installed sets and its reappearance fails.",
    "acceptance_ids": [
      "AC-2",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 2600,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "operator-contract",
    "proof_role": "Prove active skill and documentation routes name only csdlc-issue create.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1400,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a",
      "operator_workflow_names_only_current_creation_route"
    ],
    "parallel_group": "contracts",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a operator_workflow_names_only_current_creation_route`

## Failure Semantics

Fail closed on any active csdlc-init authority, installed binary presence, creation behavior regression, historical evidence mutation, or compatibility routing.

## Handoff

Retain typed evidence before convergence.
