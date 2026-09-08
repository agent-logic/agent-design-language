# Structured Task Prompt

Template: 1.0.0

Issue: 694

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Repair complete conversation restoration only; preserve provider, agent-to-agent, and live deployment behavior.

## Deliverables

- authoritative complete conversation_history.v1 source
- Observatory reload/reconnect restoration wiring
- privacy redaction pagination replay and deduplication enforcement
- focused deterministic Runtime and Observatory tests
- isolated end-to-end reload/reconnect acceptance

## Acceptance

1. AC-1: conversation_history.v1 exposes the operator outbound turn and corresponding generated reply in deterministic order with stable identity.
2. AC-2: A fresh Observatory load invokes Runtime history restoration and renders both ordered halves exactly once.
3. AC-3: Replay from history and live frames cannot duplicate restored turns.
4. AC-4: Authorization revocation privacy redaction and bounded page/history limits fail closed.
5. AC-5: An isolated end-to-end acceptance submits operator text, obtains an agent reply through production paths, discards UI state, restores from conversation_history.v1, and proves both halves exactly once.
6. AC-6: Focused Runtime and Observatory tests, formatter, contract checks, and exact-range diff hygiene pass.
7. AC-7: Independent exact-head review passes and a non-draft PR is published without merge.

## Dependencies

- #276 durable journal foundation
- #277 watermarks replay and receipts
- #278 history API and Observatory restoration intent
- #661 model-backed Shepherd replies
- #662 and #675 agent-to-agent behavior preserved

## Inputs

- agent-logic/agent-design-language#694
- agent-logic/agent-design-language#278
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/conversation_sessions.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- adl-runtime/tests/runtime_api_wss.rs
- demos/html-observatory/app.js
- demos/html-observatory/app.test.js

## Non Goals

- Agent-to-agent initiation redesign
- Provider or model configuration changes
- Permanent Wuji Runtime restart or mutation
- Cloud spend or deployment
- Durable-history architecture replacement
