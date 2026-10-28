# Structured Task Prompt

Template: 1.0.0

Issue: 169

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only V3-08 within its exact owned paths and authority boundary.

## Deliverables

- Transaction store, recovery engine, intent schema, explicit pre-network intent commit and post-readback reconciliation protocols, interruption matrix, per-platform sync/replacement safety matrix and harness, filesystem capability policy, and concurrency fixtures. `store/transaction.rs` owns lock/CAS/stage/sync/replace commit mechanics. `store/recovery.rs` is a pure classifier plus recovery-plan builder over observed canonical state, staging files, and durable intents; it cannot write directly and executes any selected repair through the transaction API.

## Acceptance

1. Only atomic replacement of `state.json` commits authority.
2. State commits before projection replacement; post-commit projection failure is a specific repair-required result, never rollback or ambiguous authority.
3. Cards, evidence indexes, and audit views are repairable projections.
4. Stale generation/digest writers fail before commit.
5. Every injected interruption converges to the prior or new valid state.
6. A remote operation cannot begin before its typed intent and parent directory are durably synced; recovery resumes committed intents through exact readback.
7. An unresolved intent is authoritative only as a pending-operation journal: it blocks competing mutation, contains no lifecycle/card state, and is consumed only after exact readback commits its outcome into `state.json`.
8. Linux, macOS, and every mutation-enabled Windows filesystem have a named, documented, fault-tested commit primitive; unproven Windows mutation fails closed while compile and read-only support remain available.
9. An injected platform-capability fixture proves the Windows fail-closed path and stable `unsupported_platform_mutation` error on every CI host; native Windows CI separately proves any mutation-enabled primitive.
10. Locks protect transaction integrity without becoming lifecycle authority.

## Dependencies

- V3-06: issue #167
- V3-07: issue #168
- V3-D11: issue #163

## Inputs

- docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-08
- docs/milestones/v0.92.1/WP_ISSUE_WAVE_v0.92.1.yaml
- docs/milestones/v0.92.1/WP_EXECUTION_SPECIFICATIONS_v0.92.1.yaml

## Non Goals

- Distributed transactions, remote rollback, lock-as-ownership, multi-file atomicity claims, GitHub mutation, or cleanup of unrelated paths.
