# Validation Planning Prompt

Template: 1.0.0

Issue: 485

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/485/design.md

Diagram: .csdlc/prepared/issues/485/diagram.mmd

## Selected Lanes

[
  {
    "lane": "root-recovery",
    "proof_role": "Record corporate recovery posture without removing existing administrator access.",
    "acceptance_ids": [
      "AC-1",
      "AC-7"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 180,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "root-recovery"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "identity-census",
    "proof_role": "Read back human, workload, and agent-visible IAM identities without credential disclosure.",
    "acceptance_ids": [
      "AC-2"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "identity-census"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "agent-toolkit-configuration",
    "proof_role": "Verify AWS CLI 2.35 or newer and record the approved Agent Toolkit for AWS Codex path.",
    "acceptance_ids": [
      "AC-3"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-toolkit-configuration"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "agent-iam-guardrails",
    "proof_role": "Verify agent IAM read-only default posture and context policy documentation.",
    "acceptance_ids": [
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-iam-guardrails"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "agent-activity-audit",
    "proof_role": "Verify CloudWatch and CloudTrail attribution readbacks for the configured profile and agent path.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "agent-activity-audit"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "billing-readback",
    "proof_role": "Verify billing ownership, budget, anomaly, export, and cost-attribution readbacks.",
    "acceptance_ids": [
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      "docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh",
      "--lane",
      "billing-readback"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "credential-redaction",
    "proof_role": "Reject credential material and unintended mutation verbs in retained AWS-B evidence.",
    "acceptance_ids": [
      "AC-8"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/485/validate-aws-b-baseline.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "typed-review-publication",
    "proof_role": "Prove current typed issue integrity before exact-head review and closing publication.",
    "acceptance_ids": [
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/485/validate-typed-state.sh"
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

- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane root-recovery`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane identity-census`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane agent-toolkit-configuration`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane agent-iam-guardrails`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane agent-activity-audit`
- `bash docs/milestones/v0.92.1/evidence/cloud/aws-b/run-access-billing-readbacks.sh --lane billing-readback`
- `bash .csdlc/prepared/issues/485/validate-aws-b-baseline.sh`
- `bash .csdlc/prepared/issues/485/validate-typed-state.sh`

## Failure Semantics

Fail closed on account ambiguity, billing-target ambiguity, required unapproved mutation, credential exposure, administrator-removal risk, scope drift, stale review, or red CI.

## Handoff

Retain typed evidence before convergence.
