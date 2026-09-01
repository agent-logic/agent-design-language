# Validation Planning Prompt

Template: 1.0.0

Issue: 345

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Prove the exact two-node Terraform topology and runner locally, verify current AWS inputs through a live read-only preflight with zero paid launches, obtain fresh exact-head review and typed publication, then run one separately authorized paid qualification and prove model residency, Guardian/Runtime/ACC execution, and zero residue.

## Lane Inputs

Design: .csdlc/prepared/issues/345/design.md

Diagram: .csdlc/prepared/issues/345/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue345-terraform-contract",
    "proof_role": "Validate the two-node Terraform graph and sanitized saved plan: exactly two On-Demand instances, one shared key pair, mandatory /32 SSH on both, private SG-only 11434, SSM recovery attachments, encrypted delete-on-termination disks, and two-node deadline termination.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 4000,
    "argv": [
      "terraform",
      "-chdir=infra/aws/runtime/gpu-proof",
      "validate"
    ],
    "parallel_group": "local-contract",
    "defer_reason": null
  },
  {
    "lane": "issue345-runner-contract",
    "proof_role": "Execute no-paid authorization, exact-review, Terraform source/input binding, saved-plan receipt, redaction, combined-cost, cleanup, and topology-negative fixtures using only worktree-local state.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "bash",
      "adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh"
    ],
    "parallel_group": "local-contract",
    "defer_reason": null
  },
  {
    "lane": "issue345-readonly-aws-preflight",
    "proof_role": "Verify the business account, immutable artifacts, quota, pricing, resolved Runtime/GPU AMIs, subnet/VPC, and zero issue/run compute without mutation.",
    "acceptance_ids": [
      "AC-4",
      "AC-8",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "preflight"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": null
  },
  {
    "lane": "issue345-paid-two-node-proof",
    "proof_role": "Apply one saved Terraform plan from the authorized exact source and inputs, retain its digest, prove real GPU model residency and separate Runtime/Guardian/six-agent ACC execution over the private Ollama route, destroy the stack, and verify zero residue within the authorized combined cost.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/run_issue345_aws_gpu_shepherd_proof.sh",
      "run",
      "--commit",
      "EXACT_REVIEWED_SHA",
      "--run-id",
      "AUTHORIZED_RUN_ID",
      "--authorization-file",
      "WORKTREE_LOCAL_AUTHORIZATION",
      "--execute"
    ],
    "parallel_group": "aws-paid-serial",
    "defer_reason": "Requires a fresh passing exact-head review, typed publication, and new operator authorization binding the exact Terraform source, inputs, and combined two-node budget."
  },
  {
    "lane": "issue345-exact-head-review",
    "proof_role": "Review the exact candidate, Terraform plan contract, retained evidence, public redaction, two-node proof truth, and cleanup denominator before publication and finish.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "deterministic": false,
    "resource_profile": "medium",
    "budget_seconds": 900,
    "budget_tokens": 5000,
    "argv": [
      "git",
      "diff",
      "--check"
    ],
    "parallel_group": "review",
    "defer_reason": "Runs after implementation and focused validation are complete at the exact candidate revision."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `terraform -chdir=infra/aws/runtime/gpu-proof validate`
- `bash adl/tools/test_run_issue345_aws_gpu_shepherd_proof.sh`
- `bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh preflight`
- `bash adl/tools/run_issue345_aws_gpu_shepherd_proof.sh run --commit EXACT_REVIEWED_SHA --run-id AUTHORIZED_RUN_ID --authorization-file WORKTREE_LOCAL_AUTHORIZATION --execute`
- `git diff --check`

## Failure Semantics

Fail closed before paid apply on any account, review, authorization, Terraform source/input, SSH /32/key, cost, deadline, artifact, quota, topology, lock, or stale-resource ambiguity. After apply, always attempt Terraform destroy and verify zero run instances and volumes; retain bounded failure evidence and never retry or fall back to another purchase path.

## Handoff

Retain typed evidence before convergence.
