# Structured Output Record

Template: 1.0.0

Issue: 116

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded WP-18C.06 operator attention inbox and intervention workflow for Runtime kernel and HTML Observatory surfaces without granting authority outside Runtime policy.

## Artifacts

- adl-runtime-kernel/src/operator_attention.rs
- adl-runtime-kernel/src/lib.rs
- adl-runtime-kernel/tests/observatory.rs
- demos/html-observatory/app.js
- demos/html-observatory/index.html
- demos/html-observatory/styles.css
- demos/html-observatory/tests/operator_attention_inbox.test.mjs

## Execution

- Added Runtime kernel operator-attention request, dedupe, rate-limit, expiry, outcome, snapshot, and restart primitives.
- Added focused Rust coverage for source authorization, urgent escalation, dedupe/rate limiting, non-authorizing outcomes, expiry, and restart preservation.
- Added HTML Observatory operator attention inbox rendering, request normalization, proposal-only outcome payloads, and focused Node proof.
- Added visible Observatory inbox markup and styling while preserving #114/#115/#117 boundaries.

## Validation

[
  {
    "command": [
      "git",
      "diff",
      "--check"
    ],
    "purpose": "Verify no whitespace or conflict-marker hygiene failures remain in the exact worktree diff.",
    "outcome": "passed",
    "evidence_ref": "issue-116-diff-hygiene.log"
  },
  {
    "command": [
      "node",
      "--test",
      "demos/html-observatory/tests/operator_attention_inbox.test.mjs"
    ],
    "purpose": "Verify the operator attention inbox DOM contract, request normalization, and proposal-only outcome payload.",
    "outcome": "passed",
    "evidence_ref": "issue-116-observatory-node-proof.log"
  },
  {
    "command": [
      "python3",
      ".csdlc/prepared/issues/116/validate_preparation_bundle.py"
    ],
    "purpose": "Verify the issue-owned preparation bundle and dependency cache truth still match canonical #112/#114/#115 gates.",
    "outcome": "passed",
    "evidence_ref": "issue-116-preparation-validator.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory",
      "operator_attention"
    ],
    "purpose": "Run the named observatory integration-test target filtered to operator attention tests.",
    "outcome": "passed",
    "evidence_ref": "issue-116-rust-operator-attention-tests.log"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--check"
    ],
    "purpose": "Verify Runtime kernel Rust formatting for the #116 touched crate.",
    "outcome": "passed",
    "evidence_ref": "issue-116-rustfmt-check.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "adl-runtime-kernel/Cargo.toml",
      "--test",
      "observatory",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict clippy for the named observatory test target after focused behavior proof.",
    "outcome": "passed",
    "evidence_ref": "issue-116-strict-clippy.log"
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
