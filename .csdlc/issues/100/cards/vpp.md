# Validation Planning Prompt

Template: 1.0.0

Issue: 100

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/100/design.md

Diagram: .csdlc/prepared/issues/100/diagram.mmd

## Selected Lanes

[
  {
    "lane": "recovery-manifest-and-drive-receipt",
    "proof_role": "Prove search provenance, exact ten-title coverage, non-empty content, unique canonical selection, digest integrity, source attribution, and retained approved-credential Drive readability evidence.",
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
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/100/validate-recovery.rb"
    ],
    "parallel_group": "focused-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/100/validate-recovery.rb`

## Failure Semantics

Fail closed on missing titles, ambiguous canonical selection, absent provenance, digest mismatch, unreadable Drive content, destructive upload requirements, publication or sharing drift, or synthetic substitution.

## Handoff

Retain typed evidence before convergence.
