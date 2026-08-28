# Structured Intent Prompt

Template: 1.0.0

Issue: 536

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Coordinate the podcast and Observatory product lanes as one dependency-safe hybrid sprint without absorbing child implementation authority.

## Required Outcome

A validated Sprint Execution Packet and typed child batch can be handed to issue-bound sessions with exact dependencies, operator gates, proof lanes, and closeout rules.

## Scope

- .csdlc/issues/536
- .csdlc/prepared/issues/536
- .csdlc/evidence/536
- adl/tools/skills/sprint-conductor/scripts/check_sprint_structured_prompt_readiness.py

## Authority

- The umbrella owns sprint coordination records only; each child owns its implementation, proof, review, publication, finish, and cleanup.

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle only
- Use issue-bound FastWork worktrees and issue-bound session goals
- Do not implement child work inside the umbrella
- Do not perform podcast directory submissions or public launch without separate explicit operator authorization
- Do not retain credentials, verification codes, recovery material, TLS private keys, or private account data
- Do not execute paid provider work without issue-specific operator authorization and budget
