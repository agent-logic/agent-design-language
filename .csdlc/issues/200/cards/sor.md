# Structured Output Record

Template: 1.0.0

Issue: 200

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Resolved all four findings from the fresh review of 20fd144ba1633c21c69a72fbd3a6b1ebb9ba84ee. The fixed 36-case denominator now injects restart faults after marker and after view, rejects conflicting and corrupt published views on exact retry, denies retained mutation permits after N+1 Pending plus wrong lineage/action use, and emits an independently enumerated 13-subassertion marker set. The immutable v3 receipt at source 22f481bf85f8a6775eef4ed9450204ad77dffe9b binds 36/36 cases, 13/13 subassertions, strict Clippy, and protected digests. Full runtime remains 264/264. Different fresh review is pending; nothing is published, merged, or closed.

## Artifacts

- adl-runtime/src/distributed/authority_reconciliation/tests.rs
- .csdlc/prepared/issues/200/produce-proof-receipt.rb
- .csdlc/prepared/issues/200/validate-proof-receipt.rb
- .csdlc/evidence/200/v3/execution-proof.json

## Execution

- Expanded crash_after_checkpoint within the approved denominator to inject and recover after checkpoint, after marker, and after published view.
- Expanded exact_retry_cached_result to prove no adapter re-execution and fail-closed conflicting and corrupt published views.
- Expanded published_permit_current to prove current read/mutation validity, read escalation denial, wrong-lineage and wrong-mutation-action denial, and retained read plus mutation denial after generation N+1 becomes Pending.
- Upgraded the proof producer and validator to v2 schema/v3 evidence with an exact 13-item required subassertion set and per-marker digests in addition to the unchanged 36-case denominator.

## Validation

[
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "--no-tests=fail",
      "-E",
      "test(/authority_reconciliation/)"
    ],
    "purpose": "Prove the unchanged exact 36-case denominator with all review-resolution assertions.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v3/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "test",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "authority_reconciliation",
      "--",
      "--nocapture",
      "--test-threads=1"
    ],
    "purpose": "Emit and bind exactly 36 case markers plus the exact 13 required subassertion markers.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v3/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--test",
      "distributed_authority_reconciliation",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Prove the owned production and test target remains warning-free.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v3/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "nextest",
      "run",
      "--locked",
      "--manifest-path",
      "adl-runtime/Cargo.toml",
      "--lib",
      "--no-tests=fail"
    ],
    "purpose": "Prove the full runtime library: 264 tests run and 264 passed.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/distributed/authority_reconciliation/tests.rs"
  },
  {
    "command": [
      "ruby",
      ".csdlc/prepared/issues/200/validate-proof-receipt.rb"
    ],
    "purpose": "Prove exact cases, exact subassertions, source/proof immutability, protected digests, and merge-safe ancestry.",
    "outcome": "passed",
    "evidence_ref": ".csdlc/evidence/200/v3/execution-proof.json"
  },
  {
    "command": [
      "cargo",
      "fmt",
      "--check",
      "--manifest-path",
      "adl-runtime/Cargo.toml"
    ],
    "purpose": "Prove Rust formatting hygiene.",
    "outcome": "passed",
    "evidence_ref": "adl-runtime/src/distributed/authority_reconciliation/tests.rs"
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
