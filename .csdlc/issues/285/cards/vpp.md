# Validation Planning Prompt

Template: 1.0.0

Issue: 285

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/285/design.md

Diagram: .csdlc/prepared/issues/285/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-adr0068-birthday-governance-handoff-evidence",
    "proof_role": "Issue-local reconciliation validator for retained and terminal birthday-to-governance handoff evidence and residual gaps.",
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
      "bash",
      ".csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh"
    ],
    "parallel_group": "serial",
    "defer_reason": null
  },
  {
    "lane": "typed-issue-validation",
    "proof_role": "Typed lifecycle/card validation before review and publication.",
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
      "285"
    ],
    "parallel_group": "serial",
    "defer_reason": "Run after implementation evidence is recorded."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/evidence/285/validate_adr0068_birthday_governance_handoff_evidence.sh`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 285`

## Failure Semantics

Fail closed if a cited terminal cache, lifecycle digest, live observation, residual-gap classification, or non-goal boundary is missing or contradictory.

## Handoff

Retain typed evidence before convergence.
