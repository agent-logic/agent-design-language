# Structured Intent Prompt

Template: 1.0.0

Issue: 504

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one end-to-end C-SDLC v3 remote delivery workflow.

## Required Outcome

One end-to-end remote delivery workflow from an accepted PVF result to safe terminal cleanup.

## Scope

- C-SDLC v3 remote delivery command model
- Exact immutable review binding
- Explicit publication modes
- Terminal finish truth derived from reviewed publication state
- Safe cleanup transition after terminal truth
- Positive and refusal proof for predecessor requirements #174 through #178

## Authority

- C-SDLC v2 remains the live lifecycle authority until explicit V3-F/#505 cutover
- C-SDLC v3 remote delivery remains construction-only in this issue
- No v3 command may mutate GitHub, finish, clean, or independently grant authority before V3-F
- Review, publication, finish, and cleanup must remain distinct typed gates
- Implementation is blocked until #503 is terminal, reconciled, and ancestral

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle routes for all current issue state
- Bind beneath /Volumes/FastWork/adl-worktrees before tracked implementation edits
- Keep the PR body visibly linked with `Closes #504` at publication
- Stop if #503 is not terminal and ancestral when implementation would begin
- Do not use raw GitHub lifecycle writes except under audited break-glass authority
