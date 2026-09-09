# Structured Planning Prompt

Template: 1.0.0

Issue: 762

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Authenticate operational context before proof/install side effects; make historical routes dormant; prove stable-binary linked-worktree topology and negative guards.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Authenticate invoking issue worktree, branch, exact HEAD and common Git directory before mutation.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Confine every operational write to the registered issue worktree; reject primary and escaping paths.",
    "acceptance_ids": [
      "AC-2"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Classify proof, install, shadow and soak explicitly; historical routes cannot mutate.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Prove one stable installed binary works from linked worktrees and rejects invalid contexts without writes.",
    "acceptance_ids": [
      "AC-4"
    ],
    "status": "completed"
  }
]

## Invariants

- Exact Git identity and native selector authentication; no writes on primary; machine JSON on stdout and human diagnostics on stderr.

## Risks

- Stale issue state, forged registration, symlink containment, historical callable side effects.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/762/design.md

Digest: b23da49e00997146f824a917b855636d1b5aba5afb306c3596799a5d6a1edd4d

## Diagram

.csdlc/prepared/issues/762/diagram.mmd

Digest: f2e4a009d598d919ce2e1479cdfbaaa5cf2bd217b19db7cd35fbe5aee9667542

## Stop Conditions

- Stop on unowned issue state or identity ambiguity.

## Handoff

Proceed only after doctor readiness.
