# Validation Planning Prompt

Template: 1.0.0

Issue: 5912

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5912/design.md

Diagram: .csdlc/prepared/issues/5912/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-birth-witness-production-path",
    "proof_role": "focused production-path integration proof",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh"
    ],
    "parallel_group": "runtime-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-birth-witness-clippy",
    "proof_role": "strict compile and lint proof",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "runtime-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/prepared/issues/5912/validate-runtime-birth-witness.sh`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed without emission on policy construction, packet build, validation, serialization, or sink errors.

## Handoff

Retain typed evidence before convergence.
