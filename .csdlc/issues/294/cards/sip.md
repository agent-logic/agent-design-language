# Structured Intent Prompt

Template: 1.0.0

Issue: 294

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Recover initialized design identity and authored artifact paths safely before bind.

## Required Outcome

A typed, CAS-guarded initialized recovery atomically canonicalizes reviewer evidence, relocates design and diagram artifacts to safe paths, preserves append-only audit truth, and bootstrap rejects unsafe authored paths.

## Scope

- csdlc-v2 typed issue state and request contracts
- initialized design-envelope recovery
- bootstrap authored-path validation
- focused unit and linked-worktree integration tests

## Authority

- Only initialized and unbound records are recoverable
- No direct issue record or card edits
- #292 is a read-only reproduction fixture
- No merge or closeout authority

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 binaries only
- Bind under /Volumes/FastWork/adl-worktrees
- Do not mutate #292 or root #114/#292 locks
- Use canonical fresh-session UUID review evidence
