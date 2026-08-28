# Validation Planning Prompt

Template: 1.0.0

Issue: 489

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/489/design.md

Diagram: .csdlc/prepared/issues/489/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-aws-runtime-platform-packet",
    "proof_role": "Proves #489 design packet readiness, #122/#488 terminal dependency gates, owned-path boundaries, no-direct-public-ingress posture, separated state ownership, and disposable cleanup invariants before bind.",
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
      ".csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "prebind-aws-runtime-platform-static",
    "proof_role": "Proves the AWS-F readback entrypoint has a static non-credentialed mode and reports cloud-mutation, credential-retention, and production-traffic posture.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/489/run-aws-f-readbacks.sh",
      "--lane=static"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "aws-f-runtime-platform-static",
    "proof_role": "After bind, verifies implemented Runtime platform Terraform, runbook, proof packet, no-direct-public-ingress invariant, state separation, and cleanup selectors.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh",
      "--phase=postbind"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #489 is bound and implementation creates the complete module/runbook/evidence surfaces."
  },
  {
    "lane": "prepublication-review-readiness-static",
    "proof_role": "Before publication, reruns the issue-owned AWS-F validator as the local readiness input for fresh exact-head review; the actual review result remains recorded by csdlc-review, not by this shell lane.",
    "acceptance_ids": [
      "AC-5"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh",
      "--phase=postbind"
    ],
    "parallel_group": "review-readiness",
    "defer_reason": "Deferred until #489 is bound and implemented; this lane supplies local proof for fresh exact-head review, while csdlc-review records the review verdict before publication."
  },
  {
    "lane": "aws-f-inventory-readonly",
    "proof_role": "After reviewed implementation and approved AWS profile, runs redacted read-only AWS identity/resource readbacks for the module denominator without mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "deterministic": false,
    "resource_profile": "small",
    "budget_seconds": 600,
    "budget_tokens": 2500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/489/run-aws-f-readbacks.sh",
      "--lane=inventory-readonly"
    ],
    "parallel_group": "aws-readonly",
    "defer_reason": "Deferred until #489 is bound, reviewed baseline exists, and approved AWS business-profile read-only context is selected without exposing credential contents."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh`
- `bash .csdlc/prepared/issues/489/run-aws-f-readbacks.sh --lane=static`
- `bash .csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh --phase=postbind`
- `bash .csdlc/prepared/issues/489/validate-aws-f-runtime-platform.sh --phase=postbind`
- `bash .csdlc/prepared/issues/489/run-aws-f-readbacks.sh --lane=inventory-readonly`

## Failure Semantics

Fail closed on direct public Runtime ingress, contradicted #122 or #488 authority, production traffic, missing cleanup proof, credential disclosure, or #496/#495 absorption.

## Handoff

Retain typed evidence before convergence.
