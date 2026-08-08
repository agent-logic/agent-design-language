# Validation Planning Prompt

Template: 1.0.0

Issue: 5905

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5905/design.md

Diagram: .csdlc/prepared/issues/5905/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-finish-focused-contract",
    "proof_role": "Prove historical reconciliation success, idempotency, disposition-conditional fields, the complete identity mismatch matrix, ambiguity rejection, distinct provenance, and unchanged routine finish behavior.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "v092-5800-live-terminal-canary",
    "proof_role": "After implementation merge, produce #5800 from exact live issue/PR state and validate its cached terminal envelope before processing the remainder.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-7",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "csdlc-finish",
      "--root",
      ".",
      "--historical-request",
      ".git/csdlc-v2/requests/v092/5800-historical.json"
    ],
    "parallel_group": "post-merge-live",
    "defer_reason": "Runs only after the reviewed implementation PR merges; failure hard-stops the remaining inventory."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes and generated-state edits before exact-head review.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
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

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_finish`
- `csdlc-finish --root . --historical-request .git/csdlc-v2/requests/v092/5800-historical.json`
- `git diff --check`

## Failure Semantics

Fail closed on open, mismatched, ambiguous, or non-terminal state and preserve the issue-specific evidence.

## Handoff

Retain typed evidence before convergence.
