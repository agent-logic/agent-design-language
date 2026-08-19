# Validation Planning Prompt

Template: 1.0.0

Issue: 256

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/256/design.md

Diagram: .csdlc/prepared/issues/256/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue256-birthday-after-observatory-packet",
    "proof_role": "Validate #256 successor authority, legacy #5836 input-only classification, merged #424 HTML Observatory startup surface, accepted #414 local resident Shepherd reference evidence, and explicit public/AWS/Unity non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      "adl/tools/validate_issue256_birthday_after_observatory.py",
      "--root",
      "/Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "birthday-contract-rust-tests",
    "proof_role": "Run the focused Runtime kernel birthday contract tests covering accepted candidate semantics, lifecycle lookalike rejection, missing evidence rejection, integrity/privacy boundaries, JCS stability, and unknown-field rejection.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "/Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory/adl-runtime-kernel/Cargo.toml",
      "--test",
      "birthday"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue256-preparation-gate",
    "proof_role": "Retain the prior issue-owned preparation gate as a regression check for scope exclusions, #84 backlog routing, #341/#343 serialization, and current successor authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/evidence/256/validate_preparation_gate.py"
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

- `python3 adl/tools/validate_issue256_birthday_after_observatory.py --root /Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory`
- `cargo test --manifest-path /Volumes/FastWork/adl-worktrees/adl-issue-256-birthday-demo-after-observatory/adl-runtime-kernel/Cargo.toml --test birthday`
- `python3 .csdlc/evidence/256/validate_preparation_gate.py`

## Failure Semantics

Fail closed before bind/implementation when Observatory, #345, or scope-exclusion gates are unsatisfied.

## Handoff

Retain typed evidence before convergence.
