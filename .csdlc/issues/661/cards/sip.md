# Structured Intent Prompt

Template: 1.0.0

Issue: 661

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make fresh resident Shepherd conversation turns use the Shepherd's configured model provider instead of a hardcoded acknowledgement.

## Required Outcome

A Shepherd turn invokes its configured provider and returns generated content in the existing reply envelope; provider failure is explicit and never becomes synthetic success.

## Scope

- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- .csdlc/prepared/issues/661
- .csdlc/issues/661

## Authority

- Issue #661 owns only Shepherd reply execution
- Issue #640 supplies configured model-backed Shepherd state
- Agent-to-agent initiation remains separate
- No live Runtime restart

## Assumptions

- none

## Operator Constraints

- Never write tracked files on main
- Do not restart or mutate the live Runtime
- Do not change Observatory attribution
- Do not widen into agent-to-agent initiation
