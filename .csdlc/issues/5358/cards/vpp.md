# Validation Planning Prompt

Template: 1.0.0

Issue: 5358

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5358/design.md

Diagram: .csdlc/prepared/issues/5358/diagram.mmd

## Selected Lanes

[
  {
    "lane": "generation-install-verify",
    "proof_role": "Verify stable exact-revision v2 installation, inventory, provenance, and selector resolution",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-install",
      "verify",
      "--repo",
      ".",
      "--bin-dir",
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2",
      "--inventory",
      "csdlc-v2/operator/coexistence.json"
    ],
    "parallel_group": "authority",
    "defer_reason": null
  },
  {
    "lane": "focused-terminal-reconciliation",
    "proof_role": "Prove normal merge, squash merge, published-head reconciliation, and merged-PR unknown mergeability handling",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate7_lifecycle"
    ],
    "parallel_group": "tests",
    "defer_reason": null
  },
  {
    "lane": "all-target-tests",
    "proof_role": "Prove complete C-SDLC v2 lifecycle and regression compatibility",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 10000,
    "argv": [
      "cargo",
      "test",
      "--locked",
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
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
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
    "parallel_group": "lint",
    "defer_reason": null
  },
  {
    "lane": "format-check",
    "proof_role": "Prove canonical Rust formatting",
    "acceptance_ids": [
      "AC-8"
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
    "proof_role": "Prove #5358 canonical record and six-card integrity after acceptance execution",
    "acceptance_ids": [
      "AC-3",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      ".",
      "--issue",
      "5358"
    ],
    "parallel_group": "authority",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-install verify --repo . --bin-dir /Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate7_lifecycle`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --all-targets`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo . --issue 5358`

## Failure Semantics

Fail closed on card corruption, claim collision, stale authority, unresolved acceptance blockers, missing exact-revision evidence, or any preparation-to-acceptance overclaim.

## Handoff

Retain typed evidence before convergence.
