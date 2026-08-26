# Structured Intent Prompt

Template: 1.0.0

Issue: 353

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make supported post-publication review recovery and republish terminally finishable without weakening review or lineage gates.

## Required Outcome

Finish reads canonical review evidence from the exact publication metadata head while retaining reviewed-substantive ancestry and governed-drift checks.

## Scope

- csdlc-v2/src/finish.rs
- csdlc-v2/src/publication.rs
- csdlc-v2/src/model.rs
- csdlc-v2/tests/gate_finish.rs
- .csdlc/prepared/issues/353/validate_preparation.rb
- .csdlc/issues/353
- .csdlc/evidence/353

## Authority

- #349 PR #352 is read-only and remains open
- #342 is untouched
- Review and CI gates remain fail closed

## Assumptions

- none

## Operator Constraints

- Typed C-SDLC v2 only
- Direct bind after design PASS
- No raw merge or hand editing
- No #349 or #342 mutation
