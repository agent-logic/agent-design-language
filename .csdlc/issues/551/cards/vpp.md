# Validation Planning Prompt

Template: 1.0.0

Issue: 551

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/551/design.md

Diagram: .csdlc/prepared/issues/551/diagram.mmd

## Selected Lanes

[
  {
    "lane": "polis-config",
    "proof_role": "Prove valid duplicate invalid mismatch diagnostics and reload-retention configuration behavior with a nonzero target.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "--no-tests=fail",
      "-E",
      "test(polis_identity)"
    ],
    "parallel_group": "runtime",
    "defer_reason": "The named polis_identity cases are #551 implementation deliverables."
  },
  {
    "lane": "polis-feed-control-openapi",
    "proof_role": "Run exact nonzero control Observatory and OpenAPI targets to prove v3 projection bounded reload diagnostics and explicit v1 v2 compatibility.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/551/validate-runtime-polis.sh"
    ],
    "parallel_group": "runtime",
    "defer_reason": "The issue-owned wrapper and named control Observatory and OpenAPI cases are #551 implementation deliverables."
  },
  {
    "lane": "html-polis-identity",
    "proof_role": "Run the exact Node file and reject a zero-test TAP result while proving feed-owned HTML rendering.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/551/validate-html-polis.sh"
    ],
    "parallel_group": "html",
    "defer_reason": "The issue-owned wrapper and Node test file are #551 implementation deliverables."
  },
  {
    "lane": "rust-format",
    "proof_role": "Prove Rust formatting for the exact workspace.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove exact branch diff hygiene.",
    "acceptance_ids": [
      "AC-5"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo nextest run --locked --manifest-path adl-runtime-kernel/Cargo.toml --test configuration --no-tests=fail -E test(polis_identity)`
- `bash .csdlc/prepared/issues/551/validate-runtime-polis.sh`
- `bash .csdlc/prepared/issues/551/validate-html-polis.sh`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check`
- `git diff --check`

## Failure Semantics

Fail closed on scope drift, Unity activation, identity ambiguity, secret exposure, zero-test proof, or stale review.

## Handoff

Retain typed evidence before convergence.
