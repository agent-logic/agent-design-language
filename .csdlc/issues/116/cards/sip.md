# Structured Intent Prompt

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Build a bounded policy-visible Observatory inbox for operator attention requests and explicit intervention outcomes.

## Required Outcome

Runtime and Observatory expose a governed attention-request lifecycle with stable source identity, priority, expiry, correlation, queue policy, inbox visibility, and explicit acknowledge, reply, defer, resolve, and refuse outcomes.

## Scope

- Typed attention-request lifecycle and policy contract
- Runtime queueing, deduplication, expiry, prioritization, rate limiting, quiet-mode, grouping, and bounded retention behavior
- Observatory inbox, unread/read state, filters, deep links, and notification preference surfaces
- Governed operator response routing through conversation paths without implicit approval
- Overload, spoofing, stale request, restart, reconnect, and recovery proof

## Authority

- Consume #112 Layer 8 authority, #270 acknowledgement API, #271 delivery-state UI, #276/#277/#278 durable journal and receipt foundations, and #114 coordination truth as dependencies only
- Do not fabricate agent identity, urgency, authority, or approval state
- Do not absorb #117 final qualification or #279/#280/#281/#282 proof-slice scope
- No push-notification vendor integration or external credential use

## Assumptions

- none

## Operator Constraints

- Use typed C-SDLC v2 lifecycle owners
- Work only in a bound FastWork worktree before source implementation
- Preserve unrelated #388 root staging
- No raw GitHub lifecycle writes
