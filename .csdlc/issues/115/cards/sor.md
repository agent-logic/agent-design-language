# Structured Output Record

Template: 1.0.0

Issue: 115

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Fixed the #115 governed-room served route so newly created Runtime rooms start at canonical turn sequence 1 and reject first-turn sequence gaps.

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
- demos/html-observatory/app.js
- adl/tools/validate_v092_governed_room_observatory.mjs
- .csdlc/evidence/115/governed-room-observatory-validation.json
- .csdlc/prepared/issues/110/graph.json
- .csdlc/locks/115.lock
- adl-runtime-kernel/src/conversation_rooms.rs
- adl-runtime-kernel/src/control.rs
- adl/tools/validate_v092_governed_room_observatory.mjs
- adl-runtime-kernel/src/control.rs
- .csdlc/prepared/issues/115/validate_governed_room_implementation.py

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
- Derived stable governed room identity from normalized explicit recipients for the served Observatory composer.
- Tracked next turn sequence per governed room id so switching rooms and returning to a prior room continues the correct Runtime sequence.
- Added focused validator coverage for switch-return behavior that would previously trigger a false reordered-turn refusal.
- Removed the tracked issue-110 coordination graph artifact from the #115 branch.
- Removed the tracked empty #115 lock file from the publishable #115 branch.
- Kept the governed-room Runtime and Observatory product implementation unchanged.
- Added an accepted governed-room delivery state for Runtime-accepted but not yet recipient-delivered turns.
- Mapped served Runtime room intent acknowledgements to accepted rather than delivered.
- Extended Runtime and Observatory proof so accepted route rows do not invent delivery evidence.
- Initialized newly created served governed rooms with next_turn_sequence 1 instead of trusting envelope.intent.turn_sequence.
- Added served-path regression coverage proving first turn_sequence 2 is refused as reordered_room_turn and does not prevent a subsequent canonical sequence 1 turn from being accepted.
- Preserved existing governed-room Runtime, Layer 8 dependency, Observatory UI, and #278 durability boundaries.

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
  },
  {
    "command": [
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml conversation_rooms -- --nocapture",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml governed_room -- --nocapture",
      "cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib -- -D warnings",
      "node adl/tools/validate_v092_governed_room_observatory.mjs",
      "bash adl/tools/test_html_observatory.sh"
    ],
    "purpose": "Reprove the #115 governed-room Runtime and Observatory surfaces after the per-room turn-sequence remediation.",
    "outcome": "passed",
    "evidence_ref": "local post-fix run: fmt passed; conversation_rooms 6 passed; governed_room 2 passed; strict clippy passed; governed-room Observatory validator passed with per_room_turn_sequence_preserved_across_room_switching; HTML Observatory smoke passed"
  },
  {
    "command": [
      "cargo fmt --manifest-path adl-runtime-kernel/Cargo.toml --check",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml conversation_rooms -- --nocapture",
      "cargo test --manifest-path adl-runtime-kernel/Cargo.toml governed_room -- --nocapture",
      "cargo clippy --manifest-path adl-runtime-kernel/Cargo.toml --lib -- -D warnings",
      "node adl/tools/validate_v092_governed_room_observatory.mjs",
      "bash adl/tools/test_html_observatory.sh",
      "git diff --check"
    ],
    "purpose": "Prove #115 governed-room accepted-vs-delivered remediation, prior per-room sequencing fix, Runtime routing, Observatory rendering, and diff hygiene before fresh exact review.",
    "outcome": "passed",
    "evidence_ref": "local run: fmt passed; conversation_rooms 6 passed; governed_room 2 passed; strict clippy passed; governed-room Observatory validator passed including accepted_route_rows_do_not_claim_delivery and per_room_turn_sequence_preserved_across_room_switching; HTML Observatory smoke passed; git diff --check passed"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/115/validate_governed_room_implementation.py"
    ],
    "purpose": "Prove #115 governed-room Runtime and Observatory implementation after first-turn sequence-gap remediation.",
    "outcome": "passed",
    "evidence_ref": "local run: fmt passed; conversation_rooms 6 passed; governed_room 3 passed including governed_room_ws_intent_rejects_non_initial_first_sequence; strict clippy passed; governed-room Observatory validator passed; HTML Observatory smoke passed; git diff --check passed"
  }
]

## Integration

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
