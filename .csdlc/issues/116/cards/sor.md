# Structured Output Record

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sor

Status: complete

## Summary

Implemented the bounded WP-18C.06 operator attention inbox and intervention workflow with trusted source authorization, validated restart restoration, Runtime quiet-mode/grouping, proposal-only Observatory outcomes, invalid-outcome rollback safety, terminal duplicate immutability, and focused local proof whose exact revision authority is the typed review assignment.

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

- Validated operator attention outcome payloads and monotonic timestamps before mutating request state or appending events.
- Added focused Rust regression coverage proving invalid reply, refusal, defer, and non-monotonic outcome attempts leave request state and event count unchanged.
- Rejected stale active duplicate/grouped submissions before changing duplicate or grouping counters.
- Returned existing terminal duplicate receipts without mutating status, duplicate_count, updated_at_millis, or event count, including restored expired requests.
- Preserved trusted Runtime source registration, principal and urgent authorization checks, quiet-mode/grouping behavior, and proposal-only Observatory outcomes.
- Retained focused proof with 8 Rust operator-attention tests, 1 Node Observatory test, rustfmt, strict clippy, preparation validator, and diff hygiene PASS; exact Git revision is bound separately by typed review assignment.

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

merged

## Publication

Publication: closed

Merge: merged

## Closeout

complete

## Follow Ups

- none
