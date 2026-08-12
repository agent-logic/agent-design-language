# Structured Output Record

Template: 1.0.0

Issue: 298

Repository: agent-logic/agent-design-language

Card: sor

Status: pre_phase

## Summary

Derive completed recovery candidates from the authorized PREPARED prior projection and audit transform

## Artifacts

- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- csdlc-v2/src/projection_recovery.rs
- csdlc-v2/src/store.rs
- csdlc-v2/src/schema.rs
- csdlc-v2/src/bin/csdlc-issue.rs
- csdlc-v2/tests/gate5.rs
- .csdlc/evidence/298/preserved-projection-recovery.log

## Execution

- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Typed tagged-CAS classification and failed-operation lineage
- Retained descriptor-relative no-follow per-node identity and mount authority
- Immutable main and per-node recovery ledgers with deterministic restart
- Recovery-owned candidate construction, atomic install, displacement, and canonical verification
- CLI/schema/store integration and focused recovery-only regression proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority and durable canonical recovery audit
- Added post-exchange candidate name-swap fail-closed proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority and durable canonical recovery audit
- Added post-exchange candidate name-swap fail-closed proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority and durable canonical recovery audit
- Added post-exchange candidate name-swap fail-closed proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority and durable canonical recovery audit
- Added post-exchange candidate name-swap fail-closed proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic 21-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Cross-mount enforcement is implemented with retained-handle fstatfs mount identity; dynamic bind-mount injection was not available locally and is not claimed
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Replaced AT_FDCWD mutations with retained no-follow parent descriptor-relative rename and exchange
- Blocked ordinary commits until every recovery attempt has a validated recovered terminal receipt
- Validated receipt schema, sequence, state, unique predecessor, and hash-chain envelopes on restart
- Inventoried recovery attempts and classified matching completed recovery as already_recovered
- Added distinct terminal receipt tamper and already-recovered classification proof
- Validated the complete 13-state recovery chain, request authority, terminal operation identity, result self-digest, and canonical binding through one shared loader
- Changed rename/exchange interfaces to consume retained anchored parent descriptors and basenames
- Added forged terminal, broken earlier chain, operation mismatch, and self-digest negative proof
- Persisted and recomputed the full PREPARED typed request authority
- Bound terminal operation and receipt result to the canonical recovery audit while allowing later canonical generations
- Revalidated exact retained-parent child identity immediately before dirfd rename/exchange
- Proved two successive ordinary commits after recovery
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority and durable canonical recovery audit
- Added post-exchange candidate name-swap fail-closed proof
- Added typed classify/recover contracts and CLI/schema exports; cleanup remains excluded for #299
- Added retained descriptor-relative no-follow traversal with fstat/fstatfs identity, mount, ownership, permission, type, and link validation
- Added immutable hash-linked main/per-node receipt ledgers and exact-prefix temporary-node create/write/fsync/no-replace-publish restart
- Added archive, candidate verification, atomic exchange/no-replace install, displacement, canonical verification, and final idempotent recovery
- Blocked ordinary commits before complete recovery and proved later ordinary commit after recovery
- Added deterministic recovery-boundary failpoints plus lineage, replacement, symlink, hardlink, mode, topology, initialized/ready, and #291-compatible regressions
- Validated full IssueRecord, all six cards, rendered Markdown, audit JSONL, cross-card bindings, and authored artifact digests before trusting a projection
- Retained the exact checked source child descriptor until rename/exchange completion
- Validated typed request authority, PREPARED classification self-digest, request classification equality, terminal result authority, and durable canonical recovery audit
- Derived completed recovery candidate record/files from the retained displaced prior projection and exact authorized audit transform
- Compared completed candidate-created receipts and terminal canonical artifacts against the authorized recovery transform
- Allowed later ordinary generations after the recovered terminal generation without requiring canonical files to remain at the recovered generation
- Added coherent candidate-chain forgery and malformed request classification fail-closed proof
- Recorded refreshed focused recovery proof with 15 passing tests

## Validation

[
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--lib"
    ],
    "purpose": "Run complete library unit suite.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-lib.log"
  },
  {
    "command": [
      "cargo",
      "clippy",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--all-targets",
      "--",
      "-D",
      "warnings"
    ],
    "purpose": "Run strict all-target Clippy.",
    "outcome": "passed",
    "evidence_ref": "csdlc-v2-strict-clippy.log"
  },
  {
    "command": [
      "cargo",
      "test",
      "--manifest-path",
      "csdlc-v2/Cargo.toml",
      "--test",
      "gate5",
      "preserved_projection_recovery"
    ],
    "purpose": "Run focused gate5 recovery tests.",
    "outcome": "passed",
    "evidence_ref": "preserved-projection-recovery.log"
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
