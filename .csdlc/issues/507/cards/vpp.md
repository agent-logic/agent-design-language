# Validation Planning Prompt

Template: 1.0.0

Issue: 507

Repository: agent-logic/agent-design-language

Card: vpp

Status: ready

## Summary

Execute the smallest proving validation DAG.

## Lane Inputs

Design: .csdlc/prepared/issues/507/design.md

Diagram: .csdlc/prepared/issues/507/diagram.mmd

## Selected Lanes

[
  {
    "lane": "prebind-drt-b-packet",
    "proof_role": "Proves #507 design packet readiness, #506 terminal cache, #345 closed predecessor observation via local cache or live read-only GitHub state, owned-path boundaries, six-resident denominator, paid/cloud proof gating, and pre-publication review gating before bind.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 120,
    "budget_tokens": 1500,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/507/validate-drt-b-six-resident.sh",
      "--lane=prebind"
    ],
    "parallel_group": "prebind-local",
    "defer_reason": null
  },
  {
    "lane": "drt-b-six-resident-uts",
    "proof_role": "After bind, verifies six distinct resident identities, one UTS workload receipt per resident, lineage digest, and replay cursor evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/507/validate-drt-b-six-resident.sh",
      "--lane=six-resident-uts"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #507 is bound and implementation creates the DRT-B proof surfaces."
  },
  {
    "lane": "drt-b-continuity-reclamation",
    "proof_role": "After bind, verifies dehydrate/restore exact population preservation, replay/idempotency, resource-envelope, negative-matrix, and cleanup-zero evidence.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "deterministic": true,
    "resource_profile": "small",
    "budget_seconds": 300,
    "budget_tokens": 2000,
    "argv": [
      "bash",
      ".csdlc/prepared/issues/507/validate-drt-b-six-resident.sh",
      "--lane=continuity-reclamation"
    ],
    "parallel_group": "postbind-local",
    "defer_reason": "Deferred until #507 is bound and implementation creates continuity/reclamation evidence."
  }
]

## Parallelization

Only declared parallel groups may overlap.

## Budgets

Seconds: 3600

Tokens: 25000

## Commands

- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=prebind`
- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=six-resident-uts`
- `bash .csdlc/prepared/issues/507/validate-drt-b-six-resident.sh --lane=continuity-reclamation`

## Failure Semantics

Fail closed if resident identity is label-driven, exact population is not preserved through dehydrate/restore, cleanup or cost evidence is ambiguous, cloud/GPU execution lacks explicit authorization, or #508/#509 scope is absorbed.

## Handoff

Retain typed evidence before convergence.
