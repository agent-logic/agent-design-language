# Validation Planning Prompt

Template: 1.0.0

Issue: 628

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/628/design.md

Diagram: .csdlc/prepared/issues/628/diagram.mmd

## Selected Lanes

[
  {
    "lane": "628-local-route-tests",
    "proof_role": "Prove local route behavior and failure paths.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "local_commands"
    ],
    "parallel_group": "628-rust",
    "defer_reason": null
  },
  {
    "lane": "628-real-issue-canary",
    "proof_role": "Prove real issue local startup and three-minute readiness measurement.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 3000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v3/Cargo.toml",
      "--test",
      "real_issue_canary"
    ],
    "parallel_group": "628-rust",
    "defer_reason": null
  },
  {
    "lane": "628-issue-validator",
    "proof_role": "Prove local command route coverage and no v2 source changes.",
    "acceptance_ids": [
      "AC-1",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/628/validate-v3-h2-local-lifecycle.sh",
      "all"
    ],
    "parallel_group": "628-focused",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `cargo test --manifest-path csdlc-v3/Cargo.toml --test local_commands`
- `cargo test --manifest-path csdlc-v3/Cargo.toml --test real_issue_canary`
- `bash .csdlc/prepared/issues/628/validate-v3-h2-local-lifecycle.sh all`

## Failure Semantics

Fail closed on stale digest, missing lifecycle state without explicit repair, unsafe primary checkout, unsupported transition, v2 fallback, GitHub mutation, or v3 authority claim.

## Handoff

Retain typed evidence before convergence.
