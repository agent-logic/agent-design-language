# Validation Planning Prompt

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/504/design.md

Diagram: .csdlc/prepared/issues/504/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-v3-e-preparation",
    "proof_role": "Prove the initialized #504 preparation packet preserves the #503 terminal dependency, V3-E acceptance denominator, v2 authority boundary, v3 construction-only boundary, and future visible `Closes #504` publication linkage requirement.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/504/validate-remote-workflow.rb"
    ],
    "parallel_group": "504-prebind-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/504/validate-remote-workflow.rb`

## Failure Semantics

Fail closed if any v3 remote delivery surface mutates live lifecycle state, if #503 is not terminal and ancestral before implementation, if publication omits visible closing linkage, or if review/finish/cleanup gates can be bypassed.

## Handoff

Retain typed evidence before convergence.
