# Validation Planning Prompt

Template: 1.0.0

Issue: 5822

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5822/design.md

Diagram: .csdlc/prepared/issues/5822/diagram.mmd

## Selected Lanes

[
  {
    "lane": "estimation-contracts-exact-target",
    "proof_role": "Prove source-derived identity and measurements, recursive calibration verification, deferred finish retention, gate-derived cycle projections, traversal rejection, and advisory semantics; zero tests fail.",
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
    "resource_profile": "medium",
    "budget_seconds": 1800,
    "budget_tokens": 9000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts"
    ],
    "parallel_group": "csdlc-v2",
    "defer_reason": null
  },
  {
    "lane": "cycle-time-comparison",
    "proof_role": "Require real equivalent terminal baseline and candidate cohorts under identical source-derived validation, review, publication, merge, and closeout gates.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "estimation_contracts",
      "cycle_comparison_derives_basis_gates_and_totals_from_verified_artifacts"
    ],
    "parallel_group": "analysis",
    "defer_reason": "Blocked: retained evidence contains no real equivalent terminal baseline and candidate cohort; issue 4617 was captured at PR publication while its issue and PR remained open."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace errors and support exact-revision review.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
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

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test estimation_contracts`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test estimation_contracts cycle_comparison_derives_basis_gates_and_totals_from_verified_artifacts`
- `git diff --check`

## Failure Semantics

Fail closed on false completion claims, path collisions, invalid lifecycle state, or missing named proof; preserve a specific blocker instead of degrading.

## Handoff

Retain typed evidence before convergence.
