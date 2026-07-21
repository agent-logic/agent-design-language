# #5337 Exact-Revision Review: 5da012921

Reviewer: `task:019f4b3e-6c61-7653-957b-7a2a6042a80d`

Reviewed revision: `5da012921d21da281530c854a14ea589c7fd6c69`

## Findings

### P1: Offline verification does not revalidate the corpus command contract

`compare.rs` checked raw identity, re-derivation, and repeat equality, while
expected step count/order, declared arguments, expected exits, and required
stdout/stderr fragments were enforced only during capture. A changed corpus or
retained evidence that never met those assertions could still verify.

Required disposition: recheck the complete command contract offline and add a
tamper test for every class.

### P1: The claimed denied-network execution boundary is not implemented

Clearing the child environment and setting `NO_PROXY=*` did not disable host
networking. A corpus-supplied network/provider invocation remained possible.

Required disposition: either enforce a platform-independent network sandbox or
narrow the claim and add an executable fail-closed policy proving that only the
approved local mock path can execute.

### P2: A hung incumbent hangs the harness indefinitely

`Command::output()` had no timeout or cancellation. A deadlocked or waiting
child could block capture indefinitely and leave partial canonical evidence.

Required disposition: add a declared timeout, kill/reap behavior, deterministic
timeout classification, atomic evidence replacement, and a hanging-child test
using a COTS timeout facility.

### P2: `unknown-run-reference` does not test an unknown run reference

The case exercised rejection of unsupported `run.agent_ref`, not reference
resolution.

Required disposition: replace it with a genuine supported reference boundary
or rename the behavior and coverage claim to the schema-shape rejection that
was observed.

### P2: Checked-in raw evidence contains machine-local private paths

Retained raw records exposed `/Users/daniel/...` and `/private/var/...` paths.

Required disposition: retain portable tokenized evidence and exact byte hashes,
and test that no committed observation contains host paths.

## Validation Observed By Reviewer

- `cargo test`: 13 passed
- strict Clippy: passed
- no lifecycle or source files changed by reviewer

## Initial Result

`changes_required`
