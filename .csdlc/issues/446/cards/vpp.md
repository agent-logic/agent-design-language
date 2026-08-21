# Validation Planning Prompt

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/446/design.md

Diagram: .csdlc/prepared/issues/446/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue446-contract",
    "proof_role": "Prove exact owned Runtime surfaces before implementation.",
    "acceptance_ids": [
      "AC-1",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/446/validate_issue446.sh"
    ],
    "parallel_group": "contract",
    "defer_reason": null
  },
  {
    "lane": "runtime-unit",
    "proof_role": "Prove compiler, gate, dispatch, denial, and Runtime integration.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl/Cargo.toml",
      "--lib",
      "uts_acc_compiler"
    ],
    "parallel_group": "runtime",
    "defer_reason": null
  },
  {
    "lane": "runtime-compile",
    "proof_role": "Prove crate boundaries.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "parallel_group": "compile",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash .csdlc/prepared/issues/446/validate_issue446.sh`
- `cargo test --manifest-path adl/Cargo.toml --lib uts_acc_compiler`
- `cargo check --manifest-path adl-runtime/Cargo.toml`

## Failure Semantics

Fail closed before actuation on parse, authority, compiler, policy, gate, adapter, identity, or receipt errors.

## Handoff

Retain typed evidence before convergence.
