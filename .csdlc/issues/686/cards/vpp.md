# Validation Planning Prompt

Template: 1.0.0

Issue: 686

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/686/design.md

Diagram: .csdlc/prepared/issues/686/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-686-contract-denominator",
    "proof_role": "Prove the production handoff and all named recovery boundaries are implemented in the owned source/test surfaces.",
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
    "budget_tokens": 500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py"
    ],
    "parallel_group": "686-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-config-generation-focused",
    "proof_role": "Exercise receipt activation, propagation, mismatch refusal, failpoints, and prior-generation restoration.",
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
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl/Cargo.toml",
      "--test",
      "csm_runtime_v3_generation"
    ],
    "parallel_group": "686-focused",
    "defer_reason": null
  },
  {
    "lane": "runtime-config-generation-diff",
    "proof_role": "Reject whitespace and conflict-marker defects.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "686-quality",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/686/issue_686_validate_config_generation_handoff.py`
- `cargo test --locked --manifest-path adl/Cargo.toml --test csm_runtime_v3_generation`
- `git diff --check`

## Failure Semantics

Fail closed without service mutation; preserve the prior committed receipt/reference and record only redacted deterministic evidence.

## Handoff

Retain typed evidence before convergence.
