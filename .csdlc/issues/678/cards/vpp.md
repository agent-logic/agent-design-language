# Validation Planning Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/678/design.md

Diagram: .csdlc/prepared/issues/678/diagram.mmd

## Selected Lanes

[
  {
    "lane": "runtime-v3-generation-install",
    "proof_role": "Prove installer-managed stable CSM route, stale-binary repair, activation, rollback, and missing-generation failure in an isolated fixture.",
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
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/678/validate-stable-csm-route.sh"
    ],
    "parallel_group": "issue-678",
    "defer_reason": null
  },
  {
    "lane": "diff-hygiene",
    "proof_role": "Reject malformed tracked changes before review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 200,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "issue-678",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/678/validate-stable-csm-route.sh`
- `git diff --check`

## Failure Semantics

Fail closed on stale stable CSM routing, broken activation or rollback switching, missing-generation mutation risk, live Runtime mutation during validation, or exact-head review failure.

## Handoff

Retain typed evidence before convergence.
