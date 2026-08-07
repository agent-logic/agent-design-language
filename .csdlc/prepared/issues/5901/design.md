# Issue 5901 Design: Claim-Free WP-04 Wave Readiness

## Outcome And Boundary

Make the already-approved Sprint 3 child packets bindable under the final
claim-free C-SDLC v2 Git-topology authority. Repair only the generic path and
governed-script readiness predicates, the one malformed #5865 path projection,
and the umbrella's stale claim-based terminal validator. Do not bind or
implement a Guardian child.

## Source Baseline

- `AGENTS.md` establishes Git branch/worktree topology as lifecycle authority.
- `csdlc-v2/src/cards.rs` owns execution-readiness validation.
- `.csdlc/issues/5863/cards/spp.values.json` and
  `.csdlc/issues/5863/cards/vpp.values.json` demonstrate a valid future-file
  child packet that current doctor rejects.
- `.csdlc/issues/5865/cards/spp.values.json` contains a serialization token in
  the path-only `affected_areas` collection.
- `.csdlc/prepared/issues/5862/validate-implementation-wave.rb` owns Sprint 3
  preflight and terminal reconciliation.
- PR #5886 removed claims; issue #5896 and PR #5897 migrated pre-topology
  records.

## Owned Paths

- `csdlc-v2/src/cards.rs`
- `csdlc-v2/src/store.rs`
- `csdlc-v2/src/bin/csdlc-finish.rs`
- `csdlc-v2/tests/gate2.rs`
- `csdlc-v2/tests/gate_finish.rs`
- `.csdlc/issues/5865/cards/sip.values.json`
- `.csdlc/issues/5865/cards/stp.values.json`
- `.csdlc/issues/5865/cards/spp.md`
- `.csdlc/issues/5865/cards/spp.values.json`
- `.csdlc/issues/5865/cards/vpp.values.json`
- `.csdlc/issues/5865/cards/srp.values.json`
- `.csdlc/issues/5865/cards/sor.values.json`
- `.csdlc/issues/5865/index.json`
- `.csdlc/issues/5865/audit.jsonl`
- `.csdlc/prepared/issues/5862/validate-implementation-wave.rb`
- `.csdlc/prepared/issues/5901/test-implementation-wave.rb`
- `.csdlc/issues/5901`
- `.csdlc/prepared/issues/5901`
- `.csdlc/evidence/5901`

## Read-Only Inputs

- Every other Sprint 3 child record and design is read-only.
- Distributed Guardian product paths are read-only and must not be created by
  this repair.
- #5800, #5820, and #5821 lifecycle state is read-only dependency evidence.

## Readiness Semantics

A future owned path is admissible only when every component is a normal
repository-relative component, no existing component is a symbolic link, and
the nearest existing non-empty prefix is a real directory whose canonical path
remains beneath the canonical repository root. Any existing non-directory
intermediate prefix is rejected rather than skipped in favor of a farther
directory ancestor. Absolute paths, traversal, placeholders, empty values,
free-form metadata, symlink ancestors pointing inside or outside the repository,
and symlink leaves remain invalid. Existing regular files and immediate-parent
future files continue to pass.

A Bash validation lane is proving only when its first argument after the
interpreter exactly matches a path in the issue's SPP `affected_areas` and is a
safe repository-relative `.sh` path under the same owned-path rules. This
admits #5878's approved and explicitly owned future validation script while
continuing to reject bare Bash, `bash -c`, unowned scripts, traversal,
placeholders, and ungoverned shell execution. Other existing non-proving shell
exclusions remain unchanged.

The #5865 serialization gate remains durable machine-readable planning truth,
but must not occupy the path-only `affected_areas` collection. Use one typed
`replace_planning_collection` SPP edit to retain the existing
`SERIALIZATION_GATE {"schema":"csdlc.serialization_gate.v1",...}` token in
`replan_triggers`, where serialization-gate drift truthfully triggers replanning,
and replace `affected_areas` with exactly the four product paths already named
by the approved #5865 design. The typed edit may update only the #5865 generated
SPP projection, all six generated values identities, index, and audit ledger;
the other five rendered card Markdown files remain unchanged.

Because #5865 is intentionally unbound and its malformed path collection is
itself what blocks binding, the typed editor must allow the same bounded
planning-collection replacement during `initialized` or `ready` that it already
allows after binding. This design-time repair authority applies only to
planning cards and still requires exact generation/digest checks, full card
verification, schema validation, audit projection, and regenerated digests.

## Terminal Reconciliation Semantics

The terminal wave validator must load and validate each immutable
`csdlc.derived_terminal.v1` envelope from the shared Git-common cache. Each
envelope must match the child's canonical initialization digest, generation,
digest, repository, PR number, merged disposition, head SHA, merge SHA,
closed-by-merged-PR state, live GitHub closing linkage, and candidate-head
ancestry. The validator must recompute the envelope digest through the same
schema contract used by `csdlc-finish`; it must not require legacy
`index.terminal`, full closeout receipts, claims, leases, heartbeats, or
protected-path authority.

Expose a read-only `csdlc-finish --root <repo> --validate-cached-issue <issue>`
mode that loads the Git-common derived envelope through the existing Rust
schema/digest validator and proves it matches the canonical issue record and
publication identity. Preserve the existing `--request` finish invocation
unchanged. The Ruby wave validator consumes this typed result, then independently
checks live GitHub closing linkage and Git ancestry.

The preflight path is claim-free as well: it must not read `index["claim"]` or
derive success from claim-null state. It verifies initialized, unbound Git
topology for the not-yet-started children, approved design/card integrity, the
exact dependency DAG, and exact disjoint owned paths. Its success output must
name claim-free topology rather than historical claim state.

## Validation

- Focused Rust tests for safe existing and future paths plus absolute,
  traversal, placeholder, unrooted, metadata, inside-symlink,
  outside-symlink, symlink-leaf, and existing-file-intermediate rejection cases.
- Focused Rust tests proving only exactly owned, governed repository-relative
  Bash scripts are admitted as validation lanes, including the approved
  future-script shape and rejection of a safe-shaped but unowned script.
- Focused typed-edit tests proving planning-collection replacement succeeds in
  `initialized` and `ready` only with exact generation/digest, rejects stale
  identity and non-planning operations, and preserves bound behavior.
- Focused finish-binary tests proving cached-envelope validation accepts an
  exact canonical envelope and rejects missing, malformed, stale-record, and
  digest-mismatched caches without GitHub mutation.
- Typed doctor for #5862 and #5863 through #5878.
- `ruby .csdlc/prepared/issues/5862/validate-implementation-wave.rb --preflight`.
- Ruby syntax validation for the wave validator.
- Deterministic fixture-backed execution of the non-preflight branch covering a
  valid derived-envelope wave plus malformed envelope, digest, head, merge, live
  linkage, and candidate-ancestry failures.
- Issue-local exact-base changed-path allowlist proving that no distributed
  Guardian product path or child topology was changed.
- Exact-head independent review before publication.

## Rollback

Revert the bounded readiness commit. No product child is bound, no distributed
source file is created, and the #5821 gate remains authoritative throughout.

## Non-Goals

- Distributed Guardian implementation or proof generation.
- Binding #5862 or any child.
- Restoring claims or any v1 lifecycle route.
- Changing the approved child DAG, denominator, or product ownership.
