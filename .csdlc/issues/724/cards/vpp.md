# Validation Planning Prompt

Template: 1.0.0

Issue: 724

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/724/design.md

Diagram: .csdlc/prepared/issues/724/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-724-focused",
    "proof_role": "Prove strict parsing, typed shared dispatch, fail-closed authority, marker/readback/receipt/reconciliation behavior, command manifest compatibility, and diff hygiene.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/724/validate-simple-issue-create.sh",
      "."
    ],
    "parallel_group": "issue-724",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/724/validate-simple-issue-create.sh .`

## Failure Semantics

Fail closed on ambiguous inputs, inactive authority, unresolved authority metadata, alternate transport, absent readback, or incomplete receipt/reconciliation proof.

## Handoff

Retain typed evidence before convergence.
