# Structured Planning Prompt

Template: 1.0.0

Issue: 648

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Use the existing reviewed local #622 repair as source evidence, replay it onto a #648 issue-bound correction path, prove ownership isolation offline, obtain fresh exact-head review, publish a corrective PR, and shepherd CI without touching live Runtime state.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Bind #648 to a FastWork worktree from current main and replay the run-scoped reload ownership repair.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Retain compatibility global registration only with identity-aware guard clearing.",
    "acceptance_ids": [
      "AC-3"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Add or preserve overlap, shutdown-order, and direct global-guard regression tests.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused offline production, safety, fmt, and clippy validation.",
    "acceptance_ids": [
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "Run fresh exact-head review, publish corrective PR, and shepherd CI without live Runtime mutation.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "pending"
  }
]

## Invariants

- Run-scoped ownership is the production path
- Compatibility global state is identity-aware and not used as production CSM ownership
- No cross-workflow provider snapshot consumption or clearing
- No live Runtime mutation
- No credential value loading or paid provider execution

## Risks

- Correct fix remains local if routed through already-merged PR #646
- Metadata-only commits can stale exact-head review without a supported bridge
- Compatibility fallback could regress independently of the run-scoped path
- Live Runtime ownership could be accidentally widened into this corrective issue

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/648/design.md

Digest: 752accf4705a4e5c4559b72ff5051629a23d03975617874c43eb1b5d09081baf

## Diagram

.csdlc/prepared/issues/648/diagram.mmd

Digest: 996951b2b4df6fd37781c583a86b695f0b0e09b22be42deb659c130ec995a9bc

## Stop Conditions

- A corrective branch cannot be created from current main without clobbering existing work
- The repair requires live Runtime mutation or provider credential execution
- Focused overlap/shutdown regression cannot be made nonzero and deterministic
- Typed publication cannot link a corrective PR to #648
- Required review or CI fails and cannot be repaired within #648 scope

## Handoff

Proceed only after doctor readiness.
