# Structured Output Record

Template: 1.0.0

Issue: 713

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented durable verbatim causal agent-to-agent conversation history for Runtime and Observatory. The change adds a first-class JCS-hashed A2A exchange receipt model, extends agent checkpoint/dehydrate/rehydrate records so outbound messages and A2A initiation metadata survive restart, projects parent and peer A2A transcript records through the authenticated Observatory WebSocket history API, and teaches the Observatory browser normalizer to preserve safe causal metadata while redacting private/provider markers. Live Wuji proof remains deferred unless explicitly operator-authorized.

## Artifacts

- adl-runtime-kernel/src/conversation_history.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/tests/conversation_history.rs
- demos/html-observatory/app.js
- demos/html-observatory/tests/conversation_sessions.test.mjs
- demos/html-observatory/tests/security_privacy_adversarial.test.mjs
- adl/tools/test_issue713_a2a_history.sh
- .csdlc/prepared/issues/713/validate-live-a2a-history.sh

## Execution

- Added ConversationHistoryA2aExchange and append_a2a_exchange to the durable conversation history store, with replay deduplication and public-safe redaction projection for outbound and reply records.
- Extended AgentTurnCheckpoint with outbound message, speaker, timestamps, initiated conversation/turn/correlation/work identifiers, and initiated reply so checkpoint, dehydrate, and rehydrate preserve complete A2A transcript context.
- Updated Runtime Observatory history projection to emit parent operator/agent records plus A2A outbound and A2A reply records in causal order with history_kind, a2a_role, causal_id, sender_id, recipient_id, work_id, and parent linkage metadata.
- Updated Observatory browser normalization and recovery tests so verbatim A2A bodies restore with correct attribution and unsafe ID/body material is redacted.
- Replaced the issue #713 deterministic validator placeholder with real Rust and Node proof commands.

## Validation

[
  {
    "command": [
      "bash",
      "adl/tools/test_issue713_a2a_history.sh"
    ],
    "purpose": "Run the issue-owned #713 deterministic A2A history validator.",
    "outcome": "passed",
    "evidence_ref": "a2a-history-focused.log"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Run Git diff whitespace hygiene after #713 implementation and lifecycle updates.",
    "outcome": "passed",
    "evidence_ref": "diff-hygiene.log"
  }
]

## Integration

pr_open

## Publication

Publication: ready

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
