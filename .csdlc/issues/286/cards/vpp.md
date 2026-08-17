# Validation Planning Prompt

Template: 1.0.0

Issue: 286

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/286/design.md

Diagram: .csdlc/prepared/issues/286/diagram.mmd

## Selected Lanes

[
  {
    "lane": "preparation-contract",
    "proof_role": "Validate #286 preparation boundary, evidence model, residual-gap policy, and #207/#288 non-claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/286/validate_preparation_bundle.py"
    ],
    "parallel_group": "286-serial-01-prep",
    "defer_reason": null
  },
  {
    "lane": "adr0069-evidence-reconciliation",
    "proof_role": "Validate the issue-local ADR 0069 evidence reconciliation packet and residual-gap classifications.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1500,
    "argv": [
      "python3",
      ".csdlc/evidence/286/validate_adr0069_evidence_reconciliation.py"
    ],
    "parallel_group": "286-serial-02-evidence",
    "defer_reason": "Deferred until bound implementation creates the evidence packet."
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed whitespace or patch artifacts before review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "286-serial-03-diff",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `python3 .csdlc/prepared/issues/286/validate_preparation_bundle.py`
- `python3 .csdlc/evidence/286/validate_adr0069_evidence_reconciliation.py`
- `git diff --check`

## Failure Semantics

Fail closed on missing exact evidence references, overclaimed ADR acceptance, sibling/parent scope absorption, stale review, PR linkage failure, CI failure, or terminal-cache mismatch.

## Handoff

Retain typed evidence before convergence.
