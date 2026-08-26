# Validation Planning Prompt

Template: 1.0.0

Issue: 306

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/306/design.md

Diagram: .csdlc/prepared/issues/306/diagram.mmd

## Selected Lanes

[
  {
    "lane": "publication-tail-exact-clean",
    "proof_role": "Focused Rust integration tests proving publication create/update/noop, interrupted-after-intent, interrupted-after-push, interrupted-after-record, metadata-tail classification, retry determinism, and exact-clean finish readiness through referenced finish-contract behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail"
    ],
    "parallel_group": "local",
    "defer_reason": "Deferred only for initialized bind readiness: #306 implementation must create issue-owned csdlc-v2/tests/publication_tail.rs in the bound worktree before validation can pass; publication remains fail-closed without this target."
  },
  {
    "lane": "strict-clippy-publication-tail",
    "proof_role": "Strict warning-free proof for touched publication code plus referenced finish-contract interactions through the focused publication_tail test target; finish.rs remains read-only unless Planning explicitly widens #306.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "publication_tail",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "local",
    "defer_reason": "Deferred only for initialized bind readiness: #306 implementation must create issue-owned csdlc-v2/tests/publication_tail.rs in the bound worktree before strict Clippy can pass; publication remains fail-closed without this target."
  },
  {
    "lane": "fresh-exact-head-review-guard",
    "proof_role": "Typed review guard verifies the fresh exact-head review packet and prevents publication without current review truth; this is the review evidence lane for AC-7, not a Rust behavioral test.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-review",
      "guard",
      "--request",
      ".git/csdlc-v2/requests/306-review-guard.json"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs only after implementation and fresh exact-head review produce .git/csdlc-v2/requests/306-review-guard.json; publication remains fail-closed without that typed review truth."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test publication_tail`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --test publication_tail -- -D warnings`
- `.adl/bin/csdlc-v2/csdlc-review guard --request .git/csdlc-v2/requests/306-review-guard.json`

## Failure Semantics

Fail closed on dirty required metadata, unsafe untracked files, mismatched remote head, duplicate publication records, retry ambiguity, stale review, or incomplete interruption proof.

## Handoff

Retain typed evidence before convergence.
