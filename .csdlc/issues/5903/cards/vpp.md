# Validation Planning Prompt

Template: 1.0.0

Issue: 5903

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5903/design.md

Diagram: .csdlc/prepared/issues/5903/diagram.mmd

## Selected Lanes

[
  {
    "lane": "sprint4-readiness-doctor-contract",
    "proof_role": "Run the current typed doctor contract tests that cover initialized planning repair and execution readiness.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 540,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate2"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "sprint4-live-readiness-validator",
    "proof_role": "Force-build and digest the current source doctor, prove all ten doctors ready, compare baseline/manifest/current serialization gates, query live prerequisites through the external GitHub proof boundary, reject retired claim text, and enforce the no-product-change allowlist.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5903/validate-readiness.rb"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "sprint4-exact-head-review-hygiene",
    "proof_role": "Reject malformed diffs before the independently executed exact-head review and typed csdlc-review record satisfy AC-7.",
    "acceptance_ids": [
      "AC-7"
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

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate2`
- `ruby .csdlc/prepared/issues/5903/validate-readiness.rb`
- `git diff --check`

## Failure Semantics

Fail closed on non-path metadata, stale claim authority, doctor failure, dependency drift, or product-path mutation.

## Handoff

Retain typed evidence before convergence.
