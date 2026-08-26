# Validation Planning Prompt

Template: 1.0.0

Issue: 388

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/388/design.md

Diagram: .csdlc/prepared/issues/388/diagram.mmd

## Selected Lanes

[
  {
    "lane": "388-preparation-validator",
    "proof_role": "Validate #388 initialized preparation bundle has issue identity, unbound topology, required tooling-defect scope markers, and explicit SOR follow-up empty-vector removal plus blank-entry refusal denominator.",
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
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/388/validate_preparation_bundle.py"
    ],
    "parallel_group": "388-prep",
    "defer_reason": "Passed during initialized/ready preparation before bind; not rerun after bind because the validator intentionally checks unbound topology."
  },
  {
    "lane": "388-focused-csdlc-store",
    "proof_role": "Prove implemented-phase SPP/VPP/SOR card-truth repair, SOR empty-vector follow-up removal, blank-entry refusal, and broader refusal matrix.",
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
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_card_truth_repair"
    ],
    "parallel_group": "388-impl",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "388-fmt",
    "proof_role": "Reject Rust formatting drift in the touched C-SDLC v2 crate.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "fmt",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--check"
    ],
    "parallel_group": "388-impl",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "388-clippy",
    "proof_role": "Reject warnings in changed C-SDLC v2 owner-tool surfaces.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
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
    "parallel_group": "388-impl",
    "defer_reason": "Runs after implementation."
  },
  {
    "lane": "388-diff",
    "proof_role": "Reject diff hygiene errors.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "388-impl",
    "defer_reason": "Runs after implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/388/validate_preparation_bundle.py`
- `cargo test --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_card_truth_repair`
- `cargo fmt --manifest-path csdlc-v2/Cargo.toml --check`
- `cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `git diff --check`

## Failure Semantics

Fail closed on stale CAS, missing/current recovery provenance, active review/publication/readiness/terminal truth, wrong card/field, empty required text, blank SOR follow-up entries, scope drift, or review finding.

## Handoff

Retain typed evidence before convergence.
