# Validation Planning Prompt

Template: 1.0.0

Issue: 239

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/239/design.md

Diagram: .csdlc/prepared/issues/239/diagram.mmd

## Selected Lanes

[
  {
    "lane": "terminal-envelope-metadata-head",
    "proof_role": "Reproduce PR #238 topology and prove metadata-only acceptance plus substantive-drift, malformed-publication-revision, and metadata-only non-ancestor rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 700,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish",
      "derived_terminal_accepts_publication_metadata_only_head_and_rejects_substantive_drift",
      "--",
      "--exact"
    ],
    "parallel_group": "focused",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed patch structure.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 100,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "post-merge-5835-cache",
    "proof_role": "Prove the merged binary validates the retained #5835 terminal envelope without tracked rewrites.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 200,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "run",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--bin",
      "csdlc-finish",
      "--",
      "--root",
      ".",
      "--validate-cached-issue",
      "5835"
    ],
    "parallel_group": "post-merge",
    "defer_reason": "This terminal acceptance lane can run only after #239 merges to main; pre-merge proof is the focused gate_finish regression."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_finish derived_terminal_accepts_publication_metadata_only_head_and_rejects_substantive_drift -- --exact`
- `git diff --check`
- `cargo run --manifest-path csdlc-v2/Cargo.toml --bin csdlc-finish -- --root . --validate-cached-issue 5835`

## Failure Semantics

Fail closed on any substantive-drift false green, malformed revision acceptance, invalid ancestry, or canonical identity weakening.

## Handoff

Retain typed evidence before convergence.
