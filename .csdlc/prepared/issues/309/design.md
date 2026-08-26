# Issue #309 Design — Proof-Led Repository Reduction

## Objective

Reduce the current `adl/src` production surface without changing supported
behavior or transferring code into an unowned compatibility layer. The exact
execution baseline is commit `e926e3bca0ab1981d77b4658d2feb4059bdf33a6`
(tree `c57bae97083b42125d7308047595ec2e96033240`), containing 485 Rust files and
265,633 physical Rust lines under `adl/src`.

There is no mandatory percentage, file, or line target. Reduction is earned
only by complete disposition evidence and passing behavior/authority proof.

## Dependency Gate

Issue #308 is terminal at canonical generation 17, canonical digest
`52f3d81eb6f9d7cc082b1efca239daca3e2dc8ff8446908bbca566fb3b8ffa8f`,
merge `9f373f5f04b0f8c9dc6e3e6cbf348fddec98486c`, and terminal digest
`8e4baf86516602ba5991f58483b5b53eb2c932b4e5f2cdc314bb61d1dc4bf844`.
That merge is ancestral to the execution baseline and its worktree is absent.

## Authority And Scope

- `adl/src` is the reduction denominator, not an instruction to delete every
  file.
- Current Runtime v2 authorities consumed by Runtime v3, #414 continuity, or
  any supported command remain protected unless an exact replacement is
  already merged and parity-proven.
- #309 owns deletion, compatibility removal, manifests, and proof needed for
  those deletions. #310 owns later aesthetic or architectural refactoring.
- Movement, build exclusion, feature gating, or copying to a new compatibility
  directory earns no reduction credit.
- `.adl` planning material is neither execution authority nor a dependency.

## Inventory Contract

Create one immutable baseline manifest row for every Git-tracked Rust file
under `adl/src`. Each row binds path, Git blob, physical line count, build
membership, owner/consumer classification, one disposition, replacement
evidence when applicable, validation requirements, and rollback source.

Create a separate normalized reference-edge manifest covering every observed
active reference from the repository-wide tracked denominator and declared
external contracts. Each edge binds a stable edge identity, source path plus
source blob (or an external contract identity), target path/symbol, reference
class (`module`, `build`, `cli`, `test`, `documentation`, `artifact`,
`workflow`, or `external_contract`), consumer owner, disposition, and evidence.
The census records its complete scanned path/blob denominator and search rules.
Missing, duplicate, unclassified, or contradictory edges fail closed; deleting
a target requires every incoming active edge to be removed or mapped to a
proven replacement.

Allowed dispositions are:

1. `retain_active`
2. `delete_dead`
3. `delete_superseded`
4. `migrate_then_delete`
5. `temporary_exception`

Missing, duplicate, contradictory, or unowned rows fail closed. Temporary
exceptions require owner, reason, and expiry. `migrate_then_delete` does not
authorize migration within #309; it records the stopping boundary for separate
owned work.

## Reduction Bands

### Band A — Dead And Unreachable

Delete only files for which compilation/build membership, module reachability,
CLI routing, documentation/artifact contracts, tests, and repository-wide
reverse references all show no active consumer. Preserve negative command
behavior and exact restoration evidence.

### Band B — Characterized Orphan Implementations

Delete a second independently reversible dead-code wave only after correcting
false-positive module/path edges and proving each implementation has no active
runtime, CLI, build, workflow, test-contract, artifact, documentation, or
external consumer. A retained historical demo artifact explains provenance but
does not substitute for consumer proof. If any candidate is actually
superseded rather than unreachable, stop and require a merged current owner
plus positive, negative, artifact, trace, persistence, error, and clean-install
parity; #309 does not delete it as dead code.

Band B source authority is commit
`f3cf4c937cbd55beb5e78b73b838033ff63bae66`; its rollback proof is the exact
revert `6ad24bc198fdab7d3b908955ba57b48836ae8ec1` and reapply
`29093a1668ca6a7f0db2f64d6f1b1361205a7620` topology. Evidence refresh must
derive from those objects and fail if later `adl/src` drift exists.

### Band C — Runtime V2 Contraction Decision

Classify every Runtime v2 file by surviving consumer. Delete only genuinely
unconsumed subtrees with an already-landed replacement. Protect lifecycle,
citizen identity, snapshot, rehydration, rollback, and admission behavior used
by #414 and current Runtime paths. If additional migration is required, stop
#309 at the proven boundary and route it separately.

## Git Reversion Plan

The immutable restoration source is baseline commit `e926e3bca0...`. Each
reduction band is one reviewable commit and has a manifest listing every
removed path and baseline blob.

Rollback procedure:

1. Freeze later writes to the affected paths and verify the current candidate
   head.
2. Revert only the failed band with `git revert <band-commit>`; never reset the
   branch or overwrite unrelated later work.
3. Verify restored path/blob equality against the baseline manifest.
4. Rerun the band's focused positive, negative, artifact, trace, persistence,
   error, and clean-install proof.
5. Retain the failed receipt and stop if restoration conflicts with later
   independently owned changes.

Before publication, test every band reversal in a disposable detached worktree
or index and prove both exact restoration and reapplication. Git object
retention is not sufficient without this executable rollback proof.

## Validation

- exact baseline and candidate file/blob/line accounting;
- complete file disposition plus normalized repository-wide reference-edge
  denominator validation;
- supported CLI clean installation and command inventory;
- positive/negative behavior plus artifact, trace, persistence, and error parity;
- #414 resident continuity regression;
- applicable Runtime v3 and owner lanes;
- strict format and Clippy for touched workspaces;
- the issue-specific PR-fast exception must match the exact sorted status/path
  manifest SHA-256
  `5b86080fd99cc41c0a25fd7d892cedfd0ae2eb0e4f8a2cfa04bc9a9be2aa48ac`;
  any added, omitted, renamed, or differently modified Rust path uses the
  ordinary fail-closed router, while the exact match runs protected
  `resident_shepherd_spot_continuity` and `adl::cli_smoke` proof;
- native macOS proof and hosted Linux proof bound to PR #460, the checked-out
  exact head, Linux/X64, and exactly `adl-path-policy`,
  `adl-tooling-contracts`, `adl-rust-fmt-clippy`, `adl-rust-tests`,
  `adl-coverage`, and `adl-ci`, with successful conclusions, per-job artifact
  digests, and a nonzero parsed test denominator for `adl-rust-tests`;
- exact base-to-candidate scope and diff hygiene;
- executable per-band Git rollback/reapply proof;
- one exact-head independent review.

Zero-test, skipped, missing-platform, unparsed-count, or self-asserted evidence
is non-proving.

## Stop Conditions

Stop before deletion when any row lacks an owner/disposition, active references
remain, replacement parity is incomplete, #414/Runtime authority would be
weakened, clean installation changes unexpectedly, rollback cannot restore exact
blobs, or the work becomes migration/refactoring owned by #310 or another issue.

## Publication Boundary

The PR reports exact achieved file and line reduction, retained-authority
reasons, exceptions, proof, and residual migration. It uses `Closes #309` and
does not claim #310, WP-21A, Sprint 6, or milestone completion.
