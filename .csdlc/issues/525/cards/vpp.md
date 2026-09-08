# Validation Planning Prompt

Template: 1.0.0

Issue: 525

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/525/design.md

Diagram: .csdlc/prepared/issues/525/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-525-tail09",
    "proof_role": "Prove the retained exact-revision planning review artifact, complete package validator, and diff hygiene.",
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
      ".csdlc/prepared/issues/525/validate-tail09.sh"
    ],
    "parallel_group": "review",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/525/validate-tail09.sh`

## Failure Semantics

Return changes-required on any unresolved actionable finding, incomplete denominator, stale revision, or unsupported planning claim.

## Handoff

Retain typed evidence before convergence.
