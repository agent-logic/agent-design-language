# Structured Output Record

Template: 1.0.0

Issue: 275

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented the bounded #275 integrated serving-authority snapshot store from terminal borrowed #367 child-lineage pairs. The store records deterministic redacted receipts, preserves immutable operation prefixes, validates persisted receipt/result/checkpoint chains on reopen, retries exact operations idempotently, rejects conflicting retries, and fails closed on capacity, corruption, rollback, A/B substitution, and checkpoint tamper.

## Artifacts

- adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs
- adl-runtime/tests/distributed_integrated_serving_authority.rs
- adl-runtime/src/distributed/mod.rs
- .csdlc/prepared/issues/275/run_exact_focused_matrix.py
- .csdlc/prepared/issues/275/validate_exact_scope.py

## Execution

- Added `adl-runtime/src/distributed/integrated_serving_authority_snapshot.rs` with a public store API accepting only borrowed `VerifiedCommittedChildLineagePair` inputs from #367.
- Registered the new module with one additive `adl-runtime/src/distributed/mod.rs` line.
- Added `adl-runtime/tests/distributed_integrated_serving_authority.rs` with the exact eight-case focused matrix for authentic pairs, immutable prefixes, all outcomes, retry/reopen, CAS rollback, corruption, terminal child evidence, A/B denial, redaction, and tamper denial.
- Repaired draft result-digest validation so historical receipts validate against their immutable operation prefix rather than the future final state.

## Validation

[
  {
    "command": [
      "#275",
      "focused-validation-bundle"
    ],
    "purpose": "Prove exact focused matrix, private unit, compile-fail API denial, exact scope, diff hygiene, and strict Clippy for the integrated serving-authority snapshot slice.",
    "outcome": "passed",
    "evidence_ref": "Local commands passed: python3 .csdlc/prepared/issues/275/run_exact_focused_matrix.py (8/8 exact tests); cargo test --lib distributed::integrated_serving_authority_snapshot::tests::normalized_receipt_rejects_tamper --exact (1/1); cargo test --doc integrated_serving_authority_snapshot (3/3 compile-fail doctests); python3 .csdlc/prepared/issues/275/validate_exact_scope.py; git diff --check c46b7cd8265a7e81566cdf82153c387595a6cccf...HEAD; cargo clippy --lib --features internal-test-fixtures -D warnings; cargo clippy --test distributed_integrated_serving_authority --features internal-test-fixtures -D warnings."
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
