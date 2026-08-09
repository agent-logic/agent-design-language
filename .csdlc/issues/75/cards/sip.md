# Structured Intent Prompt

Template: 1.0.0

Issue: 75

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Represent closing and non-closing publication linkage explicitly and safely.

## Required Outcome

Publication accepts a typed closing or part_of mode, retains that mode through remote evidence, and never treats part_of as terminal issue authority.

## Scope

- csdlc-v2/src/publication.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/model.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate6.rs
- csdlc-v2/tests/gate_finish.rs

## Authority

- GitHub remote PR state remains publication observation authority
- Only closing linkage may establish terminal issue authority
- Split-repository references must remain qualified

## Assumptions

- none

## Operator Constraints

- No AWS
- No shell or Python control plane
- Use COTS serde, schemars, and strum already present
- Run focused Rust tests only before publication
