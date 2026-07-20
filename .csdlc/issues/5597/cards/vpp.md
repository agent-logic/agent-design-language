# Validation Planning Prompt

Template: 1.0.0

Issue: 5597

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5597/design.md

Diagram: .csdlc/prepared/issues/5597/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v2-all-target-tests",
    "proof_role": "Run the complete native C-SDLC v2 all-target test suite including Gates 2, 5, 8, 9, 10 and immutable compatibility fixtures",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target",
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "v2-proof",
    "defer_reason": null
  },
  {
    "lane": "v2-strict-clippy",
    "proof_role": "Run strict all-target lint for the native v2 crate",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target",
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "v2-proof",
    "defer_reason": null
  },
  {
    "lane": "v2-owner-binaries",
    "proof_role": "Build the current native v2 owner binaries on FastWork and prove executable owner artifacts without invoking sunset v1 wrappers",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "env",
      "CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target",
      "cargo",
      "build",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bins"
    ],
    "parallel_group": "v2-owner",
    "defer_reason": null
  },
  {
    "lane": "typed-doctor",
    "proof_role": "Run the freshly built typed v2 doctor against issue 5597",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "/Volumes/FastWork/adl-builds/5597/csdlc-v2-target/debug/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5597"
    ],
    "parallel_group": "v2-owner",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove whitespace hygiene and bounded tracked scope",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "v2-owner",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `env CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target cargo test --manifest-path csdlc-v2/Cargo.toml --all-targets`
- `env CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `env CARGO_TARGET_DIR=/Volumes/FastWork/adl-builds/5597/csdlc-v2-target cargo build --manifest-path csdlc-v2/Cargo.toml --bins`
- `/Volumes/FastWork/adl-builds/5597/csdlc-v2-target/debug/csdlc-doctor --repo . --issue 5597`
- `git diff --check`

## Failure Semantics

Fail closed on incompatible registry truth, data loss, incomplete acceptance coverage, stale review, lifecycle ambiguity, or scope expansion.

## Handoff

Retain typed evidence before convergence.
