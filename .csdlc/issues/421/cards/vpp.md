# Validation Planning Prompt

Template: 1.0.0

Issue: 421

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/421/design.md

Diagram: .csdlc/prepared/issues/421/diagram.mmd

## Selected Lanes

[
  {
    "lane": "intentional-deletion-deliverables-focused",
    "proof_role": "Prove typed intentional-deletion deliverables, base/candidate deletion proof, and fail-closed false claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
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
      "gate2",
      "intentional_deletion_deliverable"
    ],
    "parallel_group": "421-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-library-regression",
    "proof_role": "Prove the readiness classification change preserves the broader typed library contract.",
    "acceptance_ids": [
      "AC-3",
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
    "parallel_group": "421-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 intentional_deletion_deliverable`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --lib`

## Failure Semantics

Fail closed unless the intentional deletion is explicitly typed, issue-owned, existed at governed base, and is deleted at exact candidate HEAD; all ordinary missing validator targets remain errors.

## Handoff

Retain typed evidence before convergence.
