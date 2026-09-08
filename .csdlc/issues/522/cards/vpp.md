# Validation Planning Prompt

Template: 1.0.0

Issue: 522

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/522/design.md

Diagram: .csdlc/prepared/issues/522/diagram.mmd

## Selected Lanes

[
  {
    "lane": "finding-census",
    "proof_role": "Prove every #520/#521 finding appears exactly once with preserved provenance.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/522/validate-remediation-ledger.rb",
      "census"
    ],
    "parallel_group": "ledger",
    "defer_reason": "Validator and ledger are #522 execution deliverables."
  },
  {
    "lane": "remediation-proof",
    "proof_role": "Prove each fix has exact-head review and meaningful validation and each deferral has complete ownership metadata.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "medium",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "ruby",
      ".csdlc/prepared/issues/522/validate-remediation-ledger.rb",
      "dispositions"
    ],
    "parallel_group": "ledger",
    "defer_reason": "Validator and ledger are #522 execution deliverables."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 300,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb census`
- `ruby .csdlc/prepared/issues/522/validate-remediation-ledger.rb dispositions`
- `git diff --check`

## Failure Semantics

Fail closed on dependency, census, provenance, proof, review, deferral, or release-blocker drift; never infer disposition from issue or CI state alone.

## Handoff

Retain typed evidence before convergence.
