# Structured Intent Prompt

Template: 1.0.0

Issue: 301

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make title-only typed issue updates durably attributable and truthfully reconciled without changing unrelated issue body content.

## Required Outcome

A title-only update preserves the prior body, durably records its operation key, and reports reconciliation only after exact readback proves title and provenance.

## Scope

- csdlc-v2 typed GitHub issue-update owner
- focused issue-owner tests
- issue-local lifecycle and evidence

## Authority

- Typed owner remains the mutation authority
- Existing body content is preserved
- No lifecycle or card authority changes

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2
- Bind beneath /Volumes/FastWork/adl-worktrees
- Fresh exact-head review
- Stop before merge
