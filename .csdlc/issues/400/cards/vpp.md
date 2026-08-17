# Validation Planning Prompt

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/400/design.md

Diagram: .csdlc/prepared/issues/400/diagram.mmd

## Selected Lanes

[
  {
    "lane": "implemented-phase-card-recovery-regression",
    "proof_role": "Prove #400 implemented-phase SPP status and STP dependency recovery with positive and negative gate5 cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovered_implemented_issue_can_correct",
      "--",
      "--nocapture"
    ],
    "parallel_group": "400-local-01",
    "defer_reason": null
  },
  {
    "lane": "implemented-phase-card-recovery-schema",
    "proof_role": "Prove public edit schema exposes the #400 recovery operations.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "public_edit_schema_exposes_implemented_recovery_card_repairs",
      "--",
      "--nocapture"
    ],
    "parallel_group": "400-local-01",
    "defer_reason": null
  },
  {
    "lane": "csdlc-v2-strict-clippy",
    "proof_role": "Reject warnings across touched C-SDLC v2 Rust targets after implementation.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
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
    "parallel_group": "400-local-02",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 recovered_implemented_issue_can_correct -- --nocapture`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 public_edit_schema_exposes_implemented_recovery_card_repairs -- --nocapture`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`

## Failure Semantics

Fail closed on stale generation/digest, unsupported phase, unsupported field, dirty topology, malformed step/dependency truth, or missing validation/review evidence.

## Handoff

Retain typed evidence before convergence.
