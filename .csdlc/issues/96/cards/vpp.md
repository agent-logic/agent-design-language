# Validation Planning Prompt

Template: 1.0.0

Issue: 96

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/96/design.md

Diagram: .csdlc/prepared/issues/96/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-ruby-regression",
    "proof_role": "Generated Git histories prove valid S-to-E-to-H acceptance and all specified drift and terminality rejections.",
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
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 8000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/5862/test-validate-implementation-wave.rb"
    ],
    "parallel_group": "focused",
    "defer_reason": "Deferred only until issue #96 creates its explicitly owned focused regression target; fail closed until that file exists and executes nonzero generated-history cases."
  },
  {
    "lane": "ruby-syntax",
    "proof_role": "Ruby parser accepts the validator source.",
    "acceptance_ids": [
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      "-c",
      ".csdlc/prepared/issues/5862/validate-implementation-wave.rb"
    ],
    "parallel_group": "syntax",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/5862/test-validate-implementation-wave.rb`
- `ruby -c .csdlc/prepared/issues/5862/validate-implementation-wave.rb`

## Failure Semantics

Fail closed on any product/evidence drift, wrong or ambiguous mapping, ancestry/head/merge mismatch, terminality weakness, denominator/DAG drift, native-proof omission, failed test, or unresolved review.

## Handoff

Retain typed evidence before convergence.
