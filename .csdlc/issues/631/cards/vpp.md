# Validation Planning Prompt

Template: 1.0.0

Issue: 631

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/631/design.md

Diagram: .csdlc/prepared/issues/631/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v3-proof-parity-install-tests",
    "proof_role": "Run focused Rust tests for v3 proof, shadow, soak, and install positive and denial behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2400,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "proof_parity_install_commands"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "v3-full-regression",
    "proof_role": "Run the complete v3 test suite after route implementation.",
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
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml"
    ],
    "parallel_group": "rust-full",
    "defer_reason": null
  },
  {
    "lane": "issue-validator",
    "proof_role": "Run the issue-owned validator and fail on missing route coverage, weak denial assertions, v2 source diffs, or missing focused tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1600,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh"
    ],
    "parallel_group": "issue-owned",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Run exact-range diff hygiene before publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test proof_parity_install_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml`
- `bash .csdlc/prepared/issues/631/validate-v3-h5-proof-parity-install.sh`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing proof manifests, stale evidence, broad parity claims, hidden soak state, install provenance mismatch, cutover implication, v2 source changes, stale/empty evidence, or missing real-issue canary coverage.

## Handoff

Retain typed evidence before convergence.
