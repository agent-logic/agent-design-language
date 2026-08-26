# Validation Planning Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/109/design.md

Diagram: .csdlc/prepared/issues/109/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-srp-contract",
    "proof_role": "prove fresh-session standard SRP instructions and retained exact-revision review truth",
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
    "budget_seconds": 30,
    "budget_tokens": 100,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/109/validate-fresh-session-srp.sh",
      "<immutable-base-sha>",
      "<exact-final-head-sha>",
      "<exact-reviewed-substantive-sha>"
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

- `bash .csdlc/prepared/issues/109/validate-fresh-session-srp.sh <immutable-base-sha> <exact-final-head-sha> <exact-reviewed-substantive-sha>`

## Failure Semantics

Fail closed on missing SRP authority, fresh-session handoff, exact-head repeat, or read-only reviewer boundaries.

## Handoff

Retain typed evidence before convergence.
