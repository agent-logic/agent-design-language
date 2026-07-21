# Validation Planning Prompt

Template: 1.0.0

Issue: 5600

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/issues/5600/retained/design.md

Diagram: .csdlc/issues/5600/retained/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-replanning-tests",
    "proof_role": "Prove all typed replacement operations, guards, atomicity, acceptance coverage, compatibility, and #5337 conversion",
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
    "budget_seconds": 900,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "tests",
    "defer_reason": null
  },
  {
    "lane": "all-target-tests",
    "proof_role": "Prove complete C-SDLC v2 regression compatibility",
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
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "tests",
    "defer_reason": null
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Prove warning-free all-target Rust implementation",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "lint",
    "defer_reason": null
  },
  {
    "lane": "format-check",
    "proof_role": "Prove canonical Rust formatting",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "lint",
    "defer_reason": null
  },
  {
    "lane": "typed-doctor",
    "proof_role": "Prove the #5600 record remains structurally and semantically valid",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-doctor",
      "--",
      "--repo",
      ".",
      "--issue",
      "5600"
    ],
    "parallel_group": "lint",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --all-targets`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-doctor -- --repo . --issue 5600`

## Failure Semantics

Fail closed on empty or misowned replacements, stale compare-and-swap truth, unauthorized phase or claim, incomplete acceptance coverage, partial regeneration, compatibility regression, stale review, or publication ambiguity.

## Handoff

Retain typed evidence before convergence.
