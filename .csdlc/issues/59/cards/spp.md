# Structured Planning Prompt

Template: 1.0.0

Issue: 59

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prove the code-ownership boundary, preserve the blocked goal as truthful history, validate and independently review the routing package, then route the bounded product contract to the Codex platform owner without an ADL implementation PR.

## Plan

Revision 2

## Steps

[
  {
    "id": "S1",
    "action": "Inspect repository code, policies, and retained telemetry consumers to locate or rule out a repo-owned create_goal admission and persistence seam.",
    "acceptance_ids": [
      "AC-1",
      "AC-2"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Record the authority boundary, invariants, external owner, and exact replacement or supersession contract in the design and diagram.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Validate the typed package and obtain independent readiness review, resolving every actionable finding without widening into a fabricated implementation.",
    "acceptance_ids": [
      "AC-5",
      "AC-6"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Route the reviewed defect contract to the OpenAI Codex goal-tool owner and await a platform fix before running the live replacement canary.",
    "acceptance_ids": [
      "AC-3",
      "AC-4"
    ],
    "status": "pending"
  }
]

## Invariants

- Blocked historical goal truth and accounting are never rewritten
- ADL does not shadow Codex thread-goal state
- Active goals remain protected from accidental overwrite
- Repository policy is not weakened as a workaround
- No code implementation proceeds without owning source authority

## Risks

- Mistaking ADL policy references for ownership of the platform goal API
- Falsely completing a blocked objective to escape the admission rule
- Creating divergent local goal state that Codex does not recognize
- Publishing an implementation PR with no executable owning seam

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/59/design.md

Digest: f525431fc1bfaea9f2854437e3b3c0ff8a4a8e2aee5d7bd6f502acd7821e5c80

## Diagram

.csdlc/prepared/issues/59/diagram.mmd

Digest: 0964dc21f2216184975cbbb77d9fdce9d9ca577846e2084d2d295784507e764d

## Stop Conditions

- No repository-owned goal admission or persistence implementation exists
- The fix requires changing a platform-provided tool contract
- The only proposed workaround weakens policy or falsifies historical state
- Authority evidence becomes ambiguous and requires operator or platform-owner clarification

## Handoff

Proceed only after doctor readiness.
