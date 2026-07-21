# WP-13 External-Band Deletion Preparation Design

Status: preparation only; no deletion authority

## Purpose

Prepare issue #5347 to remove only incumbent ADL files whose accepted owner is
outside the replacement ADL v2 language/compiler/engine/CLI core. The later
execution must be driven by an exact, reviewed file manifest and must preserve
all retained evidence and rollback truth.

## Authority Boundary

#5347 may eventually own deletion of individually listed incumbent files whose
capability owner has moved to Runtime v3, C-SDLC v2, cognitive-domain owners,
integrations, demos, or proof tooling. It does not own:

- ADL v2 language, compiler, portable engine, records, or thin CLI;
- the final incumbent language/compiler/engine/CLI deletion owned by #5346;
- Runtime v3 or C-SDLC v2 implementation;
- selector, soak, rollback, cutover, acceptance, or release decisions;
- historical evidence whose executable authority has already been removed.

Preparation protects only issue-local lifecycle and evidence paths. Product
paths may be added to the typed claim only after a reviewed manifest is frozen,
all dependency gates pass, and a disjointness check proves zero overlap with
#5346. A directory prefix is never sufficient deletion authority.

## Dependency Gate

Execution remains fail-closed until all of the following are true at the exact
candidate revision:

1. #5346 is GitHub merged and typed `closed_out`, its claim is released, its
   retained terminal receipt is valid and ancestral, and its reviewed deletion
   eligibility manifest is available.
2. #5344 soak/rollback and #5343 reversible selector switch are merged, typed
   `closed_out`, receipt-backed, claim-free, and ancestral.
3. #5358 C-SDLC v2 acceptance and #5361 Runtime v3 acceptance are merged, typed
   `closed_out`, receipt-backed, claim-free, and ancestral.
4. Every replacement revision and acceptance receipt named by a #5347 manifest
   row is ancestral and independently accepted.

The live #5346 issue currently says #5346 depends on #5347, while the operator
has directed #5347 to wait for terminal #5346. That cycle is a preparation-time
dependency cycle and stop condition. It must be reconciled through the authoritative issue graph
before either deletion issue executes; this packet does not silently choose a
weaker gate or rewrite live issue truth.

## Manifest Contract

The later immutable JSON manifest must be canonically ordered by normalized
repo-relative path and bind:

- schema version, candidate Git revision, and pinned WP-02 baseline revision;
- exact path, file kind, baseline blob/tree identity, and measured lines;
- incumbent capability and authority-rooted reachability evidence;
- accepted replacement owner, exact replacement revision, terminal receipt,
  and behavior/proof references;
- disposition: `delete_external`, `retain_owned`, `retain_evidence`,
  `handoff_to_5346`, or `blocked`;
- retention rationale and sunset condition for every retained file;
- product-path claim identity for every deletion candidate;
- whole-manifest digest and the exact #5346 manifest digest used by the
  disjointness proof.

Paths are normalized lexically and against the repository root. Absolute paths,
parent traversal, symlink escape, duplicate canonical paths, generated build
output, submodule escape, and untracked files fail closed.

## Disjointness And Deletion Rules

The #5347 and #5346 manifests must have zero canonical path intersection.
Language/compiler/engine/CLI rows route to #5346; Runtime v3, C-SDLC v2,
cognitive, integration, demo, and proof-tool rows may route to #5347 only with
an accepted external owner. Ambiguous or mixed-owner files are `blocked`, not
deleted.

Deletion is permitted only through the exact manifest, one tracked file at a
time, after a typed protected-path claim amendment. Renames, moves, generated
copies, and archived executable duplicates do not count as deletion. Deletion
must not alter Runtime v2 during this issue, and no Runtime v2 file is eligible
merely because Runtime v3 metadata exists.

## Evidence And Rollback

Before deletion, preserve behavior mappings and non-executable historical proof.
After deletion, run focused owner tests, canonical characterization/parity,
security and determinism negatives, selector rollback checks, current CI, and
the exact consumer inventory. Evidence must be deterministic, redacted,
repo-relative, exact-revision bound, and distinguish observed proof from
inference or non-claim.

Rollback is a clean Git revert of the exact deletion commit while accepted
replacement owners remain unchanged. Any missing owner, stale receipt, manifest
drift, claim collision, authoritative reference to a deleted path, or failed
post-deletion check blocks publication and merge.

## COTS And Budgets

Reuse Git object identities and path operations, the installed typed C-SDLC v2
binaries, existing repository characterization/parity tools, and standard JSON
parsers. The preparation-only receipt verifier reuses the typed `csdlc-v2`
model and its pinned BLAKE3 1.8.5 digest primitive; it is validation tooling,
not a product crate. No new product crate, workflow engine, bespoke graph database, or
deletion framework is authorized.

- manifest/gate implementation: at most 500 nonblank lines;
- focused tests and fixtures: at most 800 nonblank lines and fewer than 50 tests;
- retained issue evidence/docs: at most 1,200 nonblank lines;
- focused manifest/disjointness validation: 120 seconds;
- focused owner/consumer proof: 300 seconds;
- complete post-deletion validation: 3,600 seconds;
- net source change must be negative; deleted, retained, replacement, test, and
  evidence lines are reported separately.

Any variance requires exact-revision review and may not weaken a proof gate.

## Preparation Exit

Preparation is complete when all six typed cards, this design, the diagram,
dependency and validation executables, protected paths, COTS decisions, budgets,
and PVF lanes pass bounded review and are committed and pushed. No PR or product
change is part of this exit.
