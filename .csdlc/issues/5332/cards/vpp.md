# Validation Planning Prompt

Template: 1.0.0

Issue: 5332

Repository: danielbaustin/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/5332/design.md

Diagram: .csdlc/prepared/issues/5332/diagram.mmd

## Selected Lanes

[
  {
    "lane": "unity-ilpp-prep-shape",
    "proof_role": "Confirm issue-local preparation packet shape without Unity execution or source changes",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "true"
    ],
    "parallel_group": "unity-ilpp-prep",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `true`

## Failure Semantics

Fail closed on occupied sidecar mutation, missing native v2 issue record, stale branch confusion, Unity execution requirement, or source implementation during preparation.

## Handoff

Retain typed evidence before convergence.
