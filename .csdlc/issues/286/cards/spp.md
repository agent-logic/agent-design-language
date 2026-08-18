# Structured Planning Prompt

Template: 1.0.0

Issue: 286

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Bootstrap #286 from current main, validate the preparation boundary, obtain design review/approval, bind a FastWork worktree, create an issue-local ADR 0069 evidence reconciliation packet, validate, review, publish, observe CI, and finish if gates pass.

## Plan

Revision 5

## Steps

[
  {
    "id": "S1",
    "action": "Create and validate the #286 preparation packet with ADR 0069 boundary, evidence model, and #207/#288 non-claims.",
    "acceptance_ids": [
      "AC-1"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Obtain a fresh no-context design review and approve only if residual-gap and non-implementation boundaries are truthful.",
    "acceptance_ids": [
      "AC-5"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Bind in FastWork and author the issue-local ADR 0069 evidence reconciliation packet and deterministic validator.",
    "acceptance_ids": [
      "AC-2",
      "AC-3",
      "AC-4"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "Run focused proof, typed validation, fresh exact-head review, publication, required CI, and finish.",
    "acceptance_ids": [
      "AC-6"
    ],
    "status": "completed"
  }
]

## Invariants

- Evidence classification may be partial or residual-gap-bearing, but must not overclaim terminal proof.
- All accepted evidence claims need exact revision and retained review/outcome references.
- #288 owns shared ADR serialization; #286 owns only issue-local reconciliation truth.
- No credential-bound live evidence is synthesized.

## Risks

- Open WP-18A credential-gated proof may prevent ADR 0069 from being acceptance-ready.
- Evidence from sibling or parent issues could be misclassified as #286-owned implementation.
- Shared ADR documents could be edited too early instead of waiting for #288.

## Estimates

{
  "elapsed_seconds": 7200,
  "total_tokens": 40000,
  "validation_seconds": 1200
}

## Design

.csdlc/prepared/issues/286/design.md

Digest: e2df93f901096d9019c2f6361bf5f11d43982a252d244511e6feb496d8261038

## Diagram

.csdlc/prepared/issues/286/diagram.mmd

Digest: 5c343cc8b687993e6ace28a43ba7a3f1fb1afcbc755a7a321e2d53e225701a77

## Stop Conditions

- Preparation validator fails.
- Design review finds #286 claims implementation or shared ADR serialization scope.
- Evidence reconciliation would require credentials, cloud, Unity live host, or sibling issue mutation.
- Focused proof, exact review, publication, CI, or terminal finish fails.

## Handoff

Proceed only after doctor readiness.
