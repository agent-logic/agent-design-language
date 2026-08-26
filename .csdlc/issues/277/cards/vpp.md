# Validation Planning Prompt

Template: 1.0.0

Issue: 277

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/277/design.md

Diagram: .csdlc/prepared/issues/277/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-277-preparation-validator",
    "proof_role": "Prove #277 consumes canonical #276/#270 terminal caches and remains bounded to watermarks/idempotency/replay/receipts in the bound FastWork topology.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/277/validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-continuity-fmt",
    "proof_role": "Reject Rust formatting drift for the #277 continuity module and focused test.",
    "acceptance_ids": [
      "AC-10"
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
    "lane": "runtime-kernel-conversation-continuity-tests",
    "proof_role": "Prove restart, idempotency, replay, ambiguous dispatch, receipt, deletion, and acknowledgement-watermark behavior against the #276 journal foundation.",
    "acceptance_ids": [
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
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_continuity"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "runtime-kernel-conversation-continuity-clippy",
    "proof_role": "Reject warning regressions in the #277 focused Runtime kernel target.",
    "acceptance_ids": [
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_continuity",
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

- `python3 .csdlc/prepared/issues/277/validate_preparation_bundle.py`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --test conversation_continuity -- -D warnings`

## Failure Semantics

Fail closed on terminal dependency mismatch, non-ancestral merge SHA, scope absorption, stale acknowledgement trust, duplicate execution, ambiguous-outcome misclassification, invalid bind target, failed proof, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
