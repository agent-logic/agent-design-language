# Structured Task Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Bounded remediation of sprint review blockers and typed-tool canary defects needed before #505 can be reviewed for cutover.

## Deliverables

- Typed six-card lifecycle state for #596
- PR #597 body with visible Closes #596 and non-closing #505/#534 linkage
- Typed GitHub PR create/update operations with owner provenance and idempotent update safety
- Single csdlc v3 binary and standalone locked CI coverage
- Behavior-backed v3 importer, durability, review, finish, and cleanup fixes
- Real issue canary evidence for #592, OBS-A/#511, and OBS-B/#512

## Acceptance

1. AC-1: #596 has canonical local C-SDLC v2 issue state with SIP, STP, SPP, VPP, SRP, and SOR cards
2. AC-2: PR #597 visibly closes #596 and does not close #505
3. AC-3: PR body create/update operations are performed through typed C-SDLC v2 GitHub owners, not raw gh
4. AC-4: Repeated PR update operation keys cannot overwrite a different governed body
5. AC-5: v3 durable transaction proof fails closed across projection crash windows
6. AC-6: v3 construction remains non-authoritative until #505 cutover
7. AC-7: Focused v2, v3, CI-policy, canary, and diff-hygiene validation passes before handoff

## Dependencies

- #505 remains the explicit cutover gate
- #534 remains the sprint delivery umbrella
- Sprint 5/6 synthesis at /Volumes/FastWork/adl-reviews/csdlc-sprints-5-6-20260830/SYNTHESIS.md

## Inputs

- /Volumes/FastWork/adl-reviews/csdlc-sprints-5-6-20260830/SYNTHESIS.md
- PR #597
- Issue #596
- .csdlc/evidence/sprints-5-6-cutover-fixes/remediation-pr-create-result.json
- .csdlc/evidence/sprints-5-6-cutover-fixes/remediation-pr597-state-after-push.json

## Non Goals

- Cut over from C-SDLC v2 to v3
- Merge or finish #505
- Hide unrelated sprint defects in the remediation PR
- Use raw GitHub CLI for covered lifecycle writes
