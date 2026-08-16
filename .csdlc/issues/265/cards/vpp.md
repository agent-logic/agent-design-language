# Validation Planning Prompt

Template: 1.0.0

Issue: 265

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/265/design.md

Diagram: .csdlc/prepared/issues/265/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-265-preparation-validator",
    "proof_role": "Verify the refreshed #265 preparation packet recognizes #112 terminal ancestry and preserves child-scope boundaries.",
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
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/265/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-layer8-ingress-focused",
    "proof_role": "Prove configured Layer 8 conversation ingress refuses unauthorized requests before session/turn mutation and still authorizes allowed requests before dispatch.",
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
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "layer8_ingress",
      "--",
      "--nocapture"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-fmt",
    "proof_role": "Verify Rust formatting for the #265 Runtime kernel ingress diff.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-strict-clippy",
    "proof_role": "Verify strict warning-free Runtime kernel crate and tests after the #265 ingress gate.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Verify no conflict markers or whitespace errors in the #265 diff.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
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

- `python3 .csdlc/prepared/issues/265/validate_preparation_bundle.py`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib layer8_ingress -- --nocapture`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed: bootstrap/design may be repaired through typed v2, but bind/implementation stays blocked while #112 is not terminal and ancestral.

## Handoff

Retain typed evidence before convergence.
