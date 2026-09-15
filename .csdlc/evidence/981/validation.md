# Issue 981 validation evidence

The repair distinguishes repository-scoped issue creation transactions from
issue-local lifecycle residue by resolving a receipt through its originating
native issue-zero intent and reusing the canonical native intent and receipt
validators. The identity and recovery checks fail closed when the receipt
filename, operation digest, marker, adapter, request, intent digest, repository,
assigned issue, or expected head does not bind exactly.

## Operational proof

- The isolated repaired native candidate prepared issue #981 successfully from
  the reconciled native issue-creation transaction that previously triggered
  `LegacyMigrationRequired`.
- Native `bind` created the registered issue worktree on
  `codex/981-repository-scoped-issue-creation-receipt`.
- Native `edit 981` committed generation 3 with lifecycle digest
  `b065417dc11a5f91aee782abae83a2fec93d3f6824c2b4ab14c5a12f15074e46`.
- Native `validate 981` accepted that generation and all six rendered cards.

## Local validation

- `cargo test --manifest-path csdlc-v3/Cargo.toml repository_scoped_issue_creation_receipt -- --nocapture`
  passed the valid receipt case and the negative mismatch matrix. The negative
  matrix alters the intent operation digest, intent marker, intent request,
  receipt operation digest, receipt intent digest, and receipt filename; every
  case returns `RecoveryRequired` for observation and preparation.
- `cargo test --manifest-path csdlc-v3/Cargo.toml` passed the full component
  suite, including all 45 transaction tests. The installed prepared-measurement
  case remained intentionally ignored because it is manual-only; no test
  failed.
- `cargo fmt --manifest-path csdlc-v3/Cargo.toml -- --check` passed.
- `cargo clippy --manifest-path csdlc-v3/Cargo.toml --all-targets --all-features -- -D warnings`
  passed.
- `git diff --check` passed.
- Native `csdlc proof 981` passed against the exact current candidate recorded
  in the retained native proof artifact: its admitted `semantic_gate_a` filter
  executed 21 tests with zero failures and revalidated unchanged tracked
  inputs. The proof also verifies the repair for Cargo's unhashed final-binary
  artifact, whose dep-info lives under the hashed
  `target/intent-validation/debug/deps` entry.

Independent exact-head review, PR publication, CI, merge, native finish, and
cleanup remain pending.
