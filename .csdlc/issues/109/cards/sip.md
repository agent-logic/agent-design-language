# Structured Intent Prompt

Template: 1.0.0

Issue: 109

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Reduce missed findings and repeated review cycles without changing the standard SRP lifecycle.

## Required Outcome

Generate the standard exact-head SRP, send it to a fresh external review session, resolve findings, and repeat after substantive changes.

## Scope

- existing C-SDLC v2 review skill
- bounded operator runbook
- focused contract validator

## Authority

- standard SRP is sole review authority
- review session is read-only
- implementation session resolves findings

## Assumptions

- none

## Operator Constraints

- no new packet type
- no synthesis engine
- no daemon
- no scheduler
- no registry
- no claims
- no new lifecycle phase
- no redundant broad validation
