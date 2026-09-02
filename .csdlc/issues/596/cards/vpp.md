# Validation Planning Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/596/design.md

Diagram: .csdlc/prepared/issues/596/diagram.mmd

## Selected Lanes

[
  {
    "lane": "remediation-regression",
    "proof_role": "Prove #596 local lifecycle, PR-closing linkage, non-closing #505/#534 linkage, portable owner-lane sources, and no net csdlc-v2 source/test mutation through the issue-owned validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/596/validate-remediation-regression.sh"
    ],
    "parallel_group": "policy",
    "defer_reason": null
  },
  {
    "lane": "v3-real-issue-canary",
    "proof_role": "Exercise C-SDLC v3 against real issue records while preserving its non-authoritative pre-cutover boundary.",
    "acceptance_ids": [
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 360,
    "budget_tokens": 2600,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/596/validate-remediation-regression.sh`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test real_issue_canary`

## Failure Semantics

Fail closed on stale lifecycle state, missing typed issue cards, closing linkage to #505, non-idempotent PR update replay, projection repair ambiguity, or failed focused validation.

## Handoff

Retain typed evidence before convergence.
