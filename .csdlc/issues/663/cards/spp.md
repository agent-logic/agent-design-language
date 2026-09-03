# Structured Planning Prompt

Template: 1.0.0

Issue: 663

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Adapt the existing GCP two-node module to snapshot-restored disks, add one preparation root and one warm launch controller with timing receipts, validate focused contracts, then run a bounded review and publish; perform live GCP timing only with explicit spend authorization.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Adapt the reusable GCP two-node module for disposable Runtime and Ollama/model disks restored from exact snapshots.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Add minimal snapshot preparation and warm launch orchestration using immutable prepared content.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Add and run focused Terraform, shell-policy, topology, and timing-receipt tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Obtain bounded exact-head review, fix findings, publish, and run live timing only if explicitly authorized.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "in_progress"
  }
]

## Invariants

- ordinary workload teardown deletes restored disks and never deletes source snapshots
- Ollama has no public ingress
- normal startup is offline with respect to source, packages, and models
- timing claims name exact start and end events
- no tracked issue work occurs on main

## Risks

- G2 zonal capacity can delay or prevent restart
- snapshot identity, restored-disk device naming, or filesystem identity drift can mount the wrong content
- startup scripts can accidentally retain disposable GCS bootstrap behavior
- synthetic tests can overstate live stopped-to-ready performance

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/663/design.md

Digest: a7d297602440b61f16362fe72dbe84ea5f509085f54d23cc842c133fd1999a6d

## Diagram

.csdlc/prepared/issues/663/diagram.mmd

Digest: 1928ddccc50e68558c19efdbe8db388ed85af6ce7d71c930189139ae36f7a2a1

## Stop Conditions

- snapshot or restored-disk identity cannot be verified before activation
- implementation would require AWS or Runtime semantic changes
- paid live execution lacks explicit project and budget authorization
- focused validation or review has unresolved findings

## Handoff

Proceed only after doctor readiness.
