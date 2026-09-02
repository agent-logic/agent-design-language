# Structured Planning Prompt

Template: 1.0.0

Issue: 503

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Confirm #502 terminal dependency, bootstrap and bind #503, then implement a non-authoritative v3 local preparation path that consumes typed issue input, renders cards from the active registry, enforces registered topology during bind modeling, and emits doctor/PVF planning proof.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm #502 merged/closed-out dependency and bootstrap #503 typed issue state.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement local command contract types and issue-input handling.",
    "acceptance_ids": [
      "AC-1",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement registered-topology bind modeling for the local preparation workflow.",
    "acceptance_ids": [
      "AC-2",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Integrate active prompt-template registry card rendering into the local preparation path.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Add CLI/local workflow proof that requirements #171 through #173 reach a doctor-validated PVF plan.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- C-SDLC v2 remains sole operational authority.
- C-SDLC v3 local commands are construction-only until explicit V3-F cutover.
- Every local preparation command consumes and emits typed contracts.
- Binding proof uses registered topology rather than branch names alone.
- Generated cards come from the active prompt-template registry, not hand-edited Markdown.
- Doctor/PVF plan output distinguishes ready, blocked, failed, deferred, and skipped states.
- Issue-start simplification reduces ceremony without removing v2 authority, review, or typed validation.

## Risks

- A v3 local command accidentally becomes live lifecycle authority.
- A bind proof relies on branch-name observation instead of registered topology.
- Template rendering drifts from the active registry or requires hand edits.
- Doctor/PVF planning output conflates blocked/deferred/skipped/passed states.
- The implementation expands into V3-E remote delivery or V3-F authority cutover.
- Issue-start simplification removes necessary fail-closed checks instead of removing ceremony.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/503/design.md

Digest: f06f261a4beba106cdf8bdcde4007e4bb32e05f2c4bf6f071528f3922544d9d8

## Diagram

.csdlc/prepared/issues/503/diagram.mmd

Digest: f93d4b7764daad0bd5a65214a1930583103578ea271b56575b70b43ebda623cb

## Stop Conditions

- A local command bypasses typed state.
- Bind can be authorized from branch-name observation alone.
- Generated cards require hand edits.
- Doctor/PVF planning conflates ready, blocked, failed, deferred, skipped, or passed.
- The slice performs live GitHub or lifecycle mutation.
- The work expands into V3-E, V3-F, v2 migration, or v2 retirement.

## Handoff

Proceed only after doctor readiness.
