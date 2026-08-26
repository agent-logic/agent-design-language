# Structured Intent Prompt

Template: 1.0.0

Issue: 209

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Repair the three retrospective WP-14 defects that block Birthday review packet #5834.

## Required Outcome

Production Guardian/kernel ACIP dispatch, bounded pressure, typed errors, replay isolation, and OpenAPI/runtime signature parity are implemented and independently reviewed at an exact green revision.

## Scope

- Production ACIP dispatch through the real Guardian/kernel boundary
- Bounded queue pressure and typed success/error responses
- Principal-and-session-scoped replay state with bounded progression
- OpenAPI/runtime control-signature parity
- Focused adversarial and native proof

## Authority

- Authenticated principal and replay-domain identity are trusted only after runtime admission
- Caller-selected sequence values cannot authorize or poison unrelated traffic
- Merged #5832 evidence remains immutable historical input and does not prove the repair
- Issue #5834 remains blocked until this issue is reviewed, merged, and ancestral

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle operations and repo-owned binaries
- Work only in the issue-bound worktree; do not mutate main, PR197, PR191, or Sprint 3
- Assign a correctly named fresh reviewer before publication
- Merge only through typed finish after the exact reviewed remote head is green
- Keep closeout cleanup asynchronous
