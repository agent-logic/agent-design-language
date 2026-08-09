# Validation Planning Prompt

Template: 1.0.0

Issue: 63

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/63/design.md

Diagram: .csdlc/prepared/issues/63/diagram.mmd

## Selected Lanes

[
  {
    "lane": "implemented-sip-scope-editor-contract",
    "proof_role": "Exercise the real typed editor/store/renderer/validator path for accepted correction, exact audit, concurrency, card/phase guards, and projection drift.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2",
      "implemented_sip_scope_correction"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "implemented-review-recovery-contract",
    "proof_role": "Prove assignment-only and changes-required implemented recovery clears review truth without an invalid same-phase transition, while clean implemented recovery fails closed.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_review_recovery_clears_truth"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "focused-editor-clippy",
    "proof_role": "Reject Rust type, match exhaustiveness, review recovery, and editor binary regressions on the bounded library and csdlc-edit target.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 420,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib",
      "--bin",
      "csdlc-edit",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate2 implemented_sip_scope_correction`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_review_recovery_clears_truth`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --lib --bin csdlc-edit -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on stale concurrency truth, empty justification, wrong card or phase, retained review/publication/readiness truth, audit omission, direct Markdown drift, or validator failure.

## Handoff

Retain typed evidence before convergence.
