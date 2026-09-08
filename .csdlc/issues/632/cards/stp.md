# Structured Task Prompt

Template: 1.0.0

Issue: 632

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Prepare and execute the V3-H.6 canary/docs/readiness lane without merging #505 or claiming v3 live authority.

## Deliverables

- V3 real issue canary evidence index
- Command-equivalent route coverage matrix
- Defect register with fixed, follow-up, or cutover-blocking disposition
- Docs, skills, AGENTS, and onboarding updates for the v3 cutover boundary
- Operator changeover notification draft
- Sprint #625 review/closeout input packet
- .csdlc/prepared/issues/632/validate-v3-canary-readiness.sh
- .csdlc/prepared/issues/632/validate-v3-guidance.sh
- .csdlc/prepared/issues/632/validate-sprint-review-readiness.sh

## Acceptance

1. AC-1: Every v3 command-equivalent route is exercised by a real issue canary, a focused deterministic fixture, or an explicit cutover-blocking finding.
2. AC-2: At least one real issue reaches PR through v3 before #505 closes.
3. AC-3: Terminal finish and cleanup are proven after an authorized canary merge, or #505 remains blocked with the exact reason.
4. AC-4: All found defects are fixed in-sprint or recorded as cutover blockers/follow-ons with ownership.
5. AC-5: Docs and skills do not teach v2 as the future route after cutover, and do not teach v3 as current authority before #505.
6. AC-6: Final sprint review packet is ready for independent exact-head review.
7. AC-7: No raw gh lifecycle writes and no hidden v2 operational fallback are used as proof.

## Dependencies

- #625 sprint umbrella
- #627 command denominator and one-binary CLI shell
- #628 local lifecycle command routes
- #629 GitHub, PR, and publication routes
- #630 finish, cleanup, and cutover routes
- #631 proof, parity, soak, shadow, and install routes
- #505 final cutover decision remains open until proof is complete

## Inputs

- AGENTS.md
- csdlc-v3
- docs
- csdlc-v2/operator/skills
- .git/csdlc-v2/requests/v0921-v3-full-command-sprint/SPRINT_EXECUTION_PACKET.md
- .git/csdlc-v2/requests/v0921-v3-full-command-sprint/DEFECTS.md

## Non Goals

- Do not merge #505
- Do not retire v2 authority in this issue
- Do not merge sibling PRs
- Do not perform raw gh lifecycle writes
