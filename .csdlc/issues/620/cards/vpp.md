# Validation Planning Prompt

Template: 1.0.0

Issue: 620

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/620/design.md

Diagram: .csdlc/prepared/issues/620/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v0922-package-structure",
    "proof_role": "Prove the canonical package and machine-readable planning surfaces are present and structurally readable.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1600,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh",
      "structure"
    ],
    "parallel_group": "planning-structure",
    "defer_reason": null
  },
  {
    "lane": "tbd-scheduling-reconciliation",
    "proof_role": "Prove the relevant TBD denominator has explicit, non-silent scheduling dispositions.",
    "acceptance_ids": [
      "AC-6",
      "AC-7",
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh",
      "scheduling"
    ],
    "parallel_group": "tbd-audit",
    "defer_reason": null
  },
  {
    "lane": "v0922-package-consistency",
    "proof_role": "Prove issue granularity, cross-file consistency, review handoff, and exact-range diff hygiene.",
    "acceptance_ids": [
      "AC-9",
      "AC-10"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1800,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh",
      "consistency"
    ],
    "parallel_group": "planning-consistency",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh structure`
- `bash .csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh scheduling`
- `bash .csdlc/prepared/issues/620/validate-v0922-first-pass-planning.sh consistency`

## Failure Semantics

Fail closed on an incomplete package or TBD denominator, missing disposition, cross-file identity drift, unresolved placeholder, machine-local path, duplicate scope, silent scheduling decision, or unresolved actionable review finding.

## Handoff

Retain typed evidence before convergence.
