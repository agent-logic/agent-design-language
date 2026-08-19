# V3-08 Design

Issue: #169

## Objective

Make state mutation crash-consistent with one explicit commit point and recoverable projections.

## Scope

Advisory locking, compare-and-swap generation/digest checks, intent records, temporary writes, fsync policy, atomic `state.json` replacement, projection regeneration, recovery classification, fault injection, and concurrent writer behavior.

## Dependencies

- V3-06: issue #167
- V3-07: issue #168
- V3-D11: issue #163

## Architecture Decisions

- `V3-D05`
- `V3-D11`

## Deliverables

- Transaction store, recovery engine, intent schema, explicit pre-network intent commit and post-readback reconciliation protocols, interruption matrix, per-platform sync/replacement safety matrix and harness, filesystem capability policy, and concurrency fixtures. `store/transaction.rs` owns lock/CAS/stage/sync/replace commit mechanics. `store/recovery.rs` is a pure classifier plus recovery-plan builder over observed canonical state, staging files, and durable intents; it cannot write directly and executes any selected repair through the transaction API.

## Owned Paths

- `csdlc-v3/src/store/**`
- `csdlc-v3/src/recovery/**`
- `csdlc-v3/tests/store/**`
- `.csdlc/issues/169/**`
- `.csdlc/prepared/issues/169/**`
- `.csdlc/prepared/issues/169/validate-outcome.rb`
- `.csdlc/evidence/169/**`

Every repository path outside this exact list is read-only unless a reviewed
design revision updates the issue before execution.

## Acceptance Criteria

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

## PVF Lanes

- `v3-08-outcome-contract`: Recompute every acceptance criterion from issue-owned artifacts and producer-derived evidence. Command: `ruby .csdlc/prepared/issues/169/validate-outcome.rb`.
- `v3-08-focused-rust`: Run the focused C-SDLC v3 implementation tests owned by this work package. Command: `cargo test --locked --manifest-path csdlc-v3/Cargo.toml --all-targets`.
- `v3-08-diff-hygiene`: Reject whitespace and malformed-diff defects before exact-head review. Command: `git diff --check origin/main...HEAD`.

## Validation Proof

Fault injection at every write/sync/rename boundary, parallel writer stress, repeated recovery idempotency, disk-full/read-only fixtures, and supported-filesystem tests.

## Authority Boundary

- Issue V3-08 owns only its declared repository paths and named external operation/evidence boundary.
- Dependencies remain read-only inputs until terminal evidence satisfies the declared gate.
- The issue may not absorb remediation owned by another work package without an explicit issue-graph revision.

## Non-goals

- Distributed transactions, remote rollback, lock-as-ownership, multi-file atomicity claims, GitHub mutation, or cleanup of unrelated paths.

## Risks

- A passing artifact could overstate production, legal, or release authority.
- Path or external-account overlap could collide with another active issue.
- Evidence could become stale if it is not tied to exact revisions and producer outcomes.

## Stop Conditions

- Recovery requires guessing, a partial projection becomes authority, remote mutation enters a local transaction, or platform semantics cannot satisfy the declared commit guarantee.

## Review Prompts

- Does the implementation satisfy every acceptance criterion through producer-derived evidence?
- Are all changed repository paths within the declared owned paths?
- Are dependency, authority, non-goal, stop, and residual-risk boundaries truthful?
- Can an independent reviewer reproduce the claimed result at the exact revision?

## Source Authority

- `docs/milestones/v0.92.1/sources/CSDLC_V3_GH_INSPIRED_RUST_ARCHITECTURE_SOURCE.md#v3-08`
