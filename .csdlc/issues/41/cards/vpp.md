# Validation Planning Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/41/design.md

Diagram: .csdlc/prepared/issues/41/diagram.mmd

## Selected Lanes

[
  {
    "lane": "github-issue-read-loopback-contract",
    "proof_role": "Run the real split issue binary against deterministic loopback success, 404, auth, rate, server, and transport responses and prove JSON, exit, and redaction behavior.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 6000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject Rust API, enum, match exhaustiveness, and CLI integration regressions across the bounded C-SDLC v2 crate.",
    "acceptance_ids": [
      "AC-5",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
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
    "parallel_group": "issue-local",
    "defer_reason": null
  },
  {
    "lane": "issue-diff-hygiene",
    "proof_role": "Reject malformed whitespace and patch artifacts before exact-head review.",
    "acceptance_ids": [
      "AC-8"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on ambiguous classification, any non-404 mislabeled as not-found, any credential or raw remote detail in output, non-JSON CLI errors, unstable exits, or successful-read drift.

## Handoff

Retain typed evidence before convergence.
