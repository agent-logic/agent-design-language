# Structured Task Prompt

Template: 1.0.0

Issue: 662

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add the bounded Runtime path and deterministic proof for one governed resident agent initiating work addressed to another admitted resident agent.

## Deliverables

- Governed agent-to-agent initiation path
- Distinct identity and correlation model for initiated work
- Configured-provider recipient execution proof
- Replay/idempotency or explicit rejection proof
- Cancellation and missing/stale/unauthorized/provider-failure terminal proof
- Observatory/Inspector activity proof using authoritative events or a minimal rendering fix if required

## Acceptance

1. AC-1: A focused integration test proves a model-backed Shepherd initiates a turn to a second admitted model-backed agent
2. AC-2: The emitted event or work item identifies Beacon as sender and Ember as recipient using canonical agent identities
3. AC-3: The recipient generated result is correlated to the initiated work and cannot be confused with the Shepherd user-facing reply
4. AC-4: Duplicate or replayed initiation is idempotent or explicitly rejected under a documented rule
5. AC-5: Cancellation and recipient or provider failure produce truthful terminal state
6. AC-6: Observatory feed proof shows the agent-to-agent activity, with UI changes only if the existing Activity surface cannot render the authoritative event
7. AC-7: Exact-head independent review passes before publication

## Dependencies

- Issue #661 Shepherd configured-provider reply work must remain distinct from initiation semantics
- Current Runtime resident agent admission and configured-provider execution paths

## Inputs

- agent-logic/agent-design-language#662
- agent-logic/agent-design-language#661
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/ingress.rs
- adl-runtime-kernel/src/shepherd.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs

## Non Goals

- Do not conflate initiation with the separate Shepherd hardcoded-reply defect
- Do not implement unrestricted autonomous messaging
- Do not implement broadcast or arbitrary fan-out
- Do not implement recursive unbounded conversations
- Do not restart or modify the live Runtime as part of implementation
- Do not claim live provider acceptance without separate authorization
