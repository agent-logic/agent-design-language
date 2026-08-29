# Validation Planning Prompt

Template: 1.0.0

Issue: 493

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/493/design.md

Diagram: .csdlc/prepared/issues/493/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-gcp-d-platform-packet",
    "proof_role": "Proves #493 design packet readiness, #492 terminal dependency gate, owned-path boundaries, private-network posture, identity separation, storage-boundary invariants, and cleanup proof plan.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh",
      "--lane=all"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-review-readiness",
    "proof_role": "Proves #493 has an issue-owned executable validator and review scope before design approval; this does not claim final implementation exact-head review.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh",
      "--lane=packet"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh --lane=all`
- `bash .csdlc/prepared/issues/493/validate-gcp-d-platform-foundation.sh --lane=packet`

## Failure Semantics

Fail closed if public exposure appears, IAM is broad/key-based, storage ownership collapses, cleanup selectors are incomplete, live proof would expose credentials, or GCP-E/production/shared-VPC scope would be absorbed.

## Handoff

Retain typed evidence before convergence.
