# Structured Planning Prompt

Template: 1.0.0

Issue: 280

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap and review #280 as a bounded large-Polis performance/recovery proof issue; after terminal dependency validation, bind a FastWork worktree, add deterministic proof and narrow UI fixes if needed, then exact-review, publish, observe CI, and finish.

## Plan

Revision 1

## Steps

[
  {
    "id": "S1",
    "action": "Create the #280 six-card packet and issue-owned preparation validator that proves live issue identity, dependency ancestry, and scope boundaries.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Obtain a fresh no-context design review and approve only if #280 remains a performance/recovery proof issue with truthful #279/#281/#282 deferrals.",
    "acceptance_ids": [
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Bind in a FastWork issue worktree from current main and implement the smallest deterministic large-Polis performance/recovery proof plus any narrowly required Observatory fixes.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused proof, strict relevant checks, diff hygiene, fresh exact-head review, typed publication, required CI, and typed finish.",
    "acceptance_ids": [
      "AC-4",
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Every degradation/recovery state shown in the browser is representational and tied to Runtime-owned evidence or deterministic fixture input
- Large-Polis proof must not depend on hidden network, provider, cloud, credential, paid runner, or Unity state
- Browser changes cannot grant policy authority, synthesize delivery, mask refusal, or hide stale authorization
- #279/#281/#282 and parent coordination records stay out of #280 source/lifecycle ownership

## Risks

- A proof-only issue could uncover defects that belong to Runtime or sibling proof owners rather than #280
- Performance metrics could become brittle if they rely on machine-specific absolute timing instead of bounded deterministic counts and generous local budgets
- Fixture-based proof could overclaim live integrated behavior unless exact candidate revision and non-live boundary are recorded
- Recovery proof could accidentally validate UI-only state while missing duplicate-action prevention

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/280/design.md

Digest: a92007752035c6440f1b03e88ed42c8588ecb19e65dd3bae7dff52d90740458c

## Diagram

.csdlc/prepared/issues/280/diagram.mmd

Digest: bf28037d2821453aaa37d62e96765fd69050206b2b6219f5d98198eff509d6c6

## Stop Conditions

- Any required dependency merge is missing, noncanonical, or nonancestral
- Design review finds #280 claims Runtime authority or #279/#281/#282 scope
- A required proof needs credentials, cloud/public deployment, Unity live host, or paid runner
- Implementation would require Runtime contract changes or sibling issue worktree mutation
- Focused proof, review, CI, publication, or terminal finish fails

## Handoff

Proceed only after doctor readiness.
