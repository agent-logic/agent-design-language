# Structured Output Record

Template: 1.0.0

Issue: 425

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Implemented recordless closeout classification and receipt retention support in csdlc-finish, with fail-closed handling for contradictory historical publication evidence.

## Artifacts

- cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_recordless_closeout: 4 passed
- cargo check --manifest-path csdlc-v2/Cargo.toml: passed
- git diff --check: passed
- recordless dry-run: 8 recordless_terminal_eligible, 1 conflicting_historical_publication (#248)

## Execution

- csdlc-v2/src/bin/csdlc-finish.rs: add recordless-closeout subcommand while preserving existing finish flags
- csdlc-v2/src/finish.rs: add recordless closeout request/result/receipt types, live classification, no-projection checks, historical-publication conflict detection, and recordless receipt retention
- csdlc-v2/src/lib.rs: export recordless closeout request/result types
- csdlc-v2/tests/gate_recordless_closeout.rs: add focused fail-closed classifier tests for eligible, source-projection, conflicting publication, and PR identity mismatch paths
- .csdlc/evidence/425-v092-residual-dry-run-result.json: retained live classify-only evidence over the nine v0.92 residuals

## Validation

[]

## Integration

not_started

## Publication

Publication: not_published

Merge: not_merged

## Closeout

not_started

## Follow Ups

- none
