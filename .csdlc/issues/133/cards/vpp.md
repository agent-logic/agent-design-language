# Validation Planning Prompt

Template: 1.0.0

Issue: 133

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/133/design.md

Diagram: .csdlc/prepared/issues/133/diagram.mmd

## Selected Lanes

[
  {
    "lane": "authority-snapshot-tests",
    "proof_role": "Prove complete certificate, failure, lease, fencing, placement, migration, and recovery enumeration; revision mutation; N/N+1 drift rejection; redaction; bounds; restart parity; and cross-authority opaque-reference parity.",
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
    "budget_seconds": 1200,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--locked",
      "--test",
      "distributed_authority_snapshots"
    ],
    "parallel_group": "fastwork-runtime",
    "defer_reason": null
  },
  {
    "lane": "authority-snapshot-clippy",
    "proof_role": "Prove the exact issue-owned authority snapshot integration target is warning-free before exact-head review.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--locked",
      "--test",
      "distributed_authority_snapshots",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "fastwork-runtime",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path adl-runtime/Cargo.toml --locked --test distributed_authority_snapshots`
- `cargo clippy --manifest-path adl-runtime/Cargo.toml --locked --test distributed_authority_snapshots -- -D warnings`

## Failure Semantics

Fail closed on incomplete enumeration, capacity overflow, stale revision, restore mismatch, private evidence exposure, self-attested construction, issue #5877 path overlap, failed validation, or review drift.

## Handoff

Retain typed evidence before convergence.
