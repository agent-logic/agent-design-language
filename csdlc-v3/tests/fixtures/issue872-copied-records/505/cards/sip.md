# Structured Intent Prompt

Template: 1.0.0

Issue: 505

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Produce one operator-reviewed C-SDLC v3 authority-transition decision.

## Required Outcome

A non-authoritative C-SDLC v3 authority-transition decision packet that binds requirements #179 and #180, measured parity, migration canary rollback, observation evidence, and explicit operator disposition.

## Scope

- csdlc-v3/tests/parity/**
- docs/tooling/csdlc-v3/**
- docs/milestones/v0.92.1/evidence/csdlc-v3/v3-f/**
- .csdlc/issues/505/**
- .csdlc/prepared/issues/505/**
- .csdlc/evidence/505/**

## Authority

- C-SDLC v2 remains the sole operational lifecycle authority until #505 records explicit operator approval and the proven transition.
- Issue completion is exactly one authority-transition decision; parity, canary, rollback, and observation are evidence inputs and do not close independently.
- The #505 slice must not silently retire v2, claim unsupported platform parity, or treat planned evidence as live authority.

## Assumptions

- none

## Operator Constraints

- Do not execute #505 until #504 is terminal through merged/closed issue and typed closeout truth.
- Keep #505 ordered after V3-E issue #504.
- Cutover and retirement require explicit operator approval.
- Preserve explicit GitHub closing-linkage visibility in any later PR body, normally `Closes #505`.
