# Validation Planning Prompt

Template: 1.0.0

Issue: 369

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/369/design.md

Diagram: .csdlc/prepared/issues/369/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-exact",
    "proof_role": "Prove exactly four named bound implemented refusal and issue-275 recovery cases with nonzero per-case denominators.",
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
      "python3",
      ".csdlc/prepared/issues/369/run_exact_focused_matrix.py"
    ],
    "parallel_group": "369-01",
    "defer_reason": null
  },
  {
    "lane": "clippy",
    "proof_role": "Reject warnings across the changed owner binary and library.",
    "acceptance_ids": [
      "AC-6"
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
    "parallel_group": "369-02",
    "defer_reason": null
  },
  {
    "lane": "scope-exact",
    "proof_role": "Reject any committed staged unstaged or untracked path outside five tooling files and exact issue-owned surfaces; enforce diff hygiene.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/369/validate_exact_scope.py"
    ],
    "parallel_group": "369-03",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `python3 .csdlc/prepared/issues/369/run_exact_focused_matrix.py`
- `cargo clippy --locked --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings`
- `python3 .csdlc/prepared/issues/369/validate_exact_scope.py`

## Failure Semantics

Fail closed on stale CAS wrong phase or approval identity later authority repeated correction topology drift scope drift or review finding.

## Handoff

Retain typed evidence before convergence.
