# Validation Planning Prompt

Template: 1.0.0

Issue: 237

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/237/design.md

Diagram: .csdlc/prepared/issues/237/diagram.mmd

## Selected Lanes

[
  {
    "lane": "continuity-public-api-target",
    "proof_role": "In the one required CI job, execute the issue-owned public-boundary target and prove the verified-continuity capability and governed-cognition entrypoints remain exported.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--test",
      "capability_envelope"
    ],
    "parallel_group": "required",
    "defer_reason": null
  },
  {
    "lane": "continuity-authority-lib",
    "proof_role": "In the one required CI job, prove real signed continuity composition, self-consistent substitutions, token mismatch and replay rejection, and retained authority/privacy negatives.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib"
    ],
    "parallel_group": "required",
    "defer_reason": null
  },
  {
    "lane": "continuity-public-boundary-doc",
    "proof_role": "In the same required CI job, prove direct callers cannot import the retired raw public capability or governed-cognition APIs.",
    "acceptance_ids": [
      "AC-1",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--doc"
    ],
    "parallel_group": "required",
    "defer_reason": null
  },
  {
    "lane": "continuity-strict-lib-clippy",
    "proof_role": "In the same required CI job, reject warnings in the changed library surface without claiming the unrelated pre-existing TLS all-target lint.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 900,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--locked",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "required",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --test capability_envelope`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --lib`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --locked --doc`
- `cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --locked --lib -- -D warnings`

## Failure Semantics

Fail closed on any unverified continuity record, identity mismatch, digest substitution, authority drift, privacy regression, missing exact review, or non-green required CI.

## Handoff

Retain typed evidence before convergence.
