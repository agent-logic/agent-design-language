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
    "proof_role": "Prove #596 local lifecycle, PR-closing linkage, and non-closing #505/#534 linkage through an issue-owned validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
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
    "lane": "v2-github-pr-transport",
    "proof_role": "Prove typed PR create/update action validation, owner provenance, and idempotent conflict rejection.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 300,
    "budget_tokens": 2200,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_github_actions"
    ],
    "parallel_group": "rust-focused",
    "defer_reason": null
  },
  {
    "lane": "v3-durable-storage",
    "proof_role": "Prove durable projection repair remains required across committed-state/missing-projection crash windows.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
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
      "transactions"
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
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate_github_actions`
- `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test transactions`

## Failure Semantics

Fail closed on stale lifecycle state, missing typed issue cards, closing linkage to #505, non-idempotent PR update replay, projection repair ambiguity, or failed focused validation.

## Handoff

Retain typed evidence before convergence.
