# Structured Planning Prompt

Template: 1.0.0

Issue: 678

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap the issue, bind a FastWork worktree, change the generation installer so .adl/bin/csm is a launcher to .adl/runtime-v3/current/bin/csm, add isolated fixture regression tests, update concise docs, validate, review, and publish.

## Plan

Revision 1

## Steps

[
  {
    "id": "inspect-generation-installer",
    "action": "Inspect current Runtime v3 generation installer, stable .adl/bin/csm behavior, and existing installer tests.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "implement-stable-launcher",
    "action": "Make the installer write .adl/bin/csm as a runtime-generation launcher that execs .adl/runtime-v3/current/bin/csm.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "add-regressions",
    "action": "Add isolated fixture tests for stale stable binary repair, activation, rollback, and missing generation failure.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "docs-and-proof",
    "action": "Update concise operator docs and retain issue-local validation evidence.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- The active Runtime generation remains the single source of truth for CSM, Guardian, and kernel artifacts.
- Activation and rollback switch the stable route atomically through .adl/runtime-v3/current.
- Missing or incomplete active generations fail before service mutation.
- Local validation must not restart, reload, or stop the live Runtime.

## Risks

- A copied stable binary can drift from the active generation and reject a healthy service.
- A launcher can be too path-fragile if it assumes the caller's current working directory instead of resolving from its own path.
- Tests can accidentally exercise the live Runtime if fixture boundaries are not explicit.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/678/design.md

Digest: c2702f898a72ef448edb3e65e488725e2c60fd332fca7bc038cd550bd8ace73e

## Diagram

.csdlc/prepared/issues/678/diagram.mmd

Digest: ae358d55ee65dfd2ff5ef4d65a4bf5a37bb81031096f6b58faf57b4f52a88fdf

## Stop Conditions

- The implementation requires live Runtime restart, reload, or stop.
- The installer cannot determine the repository-local current generation path safely.
- Existing stable command semantics outside CSM would be changed.
- Validation requires provider, model, agent, or Observatory mutation.

## Handoff

Proceed only after doctor readiness.
