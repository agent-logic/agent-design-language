# Structured Intent Prompt

Template: 1.0.0

Issue: 121

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Permit majority authority to fence unavailable owners and preserve one-owner lease safety across restart.

## Required Outcome

Make activation possession operation-sensitive, commit a durable next-epoch quorum fence, preserve the portable recovery floor through snapshot and restore, and permit replacement activation only after the safe deadline.

## Scope

- adl-runtime/src/distributed/lease.rs
- adl-runtime/tests/distributed_lease.rs
- .csdlc/evidence/121

## Authority

- Issue 121 exclusively owns the two declared product paths and issue-local lifecycle/evidence
- Issue 5870 retains fencing.rs ownership and must not be edited
- Issue 5878 retains module registration and integration ownership
- No merge authority

## Assumptions

- none

## Operator Constraints

- Issue and PR exist only in agent-logic/agent-design-language
- Stack on PR 120 exact reviewed head until its parent merges
- Create an issue-bound goal before product edits
- Obtain fresh independent exact-head review before publication
- Do not merge
