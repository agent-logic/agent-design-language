# Validation Planning Prompt

Template: 1.0.0

Issue: 407

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/407/design.md

Diagram: .csdlc/prepared/issues/407/diagram.mmd

## Selected Lanes

[
  {
    "lane": "csdlc-v2-focused",
    "proof_role": "focused regression",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "recovered_implemented_issue_can_correct_only_the_sip_goal",
      "--test",
      "gate5"
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

- `cargo test --manifest-path csdlc-v2/Cargo.toml recovered_implemented_issue_can_correct_only_the_sip_goal --test gate5`

## Failure Semantics

Fail closed on stale generation/digest, unrecovered state, or broad SIP mutation authorization.

## Handoff

Retain typed evidence before convergence.
