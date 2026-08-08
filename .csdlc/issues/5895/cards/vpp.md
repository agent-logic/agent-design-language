# Validation Planning Prompt

Template: 1.0.0

Issue: 5895

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5895/design.md

Diagram: .csdlc/prepared/issues/5895/diagram.mmd

## Selected Lanes

[
  {
    "lane": "installed-generation-contract",
    "proof_role": "Run the complete focused Gate10A inventory/provenance suite, including a required test that installs the declared generation into an isolated stable layout and invokes its csdlc-issue, csdlc-validate, csdlc-doctor, and csdlc-bind binaries while proving csdlc-migrate is absent.",
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
    "budget_seconds": 600,
    "budget_tokens": 4200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate10a"
    ],
    "parallel_group": "installed",
    "defer_reason": null
  },
  {
    "lane": "creation-semantics",
    "proof_role": "Prove claim-free create, validate, doctor, and bind semantics independently of the installed-generation provenance proof.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1800,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate10a`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`

## Failure Semantics

Fail closed on active retired authority, install mismatch, stale selector/provenance, non-installed canary execution, or historical evidence churn.

## Handoff

Retain typed evidence before convergence.
