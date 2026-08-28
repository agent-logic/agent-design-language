# Validation Planning Prompt

Template: 1.0.0

Issue: 502

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG for transition, transaction, recovery, adapter, clippy, diff, and documentation-scope proof.

## Lane Inputs

Design: .csdlc/prepared/issues/502/design.md

Diagram: .csdlc/prepared/issues/502/diagram.mmd

## Selected Lanes

[
  {
    "lane": "transition-matrix",
    "proof_role": "Prove command/state pairs have explicit capability-checked allowed or rejected outcomes.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "transactions",
      "transition"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after csdlc-v3/tests/transactions.rs is implemented for #502."
  },
  {
    "lane": "transaction-failure",
    "proof_role": "Prove stale CAS, interruption, and projection-failure cases fail closed without partial authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "transactions",
      "transaction"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after csdlc-v3/tests/transactions.rs is implemented for #502."
  },
  {
    "lane": "recovery-replay",
    "proof_role": "Prove recovery replay preserves audit provenance and classifies interrupted writes deterministically.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "transactions",
      "recovery"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after csdlc-v3/tests/transactions.rs is implemented for #502."
  },
  {
    "lane": "adapter-boundary",
    "proof_role": "Prove typed Git/process adapter outcomes preserve argv, output, error, timeout, cancellation, and redaction boundaries.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "transactions",
      "adapter"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after csdlc-v3/tests/transactions.rs is implemented for #502."
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Reject Rust correctness and maintainability issues in the bounded V3-C crate surface.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after implementation changes exist."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and malformed diff defects in the bounded issue diff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "v3-c-focused",
    "defer_reason": "Runs after implementation changes exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions transition`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions transaction`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions recovery`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test transactions adapter`
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on partial authority, provenance loss, stale generation/digest writers, branch-name-only authorization, shell-string adapters, credential leakage, live GitHub/lifecycle mutation, or any implied v3 authority cutover.

## Handoff

Retain typed evidence before convergence.
