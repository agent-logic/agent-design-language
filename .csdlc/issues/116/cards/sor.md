# Structured Output Record

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded WP-18C.06 operator attention inbox and intervention workflow with trusted source authorization, validated restart restoration, proposal-only Observatory outcomes, invalid-outcome rollback safety, and exact-head local proof enforcement.

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
- Preserved trusted Runtime source registration, principal and urgent authorization checks, quiet-mode/grouping behavior, and proposal-only Observatory outcomes.
- Restored the issue-owned exact local proof runner so it asserts ISSUE_116_EXPECTED_HEAD and clean worktree before reporting PASS.
- Regenerated exact local proof at 7c5aefc44eb9beb5c94c76821cd47ddf33c93535 with 8 focused Rust tests, 1 Node test, rustfmt, strict clippy, preparation validator, and diff hygiene PASS.

## Validation

[
  {
    "command": [
      "ISSUE_116_EXPECTED_HEAD=7c5aefc44eb9beb5c94c76821cd47ddf33c93535",
      "python3",
      ".csdlc/prepared/issues/116/run_exact_local_proof.py"
    ],
    "purpose": "Run exact #116 local proof with expected HEAD assertion, clean-tree assertion, preparation validator, rustfmt, focused Rust tests, Node Observatory proof, strict clippy, and diff hygiene.",
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
