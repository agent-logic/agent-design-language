# Structured Planning Prompt

Template: 1.0.0

Issue: 670

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Verify the exact company GCP context and budget, prepare and snapshot both immutable disks, launch the snapshot-restored two-node Polis, prove readiness and real agent/tool behavior, then destroy all issue-owned compute and disks while retaining and inventorying exactly two snapshots.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Verify project, credential, billing, quota, inventory, inputs, and worst-case spend before mutation.",
    "acceptance_ids": [
      "AC-1",
      "AC-9"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Prepare, seal, snapshot, and verify the Runtime/Guardian and Ollama/model generation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Launch the real snapshot-restored two-node Polis and capture timing, networking, resident-model, and agent/tool proof.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Destroy issue-owned compute and disks, verify exactly two snapshots retained, record residual inventory and cost, review, publish, and finish.",
    "acceptance_ids": [
      "AC-1",
      "AC-7",
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- every paid mutation names the exact company project
- projected incremental cost never exceeds USD 20.00
- normal startup is offline with respect to source, packages, and models
- Ollama remains private-only and OS Login remains the SSH authority
- cleanup preserves the intended snapshots and removes all issue-owned VMs and disks
- no tracked issue work occurs on main

## Risks

- L4 zonal quota or capacity can block launch
- the current company service-account key may lack workload mutation permissions
- the pre-existing #509 artifacts may not match the #663 sealing manifest
- snapshot creation and model hydration can dominate elapsed time and cost
- a live-only defect may require a narrow code or configuration repair before rerun

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/670/design.md

Digest: ee17d9b8c53fd883b0a394d8720aa2bfd7d6a6a74e3f59eb828c73420c8e11c4

## Diagram

.csdlc/prepared/issues/670/diagram.mmd

Digest: 29f79d1a6bae4fa54f79d3f3ccefa14c7e064f492ba77aca15c599d17fa02da5

## Stop Conditions

- project or credential identity differs from the exact authorized company target
- billing is disabled or conservative projected cost can exceed USD 20.00
- snapshot or artifact identity cannot be verified
- cleanup cannot keep projected spend below the authorization ceiling
- an unresolved required qualification or review finding remains

## Handoff

Proceed only after doctor readiness.
