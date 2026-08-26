# Structured Planning Prompt

Template: 1.0.0

Issue: 541

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Update onboarding and any directly adjacent workflow guidance so docs name Gate 10D2 typed C-SDLC v2, canonical repository identity, installed binary location, and root/worktree expectations truthfully, then run focused text and diff validation.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Inspect onboarding and adjacent workflow docs for stale lifecycle route and repository identity claims.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Update the stale docs to point at Gate 10D2 typed C-SDLC v2, `.adl/bin/csdlc-v2/`, and `csdlc-v2/operator/skills/` without widening scope.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-4"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Clarify canonical `agent-logic` versus legacy `danielbaustin` repository wording and preserve review/publication/finish/cleanup boundaries.",
    "acceptance_ids": [
      "AC-3",
      "AC-5"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
    "action": "Run focused docs validation and record truthful outcome evidence.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "pending"
  }
]

## Invariants

- Gate 10D2 typed C-SDLC v2 remains current lifecycle authority
- Primary checkout stays on main
- Implementation happens only in a bound issue worktree
- Generated cards remain typed projections
- Legacy repository identity is provenance, not default current authority

## Risks

- Leaving a stale compatibility route in contributor-facing docs
- Overcorrecting historical docs that intentionally describe old milestones
- Confusing legacy repository provenance with canonical current authority
- Claiming terminal proof from non-terminal lifecycle state

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/541/design.md

Digest: 26eabe1bccbd42d412d0c3a0d224078096405e8c78d1fda9c8e4833a5123ebbf

## Diagram

.csdlc/prepared/issues/541/diagram.mmd

Digest: 8b5caa43a7f9fba6938424fa49b6af7619e4086c5018b4e0c04fdef889726231

## Stop Conditions

- A proposed edit would change runtime behavior or lifecycle tooling rather than docs
- A docs surface is intentionally historical and should not be normalized as current guidance
- The fix would require non-doc lifecycle policy changes

## Handoff

Proceed only after doctor readiness.
