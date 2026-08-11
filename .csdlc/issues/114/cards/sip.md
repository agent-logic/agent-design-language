# Structured Intent Prompt

Template: 1.0.0

Issue: 114

Repository: agent-logic/agent-design-language

Card: sip

Status: ready

## Goal

Provide Runtime-owned, policy-governed durable human-agent conversation history that preserves ordered continuity and receipts across bounded browser and Runtime restart.

## Required Outcome

After #111 and #112 are terminal and ancestral, authorized operators can reopen, page, search, export, and delete only their public-safe conversation history with stable identities, fresh read authorization, explicit lifecycle policy, and fail-closed migration and recovery.

## Scope

- Versioned Runtime-owned redb conversation history store independent from execution checkpoint and lifelog authority
- Atomic ordered turn and monotonic outcome persistence with idempotency and receipt-chain evidence
- Freshly authorized paging, search, export, retention, deletion, migration, recovery, and Observatory transcript projection
- Focused Rust, API, browser, restart, corruption, migration, deletion, redaction, and exact-head review proof

## Authority

- Merged #111 owns canonical session, turn, sequence, correlation, causation, delivery, and response identity
- Merged #112 owns Layer 8 principal, capability, policy, revocation, refusal, replay, and redacted audit authority
- Runtime is the sole history authority; browser cache, provider transcripts, and agent-private state grant no read or restore authority
- Conversation history never becomes execution checkpoint, lifelog, policy, governance, or private-memory authority

## Assumptions

- none

## Operator Constraints

- Preparation only: do not implement product code, bind execution, publish, push, open a PR, merge, close, or mutate issue #83
- Use only typed C-SDLC v2 owners and card editors; root main remains inspection-only
- Stop initialized while #111 or #112 is not terminal through a merged revision ancestral to the execution base
