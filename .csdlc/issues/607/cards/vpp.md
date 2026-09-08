# Validation Planning Prompt

Template: 1.0.0

Issue: 607

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/607/design.md

Diagram: .csdlc/prepared/issues/607/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue607-local-contracts",
    "proof_role": "Prove Terraform topology, EC2 user-data size, compressed script reconstruction, partial-state cleanup, immutable warm-volume activation, source-bound recovery evidence, authorization, residue, and cost calculations without AWS mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 3500,
    "argv": [
      "bash",
      "adl/tools/test_issue607_warm_polis.sh",
      "all"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue607-current-quota-live-payload-recovery",
    "proof_role": "Launch the retained two-8B-model warm Polis on r7i.2xlarge plus one-L4 g6.xlarge and prove shape identity, bounded readiness, both resident models, all six ACC cycles, source-bound degradation and Vector recovery, teardown, and zero disposable residue.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7",
      "AC-8",
      "AC-9",
      "AC-10",
      "AC-11",
      "AC-12"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 420,
    "budget_tokens": 6000,
    "argv": [
      "bash",
      "adl/tools/run_issue607_warm_polis.sh",
      "qualification-payload-recovery",
      "--commit",
      "7be87dd22260d30a7966d1b129123e84bb761074",
      "--run-id",
      "adl-issue607-e8925c1dc8b0-payload-recovery",
      "--storage-id",
      "adl-issue607-warm-v6",
      "--authorization-file",
      ".adl/local/issue607/runs/adl-issue607-e8925c1dc8b0-payload-recovery/authorization.json",
      "--execute"
    ],
    "parallel_group": "aws-paid-serial",
    "defer_reason": null
  },
  {
    "lane": "issue607-typed-spp-risk-repair",
    "proof_role": "Prove that implemented-phase SPP risk correction is authorized only as a review-recovery truth repair.",
    "acceptance_ids": [
      "AC-12"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "implemented_spp_risk_repair_requires_review_recovery_epoch"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "issue607-diff-hygiene",
    "proof_role": "Reject branch diff hygiene defects before publication.",
    "acceptance_ids": [
      "AC-12"
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
    "parallel_group": "hygiene",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_issue607_warm_polis.sh all`
- `bash adl/tools/run_issue607_warm_polis.sh qualification-payload-recovery --commit 7be87dd22260d30a7966d1b129123e84bb761074 --run-id adl-issue607-e8925c1dc8b0-payload-recovery --storage-id adl-issue607-warm-v6 --authorization-file .adl/local/issue607/runs/adl-issue607-e8925c1dc8b0-payload-recovery/authorization.json --execute`
- `cargo test --locked --manifest-path csdlc-v2/Cargo.toml --test gate5 implemented_spp_risk_repair_requires_review_recovery_epoch`
- `git diff --check`

## Failure Semantics

Fail closed before AWS mutation on identity review artifact volume AZ cost timing or cleanup ambiguity; after apply always destroy disposable compute while preserving only exact authorized warm volumes; never convert a missed startup target into PASS.

## Handoff

Retain typed evidence before convergence.
