# Structured Output Record

Template: 1.0.0

Issue: 278

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented #278 re-authorized conversation history APIs and Observatory transcript restoration helpers over Runtime-owned durable journal state.

## Artifacts

- adl-runtime-kernel/src/conversation_history.rs
- adl-runtime-kernel/tests/conversation_history.rs
- adl/tools/validate_v092_observatory_transcript_history.mjs
- demos/html-observatory/app.js
- .csdlc/evidence/278

## Execution

- Added adl-runtime-kernel conversation_history module for authorized page/search/export/redact/restore over journal events.
- Added focused Runtime conversation_history integration tests for stale cursors, revoked access, private-memory denial, export authority, redaction, search, and restart restore.
- Added Observatory Runtime-history normalization/restoration/redaction helpers and validator for stale incarnation rejection and unsafe-field redaction.
- Updated #278 VPP to implementation-phase proof lanes with exact available commands.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/278/validate_preparation_bundle.py"
    ],
    "purpose": "Validate #278 dependency/scope packet and terminal caches.",
    "outcome": "passed",
    "evidence_ref": "issue-278-preparation-validator.log"
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_observatory_transcript_history.mjs"
    ],
    "purpose": "Run #278 Observatory transcript history validator.",
    "outcome": "passed",
    "evidence_ref": "observatory-transcript-restore-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "conversation_history"
    ],
    "purpose": "Run #278 Runtime conversation history tests.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-conversation-history-focused.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--lib",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict clippy for touched Runtime kernel library.",
    "outcome": "passed",
    "evidence_ref": "runtime-kernel-strict-clippy.log"
  }
]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
