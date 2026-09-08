# Validation Planning Prompt

Template: 1.0.0

Issue: 687

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/687/design.md

Diagram: .csdlc/prepared/issues/687/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-provider-readiness-roster",
    "proof_role": "Prove the state taxonomy, dynamic refresh classification, and roster/API consistency with deterministic local fixtures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "agent_roster"
    ],
    "parallel_group": "runtime-readiness",
    "defer_reason": null
  },
  {
    "lane": "runtime-provider-readiness-shepherd",
    "proof_role": "Prove resident Shepherd recovery classification for every inference-readiness state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1800,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "shepherd"
    ],
    "parallel_group": "runtime-readiness",
    "defer_reason": null
  },
  {
    "lane": "runtime-provider-readiness-assembly",
    "proof_role": "Prove missing or placeholder production adapters do not receive production readiness credit.",
    "acceptance_ids": [
      "AC-6",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1200,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "assembly"
    ],
    "parallel_group": "runtime-readiness",
    "defer_reason": null
  },
  {
    "lane": "rust-format-diff",
    "proof_role": "Reject formatting defects and malformed tracked diffs.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--",
      "--check"
    ],
    "parallel_group": "runtime-readiness",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test agent_roster`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test shepherd`
- `cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test assembly`
- `cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml -- --check`

## Failure Semantics

Fail closed on unknown readiness values, any non-ready communication eligibility, placeholder readiness credit, identity drift, live external dependency, or unresolved review finding.

## Handoff

Retain typed evidence before convergence.
