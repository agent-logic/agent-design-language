# Structured Planning Prompt

Template: 1.0.0

Issue: 571

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #571, inspect the current V3-A artifacts and validator, make only bounded corrective edits, add fail-closed focused validation, run exact-range diff hygiene and contract/proof checks, then publish a reviewed PR with Closes #571.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #571 typed issue state and inventory current #500/#565 V3-A artifacts.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Repair predecessor coverage owner issue/proof lane mapping and validator rejection cases.",
    "acceptance_ids": [
      "AC-1",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Repair CONTRACT.md construction-decision evidence and retained lifecycle gate consistency.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Repair exact-range diff hygiene validation for V3-A.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run focused validation, exact-head review, and publish with Closes #571 if clean.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  }
]

## Invariants

- Every retained #161-#163 row must identify one owner issue and one proof lane.
- V3-A construction-decision evidence must bind the measured #162 disposition, thresholds or criteria, and #163/Decision 11 approval evidence.
- The default lifecycle path cannot silently omit retained bind, publication, finish, or cleanup gates.
- Diff hygiene proof must use an exact base/head range, not a clean working-tree-only check.
- C-SDLC v2 remains live authority until explicit V3-F/#505 cutover.

## Risks

- A broad prose-only predecessor mapping could continue to pass validation.
- The construction decision could remain unsupported by measured evidence.
- Lifecycle defaults could still contradict retained safety-critical gates.
- Diff hygiene could remain vacuous if checked only against the working tree.
- The corrective issue could widen into later v3 implementation or cutover authority.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/571/design.md

Digest: 89eebc366bd63a107647844064770c0ff3959678152d48ae5ae1f1999e1faedc

## Diagram

.csdlc/prepared/issues/571/diagram.mmd

Digest: 58270926aafc5ea40bc80387ba1c54a28f0ba35bb149e5db094c4d0082d48d7c

## Stop Conditions

- The patch rewrites #500/#565 historical review truth as passing.
- The patch expands into V3-B/C/D/E/F implementation or authority cutover.
- Validators still accept broad-only owner/lane mapping or working-tree-only diff hygiene.
- The issue attempts to retire v2 or treat v3 as live authority.

## Handoff

Proceed only after doctor readiness.
