# Structured Output Record

Template: 1.0.0

Issue: 5905

Repository: danielbaustin/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved final implementation-review findings by including closed PRs in issue-side closing-link discovery, failing closed on pagination, applying unique-merged terminal precedence over abandoned closed-unmerged attempts, rejecting multiple merged candidates, and exposing strict repository/SHA patterns in the public schema.

## Artifacts

- .csdlc/prepared/issues/5905/design.md
- .csdlc/prepared/issues/5905/diagram.mmd
- subagent:review-5905-implementation findings resolved
- subagent:review-5905-implementation findings resolved

## Execution

- csdlc-v2/src/finish.rs
- csdlc-v2/src/bin/csdlc-finish.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate_finish.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate_finish.rs
- csdlc-v2/src/finish.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/lib.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/tests/gate_finish.rs

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove historical reconciliation success, disposition-conditional request rejection, exact identity and linkage rejection, distinct provenance, cached-terminal matching, idempotency foundations, and unchanged routine finish behavior.",
    "outcome": "passed",
    "evidence_ref": "15 focused gate_finish tests passed; cargo check for the csdlc-finish binary passed; git diff --check passed. The live #5800 canary remains intentionally post-merge."
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove stable historical idempotency, exact single-candidate closing PR attribution, strict typed request decoding and identity validation, and all prior finish behavior.",
    "outcome": "passed",
    "evidence_ref": "17 gate_finish tests passed; issue-closing PR GraphQL inventory parser test passed; strict csdlc-finish Clippy passed; git diff --check passed."
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate_finish"
    ],
    "purpose": "Prove unique-merged terminal precedence, abandoned-attempt tolerance, multiple-merged and pagination rejection, strict public schema patterns, and all prior finish behavior.",
    "outcome": "passed",
    "evidence_ref": "17 gate_finish tests passed; issue-closing PR GraphQL inventory parser test passed; strict csdlc-finish Clippy passed; git diff --check passed."
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
