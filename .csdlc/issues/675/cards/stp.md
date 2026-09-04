# Structured Task Prompt

Template: 1.0.0

Issue: 675

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Issue completion is exactly the first-class live-style agent-to-agent initiation bridge and proof; broader autonomy, broadcast, recursive messaging, and live production mutation are out of scope.

## Deliverables

- First-class A2A action/bridge from model-backed Shepherd or resident-agent behavior into Runtime-governed initiation
- Observatory/UI rendering or control path that distinguishes operator chat from A2A initiation and terminal recipient response
- Focused deterministic tests proving live-style Beacon-to-Ember initiation
- Regression coverage preserving the #662 kernel primitive semantics

## Acceptance

1. AC-1: Beacon can request/contact Ember through a governed A2A action path instead of replying that it has no colleague when policy allows it
2. AC-2: The initiated work records distinct sender, recipient, work, conversation, turn, and correlation identity
3. AC-3: Recipient execution uses configured provider/model routing and the response returns to the initiating context without confusion with Beacon's own reply
4. AC-4: Missing recipient, unauthorized sender, unavailable authority/signing, stale recipient, cancellation, replay, and provider failure remain truthful terminal outcomes
5. AC-5: Observatory Activity/UI exposes accepted dispatch and terminal A2A result without inventing delivery truth
6. AC-6: Existing #662 direct primitive tests continue to pass

## Dependencies

- #662/#668 backend primitive exists on main
- No live Runtime/provider/AWS execution unless separately authorized

## Inputs

- agent-logic/agent-design-language#675
- agent-logic/agent-design-language#662
- agent-logic/agent-design-language#668
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- demos/html-observatory/app.js
- adl/src/csm_shepherd_agent.rs
- adl/src/csm_resident_agents.rs

## Non Goals

- Unrestricted autonomous messaging
- Broadcast or arbitrary fan-out
- Recursive unbounded conversations
- Live production Runtime mutation
- Paid provider or AWS proof without explicit authorization
- Prompt-only capability claims
