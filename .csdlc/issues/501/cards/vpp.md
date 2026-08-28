# Validation Planning Prompt

Template: 1.0.0

Issue: 501

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute focused deterministic foundation tests, targeted repository-context and projection filters, strict clippy, and diff hygiene.

## Lane Inputs

Design: .csdlc/prepared/issues/501/design.md

Diagram: .csdlc/prepared/issues/501/diagram.mmd

## Selected Lanes

[
  {
    "lane": "foundation-unit",
    "proof_role": "Prove deterministic foundation state creation and stable replay behavior.",
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
      "foundation"
    ],
    "parallel_group": "v3-b-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/foundation.rs."
  },
  {
    "lane": "repository-context",
    "proof_role": "Prove repository context is explicit, canonicalized, and independent from hidden cwd authority.",
    "acceptance_ids": [
      "AC-2"
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
      "foundation",
      "repository_context"
    ],
    "parallel_group": "v3-b-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/foundation.rs."
  },
  {
    "lane": "state-projection",
    "proof_role": "Prove projection replay and serialization order are stable.",
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
      "foundation",
      "projection"
    ],
    "parallel_group": "v3-b-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/foundation.rs."
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Reject Rust correctness and maintainability issues in the bounded V3-B crate surface.",
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
    "parallel_group": "v3-b-focused",
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
    "parallel_group": "v3-b-focused",
    "defer_reason": "Runs after implementation changes exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation repository_context`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test foundation projection`
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on hidden repository context, nondeterministic replay, missing retained-requirement proof, lifecycle mutation, GitHub mutation, or any implied v3 authority cutover.

## Handoff

Retain typed evidence before convergence.
