# Structured Planning Prompt

Template: 1.0.0

Issue: 5911

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Initialize, bind on FastWork, enforce path policy, archive and verify transcripts, validate, review, and publish without deletion.

## Plan

Revision 3

## Steps

[
  {
    "id": "step-1",
    "action": "enforce and prove the canonical FastWork worktree parent",
    "acceptance_ids": [
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "step-2",
    "action": "archive and checksum-verify the material local transcript store without deletion",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- typed binding fails before Git topology mutation on an invalid worktree parent
- archive verification precedes any deletion proposal
- source transcripts and existing worktrees remain intact

## Risks

- canonicalization of a not-yet-created worktree path
- platform-specific mount availability
- archive size and partial-copy interruption
- sensitive transcript content exposure

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/5911/design.md

Digest: d1c9aa19f2fe9852933e714905eb7ee5d03620078721157ccdafdb9b286d1991

## Diagram

.csdlc/prepared/issues/5911/design.mmd

Digest: 093bb447659406894084abe6e3b3e94b08c8e75b565714d53082db9b5a0a30ad

## Stop Conditions

- FastWork is unavailable or not writable
- typed lifecycle state conflicts with issue identity
- archive verification fails
- requested action would delete source data without separate approval

## Handoff

Proceed only after doctor readiness.
