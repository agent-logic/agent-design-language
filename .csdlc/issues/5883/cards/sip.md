# Structured Intent Prompt

Template: 1.0.0

Issue: 5883

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make csdlc-issue create the only active issue-creation command and delete the duplicate csdlc-init surface.

## Required Outcome

The duplicate binary, declarations, installed-set requirements, proof fixtures, and active operator guidance are removed while claim-free create/validate/doctor/bind behavior remains proven.

## Scope

- csdlc-v2/Cargo.toml
- csdlc-v2/src/bin/csdlc-init.rs
- csdlc-v2/operator
- csdlc-v2/src/proof.rs
- csdlc-v2/tests/gate10a.rs
- adl/tools
- docs/tooling
- AGENTS.md

## Authority

- csdlc-issue create is sole creation authority
- Historical evidence is immutable
- Issue authority remains danielbaustin/agent-design-language#5883
- Code PR publication targets agent-logic/agent-design-language
- PR body must use Closes danielbaustin/agent-design-language#5883
- This is split issue/code publication authority, not repository cutover or issue migration

## Assumptions

- none

## Operator Constraints

- No AWS
- No compatibility wrapper or alias
- No broad product suite
- Execute after #5895 and rebase before editing shared installer surfaces
