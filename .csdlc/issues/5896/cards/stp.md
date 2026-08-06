# Structured Task Prompt

Template: 1.0.0

Issue: 5896

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Implement and prove one deterministic migration for records rendered bound without topology before the #5886 cutover.

## Deliverables

- Typed migration request and result contracts
- Explicit csdlc-issue migration command
- Atomic idempotent record conversion
- Focused migration tests and cohort disposition evidence

## Acceptance

1. The exact affected cohort is inventoried before mutation
2. Every record is classified using canonical state, live issue state, and actual Git topology
3. Open never-bound records become initialized and normally bindable
4. Verified existing topology and terminal records are preserved truthfully
5. Cards, authored artifacts, audit history, identity, and digest integrity are preserved
6. Ambiguous topology and digest mismatch fail before mutation
7. The operation is idempotent and emits a per-issue disposition report
8. Doctor no longer rejects migrated records solely for bound null topology
9. Issue 5844 becomes bindable through current csdlc-bind
10. Focused tests cover all required positive and negative classifications

## Dependencies

- PR #5886 topology-only binding authority
- PR #5880 pre-cutover readiness records

## Inputs

- csdlc-v2/src/store.rs
- csdlc-v2/src/lifecycle.rs
- csdlc-v2/src/migration.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- .csdlc/issues/*/index.json

## Non Goals

- Restoring removed claim or preparation authority
- Redesigning the lifecycle state machine
- Binding or implementing migrated product issues
- Reopening terminal issues
- Hand-editing individual records
