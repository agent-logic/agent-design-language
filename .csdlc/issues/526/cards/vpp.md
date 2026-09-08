# Validation Planning Prompt

Template: 1.0.0

Issue: 526

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/526/design.md

Diagram: .csdlc/prepared/issues/526/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-526-tail10",
    "proof_role": "Prove final notes and the retained operator-authorized ceremony/readback receipt after the typed release operation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/526/validate-tail10.sh"
    ],
    "parallel_group": "ceremony-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/526/validate-tail10.sh`

## Failure Semantics

Fail closed before mutation on missing ancestry, review, authorization, candidate identity, typed route, or exact readback proof.

## Handoff

Retain typed evidence before convergence.
