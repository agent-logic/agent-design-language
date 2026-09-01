# Validation Planning Prompt

Template: 1.0.0

Issue: 496

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/496/design.md

Diagram: .csdlc/prepared/issues/496/diagram.mmd

## Selected Lanes

[
  {
    "lane": "aws-g-retirement-ledger-static",
    "proof_role": "Verifies the #496 retirement ledger inventories both CloudFormation templates, classifies consumer/reference paths with disposition-bearing rows, records #489/#495 dependency truth including merge SHAs, preserves rollback/retained-evidence boundaries, rejects deletion authority, and records the live-stack non-claim.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-g/validate-aws-g-cloudformation-retirement.sh"
    ],
    "parallel_group": "local-static",
    "defer_reason": null
  },
  {
    "lane": "aws-g-diff-hygiene",
    "proof_role": "Verifies the implemented #496 docs and lifecycle diff are whitespace-clean before exact-head review and publication.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 60,
    "budget_tokens": 500,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "local-static",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash docs/milestones/v0.92.1/evidence/cloud/aws-g/validate-aws-g-cloudformation-retirement.sh`
- `git diff --check`

## Failure Semantics

Fail closed if a template, consumer/reference path, rollback path, Terraform replacement, live-stack disposition, or credential boundary is ambiguous.

## Handoff

Retain typed evidence before convergence.
