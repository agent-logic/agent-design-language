# Validation Planning Prompt

Template: 1.0.0

Issue: 484

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/484/design.md

Diagram: .csdlc/prepared/issues/484/diagram.mmd

## Selected Lanes

[
  {
    "lane": "account-identity",
    "proof_role": "Verify the approved Agent Logic AWS business account profile without printing credentials.",
    "acceptance_ids": [
      "AC-1"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "aws",
      "sts",
      "get-caller-identity",
      "--profile",
      "agent-logic-admin"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "all-region-inventory",
    "proof_role": "Run read-only enabled-region and resource discovery with an explicit denominator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-a/run-readonly-inventory.sh"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "inventory-summary",
    "proof_role": "Build the accepted inventory table from retained read-only AWS readbacks.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-a/build-inventory-summary.sh",
      "."
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "redaction-and-no-mutation",
    "proof_role": "Reject credential material and mutation verbs in retained evidence.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/484/validate-aws-a-inventory.sh",
      "."
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "typed-review-publication",
    "proof_role": "Prove current typed issue integrity before exact-head review and closing publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor",
      "--repo",
      "/Volumes/FastWork/adl-worktrees/adl-issue-484-aws-resource-ownership-inventory",
      "--issue",
      "484"
    ],
    "parallel_group": "lifecycle",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `aws sts get-caller-identity --profile agent-logic-admin`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-a/run-readonly-inventory.sh`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-a/build-inventory-summary.sh .`
- `bash .csdlc/prepared/issues/484/validate-aws-a-inventory.sh .`
- `/Users/daniel/git/agent-design-language/.adl/bin/csdlc-v2/csdlc-doctor --repo /Volumes/FastWork/adl-worktrees/adl-issue-484-aws-resource-ownership-inventory --issue 484`

## Failure Semantics

Fail closed on account ambiguity, mutation requirement, unclassified resources, credential exposure, scope drift, stale review, or red CI.

## Handoff

Retain typed evidence before convergence.
