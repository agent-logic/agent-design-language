# Validation Planning Prompt

Template: 1.0.0

Issue: 624

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/624/design.md

Diagram: .csdlc/prepared/issues/624/diagram.mmd

## Selected Lanes

[
  {
    "lane": "corp-sidecar-hardening-register",
    "proof_role": "Prove #624 row denominator completeness, proof/follow-on disposition, evidence references, non-mutation posture, redaction hygiene, and review-ready boundary truth.",
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
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1200,
    "argv": [
      "python3",
      ".csdlc/prepared/issues/624/validate-corp-sidecar-hardening.py"
    ],
    "parallel_group": "corporate-sidecar-docs",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Prove the committed range has no whitespace errors.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 400,
    "argv": [
      "git",
      "diff",
      "--check",
      "origin/main...HEAD"
    ],
    "parallel_group": "corporate-sidecar-docs",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `python3 .csdlc/prepared/issues/624/validate-corp-sidecar-hardening.py`
- `git diff --check origin/main...HEAD`

## Failure Semantics

Fail closed on missing row disposition, missing evidence reference for proven rows, live/admin mutation claim, secret-like material, account identifiers, #497/#624 scope conflation, or diff hygiene failure.

## Handoff

Retain typed evidence before convergence.
