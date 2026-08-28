# Structured Planning Prompt

Template: 1.0.0

Issue: 502

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Confirm #501 reviewed/published dependency, bind #502 as a stack on #501 while #501 is unmerged, implement pure lifecycle transition/capability rules, deterministic transaction storage/recovery replay, typed adapter fakes, crate-local AGENTS.md guidance, and focused transaction tests for retained requirements #168 through #170.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Confirm #501 reviewed/published dependency and choose stacked base while #501 is not merged to main.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement pure lifecycle transition and capability-checking model.",
    "acceptance_ids": [
      "AC-1",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Implement deterministic transaction storage, staged commit, and recovery replay model.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Implement typed Git/process adapter traits and fake adapters for boundary proof.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4A",
    "action": "Add csdlc-v3/AGENTS.md guidance for the v3 crate, including the non-authoritative boundary and the three-minute issue-start simplification expectation.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Run transition-matrix, transaction-failure, recovery-replay, adapter-boundary, strict-clippy, diff-hygiene, and documentation-scope validation, then obtain independent review.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  }
]

## Invariants

- C-SDLC v2 remains sole operational authority.
- Every lifecycle command/state pair has an explicit allowed or rejected outcome.
- Modeled state commits are atomic and capability-checked.
- Recovery preserves audit provenance and never silently rolls back committed authority.
- Typed adapters preserve argv/status/stdout/stderr/timeout/cancellation distinctions.
- Branch/worktree observation alone never authorizes lifecycle work.
- The crate-local AGENTS.md must stay subordinate to root AGENTS.md and must not claim operational C-SDLC v3 authority.

## Risks

- A partial modeled write is accidentally treated as authoritative.
- Recovery replay drops provenance or creates an ambiguous state.
- Adapter fakes mask shell-string or credential-scope hazards.
- Stacking on #501 creates publication/base confusion.
- Crate-local agent guidance conflicts with root v2 authority or adds ceremony instead of simplifying issue starts.
- The slice expands into V3-D workflow or v2 migration behavior.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/502/design.md

Digest: df99b55e3349870fa305781b1f093b4b17ccb52d4d497a591183392c22ad149f

## Diagram

.csdlc/prepared/issues/502/diagram.mmd

Digest: a69cf80de8610e1435636d445889d2b93d0385a417ea45824038d526d065fd21

## Stop Conditions

- Partial writes can acquire authority.
- Recovery loses provenance.
- A lifecycle decision depends on branch-name observation alone.
- A Git/process adapter accepts shell strings or ambient credentials.
- The slice performs live GitHub or lifecycle mutation.
- csdlc-v3/AGENTS.md conflicts with root AGENTS.md or blocks a prepared issue from being inspected, bound, and started quickly.
- Work expands into V3-D or v2 migration.

## Handoff

Proceed only after doctor readiness.
