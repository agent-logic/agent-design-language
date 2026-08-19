# Structured Planning Prompt

Template: 1.0.0

Issue: 292

Repository: agent-logic/agent-design-language

Card: spp

Status: ready

## Summary

Add the typed operation, enforce authorization predicates, render/audit the atomic identity update, add isolated fixture tests including #112-derived evidence, validate, obtain #119-compliant fresh-session exact-head review, publish a ready PR, and stop before merge.

## Plan

Revision 1

## Steps

[
  {
    "id": "P1",
    "action": "Bootstrap, doctor, and bind #292 to the dedicated FastWork worktree.",
    "acceptance_ids": [
      "AC-8"
    ],
    "status": "completed"
  },
  {
    "id": "P2",
    "action": "Implement correct_identity_title_slug_after_decomposition in csdlc-edit with predicates, evidence binding, slug checks, sibling rejects, atomic update, render, digest, and audit.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6"
    ],
    "status": "completed"
  },
  {
    "id": "P3",
    "action": "Add isolated tests including #112-derived fixture validation without mutating #112.",
    "acceptance_ids": [
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "P4",
    "action": "Run validation, obtain fresh exact-head review, publish ready PR, shepherd required checks, and stop before merge.",
    "acceptance_ids": [
      "AC-8",
      "AC-9"
    ],
    "status": "completed"
  }
]

## Invariants

- All six cards have equal identity title and slug after a successful operation.
- Only identity title/slug plus normal generation/digest/render/audit projections change.
- Failures leave state unchanged.
- Review and publication remain fail-closed without current exact-head evidence.

## Risks

- Overbroad operation could rewrite non-identity card content.
- Weak title predicates could accidentally claim sibling issue scope.
- Fixture setup could mutate #112 if not isolated.
- Publication could proceed without fresh #119-compliant review if review truth is mishandled.

## Estimates

{
  "elapsed_seconds": 21600,
  "total_tokens": 80000,
  "validation_seconds": 3600
}

## Design

.csdlc/prepared/issues/292/design.md

Digest: d9f07d34e65d0f0e12343ef7be532ddd65573ae07878eb435f584353437d8a08

## Diagram

.csdlc/prepared/issues/292/diagram.mmd

Digest: ddb7bd579661bb727935d5276c73192db8a7d289cb7ac062140e1ea40b24e69a

## Stop Conditions

- Typed bootstrap/doctor/bind rejects topology or dirty-state conditions.
- The dedicated FastWork worktree cannot be bound safely.
- The implementation requires #112 mutation.
- Fresh-session review reports actionable findings not fixed in scope.
- Required validation or hosted required checks fail.

## Handoff

Proceed only after doctor readiness.
