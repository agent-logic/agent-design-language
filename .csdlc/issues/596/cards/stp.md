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
- PR #615 body with visible Closes #596 and non-closing #505/#534 linkage
- Issue-owned validator that rejects missing closing linkage, accidental #505/#534 closure, and any csdlc-v2 source/test diff
- V3 full-replacement denominator and real-issue canary evidence
- Captured v2 lifecycle/tooling defects for v3 replacement without mutating v2 source

## Acceptance

1. AC-1: #596 has canonical local C-SDLC v2 issue state with SIP, STP, SPP, VPP, SRP, and SOR cards
2. AC-2: PR #615 visibly closes #596 and does not close #505 or #534
3. AC-3: PR body publication is performed through typed C-SDLC v2 owners, not raw gh
4. AC-4: The remediation branch has zero net csdlc-v2 source/test mutation against origin/main
5. AC-5: V3 replacement/canary evidence is present without granting v3 lifecycle authority before #505
6. AC-6: Observed v2 lifecycle/tooling defects are captured as v3 replacement requirements rather than patched in v2
7. AC-7: Focused v2 structural validation, v3 canary, issue validator, and diff hygiene pass before handoff

## Dependencies

- #505 remains the explicit cutover gate
- #534 remains the sprint delivery umbrella
- Sprint 5/6 synthesis at /Volumes/FastWork/adl-reviews/csdlc-sprints-5-6-20260830/SYNTHESIS.md

## Inputs

- /Volumes/FastWork/adl-reviews/csdlc-sprints-5-6-20260830/SYNTHESIS.md
- PR #615
- Issue #596
- .csdlc/prepared/issues/596/pr-create-request.json
- .csdlc/evidence/604/full-cycle-defects-tail.md

## Non Goals

- Cut over from C-SDLC v2 to v3
- Merge or finish #505
- Hide unrelated sprint defects in the remediation PR
- Use raw GitHub CLI for covered lifecycle writes
