# Validation Planning Prompt

Template: 1.0.0

Issue: 363

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/363/design.md

Diagram: .csdlc/prepared/issues/363/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-363-preparation",
    "proof_role": "Prove exact issue packet and recovery-epoch markers before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/363/validate_preparation.py"
    ],
    "parallel_group": "363-00",
    "defer_reason": null
  },
  {
    "lane": "existing-recovery-feasibility",
    "proof_role": "Run the closest existing gate proving immediate recovered Implemented SPP summary correction and refusal behavior; this establishes feasibility only, not #363 completion.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "recovered_issue_can_correct_only_the_spp_plan_summary"
    ],
    "parallel_group": "363-01",
    "defer_reason": null
  },
  {
    "lane": "sequenced-recovery-regression",
    "proof_role": "After bind, prove the exact #274 intervening-operation recovery epoch and complete refusal matrix.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 12000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_plan_summary_recovery_survives_allowed_intervening_repairs"
    ],
    "parallel_group": "363-02",
    "defer_reason": "Deferred until approved bind creates the issue-owned regression."
  },
  {
    "lane": "clippy",
    "proof_role": "Reject warnings after implementation.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "large",
    "budget_seconds": 1200,
    "budget_tokens": 8000,
    "argv": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "parallel_group": "363-03",
    "defer_reason": "Deferred until implementation."
  },
  {
    "lane": "diff",
    "proof_role": "Reject diff hygiene errors.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "git",
      "diff",
      "--check",
      "dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD"
    ],
    "parallel_group": "363-04",
    "defer_reason": "Deferred until implementation."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/363/validate_preparation.py`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate5 recovered_issue_can_correct_only_the_spp_plan_summary`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_plan_summary_recovery_survives_allowed_intervening_repairs`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `git diff --check dae957c435b73d87af1f36d4e15fb088f6fd055b...HEAD`

## Failure Semantics

Fail closed on stale CAS missing recovery unsafe intervening operation review publication terminal truth scope drift or finding.

## Handoff

Retain typed evidence before convergence.
