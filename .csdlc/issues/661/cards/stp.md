# Structured Task Prompt

Template: 1.0.0

Issue: 661

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Replace the Shepherd synthetic acknowledgement with governed configured-provider execution and focused success/failure proof.

## Deliverables

- Provider-backed Shepherd reply path
- Generated-output success test
- Provider-failure no-fallback test
- Preserved reply schema and correlation

## Acceptance

1. AC-1: Fresh Shepherd turn invokes its configured provider exactly once
2. AC-2: Non-empty generated content is returned
3. AC-3: Recipient, schema, conversation, and work correlation remain correct
4. AC-4: Provider failure is explicit and cannot return the acknowledgement
5. AC-5: Existing non-Shepherd provider-backed behavior remains valid
6. AC-6: Focused tests, hygiene, and exact-head review pass

## Dependencies

- Merged issue #640

## Inputs

- agent-logic/agent-design-language#661
- adl-runtime-kernel/src/assembly.rs
- adl-runtime-kernel/src/conversation_sessions_tests.rs
- agent-logic/agent-design-language#640

## Non Goals

- Agent-to-agent initiation
- Observatory rendering or attribution
- Provider configuration redesign
- Live Runtime mutation
