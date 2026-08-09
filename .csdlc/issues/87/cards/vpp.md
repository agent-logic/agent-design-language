# Validation Planning Prompt

Template: 1.0.0

Issue: 87

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/87/design.md

Diagram: .csdlc/prepared/issues/87/diagram.mmd

## Selected Lanes

[
  {
    "lane": "acip-version-negotiation",
    "proof_role": "The issue-owned integration target proves exact, wider-compatible, future-only, and malformed inclusive minor ranges.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--test",
      "acip_version_negotiation"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "acip-strict-clippy",
    "proof_role": "Warning-denied library compilation proves the shared ACIP predicate; exact child consumer commands were also run on their live heads with this patch.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
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

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --locked --manifest-path adl-runtime/Cargo.toml --test acip_version_negotiation`
- `cargo clippy --locked --manifest-path adl-runtime/Cargo.toml --lib -- -D warnings`

## Failure Semantics

Fail closed on changed major matching, accepted malformed/future-only ranges, missing focused coverage, strict Clippy warnings, or child-owned path changes.

## Handoff

Retain typed evidence before convergence.
