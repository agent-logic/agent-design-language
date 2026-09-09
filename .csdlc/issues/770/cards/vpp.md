# Validation Planning Prompt

Template: 1.0.0

Issue: 770

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/770/design.md

Diagram: .csdlc/prepared/issues/770/diagram.mmd

## Selected Lanes

[
  {
    "lane": "public-ssh-contract",
    "proof_role": "Deterministic Terraform mocked plan contracts; small local CPU; not a release gate.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/aws/csm-runtime-spot/tests/run_contract.sh"
    ],
    "parallel_group": "local",
    "defer_reason": null
  },
  {
    "lane": "authorized-live-recovery",
    "proof_role": "Paid AWS reachability, application isolation and disposal proof; requires explicit plan approval and selected existing key.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-6"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 1200,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      "infra/aws/csm-runtime-spot/tests/run_live_proof.sh"
    ],
    "parallel_group": "live",
    "defer_reason": "Prepare exact plan and request explicit cloud approval plus existing SSH key details before implementing or running this live proof target."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash infra/aws/csm-runtime-spot/tests/run_contract.sh`
- `bash infra/aws/csm-runtime-spot/tests/run_live_proof.sh`

## Failure Semantics

Fail closed on missing proof, failed validation, missing cloud authorization, or incomplete disposal.

## Handoff

Retain typed evidence before convergence.
