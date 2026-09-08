# Structured Task Prompt

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Persist and restore complete, causally correlated A2A transcript records through Runtime API and Observatory.

## Deliverables

- durable A2A transcript model
- authenticated history projection
- Observatory recovery
- restart and rehydration proof
- live bidirectional acceptance
- reviewed PR

## Acceptance

1. AC-1: Record sender, recipient, both message bodies, status, timestamps, and causal IDs
2. AC-2: Distinguish A2A from operator conversations
3. AC-3: Use one symmetric ACIP-governed path for every agent
4. AC-4: Survive reconnect, restart, checkpoint, and rehydration without duplication
5. AC-5: Expose a bounded authenticated API projection with correct redaction
6. AC-6: Observatory restores correct attribution
7. AC-7: Focused negative and recovery tests pass
8. AC-8: Live Wuji proof captures and validates raw redacted bidirectional A2A evidence

## Dependencies

- #707 / PR #711 first-class A2A delivery behavior

## Inputs

- agent-logic/agent-design-language#713
- adl-runtime-kernel conversation and A2A paths
- Observatory conversation history recovery

## Non Goals

- No naming redesign
- No provider redesign
- No general UI redesign
- No Shepherd-only shortcut
