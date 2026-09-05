# Structured Planning Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #632, bind an execution worktree, assemble and run canary/docs/readiness proof, update operator guidance, record defect dispositions, and stop for independent sprint review.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Bootstrap #632 local typed lifecycle records from the existing GitHub issue and current sprint packet.",
    "acceptance_ids": [
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Bind the #632 execution worktree after bootstrap validation passes.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Create the route coverage matrix and real issue canary evidence index.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Update docs, skills, AGENTS, onboarding, and changeover notice guidance for the v3 cutover boundary.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Validate canary/docs/readiness evidence and prepare final sprint review inputs.",
    "acceptance_ids": [
      "AC-1",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  }
]

## Invariants

- v2 remains the sole live authority until #505
- v3 proof cannot rely on hidden v2 fallback
- GitHub closeout authority requires live closing relation and successful terminal state
- Defects discovered by canaries are not silently ignored

## Risks

- Some terminal canaries require an authorized merge before proof can complete
- Stacked PR topology can prevent GitHub closing-linkage readback
- Docs may contain stale route language outside the obvious files

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/632/design.md

Digest: 6ff2e30b0473290c866aad33ad1d72c6509adc78c89f7de9167067eb0fce59be

## Diagram

.csdlc/prepared/issues/632/diagram.mmd

Digest: e7e5a1be284aaf9562bc42e1a889f950ddb41a422dad957a42621c9d4cf5ae2e

## Stop Conditions

- A v3 route fails without a safe typed repair path
- A required terminal canary needs operator merge authority
- A docs/skills update would claim v3 live authority before #505

## Handoff

Proceed only after doctor readiness.
