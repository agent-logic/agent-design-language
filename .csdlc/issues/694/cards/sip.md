# Structured Intent Prompt

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make Runtime-owned conversation history the authoritative reload source for complete ordered operator and agent transcripts.

## Required Outcome

An isolated conversation can submit operator text, receive an agent reply, discard client state, and restore both halves exactly once from bounded authorized conversation_history.v1 data.

## Scope

- Runtime conversation_history.v1 production source
- operator outbound and agent reply persistence
- Observatory reload restoration wiring
- authorization redaction pagination replay and deduplication bounds
- isolated end-to-end reload acceptance

## Authority

- Runtime history is transcript authority
- ingress.completed remains completion telemetry rather than the sole history source
- No live Wuji Runtime mutation
- No agent-to-agent redesign
- No tracked edits on main

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2
- Bind a dedicated FastWork worktree
- Test the production history and restoration flow end to end until it passes
- Use isolated fixtures or server state
- Publish non-draft without merging
