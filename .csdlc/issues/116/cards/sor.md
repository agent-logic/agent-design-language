# Structured Output Record

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded WP-18C.06 operator attention inbox and intervention workflow with trusted source authorization, validated restart restoration, Runtime quiet-mode/grouping, and proposal-only Observatory outcomes.

## Artifacts

- adl-runtime-kernel/src/operator_attention.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/observatory.rs
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- demos/html-observatory/tests/operator_attention_inbox.test.mjs
- .csdlc/prepared/issues/116/validate_preparation_bundle.py
- .csdlc/prepared/issues/116/run_exact_local_proof.py
- .csdlc/evidence/116/issue-116-exact-local-proof.log

## Execution

- Replaced self-asserted operator-attention source privilege checks with trusted Runtime source registration and principal/urgent validation.
- Added fail-closed snapshot restore validation for schema, capacity, duplicates, source limits, request safety, event references, and timestamps.
- Added quiet-mode/grouping behavior while preserving source-bound rate and dedupe semantics.
- Added Observatory status/priority filters, deep links, read/unread view state, notification preference handling, and selected-row rendering.
- Removed stale self-referential committed HEAD-proof wording; exact revision authority remains the typed review assignment.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/116/run_exact_local_proof.py"
    ],
    "purpose": "Run focused #116 local proof with command argv, exit status, and test denominators for preparation validator, rustfmt, focused Rust tests, Node Observatory proof, strict clippy, and diff hygiene. Exact Git revision is bound separately by typed review assignment.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/116/issue-116-exact-local-proof.log"
  }
]

## Integration

worktree_only

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
