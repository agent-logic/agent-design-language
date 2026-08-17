# Structured Output Record

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded WP-18C.06 operator attention inbox and intervention workflow with trusted source authorization, validated restart restoration, proposal-only Observatory outcomes, and exact local proof provenance.

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
- Added an issue-owned exact local proof runner that prints HEAD, status, command argv, exit status, and denominators before PASS.
- Preserved bounded HTML Observatory inbox and proposal-only outcome payloads that explicitly do not grant authority.

## Validation

[
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/116/run_exact_local_proof.py"
    ],
    "purpose": "Run exact #116 local proof with HEAD/status/command/exit provenance for preparation validator, rustfmt, focused Rust tests, Node Observatory proof, strict clippy, and diff hygiene.",
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
