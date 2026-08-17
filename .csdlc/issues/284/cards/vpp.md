# Validation Planning Prompt

Template: 1.0.0

Issue: 284

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/284/design.md

Diagram: .csdlc/prepared/issues/284/diagram.mmd

## Selected Lanes

[
  {
    "lane": "focused-adr0066-guardian-authority-evidence",
    "proof_role": "Issue-local reconciliation validator for retained #142 graph evidence and residual gaps.",
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
      ".csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh"
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
      "284"
    ],
    "parallel_group": "serial",
    "defer_reason": "Stable generated owner binary is outside Cargo target output; command is run from the issue worktree with worktree-relative root."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 1200

Tokens: 10000

## Commands

- `bash .csdlc/evidence/284/validate_adr0066_guardian_authority_evidence.sh`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 284`

## Failure Semantics

Fail closed if any cited terminal cache, evidence hash, required retained proof, residual-gap classification, or non-goal boundary is missing or contradictory.

## Handoff

Retain typed evidence before convergence.
