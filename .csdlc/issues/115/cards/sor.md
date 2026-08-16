# Structured Output Record

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Refined the #115 Runtime governed room-routing slice with an explicit mention contract for accepted room turns.

## Artifacts

- adl-runtime-kernel/src/conversation_rooms.rs
- adl-runtime-kernel/src/lib.rs
- .csdlc/evidence/115/runtime-proof-summary.json
- .git/csdlc-v2/quarantine/115-duplicate-room-draft-20260816T2322Z/MANIFEST.md
- adl-runtime-kernel/src/conversation_rooms.rs
- .csdlc/evidence/115/conversation-rooms-test.log
- .csdlc/evidence/115/cargo-fmt-check.log
- .csdlc/evidence/115/cargo-clippy-lib.log

## Execution

- Added exported adl-runtime-kernel conversation_rooms module for explicit governed room turns.
- Denied implicit broadcast, duplicate recipients, unknown recipients, ineligible recipients, cross-Polis recipients, unavailable left/revoked recipients, duplicate turns, and reordered turns.
- Mapped accepted room routes to Layer 8 AddressRecipients authority scope and preserved partial delivery/refusal/timeout/unavailable/revoked states without hiding recipient identity.
- Quarantined an unexported duplicate multi_agent_rooms.rs draft under Git-common quarantine before removing it from the publishable worktree.
- Added governed room mention schema and route payload entries derived from authorized room participants.
- Preserved stable recipient ordering and display names so Observatory can render recipient mentions without inferring or expanding recipients in the browser.
- Extended the focused runtime proof to assert exact mention identities for accepted room turns.

## Validation

[
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Format check for #115 Runtime governed room-routing slice",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/cargo-fmt-check.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "conversation_rooms",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused Runtime governed room-routing proof: explicit recipients, policy/membership denial, partial delivery, Layer 8 scope reuse, ordering, and replay",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/conversation-rooms-test.log; summary .csdlc/evidence/115/runtime-proof-summary.json; 6 passed, 0 failed"
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
    "purpose": "Strict Clippy for #115 Runtime governed room-routing library slice",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/cargo-clippy-lib.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Format check after adding #115 governed room mention contract",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/cargo-fmt-check.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "conversation_rooms",
      "--",
      "--nocapture"
    ],
    "purpose": "Focused Runtime governed room-routing proof after adding stable mention contract",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/conversation-rooms-test.log; summary .csdlc/evidence/115/runtime-proof-summary.json; 6 passed, 0 failed"
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
    "purpose": "Strict Clippy for #115 Runtime governed room-routing library after mention contract refinement",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/cargo-clippy-lib.log"
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
