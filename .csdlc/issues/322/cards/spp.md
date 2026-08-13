# Structured Planning Prompt

Template: 1.0.0

Issue: 322

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #5913, obtain fresh design review, bind a FastWork worktree, repair the narrow adl-review read-only routing surface, validate with focused compatibility tests and strict Clippy, obtain exact-head review, and stop before publication unless authorized.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap and approve the bounded routing-repair design.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the smallest read-only adl-review compatibility routing repair.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Run focused compatibility validation and strict relevant Clippy.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Obtain fresh exact-head review and stop before publication.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- No provider credential is read or executed by this issue
- No v1 lifecycle wrapper becomes authoritative again
- Machine-readable output and human diagnostics remain truthful
- Read-only review commands do not mutate repository or lifecycle state

## Risks

- Compatibility help may overpromise commands whose implementations were intentionally sunset
- Routing repair may accidentally revive broad legacy multiplexer behavior
- CodeFriend naming still carries CodeBuddy compatibility artifacts

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/5913/design.md

Digest: 61e3dce4250f315ea1afafe8a84d54e1433ca4bd1621a47e75e1c66c0fd85df3

## Diagram

.csdlc/prepared/issues/5913/diagram.mmd

Digest: a67429101e30164192a2e66b9be496fe1f7cc3f07907e79241a4b2fcb270ce15

## Stop Conditions

- Any fix requires provider credentials or live hosted model calls
- Any implementation touches #112, #298, projection_recovery.rs, store.rs, or gate5.rs
- Typed lifecycle reports topology/review/validation drift
- Focused tests or exact-head review fail

## Handoff

Proceed only after doctor readiness.
