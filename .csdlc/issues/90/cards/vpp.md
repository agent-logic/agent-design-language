# Validation Planning Prompt

Template: 1.0.0

Issue: 90

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/90/design.md

Diagram: .csdlc/prepared/issues/90/diagram.mmd

## Selected Lanes

[
  {
    "lane": "code-repository-migration-tests",
    "proof_role": "Prove allowed phases, exact identity/topology, cleanliness, CAS, idempotency, atomic audit, and all negative cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "code_repository_migration"
    ],
    "parallel_group": "fastwork-rust",
    "defer_reason": null
  },
  {
    "lane": "reviewed-publication-regression",
    "proof_role": "Prove a reviewed pre-field record migrates and passes existing split-authority publication preflight without review recovery.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--locked",
      "--test",
      "gate6",
      "split_authority"
    ],
    "parallel_group": "fastwork-rust",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-quality",
    "proof_role": "Prove formatting, warning-free code, and command/schema integration.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 3000,
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
    "parallel_group": "fastwork-rust",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test code_repository_migration`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --locked --test gate6 split_authority`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --locked --all-targets -- -D warnings`

## Failure Semantics

Fail closed on wrong repository identity, incomplete or ambiguous topology, dirty worktree, unsupported phase, stale CAS, conflicting prior identity, audit duplication, review drift, or publication-check weakening.

## Handoff

Retain typed evidence before convergence.
