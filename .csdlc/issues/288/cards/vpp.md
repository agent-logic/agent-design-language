# Validation Planning Prompt

Template: 1.0.0

Issue: 288

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/288/design.md

Diagram: .csdlc/prepared/issues/288/diagram.mmd

## Selected Lanes

[
  {
    "lane": "final-adr-serialization-validator",
    "proof_role": "Prove the final ADR index, plan, candidate ADR 0065, evidence manifest, handoff packet, terminal child caches, status matrix, residual gaps, and non-accepted boundary agree.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 2000,
    "argv": [
      "python3",
      ".csdlc/evidence/288/validate_final_adr_serialization.py"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "typed-issue-validation",
    "proof_role": "Validate #288 typed lifecycle/card truth.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "288"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject whitespace and unintended broad churn.",
    "acceptance_ids": [
      "AC-5"
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
    "parallel_group": "serial",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/evidence/288/validate_final_adr_serialization.py`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 288`
- `git diff --check`

## Failure Semantics

Fail closed on missing/non-ancestral child terminal caches, contradictory ADR statuses, any Accepted claim, missing residual gaps, stale review truth, failed focused validation, or GitHub publication/finish drift.

## Handoff

Retain typed evidence before convergence.
