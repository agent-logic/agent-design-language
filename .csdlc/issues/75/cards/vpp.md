# Validation Planning Prompt

Template: 1.0.0

Issue: 75

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/75/design.md

Diagram: .csdlc/prepared/issues/75/diagram.mmd

## Selected Lanes

[
  {
    "lane": "publication-linkage",
    "proof_role": "Prove typed mode parsing, same/split exact references, ambiguity rejection, compatibility default, and retained intent evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate6"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "finish-linkage",
    "proof_role": "Prove only closing merged publication evidence may produce terminal issue authority.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "schema",
    "proof_role": "Prove the generated publication schemas include the typed linkage enum and compatibility default.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "schema::tests"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate6`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_finish`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --lib schema::tests`

## Failure Semantics

Fail closed on absent or ambiguous linkage, unqualified split authority, remote mismatch, or any part_of terminalization.

## Handoff

Retain typed evidence before convergence.
