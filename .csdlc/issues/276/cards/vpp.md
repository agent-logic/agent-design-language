# Validation Planning Prompt

Template: 1.0.0

Issue: 276

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/276/design.md

Diagram: .csdlc/prepared/issues/276/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-276-preparation-validator",
    "proof_role": "Prove #276 remains scoped to durable journal foundation and validates canonical #112/#265/#270 derived-terminal caches ancestral to current origin/main.",
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
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/276/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-journal-fmt",
    "proof_role": "Reject Rust formatting drift for the #276 kernel journal change.",
    "acceptance_ids": [
      "AC-4",
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
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-journal-tests",
    "proof_role": "Prove durable journal schema/storage/restart/corruption/retention/deletion foundation behavior.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_journal"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-journal-clippy",
    "proof_role": "Reject warning regressions in the #276 focused kernel target.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_journal",
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

- `python3 .csdlc/prepared/issues/276/validate_preparation_bundle.py`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_journal`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_journal -- -D warnings`

## Failure Semantics

Fail closed on graph mismatch, stale/open dependency truth, missing terminal cache, non-ancestral merge SHA, scope absorption, invalid bind target, or design/readiness review findings.

## Handoff

Retain typed evidence before convergence.
