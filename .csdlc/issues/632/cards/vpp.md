# Validation Planning Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/632/design.md

Diagram: .csdlc/prepared/issues/632/diagram.mmd

## Selected Lanes

[
  {
    "lane": "typed-issue-632",
    "proof_role": "Validate canonical #632 typed lifecycle record and six cards.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 30,
    "budget_tokens": 500,
    "argv": [
      ".adl/bin/csdlc-v2/csdlc-validate",
      "--root",
      ".",
      "issue",
      "--issue",
      "632"
    ],
    "parallel_group": "bootstrap",
    "defer_reason": null
  },
  {
    "lane": "route-coverage-matrix",
    "proof_role": "Prove every v3 command-equivalent route has real canary, deterministic fixture, or cutover-blocking disposition.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/632/validate-v3-canary-readiness.sh"
    ],
    "parallel_group": "readiness",
    "defer_reason": "Run after canary evidence and docs updates exist."
  },
  {
    "lane": "docs-route-scan",
    "proof_role": "Prove docs, skills, AGENTS, and onboarding guidance preserve the pre-cutover and post-cutover route boundary.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/632/validate-v3-guidance.sh"
    ],
    "parallel_group": "readiness",
    "defer_reason": "Run after guidance updates exist."
  },
  {
    "lane": "sprint-review-readiness",
    "proof_role": "Prove the final sprint review packet is complete enough for independent exact-head review.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/632/validate-sprint-review-readiness.sh"
    ],
    "parallel_group": "readiness",
    "defer_reason": "Run after canary evidence, defect dispositions, and guidance updates exist."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `.adl/bin/csdlc-v2/csdlc-validate --root . issue --issue 632`
- `bash .csdlc/prepared/issues/632/validate-v3-canary-readiness.sh`
- `bash .csdlc/prepared/issues/632/validate-v3-guidance.sh`
- `bash .csdlc/prepared/issues/632/validate-sprint-review-readiness.sh`

## Failure Semantics

Fail closed: any missing real canary proof, unowned defect, stale v2/v3 authority guidance, or raw gh lifecycle write blocks #505 cutover.

## Handoff

Retain typed evidence before convergence.
