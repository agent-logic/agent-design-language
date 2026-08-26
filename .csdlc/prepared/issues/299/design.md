# Issue #299 implementation map: exact-authority archived-projection cleanup

## Source authority

This planning packet is derived only from:

- Live issue #299: `[v0.92][C-SDLC][297.b][defect] Implement exact-authority archived-projection cleanup`
- Completed #298 implementation/PR metadata visible on PR #305:
  - PR: `https://github.com/agent-logic/agent-design-language/pull/305`
  - head: `3e4fae5ae38eee3f74c338b06370c6157a71e37a`
  - substantive implementation noted by the PR body: `be1cddf61`
  - metadata/evidence through: `65108de82`
  - PR body claims governed focused recovery lane `37 passed, 19 filtered`, csdlc-v2 library suite `78 passed`, strict all-target Clippy passed, and independent exact-revision review R25 PASS.

Issue #299 remains blocked until #298 is terminal and ancestral. This packet is therefore a ready-to-execute implementation map, not execution authority.

## Required outcome

Implement the destructive cleanup half that consumes only completed #298 recovery authority:

1. Verify #298 is terminal and ancestral to the #299 execution base.
2. Verify a completed exact recovery receipt and canonical/archive binding from #298.
3. Capture the public archived projection and every recorded node by exact identity/type exchange.
4. Remove only exact captured inodes through durable, type-correct unlink/rmdir.
5. Dispose public/private placeholders only through operation-owned type-matched counterparts.
6. Retain immutable cleanup receipts outside the private delete namespace.
7. Resume deterministically and idempotently across every intent, exchange, removal, fsync, receipt, and partial-tree boundary.

## Implementation map

### M1. Gate authority and receipt ingestion

- Add a cleanup request type that names:
  - issue number;
  - expected #299 generation/digest;
  - actor;
  - distinct cleanup operation id;
  - expected terminal #298 PR/head/merge evidence;
  - expected completed #298 recovery receipt digest;
  - expected canonical/archive manifest digest;
  - expected registered branch/worktree topology.
- Fail before mutation unless:
  - #298 is terminal and ancestral;
  - #298 completed recovery receipt agrees with verified canonical state;
  - the archived manifest is exact, complete, and tied to the recovered canonical projection;
  - #299 holds the issue lock;
  - no cleanup receipt already records a conflicting operation id.

### M2. Immutable cleanup ledger

- Store cleanup receipts outside the private delete namespace.
- Receipts must be hash-linked and append-only.
- Receipts record pre-intent and post-completion for:
  - namespace creation;
  - public archive capture;
  - each leaf/directory exchange;
  - capture receipt durability before removal intent;
  - each unlink/rmdir;
  - each file fsync;
  - each parent fsync;
  - placeholder disposal;
  - placeholder-disposal durability before final cleanup receipt;
  - final completion.
- Repeating an already complete operation returns the same completed result without deleting anything new.

### M3. Private deletion namespace and placeholders

- Create a private deletion namespace with exact owner uid/gid, private modes, mount identity, parent identity, required-absent names, and no symlink/special-node ancestors.
- For every captured public node, create an operation-owned private placeholder of the exact required type.
- Root placeholder, per-node tombstones, and public/private disposal counterparts must be type-matched.
- Any collision, third state, unsupported type, non-empty directory, owner/mode drift, mount drift, or parent replacement preserves all state and fails closed.

### M4. Exact archive capture

- Walk only the archived per-node manifest emitted by #298.
- For each manifest entry:
  - open descriptor/handle-bound;
  - revalidate device, mount, inode, ctime, link count, uid, gid, mode, type, size, digest where applicable, and parent identity;
  - atomically exchange public node with operation-owned placeholder;
  - record capture receipt before any removal.
- Do not perform recursive deletion, path-authoritative unlink, digest-only ownership, symlink following, or best-effort cleanup.

### M5. Type-correct removal and fsync

- Remove captured regular files with unlink only after exact capture receipt.
- Remove captured directories with rmdir only when receipt proves they are empty and exact.
- Fsync removed-node parents after every removal.
- Record durable completion for each node before advancing.
- Preserve unrelated sentinels and replacement inodes in every failure path.

### M6. Restart and adoption

- On restart:
  - load immutable cleanup ledger first;
  - adopt only exact receipt-owned identities and parent manifests;
  - resume at the first incomplete recorded node;
  - fail closed on any ambiguous, replaced, missing-unexpected, drifted, or unsupported state.
- Partial-tree cleanup must never infer ownership from path alone.

### M7. Integration and publication

- After implementation:
  - run focused cleanup tests;
  - run strict csdlc-v2 validation in proportion to touched surfaces;
  - assign an exact-head #119 review;
  - fix actionable findings;
  - publish only after #298 remains terminal and ancestral and #299 review passes.

