# Validation Planning Prompt

Template: 1.0.0

Issue: 387

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/387/design.md

Diagram: .csdlc/prepared/issues/387/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-gate5-implemented-card-repair",
    "proof_role": "Run the exact named gate5 regression proving the #114-shaped implemented-phase repair sequence, stale-CAS rejection, active review assignment/review evidence rejection, readiness/publication/terminal truth rejection, reviewed/published/merge_ready phase rejection, fresh review assignment after repair, and publication failure before fresh review evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 240,
    "budget_tokens": 1500,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_phase_card_truth_repair_unblocks_fresh_review_assignment"
    ],
    "parallel_group": "387-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-fmt",
    "proof_role": "Reject formatting drift in the changed C-SDLC v2 crate.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 500,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "parallel_group": "387-local",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-clippy-focused",
    "proof_role": "Reject warning regressions across the changed C-SDLC v2 surfaces.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 800,
    "budget_tokens": 2000,
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
    "parallel_group": "387-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_phase_card_truth_repair_unblocks_fresh_review_assignment`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --check`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale CAS, over-broad mutation authorization, failed focused regression, missing review, or publication guard drift.

## Handoff

Retain typed evidence before convergence.
