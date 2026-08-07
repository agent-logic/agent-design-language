# Validation Planning Prompt

Template: 1.0.0

Issue: 5906

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5906/design.md

Diagram: .csdlc/prepared/issues/5906/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-finish-focused",
    "proof_role": "Prove single-candidate compatibility, unique-latest selection, fail-closed rejection cases, unchanged routine gates, and the contract used for final live reconciliation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_finish`

## Failure Semantics

Fail closed on absent or tied timestamp evidence, stale identity, failed focused proof, or review findings.

## Handoff

Retain typed evidence before convergence.
