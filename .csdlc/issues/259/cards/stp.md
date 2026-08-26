# Structured Task Prompt

Template: 1.0.0

Issue: 259

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Governed transport certificate/authority binding only; distributed Runtime caller migration remains #260 and parent integration remains #203.

## Deliverables

- Governed transport code path consumes authority-bound certificate handles from the #258 adapter boundary.
- Focused positive and negative transport authority tests.
- Truthful validation/review/publication evidence for #259.

## Acceptance

1. AC-1: Governed transport authorization succeeds through an authority-bound certificate handle.
2. AC-2: Governed transport authorization rejects raw-store or caller-nominated certificate authority bypass.
3. AC-3: #259 changed paths do not migrate non-transport distributed Runtime callers owned by #260.
4. AC-4: Focused Runtime transport tests and strict Clippy pass before publication.
5. AC-5: Fresh exact-head review finds no actionable P1/P2 findings before publication.

## Dependencies

- #191 terminal/reconciled/ancestral
- #201 terminal/reconciled/ancestral
- #202 terminal/reconciled/ancestral
- #199 terminal/reconciled/ancestral
- #200 terminal/reconciled/ancestral
- #258 terminal/reconciled/ancestral at 193f77d24a693f955a2fcf3bdfc759ad1db8aff4

## Inputs

- .git/csdlc-v2/derived-terminal/258.json
- .git/csdlc-v2/operator-packets/issue-203-recovery-readiness-after-258-20260813T092714Z.md
- .git/csdlc-v2/operator-packets/post-258-259-260-readiness-audit-20260813T092537Z.md

## Non Goals

- Migration/recovery/peripheral caller migration owned by #260
- Parent #203 integration/publication/closeout
- #205 Shepherd/Observatory serving-eligibility authority
- Cleanup or mutation of preserved #203 worktrees
