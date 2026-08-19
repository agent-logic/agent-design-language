# Structured Intent Prompt

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Implement a typed C-SDLC v2 recovery/classification route for already-merged closeout authority when no source issue projection exists at the merged PR head.

## Required Outcome

Closed-by-merged v0.92 residuals with no issue projection at merged PR head can be classified or safely materialized through typed C-SDLC v2 without synthesizing normal implementation/card proof or weakening ordinary publication/terminal guards.

## Scope

- csdlc-v2 recordless/no-projection closeout recovery request/result models
- live GitHub validation for exact issue, PR, head SHA, merge SHA, repository, and closing linkage
- safe fail-closed classification for contradictory retained publication evidence
- focused tests for positive recordless terminal and negative ambiguity cases

## Authority

- Normal active-issue publication/review/finish gates remain unchanged
- Recordless terminal recovery may not invent review, implementation, publication, or card truth
- GitHub state is read-only for recovery validation
- Contradictory #248 precedence remains fail-closed until explicitly resolved

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 owners for lifecycle and publication
- Bind implementation under /Volumes/FastWork/adl-worktrees
- Do not use raw GitHub writes
- Do not edit product Runtime/Observatory/provider/Unity/AWS surfaces
- Keep primary main clean after bind
