# Structured Intent Prompt

Template: 1.0.0

Issue: 400

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide a typed implemented-phase recovery route for review-blocking SPP step truth and STP dependency truth repairs without raw card edits or lifecycle reset.

## Required Outcome

Implemented issues can truthfully repair #117-style SPP step status and STP dependency drift through typed C-SDLC v2 operations while preserving review and publication gates.

## Scope

- implemented-phase typed card recovery
- SPP plan-step status truth repair
- STP dependency truth repair
- generation/digest CAS and audit preservation

## Authority

- typed C-SDLC v2 owner binaries remain lifecycle authority
- generated cards are not edited directly
- fresh review and publication gates remain required

## Assumptions

- none

## Operator Constraints

- keep primary main tracked files clean
- work only in the bound FastWork issue worktree after bind
- do not touch moving unrelated issue worktrees
