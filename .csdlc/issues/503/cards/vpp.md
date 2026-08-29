# Validation Planning Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/503/design.md

Diagram: .csdlc/prepared/issues/503/diagram.mmd

## Selected Lanes

[
  {
    "lane": "cli-contract",
    "proof_role": "Prove local preparation commands consume typed contracts and produce typed outputs without shell or live lifecycle authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
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
      "local_commands",
      "contract"
    ],
    "parallel_group": "v3-d-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/local_commands.rs."
  },
  {
    "lane": "bind-topology",
    "proof_role": "Prove local bind modeling requires registered topology and rejects branch-name-only authority.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
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
      "local_commands",
      "topology"
    ],
    "parallel_group": "v3-d-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/local_commands.rs."
  },
  {
    "lane": "card-roundtrip",
    "proof_role": "Prove cards render from the active prompt-template registry and round-trip without hand editing.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
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
      "local_commands",
      "card"
    ],
    "parallel_group": "v3-d-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/local_commands.rs."
  },
  {
    "lane": "doctor-findings",
    "proof_role": "Prove one typed issue input reaches a doctor-validated PVF plan with non-conflated ready/blocked/failed/deferred/skipped outcomes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
      "local_commands",
      "doctor"
    ],
    "parallel_group": "v3-d-focused",
    "defer_reason": "Deferred until implementation creates the exact issue-owned harness csdlc-v3/tests/local_commands.rs."
  },
  {
    "lane": "strict-clippy",
    "proof_role": "Reject Rust correctness and maintainability issues in the bounded V3-D crate surface.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
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
    "parallel_group": "v3-d-focused",
    "defer_reason": "Runs after implementation changes exist."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and malformed diff defects in the bounded issue diff.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "v3-d-final",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands contract`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands topology`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands card`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands doctor`
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on v3 local commands bypassing typed contracts, branch-name-only bind authorization, hand-edited generated cards, conflated doctor/PVF outcomes, live GitHub/lifecycle mutation from csdlc-v3, or any implied v3 authority cutover.

## Handoff

Retain typed evidence before convergence.
