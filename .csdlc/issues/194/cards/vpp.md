# Validation Planning Prompt

Template: 1.0.0

Issue: 194

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/194/design.md

Diagram: .csdlc/prepared/issues/194/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue194-local-contract",
    "proof_role": "Prove template invariants, preflight denials, local command syntax, redaction behavior, and diff hygiene.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      "adl/tools/test_issue194_private_network_template.sh"
    ],
    "parallel_group": "local-contract",
    "defer_reason": null
  },
  {
    "lane": "issue194-live-private-aws",
    "proof_role": "Run bounded Agent Logic AWS private network/model proof and retain redacted receipts plus assert-zero cleanup.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3000,
    "budget_tokens": 3000,
    "argv": [
      "bash",
      "adl/tools/issue194_private_wuji_aws_runner.sh",
      "status",
      "<run-id>"
    ],
    "parallel_group": "live-private",
    "defer_reason": "Live AWS runner is manually gated to avoid accidental spend; retained receipts record exact run ids."
  },
  {
    "lane": "issue194-serial-hybrid-recovery",
    "proof_role": "Prove one Wuji voter plus two AWS voters across snapshot recovery, Wuji partition, AWS continuity, heal/demotion, and one-of-three halt.",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "large",
    "budget_seconds": 3600,
    "budget_tokens": 4000,
    "argv": [
      "bash",
      "adl/tools/issue194_private_wuji_aws_runner.sh",
      "serial-hybrid-recovery",
      "<run-id>"
    ],
    "parallel_group": "live-hybrid",
    "defer_reason": "Not implemented in the current harness; remains the primary #194 acceptance gap."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 7200

Tokens: 50000

## Commands

- `bash adl/tools/test_issue194_private_network_template.sh`
- `bash adl/tools/issue194_private_wuji_aws_runner.sh status <run-id>`
- `bash adl/tools/issue194_private_wuji_aws_runner.sh serial-hybrid-recovery <run-id>`

## Failure Semantics

Fail closed on wrong AWS profile, public route/IP/ingress, TTL outside bounds, active tagged resources before launch, SSM/model/S3 smoke failure, cleanup nonzero, missing redacted receipt, or incomplete serial hybrid proof.

## Handoff

Retain typed evidence before convergence.
