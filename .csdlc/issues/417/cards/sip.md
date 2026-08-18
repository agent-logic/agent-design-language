# Structured Intent Prompt

Template: 1.0.0

Issue: 417

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Remove the typed implemented-recovery deadlock while preserving recovery provenance and downstream authority invalidation.

## Required Outcome

#417 is terminally merged with focused proof that the exact implemented recovery sequence can reach authored design refresh without reviving downstream authority.

## Scope

- Implemented-state authored design recovery eligibility in csdlc-v2/src/store.rs.
- Focused exact-sequence regression tests in csdlc-v2/tests/gate5.rs.

## Authority

- Change only typed C-SDLC v2 recovery classification and focused regression coverage.
- Do not mutate issue #414, #268, or #269 lifecycle or product state.
- Do not weaken fresh-session, exact-generation, CAS, review, publication, or terminal gates.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle owners only.
- Implement in a bound FastWork issue worktree.
- Obtain fresh design and exact-head implementation reviews.
- Merge, finish, clean, and install the reviewed owner binary before #414 resumes.
