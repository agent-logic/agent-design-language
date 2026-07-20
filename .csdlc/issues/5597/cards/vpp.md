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
    "proof_role": "Run the complete native v2 all-target suite including Gate 10 install provenance",
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
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets"
    ],
    "parallel_group": "v2-tests",
    "defer_reason": null
  },
  {
    "lane": "v2-strict-clippy",
    "proof_role": "Run strict all-target native v2 lint",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
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
    "parallel_group": "v2-lint",
    "defer_reason": null
  },
  {
    "lane": "ordered-owner-doctor",
    "proof_role": "Build current v2 owner binaries and then run typed doctor from that exact target in one fail-closed ordered script",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5597/validate-native-owner.sh"
    ],
    "parallel_group": "v2-owner",
    "defer_reason": null
  },
  {
    "lane": "bounded-revision-diff",
    "proof_role": "Prove origin/main-to-reviewed-revision whitespace hygiene and exact protected-path scope",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/5597/validate-bounded-diff.sh",
      "origin/main",
      "HEAD"
    ],
    "parallel_group": "diff-proof",
    "defer_reason": null
  },
  {
    "lane": "worktree-diff-hygiene",
    "proof_role": "Detect whitespace defects in any remaining uncommitted tracked worktree diff",
    "acceptance_ids": [
      "AC-8"
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
    "parallel_group": "diff-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --all-targets`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `bash .csdlc/prepared/issues/5597/validate-native-owner.sh`
- `bash .csdlc/prepared/issues/5597/validate-bounded-diff.sh origin/main HEAD`
- `git diff --check`

## Failure Semantics

Fail closed on incompatible registry truth, data loss, incomplete acceptance coverage, stale review, lifecycle ambiguity, or scope expansion.

## Handoff

Retain typed evidence before convergence.
