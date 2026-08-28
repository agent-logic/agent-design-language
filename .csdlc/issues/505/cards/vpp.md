# Validation Planning Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/505/design.md

Diagram: .csdlc/prepared/issues/505/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-v3-f-preparation",
    "proof_role": "Prove the initialized #505 preparation packet preserves the #504 terminal dependency, v2-live authority boundary, no-silent-retirement rule, explicit operator approval gate, and future visible `Closes #505` publication linkage requirement.",
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
      ".csdlc/prepared/issues/505/validate-authority-transition-prep.rb"
    ],
    "parallel_group": "505-prebind-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/505/validate-authority-transition-prep.rb`

## Failure Semantics

Fail closed if the packet omits the #504 dependency, v2 live-authority boundary, no-silent-retirement rule, explicit operator approval gate, or future visible closing linkage.

## Handoff

Retain typed evidence before convergence.