## Test matrix

| ID | Acceptance | Test intent | Expected result |
| --- | --- | --- | --- |
| T1 | AC-1 | Cleanup request without terminal #298 receipt | fails before mutation |
| T2 | AC-1 | #298 terminal receipt exists but merge SHA not ancestral to execution base | fails before mutation |
| T3 | AC-1 | Completed #298 receipt and canonical/archive binding agree exactly | cleanup can start |
| T4 | AC-2 | Attempt path-authoritative deletion without capture receipt | impossible/fails |
| T5 | AC-2 | Symlink in archive tree or ancestor | fails closed, preserves state |
| T6 | AC-2 | Digest matches but inode/device/ctime differs | fails closed, replacement survives |
| T7 | AC-3 | Regular file capture, unlink, parent fsync | receipt sequence complete |
| T8 | AC-3 | Empty directory capture, rmdir, parent fsync | receipt sequence complete |
| T9 | AC-3 | Non-empty directory in cleanup target | fails closed, preserves directory |
| T10 | AC-3 | Root placeholder type mismatch | fails closed before removal |
| T11 | AC-4 | Public node replaced between capture precheck and exchange | fails closed, unrelated replacement survives |
| T12 | AC-4 | Private namespace owner/mode drift | fails closed |
| T13 | AC-4 | Mount or parent identity drift | fails closed |
| T14 | AC-4 | Unsupported node type in manifest | fails closed |
| T15 | AC-4/5 | Crash after terminal gate and recovery receipt load, before cleanup namespace creation | restart revalidates #298 terminal ancestry and mutates nothing |
| T16 | AC-4/5 | Crash after cleanup namespace creation, before any capture intent | restart adopts only the exact operation namespace or fails closed without removing unrelated node |
| T17 | AC-4/5 | Crash after capture intent before exchange | restart resumes safely without deletion |
| T18 | AC-4/5 | Crash after exchange before capture receipt | restart refuses ambiguous post-exchange state unless exact operation-owned identity and parent manifest match |
| T19 | AC-4/5 | Crash after capture receipt before removal intent | restart resumes removal only from the receipt-owned private inode |
| T20 | AC-5 | Crash after unlink/rmdir before parent fsync receipt | restart completes or fails closed from exact receipts |
| T21 | AC-5/6 | Crash after parent fsync before completion receipt | restart records completion only when all durable receipt prerequisites match |
| T22 | AC-5/6 | Crash during placeholder disposal | restart disposes only operation-owned type-matched placeholders and preserves third-party replacements |
| T23 | AC-5/6 | Crash after placeholder disposal before final cleanup receipt | restart records final immutable receipt without re-deleting |
| T24 | AC-5 | Repeat complete cleanup | idempotent completed result |
| T25 | AC-6 | Cleanup success | recovery evidence and cleanup ledger survive |
| T26 | AC-7 | Sentinel adjacent to archived tree | sentinel survives success and every failure |
| T27 | AC-7 | Replacement inode at former public path | replacement survives |
| T28 | AC-8 | Exact-head review finding unresolved | publication blocked |

## Owned file map

Primary owned surfaces should be established after #298 is terminal and ancestral. Candidate #299-owned paths:

- `csdlc-v2/src/projection_cleanup.rs` or equivalent new cleanup module.
- `csdlc-v2/tests/archived_projection_cleanup.rs` or equivalent focused cleanup integration test.
- `csdlc-v2/src/schema.rs` only for public request/result schema registration.
- `csdlc-v2/src/bin/csdlc-edit.rs` or the appropriate owner binary only for a narrow command route.

Collision-held surfaces until explicit release:

- Do not touch issue/worktree #298.
- Do not touch `projection_recovery.rs`.
- Do not touch `store.rs`.
- Do not touch `gate5.rs`.

If #299 ultimately must integrate with any held surface, stop at the checkpoint and coordinate after explicit release.

## Collision-safe checkpoints

1. Before bootstrap: verify main clean, #298 terminal+ancestral, and no freeze on #299-owned paths.
2. Before bind: run typed doctor for #299 and confirm design approval is current.
3. Before first code edit: inspect current post-#298 file topology and confirm owned files do not collide with #298 remediation.
4. Before any command touching cleanup/recovery shared surfaces: explicitly re-confirm `projection_recovery.rs`, `store.rs`, and `gate5.rs` release status.
5. Before review assignment: ensure #299 includes #298 terminal+ancestral evidence and focused cleanup proof.
6. Before publication: re-run typed PR/readiness checks and keep #296/#297 graph truth non-overstated.

## Non-goals retained

- No #298 classification or canonical recovery implementation.
- No general filesystem cleaner.
- No #291, #294, or #296 mutation.
- No publication, merge, or closeout in the preparation lane.
