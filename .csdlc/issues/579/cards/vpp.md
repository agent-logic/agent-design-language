# Validation Planning Prompt

Template: 1.0.0

Issue: 579

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/579/design.md

Diagram: .csdlc/prepared/issues/579/diagram.mmd

## Selected Lanes

[
  {
    "lane": "issue-579-corrective-validator",
    "proof_role": "Prove the #579 corrective scope, including AWS-F runtime-platform public-edge boundaries, closed ingress defaults, state isolation truth, proof/runbook truth, and delegation to the stable static validator.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/579/validate-aws-f-corrective.sh",
      "--lane=all"
    ],
    "parallel_group": "579-local",
    "defer_reason": null
  },
  {
    "lane": "579-diff-hygiene",
    "proof_role": "Rejects conflict artifacts and whitespace errors across the corrective diff.",
    "acceptance_ids": [
      "AC-6"
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
    "parallel_group": "579-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/579/validate-aws-f-corrective.sh --lane=all`
- `git diff --check`

## Failure Semantics

Fail closed on public-edge ownership regression, validator false pass, overstated proof, advisory-only state isolation, production-resilience overclaim, credential disclosure, paid cloud mutation without approval, or terminal #489 mutation.

## Handoff

Retain typed evidence before convergence.
