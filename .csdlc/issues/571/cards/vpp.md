# Validation Planning Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/571/design.md

Diagram: .csdlc/prepared/issues/571/diagram.mmd

## Selected Lanes

[
  {
    "lane": "v3a-predecessor-owner-proof-lanes",
    "proof_role": "Reject missing, empty, duplicated, or broad-only owner issue and proof-lane data for retained #161-#163 predecessor rows.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/571/validate-v3a-followup.rb"
    ],
    "parallel_group": "v3a-contract",
    "defer_reason": "Runs after V3-A corrective artifact edits are made."
  },
  {
    "lane": "v3a-construction-decision-evidence",
    "proof_role": "Verify CONTRACT.md records measured #162 construction-slice disposition, criteria or thresholds, and #163/Decision 11 binding.",
    "acceptance_ids": [
      "AC-2",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/571/validate-v3a-followup.rb"
    ],
    "parallel_group": "v3a-contract",
    "defer_reason": "Runs after CONTRACT.md is repaired."
  },
  {
    "lane": "v3a-lifecycle-gate-consistency",
    "proof_role": "Verify proportional-lifecycle.json default path cannot omit retained bind, publication, finish, or cleanup gates.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/571/validate-v3a-followup.rb"
    ],
    "parallel_group": "v3a-contract",
    "defer_reason": "Runs after lifecycle matrix repair."
  },
  {
    "lane": "exact-range-diff-hygiene",
    "proof_role": "Verify diff hygiene is checked against an explicit base/head range.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/571/validate-v3a-followup.rb"
    ],
    "parallel_group": "v3a-final",
    "defer_reason": "Runs after validator repair."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `ruby .csdlc/prepared/issues/571/validate-v3a-followup.rb`
- `ruby .csdlc/prepared/issues/571/validate-v3a-followup.rb`
- `ruby .csdlc/prepared/issues/571/validate-v3a-followup.rb`
- `ruby .csdlc/prepared/issues/571/validate-v3a-followup.rb`

## Failure Semantics

Fail closed on broad-only predecessor mapping, missing construction-decision evidence, contradictory lifecycle gate defaults, working-tree-only diff hygiene, historical-review rewriting, or premature v3 authority.

## Handoff

Retain typed evidence before convergence.
