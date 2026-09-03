# Validation Planning Prompt

Template: 1.0.0

Issue: 656

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/656/design.md

Diagram: .csdlc/prepared/issues/656/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-generation-contract",
    "proof_role": "Prove matched staging, receipt integrity, activation, predecessor retention, and rollback.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      "adl/tools/test_runtime_v3_generation_install.sh"
    ],
    "parallel_group": "runtime-generation",
    "defer_reason": null
  },
  {
    "lane": "runtime-generation-premutation",
    "proof_role": "Prove invalid generations and mismatched service definitions are rejected before mutation on the executable CSM command surface.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "preflight_"
    ],
    "parallel_group": "runtime-generation",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash adl/tools/test_runtime_v3_generation_install.sh`
- `cargo test --manifest-path adl/Cargo.toml --bin adl preflight_`

## Failure Semantics

Fail closed before service mutation on a missing artifact, hash or receipt mismatch, incompatible schema, partial activation, or unverified rollback target.

## Handoff

Retain typed evidence before convergence.
