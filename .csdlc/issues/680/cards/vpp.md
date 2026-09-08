# Validation Planning Prompt

Template: 1.0.0

Issue: 680

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/680/design.md

Diagram: .csdlc/prepared/issues/680/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-bundle",
    "proof_role": "Pre-bind issue-owned bootstrap denominator for #680 card/design coherence.",
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
      "python3",
      ".csdlc/prepared/issues/680/issue_680_validate_preparation_bundle.py"
    ],
    "parallel_group": "local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/680/issue_680_validate_preparation_bundle.py`

## Failure Semantics

Fail closed on lifecycle, credential, model-id, or validation ambiguity; record exact deferred live-provider proof rather than claiming it.

## Handoff

Retain typed evidence before convergence.
