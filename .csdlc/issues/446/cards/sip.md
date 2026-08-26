# Structured Intent Prompt

Template: 1.0.0

Issue: 446

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Make ACC-governed tool execution a production Runtime capability for long-lived resident agents.

## Required Outcome

Every resident tool proposal is compiled, authority-checked, Freedom-Gate evaluated, governed-dispatched or denied, and retained as a Runtime-owned terminal receipt.

## Scope

- Typed resident tool authority
- Runtime proposal processing
- UTS-to-ACC and Freedom Gate routing
- Sealed adapter dispatch
- Redacted lineage-bound receipts

## Authority

- Provider output is proposal data only
- Runtime-loaded authority is canonical
- Only governed adapters may actuate
- #268 consumes this capability

## Assumptions

- none

## Operator Constraints

- Implement before resuming #268
- Use the #446 worktree
- Never execute #269
- Do not restore #5347 demo
