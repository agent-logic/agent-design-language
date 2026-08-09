# Validation Planning Prompt

Template: 1.0.0

Issue: 78

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/78/design.md

Diagram: .csdlc/prepared/issues/78/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-focused-tests",
    "proof_role": "Prove positive correction, recovery provenance, phase/card/input/CAS/drift rejection, atomic projections, audit evidence, and the request shape consumed by issue #73.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate5"
    ],
    "parallel_group": "local-fastwork",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-quality",
    "proof_role": "Prove formatting and warning-free production and test targets.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local-fastwork",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate5`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`

## Failure Semantics

Fail closed on absent recovery provenance, non-STP targets, wrong phase, malformed replacements, stale CAS, projection drift, incomplete audit evidence, or unrelated lifecycle widening.

## Handoff

Retain typed evidence before convergence.
