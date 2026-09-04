# Validation Planning Prompt

Template: 1.0.0

Issue: 659

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/659/design.md

Diagram: .csdlc/prepared/issues/659/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-convergence-preparation",
    "proof_role": "Prove the issue plan names every configuration field, generous default and bound, separates listener-open from authenticated readiness, includes the configuration owner, and retains the fixed-wait implementation denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      ".csdlc/prepared/issues/659/validate-runtime-convergence.sh"
    ],
    "parallel_group": "659-preparation",
    "defer_reason": null
  },
  {
    "lane": "runtime-convergence-config",
    "proof_role": "Prove backward-compatible defaults, every minimum and maximum bound, and invalid convergence configuration rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "configuration",
      "service_convergence_"
    ],
    "parallel_group": "659-runtime",
    "defer_reason": "The named nonzero tests are issue #659 implementation deliverables."
  },
  {
    "lane": "runtime-convergence-service-control",
    "proof_role": "Prove deterministic slow success, exact-stage true expiry, independent listener and full-readiness gates, service-manager ownership, and rollback or recovery.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "convergence_"
    ],
    "parallel_group": "659-runtime",
    "defer_reason": "The named nonzero tests are issue #659 implementation deliverables and do not restart the live Runtime."
  },
  {
    "lane": "runtime-convergence-quality",
    "proof_role": "Prove formatting, warning-free touched targets, and diff hygiene without broad Runtime execution.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 1200,
    "budget_tokens": 5000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--bin",
      "adl",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "659-quality",
    "defer_reason": "Run after the bounded implementation is complete."
  },
  {
    "lane": "runtime-convergence-diff",
    "proof_role": "Reject whitespace and conflict-marker defects in the issue diff.",
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
    "parallel_group": "659-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.csdlc/prepared/issues/659/validate-runtime-convergence.sh`
- `cargo test --locked --manifest-path adl-runtime-kernel/Cargo.toml --test configuration service_convergence_`
- `cargo test --locked --manifest-path adl/Cargo.toml --bin adl convergence_`
- `cargo clippy --locked --manifest-path adl/Cargo.toml --bin adl -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on invalid convergence configuration, competing Runtime ownership, loss of recoverable service state, API-timeout scope creep, live Runtime mutation, stale stack base, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
