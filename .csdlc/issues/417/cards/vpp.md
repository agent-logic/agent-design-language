# Validation Planning Prompt

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/417/design.md

Diagram: .csdlc/prepared/issues/417/diagram.mmd

## Selected Lanes

[
  {
    "lane": "implemented-design-refresh-recovery-focused",
    "proof_role": "Prove exact implemented recovery ordering, provenance, compatibility, and authority clearing.",
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
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_design_refresh"
    ],
    "parallel_group": "417-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-library-regression",
    "proof_role": "Prove the store change preserves the broader typed library contract.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "parallel_group": "417-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_design_refresh`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib`

## Failure Semantics

Fail closed without a current implemented recovery epoch, preserve all audit provenance, and do not permit design refresh to restore or retain downstream review/publication authority.

## Handoff

Retain typed evidence before convergence.
