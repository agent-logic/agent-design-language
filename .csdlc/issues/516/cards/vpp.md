# Validation Planning Prompt

Template: 1.0.0

Issue: 516

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/516/design.md

Diagram: .csdlc/prepared/issues/516/diagram.mmd

## Selected Lanes

[
  {
    "lane": "release-tail-denominator",
    "proof_role": "Prove every planned issue and retained dependency is represented exactly once with canonical acceptance authority.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1600,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/516/validate-release-tail-admission.sh",
      "denominator"
    ],
    "parallel_group": "census",
    "defer_reason": null
  },
  {
    "lane": "implementation-gap-analysis",
    "proof_role": "Prove each acceptance row has production implementation, meaningful validation, current review, and truthful closeout evidence or an explicit finding.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/516/validate-release-tail-admission.sh",
      "gaps"
    ],
    "parallel_group": "gap-analysis",
    "defer_reason": null
  },
  {
    "lane": "admission-consistency",
    "proof_role": "Prove Markdown, JSON, ancestry, findings, owners, and the final fail-closed admission decision agree.",
    "acceptance_ids": [
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/516/validate-release-tail-admission.sh",
      "decision"
    ],
    "parallel_group": "decision",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/516/validate-release-tail-admission.sh denominator`
- `bash .csdlc/prepared/issues/516/validate-release-tail-admission.sh gaps`
- `bash .csdlc/prepared/issues/516/validate-release-tail-admission.sh decision`

## Failure Semantics

Fail closed on an incomplete denominator, missing production-path proof, stale or contradictory evidence, unresolved collision, unresolved P0/P1 finding, unowned material gap, or disagreement between gap reports and admission decision.

## Handoff

Retain typed evidence before convergence.
