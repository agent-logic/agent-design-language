# Validation Planning Prompt

Template: 1.0.0

Issue: 203

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/203/design.md

Diagram: .csdlc/prepared/issues/203/diagram.mmd

## Selected Lanes

[
  {
    "lane": "integration-closeout-proof",
    "proof_role": "Validate canonical #258/#259/#260 merge ancestry, zero product diff, current 4-test identity boundary, current 5-test caller guard, strict Clippy, and historical superseded nonclaim.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 5000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/203/produce-proof-receipt.rb"
    ],
    "parallel_group": "203-closeout",
    "defer_reason": null
  },
  {
    "lane": "integration-closeout-receipt",
    "proof_role": "Validate immutable v3 command/log digests, current denominators, strict Clippy, exact main binding, zero product diff, and historical proof superseded_nonclaim.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/203/validate-proof-receipt.rb"
    ],
    "parallel_group": "203-proof",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/203/produce-proof-receipt.rb`
- `ruby .csdlc/prepared/issues/203/validate-proof-receipt.rb`

## Failure Semantics

Fail closed on missing/invalid token, artifact mismatch, raw-store access, stale or escalated grant, wrong authority/time/membership/floor binding, local-clock canonical drift, partial publication, conflicting retry, rollback, corruption, capacity, unsafe path, zero-test proof, source drift, or unresolved review findings.

## Handoff

Retain typed evidence before convergence.
