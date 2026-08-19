# Structured Intent Prompt

Template: 1.0.0

Issue: 421

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Remove the typed readiness deadlock for intentional deletion deliverables while preserving exact candidate scope and fail-closed validator behavior.

## Required Outcome

#421 is terminally merged with focused proof that intentional deletion deliverables can remain in exact scope without weakening missing-validator failures.

## Scope

- Intentional-deletion deliverable representation and readiness classification in csdlc-v2/src/cards.rs.
- Focused readiness regression tests in csdlc-v2/tests/gate2.rs.

## Authority

- Change only typed C-SDLC v2 readiness classification and focused regression coverage.
- Do not mutate issue #414 product code, evidence, design, publication, AWS work, or #268/#269.
- Do not remove deleted paths from exact review scope and do not treat arbitrary missing files as acceptable.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle owners only.
- Implement in a bound FastWork issue worktree.
- Obtain fresh independent design and exact-head implementation reviews.
- Publish, merge, finish, clean, and install the reviewed owner binary before #414 resumes.
