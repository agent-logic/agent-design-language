# Validation Planning Prompt

Template: 1.0.0

Issue: 353

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/353/design.md

Diagram: .csdlc/prepared/issues/353/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-353-preparation",
    "proof_role": "Prove initialized local packet integrity; external #349/#342 freeze is separately observed read-only.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/353/validate_preparation.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue-353-finish-suite",
    "proof_role": "Run the complete existing gate_finish integration target including new positive/null/unequal/non-governed anchor regressions; Cargo reports the nonzero denominator.",
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
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-format",
    "proof_role": "Prove formatting.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all",
      "--",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-clippy",
    "proof_role": "Reject warnings in every target.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 1500,
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
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/353/validate_preparation.rb`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_finish`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --all -- --check`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale topology, unequal review, non-governed drift, validation failure, or #349/#342 mutation.

## Handoff

Retain typed evidence before convergence.
