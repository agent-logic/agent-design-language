# Structured Intent Prompt

Template: 1.0.0

Issue: 244

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make cleanup-race admission and execution deadlines deterministic so the required Runtime lane and PR #242 can proceed.

## Required Outcome

The proof queues re-authentication and duplicate attachment in server order, observes the existing in-flight turn before its deadline, and receives exactly one terminal result without changing production behavior.

## Scope

- cleanup-race client-frame ordering in the regression test
- existing in-flight attachment and exactly-once terminal proof
- required Runtime lane evidence

## Authority

- Only the existing Runtime conversation owner admits and executes turns.
- Authentication and token rotation semantics remain unchanged.
- The follow-on does not alter #237 capability authority.

## Assumptions

- none

## Operator Constraints

- Keep #237 and PR #242 unchanged.
- Use typed C-SDLC v2 lifecycle owners and an issue-bound FastWork worktree.
- Run only focused repeated proof and the required Runtime lane.
