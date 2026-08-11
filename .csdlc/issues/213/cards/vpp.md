# Validation Planning Prompt

Template: 1.0.0

Issue: 213

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/213/design.md

Diagram: .csdlc/prepared/issues/213/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-gate2",
    "proof_role": "The complete Gate 2 integration target reproduces the literal #205 prior-planning-edit plus design-change sequence and proves guarded initialized/ready ordinal-acceptance and pending-step repair, atomic current binding refresh, exact before/after bytes, path/identity/CAS/coverage failure, regenerated projections, audit preservation, doctor blocking, exact reapproval, and explicit bound/implemented compatibility.",
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
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "213-serial-01-gate2",
    "defer_reason": null
  },
  {
    "lane": "csdlc-clippy",
    "proof_role": "Strict all-target Clippy rejects warnings and unsafe owner-level API changes after Gate 2 passes.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "213-serial-02-clippy",
    "defer_reason": null
  },
  {
    "lane": "csdlc-fmt",
    "proof_role": "Formatting check proves the touched Rust source and test remain canonical.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "213-serial-03-fmt",
    "defer_reason": null
  },
  {
    "lane": "base-to-source-diff-hygiene",
    "proof_role": "The committed origin/main...HEAD range rejects whitespace, blank-line-at-EOF, and patch defects across the complete preparation and implementation source rather than only uncommitted work.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "213-serial-04-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on any phase widening beyond the exact operations, stale CAS, invalid card ownership, broken acceptance coverage, partial generation/audit/projection mutation, stale design approval, or validation/review failure.

## Handoff

Retain typed evidence before convergence.
