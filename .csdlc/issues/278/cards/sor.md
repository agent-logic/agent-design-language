# Structured Output Record

Template: 1.0.0

Issue: 278

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Remediated #278 review P1 by making re-authorized conversation history reads honor Runtime journal deletion markers. The shared history visibility helper now returns no records for deleted conversations, and the focused Runtime regression proves page, search, export, and Observatory restore all hide deleted transcript history. Publication and merge are not yet attempted.

## Artifacts

- adl-runtime-kernel/src/conversation_history.rs
- adl-runtime-kernel/tests/conversation_history.rs
- .csdlc/evidence/278/issue-278-preparation-validator.log sha256=a74a551afb309432adce3831f480d3ae4c88b657f7e470c9e4678dacb37e1ee5
- .csdlc/evidence/278/runtime-kernel-conversation-history-fmt.log sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855
- .csdlc/evidence/278/runtime-kernel-conversation-history-focused.log sha256=cfdf9229396a1991e68e6f105772ad5eddb3fe69a741562bb467c27e5f94ffb9
- .csdlc/evidence/278/observatory-transcript-restore-validator.log sha256=a9906d8870f32db6ff606f4c244bc072a5688b4112d4b73214b53ebf38d61f55
- .csdlc/evidence/278/runtime-kernel-strict-clippy.log sha256=22e0df986057b9ba5be07526ac5fb348862a3dedef42108383cc3e088acbb889
- .csdlc/evidence/278/diff-check.log sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

## Execution

- adl-runtime-kernel/src/conversation_history.rs: visible_records now returns an empty history projection when the journal snapshot marks the conversation deleted
- adl-runtime-kernel/tests/conversation_history.rs: added deletion_marker_hides_page_search_export_and_restore_history covering page/search/export/restore
- .csdlc/evidence/278: refreshed preparation, fmt, focused Runtime, Observatory validator, strict clippy, and diff-hygiene logs after the remediation

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/278/validate_preparation_bundle.py"
    ],
    "purpose": "Validate #278 dependency/scope packet and terminal caches after review recovery.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/issue-278-preparation-validator.log sha256=a74a551afb309432adce3831f480d3ae4c88b657f7e470c9e4678dacb37e1ee5"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Rust formatting check after deletion-visibility remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/runtime-kernel-conversation-history-fmt.log sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
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
    "purpose": "Focused #278 Runtime proof including deletion marker hiding across page/search/export/restore; 5 tests passed.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/runtime-kernel-conversation-history-focused.log sha256=cfdf9229396a1991e68e6f105772ad5eddb3fe69a741562bb467c27e5f94ffb9"
  },
  {
    "command": [
      "node",
      "adl/tools/validate_v092_observatory_transcript_history.mjs"
    ],
    "purpose": "Validate Observatory transcript restoration helper behavior after Runtime history remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/observatory-transcript-restore-validator.log sha256=a9906d8870f32db6ff606f4c244bc072a5688b4112d4b73214b53ebf38d61f55"
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
    "purpose": "Strict Runtime kernel clippy after deletion-visibility remediation.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/runtime-kernel-strict-clippy.log sha256=22e0df986057b9ba5be07526ac5fb348862a3dedef42108383cc3e088acbb889"
  },
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Whitespace and patch hygiene after refreshed evidence logs.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/278/diff-check.log sha256=e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
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
