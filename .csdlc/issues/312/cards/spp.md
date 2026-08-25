# Structured Planning Prompt

Template: 1.0.0

Issue: 312

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Prepare in parallel, then bind after #311 merges; pin merged producer revisions without waiting for closeout; regenerate the complete declared inventory; verify every current claim against canonical reviewed evidence; correct stale current docs; run structural, link, command, redaction, no-.adl, and forged-evidence negatives; retain the review packet and release-truth diff; obtain one fresh exact-head review.

## Plan

Revision 3

## Steps

[
  {
    "id": "S1",
    "action": "Verify #311's exact reviewed PR is merged into the candidate base, pin its blocked gate packet and other merged producer revisions, and ignore asynchronous closeout state.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Regenerate the exact documentation inventory and classify every surface by current owner, claim status, evidence source, and required action.",
    "acceptance_ids": [
      "AC-2",
      "AC-3"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Correct stale current documentation and release claims without rewriting historical evidence or widening into implementation.",
    "acceptance_ids": [
      "AC-3",
      "AC-4",
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run production validation and adversarial negatives for structure, links, commands, evidence, redaction, no-.adl dependencies, exact scope, and claim boundaries.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "S5",
    "action": "Retain the docs-review packet and release-truth diff, route any ADR candidate explicitly, and obtain fresh exact-head review before publication.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "pending"
  }
]

## Invariants

- A blocked #311 quality result is preserved as current documentation truth and does not serialize WP-23 execution
- Every declared documentation surface appears exactly once in the inventory
- Current claims require canonical landed evidence; absence remains an explicit blocker or non-claim
- Historical evidence and legacy provenance are immutable
- No tracked .adl path or dependency is introduced
- WP-23 does not implement missing behavior or grant release authority

## Risks

- The large documentation surface may contain internally consistent but stale claims
- Legacy migration identities may be mistaken for current canonical authority
- A link or command can resolve syntactically while naming a retired route
- Release language may overstate blocked or platform-specific behavior
- A producer may merge overlapping document changes after the candidate is pinned and must be incorporated before publication

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/312/design.md

Digest: 38fa518b25be9d70587b17fbb4c0710504ab781a6fd72a0eb0c7fa1b2dc37f95

## Diagram

.csdlc/prepared/issues/312/diagram.mmd

Digest: fe9cb325563d18e0ddbef78a83b0335cc7180474da6520add1cc60f512983c42

## Stop Conditions

- #311 is not merged into the #312 execution base or its blocked gate packet cannot be observed
- An overlapping producer merges changed document bytes that cannot be incorporated before publication
- Any declared documentation surface is missing, duplicate, or ambiguously owned
- A current claim lacks canonical landed evidence
- Fixing a documentation blocker requires product implementation outside #312

## Handoff

Proceed only after doctor readiness.
