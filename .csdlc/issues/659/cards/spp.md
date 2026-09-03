# Structured Planning Prompt

Template: 1.0.0

Issue: 659

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bind from current main after the merged #656/#658 prerequisite, add one backward-compatible validated service-convergence policy, separate listener-open from full authenticated readiness, replace fixed operational waits while preserving service-manager ownership and recovery, run focused nonzero proof, then obtain exact-head review and publish the issue PR.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Inventory fixed Runtime-v3 service waits and define validated stage-specific convergence policy.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Apply policy to stop, unload, listener, and readiness convergence while preserving service-manager ownership and recoverable failure state.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add focused deterministic slow-success, true-expiry, invalid-config, and recovery tests.",
    "acceptance_ids": [
      "AC-2",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused validation, strict Clippy, formatting, diff hygiene, exact-head review, and publish without live restart.",
    "acceptance_ids": [
      "AC-5",
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- service manager remains sole process owner
- invalid policy fails before service mutation
- deadline failure preserves recoverability
- API request semantics remain unchanged
- main remains inspection-only

## Risks

- overly broad configuration could alter unrelated request semantics
- tests could become slow if they use wall-clock production durations
- timeout handling could accidentally stop a recoverable service
- stacking on a changing #658 head could create review drift

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/659/design.md

Digest: dbe55212ed1a7be8e1ff66040e774fa0fe144abf85537398e81d9d2e1c26b19c

## Diagram

.csdlc/prepared/issues/659/diagram.mmd

Digest: 95257e85456cc87a62c355163b700cabb00f050ad0ce18e90dd01ab6a80d177a

## Stop Conditions

- PR #658 fails with a substantive atomic-generation defect
- implementation requires live Runtime restart
- scope expands into provider, API, cloud, or Observatory behavior
- focused proof or review has unresolved findings

## Handoff

Proceed only after doctor readiness.
