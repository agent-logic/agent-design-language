# Structured Output Record

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Hardened the #115 governed room UI against accidental broad room sends by enforcing an explicit 1-8 recipient bound and removing a fragile CSS.escape selector dependency.

## Artifacts

- adl-runtime-kernel/src/conversation_rooms.rs
- adl-runtime-kernel/src/lib.rs
- .csdlc/evidence/115/runtime-proof-summary.json
- .git/csdlc-v2/quarantine/115-duplicate-room-draft-20260816T2322Z/MANIFEST.md
- adl-runtime-kernel/src/conversation_rooms.rs
- .csdlc/evidence/115/conversation-rooms-test.log
- .csdlc/evidence/115/cargo-fmt-check.log
- .csdlc/evidence/115/cargo-clippy-lib.log
- demos/html-observatory/index.html
- demos/html-observatory/app.js
- demos/html-observatory/styles.css
- adl/tools/validate_v092_governed_room_observatory.mjs
- .csdlc/evidence/115/governed-room-observatory-validation.json
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- adl/tools/validate_v092_governed_room_observatory.mjs
- .csdlc/evidence/115/governed-room-observatory-validation.json

## Execution

- Added exported adl-runtime-kernel conversation_rooms module for explicit governed room turns.
- Denied implicit broadcast, duplicate recipients, unknown recipients, ineligible recipients, cross-Polis recipients, unavailable left/revoked recipients, duplicate turns, and reordered turns.
- Mapped accepted room routes to Layer 8 AddressRecipients authority scope and preserved partial delivery/refusal/timeout/unavailable/revoked states without hiding recipient identity.
- Quarantined an unexported duplicate multi_agent_rooms.rs draft under Git-common quarantine before removing it from the publishable worktree.
- Added governed room mention schema and route payload entries derived from authorized room participants.
- Preserved stable recipient ordering and display names so Observatory can render recipient mentions without inferring or expanding recipients in the browser.
- Extended the focused runtime proof to assert exact mention identities for accepted room turns.
- Added a Multi-agent room panel to the HTML Observatory communication surface with explicit multi-select participants, participant chips, transcript, composer, and send control.
- Added exported governed-room JavaScript helpers for participant normalization, explicit recipient validation, room-turn intent creation, route normalization, and per-recipient delivery rows.
- Wired Runtime v3 roster updates into the room participant selector without allowing hidden browser-side recipient expansion.
- Added a focused non-credential validator for explicit recipient denial, stable recipient ordering, attributed partial delivery rows, and static DOM anchors.
- Rejected room turns with more than eight explicit recipients before send.
- Updated the UI help text to disclose the bounded 1-8 recipient contract.
- Rendered prepared room delivery labels from selected options without relying on CSS.escape availability.

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
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_governed_room_observatory.mjs"
    ],
    "purpose": "Focused #115 Observatory room proof: explicit recipients, no implicit broadcast, stable room intent, attributed partial delivery rows, and static DOM anchors",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/governed-room-observatory-validation.json"
  },
  {
    "command": [
      "bash",
      "adl/tools/test_html_observatory.sh"
    ],
    "purpose": "Existing HTML Observatory Runtime v3, signed command, and roster projection contract smoke after adding governed room UI",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/html-observatory-smoke.log"
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_governed_room_observatory.mjs"
    ],
    "purpose": "Focused #115 Observatory room proof after enforcing 1-8 explicit recipient bound",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/115/governed-room-observatory-validation.json"
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
